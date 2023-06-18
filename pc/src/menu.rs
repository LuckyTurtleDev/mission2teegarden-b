use std::vec;

use crate::{update::init_level, Activity, GameState, Phase, LEVELS};
use m3_models::{Key, ToPcGameEvent};

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
			/*.background(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/selection_box_background.png"),
				None
			))*/
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
			/*.background_hovered(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_hovered_background.png"),
				None
			))
			.background_clicked(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_clicked_background.png"),
				None
			))*/
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
		let window_style = root_ui()
			.style_builder()
			/*.background(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/selection_box_background.png"),
				None
			))*/
			.background_margin(RectOffset::new(15.0, 15.0, 7.5, 7.5))
			.margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
			.build();
		let button_style = root_ui()
			.style_builder()
			.background(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_focused_background.png"),
				None
			))
			.background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
			.margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
			/*.background_hovered(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_hovered_background.png"),
				None
			))
			.background_clicked(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_clicked_background.png"),
				None
			))*/
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
					return -1;
				} else if *key == Key::Up {
					return 1;
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
		clear_background(GRAY);
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
			button_focused_index += evaluate_events(&events, &mut enter_pressed) % 2;
			clear_background(BLACK);
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

				if ui.button(vec2(200.0 - 60.0, 100.0), "Play") {
					self.activity = Activity::SelectLevel;
				}
				ui.pop_skin();
				ui.push_skin(&skin_2);
				if ui.button(vec2(200.0 - 60.0, 200.0), "Quit") {
					self.running = false;
				}
			});

			next_frame().await;
		}
	}

	pub(crate) async fn build_level_menu(&mut self) {
		clear_background(BLACK);
		let screen_width = screen_width();
		let screen_height = screen_height();
		let menu_size = vec2(screen_width * 0.5, screen_height * 0.8);
		let menu_position = vec2(
			(screen_width - menu_size.x) / 2.0,
			(screen_height - menu_size.y) / 2.0
		);
		/*let menu_skin = get_std_menu_skin();
		root_ui().push_skin(&menu_skin);*/
		while self.activity == Activity::SelectLevel {
			root_ui().window(hash!(), menu_position, menu_size, |ui| {
				if ui.button(None, "Tutorial") {
					debug!("Play pressed");
					todo!("Tutorial");
				}
				for x in 0..LEVELS.len() {
					if ui.button(None, format!("Level {}", x + 1)) {
						self.level_num = x;
						init_level(self);
						self.activity = Activity::GameRound(Phase::Select);
					}
				}
				if ui.button(None, "Quit") {
					self.running = false;
				}
			});
			next_frame().await;
		}
	}
}
