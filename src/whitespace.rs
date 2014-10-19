use super::len_while;

pub trait Whitespace: Clone + Eq + ::std::fmt::Show {
	fn strip_len(&self, s: &str) -> uint;

	fn token_len<'a>(&self, _: &'a str) -> Option<(uint, &'a str)> {
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

#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct ExplicitNewline;

impl Whitespace for ExplicitNewline {
	fn strip_len(&self, s: &str) -> uint {
		len_while(s, |ch| ch.is_whitespace() && !(ch == '\r' || ch == '\n')).unwrap_or(0)
	}

	fn token_len<'a>(&self, s: &'a str) -> Option<(uint, &'a str)> {
		if s.starts_with("\r\n") {
			Some((2, "\n"))
		} else if s.starts_with("\r") || s.starts_with("\n") {
			Some((1, "\n"))
		} else {
			None
		}
	}
}

#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct Explicit;

impl Whitespace for Explicit {
	fn strip_len(&self, _: &str) -> uint {
		0
	}

	fn token_len<'a>(&self, s: &'a str) -> Option<(uint, &'a str)> {
		if s.starts_with("\r\n") {
			Some((2, "\n"))
		} else if s.starts_with("\r") || s.starts_with("\n") {
			Some((1, "\n"))
		} else {
			len_while(s, |ch| ch.is_whitespace() && !(ch == '\r' || ch == '\n')).map(|n| (n, " "))
		}
	}
}

#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct ExplicitAny;

impl Whitespace for ExplicitAny {
	fn strip_len(&self, _: &str) -> uint {
		0
	}

	fn token_len<'a>(&self, s: &'a str) -> Option<(uint, &'a str)> {
		len_while(s, |ch| ch.is_whitespace()).map(|n| (n, " "))
	}
}

#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct Exact;

impl Whitespace for Exact {
	fn strip_len(&self, _: &str) -> uint {
		0
	}

	fn token_len<'a>(&self, s: &'a str) -> Option<(uint, &'a str)> {
		if s.len() == 0 || !s.char_at(0).is_whitespace() {
			None
		} else {
			Some((1, s.slice_to(1)))
		}
	}
}
