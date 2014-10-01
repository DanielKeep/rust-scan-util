#![feature(phase)]

#[phase(plugin, link)] extern crate log;

pub use cursor::Cursor;
pub use scan_error::{ScanError, NothingMatched, OtherScanError, ScanIoError};
pub use scanner::{Scanner, NegInt};
pub use tokenizer::Tokenizer;
pub use whitespace::Whitespace;

pub mod cursor;
pub mod scan_error;
pub mod scanner;
pub mod tokenizer;
pub mod whitespace;

fn len_while(s: &str, pred: |char| -> bool) -> Option<uint> {
	s.char_indices()
		.take_while(|&(_,ch)| pred(ch))
		.last()
		.map(|(i,_)| {
			let ::std::str::CharRange { ch: _, next } = s.char_range_at(i);
			next
		})
}
