use log::debug;
use m3_models::AvailableCards;
use serde::{
	ser::{SerializeMap, Serializer},
	Deserialize, Serialize,
	__private::ser::FlatMapSerializeStructVariantAsMapValue
};
use std::{iter, path::Path};
use thiserror::Error;
use tiled::{LayerTile, LayerType, Loader, Properties};

pub mod tiles;
use tiles::{InvalidTileID, MapBaseTile, ObjectTile, PlayerTile, Tile};

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
	name: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
	pub start: (u8, u8),
	pub orientation: Orientation,
	pub goal: Option<(u8, u8)>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Map {
	pub name: String,
	pub width: u8,
	pub height: u8,
	pub base_layer: Vec<Vec<MapBaseTile>>,
	pub object_layer: Vec<Vec<Option<ObjectTile>>>,
	pub global_goal: Option<(u8, u8)>,
	pub player_1: Player,
	pub player_2: Option<Player>,
	pub player_3: Option<Player>,
	pub player_4: Option<Player>,
	pub cards: AvailableCards
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Orientation {
	North,
	South,
	East,
	West
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
	fn try_from(value: &LayerTile) -> Result<Self, Self::Error> {
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
	#[error("{0} Layer should be a {1}")]
	WrongLayer(usize, String),
	#[error("{0} Layer Infinite")]
	InfiniteTileLayer(String),
	#[error("Map is to widht. Max size is 255x255 tiles")]
	ToWidth,
	#[error("Map is to hight. Max size is 255x255 tiles")]
	ToHight,
	#[error("{0}")]
	InvalidTileId(#[from] InvalidTileID),
	#[error("Map needs at least one player")]
	NoPlayer,
	#[error("{0}")]
	InvalidOritation(#[from] InvalidOritation),
	#[error("Failed to load Map Properties:\n{}\n{}", .str, .err)]
	MapProperty { str: String, err: serde_json::Error }
}

impl Map {
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
								let tile = match tile_layer.get_tile(x.into(), y.into()) {
									Some(tile) => MapBaseTile::try_from(tile.id()),
									None => Ok(MapBaseTile::default())
								}?;
								column.push(tile);
							}
							base_layer.push(column);
						}
					},
					_ => return Err(MapError::WrongLayer(i, "TileLayer".to_owned()))
				},
				1 => match layer.layer_type() {
					LayerType::Tiles(tile_layer) => {
						for x in 0..width {
							let mut column = Vec::with_capacity(width as usize);
							for y in 0..height {
								let tile = match tile_layer.get_tile(x.into(), y.into()) {
									Some(tile) => Some(ObjectTile::try_from(tile.id())?),
									None => None
								};
								column.push(tile);
							}
							object_layer.push(column);
						}
					},
					_ => return Err(MapError::WrongLayer(i, "TileLayer".to_owned()))
				},
				2 => match layer.layer_type() {
					LayerType::Tiles(tile_layer) => {
						for x in 0..width {
							for y in 0..height {
								if let Some(tile) =
									tile_layer.get_tile(x.into(), y.into())
								{
									let orientation = Orientation::try_from(&tile)?;
									let tile = PlayerTile::try_from(tile.id())?;
									let player = Some(Player {
										start: (x, y),
										orientation,
										goal: None
									});
									match tile {
										PlayerTile::Car1 => player_1 = player,
										PlayerTile::Car2 => player_2 = player,
										PlayerTile::Car3 => player_3 = player,
										PlayerTile::Car4 => player_4 = player,
										PlayerTile::GlobalGoal => {
											global_goal = Some((x, y))
										},
									}
								}
							}
						}
					},
					_ => return Err(MapError::WrongLayer(i, "TileLayer".to_owned()))
				},
				_ => return Err(MapError::ToManyLayers)
			}
		}
		let player_1 = player_1.ok_or(MapError::NoPlayer)?;
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
			cards
		})
	}

	/// return an iterator over all BasteTiles and its x and y postion
	pub fn iter_base_layer(&self) -> impl Iterator<Item = (u8, u8, &MapBaseTile)> {
		self.base_layer.iter().enumerate().flat_map(|(x, y_vec)| {
			y_vec
				.iter()
				.enumerate()
				.map(move |(y, item)| (x as u8, y as u8, item))
		})
	}

	/// return an iterator over all ObjectTiles and its x and y postion
	pub fn iter_object_layer(&self) -> impl Iterator<Item = (u8, u8, ObjectTile)> + '_ {
		self.object_layer.iter().enumerate().flat_map(|(x, y_vec)| {
			y_vec
				.iter()
				.enumerate()
				.filter_map(move |(y, item)| item.map(|item| (x as u8, y as u8, item)))
		})
	}

	/// return an iterator over all player goals tiles and its x and y postion
	pub fn iter_player_goals(&self) -> impl Iterator<Item = (u8, u8, PlayerTile)> + '_ {
		iter::once(self.global_goal)
			.flat_map(|goal| goal.map(|(x, y)| (x, y, PlayerTile::GlobalGoal)))
	}

	/// return an iterator over all static Tiles and its x and y postion.
	/// starting from the lowest layer
	pub fn iter_all(&self) -> impl Iterator<Item = (u8, u8, Tile)> + '_ {
		let base = self
			.iter_base_layer()
			.map(|(x, y, tile)| (x, y, Tile::MapBaseTile(tile.to_owned())));
		let objects = self
			.iter_object_layer()
			.map(|(x, y, tile)| (x, y, Tile::MapObjectTile(tile.to_owned())));
		let goals = self
			.iter_player_goals()
			.map(|(x, y, tile)| (x, y, Tile::PlayerTile(tile)));
		base.chain(objects).chain(goals)
	}
}
