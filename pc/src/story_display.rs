use log::debug;
use m3_models::ToPcGameEvent;
use macroquad::{
	hash,
	math::Vec2,
	prelude::{screen_height, screen_width, vec2, BLACK, WHITE},
	text::{draw_text_ex, measure_text, TextParams},
	texture::{draw_texture_ex, DrawTextureParams},
	ui::{root_ui, widgets},
	window::{clear_background, next_frame}
};

use crate::{tiles::GetTexture, Activity, GameState, Phase};

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

impl GameState {
	pub(crate) async fn display_speech(&mut self) {
		if let Some(ref mut game_run) = self.game_run {
			let mut speeches = &game_run.level.story.pre_level;
			if let Activity::GameRound(Phase::Finish) = self.activity {
				speeches = &game_run.level.story.after_level;
			}
			let font_size = 40;
			for speech in speeches {
				let mut events = self.input_players.get_events();
				let text_dim = measure_text(&speech.text, None, font_size, 1.0);
				while !button_pressed(&events) {
					let screen_width = screen_width();
					if let Some(background) = &speech.background {
						let background_texture = background.texture();
						let draw_params = DrawTextureParams {
							dest_size: Some(Vec2::new(
								screen_width,
								background_texture.height() * screen_width
									/ background_texture.width()
							)),
							..Default::default()
						};
						draw_texture_ex(
							background.texture(),
							0.0,
							0.0,
							WHITE,
							draw_params
						);
					}
					let speech_lines: &Vec<&str> = &speech.text.split('\n').collect();
					debug!("Number of Lines: {}", speech_lines.len());
					for (x, line) in speech_lines.iter().enumerate() {
						let text_params = TextParams {
							font_size,
							color: WHITE,
							..Default::default()
						};
						draw_text_ex(
							line,
							0.0,
							500.0 + x as f32 * text_dim.height,
							text_params
						);
					}
					events = self.input_players.get_events();
					next_frame().await;
				}
			}
			self.activity = Activity::GameRound(Phase::Select);
		}
	}
}
