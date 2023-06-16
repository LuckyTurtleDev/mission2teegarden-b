use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Character {
	Captain
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Background {
	OuterSpace
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Speech {
	/// The text, which should be show at the botton of the screen.
	text: String,
	/// Profile picture of the speaking character
	profil: Option<Character>,
	/// Background picture, which is shown during the speech
	background: Option<Background>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Story {
	#[serde(default)]
	pub pre_level: Vec<Speech>,
	#[serde(default)]
	pub after_level: Vec<Speech>
}

#[cfg(test)]
mod tests {
	use super::*;
	use basic_toml as toml;

	#[test]
	fn load_story() {
		let toml = r#"
        [[pre_level]]
        text = "hi, I am the captain ..."
		profil = "Captain"
        "#;
		let _config: Story = toml::from_str(&toml).unwrap();
	}
}
