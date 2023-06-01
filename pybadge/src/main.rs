#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unused_must_use)]
#![no_std]
#![no_main]

use activitys::Activity;
use bincode::{decode_from_slice, encode_into_slice, error::DecodeError};
use embedded_graphics::{
	draw_target::DrawTarget,
	mono_font::{
		ascii::{FONT_6X10, FONT_9X15},
		MonoTextStyle
	},
	prelude::*,
	text::{renderer::CharacterStyle, Text}
};
use heapless::Vec;
use m3_models::{
	AvailableCards, Card, Key, MessageToPc, MessageToPyBadge, ToPcGameEvent,
	ToPcProtocol, ToPybadgeProtocol, ToPypadeGameEvent
};
use pybadge_high::{
	buttons,
	buttons::{Button, Buttons},
	prelude::*,
	time::uptime,
	Color, Display, PyBadge
};
use strum::IntoEnumIterator;

mod activitys;
mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const _CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

const _DISPLAY_WIDHT: u16 = 160;
const _DISPLAY_HIGHT: u16 = 128;

fn read_events(usb_data: &mut Vec<u8, 128>) -> Vec<MessageToPyBadge, 10> {
	let mut events = Vec::<MessageToPyBadge, 10>::new();
	//vec has max size of 10
	for _ in 0..10 {
		match decode_from_slice(usb_data.as_slice(), bincode::config::standard()) {
			Ok((event, len)) => {
				events.push(event).unwrap();
				//remove readed bytes
				let remaining_data: Vec<u8, 128> =
					usb_data[..len].iter().copied().collect();
				usb_data.clear();
				usb_data
					.extend_from_slice(remaining_data.as_slice())
					.unwrap();
			},
			Err(err) => {
				match err {
					//we need to wait for more data first
					DecodeError::UnexpectedEnd { .. } => break,
					_ => panic!("Could not decode message\n  {}", err)
				}
			}
		};
	}
	events
}

fn send_event(event: MessageToPc) {
	let mut buf = [0_u8; 128];
	let len = encode_into_slice(event, &mut buf, bincode::config::standard()).unwrap();
	usb::wirte(&buf[..len]);
}

/// convert keys of `pybadge-high` crate, to the keys of the `m3-models` crate.
fn convert_keys(button: pybadge_high::buttons::Button) -> Key {
	match button {
		Button::B => Key::B,
		Button::A => Key::A,
		Button::Up => Key::Up,
		Button::Left => Key::Left,
		Button::Down => Key::Down,
		Button::Right => Key::Right,
		Button::Start => Key::Start,
		Button::Sesect => Key::Select
	}
}

fn send_button_state(buttons: &Buttons) {
	for event in buttons.events() {
		if let buttons::Event::Pressed(key) = event {
			send_event(MessageToPc::GameEvent(ToPcGameEvent::KeyPressed(
				convert_keys(key)
			)));
		}
	}
}

struct State<'a> {
	display: Display,
	buttons: Buttons,
	/// initinal avaibale cards
	init_avaiable_cards: AvailableCards,
	/// count of different card types
	card_type_count: u8,
	/// count of cards, which are still be able to select
	avaiable_cards: AvailableCards,
	/// solution created by the player
	solution: Vec<Card, 12>,
	activity: Activity,
	cursor: (u8, u8),
	wait_count: u8,
	/// positon of the wait card to allow faster update
	wait_card_index: Option<u8>,
	text_style: MonoTextStyle<'a, Color>,
	text_style_large: MonoTextStyle<'a, Color>,
	text_style_on_card: MonoTextStyle<'a, Color>
}

impl State<'_> {
	/// clear and draw the hole activity
	fn init_activity(&mut self) {
		match self.activity {
			Activity::Waiting => {},
			Activity::Selecter => activitys::card_selecter::init(self),
			Activity::GameOver(game_over_type) => {
				activitys::game_over::init(self, &game_over_type.clone())
			},
		}
	}
	/// only partional update the display, to improve fps
	fn update_draw(&mut self) {
		match self.activity {
			Activity::Waiting => {},
			Activity::Selecter => activitys::card_selecter::update(self),
			Activity::GameOver(_) => {}
		}
	}
}

#[entry]
fn main() -> ! {
	let text_style = MonoTextStyle::new(&FONT_6X10, Color::WHITE);
	let mut usb_data = Vec::<u8, 128>::new();
	let pybadge = PyBadge::take().unwrap();
	let mut delay = pybadge.delay;
	let mut display = pybadge.display;
	let mut buttons = pybadge.buttons;
	display.clear(Color::BLACK).unwrap();
	Text::new(
		"Please connect pybadge to \n  pc then start the game",
		Point::new(5, 50),
		text_style
	)
	.draw(&mut display)
	.ok();
	usb::init(pybadge.usb_builder);
	//wait for connection with pc;
	while !read_events(&mut usb_data)
		.iter()
		.any(|f| f == &MessageToPyBadge::Protocol(ToPybadgeProtocol::ConnectionRequest))
	{
		delay.delay_ms(50_u16);
		usb::read(&mut usb_data);
	}
	display.clear(Color::BLACK).unwrap();
	Text::new(
		"Press anykey to \n join the game",
		Point::new(35, 50),
		text_style
	)
	.draw(&mut display)
	.ok();
	while !buttons.some_pressed() {
		buttons.update();
	}
	send_event(MessageToPc::Protocol(ToPcProtocol::ConnectionResponse));
	display.clear(Color::BLACK).unwrap();
	Text::new("look at the pc screen", Point::new(15, 50), text_style)
		.draw(&mut display)
		.ok();
	//Todo: do not throw away event, wihich are directly send after ConnectionRequest
	let avaiable_cards = AvailableCards {
		left: 3,
		right: 3,
		motor_on: 2,
		motor_off: 2,
		wait: 9
	};
	let mut text_style_large = MonoTextStyle::new(&FONT_9X15, Color::WHITE);
	text_style_large.set_background_color(Some(Color::BLACK));
	let mut state = State {
		display,
		buttons,
		init_avaiable_cards: avaiable_cards.clone(),
		card_type_count: Card::iter()
			.filter(|f| avaiable_cards.card_count(f) != 0)
			.count() as u8,
		avaiable_cards,
		solution: Vec::new(),
		activity: Activity::Selecter,
		cursor: (0, 0),
		wait_count: 1,
		wait_card_index: None,
		text_style,
		text_style_large,
		text_style_on_card: MonoTextStyle::new(&FONT_9X15, Color::BLACK)
	};
	let mut last_activity = Activity::Waiting;
	let mut timestamp;
	loop {
		timestamp = uptime();
		send_event(MessageToPc::KeepAlive);
		state.buttons.update();
		send_button_state(&state.buttons);
		usb::read(&mut usb_data);
		let events = read_events(&mut usb_data);
		for event in events {
			match event {
				MessageToPyBadge::Protocol(_) => {},
				MessageToPyBadge::GameEvent(event) => match event {
					ToPypadeGameEvent::NewLevel(_) => todo!(),
					ToPypadeGameEvent::Retry => state.activity = Activity::Selecter,
					ToPypadeGameEvent::GameOver(game_over_type) => {
						state.activity = Activity::GameOver(game_over_type)
					},
				}
			}
		}
		if last_activity != state.activity {
			state.init_activity();
			last_activity = state.activity.clone();
		}
		state.update_draw();
		//60fps
		let frame_time = uptime().0 - timestamp.0;
		if frame_time < 16 {
			delay.delay_ms(16 - frame_time);
		}
	}
}
