//-- ./src/utils/direntry_froms.rs

//! Convert a directory to a `String``

#![allow(unused)] // For beginning only.

use crate::prelude::*;
use std::fs::DirEntry;

/// Transform to a String
impl TryFrom<W<&DirEntry>> for String {
	type Error = Error;
	fn try_from(val: W<&DirEntry>) -> Result<String> {
		val.0
			.path()
			.to_str()
			.map(String::from)
			.ok_or_else(|| Error::Generic(format!("Invalid path {:?}", val.0)))
	}
}

#[cfg(test)]
pub mod tests {
	// Bring file/module functions into unit test scope
	use super::*;

	// Override with more flexible error
	pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>;

	#[test]
	fn test_number_1() -> Result<()> {

		Ok(())
	}

}