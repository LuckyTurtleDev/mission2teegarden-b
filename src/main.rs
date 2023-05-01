#![no_std]
#![no_main]

use embedded_graphics::{
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle}
};
use pybadge::{prelude::*, Color, PyBadge};
use pybadge::NeoPixelColor;
use pybadge_high as pybadge;

#[entry]
fn main() -> ! {
	let mut pybadge = PyBadge::take().unwrap();
    let mut display = pybadge.display;
    display.clear(Color::BLACK).unwrap();

	loop {}
}