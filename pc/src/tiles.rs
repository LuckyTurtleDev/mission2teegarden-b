use crate::GetTexture;
pub use m3_map::tiles::MapBaseTile;
use tetra::{graphics::Texture, Context};

///Store all Textures
pub struct Textures {
	grass: Texture
}

impl Textures {
	///init all Textures
	pub fn init(ctx: &mut Context) -> Textures {
		Textures {
			grass: Texture::from_encoded(
				ctx,
				include_bytes!("../assets/img/map-tiles/grass.png")
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
