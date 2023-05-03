use num_enum::TryFromPrimitive;
use self_rust_tokenize::SelfRustTokenize;
use thiserror::Error;

///Store all Tiles, with can be used at the map background
#[derive(
	Clone, Copy, Debug, Default, Eq, SelfRustTokenize, PartialEq, TryFromPrimitive,
)]
#[repr(u8)]
pub enum MapBaseTile {
	//numbers must match them from the Tiled tilesets
	#[default]
	Grass = 1,
	Puddle = 2
}

#[derive(Debug, Copy, Clone, Error)]
pub enum InvalidTileID {
	#[error("invalid tiel id {0}")]
	InvalidId(u32)
}

impl TryFrom<u32> for MapBaseTile {
	type Error = InvalidTileID;
	fn try_from(value: u32) -> Result<MapBaseTile, Self::Error> {
		let value_u8: u8 = value
			.try_into()
			.map_err(|_| Self::Error::InvalidId(value))?;
		Self::try_from_primitive(value_u8).map_err(|_| Self::Error::InvalidId(value))
	}
}
