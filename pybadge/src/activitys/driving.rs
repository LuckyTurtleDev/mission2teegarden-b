use super::DrawObject;
use crate::{activitys::draw_card, State};
use embedded_graphics::prelude::*;
use pybadge_high::Color;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DrivingState {
	/// index of card, which is current evaluted by the robot
	pub(crate) active_card: Option<u8>,
	/// index of previous card, which is was evaluted by the robot
	last_active_card: Option<u8>
}

pub(crate) fn init(state: &mut State<'_>) {
	state.display.clear(Color::BLACK).unwrap();
	for (i, card) in state.submitted_solution.iter().enumerate() {
		draw_card(
			i as u8,
			1,
			DrawObject::Card(card),
			&mut state.display,
			state.text_style_on_card
		);
	}
}

pub(crate) fn update(state: &mut State<'_>) {
	if state.driving_state.active_card != state.driving_state.last_active_card {
		if let Some(card_index) = state.driving_state.last_active_card {
			draw_card(
				card_index,
				1,
				DrawObject::Frame,
				&mut state.display,
				state.text_style_on_card
			);
		}
		if let Some(card_index) = state.driving_state.active_card {
			draw_card(
				card_index,
				1,
				DrawObject::Cursor,
				&mut state.display,
				state.text_style_on_card
			);
		}
		state.driving_state.last_active_card = state.driving_state.active_card;
	}
}
