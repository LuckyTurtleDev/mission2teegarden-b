#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]

use log::{debug, info};
use m3_macro::include_map;
use m3_map::{Map, Orientation};
use m3_models::AvailableCards;
use macroquad::{prelude::*, window, Window};
use my_env_logger_style::TimestampPrecision;
use once_cell::sync::Lazy;
use sound::SoundPlayer;

mod assets;
mod cards_ev;
use cards_ev::{evaluate_cards, CarAction};
mod draw;
mod menu;
mod sound;
mod tiles;
use tiles::TEXTURES;
mod update;
use update::setup_players;
mod usb;
use usb::Players;

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

#[derive(PartialEq)]
enum Phase {
	Select,
	Drive
}

#[derive(PartialEq)]
enum Activity {
	Menu,
	SelectLevel,
	GameRound(Phase)
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Rotation {
	RotateLeft,
	RotateRight,
	NoRotation
}
struct PlayerState {
	position: (u8, u8),
	orientation: Orientation,
	next_action: Option<CarAction>,
	rotation: Rotation,
	/// either reached goal or out of map
	finished: bool,
	crashed: bool,
	/// Is `None` if the player has not create/send any solution.
	solution: Option<cards_ev::CardIter>
}

struct GameRun {
	level: Map,
	player_states: Vec<PlayerState>
}

struct GameState {
	sound_player: SoundPlayer,
	player_count: u8,
	activity: Activity,
	game_run: Option<GameRun>,
	input_players: Players,
	delta_time: f32,
	movement_time: f32,
	level_num: usize,
	running: bool
}

impl GameState {
	fn new() -> GameState {
		let sound_player = sound::SoundPlayer::new();
		Lazy::force(&TEXTURES);
		let mut level = Map::from_string(LEVELS[0]).unwrap(); //tests check if map is vaild
		level.cards = AvailableCards {
			left: 3,
			right: 3,
			motor_on: 2,
			motor_off: 2,
			wait: 9
		};
		debug!("load level{:#?}", level);
		let player_states = level
			.iter_player()
			.map(|f| PlayerState {
				position: f.position,
				orientation: f.orientation,
				next_action: None,
				rotation: Rotation::NoRotation,
				finished: false,
				crashed: false,
				solution: None
			})
			.collect();
		let game_run = GameRun {
			level,
			player_states
		};

		GameState {
			sound_player,
			activity: Activity::Menu,
			game_run: None,
			input_players: usb::Players::init(),
			delta_time: 0.0,
			movement_time: 0.5,
			player_count: 0,
			level_num: 0,
			running: true
		}
	}
}

async fn run_game() {
	let mut game_state = GameState::new();
	while game_state.running {
		game_state.sound_player.poll();
		//let events = game_state.input_players.get_events();
		match game_state.activity {
			Activity::GameRound(Phase::Select) => {
				//game_state.update(&events).await;
				game_state.draw().await;
				setup_players(&mut game_state)
			},
			Activity::GameRound(Phase::Drive) => {
				game_state.update().await;
				game_state.draw().await;
			},
			Activity::Menu => {
				game_state.build_menu().await;
			},
			Activity::SelectLevel => game_state.build_level_menu().await
		}
		log::warn!("hii");
		next_frame().await;
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
