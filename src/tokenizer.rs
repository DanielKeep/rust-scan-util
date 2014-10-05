use std::default::Default;
use super::len_while;

pub trait Tokenizer: Clone + Default + ::std::fmt::Show {
	fn token_len(&self, s: &str) -> Option<uint>;
}

#[deriving(Clone, Default, Show)]
pub struct WordsAndInts;

impl Tokenizer for WordsAndInts {
	fn token_len(&self, s: &str) -> Option<uint> {
		if s.len() == 0 {
			return None;
		}

		let ch0 = s.char_at(0);

		if ch0.is_alphabetic() {
			len_while(s, |ch| ch.is_alphabetic())
		} else if ch0.is_digit() {
			len_while(s, |ch| ch.is_digit())
		} else {
			None
		}
	}
}

#[test]
fn test_words_and_ints() {
	let tl = |s:&str| WordsAndInts.token_len(s);

	assert_eq!(tl(""), None);
	assert_eq!(tl("_"), None);
	assert_eq!(tl("abc"), Some(3));
	assert_eq!(tl("abc def"), Some(3));
	assert_eq!(tl("abc123"), Some(3));
	assert_eq!(tl("abc_def"), Some(3));
	assert_eq!(tl("123"), Some(3));
	assert_eq!(tl("123 456"), Some(3));
	assert_eq!(tl("123abc"), Some(3));
	assert_eq!(tl("123_456"), Some(3));
	assert_eq!(tl("123.456"), Some(3));
}
