use macroquad::{
	prelude::ImageFormat,
	texture::{Image, Texture2D}
};
use mission2teegarden_b_macro::include_map;
use once_cell::sync::Lazy;

macro_rules! include_music {
	($file:expr) => {
		Song {
			file_name: $file,
			data: include_bytes!(concat!("../assets/sound/music/", $file))
		}
	};
}

/// currently all music are from [HoliznaCC0](https://freemusicarchive.org/music/holiznacc0/) and license under CC0 1.0.
/// See source code for more information, about invidiual titels.
pub(crate) const MUSIC: Music = Music {
	// https://freemusicarchive.org/music/holiznacc0/power-pop/mutant-club/
	titel_music: include_music!("HoliznaCC0 - Mutant Club.mp3"),
	background_music: &[
		// https://freemusicarchive.org/music/holiznacc0/forager/ancient-memories/
		include_music!("HoliznaCC0 - Ancient Memories.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/beats-from-the-crypt/dance-of-the-dead/
		include_music!("HoliznaCC0 - Dance Of The Dead.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/forager/dusty-attic/
		include_music!("HoliznaCC0 - Dusty Attic.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/interstellar-pop-songs/earth/
		include_music!("HoliznaCC0 - Earth.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/stranger-than-fiction-halloween-sci-fi/little-green-men/
		include_music!("HoliznaCC0 - Little Green Men.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/interstellar-pop-songs/mercury-1/
		include_music!("HoliznaCC0 - Mercury.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/interstellar-pop-songs/saturn-1/
		include_music!("HoliznaCC0 - Saturn.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/stranger-than-fiction-halloween-sci-fi/sky-fish/
		include_music!("HoliznaCC0 - Sky Fish.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/stranger-than-fiction-halloween-sci-fi/somethings-out-there/
		include_music!("HoliznaCC0 - Somethings Out There.mp3"),
		// https://freemusicarchive.org/music/holiznacc0/war-game-commission/track-1/
		include_music!("HoliznaCC0 - Track 1.mp3")
	]
};

/// see [Sounds] struct
pub(crate) const SOUNDS: Sounds = Sounds {
	crash: include_bytes!("../assets/sound/effects/crash.mp3"),
	driving: include_bytes!("../assets/sound/effects/driving.mp3"),
	gravel_road: include_bytes!("../assets/sound/effects/gravel-road.mp3")
};

pub(crate) struct Song {
	pub(crate) file_name: &'static str,
	pub(crate) data: &'static [u8]
}

/// see [MUSIC] const
pub(crate) struct Music {
	pub(crate) titel_music: Song,
	pub(crate) background_music: &'static [Song]
}

/// Currently all Sound **effecst** are from license under [Pixbay license](https://pixabay.com/service/terms/) .
pub(crate) struct Sounds {
	/// <https://pixabay.com/sound-effects/clank-car-crash-collision-6206/>
	pub(crate) crash: &'static [u8],
	/// <https://pixabay.com/sound-effects/driving-in-a-car-6227/>
	pub(crate) driving: &'static [u8],
	/// <https://pixabay.com/sound-effects/gravel-road-6747/>
	pub(crate) gravel_road: &'static [u8]
}

///store maps as String binary format
///call `Map::from_str()`
pub(crate) static LEVELS: Lazy<Vec<&str>> = Lazy::new(|| {
	vec![
		include_map!("pc/assets/level/001.tmx"),
		include_map!("pc/assets/level/002.tmx"),
		include_map!("pc/assets/level/003.tmx"),
		include_map!("pc/assets/level/004.tmx"),
		include_map!("pc/assets/level/005.tmx"),
		include_map!("pc/assets/level/006.tmx"),
	]
});

#[allow(clippy::redundant_closure)] //false positive?
pub(crate) static TEXTURES: Lazy<Textures> = Lazy::new(|| Textures::init());

pub(crate) trait GetTexture {
	fn texture(&self) -> Texture2D;
}

///Store all Textures
pub(crate) struct Textures {
	pub(crate) grass: Texture2D,
	pub(crate) grass_corner_sand: Texture2D,
	pub(crate) half_grass_sand: Texture2D,
	pub(crate) sand: Texture2D,
	pub(crate) sand_corner_grass: Texture2D,
	pub(crate) sand_double_corner_grass: Texture2D,
	pub(crate) grass_double_corner_sand: Texture2D,
	pub(crate) strange_grass: Texture2D,
	pub(crate) strange_grass_corner: Texture2D,
	pub(crate) strange_grass_inner_corner: Texture2D,
	pub(crate) strange_grass_half: Texture2D,
	pub(crate) strange_grass_double_corner: Texture2D,
	pub(crate) stone: Texture2D,
	pub(crate) strange_stone: Texture2D,
	pub(crate) player1_car: Texture2D,
	pub(crate) player2_car: Texture2D,
	pub(crate) player3_car: Texture2D,
	pub(crate) player4_car: Texture2D,
	pub(crate) global_goal: Texture2D,
	pub(crate) player1_goal: Texture2D,
	pub(crate) player2_goal: Texture2D,
	pub(crate) player3_goal: Texture2D,
	pub(crate) player4_goal: Texture2D,
	pub(crate) fire: Texture2D,
	pub(crate) captain: Texture2D,
	pub(crate) outer_space: Texture2D,
	pub(crate) title_background: Texture2D,
	pub(crate) button_background: Image,
	pub(crate) button_focused_background: Image
}

impl Textures {
	///init all Textures
	pub(crate) fn init() -> Textures {
		Textures {
			grass: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/grass.png"),
				Some(ImageFormat::Png)
			),
			grass_corner_sand: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/grass_corner_sand.png"),
				Some(ImageFormat::Png)
			),
			half_grass_sand: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/half_grass_sand.png"),
				Some(ImageFormat::Png)
			),
			sand: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/sand.png"),
				Some(ImageFormat::Png)
			),
			sand_corner_grass: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/sand_corner_grass.png"),
				Some(ImageFormat::Png)
			),
			sand_double_corner_grass: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/sand_double_corner_grass.png"),
				Some(ImageFormat::Png)
			),
			grass_double_corner_sand: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/grass_double_corner_sand.png"),
				Some(ImageFormat::Png)
			),
			strange_grass: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/strange_grass.png"),
				Some(ImageFormat::Png)
			),
			strange_grass_corner: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/strange_grass_corner.png"),
				Some(ImageFormat::Png)
			),
			strange_grass_inner_corner: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/strange_grass_inner_corner.png"),
				Some(ImageFormat::Png)
			),
			strange_grass_double_corner: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/strange_grass_double_corner.png"),
				Some(ImageFormat::Png)
			),
			strange_grass_half: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/BaseTiles/strange_grass_half.png"),
				Some(ImageFormat::Png)
			),
			stone: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/ObjectTiles/stone.png"),
				Some(ImageFormat::Png)
			),
			strange_stone: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/ObjectTiles/strange_stone.png"),
				Some(ImageFormat::Png)
			),
			player1_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player1_car_new.png"),
				Some(ImageFormat::Png)
			),
			player2_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player2_car_new.png"),
				Some(ImageFormat::Png)
			),
			player3_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player3_car_new.png"),
				Some(ImageFormat::Png)
			),
			player4_car: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player4_car_new.png"),
				Some(ImageFormat::Png)
			),
			global_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/goal.png"),
				Some(ImageFormat::Png)
			),
			player1_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player1_goal.png"),
				Some(ImageFormat::Png)
			),
			player2_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player2_goal.png"),
				Some(ImageFormat::Png)
			),
			player3_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player3_goal.png"),
				Some(ImageFormat::Png)
			),
			player4_goal: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Player/player4_goal.png"),
				Some(ImageFormat::Png)
			),
			fire: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Animations/crash_fire_2.png"),
				Some(ImageFormat::Png)
			),
			captain: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Character/captain.png"),
				Some(ImageFormat::Png)
			),
			outer_space: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Backgrounds/outer_space.png"),
				Some(ImageFormat::Png)
			),
			title_background: Texture2D::from_file_with_format(
				include_bytes!("../assets/img/Menu/menu_background.png"),
				Some(ImageFormat::Png)
			),
			button_background: Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_background.png"),
				Some(ImageFormat::Png)
			),
			button_focused_background: Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_focused_background.png"),
				Some(ImageFormat::Png)
			)
		}
	}

	pub(crate) fn get_player_textures(&self) -> Vec<Texture2D> {
		vec![
			self.player1_car,
			self.player2_car,
			self.player3_car,
			self.player4_car,
		]
	}
}
