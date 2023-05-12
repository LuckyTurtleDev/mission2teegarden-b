use crate::GetTexture;
pub use m3_map::tiles::MapBaseTile;
use m3_map::tiles::{ObjectTile, PlayerTile, Tiles};
use tetra::{graphics::Texture, Context};

///Store all Textures
pub struct Textures {
	grass: Texture,
	stone: Texture,
	player1_car: Texture,
	player2_car: Texture,
	player3_car: Texture,
	player4_car: Texture,
	global_goal: Texture,
}

impl Textures {
	///init all Textures
	pub fn init(ctx: &mut Context) -> Textures {
		Textures {
			grass: Texture::from_encoded(
				ctx,
				include_bytes!("../assets/img/BaseTiles/grass.png")
			)
			.unwrap(),
			stone: Texture::from_encoded(
				ctx,
				include_bytes!("../assets/img/ObjectTiles/stone.png")
			)
			.unwrap(),
			player1_car: Texture::from_encoded(
				ctx,
				include_bytes!("../assets/img/Player/player1_car.png")
			).unwrap(),
			player2_car: Texture::from_encoded(
				ctx,
				include_bytes!("../assets/img/Player/player2_car.png")
			).unwrap(),
			player3_car: Texture::from_encoded(
				ctx,
				include_bytes!("../assets/img/Player/player3_car.png")
			).unwrap(),
			player4_car: Texture::from_encoded(
				ctx,
				include_bytes!("../assets/img/Player/player4_car.png")
			)
			.unwrap(),
			global_goal: Texture::from_encoded(
				ctx,
				include_bytes!("../assets/img/Player/goal.png")
			)
			.unwrap()
			
		}
	}
}

impl<'a> GetTexture<'a> for MapBaseTile {
	///get Texture assioated with this Tile
	fn texture(&self, textures: &'a Textures) -> &'a Texture {
		match self {
			Self::Grass => &textures.grass
		}
	}
}

impl<'a> GetTexture<'a> for ObjectTile {
	///get Texture assioated with this Tile
	fn texture(&self, textures: &'a Textures) -> &'a Texture {
		match self {
			Self::Stone => &textures.stone
		}
	}
}

impl<'a> GetTexture<'a> for PlayerTile {
	///get Texture assioated with this Tile
	fn texture(&self, textures: &'a Textures) -> &'a Texture {
		match self {
			Self::Car1 => &textures.player1_car,
			Self::Car2 => &textures.player1_car,
			Self::Car3 => &textures.player1_car,
			Self::Car4 => &textures.player1_car,
			Self::GlobalGoal => &textures.global_goal,
		}
	}
}

impl<'a> GetTexture<'a> for Tile {
	match Tile
}

#[cfg(test)]
mod tests {
	use crate::tiles::Textures;
	use tetra::ContextBuilder;

	#[test]
	#[ignore] //can only run if gui is aviable. Ignore it, so ci does not fail
	///verify that all image files are valid and supported
	fn texture_init() {
		let mut ctx = ContextBuilder::new("test", 640, 480).build().unwrap();
		Textures::init(&mut ctx);
	}
}
