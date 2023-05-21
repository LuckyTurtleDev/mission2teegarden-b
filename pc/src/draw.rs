use crate::{tiles::GetTexture, GameState, TEXTURES};
use macroquad::{math::Vec2, prelude::*};

impl GameState {
	///draw the current game state
	pub async fn draw(&self) {
		clear_background(BLACK);
		let screen_width = screen_width();
		let screen_height = screen_height();

		match &self.level {
			None => todo!(),
			Some(map) => {
				let dest_size = (screen_width / map.width as f32)
					.min(screen_height / map.height as f32);
				//center map, by using offset
				let offset_x = (screen_width - dest_size * map.width as f32) / 2.0;
				let offset_y = (screen_height - dest_size * map.height as f32) / 2.0;
				for (x, y, tile) in map.iter_all() {
					let texture = tile.texture(&TEXTURES);
					let draw_params = DrawTextureParams {
						dest_size: Some(Vec2::new(dest_size, dest_size)),
						..Default::default()
					};
					draw_texture_ex(
						texture.clone(),
						x as f32 * dest_size + offset_x,
						y as f32 * dest_size + offset_y,
						//This param can filter colors.
						//Set every value to 1 to keep all colors, by using WHITE
						WHITE,
						draw_params
					);
				}
			}
		}
	}
}
