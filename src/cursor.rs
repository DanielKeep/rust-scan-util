/*!
This module provides the `ScanCursor` trait, and its concrete `Cursor` implementation.

These are used by the generated code as a way to track scanning progress through an input string, and to centralise various bits of functionality.
*/
use super::{Tokenizer, Whitespace, CompareStrs};
use super::{ScanError, OtherScanError};

use std::fmt::{mod, Show, Formatter};
use std::str::CharRange;

/**
The `ScanCursor` trait serves several purposes:

- It keeps track of progress through an input string; specifically, the offset into the input.
- It provides centralised access to various pieces of scanning functionality: tokenisation, whitespace skipping/tokenisation and string comparisons.
- It provides convenience methods for constructing errors.

The reason this trait exists, rather than directly using the `Cursor` type was to simplify type signatures.
*/
pub trait ScanCursor<'scanee>: Clone + Eq {
	/**
Return a successor cursor if the next token matches the provided string.
	*/
	fn expect_tok(&self, s: &str) -> Result<Self, ScanError>;

	/**
Return the number of bytes consumed by this cursor, relative to the start of the input.
	*/
	fn consumed(&self) -> uint;

	/**
Pop the next token, returning a slice of the input and the successor cursor.

If there are no further tokens in the input, returns `None`.
	*/
	fn pop_token(&self) -> Option<(&'scanee str, Self)>;

	/**
Return a successor cursor with all leading, irrelevant whitespace skipped.  This will always succeed.
	*/
	fn pop_ws(&self) -> Self;

	/**
Return a successor cursor which is `from` bytes further along than the current one.
	*/
	fn slice_from(&self, from: uint) -> Self;

	/**
Return a slice of the input from the current cursor position, which is `to` bytes in length.
	*/
	fn str_slice_to(&self, to: uint) -> &'scanee str;

	/**
Return a slice of the input between the current cursor and the `to` cursor.  This slice is left inclusive, right exclusive.
	*/
	fn str_slice_to_cur(&self, to: &Self) -> &'scanee str;

	/**
Return a slice of the input string, from the current cursor position to the end of input.
	*/
	fn tail_str(&self) -> &'scanee str;

	/**
Return `true` if and only if there is no input remaining.  That is, this indicates if the current cursor is at the end of the input.

**Note**: depending on the tokeniser, this *might not* be equivalent to the statement "there are no tokens left".
	*/
	fn is_empty(&self) -> bool;

	/**
Compare two strings for equality, using the cursor's string comparator.
	*/
	fn compare_strs(&self, a: &str, b: &str) -> bool;

	/**
Returns a nil result if there are no remaining tokens in the input.

**Note**: depending on the tokeniser, this *might not* be equivalent to the statement "the cursor is at the end of the input".
	*/
	fn expect_eof(&self) -> Result<(), ScanError> {
		if self.pop_token().is_some() {
			Err(self.expected_eof())
		} else {
			Ok(())
		}
	}

	/**
Create a `ScanError` tied to the current position, providing `desc` as an explanation.  The generated message will include the next token which (presumably) was not what you expected.
	*/
	fn expected(&self, desc: &str) -> ScanError {
		let msg = match self.pop_token() {
			Some((got, _)) => format!("expected {}, got `{}`", desc, got.escape_default()),
			None => format!("expected {}, got end of input", desc)
		};

		OtherScanError(msg, self.consumed())
	}

	/**
Create a `ScanError` tied to the current position, indicating that you expected a specific token `tok`.  The generated message will include the next token which was not what you expected.
	*/
	fn expected_tok(&self, tok: &str) -> ScanError {
		let toks = [tok];
		self.expected_one_of(&toks)
	}

	/**
Create a `ScanError` tied to the current position, indicating that you expected to reach the end of input.  The generated message will include the next token.
	*/
	fn expected_eof(&self) -> ScanError {
		self.expected_one_of(&[])
	}

	/**
Create a `ScanError` tied to the current position, indicating that you expected one of a specific set of tokens, `toks`.  The generated message will include the next token which was not what you expected.

When a single token is provided, this is equivalent to `expected_tok`.  When no tokens are provided, this is equivalent to `expected_eof`.
	*/
	fn expected_one_of(&self, toks: &[&str]) -> ScanError {
		let mut toks = toks.iter().map(|s| format!("`{}`", s.escape_default()));
		let toks = {
			if let Some(first) = toks.next() {
				Some(toks.fold(first, |a,b| format!("{}, {}", a, b)))
			} else {
				None
			}
		};

		let msg = match (toks, self.pop_token()) {
			(Some(exp), Some((got, _))) => format!("expected {}, got `{}`", exp, got.escape_default()),
			(Some(exp), None) => format!("expected {}, got end of input", exp),
			(None, Some((got, _))) => format!("expected end of input, got `{}`", got.escape_default()),
			(None, None) => "expected end of input".into_string()
		};

		OtherScanError(msg, self.consumed())
	}

	/**
Create a `ScanError` tied to the current position, indicating that you expected a certain minimum number of repeats.  This is a convenience method for the code generated by the repeat pattern construct.
	*/
	fn expected_min_repeats(&self, min: uint, got: uint) -> ScanError {
		OtherScanError(format!("expected at least {} repeats, got {}", min, got), self.consumed())
	}
}

/**
This structure implements the `ScanCursor` trait.
*/
#[deriving(Clone, Eq, PartialEq)]
pub struct Cursor<'a, Tok: Tokenizer, Sp: Whitespace, Cs: CompareStrs> {
	slice: &'a str,
	offset: uint,
	tc: Tok,
	sp: Sp,
	cs: Cs,
}

impl<'a, Tok: Tokenizer, Sp: Whitespace, Cs: CompareStrs> Cursor<'a, Tok, Sp, Cs> {
	pub fn new<'b>(s: &'b str, tc: Tok, sp: Sp, cs: Cs) -> Cursor<'b, Tok, Sp, Cs> {
		Cursor {
			slice: s,
			offset: 0,
			tc: tc,
			sp: sp,
			cs: cs,
		}
	}
}

impl<'a, Tok: Tokenizer, Sp: Whitespace, Cs: CompareStrs> Show for Cursor<'a, Tok, Sp, Cs> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		try!(write!(f, "Cursor<{}, {}, {}> {{ offset: {}, .. }}", self.tc, self.sp, self.cs, self.offset));
		Ok(())
	}
}

impl<'a, Tok: Tokenizer, Sp: Whitespace, Cs: CompareStrs> ScanCursor<'a> for Cursor<'a, Tok, Sp, Cs> {
	fn expect_tok(&self, s: &str) -> Result<Cursor<'a, Tok, Sp, Cs>, ScanError> {
		debug!("{}.expect_tok({})", self, s);
		match self.pop_token() {
			Some((tok, ref cur)) if self.compare_strs(s, tok) => Ok(cur.clone()),
			_ => Err(self.expected_tok(s))
		}
	}

	fn consumed(&self) -> uint {
		self.offset
	}

	fn pop_token(&self) -> Option<(&'a str, Cursor<'a, Tok, Sp, Cs>)> {
		debug!("{}.pop_token()", self);
		// First, strip out leading whitespace.  It's up to the whitespace policy to *not* strip characters it wants to turn into a token.
		let cur = self.pop_ws();

		// Next, check to see if there is a whitespace token.  This allows the space policy to do things like ignore most whitespace, but turn line breaks into explicit tokens.  Note that unlike the regular Tokenizer, the Whitespace policy is responsible for returning the str slice itself.  This is used to do things like map all whitespace to a single `" "` token.
		match self.sp.token_len(cur.tail_str()) {
			Some((end, s)) => {
				debug!("{}.pop_token - sp token `{}`", self, s.escape_default());
				return Some((s, cur.slice_from(end)));
			},
			None => ()
		}

		// Do not assume that empty input means we can't match a token; the token class might, for example, turn end-of-input into an explicit token.
		let tail_str = cur.tail_str();
		match self.tc.token_len(tail_str) {
			Some(end) => {
				let tok = cur.str_slice_to(end);
				debug!("{}.pop_token - token `{}`", self, tok.escape_default());
				Some((tok, cur.slice_from(end)))
			},
			None => {
				// One of two things: either we have some input left and will thus return a single-character token, or there is nothing left whereby we return None.
				if cur.is_empty() {
					debug!("{}.pop_token - no token", self);
					return None;
				} else {
					let CharRange { ch: _, next } = tail_str.char_range_at(0);
					let tok = cur.str_slice_to(next);
					debug!("{}.pop_token - def token `{}`", self, tok.escape_default());
					Some((tok, cur.slice_from(next)))
				}
			},
		}
	}

	fn pop_ws(&self) -> Cursor<'a, Tok, Sp, Cs> {
		debug!("{}.pop_ws()", self);

		self.slice_from(self.sp.strip_len(self.tail_str()))
	}

	fn slice_from(&self, from: uint) -> Cursor<'a, Tok, Sp, Cs> {
		Cursor {
			offset: ::std::cmp::min(self.slice.len(), self.offset + from),
			..self.clone()
		}
	}

	fn str_slice_to(&self, to: uint) -> &'a str {
		self.tail_str().slice_to(to)
	}

	fn str_slice_to_cur(&self, to: &Cursor<'a, Tok, Sp, Cs>) -> &'a str {
		self.slice.slice(self.offset, to.offset)
	}

	fn tail_str(&self) -> &'a str {
		self.slice.slice_from(self.offset)
	}

	fn is_empty(&self) -> bool {
		self.offset == self.slice.len()
	}

	fn compare_strs(&self, a: &str, b: &str) -> bool {
		self.cs.compare_strs(a, b)
	}
}
