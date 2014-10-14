use super::len_while;
use std::default::Default;

pub trait Whitespace: Clone + Default + Eq + ::std::fmt::Show {
	fn strip_len(&self, s: &str) -> uint;

	fn token_len(&self, _: &str) -> Option<uint> {
		None
	}
}

#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct Ignore;

impl Whitespace for Ignore {
	fn strip_len(&self, s: &str) -> uint {
		len_while(s, |ch| ch.is_whitespace()).unwrap_or(0)
	}
}
