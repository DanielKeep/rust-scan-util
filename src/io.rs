/*!
This module provides some miscellaneous IO support routines.
*/

use std::io::{IoError, IoResult, OtherIoError};

/**
Reads a line of input from the given `Reader`.  This does not require a push-back buffer.  It returns the line *with* the line terminator.

Note that this function *does not* support old-school Mac OS newlines (i.e. a single carriage return).  If it encounters a carriage return which is *not* immediately followed by a line feed, the carriage return will be included as part of the line.
*/
pub fn read_line<R: Reader>(r: &mut R) -> IoResult<String> {
	let mut line = String::new();
	loop {
		match read_utf8_char(r) {
			Ok('\n') => {
				line.push('\n');
				break;
			},
			Ok(c) => {
				line.push(c);
			}
			Err(err) => {
				if err.kind == ::std::io::EndOfFile && line.len() > 0 {
					break
				} else {
					return Err(err)
				}
			}
		}
	}
	Ok(line)
}

#[test]
fn test_read_line() {
	use std::borrow::ToOwned;

	let s = "line one\nline two\r\nline three\n";
	let mut r = ::std::io::BufReader::new(s.as_bytes());
	let oks = |s:&str| Ok(s.to_owned());

	assert_eq!(read_line(&mut r), oks("line one\n"));
	assert_eq!(read_line(&mut r), oks("line two\r\n"));
	assert_eq!(read_line(&mut r), oks("line three\n"));
}

/**
Reads a single UTF-8 encoded Unicode code point from a `Reader`.
*/
pub fn read_utf8_char<R: Reader>(r: &mut R) -> IoResult<char> {
	fn invalid_utf8<T>(b: u8, initial: bool) -> IoResult<T> {
		Err(IoError {
			kind: OtherIoError,
			desc: "invalid utf-8 sequence",
			detail: if initial {
				Some(format!("invalid initial code unit {:#02x}", b))
			} else {
				Some(format!("invalid continuation code unit {:#02x}", b))
			}
		})
	}

	fn invalid_cp<T>(cp: u32) -> IoResult<T> {
		Err(IoError {
			kind: OtherIoError,
			desc: "invalid Unicode code point",
			detail: Some(format!("invalid code point {:#08x}", cp))
		})
	}

	// Why not use std::str::utf8_char_width?  We need to know the encoding to mask away the size bits anyway.
	let (mut cp, n) = match try!(r.read_u8()) {
		b @ 0b0000_0000 ... 0b0111_1111 => (b as u32, 0),
		b @ 0b1100_0000 ... 0b1101_1111 => ((b & 0b0001_1111) as u32, 1),
		b @ 0b1110_0000 ... 0b1110_1111 => ((b & 0b0000_1111) as u32, 2),
		b @ 0b1111_0000 ... 0b1111_0111 => ((b & 0b0000_0111) as u32, 3),
		b @ 0b1111_1000 ... 0b1111_1011 => ((b & 0b0000_0011) as u32, 4),
		b @ 0b1111_1100 ... 0b1111_1101 => ((b & 0b0000_0001) as u32, 5),
		b => return invalid_utf8(b, true)
	};

	for _ in range(0u, n) {
		let b = match try!(r.read_u8()) {
			b @ 0b10_000000 ... 0b10_111111 => (b & 0b00_111111) as u32,
			b => return invalid_utf8(b, false)
		};
		cp = (cp << 6) | b;
	}

	::std::char::from_u32(cp)
		.map(|c| Ok(c))
		.unwrap_or_else(|| invalid_cp(cp))
}

#[test]
fn test_read_utf8_char() {
	fn test_str(s: &str) {
		let mut reader = ::std::io::BufReader::new(s.as_bytes());
		for c in s.chars() {
			assert_eq!(Ok(c), read_utf8_char(&mut reader))
		}
	}

	fn first(s: &[u8]) -> IoResult<char> {
		let mut reader = ::std::io::BufReader::new(s);
		read_utf8_char(&mut reader)
	}

	test_str("abcdef");
	test_str("私の日本語わ下手ですよ！");

	assert!(first(&[0b1000_0000u8]).is_err());
	assert!(first(&[0b1100_0000u8, 0b0000_0000]).is_err());
}

/**
Reads a single line from standard input.
*/
pub fn stdin_read_line() -> IoResult<String> {
	read_line(&mut ::std::io::stdio::stdin_raw())
}
