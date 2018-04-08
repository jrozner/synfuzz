extern crate rand;

mod combinator;
mod value;

pub use combinator::*;
pub use value::*;

// many (done)
// many1 (done)
// remote (call another production) (done)
// ch (done)
// byte (done)
// not
// choice (one of n) (done)
// option (done)
// token
// string (done)
// sep_by
// sep_by1
// range (n..m times)
// repeat_n (exactly n times) (done)
// seq (done)

pub trait Generator {
    fn generate(&self) -> Vec<u8>;
}
