#![no_std]
#![no_main]

use bincode::{decode_from_slice, encode_into_slice, error::DecodeError, Encode};
use heapless::{Deque, Vec};
use m3_models::{MessageToPc, MessageToPyBadge};
use pybadge::PyBadge;
use pybadge_high as pybadge;
use pybadge_high::prelude::*;

mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn read_events(usb_data: &mut Vec<u8, 128>) -> Vec<MessageToPyBadge, 10> {
	let mut events = Vec::<MessageToPyBadge, 10>::new();
	//vec has max size of 10
	for _ in 0..10 {
		match decode_from_slice(usb_data.as_slice(), bincode::config::standard()) {
			Ok((event, len)) => {
				events.push(event);
				//remove readed bytes
				let remaining_data: Vec<u8, 128> =
					usb_data[..len].iter().map(|f| (*f)).collect();
				usb_data.clear();
				usb_data.extend_from_slice(remaining_data.as_slice());
			},
			Err(err) => {
				match err {
					//we need to wait for more data first
					DecodeError::UnexpectedEnd { .. } => break,
					_ => panic!()
				}
			}
		};
	}
	events
}

fn send_event(event: MessageToPc) {
	let array = [0_u8; 128];
	let len = event.encode_into_slice(&mut array).unwrap();
	usb::read(array[..len]).unwrap();
}

/// wait for the pc to start handshake
/// Return err, if pc need more the 5s to finish handshake after starting shake
fn connect(usb_data: Vec<u8, 128>) -> Result<(), ()> {
	while read_events(&mut usb_data)
		.iter()
		.any(|f| f == MessageToPyBadge::ConnectionRequest)
	{
		pybadge.delay.delay_ms(50_u16);
	}
	send_event(MessageToPc::ConnectionResponse);
	//timeout after 5 seconds
	for _ in 0..100 {
		pybadge.delay.delay_ms(50_u16);
		if read_events(&mut usb_data)
			.iter()
			.any(|f| f == MessageToPyBadge::ConnectionConfirmation)
		{
			Ok(())
		}
	}
	Err(())
}

#[entry]
fn main() -> ! {
	let mut usb_data = Vec::<u8, 128>::new();
	let mut pybadge = PyBadge::take().unwrap();
	usb::init(pybadge.usb_builder);
	//wait for connection with pc;
	while let Err(_) = connect(&mut usb_data) {}
	loop {
		pybadge.red_led.on().unwrap();
		pybadge.delay.delay_ms(1000_u16);
		pybadge.red_led.off().unwrap();
		pybadge.delay.delay_ms(1000_u16);
	}
}
