use crate::{assets::GetTexture, GameState, Orientation, Rotation, TEXTURES};
use macroquad::{math::Vec2, prelude::*};

impl GameState {
	///draw crash fire

	///draw the current game state
	pub(crate) async fn draw(&mut self) {
		clear_background(BLACK);
		let screen_width = screen_width();
		let screen_height = screen_height();
		match &self.game_run {
			None => todo!(),
			Some(game_run) => {
				//draw map
				let dest_size = (screen_width / game_run.level.width as f32)
					.min(screen_height / game_run.level.height as f32);
				//center map, by using offset
				let map_offset_x =
					(screen_width - dest_size * game_run.level.width as f32) / 2.0;
				let map_offset_y =
					(screen_height - dest_size * game_run.level.height as f32) / 2.0;
				for (x, y, tile, orientation) in game_run.level.iter_all() {
					let texture = tile.texture();
					let draw_params = DrawTextureParams {
						dest_size: Some(Vec2::new(dest_size, dest_size)),
						rotation: orientation.rotation(),
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
					debug!(
						"Player is some: {}",
						self.input_players.players[x].is_some()
					);
					if self.input_players.players[x].is_some() {
						if !game_run.player_states[x].finished {
							let texture = player_textures[x];
							// Car drives forward
							if game_run.player_states[x].rotation == Rotation::NoRotation
							{
								let current_pos_x =
									if game_run.player_states[x].position.0 == 255 {
										-1
									} else {
										game_run.player_states[x].position.0 as i16
									};
								let current_pos_y =
									if game_run.player_states[x].position.1 == 255 {
										-1
									} else {
										game_run.player_states[x].position.1 as i16
									};
								let relative_pos_x =
									(current_pos_x as f32 - player.position.0 as f32)
										* dest_size / (self.movement_time / self.delta_time);
								let relative_pos_y =
									(current_pos_y as f32 - player.position.1 as f32)
										* dest_size / (self.movement_time / self.delta_time);
								let rotation: f32 = match player.orientation {
									Orientation::North => 0.0,
									Orientation::East => 90.0,
									Orientation::South => 180.0,
									Orientation::West => 270.0
								};
								let draw_params = DrawTextureParams {
									dest_size: Some(Vec2::new(dest_size, dest_size)),
									rotation: rotation.to_radians(),
									..Default::default()
								};
								let pos_x = player.position.0 as f32 * dest_size
									+ relative_pos_x + map_offset_x;
								let pos_y = player.position.1 as f32 * dest_size
									+ relative_pos_y + map_offset_y;
								draw_texture_ex(
									texture,
									pos_x,
									pos_y,
									WHITE,
									draw_params
								);
								if game_run.player_states[x].crashed {
									if let Some(emitter) = self.animation_emitter.as_mut()
									{
										emitter.draw(vec2(
											pos_x + dest_size / 2.0,
											pos_y + dest_size / 2.0
										));
									}
								}
							// Car makes a turn
							} else {
								let sign = match game_run.player_states[x].rotation {
									Rotation::RotateLeft => -1.0,
									Rotation::RotateRight => 1.0,
									_ => unreachable!()
								};
								let angle_offset = match player.orientation {
									Orientation::North => 0.0,
									Orientation::East => 90.0,
									Orientation::South => 180.0,
									Orientation::West => 270.0
								};
								let angle: f32 = (90.0
									/ (self.movement_time / self.delta_time))
									* sign + angle_offset;
								let draw_params = DrawTextureParams {
									dest_size: Some(Vec2::new(dest_size, dest_size)),
									rotation: angle.to_radians(),
									..Default::default()
								};
								let pos_x =
									player.position.0 as f32 * dest_size + map_offset_x;
								let pos_y =
									player.position.1 as f32 * dest_size + map_offset_y;
								draw_texture_ex(
									texture,
									pos_x,
									pos_y,
									WHITE,
									draw_params
								);
								if game_run.player_states[x].crashed {
									if let Some(emitter) = self.animation_emitter.as_mut()
									{
										emitter.draw(vec2(
											pos_x + dest_size / 2.0,
											pos_y + dest_size
										));
									}
								}
							}
						}
					}
				}
				//draw map border
				//top of map
				draw_rectangle(
					map_offset_x,
					map_offset_y - dest_size,
					dest_size * game_run.level.width as f32,
					dest_size,
					BLACK
				);
				// left of map
				draw_rectangle(
					map_offset_x - dest_size,
					map_offset_y,
					dest_size,
					dest_size * game_run.level.height as f32,
					BLACK
				);
				// right of map
				draw_rectangle(
					map_offset_x + dest_size * game_run.level.width as f32,
					map_offset_y,
					dest_size,
					dest_size * game_run.level.height as f32,
					BLACK
				);
				// bottom of map
				draw_rectangle(
					map_offset_x,
					map_offset_y + dest_size * game_run.level.height as f32,
					dest_size * game_run.level.width as f32,
					dest_size,
					BLACK
				);
			}
		}
	}
}
