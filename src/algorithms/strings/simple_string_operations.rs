/// Count the occurence of a character in a given string slice
///
/// # Examples
///
/// ```rust
/// assert_eq!(3, count_char("banana", &'a'))
/// ```
fn count_char(s: &str, c: &char) -> usize {
  s.chars().filter(|ch| ch == c).count()
}

///Reverses a given string slice
///
/// # Examples
/// ```rust
/// assert_eq!("!dlroW ,olleH", reverse_string("Hello, World!"))
/// ```
fn reverse_string(s: &str) -> String {
  s.chars().rev().collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn count_char_with_banana() {
    assert_eq!(3, count_char("banana", &'a'))
  }

  #[test]
  fn reverse_hello_world() {
    assert_eq!("!dlroW ,olleH", reverse_string("Hello, World!"))
  }
}
