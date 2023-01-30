pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple for newtype pattern
// (to implement external traits on external types)
pub struct W<T>(pub T);

// Common aliases
pub use std::format as f;
