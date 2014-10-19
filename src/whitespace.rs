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

#[test]
fn test_ws_ignore() {
	fn sp<'a>(s: &'a str) -> (uint, Option<(uint, &'a str)>) {
		(Ignore.strip_len(s), Ignore.token_len(s))
	}

	assert_eq!(sp(""), (0, None));
	assert_eq!(sp(" "), (1, None));
	assert_eq!(sp("\t"), (1, None));
	assert_eq!(sp("\r"), (1, None));
	assert_eq!(sp("\n"), (1, None));
	assert_eq!(sp("\r\n"), (2, None));
	assert_eq!(sp(" \t\r\n  x "), (6, None));
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

#[test]
fn test_ws_explicit_newline() {
	fn sp<'a>(s: &'a str) -> (uint, Option<(uint, &'a str)>) {
		(ExplicitNewline.strip_len(s), ExplicitNewline.token_len(s))
	}

	assert_eq!(sp(""), (0, None));
	assert_eq!(sp(" "), (1, None));
	assert_eq!(sp("\t"), (1, None));
	assert_eq!(sp("\r"), (0, Some((1, "\n"))));
	assert_eq!(sp("\n"), (0, Some((1, "\n"))));
	assert_eq!(sp("\r\n"), (0, Some((2, "\n"))));
	assert_eq!(sp(" \t\r\n  x "), (2, None));
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

#[test]
fn test_ws_explicit() {
	fn sp<'a>(s: &'a str) -> (uint, Option<(uint, &'a str)>) {
		(Explicit.strip_len(s), Explicit.token_len(s))
	}

	assert_eq!(sp(""), (0, None));
	assert_eq!(sp(" "), (0, Some((1, " "))));
	assert_eq!(sp("\t"), (0, Some((1, " "))));
	assert_eq!(sp("\r"), (0, Some((1, "\n"))));
	assert_eq!(sp("\n"), (0, Some((1, "\n"))));
	assert_eq!(sp("\r\n"), (0, Some((2, "\n"))));
	assert_eq!(sp(" \t\r\n  x "), (0, Some((2, " "))));
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

#[test]
fn test_ws_explicit_any() {
	fn sp<'a>(s: &'a str) -> (uint, Option<(uint, &'a str)>) {
		(ExplicitAny.strip_len(s), ExplicitAny.token_len(s))
	}

	assert_eq!(sp(""), (0, None));
	assert_eq!(sp(" "), (0, Some((1, " "))));
	assert_eq!(sp("\t"), (0, Some((1, " "))));
	assert_eq!(sp("\r"), (0, Some((1, " "))));
	assert_eq!(sp("\n"), (0, Some((1, " "))));
	assert_eq!(sp("\r\n"), (0, Some((2, " "))));
	assert_eq!(sp(" \t\r\n  x "), (0, Some((6, " "))));
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
		} else if s.starts_with("\r\n") {
			Some((2, s.slice_to(2)))
		} else {
			Some((1, s.slice_to(1)))
		}
	}
}

#[test]
fn test_ws_exact() {
	fn sp<'a>(s: &'a str) -> (uint, Option<(uint, &'a str)>) {
		(Exact.strip_len(s), Exact.token_len(s))
	}

	assert_eq!(sp(""), (0, None));
	assert_eq!(sp(" "), (0, Some((1, " "))));
	assert_eq!(sp("\t"), (0, Some((1, "\t"))));
	assert_eq!(sp("\r"), (0, Some((1, "\r"))));
	assert_eq!(sp("\n"), (0, Some((1, "\n"))));
	assert_eq!(sp("\r\n"), (0, Some((2, "\r\n"))));
	assert_eq!(sp(" \t\r\n  x "), (0, Some((1, " "))));
}
