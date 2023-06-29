use crate::assets::{MUSIC, SOUNDS};
use log::info;
use rand::{seq::SliceRandom, thread_rng};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::{io::Cursor, path::PathBuf, time::Duration};

enum CurrentBackgroundMusic {
	Titel,
	Level
}

pub(crate) struct SoundPlayer {
	output_handle: OutputStreamHandle,
	_stream: OutputStream,
	/// music sink handle
	background_music: Sink,
	/// drivig sound sink handle
	driving: Sink,
	/// driving gravel sound
	gravel: Sink,
	/// true if the car if driving
	is_driving: bool,
	current_background_music: CurrentBackgroundMusic
}

impl SoundPlayer {
	pub(crate) fn new() -> Self {
		let (_stream, output_handle) =
			OutputStream::try_default().expect("failed to access default audio device");
		let (background_music, background_music_output) = Sink::new_idle();
		output_handle.play_raw(background_music_output).unwrap();
		let (driving, driving_output) = Sink::new_idle();
		driving.set_volume(1.0);
		output_handle.play_raw(driving_output).unwrap();
		let (gravel, gravel_output) = Sink::new_idle();
		gravel.set_volume(0.6);
		output_handle.play_raw(gravel_output).unwrap();
		let mut sound_player = SoundPlayer {
			_stream,
			output_handle,
			background_music,
			driving,
			gravel,
			is_driving: false,
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

	/// play a crash sound once
	pub(crate) fn play_crash(&mut self) {
		let source = Decoder::new(Cursor::new(SOUNDS.crash))
			.unwrap()
			.skip_duration(Duration::from_millis(100));
		self.output_handle
			.play_raw(source.convert_samples())
			.unwrap();
	}

	/// play driving sound
	pub(crate) fn play_driving_looped(&mut self) {
		if !self.is_driving {
			self.is_driving = true;
			let decoder = Decoder::new_looped(Cursor::new(SOUNDS.driving)).unwrap();
			self.driving.append(decoder);
			let decoder = Decoder::new_looped(Cursor::new(SOUNDS.gravel_road)).unwrap();
			self.gravel.append(decoder);
		}
	}

	/// play driving sound
	pub(crate) fn stopp_driving(&mut self) {
		if self.is_driving {
			self.is_driving = false;
			self.driving.stop();
			self.gravel.stop();
		}
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
