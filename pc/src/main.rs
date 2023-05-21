use log::{debug, info};
use m3_macro::include_map;
use m3_map::Map;
use macroquad::{math::Vec2, prelude::*, window, Window};
use my_env_logger_style::TimestampPrecision;
use once_cell::sync::Lazy;
use tiles::GetTexture;

mod tiles;
use tiles::TEXTURES;
use usb::Players;

mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

///store maps as String binary format
///call `Map::from_str()`
static LEVELS: Lazy<Vec<&str>> =
	Lazy::new(|| vec![include_map!("pc/assets/level/001.tmx")]);

struct GameState {
	level: Option<Map>,
	players: Players
}

impl GameState {
	fn new() -> GameState {
		Lazy::force(&TEXTURES);
		let level = Map::from_string(LEVELS.first().unwrap()).unwrap(); //tests check if map is vaild
		GameState {
			level: Some(level),
			players: usb::Players::init()
		}
	}

	///draw the current game state
	async fn draw(&self) {
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

	///update the current state.
	async fn update(&mut self) {
		let player_events = self.players.get_events();
		//use delta time, to avoid that the logic is effected by frame drops
	}
}

async fn run_game() {
	let mut game_state = GameState::new();
	loop {
		game_state.update().await;
		game_state.draw().await;
		next_frame().await
	}
}

fn main() {
	my_env_logger_style::set_timestamp_precision(TimestampPrecision::Disable);
	my_env_logger_style::just_log();
	info!("🚗 {CARGO_PKG_NAME}  v{CARGO_PKG_VERSION} 🚗");
	debug!("load level{:#?}", LEVELS[0]);
	Window::from_config(
		window::Conf {
			sample_count: 8, //anti-aliasing
			window_title: format!("{CARGO_PKG_NAME} v{CARGO_PKG_VERSION}"),
			high_dpi: true,
			#[cfg(not(debug_assertions))]
			fullscreen: true,
			..Default::default()
		},
		run_game()
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_bundeld_maps() {
		for (i, map) in LEVELS.iter().enumerate() {
			//test if map can be deserialize
			Map::from_string(map).expect("map with index {i} is not valid");
		}
	}
}
