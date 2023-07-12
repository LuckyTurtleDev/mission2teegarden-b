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

#[derive(Debug, Clone, Error)]
pub enum InvalidTile {
	#[error("invalid tiel id {0}")]
	InvalidId(u32),
	#[error("wrong Tile set. Found {0:?} expected {1:?}")]
	WrongTileset(String, String)
}

///Store all Tiles, with can be used at the map background
#[derive(
	Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize, TryFromPrimitive,
)]
#[repr(u8)]
pub enum MapBaseTile {
	//numbers must match them from the Tiled tilesets
	#[default]
	Grass = 0,
	GrassCornerSand = 1,
	HalfGrassSand = 2,
	Sand = 3,
	SandCornerGrass = 4,
	GrassDoubleCorner = 5,
	SandDoubleCorner = 6,
	StrangeGrass = 7,
	StrangeOuterCorner = 8,
	StrangeDoubleCorner = 9,
	StrangeHalf = 10,
	StrangeInnerCorner = 11
}

impl Passable for MapBaseTile {
	fn passable(&self) -> bool {
		match self {
			Self::Grass => true,
			Self::GrassCornerSand => true,
			Self::HalfGrassSand => true,
			Self::Sand => true,
			Self::SandCornerGrass => true,
			Self::SandDoubleCorner => true,
			Self::GrassDoubleCorner => true,
			Self::StrangeGrass => true,
			Self::StrangeOuterCorner => true,
			Self::StrangeInnerCorner => true,
			Self::StrangeHalf => true,
			Self::StrangeDoubleCorner => true
		}
	}
}

impl<'a> TryFrom<&tiled::LayerTile<'a>> for MapBaseTile {
	type Error = InvalidTile;
	fn try_from(value: &tiled::LayerTile<'a>) -> Result<MapBaseTile, Self::Error> {
		if value.get_tileset().name != "BaseTiles" {
			return Err(InvalidTile::WrongTileset(
				value.get_tileset().name.clone(),
				"BaseTiles".to_owned()
			));
		};
		let value_u8: u8 = value
			.id()
			.try_into()
			.map_err(|_| Self::Error::InvalidId(value.id()))?;
		Self::try_from_primitive(value_u8).map_err(|_| Self::Error::InvalidId(value.id()))
	}
}

///Store all Tiles, with can be place the layer above the background
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, TryFromPrimitive)]
#[repr(u8)]
pub enum ObjectTile {
	//numbers must match them from the Tiled tilesets
	Stone = 1
	StrangeStone = 2
}

impl Passable for ObjectTile {
	fn passable(&self) -> bool {
		match self {
			Self::Stone => false,
			Self::StrangeStone => false
		}
	}
}

impl<'a> TryFrom<&tiled::LayerTile<'a>> for ObjectTile {
	type Error = InvalidTile;
	fn try_from(value: &tiled::LayerTile<'a>) -> Result<ObjectTile, Self::Error> {
		if value.get_tileset().name != "ObjectTiles" {
			return Err(InvalidTile::WrongTileset(
				value.get_tileset().name.clone(),
				"BaseTiles".to_owned()
			));
		};
		let value_u8: u8 = value
			.id()
			.try_into()
			.map_err(|_| Self::Error::InvalidId(value.id()))?;
		Self::try_from_primitive(value_u8).map_err(|_| Self::Error::InvalidId(value.id()))
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

impl<'a> TryFrom<&tiled::LayerTile<'a>> for PlayerTile {
	type Error = InvalidTile;
	fn try_from(value: &tiled::LayerTile<'a>) -> Result<PlayerTile, Self::Error> {
		if value.get_tileset().name != "Player" {
			return Err(InvalidTile::WrongTileset(
				value.get_tileset().name.clone(),
				"BaseTiles".to_owned()
			));
		};
		let value_u8: u8 = value
			.id()
			.try_into()
			.map_err(|_| Self::Error::InvalidId(value.id()))?;
		Self::try_from_primitive(value_u8).map_err(|_| Self::Error::InvalidId(value.id()))
	}
}
