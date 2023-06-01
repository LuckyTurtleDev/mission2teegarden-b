use crate::{tiles::GetTexture, GameState, TEXTURES, RotationPoint};
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
				let map_offset_x =
					(screen_width - dest_size * game_run.level.width as f32) / 2.0;
				let map_offset_y =
					(screen_height - dest_size * game_run.level.height as f32) / 2.0;
				for (x, y, tile) in game_run.level.iter_all() {
					let texture = tile.texture(&TEXTURES);
					let draw_params = DrawTextureParams {
						dest_size: Some(Vec2::new(dest_size, dest_size)),
						..Default::default()
					};
					draw_texture_ex(
						texture,
						x as f32 * dest_size + map_offset_x,
						y as f32 * dest_size + map_offset_y,
						//This param can filter colors.
						//Set every value to 1 to keep all colors, by using WHITE
						WHITE,
						draw_params
					);
				}

				//draw players
				let player_textures = TEXTURES.get_player_textures();
                for (x, player) in game_run.level.iter_player().enumerate() {
					let texture = player_textures[x];
					// Car drives forward
                    if game_run.player_states[x].next_rotation_point == RotationPoint::NoRotation {
						let relative_pos_x = (game_run.player_states[x].position.0 as f32 - player.position.0 as f32) * dest_size / (self.movement_time/self.delta_time);
                        let relative_pos_y = (game_run.player_states[x].position.1 as f32 - player.position.1 as f32) * dest_size / (self.movement_time/self.delta_time);
                        let draw_params = DrawTextureParams {
							dest_size: Some(Vec2::new(dest_size, dest_size)),
							..Default::default()
						};
						//println!("PlayerState {} {}", game_run.player_states[x].position.0, game_run.player_states[x].position.1);
						/*println!("Position X + Offset X: ({} + {} = {})", player.position.0 as f32 * dest_size, offset_x, player.position.0 as f32 + offset_x);
						println!("Position Y + Offset Y: ({} + {} = {})", player.position.1 as f32 * dest_size, offset_y, player.position.1 as f32 + offset_y);*/
                        draw_texture_ex(
                            texture,
                        	player.position.0 as f32 * dest_size + relative_pos_x + map_offset_x,
                            player.position.1 as f32 * dest_size + relative_pos_y + map_offset_y,
                            WHITE,
                            draw_params
                        );
					// Car makes a turn	
                    } else {
						let pivotXY = match game_run.player_states[x].next_rotation_point {
							RotationPoint::TopLeft => Vec2::new(player.position.0 as f32, player.position.1 as f32),
							RotationPoint::TopRight => Vec2::new(player.position.0 as f32 + dest_size, player.position.1 as f32),
							RotationPoint::BottomLeft => Vec2::new(player.position.0 as f32, player.position.1 as f32 + dest_size),
							RotationPoint::BottomRight => Vec2::new(player.position.0 as f32 + dest_size, player.position.1 as f32 + dest_size),
							// Never happens
							RotationPoint::NoRotation => Vec2::new(0.0, 0.0)
						};
						let angle = 90.0 / (self.movement_time/self.delta_time);
						let draw_params = DrawTextureParams {
							dest_size: Some(Vec2::new(dest_size, dest_size)),
							rotation: angle,
							pivot: Some(pivotXY),
							..Default::default()
						};
						draw_texture_ex(
							texture,
							player.position.0 as f32,
							player.position.1 as f32,
							WHITE,
							draw_params
						);
					}
				}
			}
		}
        
	}
}
