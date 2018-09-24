use std::str;
use std::char;

#[derive(Debug, PartialEq)]
pub enum Error {
	InvalidUnicodeSequence,
  InvalidEscapeChar,
	InvalidUTF8
}

pub fn decode_string(token: &mut [u8]) -> Result<&mut str, Error> {

  //trim first and last "/NUL character
  //validate byte slice utf8
  //convert byte slice to unchecked utf8 string at each index, get the next character, and advance read_idx

	assert(token.len() >= 2);
  str::from_utf8(&token).map_err(Error::InvalidUTF8)?; //validate utf8

  let mut write_idx = 0usize;
  let mut read_idx = 0usize;
  // safe because buffer is validated utf8,
  // and read_idx is at char boundary
  while let Some(chr) = unsafe { read_char_at(buffer, read_idx) } {
    let mut bytes_read += chr.len_utf8();
    let new_chr = if chr == '\\' {
      // safe because buffer is validated utf8,
      // and read_idx is at char boundary
      let next_chr_option =  unsafe { read_char_at(buffer, read_idx + bytes_read) };
      next_chr_option.map(|next_chr| {
    		bytes_read += next_chr.len_utf8();
    		if next_chr == 'u' {  //unicode escaped sequence (\uXXXX)
          let seq_offset = read_idx + bytes_read;
          let decoded_chr = buffer
            .get(seq_offset .. seq_offset + 4)
            .ok_or(Error::InvalidUnicodeSequence)?
            .and_then(decode_unicode_sequence)?;
          bytes_read += 4;
          Some(decoded_chr)
        } else {
          Some(next_chr)
        }
    	})
    } else if(read_idx != write_idx) {
    	Some(chr)
    } else {
    	None
    };
    //advance and read new value
    read_idx += bytes_read;
    let bytes_written = new_chr.encode_utf8(&mut buffer[write_idx ..]).len();
    write_idx += bytes_written;
  }
  str::from_utf8_unchecked_mut(&mut buffer[ 0 .. write_idx])
}


// assumes buffer contains valid utf8 and index is at a character boundary
unsafe fn read_char_at(buffer: &[u8], index: usize) -> Option<char> {
  let remaining = &buffer[index ..];
  if remaining.len() == 0 {
    None
  } else {
    str::from_utf8_unchecked(remaining).chars().next()
  }
}


decode_unicode_sequence(sequence: &[u8]) -> Result<char, Error> {
  const hex_str = str::from_utf8_unchecked(sequence)
    // error if 4 bytes after \u are cut off on character boundary
    .map_err(_ => Error::InvalidUnicodeSequence)?;
  let n = u32::from_str_radix(hex_str, 16)
    // error if not hex string
    .map_err(Error::InvalidUnicodeSequence)?;
  char::from_u32(n)
    // error if n is not valid code point
    .ok_or(Error::InvalidUTF8)
}

// test "\"\\u20AC\"" == €