
// #![no_std]

mod code;

#[macro_use]
mod board;
mod boards;

pub fn main() {
	boards::test();
}
