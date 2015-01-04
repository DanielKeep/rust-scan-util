/*!
This module provides the `Whitespace` trait and its implementations.
*/
use super::len_while;

/**
Implementations of the `Whitespace` trait are responsible for controlling when whitespace can be skipped, and when to turn whitespace into an explicit token.
*/
pub trait Whitespace: Eq + ::std::fmt::Show {
	/**
Indicates how many bytes at the start of the given string are "skippable" whitespace.
	*/
	fn strip_len(&self, s: &str) -> uint;

	/**
Indicates the length of an explicit whitespace token at the start of the given string, and what its contents should be, if one exists at all.

For example, if the implementing policy is to collapse all runs of non-newline whitespace together, the result of calling this on `"  \t \t\n \t x  y  "` would be `Some(5, " ")`.  The output string slice has the lifetime of the input string so that implementations can return *either* a static string *or* a slice of the input.

The default implementation provided assumes that there are no explicit whitespace tokens, and always returns `None`.
	*/
	fn token_len<'a>(&self, _: &'a str) -> Option<(uint, &'a str)> {
		None
	}
}

/**
This policy simply skips over all codepoints that satisfy the `White_Space` property.
*/
#[derive(Clone, Copy, Default, Eq, PartialEq, Show)]
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

/**
This policy simply skips over all codepoints that satisfy the `White_Space` property, *except* for line terminators, which become an explicit `"\n"` token.
*/
#[derive(Clone, Copy, Default, Eq, PartialEq, Show)]
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

/**
This policy does not skip any whitespace, instead creating explicit tokens in two situations:

- Single newline sequences (i.e. `\r\n`, `\r` or `\n`) become a single `"\n"` token.
- Runs of all other whitespace are collapsed to a single `" "` token.
*/
#[derive(Clone, Copy, Default, Eq, PartialEq, Show)]
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

/**
This policy collapses all runs of code points satisfying the `White_Space` property into a single `" "` token, including newlines.
*/
#[derive(Clone, Copy, Default, Eq, PartialEq, Show)]
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

/**
This policy turns newline sequences and all whitespace code points into individual tokens.  That is, tab and space produce different tokens, as do Windows newlines and UNIX newlines.
*/
#[derive(Clone, Copy, Default, Eq, PartialEq, Show)]
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
