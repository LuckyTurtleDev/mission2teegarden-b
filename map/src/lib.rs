use num_enum::TryFromPrimitive;
use thiserror::Error;
use tiled::{Error, LayerType, Loader, TileLayer};
pub mod tiles;
use std::slice::Iter;
use tiles::{InvalidTileID, MapBaseTile};

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
	pub fn from_tmx(path: &str) -> Result<Self, MapError> {
		let map = Loader::new().load_tmx_map(path)?;
		let width: u8 = map.width.try_into().map_err(|_| MapError::ToWidth)?;
		let height: u8 = map.height.try_into().map_err(|_| MapError::ToHight)?;
		let mut base_layer = Vec::with_capacity(height as usize);
		for (i, layer) in map.layers().enumerate() {
			match i {
				1 => match layer.layer_type() {
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
						todo!()
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

	pub fn iter_map(&self) -> MapIter {
		let mut row_iter = self.base_layer.iter();
		let column_iter = row_iter.next().unwrap().iter();
		MapIter {
			row_iter,
			column_iter,
			x_pos: 0,
			y_pos: 0
		}
	}
}

pub struct MapIter<'a> {
	row_iter: Iter<'a, Vec<MapBaseTile>>,
	column_iter: Iter<'a, MapBaseTile>,
	x_pos: u8,
	y_pos: u8
}

//this is very ugly
impl<'a> Iterator for MapIter<'a> {
	type Item = (u8, u8, &'a MapBaseTile);
	fn next(&mut self) -> Option<Self::Item> {
		match self.column_iter.next() {
			Some(item) => {
				let x_pos = self.x_pos;
				self.x_pos += 1;
				Some((x_pos, self.y_pos, item))
			},
			None => match self.row_iter.next() {
				Some(row) => {
					self.column_iter = row.iter();
					self.x_pos = 0;
					let y_pos = self.y_pos;
					self.y_pos += 1;
					Some((self.x_pos, y_pos, self.column_iter.next().unwrap()))
				},
				None => None
			}
		}
	}
}
