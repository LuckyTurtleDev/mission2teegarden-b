#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use, unsafe_code)]

//! # Installation (Pc):
//! Mission to Teegarden b is avaibale at the following repositories:
//!
//! [![Packaging status](https://repology.org/badge/vertical-allrepos/mission2teegarden_b.svg)](https://repology.org/project/mission2teegarden-b/versions)
//!
//! Prebuild binarys can also been downloaded from the
#![doc=concat!("[Github release](https://github.com/LuckyTurtleDev/mission2teegarden_b/releases/v",env!("CARGO_PKG_VERSION"),").")]
//! ### Building from source:
//! Alternative you can easily build Mission to Teegarden b  by yourself:
//! * on Linux install the following development dependencies:
//!   * [`alsa-lib`](https://github.com/alsa-project/alsa-lib)
//!   * [`libudev`](https://github.com/systemd/systemd)
//! At some distros (lika Alpine and Debian) seperate development packages exist, regular suffixed with `-dev`.
//! It this the case make sure that you have also installed the `*-dev` packages.
//! * [install rust](https://www.rust-lang.org/tools/install)
#![doc=concat!("* [Download](https://github.com/LuckyTurtleDev/mission2teegarden_b/archive/refs/tags/v",env!("CARGO_PKG_VERSION"),".zip)")]
//! and unpack the source code.
//! * run `cargo install --path pc --locked` inside the unpacked folder, to build and install the mission2teegarden-b.
//! See the [rust book](https://doc.rust-lang.org/cargo/commands/cargo-install.html) for more information about cargo install.
//! * make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable, otherwise the `mission2teegarden-b` executeable can not be found.
//!
//!
//! # Flash Pybadge:
//! * Install an UF2 flasher. I recommand using [hf2-cli](https://crates.io/crates/hf2-cli).
//! * Download and unpack Pybadge binary from
#![doc=concat!("[Github release](https://github.com/LuckyTurtleDev/mission2teegarden_b/releases/v",env!("CARGO_PKG_VERSION"),").")]
//! * Press the reset button of the pybdage twice, to enter the bootloader.
//! * After this execute `hf2 elf mission2teegarden-b-pybadge` (or the corresponding command of your flahing tool) to flash the binary to the pybadge.
//! * Press the reset button again.
//! ### Building from source:
//! Alternative you can build m3 by yourself:
//! * [install rustup](https://www.rust-lang.org/tools/install)
//! * run `cargo install hf2-cli --locked` to build and install the [hf2-cli](https://crates.io/crates/hf2-cli) flasher.
//! See the [rust book](https://doc.rust-lang.org/cargo/commands/cargo-install.html) for more information about cargo install.
//! * make sure that `~/.cargo/bin` is listed at the `PATH` enviroment variable otherwise the executeable can not be found..
//! * install the rust `thumbv7em-none-eabihf` target (the architecture of the pybadge) by executing `rustup target install thumbv7em-none-eabihf`.
//! * optional: install nightly toolchain for better error messages at the pybadge. `rustup toolchain install nightly --target thumbv7em-none-eabihf`
#![doc=concat!("* [Download](https://github.com/LuckyTurtleDev/mission2teegarden_b/archive/refs/tags/v",env!("CARGO_PKG_VERSION"),".zip)")]
//! and unpack the source code (if not already done).
//! * press the reset button of the pybadge twice to enter bootloader
//! * compile and flash program by running `cargo +nightly run --release -locked` inside the downloaded `pybadge` folder.
//! `+nightly` is optional and have to be left out if the "install nightly toolchain" step was skip.
//! Please use `+nightly` for bug reports.
//! * Press the reset button again.

use assets::LEVELS;
use clap::Parser;
use log::info;
use macroquad::{prelude::*, window, Window};
use macroquad_particles::Emitter;
use mission2teegarden_b_map::{Map, Orientation};
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
	/// time of one round in seconds
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
	game_state.sound_player.play_driving_looped();
	while game_state.running {
		game_state.sound_player.poll();
		match game_state.activity {
			Activity::GameRound(Phase::Drive) => {
				if let Some(ref game_run) = game_state.game_run {
					if 0 == game_state
						.input_players
						.players
						.iter()
						.flatten()
						.zip(game_run.player_states.iter())
						.filter(|(_player, state)| {
							!(state.finished
								|| state.crashed || state.out_of_map
								|| state.next_action.is_none())
						})
						.count()
					{
						game_state.sound_player.stopp_driving();
					} else {
						game_state.sound_player.play_driving_looped();
					}
				} else {
					game_state.sound_player.stopp_driving();
				}
			},
			_ => game_state.sound_player.stopp_driving()
		}
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
				activate_players(&mut game_state, false);
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
	ValidateMap(mission2teegarden_b_map::commands::OptValidateMap),
	/// Export a tiled map to an mission2teegarden-b level
	ExportMap(mission2teegarden_b_map::commands::OptExportMap),
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
		Opt::ValidateMap(opt) => mission2teegarden_b_map::commands::validate(opt),
		Opt::ExportMap(opt) => mission2teegarden_b_map::commands::export(opt),
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
