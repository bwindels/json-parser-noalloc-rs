use super::constants::*;
use std::str;

#[derive(Debug, PartialEq)]
pub enum Number {
	Integer(i64),
	Unsigned(u64),
	Float(f64)
}

#[derive(Debug, PartialEq)]
pub enum Error {
	InvalidInteger,
	InvalidUnsigned,
	InvalidFloat
}

pub fn parse_number(token: &[u8]) -> Result<Number, Error> {
	let mut has_exponent = false;
	let mut has_period = false;
	let mut has_minus = false;
	token.iter().for_each(|chr| {
		match *chr {
			PERIOD => has_period = true,
			MINUS => has_minus = true,
			LOWERCASE_E | UPPERCASE_E => has_exponent = true,
			_ => {}
		}
	});
	let token_str = str::from_utf8(token).map_err(|_| Error::InvalidFloat)?;
	if has_exponent || has_period {
		let n = token_str.parse::<f64>().map_err(|_| Error::InvalidFloat)?;
		Ok(Number::Float(n))
	}
	else if has_minus {
		let n = token_str.parse::<i64>().map_err(|_| Error::InvalidInteger)?;
		Ok(Number::Integer(n))
	}
	else {
		let n = token_str.parse::<u64>().map_err(|_| Error::InvalidUnsigned)?;
		Ok(Number::Unsigned(n))
	}
}

#[cfg(test)]
mod tests {
	use super::{parse_number, Number, Error};

	#[test]
	fn test_unsigned() {
		assert_eq!(parse_number(b"0"), Ok(Number::Unsigned(0)));
		assert_eq!(parse_number(b"1"), Ok(Number::Unsigned(1)));
		assert_eq!(parse_number(b"18446744073709551615"), Ok(Number::Unsigned(18446744073709551615)));
		assert_eq!(parse_number(b"18446744073709551616"), Err(Error::InvalidUnsigned));
		assert_eq!(parse_number(b"1a"), Err(Error::InvalidUnsigned));
	}

	#[test]
	fn test_integer() {
		assert_eq!(parse_number(b"-1"), Ok(Number::Integer(-1)));
		assert_eq!(parse_number(b"-9223372036854775808"), Ok(Number::Integer(-9223372036854775808)));
		assert_eq!(parse_number(b"-9223372036854775809"), Err(Error::InvalidInteger));
		assert_eq!(parse_number(b"-1a"), Err(Error::InvalidInteger));
	}

	#[test]
	fn test_float() {
		assert_eq!(parse_number(b".4"), Ok(Number::Float(0.4)));
		assert_eq!(parse_number(b"2e2"), Ok(Number::Float(200f64)));
		assert_eq!(parse_number(b"3141592e-6"), Ok(Number::Float(3.141592)));
		assert_eq!(parse_number(b"-1.a"), Err(Error::InvalidFloat));
	}
}