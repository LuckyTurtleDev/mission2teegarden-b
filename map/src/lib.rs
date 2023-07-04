#![warn(rust_2018_idioms, unreachable_pub)]
#![deny(rustdoc::bare_urls, rustdoc::broken_intra_doc_links)]
#![forbid(unused_must_use, unsafe_code)]


//! # Mapeditor
//!
//! Mission to Teegarden b allow creating custom maps, by using the [Tiled Map editor](https://www.mapeditor.org/).
//! This does include support for available instructions and story.
//!
//! ### Limitaions
//! There exist some conditions and limitation how the map is structured:
//! * the map must be finite.
//! * all layers must be finite.
//! * no custom Tileset can be used. So only the Tilesets available at github
//! ([`BaseTiles.tsx`](https://github.com/LuckyTurtleDev/mission2teegarden_b/blob/main/pc/assets/img/BaseTiles/BaseTiles.tsx),
//! [`ObjectTiles.tsx`](https://github.com/LuckyTurtleDev/mission2teegarden_b/blob/main/pc/assets/img/ObjectTiles/ObjectTiles.tsx),
//! [`Player.tsx`](https://github.com/LuckyTurtleDev/mission2teegarden_b/blob/main/pc/assets/img/Player/Player.tsx)) can be used.
//! * All layers must be a Tile layer.
//! * The 1. Layer must only use Tiles from the `BaseTiles` set.
//! * The 2. Layer must only use Tiles from the `ObjectTiles` set.
//! * The 3. Layer must only use Tiles from the `Player` set.
//! * If a field at layer 1. is not set `Grass` is used as default.
//! * If player `i` have a start postion. All player `<i` must also have a start postion.
//! * At least player `i` must have a start postion.
//! * If global goal was not set, each player (which have a start position), must have a player goal.
//!
//! ### Available Instructions
//! Available instruction can be added, by adding a "Custom properties" with type `int` to the Map.
//! The properties must be named like the fields of the [`AvailableCards`](crate::AvailableCards) struct.
//! If no propertie for an instruction is set, `0` is used as default.
//! Keep in mind that the player can only use `12` cards in total.
//!
//! ### Story
//! An optional story can be added by creating a map property called `story` from type `string`
//! For decoding the story inside the property the [toml](https://toml.io/) format is used.
//! Currently only story elements before and after the level are supported.
//!
//! Take a look at this example story:
//! ```
//! # let toml = r#"
//! [[pre_level]]
//! text = "hi, I am the captain ..."
//! profil = "Captain"
//! background = "OuterSpace"
//!
//! [[pre_level]]
//! text = "now it is you turn!"
//!
//! [[after_level]]
//! text = "You have mastered the challenge!"
//! profil = "Captain"
//! # "#;
//! # let _config: mission2teegarden_b_map::story::Story = basic_toml::from_str(&toml).unwrap_or_else(|err| panic!("{}", err));
//! ```
//! The story has two lists `pre_level` and `after_level`, both are optional.
//! Each list include zero or more [`Speech`s](crate::story::Speech).
//! The [`Speech`s](crate::story::Speech) from `pre_level` are shown before the level starts,
//! The ones from `after_level` are show, after the level was finish successfully.
//! A [`Speech`](crate::story::Speech) exist out of a `text`, a `profil`picture and a `background`.
//! The last two are optional.
//! `profil` defined the picture, which is show left from the text.
//! All variants of [`Character`](`crate::story::Character`) can be used for this.
//! If `profil` is not set, no picture will be shown.
//! `background` define the background with is show above the text.
//! All variants of [`Background`](`crate::story::Background`) can be used for this.
//! If `background` is not set, the level will be shown.
//!
//! For more info see the [`Story`](`crate::story::Story`) struct.
//!
//! ### Map validation
//! The map can be validate by using the game or this crate.
//! ```bash
//! mission2teegarden-b validate-map <FILE>
//! mission2teegarden-b-map validate <FILE>
//! ```
//!
//! ### Map export
//! Map exporting works similary to validation:
//! ```bash
//! mission2teegarden-b export-map <FILE>
//! mission2teegarden-b-map export <FILE>
//! ```
//! Executing one of the commands creates a file with the same basename as the original file and the extension `.m2tb_map`.
//! Since the map format is not stable yet, it is strongly recommanded to keep the original `.tmx` file


use basic_toml as toml;
use log::debug;
use mission2teegarden_b_models::AvailableCards;
use ron::error::SpannedError;
use serde::{
	ser::{SerializeMap, Serializer},
	Deserialize, Serialize
};
use std::{
	f32::consts::PI,
	io, iter,
	path::{Path, PathBuf}
};
use story::Story;
use thiserror::Error;
use tiled::{LayerTile, LayerType, Loader, Properties};

pub mod commands;
pub mod story;
pub mod tiles;
use tiles::{InvalidTile, MapBaseTile, ObjectTile, Passable, PlayerTile, Tile};

/// allow Serialization of MapProporties
struct PropertiesSerde(Properties);
impl Serialize for PropertiesSerde {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		let mut map = serializer.serialize_map(Some(self.0.len()))?;
		for (key, value) in self.0.clone() {
			match value {
				tiled::PropertyValue::IntValue(value) => {
					map.serialize_entry(&key, &value)
				},
				tiled::PropertyValue::BoolValue(value) => {
					map.serialize_entry(&key, &value)
				},
				tiled::PropertyValue::FileValue(value) => {
					map.serialize_entry(&key, &value)
				},
				tiled::PropertyValue::FloatValue(value) => {
					map.serialize_entry(&key, &value)
				},
				tiled::PropertyValue::ColorValue(_) => Ok(()), /* should I return an error instead? */
				tiled::PropertyValue::ObjectValue(value) => {
					map.serialize_entry(&key, &value)
				},
				tiled::PropertyValue::StringValue(value) => {
					map.serialize_entry(&key, &value)
				},
			}?;
		}
		map.end()
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MapProperties {
	#[serde(flatten)]
	cards: AvailableCards,
	name: Option<String>,
	story: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
	pub position: (u8, u8),
	pub orientation: Orientation,
	pub goal: Option<(u8, u8)>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Map {
	pub name: String,
	pub width: u8,
	pub height: u8,
	pub base_layer: Vec<Vec<(MapBaseTile, Orientation)>>,
	pub object_layer: Vec<Vec<Option<(ObjectTile, Orientation)>>>,
	pub global_goal: Option<(u8, u8)>,
	//this was a stupid idea
	//now I must impl everything 4 times
	//I should refactor this to `[Option<Player>;4]`, if I have time.
	//Or add at leat an interator over all Players.
	pub player_1: Player,
	pub player_2: Option<Player>,
	pub player_3: Option<Player>,
	pub player_4: Option<Player>,
	pub cards: AvailableCards,
	#[serde(default)]
	pub story: Story
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum Orientation {
	#[default]
	North,
	South,
	East,
	West
}

impl Orientation {
	/// return the rotate of the Orientation in grad
	pub fn rotation(&self) -> f32 {
		match self {
			Self::North => 0.0,
			Self::South => PI,
			Self::East => 0.5 * PI,
			Self::West => 1.5 * PI
		}
	}
}

#[derive(Error, Debug)]
#[error("Invalid Tile Oritation (horizontally flip: {}, vertically flip: {}, diagonally flip: {})\nKeep in mind that only rotation is supported", .filp_h, .filp_v, .filp_d)]
pub struct InvalidOritation {
	///Whether this tile is flipped on its Y axis (horizontally).
	filp_h: bool,
	///Whether this tile is flipped on its X axis (vertically).
	filp_v: bool,
	///Whether this tile is flipped diagonally.
	filp_d: bool
}

impl TryFrom<&LayerTile<'_>> for Orientation {
	type Error = InvalidOritation;
	fn try_from(value: &LayerTile<'_>) -> Result<Self, Self::Error> {
		match (value.flip_h, value.flip_v, value.flip_d) {
			(false, false, false) => Ok(Orientation::North),
			(true, true, false) => Ok(Orientation::South),
			(true, false, true) => Ok(Orientation::East),
			(false, true, true) => Ok(Orientation::West),
			_ => Err(InvalidOritation {
				filp_h: value.flip_h,
				filp_v: value.flip_v,
				filp_d: value.flip_d
			})
		}
	}
}

#[derive(Error, Debug)]
pub enum MapError {
	#[error("error loading file {0}")]
	TieledError(#[from] tiled::Error),
	#[error("map has to many layers")]
	ToManyLayers,
	#[error("{0}. Layer should be a {1}")]
	WrongLayerType(usize, String),
	#[error("{0}. Layer Infinite")]
	InfiniteTileLayer(String),
	#[error("Map is to widht. Max size is 255x255 tiles")]
	ToWidth,
	#[error("Map is to hight. Max size is 255x255 tiles")]
	ToHight,
	#[error("Found invalid Tile at Layes {0}: {1}")]
	InvalidTile(usize, InvalidTile),
	#[error("Player is missing")]
	PlayerMissing(usize),
	#[error("{0}")]
	InvalidOritation(#[from] InvalidOritation),
	#[error("Failed to load Map Properties:\n{}\n{}", .str, .err)]
	MapProperty { str: String, err: serde_json::Error },
	#[error("failed to read file story file {1:?}:\n{0}")]
	IoError(io::Error, PathBuf),
	#[error("could not prase story toml file:\n{0}")]
	TomlError(#[from] toml::Error)
}

impl Map {
	///return if all static tiles at x,y postion are passable
	pub fn passable(&self, x: u8, y: u8) -> bool {
		if x >= self.width || y >= self.height {
			//car can leave the map an drive away (game over)
			return true;
		}
		self.base_layer[x as usize][y as usize].0.passable()
			&& self.object_layer[x as usize][y as usize]
				.map(|obejct| obejct.0.passable())
				.unwrap_or(true)
	}

	///Load map from String.
	///Allowing to load map from binary format
	pub fn from_string(str: &str) -> Result<Self, SpannedError> {
		ron::from_str(str)
	}

	///Convert map to String, to be used as binary file format
	#[allow(clippy::inherent_to_string)]
	pub fn to_string(&self) -> String {
		ron::to_string(self).unwrap()
	}

	pub fn from_tmx(path: impl AsRef<Path>) -> Result<Self, MapError> {
		let path = path.as_ref();
		let map = Loader::new().load_tmx_map(path)?;
		let width: u8 = map.width.try_into().map_err(|_| MapError::ToWidth)?;
		let height: u8 = map.height.try_into().map_err(|_| MapError::ToHight)?;
		let map_properties =
			serde_json::to_string_pretty(&PropertiesSerde(map.properties.clone()))
				.unwrap();
		debug!("load Map Properties: {map_properties}");
		//Do I really need to convert this to json and back?
		//Is their no serde intern format, which I can use?
		//Why can I not use ron for this https://github.com/ron-rs/ron/issues/456 ?
		let map_properties: MapProperties = serde_json::from_str(&map_properties)
			.map_err(|err| MapError::MapProperty {
				str: map_properties,
				err
			})?;
		let cards = map_properties.cards;
		let name = map_properties
			.name
			.unwrap_or_else(|| path.to_string_lossy().into());
		let mut base_layer = Vec::with_capacity(height as usize);
		let mut object_layer = Vec::with_capacity(height as usize);
		let mut global_goal = None;
		let mut player_1 = None;
		let mut player_2 = None;
		let mut player_3 = None;
		let mut player_4 = None;
		for (i, layer) in map.layers().enumerate() {
			// this is ugly. Should i refactor this?
			match i {
				0 => match layer.layer_type() {
					LayerType::Tiles(tile_layer) => {
						for x in 0..width {
							let mut column = Vec::with_capacity(width as usize);
							for y in 0..height {
								let tile_and_orientation = match tile_layer
									.get_tile(x.into(), y.into())
								{
									Some(tile) => (
										MapBaseTile::try_from(&tile).map_err(|err| {
											MapError::InvalidTile(i, err)
										})?,
										Orientation::try_from(&tile)?
									),
									None => (MapBaseTile::default(), Default::default())
								};
								column.push(tile_and_orientation);
							}
							base_layer.push(column);
						}
					},
					_ => return Err(MapError::WrongLayerType(i, "TileLayer".to_owned()))
				},
				1 => match layer.layer_type() {
					LayerType::Tiles(tile_layer) => {
						for x in 0..width {
							let mut column = Vec::with_capacity(width as usize);
							for y in 0..height {
								let tile = match tile_layer.get_tile(x.into(), y.into()) {
									Some(tile) => Some((
										ObjectTile::try_from(&tile).map_err(|err| {
											MapError::InvalidTile(i, err)
										})?,
										Orientation::try_from(&tile)?
									)),
									None => None
								};
								column.push(tile);
							}
							object_layer.push(column);
						}
					},
					_ => return Err(MapError::WrongLayerType(i, "TileLayer".to_owned()))
				},
				2 => match layer.layer_type() {
					LayerType::Tiles(tile_layer) => {
						let mut player1_goal = None;
						let mut player2_goal = None;
						let mut player3_goal = None;
						let mut player4_goal = None;
						for x in 0..width {
							for y in 0..height {
								if let Some(tile) =
									tile_layer.get_tile(x.into(), y.into())
								{
									let orientation = Orientation::try_from(&tile)?;
									let tile = PlayerTile::try_from(&tile)
										.map_err(|err| MapError::InvalidTile(i, err))?;
									let player = Some(Player {
										position: (x, y),
										orientation,
										goal: None
									});
									let goal = Some((x, y));
									match tile {
										PlayerTile::Car1 => player_1 = player,
										PlayerTile::Car2 => player_2 = player,
										PlayerTile::Car3 => player_3 = player,
										PlayerTile::Car4 => player_4 = player,
										PlayerTile::Goal1 => player1_goal = goal,
										PlayerTile::Goal2 => player2_goal = goal,
										PlayerTile::Goal3 => player3_goal = goal,
										PlayerTile::Goal4 => player4_goal = goal,
										PlayerTile::GlobalGoal => global_goal = goal
									}
								}
							}
						}
						//this has become ugly; Mabye I should store the players in another way
						//maybe an arry of [player;4]
						player_1 = player_1.map(|mut f| {
							f.goal = player1_goal;
							f
						});
						player_2 = player_2.map(|mut f| {
							f.goal = player2_goal;
							f
						});
						player_3 = player_3.map(|mut f| {
							f.goal = player3_goal;
							f
						});
						player_4 = player_4.map(|mut f| {
							f.goal = player4_goal;
							f
						});
					},
					_ => return Err(MapError::WrongLayerType(i, "TileLayer".to_owned()))
				},
				_ => return Err(MapError::ToManyLayers)
			}
		}
		let player_1 = player_1.ok_or(MapError::PlayerMissing(1))?;
		// if player i does exist, player i-1 must also exist
		if (player_4.is_some() && player_3.is_none())
			|| (player_3.is_some() && player_2.is_none())
		{
			player_2.as_ref().ok_or(MapError::PlayerMissing(2))?;
			player_3.as_ref().ok_or(MapError::PlayerMissing(3))?;
			player_4.as_ref().ok_or(MapError::PlayerMissing(4))?;
		}

		let story: Story = if let Some(story_toml) = map_properties.story {
			toml::from_str(&story_toml)?
		} else {
			Default::default()
		};

		Ok(Map {
			name,
			width,
			height,
			base_layer,
			object_layer,
			global_goal,
			player_1,
			player_2,
			player_3,
			player_4,
			cards,
			story
		})
	}

	pub fn iter_player(&self) -> impl Iterator<Item = &Player> {
		iter::once(&self.player_1)
			.chain(iter::once(&self.player_2).flatten())
			.chain(iter::once(&self.player_3).flatten())
			.chain(iter::once(&self.player_4).flatten())
	}

	pub fn iter_mut_player(&mut self) -> impl Iterator<Item = &mut Player> {
		iter::once(&mut self.player_1)
			.chain(iter::once(&mut self.player_2).flatten())
			.chain(iter::once(&mut self.player_3).flatten())
			.chain(iter::once(&mut self.player_4).flatten())
	}

	/// return an iterator over all BasteTiles and its x and y postion
	pub fn iter_base_layer(
		&self
	) -> impl Iterator<Item = (u8, u8, MapBaseTile, Orientation)> + '_ {
		self.base_layer.iter().enumerate().flat_map(|(x, y_vec)| {
			y_vec
				.iter()
				.enumerate()
				.map(move |(y, item)| (x as u8, y as u8, item.0, item.1))
		})
	}

	/// return an iterator over all ObjectTiles and its x and y postion
	pub fn iter_object_layer(
		&self
	) -> impl Iterator<Item = (u8, u8, ObjectTile, Orientation)> + '_ {
		self.object_layer.iter().enumerate().flat_map(|(x, y_vec)| {
			y_vec.iter().enumerate().filter_map(move |(y, item)| {
				item.map(|item| (x as u8, y as u8, item.0, item.1))
			})
		})
	}

	/// return an iterator over all player goals tiles and its x and y postion
	pub fn iter_player_goals(&self) -> impl Iterator<Item = (u8, u8, PlayerTile)> + '_ {
		iter::once(self.global_goal)
			.flatten()
			.map(|(x, y)| (x, y, PlayerTile::GlobalGoal))
			.chain(
				iter::once(&self.player_1)
					.filter_map(|player| player.goal)
					.map(|(x, y)| (x, y, PlayerTile::Goal1))
			)
			.chain(
				iter::once(&self.player_2)
					.flatten()
					.filter_map(|player| player.goal)
					.map(|(x, y)| (x, y, PlayerTile::Goal2))
			)
			.chain(
				iter::once(&self.player_3)
					.flatten()
					.filter_map(|player| player.goal)
					.map(|(x, y)| (x, y, PlayerTile::Goal3))
			)
			.chain(
				iter::once(&self.player_4)
					.flatten()
					.filter_map(|player| player.goal)
					.map(|(x, y)| (x, y, PlayerTile::Goal4))
			)
	}

	/// return an iterator over all static Tiles and its x and y postion.
	/// starting from the lowest layer
	pub fn iter_all(&self) -> impl Iterator<Item = (u8, u8, Tile, Orientation)> + '_ {
		let base = self.iter_base_layer().map(|(x, y, tile, orientation)| {
			(x, y, Tile::MapBaseTile(tile.to_owned()), orientation)
		});
		let objects = self.iter_object_layer().map(|(x, y, tile, orientation)| {
			(x, y, Tile::MapObjectTile(tile.to_owned()), orientation)
		});
		let goals = self
			.iter_player_goals()
			.map(|(x, y, tile)| (x, y, Tile::PlayerTile(tile), Orientation::default()));
		base.chain(objects).chain(goals)
	}
}
