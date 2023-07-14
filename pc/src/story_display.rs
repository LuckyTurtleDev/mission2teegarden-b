use crate::{assets::GetTexture, GameState};
use macroquad::{
	math::Vec2,
	prelude::{draw_rectangle, screen_height, screen_width, Color, WHITE},
	text::{draw_text_ex, measure_text, TextParams},
	texture::{draw_texture_ex, DrawTextureParams},
	window::next_frame
};
use mission2teegarden_b_map::story::Speech;
use mission2teegarden_b_models::ToPcGameEvent;

/// Check if any button is pressed.
fn button_pressed(events: &[Option<Vec<ToPcGameEvent>>; 4]) -> bool {
	for player_events in events.iter().flatten() {
		for event in player_events {
			if let ToPcGameEvent::KeyPressed(_key) = event {
				return true;
			}
		}
	}
	false
}

/// Get the font size that will scale with size of screen.
fn get_font_size(box_width: f32, box_height: f32) -> u16 {
	let mut font_size = (box_height / 7.0) as u16;
	let example_long_sentence =
		"I hope you all still know how to operate the robots, but just in case...";
	let ratio =
		box_width / measure_text(example_long_sentence, None, font_size, 1.0).width;
	if ratio < 1.0 {
		font_size = (font_size as f32 * ratio) as u16;
	}
	font_size
}

impl GameState {
	/// Display the story text for the current level.
	pub(crate) async fn display_speech(&mut self, speeches: &Vec<Speech>) {
		for speech in speeches {
			let mut events = self.input_players.get_events();
			let speech_lines: &Vec<&str> = &speech.text.split('\n').collect();
			let speech_line_groups: Vec<Vec<&str>> =
				speech_lines.chunks(5).map(|chunk| chunk.to_vec()).collect();
			while !button_pressed(&events) {
				for group in &speech_line_groups {
					events = self.input_players.get_events();
					while !button_pressed(&events) {
						let screen_width = screen_width();
						let screen_height = screen_height();
						if let Some(background) = &speech.background {
							let background_texture = background.texture();
							let draw_params = DrawTextureParams {
								dest_size: Some(Vec2::new(screen_width, screen_height)),
								..Default::default()
							};
							draw_texture_ex(
								background_texture,
								0.0,
								0.0,
								WHITE,
								draw_params
							);
						} else {
							self.draw().await;
						}
						let text_box_height = screen_height / 4.0;
						let text_box_margin_bottom = screen_height / 20.0;
						let text_box_position_y =
							screen_height - text_box_height - text_box_margin_bottom;
						// draw text box
						draw_rectangle(
							0.0,
							text_box_position_y,
							screen_width,
							text_box_height,
							Color::new(0.0, 0.0, 0.0, 0.8)
						);
						// draw narrator
						let mut profil_texture_width = 0.0;
						if let Some(profil) = &speech.profil {
							let profil_texture = profil.texture();
							profil_texture_width = profil_texture.width()
								* text_box_height / profil_texture
								.height();
							let draw_params = DrawTextureParams {
								dest_size: Some(Vec2::new(
									profil_texture_width,
									text_box_height
								)),
								..Default::default()
							};
							draw_texture_ex(
								profil_texture,
								0.0,
								text_box_position_y,
								WHITE,
								draw_params
							);
						}
						// draw text
						let font_size = get_font_size(
							screen_width - profil_texture_width,
							text_box_height
						);
						for (x, line) in group.iter().enumerate() {
							//let text_dim = measure_text(line, None, font_size, 1.0);
							let max_text_dim =
								measure_text(&speech.text, None, font_size, 1.0);
							let text_params = TextParams {
								font_size,
								color: WHITE,
								..Default::default()
							};
							draw_text_ex(
								line,
								profil_texture_width + 10.0,
								text_box_position_y
									+ max_text_dim.height * x as f32 + max_text_dim.offset_y
									+ 20.0,
								text_params
							);
						}
						events = self.input_players.get_events();
						next_frame().await;
					}
				}
			}
		}
	}
}
