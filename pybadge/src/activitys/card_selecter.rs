use crate::{send_event, State};
use core::{fmt::Write, mem};
use embedded_graphics::{
	mono_font::MonoTextStyle,
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
	text::Text
};
use embedded_sprites::{image::Image, include_image, sprite::Sprite};
use heapless::String;
use m3_models::{Card, MessageToPc, ToPcGameEvent};
use pybadge_high::{
	buttons::{Button, Event},
	Color, Display
};
use strum::IntoEnumIterator;

#[include_image]
const IMG_CARD_LEFT: Image<'_, Color> = "pybadge/img/Left.png";
#[include_image]
const IMG_CARD_RIGHT: Image<'_, Color> = "pybadge/img/Right.png";
#[include_image]
const IMG_CARD_MOVE: Image<'_, Color> = "pybadge/img/Move.png";
#[include_image]
const IMG_CARD_STOP: Image<'_, Color> = "pybadge/img/Stop.png";
#[include_image]
const IMG_CARD_WAIT: Image<'_, Color> = "pybadge/img/Wait.png";
#[include_image]
const IMG_CARD_SELETED: Image<'_, Color> = "pybadge/img/CardSelected.png";
#[include_image]
const IMG_CARD_FRAME: Image<'_, Color> = "pybadge/img/CardFrame.png";

/// max count of card per line
const CARD_LINE_LENGTH: u8 = 6;
const CARD_SELECTION_HIGHT: u8 = 91;

fn get_card_image(card: &Card) -> Image<'static, Color> {
	match card {
		Card::Left => IMG_CARD_LEFT,
		Card::Right => IMG_CARD_RIGHT,
		Card::MotorOn => IMG_CARD_MOVE,
		Card::MotorOff => IMG_CARD_STOP,
		Card::Wait(_) => IMG_CARD_WAIT
	}
}

/// draw the number of avaibale cards above a card type
/// The number is drawn at line postion i at the heigh y.
fn draw_count(
	i: u8,
	count: u8,
	display: &mut Display,
	text_style: MonoTextStyle<'_, Color>
) {
	let mut count_str = String::<3>::new();
	write!(count_str, "{count}").unwrap();
	//clean old number
	Text::new(&count_str, Point::new((26 * i + 9) as i32, 87), text_style)
		.draw(display)
		.unwrap();
}

/// draw a card or clearr field is None.
/// The card is drawn at postion i at the line which start the heigh y,
/// with build in line break
fn draw_card(
	i: u8,
	y: u8,
	card: Option<&Card>,
	display: &mut Display,
	text_style_on_card: MonoTextStyle<'_, Color>
) {
	let top_left = Point::new(
		(26 * (i % CARD_LINE_LENGTH) + 2) as i32,
		(y + 38 * (i / CARD_LINE_LENGTH)) as i32
	);
	if let Some(card) = card {
		Sprite::new(top_left, &get_card_image(card))
			.draw(display)
			.unwrap();
		if let Card::Wait(wait_count) = card {
			let mut wait_count_str = String::<3>::new();
			write!(wait_count_str, "{}", wait_count).unwrap();
			Text::new(
				&wait_count_str,
				Point::new(
					(26 * (i % CARD_LINE_LENGTH) + 9) as i32,
					(y + 15 + 38 * (i / CARD_LINE_LENGTH)) as i32
				),
				text_style_on_card
			)
			.draw(display)
			.unwrap();
		}
	} else {
		// clear the postion of the card by filling it with black
		Rectangle::with_corners(top_left, top_left + Point::new(25, 36))
			.into_styled(
				PrimitiveStyleBuilder::new()
					.fill_color(Color::BLACK)
					.build()
			)
			.draw(display)
			.unwrap();
	}
}

pub(crate) fn init(state: &mut State<'_>) {
	state.wait_card_index = None;
	state.display.clear(Color::BLACK).unwrap();
	//draw only cards, which are aviable for this level
	for (i, (count, card)) in Card::iter()
		.filter_map(|card| {
			if state.init_avaiable_cards.card_count(&card) == 0 {
				None
			} else {
				Some((state.avaiable_cards.card_count(&card), card))
			}
		})
		.enumerate()
	{
		//wait count must be set for wait cards
		let card = if let Card::Wait(_) = card {
			state.wait_card_index = Some(i as u8);
			Card::Wait(state.wait_count)
		} else {
			card
		};
		draw_card(
			i as u8,
			CARD_SELECTION_HIGHT,
			Some(&card),
			&mut state.display,
			state.text_style_on_card
		);
		draw_count(i as u8, count, &mut state.display, state.text_style_large);
	}
	state.cursor = (0, 1);
}

pub(crate) fn update(state: &mut State<'_>) {
	if state.buttons.some_pressed() {
		let last_cursor_pos = state.cursor;
		let last_wait_count = state.wait_count;
		// ad a card, if a is pressed
		let mut a_pressed = false;
		for event in state.buttons.events() {
			if let Event::Pressed(button) = event {
				match button {
					// cursor pos was eventuell changed and is now invalid
					// we need to make it valid again first
					Button::A => a_pressed = true,
					Button::B => {
						if let Some(card) = state.solution.pop() {
							let i = state.solution.len() as u8;
							let new_count = state.avaiable_cards.card_count(&card) + 1;
							state.avaiable_cards.set_card_count_mut(&card, new_count);
							draw_card(
								i,
								1,
								None,
								&mut state.display,
								state.text_style_on_card
							);
							for (i, card_iter) in Card::iter()
								.filter(|card| {
									state.init_avaiable_cards.card_count(card) != 0
								})
								.enumerate()
							{
								//compare the type of enum, without the field; see https://doc.rust-lang.org/std/mem/fn.discriminant.html
								if mem::discriminant(&card)
									== mem::discriminant(&card_iter)
								{
									let count = state.avaiable_cards.card_count(&card);
									draw_count(
										i as u8,
										count,
										&mut state.display,
										state.text_style_large
									)
								}
							}
						}
					},
					Button::Right => state.cursor.0 += 1,
					Button::Left => state.cursor.0 -= 1,
					Button::Up => state.wait_count += 1,
					Button::Down => state.wait_count -= 1,
					Button::Start => {
						// can not use [None;16], because "the trait `core::marker::Copy` is not implemented for `Card`"
						let mut solution = [0; 12].map(|_| None);
						for (i, card) in state.solution.iter().enumerate() {
							//array has the same length as the vec, so this shoud never panic
							solution[i] = Some(card.clone());
						}
						send_event(MessageToPc::GameEvent(ToPcGameEvent::Solution(
							solution
						)));
					},
					_ => {}
				}
			}
		}
		//check if all params are still in a valid range
		//and fix them if not
		if state.cursor.0 == u8::MAX {
			state.cursor.0 = state.card_type_count - 1;
		}
		if state.cursor.0 >= state.card_type_count {
			state.cursor.0 = 0;
		}
		if state.wait_count == 0 {
			state.wait_count = 9
		}
		if state.wait_count > 9 {
			state.wait_count = 1
		}
		// add a card to solutios
		if a_pressed && state.solution.len() < 12 {
			//update card state
			for (i, card) in Card::iter()
				.filter(|card| state.init_avaiable_cards.card_count(card) != 0)
				.enumerate()
			{
				// the card below the cursor
				if i as u8 == state.cursor.0 {
					let count: &mut u8 = state.avaiable_cards.card_count_mut(&card);
					if count == &0_u8 {
						continue;
					}
					*count -= 1;
					//draw new card count
					draw_count(
						i as u8,
						*count,
						&mut state.display,
						state.text_style_large
					);
					// card wait count must be set to the curren value in state
					let card = if let Card::Wait(_) = card {
						Card::Wait(state.wait_count)
					} else {
						card
					};
					state.solution.push(card).unwrap();
				}
			}
			//draw new card
			if let Some(card) = state.solution.last()
			//solution can be added
			{
				let i = (state.solution.len() - 1) as u8;
				draw_card(
					i,
					1,
					Some(card),
					&mut state.display,
					state.text_style_on_card
				);
			}
		}
		if last_wait_count != state.wait_count {
			if let Some(wait_card_pos) = state.wait_card_index {
				draw_card(
					wait_card_pos,
					CARD_SELECTION_HIGHT,
					Some(&Card::Wait(state.wait_count)),
					&mut state.display,
					state.text_style_on_card
				);
			}
		}
		if last_cursor_pos != state.cursor {
			Sprite::new(
				Point::new(
					(26 * last_cursor_pos.0 + 2) as i32,
					CARD_SELECTION_HIGHT as i32
				),
				&IMG_CARD_FRAME
			)
			.draw(&mut state.display)
			.unwrap();
		}
		//                         updating wait count, does override the cursor, so it must be redrawn
		if last_cursor_pos != state.cursor || last_wait_count != state.wait_count {
			Sprite::new(
				Point::new(
					(26 * state.cursor.0 + 2) as i32,
					CARD_SELECTION_HIGHT as i32
				),
				&IMG_CARD_SELETED
			)
			.draw(&mut state.display)
			.unwrap();
		}
	}
}