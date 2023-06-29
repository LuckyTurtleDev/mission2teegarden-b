#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]

use assets::LEVELS;
use clap::Parser;
use log::info;
use m3_map::{Map, Orientation};
use m3_models::{AvailableCards, ToPypadeGameEvent};
use macroquad::{prelude::*, window, Window};
use macroquad_particles::Emitter;
use my_env_logger_style::TimestampPrecision;
use once_cell::sync::Lazy;
use sound::SoundPlayer;
use std::{io::IsTerminal, process};

mod assets;
use assets::TEXTURES;
mod cards_ev;
use cards_ev::{evaluate_cards, CarAction};
mod animations;
mod draw;
mod menu;
mod sound;
mod story_display;
pub mod tiles;
mod update;
use update::{activate_players, init_level, setup_players};
mod usb;
use usb::Players;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq)]
enum Phase {
	Introduction,
	Select,
	Drive,
	Finish
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

#[derive(Debug)]
struct PlayerState {
	position: (u8, u8),
	orientation: Orientation,
	next_action: Option<CarAction>,
	rotation: Rotation,
	/// either reached goal or out of map
	finished: bool,
	crashed: bool,
	out_of_map: bool,
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
	animation_emitter: Option<Emitter>,
	running: bool
}

impl GameState {
	fn new() -> GameState {
		let sound_player = sound::SoundPlayer::new();
		Lazy::force(&TEXTURES);

		GameState {
			sound_player,
			activity: Activity::Menu,
			game_run: None,
			input_players: usb::Players::init(),
			delta_time: 0.0,
			movement_time: 0.5,
			player_count: 0,
			level_num: 0,
			animation_emitter: None,
			running: true
		}
	}
}

async fn run_game() {
	let mut game_state = GameState::new();
	while game_state.running {
		game_state.sound_player.poll();
		match game_state.activity {
			Activity::GameRound(Phase::Introduction) => {
				game_state
					.display_speech(
						&game_state
							.game_run
							.as_ref()
							.unwrap()
							.level
							.story
							.pre_level
							.clone()
					)
					.await;
				activate_players(
					&mut game_state,
					ToPypadeGameEvent::NewLevel(AvailableCards {
						left: 3,
						right: 3,
						motor_on: 2,
						motor_off: 2,
						wait: 9
					})
				);
				game_state.activity = Activity::GameRound(Phase::Select);
			},
			Activity::GameRound(Phase::Finish) => {
				game_state
					.display_speech(
						&game_state
							.game_run
							.as_ref()
							.unwrap()
							.level
							.story
							.after_level
							.clone()
					)
					.await;
				game_state.activity = Activity::SelectLevel;
			},
			Activity::GameRound(Phase::Select) => {
				game_state.draw().await;
				setup_players(&mut game_state).await;
			},
			Activity::GameRound(Phase::Drive) => {
				game_state.update().await;
				game_state.draw().await;
			},
			Activity::Menu => {
				game_state.build_menu().await;
			},
			Activity::SelectLevel => {
				game_state.build_level_menu().await;
				init_level(&mut game_state);
				game_state.load_fire_emitter().await;
			}
		}
		next_frame().await;
	}
}

#[derive(Debug, Default, Parser)]
pub struct OptPlay {
	/// An optional level to start file (can be tiled map or mission2teegarden-b level)
	file: Option<String>
}

#[derive(Debug, Parser)]
enum Opt {
	/// Validate a Tiled map
	ValidateMap(m3_map::commands::OptValidateMap),
	/// Export a tiled map to an mission2teegarden-b level
	ExportMap(m3_map::commands::OptExportMap),
	/// Start the game.
	Play(OptPlay)
}

impl Default for Opt {
	fn default() -> Self {
		Opt::Play(Default::default())
	}
}

fn main() {
	my_env_logger_style::set_timestamp_precision(TimestampPrecision::Disable);
	my_env_logger_style::just_log();
	info!("🚗 {CARGO_PKG_NAME}  v{CARGO_PKG_VERSION} 🚗");
	let opt = if std::io::stdin().is_terminal() {
		Opt::parse()
	} else {
		Default::default()
	};
	let result = match opt {
		Opt::ValidateMap(opt) => m3_map::commands::validate(opt),
		Opt::ExportMap(opt) => m3_map::commands::export(opt),
		Opt::Play(opt) => {
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
			Ok(())
		}
	};
	if let Err(err) = result {
		eprintln!("{err:?}");
		process::exit(1);
	}
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
