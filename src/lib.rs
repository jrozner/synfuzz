extern crate rand;

mod combinator;
mod value;

pub use combinator::*;
pub use value::*;

/// A trait for all Generators to implement. This allows pervasive use of
/// impl trait throughout the implementations of the various Generators and
/// allows not specifying concrete types.
pub trait Generator {
    /// Generate a value from the specific implementation of the Generator
    fn generate(&self) -> Vec<u8>;
}
