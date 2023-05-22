use log::{debug, info};
use m3_macro::include_map;
use m3_map::{Map, Orientation};
use macroquad::{prelude::*, window, Window};
use my_env_logger_style::TimestampPrecision;
use once_cell::sync::Lazy;
use m3_models::CardIter;

mod tiles;
use tiles::TEXTURES;
use usb::Players;

mod draw;
mod update;
mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

///store maps as String binary format
///call `Map::from_str()`
static LEVELS: Lazy<Vec<&str>> = Lazy::new(|| {
	vec![
		include_map!("pc/assets/level/001.tmx"),
		include_map!("pc/assets/level/002.tmx"),
		include_map!("pc/assets/level/003.tmx"),
	]
});

struct PlayerState <CarAction>{
	position: (u8, u8),
	orientation: Orientation,
	next_action: Option<CarAction>,
	card_iter: CardIter
}

struct GameRun {
	level: Map,
	player_states: Vec<PlayerState>
}

struct GameState {
	game_run: Option<GameRun>,
	input: Players
}

impl GameState {
	fn new() -> GameState {
		Lazy::force(&TEXTURES);
		let level = Map::from_string(LEVELS[0]).unwrap(); //tests check if map is vaild
		debug!("load level{:#?}", level);
		let player_states = level.iter_player().map(|f|
			PlayerState{
				position: f.position,
				orientation: f.orientation,
				next_action: None,
				card_iter: todo!()
			}
		).collect();
		let game_run = GameRun {
			level,
			player_states
		};

		GameState {
			game_run: Some(game_run),
			input: usb::Players::init()
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
			Map::from_string(map).expect(&format!("map with index {i} is not valid"));
		}
	}
}
