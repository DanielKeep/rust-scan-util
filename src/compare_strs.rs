/*!
This module provides the `CompareStrs` trait and its implementations.

This trait is used to implement string comparison during scanning.  Specifically, it is used when matching literal tokens (i.e. is "BaNaNa" a suitable match for "banana"?).  It is also provided, through the `Cursor`, to scanners, though they are free to ignore it.
*/
use std::ascii::AsciiExt;
use std::fmt::Show;

/**
This trait provides equality comparison for strings.
*/
pub trait CompareStrs: Eq + Show {
	/**
Compare two strings, returning `true` if they are considered equal under the semantics of the implementing type.
	*/
	fn compare_strs<'a>(&self, a: &str, b: &str) -> bool;
}

/**
Provides exact comparison semantics: two strings are equal if and only if their binary UTF-8 representations are identical.

This *does not* take Unicode normalisation into account.
*/
#[derive(Clone, Copy, Default, Eq, PartialEq, Show)]
pub struct Exact;

impl CompareStrs for Exact {
	fn compare_strs<'a>(&self, a: &str, b: &str) -> bool {
		a == b
	}
}

#[test]
fn test_cs_exact() {
	let cs = |a,b| Exact.compare_strs(a, b);

	let s = "abc ΑΒΓαβγ";

	assert_eq!(cs(s, "abc ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "Abc ΑΒΓαβγ"), false);
	assert_eq!(cs(s, "aBc ΑΒΓαβγ"), false);
	assert_eq!(cs(s, "abC ΑΒΓαβγ"), false);
	assert_eq!(cs(s, "abc αΒΓαβγ"), false);
	assert_eq!(cs(s, "abc ΑβΓαβγ"), false);
	assert_eq!(cs(s, "abc ΑΒγαβγ"), false);
	assert_eq!(cs(s, "abc ΑΒΓΑβγ"), false);
	assert_eq!(cs(s, "abc ΑΒΓαΒγ"), false);
	assert_eq!(cs(s, "abc ΑΒΓαβΓ"), false);
}

/**
Provides case-insensitive semantics for code points within the ASCII range: two strings are equal if and only if their binary UTF-8 representations are identical, with the exception of the case of latin characters in the ASCII range.

This *does not* take Unicode normalisation into account.

This is provided as a (possibly faster) alternative to the default `CaseInsensitive` implementation.
*/
#[derive(Clone, Copy, Default, Eq, PartialEq, Show)]
pub struct AsciiCaseInsensitive;

impl CompareStrs for AsciiCaseInsensitive {
	fn compare_strs<'a>(&self, a: &str, b: &str) -> bool {
		a.eq_ignore_ascii_case(b)
	}
}

#[test]
fn test_cs_ascii_case_insensitive() {
	let cs = |a,b| AsciiCaseInsensitive.compare_strs(a, b);

	let s = "abc ΑΒΓαβγ";

	assert_eq!(cs(s, "abc ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "Abc ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "aBc ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "abC ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "abc αΒΓαβγ"), false);
	assert_eq!(cs(s, "abc ΑβΓαβγ"), false);
	assert_eq!(cs(s, "abc ΑΒγαβγ"), false);
	assert_eq!(cs(s, "abc ΑΒΓΑβγ"), false);
	assert_eq!(cs(s, "abc ΑΒΓαΒγ"), false);
	assert_eq!(cs(s, "abc ΑΒΓαβΓ"), false);
}

/**
Provides case-insensitive semantics: two strings are equal if and only if their binary UTF-8 representations are identical, with the exception of the case of code points.

**Note**: this implementation is not entirely correct.  It does not account for cases where a single code point maps to more than one lowercase codepoint, nor is it locale-aware.  This is considered a bug, and may be fixed in future.

This *does not* take Unicode normalisation into account.
*/
#[derive(Clone, Copy, Default, Eq, PartialEq, Show)]
pub struct CaseInsensitive;

impl CompareStrs for CaseInsensitive {
	fn compare_strs<'a>(&self, a: &str, b: &str) -> bool {
		if a.len() != b.len() { return false; }

		// BUG: This fails to consider cases that map one codepoint to more than one lowercase codepoint.  It's also not (AFAIK) locale-aware.
		a.chars().zip(b.chars()).all(|(ca, cb)| ca.to_lowercase() == cb.to_lowercase())
	}
}

#[test]
fn test_cs_case_insensitive() {
	let cs = |a,b| CaseInsensitive.compare_strs(a, b);

	let s = "abc ΑΒΓαβγ";

	assert_eq!(cs(s, "abc ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "Abc ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "aBc ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "abC ΑΒΓαβγ"), true);
	assert_eq!(cs(s, "abc αΒΓαβγ"), true);
	assert_eq!(cs(s, "abc ΑβΓαβγ"), true);
	assert_eq!(cs(s, "abc ΑΒγαβγ"), true);
	assert_eq!(cs(s, "abc ΑΒΓΑβγ"), true);
	assert_eq!(cs(s, "abc ΑΒΓαΒγ"), true);
	assert_eq!(cs(s, "abc ΑΒΓαβΓ"), true);
}
