use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub trait Passable {
	///return when car can boss ofer this object
	fn passable(&self) -> bool;
}

pub enum Tile {
	MapBaseTile(MapBaseTile),
	MapObjectTile(ObjectTile),
	PlayerTile(PlayerTile)
}

#[derive(Debug, Copy, Clone, Error)]
pub enum InvalidTileID {
	#[error("invalid tiel id {0}")]
	InvalidId(u32)
}

///Store all Tiles, with can be used at the map background
#[derive(
	Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize, TryFromPrimitive,
)]
#[repr(u8)]
pub enum MapBaseTile {
	//numbers must match them from the Tiled tilesets
	#[default]
	Grass = 0
}

impl Passable for MapBaseTile {
	fn passable(&self) -> bool {
		match self {
			Self::Grass => true
		}
	}
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum ObjectTile {
	//numbers must match them from the Tiled tilesets
	Stone = 1
}

impl Passable for ObjectTile {
	fn passable(&self) -> bool {
		match self {
			Self::Stone => false
		}
	}
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum PlayerTile {
	//numbers must match them from the Tiled tilesets
	Car1 = 0,
	Car2 = 1,
	Car3 = 2,
	Car4 = 3,
	/// goal, which can be used by all players
	GlobalGoal = 4,
	Goal1 = 5,
	Goal2 = 6,
	Goal3 = 7,
	Goal4 = 8
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
