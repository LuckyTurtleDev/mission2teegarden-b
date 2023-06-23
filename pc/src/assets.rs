pub(crate) const MUSIC: Music = Music {
	holiznacc0_mutant_club: include_bytes!(
		"/home/lukas/git/m3/pc/assets/sound/music/HoliznaCC0 - Mutant Club.mp3"
	)
};

pub(crate) const SOUNDS: Sounds = Sounds {};

pub(crate) struct Music {
	pub(crate) holiznacc0_mutant_club: &'static [u8]
}

pub(crate) struct Sounds {}

//pub(crate) static SOUND_OUTPUT: Lazy<OutputStream> = Lazy::new(|| OutputStream::try_default().expect("failed to access default audio sink").1);
//pub fn test() {
//	let (_stream, stream_handle) = OutputStream::try_default().expect("failed to access default audio sink");
//}
/*
use macroquad::audio::{load_sound_from_bytes, Sound};
use once_cell::sync::OnceCell;

pub(crate) struct Mp3(());

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
*/
