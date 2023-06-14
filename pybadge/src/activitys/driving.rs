use crate::{activitys::draw_card, State};

use embedded_graphics::prelude::*;

use pybadge_high::Color;
use strum::IntoEnumIterator;

pub(crate) fn init(state: &mut State<'_>) {
	state.display.clear(Color::BLACK).unwrap();
	for (i, card) in state.submitted_solution.iter().enumerate() {
		draw_card(
			i as u8,
			1,
			Some(card),
			&mut state.display,
			state.text_style_on_card
		);
	}
}

pub(crate) fn update(_state: &mut State<'_>) {}
