//-- ./src/main.rs

#![allow(unused)] // For beginning only.

use crate::prelude::*;

mod error;
mod prelude;
mod utils;

/// Binary entry point
fn main() -> Result<()> {
	println!("Hello, world!");

	Ok(())
}
