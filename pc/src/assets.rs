use std::future::Future;

use macroquad::audio::{load_sound_from_bytes, Sound};
use once_cell::sync::OnceCell;

pub(crate) static SOUNDS: OnceCell<Sounds> = OnceCell::new();

pub(crate) struct Sounds {
	/// Music with is played at background
	pub(crate) background: Sound
}

impl Sounds {
	pub(crate) async fn init() {
		let sound = Sounds {
			background: load_sound_from_bytes(include_bytes!("TODO")).await.unwrap()
		};
		SOUNDS.set(sound).unwrap_or_else(|_| panic!());
	}
}
