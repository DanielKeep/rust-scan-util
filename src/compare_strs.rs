use std::ascii::AsciiExt;
use std::default::Default;
use std::fmt::Show;

pub trait CompareStrs: Clone + Default + Eq + Show {
	fn compare_strs<'a>(&self, a: &str, b: &str) -> bool;
}

#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct Exact;

impl CompareStrs for Exact {
	fn compare_strs<'a>(&self, a: &str, b: &str) -> bool {
		a == b
	}
}

#[deriving(Clone, Default, Eq, PartialEq, Show)]
pub struct AsciiCaseInsensitive;

impl CompareStrs for AsciiCaseInsensitive {
	fn compare_strs<'a>(&self, a: &str, b: &str) -> bool {
		a.eq_ignore_ascii_case(b)
	}
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
