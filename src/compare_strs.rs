use std::ascii::AsciiExt;
use std::fmt::Show;

pub trait CompareStrs: Clone + Eq + Show {
	fn compare_strs<'a>(&self, a: &str, b: &str) -> bool;
}

#[deriving(Clone, Default, Eq, PartialEq, Show)]
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

#[deriving(Clone, Default, Eq, PartialEq, Show)]
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

#[deriving(Clone, Default, Eq, PartialEq, Show)]
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
