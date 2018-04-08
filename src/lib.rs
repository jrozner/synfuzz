extern crate rand;

mod combinator;
mod value;

pub use combinator::*;
pub use value::*;

pub trait Generator {
    fn generate(&self) -> Vec<u8>;
}
