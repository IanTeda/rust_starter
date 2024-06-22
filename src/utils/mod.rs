//-- ./src/utils

#![allow(unused)] // For beginning only.

//! Utility modules that don't fit into other places

//-- UNIT TESTS ----------------------------------------------------------------
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