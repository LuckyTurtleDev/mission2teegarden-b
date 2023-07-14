#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![deny(rustdoc::bare_urls, rustdoc::broken_intra_doc_links)]
#![forbid(unused_must_use, unsafe_code)]
// disable terminal popup on windows
#![windows_subsystem = "windows"]

//! Welcome to a journey to the unexplored planet Teegarden b,
//! to find a new home for humanity.
//! Robots were sent to the planet's surface for exploration.
//! Program these robots from the safety of your spaceship,
//! by using your [pybadge](https://www.adafruit.com/product/4200).
//! <div align="center">
//! 	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/2a4af8f4-28a4-4589-99c3-2b18de4de267" width=60%>
//! </div>
//! What hazards await you on the planet?
//! Face them with up to 4 players.
//! Work together to solve all puzzles and challenges.
//! Will you be able to prepare everything, so humans can arrive on the planet?
//! <div align="center">
//! 	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/71af7810-5927-4d05-be75-9ca37617c411" width=49%>
//! 	<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/779ec8f7-6e15-4e2c-b737-b1ad5477d9f2" width=49%>
//! </div>
//! Try out Mission to Teegarden b now for free and figure it out.
//!
//! # Installation (Pc):
//! Mission to Teegarden b is available at the following repositories:
//!
//! [![Packaging status](https://repology.org/badge/vertical-allrepos/mission2teegarden-b.svg)](https://repology.org/project/mission2teegarden-b/versions)
//!
//! Prebuild binaries can also be downloaded from the
#![doc=concat!("[GitHub release](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/v",env!("CARGO_PKG_VERSION"),").")]
//! On Linux the following dependencies are needed.
//! * [`alsa-lib`](https://github.com/alsa-project/alsa-lib)
//! * [`gtk3`](https://gitlab.gnome.org/GNOME/gtk)
//! * [`libudev`](https://github.com/systemd/systemd)
//! Regular these are already installed.
//!
//! Mission to Teegarden b is only tested on Arch Linux and Ubuntu.
//! MacOS and Windows versions complie sucessfull but are untested.
//! Supressing standby on MacOS is temporary disable, see [#157](https://github.com/LuckyTurtleDev/mission2teegarden-b/issues/157).
//!
//!
//! ### Building from source:
//! Alternative you can easily build Mission to Teegarden b  by yourself:
//! * On Linux, install the following development dependencies.
//! On some distros (like Alpine and Debian), separate development packages exist, regular suffixed with `-dev`.
//! If this is the case, make sure that you have also installed the `*-dev` version.
//!   * [`alsa-lib`](https://github.com/alsa-project/alsa-lib)
//!   * [`gtk3`](https://gitlab.gnome.org/GNOME/gtk)
//!   * [`libudev`](https://github.com/systemd/systemd)
//! * [Install rust](https://www.rust-lang.org/tools/install)
#![doc=concat!("* [Download](https://github.com/LuckyTurtleDev/mission2teegarden-b/archive/refs/tags/v",env!("CARGO_PKG_VERSION"),".zip)")]
//! and unpack the source code.
//! * Run `cargo install --path pc --locked` inside the unpacked folder, to build and install mission2teegarden-b.
//! See the [rust book](https://doc.rust-lang.org/cargo/commands/cargo-install.html) for more information about cargo install.
//! * Make sure that `~/.cargo/bin` is listed in the `PATH` environment variable otherwise, the `mission2teegarden-b` executable can not be found.
//!
//!
//! # Flash Pybadge:
//! * Install an UF2 flasher. I recommend using [hf2-cli](https://crates.io/crates/hf2-cli).
//! * Download and unpack Pybadge binary from
#![doc=concat!("[GitHub release](https://github.com/LuckyTurtleDev/mission2teegarden-b/releases/v",env!("CARGO_PKG_VERSION"),").")]
//! * Press the reset button of the pybdage twice, to enter the bootloader.
//! * After this, execute `hf2 elf mission2teegarden-b-pybadge.elf` (or the corresponding command of your flashing tool) to flash the binary to the pybadge.
//! * Press the reset button again.
//! ### Building from source:
//! Alternative you can build m3 by yourself:
//! * [Install rustup](https://www.rust-lang.org/tools/install)
//! * [Install hf2-cli](https://crates.io/crates/hf2-cli) flasher.
//! * Install the rust `thumbv7em-none-eabihf` target (the architecture of the pybadge) by executing `rustup target install thumbv7em-none-eabihf`.
//! * Optional: install nightly toolchain for better error messages at the pybadge. `rustup toolchain install nightly --target thumbv7em-none-eabihf`
#![doc=concat!("* [Download](https://github.com/LuckyTurtleDev/mission2teegarden-b/archive/refs/tags/v",env!("CARGO_PKG_VERSION"),".zip)")]
//! and unpack the source code (if not already done).
//! * Press the reset button of the pybadge twice to enter bootloader
//! * Compile and flash program by running `cargo +nightly run --release -locked` inside the downloaded `pybadge` folder.
//! `+nightly` is optional and have to be left out if the "install nightly toolchain" step was skip.
//! Please use `+nightly` for bug reports.
//! * Press the reset button again.
//!
//! # Map/Level Editor:
//! Mission to Teegarden b allow creating custom maps/levels, by using the powerfull [Tiled Map editor](https://www.mapeditor.org/).
//! See [here](mission2teegarden_b_map) for more information about creating maps.
//! <div align="center">
//!		<img src="https://github.com/LuckyTurtleDev/mission2teegarden-b/assets/44570204/68403ebd-ce64-4baa-bba2-b52962b89d5c" width=80%>
//! </div>

use anyhow::Context;
use assets::LEVELS;
use clap::Parser;
#[cfg(not(target_os = "macos"))]
use keepawake::AwakeHandle;
use log::info;
use macroquad::{prelude::*, window, Window};
use macroquad_particles::Emitter;
use mission2teegarden_b_map::{Map, Orientation};
use my_env_logger_style::TimestampPrecision;
use once_cell::sync::Lazy;
use sound::SoundPlayer;
use std::{
	io::IsTerminal,
	process::{self, exit}
};

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
use update::{activate_players, setup_players};
mod usb;
use usb::Players;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const GAME_TITLE: &str = "Mission to Teegarden b";

#[derive(PartialEq)]
enum Phase {
	Introduction,
	Select,
	Drive,
	Finish,
	Pause
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
	original_map: Map,
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
	running: bool,
	#[cfg(not(target_os = "macos"))]
	/// Supress standby while playing the game
	_keep_awake: Option<AwakeHandle>,
	/// Index which button is currently focused in pause menu
	button_focused_index: u8
}

impl GameState {
	fn new(level: Option<Map>) -> GameState {
		let sound_player = sound::SoundPlayer::new();
		Lazy::force(&TEXTURES);
		#[cfg(not(target_os = "macos"))]
		let keep_awake = keepawake::Builder::new()
			.display(true)
			.app_name(CARGO_PKG_NAME)
			.reason(format!("user play {GAME_TITLE}"))
			.create()
			.map_err(|err| {
				let err = err.context("failed to suppress standby");
				warn!("{err:?}");
			})
			.ok();
		let (game_run, activity) = if let Some(level) = level {
			let game_state = GameRun {
				original_map: level.clone(),
				level: level.clone(),
				player_states: level
					.iter_player()
					.map(|f| PlayerState {
						position: f.position,
						orientation: f.orientation,
						next_action: None,
						rotation: Rotation::NoRotation,
						finished: false,
						crashed: false,
						out_of_map: false,
						solution: None
					})
					.collect()
			};
			(Some(game_state), Activity::GameRound(Phase::Introduction))
		} else {
			(None, Activity::Menu)
		};
		GameState {
			sound_player,
			activity,
			game_run,
			input_players: usb::Players::init(),
			delta_time: 0.0,
			movement_time: 0.5,
			player_count: 0,
			level_num: 0,
			animation_emitter: None,
			running: true,
			#[cfg(not(target_os = "macos"))]
			_keep_awake: keep_awake,
			button_focused_index: 0
		}
	}
}

async fn run_game(opt: OptPlay) {
	let level = opt.file.and_then(|path| {
		Map::load_from_file(path)
			.with_context(|| "failed to load map")
			.map_err(|err| {
				error!("{err:?}");
				exit(1)
			})
			.ok()
	});
	let mut game_state = GameState::new(level);
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
				game_state.load_fire_emitter().await;
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
				game_state.reset_pybadge_screens();
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
				if game_state.pause_button_pressed() {
					game_state.activity = Activity::GameRound(Phase::Pause);
				}
				game_state.draw().await;
				setup_players(&mut game_state).await;
			},
			Activity::GameRound(Phase::Drive) => {
				if game_state.pause_button_pressed() {
					game_state.activity = Activity::GameRound(Phase::Pause);
				}
				game_state.update().await;
				game_state.draw().await;
			},
			Activity::GameRound(Phase::Pause) => {
				game_state.draw().await;
				game_state.build_level_pause_menu().await;
			},
			Activity::Menu => {
				game_state.build_menu().await;
			},
			Activity::SelectLevel => {
				game_state.build_level_menu().await;
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
	info!("🪐 {CARGO_PKG_NAME}  v{CARGO_PKG_VERSION}  🪐");
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
				run_game(opt)
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
			Map::from_string(map)
				.unwrap_or_else(|_| panic!("map with index {i} is not valid"));
		}
	}
}
