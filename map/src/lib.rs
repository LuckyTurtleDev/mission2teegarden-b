use self_rust_tokenize::SelfRustTokenize;
use std::path::Path;
use thiserror::Error;
use tiled::{LayerType, Loader};

pub mod tiles;
use tiles::{InvalidTileID, MapBaseTile};

#[derive(Clone, Debug, SelfRustTokenize)]
pub struct Map {
	pub width: u8,
	pub height: u8,
	pub base_layer: Vec<Vec<MapBaseTile>>
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
	InvalidTileId(#[from] InvalidTileID)
}

impl Map {
	pub fn from_tmx(path: impl AsRef<Path>) -> Result<Self, MapError> {
		let map = Loader::new().load_tmx_map(path)?;
		let width: u8 = map.width.try_into().map_err(|_| MapError::ToWidth)?;
		let height: u8 = map.height.try_into().map_err(|_| MapError::ToHight)?;
		let mut base_layer = Vec::with_capacity(height as usize);
		for (i, layer) in map.layers().enumerate() {
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
				_ => return Err(MapError::ToManyLayers)
			}
		}
		Ok(Map {
			width,
			height,
			base_layer
		})
	}

	/// return an iterator over all BasteTiles and its x and y postion
	pub fn iter_base_layer(&self) -> impl Iterator<Item = (u8, u8, &MapBaseTile)> {
<<<<<<< HEAD
		self.base_layer.iter().enumerate().flat_map(|(x, y_vec)| {
			y_vec
||||||| parent of 1c5301d (add missing clone; fix iterator)
		self.base_layer.iter().enumerate().flat_map(|(y, x_vec)| {
			x_vec
=======
		self.base_layer.iter().enumerate().flat_map(|(x, x_vec)| {
			x_vec
>>>>>>> 1c5301d (add missing clone; fix iterator)
				.iter()
				.enumerate()
				.map(move |(y, item)| (x as u8, y as u8, item))
		})
	}
}
