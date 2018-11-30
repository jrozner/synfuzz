extern crate rand;

#[cfg(test)]
extern crate regex;
#[macro_use]
extern crate log;

mod combinator;
mod value;

pub use combinator::*;
pub use value::*;

/// A trait for all Generators to implement. This allows pervasive use of
/// impl trait throughout the implementations of the various Generators and
/// allows not specifying concrete types.
pub trait Generator: ::std::fmt::Debug {
    /// Generate a value from the specific implementation of the Generator
    fn generate(&self) -> Vec<u8>;

    fn generate_lazy<I: Iterator>(&self) -> GenerateIter<I>;

    /// Generate a value of the negation of the specified Generator
    fn negate(&self) -> Vec<u8>;
}

struct GenerateIter<I: Iterator> {
    iter: I,
}

impl<I: Iterator> Iterator for GenerateIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        None
    }
}
