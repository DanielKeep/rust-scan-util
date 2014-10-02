use super::{Cursor, Tokenizer, Whitespace};
use super::{ScanError, OtherScanError};

pub trait Scanner<T> {
	fn scan<'a, Tok: Tokenizer, Sp: Whitespace>(cursor: &Cursor<'a, Tok, Sp>) -> Result<(Self, Cursor<'a, Tok, Sp>), ScanError>;
	fn scanned_value(self) -> T;
}

impl Scanner<int> for int {
	fn scan<'a, Tok: Tokenizer, Sp: Whitespace>(cursor: &Cursor<'a, Tok, Sp>) -> Result<(int, Cursor<'a, Tok, Sp>), ScanError> {
		use std::from_str::FromStr;

		let int_str_end = scan_int(cursor.tail_str());

		let int_str_end = match int_str_end {
			Some(i) => i,
			None => {
				return Err(OtherScanError(format!("expected integer, got `{}`", cursor.tail_str()), cursor.consumed()));
			}
		};

		let int_str = cursor.str_slice_to(int_str_end);
		let cursor = cursor.slice_from(int_str_end);

		FromStr::from_str(int_str)
			.map(ref |i| Ok((i, cursor.clone())))
			.unwrap_or_else(|| Err(OtherScanError(format!("expected integer, got `{}`", int_str), cursor.consumed())))
	}

	fn scanned_value(self) -> int {
		self
	}
}

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
}
