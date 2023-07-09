use crate::State;
use embedded_graphics::{prelude::*, text::Text};
use mission2teegarden_b_models::GameOver;
use pybadge_high::Color;

pub(crate) fn init(
	state: &mut State<'_>,
	game_over_type: &mission2teegarden_b_models::GameOver
) {
	state.display.clear(Color::BLACK).unwrap();
	//draw only cards, which are aviable for this level
	Text::new("Game Over", Point::new(40, 40), state.text_style_large)
		.draw(&mut state.display)
		.unwrap();
	let (text, x) = match game_over_type {
		GameOver::DriveAway => (" You robot have moved into\n unknown terrain and\nlost connection", 20),
		GameOver::TimeOut => ("Your robot has successfully\n completed its task", 25),
		GameOver::Crash => ("          Kaboom!\n\n    You robot have crashed!\n looks like it will not work anymore",2)
	};
	Text::new(text, Point::new(x, 60), state.text_style)
		.draw(&mut state.display)
		.unwrap();
}
