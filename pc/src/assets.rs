macro_rules! include_music {
	($file:expr) => {
		Song {
			file_name: $file,
			data: include_bytes!(concat!("../assets/sound/music/", $file))
		}
	};
}

pub(crate) const MUSIC: Music = Music {
	/// used as title music
	titel_music: include_music!("HoliznaCC0 - Mutant Club.mp3"),
	background_music: &[
		include_music!("HoliznaCC0 - Ancient Memories.mp3"),
		include_music!("HoliznaCC0 - Dance Of The Dead.mp3"),
		include_music!("HoliznaCC0 - Dusty Attic.mp3"),
		include_music!("HoliznaCC0 - Earth.mp3"),
		include_music!("HoliznaCC0 - Little Green Men.mp3"),
		include_music!("HoliznaCC0 - Mercury.mp3"),
		include_music!("HoliznaCC0 - Saturn.mp3"),
		include_music!("HoliznaCC0 - Sky Fish.mp3"),
		include_music!("HoliznaCC0 - Somethings Out There.mp3"),
		include_music!("HoliznaCC0 - Track 1.mp3")
	]
};

pub(crate) const SOUNDS: Sounds = Sounds {};

pub(crate) struct Song {
	pub(crate) file_name: &'static str,
	pub(crate) data: &'static [u8]
}

pub(crate) struct Music {
	pub(crate) titel_music: Song,
	pub(crate) background_music: &'static [Song]
}

pub(crate) struct Sounds {}
