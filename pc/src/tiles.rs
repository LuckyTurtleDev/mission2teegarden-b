pub use m3_map::tiles::MapBaseTile;
use m3_map::tiles::{ObjectTile, PlayerTile, Tile};
use macroquad::{prelude::ImageFormat, texture::Texture2D};
use once_cell::sync::Lazy;

pub static TEXTURES: Lazy<Textures> = Lazy::new(|| Textures::init());

pub trait GetTexture {
	fn texture(&self, textures: &Textures) -> Texture2D;
}

///Store all Textures
pub struct Textures {
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
	player4_goal: Texture2D
}

impl Textures {
	///init all Textures
	pub fn init() -> Textures {
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
			)
		}
	}
}

impl GetTexture for MapBaseTile {
	///get Texture assioated with this Tile
	fn texture(&self, textures: &Textures) -> Texture2D {
		match self {
			Self::Grass => textures.grass
		}
	}
}

impl GetTexture for ObjectTile {
	///get Texture assioated with this Tile
	fn texture(&self, textures: &Textures) -> Texture2D {
		match self {
			Self::Stone => textures.stone
		}
	}
}

impl GetTexture for PlayerTile {
	///get Texture assioated with this Tile
	fn texture(&self, textures: &Textures) -> Texture2D {
		match self {
			Self::Car1 => textures.player1_car,
			Self::Car2 => textures.player1_car,
			Self::Car3 => textures.player1_car,
			Self::Car4 => textures.player1_car,
			Self::GlobalGoal => textures.global_goal,
			Self::Goal1 => textures.player1_goal,
			Self::Goal2 => textures.player2_goal,
			Self::Goal3 => textures.player3_goal,
			Self::Goal4 => textures.player4_goal
		}
	}
}

impl GetTexture for Tile {
	fn texture(&self, textures: &Textures) -> Texture2D {
		match self {
			Tile::MapBaseTile(tile) => tile.texture(textures),
			Tile::MapObjectTile(tile) => tile.texture(textures),
			Tile::PlayerTile(tile) => tile.texture(textures)
		}
	}
}
