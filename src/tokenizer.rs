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
