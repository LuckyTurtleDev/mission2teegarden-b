use crate::{Activity, GameState, Phase, LEVELS};
use macroquad::{
	hash,
	prelude::*,
	ui::{root_ui, Skin}
};

fn get_menu_skin() -> Skin {
	let button_font_size = 40;
	{
		let window_style = root_ui()
			.style_builder()
			.background(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/selection_box_background.png"),
				None
			))
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
			.background_hovered(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_background.png"),
				None
			))
			.background_clicked(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_background.png"),
				None
			))
			.text_color(Color::from_rgba(180, 180, 100, 255))
			.font_size(button_font_size)
			.build();
		Skin {
			window_style,
			button_style,
			..root_ui().default_skin()
		}
	}
}

impl GameState {
	pub(crate) fn build_menu(&mut self) {
		clear_background(GRAY);
		let screen_width = screen_width();
		let screen_height = screen_height();
		let menu_size = vec2(screen_width * 0.5, screen_height * 0.8);
		let menu_position = vec2(
			(screen_width - menu_size.x) / 2.0,
			(screen_height - menu_size.y) / 2.0
		);
		let menu_skin = get_menu_skin();
		root_ui().push_skin(&menu_skin);
		root_ui().window(hash!(), menu_position, menu_size, |ui| {
			if ui.button(None, "Play") {
				debug!("Play pressed");
				self.activity = Activity::SelectLevel;
			}
			if ui.button(None, "Quit") {
				self.running = false;
			}
		});
	}

	pub(crate) fn build_level_menu(&mut self) {
		clear_background(GRAY);
		let screen_width = screen_width();
		let screen_height = screen_height();
		let menu_size = vec2(screen_width * 0.5, screen_height * 0.8);
		let menu_position = vec2(
			(screen_width - menu_size.x) / 2.0,
			(screen_height - menu_size.y) / 2.0
		);
		let menu_skin = get_menu_skin();
		root_ui().push_skin(&menu_skin);
		root_ui().window(hash!(), menu_position, menu_size, |ui| {
			if ui.button(None, "Tutorial") {
				debug!("Play pressed");
				self.activity = Activity::SelectLevel;
			}
			for x in 0..LEVELS.len() {
				if ui.button(None, format!("Level {}", x + 1)) {
					//self.game_run = Some(GameRun::new(x));
					self.activity = Activity::GameRound(Phase::Select);
				}
			}
			if ui.button(None, "Quit") {
				self.running = false;
			}
		});
	}
}