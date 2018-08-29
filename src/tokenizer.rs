use fallible_iterator::FallibleIterator;

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
const BACKSLASH             : u8 = 0x5C;
const TRUE       : &'static [u8] = b"true";
const FALSE      : &'static [u8] = b"false";
const NULL       : &'static [u8] = b"null";

#[derive(Debug, PartialEq)]
pub enum Error {
  UnterminatedString
}

pub type TokenizeResult<T> = Result<T, Error>;

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
  chr >= DIGIT_ZERO && chr <= DIGIT_NINE
}

fn is_ascii_whitespace(chr: u8) -> bool {
  match chr {
    SPACE | TAB | LINE_FEED | CARRIAGE_RETURN => true,
    _ => false
  }
}

fn find_string_literal(data: &[u8]) -> TokenizeResult<Option<usize>> {
  if data[0] != DOUBLE_QUOTE {
    return Ok(None);
  }
  let end = data
    .windows(2)
    .position(|window| {
      window[1] == DOUBLE_QUOTE && window[0] != BACKSLASH
    });
  let len = end.map(|n| Some(n + 2));
  len.ok_or(Error::UnterminatedString)
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

fn split_next_token<'a>(data: &'a mut [u8]) -> TokenizeResult<(Option<Token<'a>>, Option<&'a mut [u8]>)> {
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
    return Ok( (Some(token), remaining) );
  }

  if data.starts_with(TRUE) {
    let (_, remaining) = split(data, TRUE.len());
    return Ok( (Some(Token::True), remaining) );
  }
  if data.starts_with(FALSE) {
    let (_, remaining) = split(data, FALSE.len());
    return Ok( (Some(Token::False), remaining) );
  }
  if data.starts_with(NULL) {
    let (_, remaining) = split(data, NULL.len());
    return Ok( (Some(Token::Null), remaining) );
  }

  if let Some(len) = find_whitespace(data) {
    let (_, remaining) = split(data, len);
    return Ok( (Some(Token::Whitespace), remaining) );
  }
  if let Some(len) = find_string_literal(data)? {
    let (string_literal, remaining) = split(data, len);
    return Ok( (Some(Token::String(string_literal)), remaining) );
  }
  if let Some(len) = find_number_literal(data) {
    let (number_literal, remaining) = split(data, len);
    return Ok( (Some(Token::Number(number_literal)), remaining) );
  }

  return Ok( (None, Some(data)) );
}

pub struct Tokenizer<'a> {
  data: Option<&'a mut [u8]>
}

impl<'a> FallibleIterator for Tokenizer<'a> {
  type Item = Token<'a>;
  type Error = Error;

  fn next(&mut self) -> TokenizeResult<Option<Self::Item>> {
    let data_option = self.data.take();
    let (token, remaining_data) = data_option
      .map(split_next_token)
      .unwrap_or(Ok( (None, None) ))?;
    self.data = remaining_data;
    Ok(token)
  }
}


#[cfg(test)]
mod tests {
  use fallible_iterator::FallibleIterator;
  use super::{Tokenizer, Token, Error};
  use self::helpers::copy_str;

  #[test]
  fn test_basic_object() {
    let mut json = [0u8; 29];
    copy_str(&mut json, b"{\"foo\":   [3.14909, \"baaar\"]}");
    let mut foo = [0u8; 5];
    copy_str(&mut foo, b"\"foo\"");
    let mut baaar = [0u8; 7];
    copy_str(&mut baaar, b"\"baaar\"");
    
    let mut tokenizer = Tokenizer {data: Some(json.as_mut())};

    assert_eq!(tokenizer.next(), Ok(Some(Token::BeginObject)));
    assert_eq!(tokenizer.next(), Ok(Some(Token::String(&mut foo))));
    assert_eq!(tokenizer.next(), Ok(Some(Token::Colon)));
    assert_eq!(tokenizer.next(), Ok(Some(Token::Whitespace)));
    assert_eq!(tokenizer.next(), Ok(Some(Token::BeginArray)));
    assert_eq!(tokenizer.next(), Ok(Some(Token::Number(b"3.14909"))));
    assert_eq!(tokenizer.next(), Ok(Some(Token::Comma)));
    assert_eq!(tokenizer.next(), Ok(Some(Token::Whitespace)));
    assert_eq!(tokenizer.next(), Ok(Some(Token::String(&mut baaar))));
    assert_eq!(tokenizer.next(), Ok(Some(Token::EndArray)));
    assert_eq!(tokenizer.next(), Ok(Some(Token::EndObject)));
    assert_eq!(tokenizer.next(), Ok(None));
  }

  #[test]
  fn test_string_empty() {
    let mut json = [0u8; 2];
    copy_str(&mut json, b"\"\"");
    let mut expected_string = [0u8; 2];
    copy_str(&mut expected_string, &json);
    let mut tokenizer = Tokenizer {data: Some(json.as_mut())};
    assert_eq!(tokenizer.next(), Ok(Some(Token::String(&mut expected_string))));
    assert_eq!(tokenizer.next(), Ok(None));
  }

  #[test]
  fn test_string_with_escaped_quotation_mark() {
    let mut json = [0u8; 4];
    copy_str(&mut json, b"\"\\\"\"");
    let mut expected_string = [0u8; 4];
    copy_str(&mut expected_string, &json);
    let mut tokenizer = Tokenizer {data: Some(json.as_mut())};
    assert_eq!(tokenizer.next(), Ok(Some(Token::String(&mut expected_string))));
    assert_eq!(tokenizer.next(), Ok(None));
  }

  #[test]
  fn test_string_unterminated() {
    let mut json = [0u8; 6];
    copy_str(&mut json, b"\"hello");
    let mut tokenizer = Tokenizer {data: Some(json.as_mut())};
    assert_eq!(tokenizer.next(), Err(Error::UnterminatedString));
  }

  mod helpers {
    pub fn copy_str(mut_data: &mut [u8], data: &[u8]) {
      assert_eq!(data.len(), mut_data.len());
      for i in 0..data.len() {
        mut_data[i] = data[i];
      }
    }
  }
}