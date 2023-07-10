use crate::{update::init_level, Activity, GameState, Phase, LEVELS};
use mission2teegarden_b_map::{Map, MAP_FILE_EXTENSION};
use mission2teegarden_b_models::{Key, ToPcGameEvent};

use macroquad::{
	hash,
	prelude::*,
	ui::{root_ui, widgets, Skin}
};
use rfd::FileDialog;

fn get_button_font_size(container_height: f32) -> u16 {
	let font_size = 20;
	let max_text = "###########";
	let text_dim = measure_text(max_text, None, font_size, 1.0);
	let relative_width =
		(container_height * 0.75) / (text_dim.height + text_dim.offset_y);
	font_size * relative_width as u16
}

fn get_button_skin(font_size: u16) -> Skin {
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
		.font_size(font_size)
		.build();
	Skin {
		window_style,
		button_style,
		..root_ui().default_skin()
	}
}

fn get_button_focused_skin(font_size: u16) -> Skin {
	let button_style = root_ui()
		.style_builder()
		.background(Image::from_file_with_format(
			include_bytes!("../assets/img/Menu/button_focused_background.png"),
			None
		))
		.build();
	let button_style = root_ui()
		.style_builder()
		.background(Image::from_file_with_format(
			include_bytes!("../assets/img/Menu/button_focused_background.png"),
			None
		))
		.background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
		.margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
		.text_color(Color::from_rgba(180, 180, 100, 255))
		.font_size(font_size)
		.build();
	Skin {
		button_style,
		..root_ui().default_skin()
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
		let mut button_focused_index = 0;
		let mut enter_pressed = false;

		while self.activity == Activity::Menu && self.running {
			let events = self.input_players.get_events();
			let screen_width = screen_width();
			let screen_height = screen_height();
			let button_size = vec2(screen_width / 3.0, screen_height / 10.0);
			let font_size = get_button_font_size(button_size.y);
			let button_skin = get_button_skin(font_size);
			let button_focused_skin = get_button_focused_skin(font_size);
			let mut skin_1 = &button_focused_skin.clone();
			let mut skin_2 = &button_skin.clone();

			root_ui().push_skin(skin_1);
			let relative_size = (screen_width / background_texture.width())
				.max(screen_height / background_texture.height());
			let background_dim = vec2(
				relative_size * background_texture.width(),
				relative_size * background_texture.height()
			);
			let draw_params = DrawTextureParams {
				dest_size: Some(background_dim),
				..Default::default()
			};
			draw_texture_ex(
				background_texture,
				-((background_dim.x - screen_width) / 2.0),
				-((background_dim.y - screen_height) / 2.0),
				WHITE,
				draw_params
			);
			button_focused_index = (button_focused_index
				+ evaluate_events(&events, &mut enter_pressed))
			.clamp(0, 1);
			// It does not work when group has same size as screen
			root_ui().group(
				hash!(),
				vec2(screen_width - 1.0, screen_height - 1.0),
				|ui| {
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
					ui.push_skin(skin_1);

					let play_button = widgets::Button::new("Play")
						.position(vec2(
							(screen_width - button_size.x) / 2.0,
							(screen_height - button_size.y) / 2.0 - 1.5 * button_size.y
						))
						.size(button_size)
						.ui(ui);
					if play_button {
						self.activity = Activity::SelectLevel;
					}
					ui.pop_skin();
					ui.push_skin(skin_2);
					let quit_button = widgets::Button::new("Quit")
						.position(vec2(
							(screen_width - button_size.x) / 2.0,
							(screen_height - button_size.y) / 2.0 + 1.5 * button_size.y
						))
						.size(button_size)
						.ui(ui);
					if quit_button {
						self.running = false;
					}
				}
			);

			next_frame().await;
		}
	}

	pub(crate) async fn build_level_menu(&mut self) {
		let background_texture = load_texture("assets/img/Menu/menu_background.png")
			.await
			.unwrap();
		let mut enter_pressed = false;
		let mut button_focused_index = 0;

		while self.activity == Activity::SelectLevel && self.running {
			let events = self.input_players.get_events();
			let screen_width = screen_width();
			let screen_height = screen_height();
			let button_size = vec2(screen_width / 3.0, screen_height / 12.0);
			let font_size = get_button_font_size(button_size.y);
			// "wrapper" which contains buttons, only for position and size
			let level_wrapper_height = screen_height * 0.7;
			let wrapper_offset_top_bottom = (screen_height - level_wrapper_height) / 2.0;
			// distance between buttons
			let button_offset = level_wrapper_height / (LEVELS.len() + 2) as f32;
			let button_skin = get_button_skin(font_size);
			let button_focused_skin = get_button_focused_skin(font_size);
			button_focused_index = (button_focused_index
				+ evaluate_events(&events, &mut enter_pressed))
			.clamp(0, (LEVELS.len() + 1) as i8);
			let relative_size = (screen_width / background_texture.width())
				.max(screen_height / background_texture.height());
			let background_dim = vec2(
				relative_size * background_texture.width(),
				relative_size * background_texture.height()
			);
			let draw_params = DrawTextureParams {
				dest_size: Some(background_dim),
				..Default::default()
			};
			draw_texture_ex(
				background_texture,
				-((background_dim.x - screen_width) / 2.0),
				-((background_dim.y - screen_height) / 2.0),
				WHITE,
				draw_params
			);
			root_ui().group(
				hash!(),
				vec2(screen_width - 1.0, screen_height - 1.0),
				|ui| {
					for (i, _level) in LEVELS.clone().into_iter().enumerate() {
						ui.pop_skin();
						if button_focused_index == i as i8 {
							if enter_pressed {
								self.level_num = i;
								self.activity = Activity::GameRound(Phase::Introduction);
								let level =
									Map::from_string(LEVELS[self.level_num]).unwrap();
								init_level(self, level);
								self.sound_player.play_level_music();
							}
							let skin = &button_focused_skin.clone();
							ui.push_skin(skin);
						} else {
							let skin = &button_skin.clone();
							ui.push_skin(skin);
						}
						let level_button =
							widgets::Button::new(format!("Level {}", i + 1))
								.position(vec2(
									(screen_width - button_size.x) / 2.0,
									wrapper_offset_top_bottom + i as f32 * button_offset
								))
								.size(button_size)
								.ui(ui);
						if level_button {
							self.activity = Activity::GameRound(Phase::Introduction);
						}
					}

					ui.pop_skin();
					if button_focused_index == LEVELS.len() as i8 {
						if enter_pressed {
							self.activity = Activity::GameRound(Phase::Introduction);
							let file = FileDialog::new()
								.add_filter("level", &["tmx", MAP_FILE_EXTENSION])
								.pick_file()
								.unwrap_or_default();
							let level = Map::load_from_file(file);
							match level {
								Ok(level) => {
									init_level(self, level);
									self.sound_player.play_level_music();
								},
								Err(e) => panic!("Problem loading File: {:?}", e)
							}
						}
						let skin = &button_focused_skin.clone();
						ui.push_skin(skin);
					} else {
						let skin = &button_skin.clone();
						ui.push_skin(skin);
					}
					let load_level_button = widgets::Button::new("Import Level")
						.position(vec2(
							(screen_width - button_size.x) / 2.0,
							wrapper_offset_top_bottom
								+ LEVELS.len() as f32 * button_offset
						))
						.size(button_size)
						.ui(ui);
					if load_level_button {
						self.activity = Activity::GameRound(Phase::Introduction);
						let level = Map::from_string(LEVELS[self.level_num]).unwrap();
						init_level(self, level);
						self.sound_player.play_level_music();
					}

					ui.pop_skin();
					if button_focused_index == (LEVELS.len() + 1) as i8 {
						if enter_pressed {
							self.activity = Activity::Menu;
						}
						let skin = &button_focused_skin.clone();
						ui.push_skin(skin);
					} else {
						let skin = &button_skin.clone();
						ui.push_skin(skin);
					}
					let back_button = widgets::Button::new("Back")
						.position(vec2(
							(screen_width - button_size.x) / 2.0,
							wrapper_offset_top_bottom
								+ (LEVELS.len() + 1) as f32 * button_offset
						))
						.size(button_size)
						.ui(ui);
					if back_button {
						self.activity = Activity::Menu;
					}
				}
			);
			next_frame().await;
		}
	}
}
