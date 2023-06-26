use crate::assets::MUSIC;
use log::info;
use rand::{seq::SliceRandom, thread_rng};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{io::Cursor, path::PathBuf};

enum CurrentBackgroundMusic {
	Titel,
	Level
}

pub(crate) struct SoundPlayer {
	output_handle: OutputStreamHandle,
	_stream: OutputStream,
	background_music: Sink,
	current_background_music: CurrentBackgroundMusic
}

impl SoundPlayer {
	pub(crate) fn new() -> Self {
		let (_stream, output_handle) =
			OutputStream::try_default().expect("failed to access default audio device");
		let (background_music, background_music_output) = Sink::new_idle();
		output_handle.play_raw(background_music_output).unwrap();
		let mut sound_player = SoundPlayer {
			_stream,
			output_handle,
			background_music,
			current_background_music: CurrentBackgroundMusic::Level
		};
		sound_player.play_titel_music();
		sound_player
	}

	pub(crate) fn play_titel_music(&mut self) {
		self.background_music.stop();
		self.background_music.set_speed(1.0);
		self.background_music.set_volume(0.8);
		info!(
			"play song {:?}",
			PathBuf::from(MUSIC.titel_music.file_name)
				.file_stem()
				.unwrap()
		);
		self.background_music
			.append(Decoder::new_looped(Cursor::new(MUSIC.titel_music.data)).unwrap());
		self.background_music.play();
		self.current_background_music = CurrentBackgroundMusic::Titel;
	}

	pub(crate) fn play_level_music(&mut self) {
		self.background_music.stop();
		self.background_music.set_speed(1.0);
		self.background_music.set_volume(0.6);
		self.background_music.play();
		self.current_background_music = CurrentBackgroundMusic::Level;
	}

	pub(crate) fn poll(&self) {
		match self.current_background_music {
			CurrentBackgroundMusic::Titel => {},
			CurrentBackgroundMusic::Level => {
				if self.background_music.empty() {
					let song = MUSIC.background_music.choose(&mut thread_rng()).unwrap();
					info!(
						"play song {:?}",
						PathBuf::from(song.file_name).file_stem().unwrap()
					);
					self.background_music
						.append(Decoder::new(Cursor::new(song.data)).unwrap());
				}
			},
		}
	}
}
