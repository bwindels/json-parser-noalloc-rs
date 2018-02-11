const DOUBLE_QUOTE          : u8 = 0x22;
const COLON                 : u8 = 0x3A;
const COMMA                 : u8 = 0x2C;
const PERIOD                : u8 = 0x2E;
const LEFT_CURLY_BRACKET    : u8 = 0x7B;
const RIGHT_CURLY_BRACKET   : u8 = 0x7D;
const LEFT_SQUARE_BRACKET   : u8 = 0x5B;
const RIGHT_SQUARE_BRACKET  : u8 = 0x5D;
const SPACE                 : u8 = 0x20;
const TAB                   : u8 = 0x09;
const LINE_FEED             : u8 = 0x0A;
const CARRIAGE_RETURN       : u8 = 0x0D;
const DIGIT_ZERO            : u8 = 0x30;
const DIGIT_NINE            : u8 = 0x39;
const TRUE       : &'static [u8] = b"true";
const FALSE      : &'static [u8] = b"false";
const NULL       : &'static [u8] = b"null";

pub struct Position {
  offset: usize
}

pub enum ErrorKind {
  UnexpectedSequence,
  OpenString
}

pub struct Error {
  position: Position,
  kind: ErrorKind
}

pub struct Tokenizer<'a> {
  data: Option<&'a mut [u8]>
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  BeginObject,
  EndObject,
  BeginArray,
  EndArray,
  Comma,
  Colon,
  String(&'a mut [u8]),
  Number(&'a [u8]),
  True,
  False,
  Null,
  Whitespace
}

fn is_ascii_digit(chr: u8) -> bool {
  match chr {
    DIGIT_ZERO ... DIGIT_NINE => true,
    _ => false
  }
}

fn is_ascii_whitespace(chr: u8) -> bool {
  match chr {
    SPACE | TAB | LINE_FEED | CARRIAGE_RETURN => true,
    _ => false
  }
}

fn find_string_literal(data: &[u8]) -> Option<usize> {
  if data[0] != DOUBLE_QUOTE {
    return None;
  }
  //check \" by iterating over iter().windows(2)
  let end = data[1..].iter().position(|b| *b == DOUBLE_QUOTE).unwrap_or(data.len());
  Some(end + 1 + 1)
}

fn find_number_literal(data: &[u8]) -> Option<usize> {
  let end = data.iter().position(|b| !is_ascii_digit(*b) && *b != PERIOD).unwrap_or(data.len());
  if end != 0 {
    Some(end)
  }
  else {
    None
  }
}

fn find_whitespace(data: &[u8]) -> Option<usize> {
  let end = data.iter().position(|b| !is_ascii_whitespace(*b)).unwrap_or(data.len());
  if end != 0 {
    Some(end)
  }
  else {
    None
  }
}

fn split(data: &mut [u8], index: usize) -> (&mut [u8], Option<&mut [u8]>) {
  let (token, remaining) = data.split_at_mut(index);
  let remaining = if remaining.len() != 0 {
    Some(remaining)
  }
  else {
    None
  };
  return (token, remaining);
}

fn split_next_token<'a>(data: &'a mut [u8]) -> (Option<Token<'a>>, Option<&'a mut [u8]>) {
  let simple_token = match data[0] {
    LEFT_CURLY_BRACKET => Some(Token::BeginObject),
    RIGHT_CURLY_BRACKET => Some(Token::EndObject),
    LEFT_SQUARE_BRACKET => Some(Token::BeginArray),
    RIGHT_SQUARE_BRACKET => Some(Token::EndArray),
    COMMA => Some(Token::Comma),
    COLON => Some(Token::Colon),
    _ => None
  };

  if let Some(token) = simple_token {
    let (_, remaining) = split(data, 1);
    return (Some(token), remaining);
  }

  if data.starts_with(TRUE) {
    let (_, remaining) = split(data, TRUE.len());
    return (Some(Token::True), remaining);
  }
  if data.starts_with(FALSE) {
    let (_, remaining) = split(data, FALSE.len());
    return (Some(Token::False), remaining);
  }
  if data.starts_with(NULL) {
    let (_, remaining) = split(data, NULL.len());
    return (Some(Token::Null), remaining);
  }

  if let Some(len) = find_whitespace(data) {
    let (_, remaining) = split(data, len);
    return (Some(Token::Whitespace), remaining);
  }
  if let Some(len) = find_string_literal(data) {
    let (string_literal, remaining) = split(data, len);
    return (Some(Token::String(string_literal)), remaining);
  }
  if let Some(len) = find_number_literal(data) {
    let (number_literal, remaining) = split(data, len);
    return (Some(Token::Number(number_literal)), remaining);
  }

  return (None, Some(data));
}

impl<'a> Iterator for Tokenizer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let data_option = self.data.take();
    let (token, remaining_data) = data_option
      .map(split_next_token)
      .unwrap_or((None, None));
    self.data = remaining_data;
    token
  }
}


#[cfg(test)]
mod tests {
  use super::{Tokenizer, Token};

  #[test]
  fn it_works() {
    let data = b"{\"foo\":   [3.14, \"baaar\"]}";
    let mut mut_data = [0u8; 26];
    assert_eq!(data.len(), mut_data.len());
    for i in 0..data.len() {
      mut_data[i] = data[i];
    }
    let mut tokenizer = Tokenizer {data: Some(mut_data.as_mut())};

    assert_eq!(tokenizer.next(), Some(Token::BeginObject));
    let token = tokenizer.next();

    //assert_eq!(tokenizer.next(), Some(Token::String(b"\"foo\"")));
    while let Some(token) = tokenizer.next() {
      println!("token: {:?}", token);
    }
  }
  /*
  mod helpers {
    fn is_string_token<'a>(token: Token<'a>, )
  }*/
}