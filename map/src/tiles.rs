use num_enum::TryFromPrimitive;
use self_rust_tokenize::SelfRustTokenize;
use thiserror::Error;

#[derive(Debug, Copy, Clone, Error)]
pub enum InvalidTileID {
	#[error("invalid tiel id {0}")]
	InvalidId(u32)
}

///Store all Tiles, with can be used at the map background
#[derive(
	Clone, Copy, Debug, Default, Eq, SelfRustTokenize, PartialEq, TryFromPrimitive,
)]
#[repr(u8)]
pub enum MapBaseTile {
	//numbers must match them from the Tiled tilesets
	#[default]
	Grass = 0
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

///Store all Tiles, with can be place the layer above the background
#[derive(Clone, Copy, Debug, Eq, SelfRustTokenize, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ObjectTile {
	//numbers must match them from the Tiled tilesets
	Stone = 1
}

impl TryFrom<u32> for ObjectTile {
	type Error = InvalidTileID;
	fn try_from(value: u32) -> Result<ObjectTile, Self::Error> {
		let value_u8: u8 = value
			.try_into()
			.map_err(|_| Self::Error::InvalidId(value))?;
		Self::try_from_primitive(value_u8).map_err(|_| Self::Error::InvalidId(value))
	}
}

///Store all Tiles, with can be place the layer above the background
#[derive(Clone, Copy, Debug, Eq, SelfRustTokenize, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum PlayerTile {
	//numbers must match them from the Tiled tilesets
	Car1 = 0,
	Car2 = 1,
	Car3 = 2,
	Car4 = 3,
	//goal, which can be used by all players
	GlobalGoal = 4
}

impl TryFrom<u32> for PlayerTile {
	type Error = InvalidTileID;
	fn try_from(value: u32) -> Result<PlayerTile, Self::Error> {
		let value_u8: u8 = value
			.try_into()
			.map_err(|_| Self::Error::InvalidId(value))?;
		Self::try_from_primitive(value_u8).map_err(|_| Self::Error::InvalidId(value))
	}
}
