/*!
This module provides the `Tokenizer` trait and its implementations.
*/
use super::len_while;

/**
This trait is used to tokenise both input strings fed to a `scan*` macro *and* string literals in scan patterns.
*/
pub trait Tokenizer: Clone + Eq + ::std::fmt::Show {
	/**
If there is a valid token present at the start of the given string, return its length.  Otherwise, return `None`.  Note that `ScanCursor::pop_token` will automatically turn the next single code point into a token if this method returns `None`.  When implementing this function, you may rely on this behaviour.
	*/
	fn token_len(&self, s: &str) -> Option<uint>;
}

/**
Tokenises a string into words and integers.  Specifically, a word is a sequence of one or more code points which have the `Alphabetic` property; an integer is a sequence of one or more code points which are in the `N*` general category.
*/
#[deriving(Clone, Default, Eq, PartialEq, Show)]
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

/**
Tokenises a string into identifiers and integers.  Specifically, an identifier is a sequence of one code point which is either an underscore or satisfies the `XID_Start` property, followed by zero or more code points which satisfy the `XID_Continue` property; an integer is a sequence of one or more code points which are in the `N*` general category.
*/
#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct IdentsAndInts;

impl Tokenizer for IdentsAndInts {
	fn token_len(&self, s: &str) -> Option<uint> {
		if s.len() == 0 {
			return None;
		}

		let ch0 = s.char_at(0);

		if ch0 == '_' || ch0.is_XID_start() {
			len_while(s, |ch| ch.is_XID_continue())
		} else if ch0.is_digit() {
			len_while(s, |ch| ch.is_digit())
		} else {
			None
		}
	}
}

#[test]
fn test_idents_and_ints() {
	let tl = |s:&str| IdentsAndInts.token_len(s);

	assert_eq!(tl(""), None);
	assert_eq!(tl("_"), Some(1));
	assert_eq!(tl("abc"), Some(3));
	assert_eq!(tl("abc def"), Some(3));
	assert_eq!(tl("abc123"), Some(6));
	assert_eq!(tl("abc_def"), Some(7));
	assert_eq!(tl("123"), Some(3));
	assert_eq!(tl("123abc"), Some(3));
	assert_eq!(tl("_123abc"), Some(7));
	assert_eq!(tl("123 456"), Some(3));
	assert_eq!(tl("123abc"), Some(3));
	assert_eq!(tl("123_456"), Some(3));
	assert_eq!(tl("123.456"), Some(3));
}

/**
Tokenises a string into space-delimited tokens.  Specifically, a token will be a sequence of one or more code points which *do not* satisfy the `White_Space` property.
*/
#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct SpaceDelimited;

impl Tokenizer for SpaceDelimited {
	fn token_len(&self, s: &str) -> Option<uint> {
		if s.len() == 0 {
			return None;
		}

		len_while(s, |ch| !ch.is_whitespace())
	}
}

#[test]
fn test_space_delimited() {
	let tl = |s:&str| SpaceDelimited.token_len(s);

	assert_eq!(tl(""), None);
	assert_eq!(tl(" abc"), None);
	assert_eq!(tl("_"), Some(1));
	assert_eq!(tl("abc"), Some(3));
	assert_eq!(tl("abc def"), Some(3));
	assert_eq!(tl("abc123"), Some(6));
	assert_eq!(tl("abc_def"), Some(7));
	assert_eq!(tl("123"), Some(3));
	assert_eq!(tl("123abc"), Some(6));
	assert_eq!(tl("_123abc"), Some(7));
	assert_eq!(tl("123 456"), Some(3));
	assert_eq!(tl("123abc"), Some(6));
	assert_eq!(tl("123_456"), Some(7));
	assert_eq!(tl("123.456"), Some(7));
}

/**
This tokeniser interprets the entire input string as a single token *unless* the input string is empty, in which case it returns `None`.

As a result, this tokeniser is almost totally useless at runtime.  If you *do* use this tokeniser for a pattern, you will almost certainly want to specify a different runtime tokeniser using `#[runtime_tok="..."]`.
*/
#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct Explicit;

impl Tokenizer for Explicit {
	fn token_len(&self, s: &str) -> Option<uint> {
		if s.len() == 0 {
			return None;
		}

		Some(s.len())
	}
}

#[test]
fn test_explicit() {
	let tl = |s:&str| Explicit.token_len(s);

	assert_eq!(tl(""), None);
	assert_eq!(tl(" abc"), Some(4));
	assert_eq!(tl("_"), Some(1));
	assert_eq!(tl("abc"), Some(3));
	assert_eq!(tl("abc def"), Some(7));
	assert_eq!(tl("abc123"), Some(6));
	assert_eq!(tl("abc_def"), Some(7));
	assert_eq!(tl("123"), Some(3));
	assert_eq!(tl("123abc"), Some(6));
	assert_eq!(tl("_123abc"), Some(7));
	assert_eq!(tl("123 456"), Some(7));
	assert_eq!(tl("123abc"), Some(6));
	assert_eq!(tl("123_456"), Some(7));
	assert_eq!(tl("123.456"), Some(7));
}
