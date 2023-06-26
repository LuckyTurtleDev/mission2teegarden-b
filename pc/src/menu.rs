use crate::{Activity, GameState, Phase, LEVELS};
use m3_models::{AvailableCards, Key, ToPcGameEvent, ToPypadeGameEvent};

use macroquad::{
	hash,
	prelude::*,
	ui::{root_ui, Skin}
};

const BUTTON_FONT_SIZE: u16 = 16;

fn get_button_skin() -> Skin {
	{
		let window_style = root_ui()
			.style_builder()
			.background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
			.margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
			.build();
		let button_style = root_ui()
			.style_builder()
			.background(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_background.png"),
				None
			))
			.background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
			.margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
			.text_color(Color::from_rgba(180, 180, 100, 255))
			.font_size(BUTTON_FONT_SIZE)
			.build();
		Skin {
			window_style,
			button_style,
			..root_ui().default_skin()
		}
	}
}

fn get_button_focused_skin() -> Skin {
	{
		let button_style = root_ui()
			.style_builder()
			.background(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_focused_background.png"),
				None
			))
			.background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
			.margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
			.text_color(Color::from_rgba(180, 180, 100, 255))
			.font_size(BUTTON_FONT_SIZE)
			.build();
		Skin {
			button_style,
			..root_ui().default_skin()
		}
	}
}

fn evaluate_events(
	events: &[Option<Vec<ToPcGameEvent>>; 4],
	enter_pressed: &mut bool
) -> i8 {
	for player_events in events.iter().flatten() {
		for event in player_events {
			if let ToPcGameEvent::KeyPressed(key) = event {
				if *key == Key::Down {
					return 1;
				} else if *key == Key::Up {
					return -1;
				} else if *key == Key::A {
					debug!("Enter pressed");
					*enter_pressed = true;
				}
			}
		}
	}
	0
}

impl GameState {
	pub(crate) async fn build_menu(&mut self) {
		let background_texture = load_texture("assets/img/Menu/menu_background.png")
			.await
			.unwrap();
		let screen_width = screen_width();
		let screen_height = screen_height();
		let menu_size = vec2(screen_width * 0.5, screen_height * 0.8);
		let menu_position = vec2(
			(screen_width - menu_size.x) / 2.0,
			(screen_height - menu_size.y) / 2.0
		);
		let mut button_focused_index = 0;

		let button_skin = get_button_skin();
		let button_focused_skin = get_button_focused_skin();
		let mut skin_1 = &button_focused_skin.clone();
		let mut skin_2 = &button_skin.clone();
		let mut enter_pressed = false;
		root_ui().push_skin(&skin_1);
		while self.activity == Activity::Menu && self.running {
			let events = self.input_players.get_events();
			button_focused_index = (button_focused_index
				+ evaluate_events(&events, &mut enter_pressed))
			.clamp(0, 1);
			draw_texture(background_texture, 0.0, 0.0, WHITE);
			root_ui().window(hash!(), menu_position, menu_size, |ui| {
				ui.pop_skin();
				if button_focused_index == 0 {
					if enter_pressed {
						self.activity = Activity::SelectLevel;
					}
					skin_1 = &button_focused_skin;
					skin_2 = &button_skin;
				} else {
					if enter_pressed {
						self.running = false;
					}
					skin_1 = &button_skin;
					skin_2 = &button_focused_skin;
				}
				ui.push_skin(&skin_1);

				if ui.button(vec2(140.0, 100.0), "Play") {
					self.activity = Activity::SelectLevel;
				}
				ui.pop_skin();
				ui.push_skin(&skin_2);
				if ui.button(vec2(140.0, 200.0), "Quit") {
					self.running = false;
				}
			});

			next_frame().await;
		}
	}

	pub(crate) async fn build_level_menu(&mut self) {
		let background_texture = load_texture("assets/img/Menu/menu_background.png")
			.await
			.unwrap();
		let screen_width = screen_width();
		let screen_height = screen_height();
		let menu_size = vec2(screen_width * 0.5, screen_height * 0.8);
		let menu_position = vec2(
			(screen_width - menu_size.x) / 2.0,
			(screen_height - menu_size.y) / 2.0
		);
		let button_skin = get_button_skin();
		let button_focused_skin = get_button_focused_skin();
		let mut enter_pressed = false;
		let mut button_focused_index = 0;

		while self.activity == Activity::SelectLevel && self.running {
			let events = self.input_players.get_events();
			button_focused_index = (button_focused_index
				+ evaluate_events(&events, &mut enter_pressed))
			.clamp(0, LEVELS.len() as i8);
			draw_texture(background_texture, 0.0, 0.0, WHITE);
			root_ui().window(hash!(), menu_position, menu_size, |ui| {
				for (i, _level) in LEVELS.clone().into_iter().enumerate() {
					ui.pop_skin();
					if button_focused_index == i as i8 {
						if enter_pressed {
							self.level_num = i;
							self.activity = Activity::GameRound(Phase::Introduction);
							self.sound_player.play_level_music();
						}
						let skin = &button_focused_skin.clone();
						ui.push_skin(skin);
					} else {
						let skin = &button_skin.clone();
						ui.push_skin(skin);
					}
					if ui.button(vec2(140.0, i as f32 * 50.0), format!("Level {}", i + 1))
					{
						self.activity = Activity::GameRound(Phase::Introduction);
					}
				}

				ui.pop_skin();
				if button_focused_index == LEVELS.len() as i8 {
					if enter_pressed {
						self.activity = Activity::Menu;
					}
					let skin = &button_focused_skin.clone();
					ui.push_skin(skin);
				} else {
					let skin = &button_skin.clone();
					ui.push_skin(skin);
				}
				if ui.button(vec2(140.0, LEVELS.len() as f32 * 50.0), "Back") {
					self.activity = Activity::Menu;
				}
			});
			next_frame().await;
		}
	}
}
