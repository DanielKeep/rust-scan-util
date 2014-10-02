use std::from_str::FromStr;

use super::ScanCursor;
use super::{ScanError, OtherScanError};

macro_rules! from_str_scanner {
	($scan_fn:path -> $T:ty as $name:expr) => {
		impl Scanner<$T> for $T {
			fn scan<'a, Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<($T, Cur), ScanError> {
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

pub trait Scanner<T> {
	fn scan<'a, Cur: ScanCursor<'a>>(cursor: &Cur) -> Result<(Self, Cur), ScanError>;
}

from_str_scanner! { scan_int -> int as "integer" }
from_str_scanner! { scan_uint -> uint as "unsigned integer" }

fn next_char_at(s: &str, at: uint) -> uint {
	let ::std::str::CharRange { ch: _, next } = s.char_range_at(at);
	next
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

	fn scan_a<'a, T: Scanner<T>>(s: &'a str) -> Result<(T, Cursor<'a, WordsAndInts, Ignore>), ScanError> {
		Scanner::scan(&cur(s))
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
