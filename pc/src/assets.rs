pub(crate) const MUSIC: Music = Music {
	/// used as title music
	holiznacc0_mutant_club: include_bytes!(
		"../assets/sound/music/HoliznaCC0 - Mutant Club.mp3"
	),
	background_music: &[
		include_bytes!("../assets/sound/music/HoliznaCC0 - Ancient Memories.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Carnival Of Souls.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Dance Of The Dead.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Dusty Attic.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Earth.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Little Green Men.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Mercury.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Saturn.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Sky Fish.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Somethings Out There.mp3"),
		include_bytes!("../assets/sound/music/HoliznaCC0 - Track 1.mp3")
	]
};

pub(crate) const SOUNDS: Sounds = Sounds {};

pub(crate) struct Music {
	pub(crate) holiznacc0_mutant_club: &'static [u8],
	pub(crate) background_music: &'static [&'static [u8]]
}

pub(crate) struct Sounds {}
