use std::from_str::FromStr;

use super::ScanCursor;
use super::{ScanError, OtherScanError};

macro_rules! from_str_scanner {
	($scan_fn:path -> $T:ty as $name:expr) => {
		impl<'a> Scanner<'a, $T> for $T {
			fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<($T, Cur), ScanError> {
				let err = |cur:&Cur| Err(OtherScanError(format!(concat!("expected ",$name,", got `{}`"), cur.tail_str()), cursor.consumed()));

				let end = match $scan_fn(cursor.tail_str()) {
					Some(i) => i,
					None => return err(cursor)
				};

				let s = cursor.str_slice_to(end);
				let cursor = cursor.slice_from(end);

				FromStr::from_str(s)
					.map(ref |i| Ok((i, cursor.clone())))
					.unwrap_or_else(|| err(&cursor))
			}
		}
	};
}

pub trait Scanner<'a, T> {
	fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<(Self, Cur), ScanError>;
}

impl<'a> Scanner<'a, bool> for bool {
	fn scan<Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<(bool, Cur), ScanError> {
		cursor.expect_tok("true").map(|c| (true, c))
			.or_else(|_| cursor.expect_tok("false").map(|c| (false, c)))
			.or_else(|_| Err(OtherScanError(format!("expected `true` or `false`, got `{}`", cursor.tail_str()), cursor.consumed())))
	}
}

from_str_scanner! { scan_float -> f32 as "real number" }
from_str_scanner! { scan_float -> f64 as "real number" }
from_str_scanner! { scan_int -> i8 as "8-bit integer" }
from_str_scanner! { scan_int -> i16 as "16-bit integer" }
from_str_scanner! { scan_int -> i32 as "32-bit integer" }
from_str_scanner! { scan_int -> i64 as "64-bit integer" }
from_str_scanner! { scan_int -> int as "integer" }
from_str_scanner! { scan_uint -> u8 as "8-bit unsigned integer" }
from_str_scanner! { scan_uint -> u16 as "16-bit unsigned integer" }
from_str_scanner! { scan_uint -> u32 as "32-bit unsigned integer" }
from_str_scanner! { scan_uint -> u64 as "64-bit unsigned integer" }
from_str_scanner! { scan_uint -> uint as "unsigned integer" }

fn next_char_at(s: &str, at: uint) -> uint {
	let ::std::str::CharRange { ch: _, next } = s.char_range_at(at);
	next
}

fn scan_float(s: &str) -> Option<uint> {
	enum State {
		Start,
		Whole,
		Suffix,
		ExponentStart,
		Exponent,
	}

	let mut state = Start;

	for (i,c) in s.char_indices() {
		match state {
			Start => {
				assert!(i == 0);
				match c {
					'0'..'9' | '-' => state = Whole,
					_ => return None
				}
			},
			Whole => match c {
				'0'..'9' => (),
				'.' => state = Suffix,
				'e' | 'E' => state = ExponentStart,
				_ => return Some(i)
			},
			Suffix => match c {
				'0'..'9' => (),
				'e' | 'E' => state = ExponentStart,
				_ => return Some(i),
			},
			ExponentStart => match c {
				'+' | '-' | '0'..'9' => state = Exponent,
				_ => return Some(i)
			},
			Exponent => match c {
				'0'..'9' => (),
				_ => return Some(i)
			}
		}
	}

	return Some(s.len())
}

fn scan_uint<'a>(s: &'a str) -> Option<uint> {
	s.char_indices()
		.take_while(|&(_,c)| '0' <= c && c <= '9')
		.map(|(i,_)| Some(next_char_at(s, i)))
		.last().unwrap_or(None)
}

fn scan_int<'a>(s: &'a str) -> Option<uint> {
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

	fn cur<'a>(s: &'a str) -> Cursor<'a, WordsAndInts, Ignore> {
		Cursor::new(s, WordsAndInts, Ignore)
	}

	fn scan_a<'a, T: Scanner<'a, T>>(s: &'a str) -> Result<(T, Cursor<'a, WordsAndInts, Ignore>), ScanError> {
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
	fn test_floats() {
		use std::from_str::FromStr;
		use std::num::{Float, Zero};

		fn test<'a, F: Float + Scanner<'a, F> + FromStr>() {
			let f = |v:F| v;
			let fs = |s:&str| -> F FromStr::from_str(s).unwrap();
			
			assert!(scan_a::<F>("").err().is_some());
			assert!(scan_a::<F>("0").ok().unwrap().0 == f(Zero::zero()));
			assert!(scan_a::<F>("0.0").ok().unwrap().0 == f(Zero::zero()));
			assert!(scan_a::<F>("-0").ok().unwrap().0 == -f(Zero::zero()));
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
		use std::num::{Bounded, Int, Zero};

		fn test<'a, I: Int + Scanner<'a, I> + Show>(check_past: bool) {
			let zero: I = Zero::zero();
			let min: I = Bounded::min_value();
			let max: I = Bounded::max_value();

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
}
