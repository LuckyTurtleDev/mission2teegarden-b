use crate::State;
use embedded_graphics::{prelude::*, text::Text};

use pybadge_high::Color;

pub(crate) fn init(state: &mut State<'_>) {
	state.display.clear(Color::BLACK).unwrap();
	Text::new(
		"look at the pc screen",
		Point::new(15, 50),
		state.text_style
	)
	.draw(&mut state.display)
	.ok();
}
