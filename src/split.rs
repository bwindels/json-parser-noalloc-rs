pub fn split_mut(data: &mut [u8], index: usize) -> (&mut [u8], Option<&mut [u8]>) {
  let (token, remaining) = data.split_at_mut(index);
  let remaining = if remaining.len() != 0 {
    Some(remaining)
  }
  else {
    None
  };
  return (token, remaining);
}

pub fn split(data: &[u8], index: usize) -> (&[u8], Option<&[u8]>) {
  let (token, remaining) = data.split_at(index);
  let remaining = if remaining.len() != 0 {
    Some(remaining)
  }
  else {
    None
  };
  return (token, remaining);
}