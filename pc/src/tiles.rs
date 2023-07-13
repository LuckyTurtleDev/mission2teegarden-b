use crate::assets::{GetTexture, TEXTURES};
use macroquad::texture::Texture2D;
pub(crate) use mission2teegarden_b_map::tiles::MapBaseTile;
use mission2teegarden_b_map::{
	story::{Background, Character},
	tiles::{ObjectTile, PlayerTile, Tile}
};

impl GetTexture for MapBaseTile {
	///get Texture assioated with this Tile
	fn texture(&self) -> Texture2D {
		match self {
			Self::Grass => TEXTURES.grass,
			Self::GrassCornerSand => TEXTURES.grass_corner_sand,
			Self::HalfGrassSand => TEXTURES.half_grass_sand,
			Self::Sand => TEXTURES.sand,
			Self::SandCornerGrass => TEXTURES.sand_corner_grass,
			Self::SandDoubleCorner => TEXTURES.sand_double_corner_grass,
			Self::GrassDoubleCorner => TEXTURES.grass_double_corner_sand,
			Self::StrangeGrass => TEXTURES.strange_grass,
			Self::StrangeOuterCorner => TEXTURES.strange_grass_corner,
			Self::StrangeInnerCorner => TEXTURES.strange_grass_inner_corner,
			Self::StrangeDoubleCorner => TEXTURES.strange_grass_double_corner,
			Self::StrangeHalf => TEXTURES.strange_grass_half
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
