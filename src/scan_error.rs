use std::fmt;
use std::fmt::{Formatter, FormatError};

/*
This is used to indicate why a scan has failed.
*/
#[deriving(Clone, Eq, PartialEq, Show)]
pub enum ScanError {
	NothingMatched,
	OtherScanError(String, uint),
	ScanIoError(::std::io::IoError),
}

impl ScanError {
	/*
	Takes two `ScanError` values and returns the "most interesting" one.  The general rules are:

	* An IO error takes precedence over anything else.
	* Scan errors which happened further along the input take precedence.  This should hopefully be the error from the most relevant arm.
	*/
	pub fn or(self, other: ScanError) -> ScanError {
		match (self, other) {
			(ScanIoError(ioerr), _) | (_, ScanIoError(ioerr)) => ScanIoError(ioerr),
			(NothingMatched, other) | (other, NothingMatched) => other,
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
			&NothingMatched => write!(f, "nothing matched"),
			&OtherScanError(ref msg, at) => write!(f, "at offset {}: {}", at, msg),
			&ScanIoError(ref err) => write!(f, "io error: {}", err),
		}
	}
}
