#![feature(macro_rules)]
#![feature(phase)]

#[phase(plugin, link)] extern crate log;

pub use compare_strs::CompareStrs;
pub use cursor::{Cursor, ScanCursor};
pub use scan_error::{ScanResult, ScanError, OtherScanError, ScanIoError};
pub use scanner::Scanner;
pub use tokenizer::Tokenizer;
pub use whitespace::Whitespace;

pub mod compare_strs;
pub mod cursor;
pub mod io;
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
