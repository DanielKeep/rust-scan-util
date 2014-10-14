use super::{Tokenizer, Whitespace};
use super::{ScanError, OtherScanError};

use std::str::CharRange;

pub trait ScanCursor<'scanee>: Clone + Eq {
	fn expect_tok(&self, s: &str) -> Result<Self, ScanError>;
	fn consumed(&self) -> uint;
	fn pop_token(&self) -> Option<(&'scanee str, Self)>;
	fn pop_ws(&self) -> Self;
	fn slice_from(&self, from: uint) -> Self;
	fn str_slice_to(&self, to: uint) -> &'scanee str;
	fn tail_str(&self) -> &'scanee str;
	fn is_empty(&self) -> bool;

	fn expect_eof(&self) -> Result<(), ScanError> {
		if self.pop_token().is_some() {
			Err(self.expected_eof())
		} else {
			Ok(())
		}
	}

	fn expected(&self, desc: &str) -> ScanError {
		let msg = match self.pop_token() {
			Some((got, _)) => format!("expected {}, got `{}`", desc, got),
			None => format!("expected {}, got end of input", desc)
		};

		OtherScanError(msg, self.consumed())
	}

	fn expected_tok(&self, tok: &str) -> ScanError {
		let toks = [tok];
		self.expected_one_of(&toks)
	}

	fn expected_eof(&self) -> ScanError {
		self.expected_one_of(&[])
	}

	fn expected_one_of(&self, toks: &[&str]) -> ScanError {
		let mut toks = toks.iter().map(|s| format!("`{}`", s));
		let toks = {
			let first = toks.next();
			first.map(|first| toks.fold(first, |a,b| format!("{}, {}", a, b)))
		};

		let msg = match (toks, self.pop_token()) {
			(Some(exp), Some((got, _))) => format!("expected {}, got `{}`", exp, got),
			(Some(exp), None) => format!("expected {}, got end of input", exp),
			(None, Some((got, _))) => format!("expected end of input, got `{}`", got),
			(None, None) => "expected end of input".into_string()
		};

		OtherScanError(msg, self.consumed())
	}

	fn expected_min_repeats(&self, min: uint, got: uint) -> ScanError {
		OtherScanError(format!("expected at least {} repeats, got {}", min, got), self.consumed())
	}
}

#[deriving(Clone, Eq, PartialEq, Show)]
pub struct Cursor<'a, Tok: Tokenizer, Sp: Whitespace> {
	slice: &'a str,
	offset: uint,
	tc: Tok,
	sp: Sp,
}

impl<'a, Tok: Tokenizer, Sp: Whitespace> Cursor<'a, Tok, Sp> {
	pub fn new<'b>(s: &'b str, tc: Tok, sp: Sp) -> Cursor<'b, Tok, Sp> {
		Cursor {
			slice: s,
			offset: 0,
			tc: tc,
			sp: sp,
		}
	}
}

impl<'a, Tok: Tokenizer, Sp: Whitespace> ScanCursor<'a> for Cursor<'a, Tok, Sp> {
	fn expect_tok(&self, s: &str) -> Result<Cursor<'a, Tok, Sp>, ScanError> {
		debug!("{}.expect_tok({})", self, s);
		match self.pop_token() {
			Some((tok, ref cur)) if eq_ignore_case(s, tok) => Ok(cur.clone()),
			_ => Err(self.expected_tok(s))
		}
	}

	fn consumed(&self) -> uint {
		self.offset
	}

	fn pop_token(&self) -> Option<(&'a str, Cursor<'a, Tok, Sp>)> {
		debug!("{}.pop_token()", self);
		let cur = self;

		// First, check to see if there is a whitespace token.  This allows the space policy to do things like ignore most whitespace, but turn line breaks into explicit tokens.
		match self.sp.token_len(cur.tail_str()) {
			Some(end) => return Some((cur.str_slice_to(end), cur.slice_from(end))),
			None => ()
		}

		// If that didn't work, strip out leading whitespace.
		let cur = self.pop_ws();

		// Do not assume that empty input means we can't match a token; the token class might, for example, turn end-of-input into an explicit token.
		let tail_str = cur.tail_str();
		match self.tc.token_len(tail_str) {
			Some(end) => Some((cur.str_slice_to(end), cur.slice_from(end))),
			None => {
				// One of two things: either we have some input left and will thus return a single-character token, or there is nothing left whereby we return None.
				if cur.is_empty() {
					return None;
				} else {
					let CharRange { ch: _, next } = tail_str.char_range_at(0);
					Some((cur.str_slice_to(next), cur.slice_from(next)))
				}
			},
		}
	}

	fn pop_ws(&self) -> Cursor<'a, Tok, Sp> {
		debug!("{}.pop_ws()", self);

		self.slice_from(self.sp.strip_len(self.tail_str()))
	}

	fn slice_from(&self, from: uint) -> Cursor<'a, Tok, Sp> {
		Cursor {
			offset: ::std::cmp::min(self.slice.len(), self.offset + from),
			..self.clone()
		}
	}

	fn str_slice_to(&self, to: uint) -> &'a str {
		self.tail_str().slice_to(to)
	}

	fn tail_str(&self) -> &'a str {
		self.slice.slice_from(self.offset)
	}

	fn is_empty(&self) -> bool {
		self.offset == self.slice.len()
	}
}

fn eq_ignore_case<S: Str, T: Str>(left: S, right: T) -> bool {
	let left = left.as_slice();
	let right = right.as_slice();

	(left.len() == right.len() 
		&& (left.chars().zip(right.chars())
			.all(|(l,r)| l.to_lowercase() == r.to_lowercase())))
}
