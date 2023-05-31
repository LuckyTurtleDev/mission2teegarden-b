use crate::{tiles::GetTexture, GameState, TEXTURES};
use macroquad::{math::Vec2, prelude::*};

impl GameState {
	///draw the current game state
	pub async fn draw(&self) {
		clear_background(BLACK);
		let screen_width = screen_width();
		let screen_height = screen_height();

		match &self.game_run {
			None => todo!(),
			Some(game_run) => {
				let dest_size = (screen_width / game_run.level.width as f32)
					.min(screen_height / game_run.level.height as f32);
				//center map, by using offset
				let offset_x =
					(screen_width - dest_size * game_run.level.width as f32) / 2.0;
				let offset_y =
					(screen_height - dest_size * game_run.level.height as f32) / 2.0;
				for (x, y, tile) in game_run.level.iter_all() {
					let texture = tile.texture(&TEXTURES);
					let draw_params = DrawTextureParams {
						dest_size: Some(Vec2::new(dest_size, dest_size)),
						..Default::default()
					};
					draw_texture_ex(
						texture,
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
