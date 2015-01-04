/*!
This module provides both the `Scanner` trait, and the implementations for various basic types.

If you want to implement your own, the simplest way is to use the `scanner!` macro from the main `scan` package.  However, you can also implement a scanner by hand.
*/

use super::{ScanCursor, ScanError};

/**
This macro is a shortcut used in this module.  It implements a scanner for the type `T` given two constraints:

- The existance of a function `scan_fn` which takes a string and returns either `Some(uint)` with the length of the string slice to convert *or* a `None` indicating that there is no valid value to scan.
- That there exists an implementation of `std::str::FromStr` for `T` which can be used to convert the string slice denoted by `scan_fn` into a value of type `T`.

The `name` parameter is used in error messages to identify what sort of token was expected, when `scan_fn` returns `None`.
*/
#[macro_export]
macro_rules! from_str_slice_scanner {
	($scan_fn:path -> $T:ty as $name:expr) => {
		impl<'a> Scanner<'a> for $T {
			fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<($T, Cur), ScanError> {
				let err = |:| Err(cursor.expected($name));

				let end = match $scan_fn(cursor.tail_str()) {
					Some(i) => i,
					None => return err()
				};

				let s = cursor.str_slice_to(end);
				let cursor = cursor.slice_from(end);

				s.parse()
					.map(|i| Ok((i, cursor.clone())))
					.unwrap_or_else(err)
			}
		}
	};
}

/**
This trait provides the abstract interface for extracting strongly-typed values out of a string.
*/
pub trait Scanner<'a> {
	/**
The `scan` function's job is to, given a `ScanCursor`, either:

- return a value extracted from the input cursor's position *and* a successor cursor past the end of the input string which was consumed, or
- a `ScanError` describing why a value could not be extracted.

Another way of putting it: you should either parse a value and fast-forward the cursor over the parts of the input you used, or explain why you couldn't.
	*/
	fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<(Self, Cur), ScanError>;
}

impl<'a> Scanner<'a> for bool {
	fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<(bool, Cur), ScanError> {
		cursor.expect_tok("true").map(|c| (true, c))
			.or_else(|_| cursor.expect_tok("false").map(|c| (false, c)))
			.or_else(|_| Err(cursor.expected("`true` or `false`")))
	}
}

impl<'a> Scanner<'a> for char {
	fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<(char, Cur), ScanError> {
		let s = cursor.tail_str();
		if s.len() == 0 {
			Err(cursor.expected("character"))
		} else {
			let ::std::str::CharRange { ch, next } = s.char_range_at(0);
			Ok((ch, cursor.slice_from(next)))
		}
	}
}

impl<'a> Scanner<'a> for &'a str {
	fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<(&'a str, Cur), ScanError> {
		cursor.pop_token().map(|sc| Ok(sc))
			.unwrap_or_else(|| Err(cursor.expected("any token")))
	}
}

impl<'a> Scanner<'a> for String {
	fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<(String, Cur), ScanError> {
		use std::borrow::ToOwned;
		cursor.pop_token().map(|(s,c)| Ok((s.to_owned(), c)))
			.unwrap_or_else(|| Err(cursor.expected("any token")))
	}
}

impl<'a> Scanner<'a> for () {
	fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<((), Cur), ScanError> {
		Ok(((), cursor.clone()))
	}
}

from_str_slice_scanner! { scan_float -> f32 as "real number" }
from_str_slice_scanner! { scan_float -> f64 as "real number" }
from_str_slice_scanner! { scan_int -> i8 as "8-bit integer" }
from_str_slice_scanner! { scan_int -> i16 as "16-bit integer" }
from_str_slice_scanner! { scan_int -> i32 as "32-bit integer" }
from_str_slice_scanner! { scan_int -> i64 as "64-bit integer" }
from_str_slice_scanner! { scan_int -> int as "integer" }
from_str_slice_scanner! { scan_uint -> u8 as "8-bit unsigned integer" }
from_str_slice_scanner! { scan_uint -> u16 as "16-bit unsigned integer" }
from_str_slice_scanner! { scan_uint -> u32 as "32-bit unsigned integer" }
from_str_slice_scanner! { scan_uint -> u64 as "64-bit unsigned integer" }
from_str_slice_scanner! { scan_uint -> uint as "unsigned integer" }

/**
This function is just a short-hand way of accessing the byte offset *after* the code point at a given position in a string.
*/
pub fn next_char_at(s: &str, at: uint) -> uint {
	let ::std::str::CharRange { ch: _, next } = s.char_range_at(at);
	next
}

/**
This function scans the length of a float literal from a string.

Note that this doesn't support various bits of normal Rust syntax; like embedded underscores or hex literals.
*/
pub fn scan_float(s: &str) -> Option<uint> {
	enum State {
		Start,
		Whole,
		Suffix,
		ExponentStart,
		Exponent,
	}

	let mut state = State::Start;

	for (i,c) in s.char_indices() {
		match state {
			State::Start => {
				assert!(i == 0);
				match c {
					'0'...'9' | '-' => state = State::Whole,
					_ => return None
				}
			},
			State::Whole => match c {
				'0'...'9' => (),
				'.' => state = State::Suffix,
				'e' | 'E' => state = State::ExponentStart,
				_ => return Some(i)
			},
			State::Suffix => match c {
				'0'...'9' => (),
				'e' | 'E' => state = State::ExponentStart,
				_ => return Some(i),
			},
			State::ExponentStart => match c {
				'+' | '-' | '0'...'9' => state = State::Exponent,
				_ => return Some(i)
			},
			State::Exponent => match c {
				'0'...'9' => (),
				_ => return Some(i)
			}
		}
	}

	return Some(s.len())
}

/**
This function scans the length of an unsigned integer literal from a string.

Note that this doesn't support embedded underscores, or non-decimal bases.
*/
pub fn scan_uint<'a>(s: &'a str) -> Option<uint> {
	s.char_indices()
		.take_while(|&(_,c)| '0' <= c && c <= '9')
		.map(|(i,_)| Some(next_char_at(s, i)))
		.last().unwrap_or(None)
}

/**
This function scans the length of a (potentially) signed integer literal from a string.

Note that this doesn't support embedded underscores, or non-decimal bases.
*/
pub fn scan_int<'a>(s: &'a str) -> Option<uint> {
	if s.len() == 0 { return None }

	let (s, off) = if s.char_at(0) == '-' {
		(s.slice_from(1), 1)
	} else {
		(s, 0)
	};

	scan_uint(s).map(|end| end+off)
}

#[cfg(test)]
mod test {
	use Cursor;
	use ScanError;
	use super::Scanner;
	use tokenizer::WordsAndInts;
	use whitespace::Ignore;
	use compare_strs::CaseInsensitive;

	fn cur<'a>(s: &'a str) -> Cursor<'a, WordsAndInts, Ignore, CaseInsensitive> {
		Cursor::new(s, WordsAndInts, Ignore, CaseInsensitive)
	}

	fn scan_a<'a, T: Scanner<'a>>(s: &'a str) -> Result<(T, Cursor<'a, WordsAndInts, Ignore, CaseInsensitive>), ScanError> {
		Scanner::scan(&cur(s))
	}

	#[test]
	fn test_bool() {
		assert!(scan_a::<bool>("").err().is_some());
		assert!(scan_a::<bool>("true").unwrap().0 == true);
		assert!(scan_a::<bool>("false").unwrap().0 == false);
		assert!(scan_a::<bool>("yes").err().is_some());
		assert!(scan_a::<bool>("no").err().is_some());
		assert!(scan_a::<bool>("on").err().is_some());
		assert!(scan_a::<bool>("off").err().is_some());
		assert!(scan_a::<bool>("1").err().is_some());
		assert!(scan_a::<bool>("0").err().is_some());
	}

	#[test]
	fn test_char() {
		assert!(scan_a::<char>("").err().is_some());
		assert!(scan_a::<char>("x").unwrap().0 == 'x');
		assert!(scan_a::<char>("xy").unwrap().0 == 'x');
		assert!(scan_a::<char>("日本語").unwrap().0 == '日');
	}

	#[test]
	fn test_floats() {
		use std::str::FromStr;
		use std::num::Float;

		fn test<'a, F: Float + Scanner<'a> + FromStr>() {
			let f = |v:F| v;
			let fs = |s:&str| -> F s.parse().unwrap();
			
			assert!(scan_a::<F>("").err().is_some());
			assert!(scan_a::<F>("0").ok().unwrap().0 == f(Float::zero()));
			assert!(scan_a::<F>("0.0").ok().unwrap().0 == f(Float::zero()));
			assert!(scan_a::<F>("-0").ok().unwrap().0 == -f(Float::zero()));
			assert!(scan_a::<F>("1.0").ok().unwrap().0 == fs("1.0"));
			assert!(scan_a::<F>("1.00").ok().unwrap().0 == fs("1.0"));
			assert!(scan_a::<F>("1.0e0").ok().unwrap().0 == fs("1.0"));
			assert!(scan_a::<F>("1.0e1").ok().unwrap().0 == fs("10.0"));
		}

		test::<f32>();
		test::<f64>();
	}

	#[test]
	fn test_sized_ints() {
		use std::fmt::Show;
		use std::num::Int;

		fn test<'a, I: Int + Scanner<'a> + Show>(check_past: bool) {
			let zero: I = Int::zero();
			let min: I = Int::min_value();
			let max: I = Int::max_value();

			assert!(scan_a::<I>("").err().is_some());
			assert!(scan_a::<I>("0").ok().unwrap().0 == zero);
			assert!(scan_a::<I>(format!("{}", min).as_slice()).ok().unwrap().0 == min);
			assert!(scan_a::<I>(format!("{}", max).as_slice()).ok().unwrap().0 == max);

			if check_past {
				let past_min: i64 = (min.to_i64()).unwrap() - 1;
				let past_max: u64 = (max.to_u64()).unwrap() + 1;
				assert!(scan_a::<I>(format!("{}", past_min).as_slice()).err().is_some());
				assert!(scan_a::<I>(format!("{}", past_max).as_slice()).err().is_some());
			}
		}

		test::<i8>(true);
		test::<i16>(true);
		test::<i32>(true);
		test::<i64>(false);
		test::<u8>(true);
		test::<u16>(true);
		test::<u32>(true);
		test::<u64>(false);
	}

	#[test]
	fn test_int() {
		assert!(scan_a::<int>("").err().is_some());
		assert!(scan_a::<int>("0").ok().unwrap().0 == 0);
		assert!(scan_a::<int>("42").ok().unwrap().0 == 42);
		assert!(scan_a::<int>("1_234").ok().unwrap().0 == 1);
		assert!(scan_a::<int>("x").err().is_some());
		assert!(scan_a::<int>("0x").ok().unwrap().0 == 0);
		assert!(scan_a::<int>("42x").ok().unwrap().0 == 42);
		assert!(scan_a::<int>("-").err().is_some());
		assert!(scan_a::<int>("-0").ok().unwrap().0 == 0);
		assert!(scan_a::<int>("-42").ok().unwrap().0 == -42);
		assert!(scan_a::<int>("-1_234").ok().unwrap().0 == -1);
	}

	#[test]
	fn test_uint() {
		assert!(scan_a::<uint>("").err().is_some());
		assert!(scan_a::<uint>("0").ok().unwrap().0 == 0);
		assert!(scan_a::<uint>("42").ok().unwrap().0 == 42);
		assert!(scan_a::<uint>("1_234").ok().unwrap().0 == 1);
		assert!(scan_a::<uint>("x").err().is_some());
		assert!(scan_a::<uint>("0x").ok().unwrap().0 == 0);
		assert!(scan_a::<uint>("42x").ok().unwrap().0 == 42);
		assert!(scan_a::<uint>("-").err().is_some());
		assert!(scan_a::<uint>("-0").err().is_some());
		assert!(scan_a::<uint>("-42").err().is_some());
		assert!(scan_a::<uint>("-1_234").err().is_some());
	}

	#[test]
	fn test_str() {
		assert!(scan_a::<&str>("").err().is_some());
		assert!(scan_a::<&str>("a").ok().unwrap().0 == "a");
		assert!(scan_a::<&str>("a b").ok().unwrap().0 == "a");
		assert!(scan_a::<&str>("abc").ok().unwrap().0 == "abc");
		assert!(scan_a::<&str>("ab-c").ok().unwrap().0 == "ab");
	}
}
