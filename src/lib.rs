/*!

This is the runtime support library for the [`rust-scan`](https://github.com/DanielKeep/rust-scan) package.  If you want to *use* the `scan*` family of macros, go read the documentation there.  You only need to worry about this package if you intend to:

- Implement a `Scanner` for a type manually, *without* using the `scanner!` macro.
- Implement a custom `Tokenizer`, `Whitespace` or `CompareStrs` type for use at runtime (you sadly cannot provide a custom implementation of these at compile time).
- You want to use one of the utility routines this package happens to have.

# Organisation

As a brief overview of where to look for things:

- `compare_strs`: contains the `CompareStrs` trait and its implementations.  These are used for comparing scanned tokens for equality, and is how case-sensitive/case-insensitive comparisons are implemented.
- `cursor`: contains the `ScanCursor` trait and the concrete `Cursor` type.  These are used to track scanning progress through an input string, and provide tokenisation, whitespace skipping and string comparison to scanners.
- `io`: contains some IO support routines.  Most notably, a `read_line` function that does not require buffering.
- `scan_error`: contains the `ScanError` enumeration, which is (unsurprisingly) used to represent scanning errors.
- `scanner`: contains the `Scanner` trait and the default implementations of it for various basic types.  These are how the `scan*` macros capture values.
- `tokenizer`: contains the `Tokenizer` trait and its implementations.  These are used for extracting a token from an input string.
- `whitespace`: contains the `Whitespace` trait and its implementations.  These are used for both skipping whitespace and turning whitespace into tokens.

## License

This package is provided under the MIT license.

*/
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
