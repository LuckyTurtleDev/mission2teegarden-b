use std::io::Cursor;

use crate::assets::MUSIC;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

pub(crate) struct SoundPlayer {
	output_handle: OutputStreamHandle,
	_stream: OutputStream,
	background_music: Sink
}

impl SoundPlayer {
	pub(crate) fn new() -> Self {
		let (_stream, output_handle) =
			OutputStream::try_default().expect("failed to access default audio device");
		let mut background_music = output_handle
			.play_once(Cursor::new(MUSIC.holiznacc0_mutant_club))
			.unwrap();
		background_music.set_volume(0.9);
		SoundPlayer {
			_stream,
			output_handle,
			background_music
		}
	}

	pub(crate) fn poll(&self) {
		if self.background_music.empty() {
			self.background_music.append(
				Decoder::new_looped(Cursor::new(MUSIC.holiznacc0_mutant_club)).unwrap()
			);
		}
	}
}
