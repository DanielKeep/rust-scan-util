/*!
This module provides the `ScanError` type, which encodes the various kinds of errors that can arise during scanning.
*/
use std::fmt;
use std::fmt::{Formatter, FormatError};

pub use self::ScanError::{OtherScanError, ScanIoError};

pub type ScanResult<T> = Result<T, ScanError>;

/**
This is used to indicate why a scan has failed.

If you format a `ScanError` with `{:s}`, you will get a nicely formatted version of the error.  If Rust grows a `Repr` formatting trait in the future, it is likely that the current `Show` will become `Repr`, and `String` will become `Show`.
*/
#[deriving(Clone, Eq, PartialEq, Show)]
pub enum ScanError {
	/**
Some other scan error occurred.  The `String` is the message describing the problem, the `uint` is the offset within the input at which the error occurred.
	*/
	OtherScanError(String, uint),
	/**
Indicates that an underlying IO operation failed.
	*/
	ScanIoError(::std::io::IoError),
}

impl ScanError {
	/**
Takes two `ScanError` values and returns the "most interesting" one.  The general rules are:

* An IO error takes precedence over anything else.
* Scan errors which happened further along the input take precedence.  This should hopefully be the error from the most relevant arm.
	*/
	pub fn or(self, other: ScanError) -> ScanError {
		match (self, other) {
			(ScanIoError(ioerr), _) | (_, ScanIoError(ioerr)) => ScanIoError(ioerr),
			(OtherScanError(msga, offa), OtherScanError(msgb, offb)) => {
				if offa > offb {
					OtherScanError(msga, offa)
				} else {
					OtherScanError(msgb, offb)
				}
			}
		}
	}
}

impl fmt::String for ScanError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
		match self {
			&OtherScanError(ref msg, at) => write!(f, "at offset {}: {}", at, msg),
			&ScanIoError(ref err) => write!(f, "io error: {}", err),
		}
	}
}
