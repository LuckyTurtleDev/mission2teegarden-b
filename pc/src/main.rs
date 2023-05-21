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

mod draw;
mod update;
mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

static LEVELS: Lazy<Vec<Map>> =
	Lazy::new(|| vec![include_map!("pc/assets/level/001.tmx")]);

struct GameState {
	level: Option<Map>,
	players: Players
}

impl GameState {
	fn new() -> GameState {
		Lazy::force(&TEXTURES);
		GameState {
			level: Some(LEVELS.first().unwrap().to_owned()),
			players: usb::Players::init()
		}
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
