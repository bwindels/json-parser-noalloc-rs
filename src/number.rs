use super::constants::*;

pub enum Number {
	Integer(i64),
	Unsigned(u64),
	Float(f64)
}

pub enum Error {
	InvalidInteger,
	InvalidUnsigned,
	InvalidFloat
}

pub fn is_ascii_digit(chr: u8) -> bool {
  chr >= DIGIT_ZERO && chr <= DIGIT_NINE
}

fn is_nonzero_ascii_digit(chr: u8) -> bool {
  chr > DIGIT_ZERO && chr <= DIGIT_NINE
}
