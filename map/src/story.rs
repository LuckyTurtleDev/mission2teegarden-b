use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Character {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Background {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Speech {
	/// The text, which should be show at the botton of the screen.
	text: String,
	/// Profile picture of the speaking character
	profil: Option<Character>,
	/// Background picture, which is shown during the speech
	background: Option<Background>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Story {
	pub pre_level: Vec<Speech>,
	pub after_level: Vec<Speech>
}
