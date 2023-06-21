pub(crate) use m3_map::tiles::MapBaseTile;
use m3_map::{
	story::{Background, Character},
	tiles::{ObjectTile, PlayerTile, Tile}
};
use macroquad::{prelude::ImageFormat, texture::Texture2D};
use once_cell::sync::Lazy;

#[allow(clippy::redundant_closure)] //false positive?
pub(crate) static TEXTURES: Lazy<Textures> = Lazy::new(|| Textures::init());

pub(crate) trait GetTexture {
	fn texture(&self) -> Texture2D;
}

///Store all Textures
pub(crate) struct Textures {
	grass: Texture2D,
	stone: Texture2D,
	player1_car: Texture2D,
	player2_car: Texture2D,
	player3_car: Texture2D,
	player4_car: Texture2D,
	global_goal: Texture2D,
	player1_goal: Texture2D,
	player2_goal: Texture2D,
	player3_goal: Texture2D,
	player4_goal: Texture2D,
	captain: Texture2D,
	outer_space: Texture2D
}

impl Textures {
	///init all Textures
	pub(crate) fn init() -> Textures {
		Textures {
			grass: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/grass.png"),
				Some(ImageFormat::Png)
			),
			stone: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/ObjectTiles/stone.png"),
				Some(ImageFormat::Png)
			),
			player1_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player1_car.png"),
				Some(ImageFormat::Png)
			),
			player2_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player2_car.png"),
				Some(ImageFormat::Png)
			),
			player3_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player3_car.png"),
				Some(ImageFormat::Png)
			),
			player4_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player4_car.png"),
				Some(ImageFormat::Png)
			),
			global_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/goal.png"),
				Some(ImageFormat::Png)
			),
			player1_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player1_goal.png"),
				Some(ImageFormat::Png)
			),
			player2_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player2_goal.png"),
				Some(ImageFormat::Png)
			),
			player3_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player3_goal.png"),
				Some(ImageFormat::Png)
			),
			player4_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player4_goal.png"),
				Some(ImageFormat::Png)
			),
			captain: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Character/captain.png"),
				Some(ImageFormat::Png)
			),
			outer_space: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Backgrounds/outer_space.png"),
				Some(ImageFormat::Png)
			)
		}
	}

	pub(crate) fn get_player_textures(&self) -> Vec<Texture2D> {
		vec![
			self.player1_car,
			self.player2_car,
			self.player3_car,
			self.player4_car,
		]
	}
}

impl GetTexture for MapBaseTile {
	///get Texture assioated with this Tile
	fn texture(&self) -> Texture2D {
		match self {
			Self::Grass => TEXTURES.grass
		}
	}
}

impl GetTexture for ObjectTile {
	///get Texture assioated with this Tile
	fn texture(&self) -> Texture2D {
		match self {
			Self::Stone => TEXTURES.stone
		}
	}
}

impl GetTexture for PlayerTile {
	///get Texture assioated with this Tile
	fn texture(&self) -> Texture2D {
		match self {
			Self::Car1 => TEXTURES.player1_car,
			Self::Car2 => TEXTURES.player2_car,
			Self::Car3 => TEXTURES.player3_car,
			Self::Car4 => TEXTURES.player4_car,
			Self::GlobalGoal => TEXTURES.global_goal,
			Self::Goal1 => TEXTURES.player1_goal,
			Self::Goal2 => TEXTURES.player2_goal,
			Self::Goal3 => TEXTURES.player3_goal,
			Self::Goal4 => TEXTURES.player4_goal
		}
	}
}

impl GetTexture for Tile {
	fn texture(&self) -> Texture2D {
		match self {
			Tile::MapBaseTile(tile) => tile.texture(),
			Tile::MapObjectTile(tile) => tile.texture(),
			Tile::PlayerTile(tile) => tile.texture()
		}
	}
}

impl GetTexture for Character {
	fn texture(&self) -> Texture2D {
		match self {
			Character::Captain => TEXTURES.captain
		}
	}
}

impl GetTexture for Background {
	fn texture(&self) -> Texture2D {
		match self {
			Background::OuterSpace => TEXTURES.outer_space
		}
	}
}
