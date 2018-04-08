extern crate rand;

mod combinator;
mod value;

pub use combinator::*;
pub use value::*;

// many (done)
// many1 (done)
// and (done)
// or (done)
// remote (call another production) (done)
// ch (done)
// byte
// not
// choice (one of n) (done)
// option (done)
// token
// string (done)
// sep_by
// sep_by1
// at_most (up to n times)
// at_least (at least n times)
// exactly (exactly n times)
// sequence

pub trait Generator {
    fn generate(&self) -> Vec<u8>;
}
