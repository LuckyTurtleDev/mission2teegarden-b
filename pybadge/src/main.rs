#![no_std]
#![no_main]

use bincode::{decode_from_slice, encode_into_slice, error::DecodeError};
use embedded_graphics::{draw_target::DrawTarget, prelude::*};
use heapless::Vec;
use m3_models::{MessageToPc, MessageToPyBadge};
use pybadge_high::{prelude::*, Color, Delay, PyBadge};

mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

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

/// wait for the pc to start handshake
/// Return err, if pc need more the 5s to finish handshake after starting shake
fn connect(usb_data: &mut Vec<u8, 128>, delay: &mut Delay) -> Result<(), ()> {
	while read_events(usb_data)
		.iter()
		.any(|f| f == &MessageToPyBadge::ConnectionRequest)
	{
		delay.delay_ms(50_u16);
		usb::read(usb_data);
	}
	send_event(MessageToPc::ConnectionResponse);
	//timeout after 5 seconds
	for _ in 0..100 {
		delay.delay_ms(50_u16);
		usb::read(usb_data);
		if read_events(usb_data)
			.iter()
			.any(|f| f == &MessageToPyBadge::ConnectionConfirmation)
		{
			return Ok(());
		}
	}
	Err(())
}

#[entry]
fn main() -> ! {
	let mut usb_data = Vec::<u8, 128>::new();
	let mut pybadge = PyBadge::take().unwrap();
	let mut display = pybadge.display;
	display.clear(Color::BLACK).unwrap();
	usb::init(pybadge.usb_builder);
	//wait for connection with pc;
	while connect(&mut usb_data, &mut pybadge.delay).is_err() {}
	loop {
		usb::read(&mut usb_data);
		pybadge.red_led.on().unwrap();
		pybadge.delay.delay_ms(1000_u16);
		pybadge.red_led.off().unwrap();
		pybadge.delay.delay_ms(1000_u16);
	}
}
