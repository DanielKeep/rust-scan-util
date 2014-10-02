use super::{Cursor, Tokenizer, Whitespace};
use super::{ScanError, OtherScanError};

pub trait Scanner<T> {
	fn scan<'a, Tok: Tokenizer, Sp: Whitespace>(cursor: &Cursor<'a, Tok, Sp>) -> Result<(Self, Cursor<'a, Tok, Sp>), ScanError>;
	fn scanned_value(self) -> T;
}

impl Scanner<int> for int {
	fn scan<'a, Tok: Tokenizer, Sp: Whitespace>(cursor: &Cursor<'a, Tok, Sp>) -> Result<(int, Cursor<'a, Tok, Sp>), ScanError> {
		use std::from_str::FromStr;
		debug!("int.scan({})", cursor);

		let int_str_end = cursor.tail_str().char_indices()
			.take_while(|&(_,c)| '0' <= c && c <= '9')
			.map(|(i,_)| Some(i))
			.last().unwrap_or(None);

		let int_str_end = match int_str_end {
			Some(i) => {
				let ::std::str::CharRange { ch:_, next } = cursor.tail_str().char_range_at(i);
				next
			},
			None => {
				return Err(OtherScanError(format!("expected integer, got `{}`", cursor.tail_str()), cursor.consumed()));
			}
		};

		let int_str = cursor.str_slice_to(int_str_end);
		let cursor = cursor.slice_from(int_str_end);

		debug!(" - int_str: `{}`", int_str);
		debug!(" - cursor: {}", cursor);

		FromStr::from_str(int_str)
			.map(ref |i| Ok((i, cursor.clone())))
			.unwrap_or_else(|| Err(OtherScanError(format!("expected integer, got `{}`", int_str), cursor.consumed())))
	}

	fn scanned_value(self) -> int {
		self
	}
}
