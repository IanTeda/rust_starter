//-- ./src/prelude.rs

//! A collection of names that are automatically brought into scope of every module in a crate.

// Re-export the crate Error.
pub use crate::error::Error;

// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for new type pattern, mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);
