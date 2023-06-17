use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Character {
	Captain
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Background {
	OuterSpace
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Speech {
	/// The text, which should be show at the botton of the screen.
	text: String,
	/// Profile picture of the speaking character
	profil: Option<Character>,
	/// Background picture, which is shown during the speech
	background: Option<Background>
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
		background = "OuterSpace"

		[[pre_level]]
		text = "now it is you turn!"
		
		[[after_level]]
		text = "You have mastered the challenge!"
		profil = "Captain"
		"#;
		let config: Story = toml::from_str(&toml).unwrap_or_else(|err| panic!("{}", err));
		let control = Story {
			pre_level: vec![
				Speech {
					text: "hi, I am the captain ...".to_owned(),
					profil: Some(Character::Captain),
					background: Some(Background::OuterSpace)
				},
				Speech {
					text: "now it is you turn!".to_owned(),
					profil: None,
					background: None
				},
			],
			after_level: vec![Speech {
				text: "You have mastered the challenge!".to_owned(),
				profil: Some(Character::Captain),
				background: None
			}]
		};
		assert_eq!(config, control);
	}
}
