#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]

use log::{debug, info};
use m3_macro::include_map;
use m3_map::{Map, Orientation};
use macroquad::{prelude::*, window, Window};
use my_env_logger_style::TimestampPrecision;
use once_cell::sync::Lazy;
//use m3_models::CardIter;

mod cards_ev;
mod tiles;
use cards_ev::CarAction;
use tiles::TEXTURES;
use usb::Players;

use crate::cards_ev::evaluate_cards;

use m3_models::Card;
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RotationPoint {
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
	NoRotation
}

struct Rotation {
	pivot: RotationPoint,
	sign: i8
}

struct PlayerState {
	position: (u8, u8),
	orientation: Orientation,
	next_action: Option<CarAction>,
	rotation: Rotation,
	card_iter: cards_ev::CardIter
}

struct GameRun {
	level: Map,
	player_states: Vec<PlayerState>
}

struct GameState {
	game_run: Option<GameRun>,
	input_players: Players,
	delta_time: f32,
	movement_time: f32
}

impl GameState {
	fn new() -> GameState {
		let cards = vec![
			Card::MotorOn,
			Card::Wait(3),
			Card::Left,
			Card::Wait(2),
			Card::MotorOff,
		]; //TODO: load cards from pybadge
		Lazy::force(&TEXTURES);
		let level = Map::from_string(LEVELS[0]).unwrap(); //tests check if map is vaild
		debug!("load level{:#?}", level);
		let player_states = level
			.iter_player()
			.map(|f| PlayerState {
				position: f.position,
				orientation: f.orientation,
				next_action: None,
				rotation: Rotation {
					pivot: RotationPoint::NoRotation,
					sign: 1
				},
				card_iter: evaluate_cards(cards.clone())
			})
			.collect();
		let game_run = GameRun {
			level,
			player_states
		};

		GameState {
			game_run: Some(game_run),
			input_players: usb::Players::init(),
			delta_time: 0.0,
			movement_time: 0.5
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
