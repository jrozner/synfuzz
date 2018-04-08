extern crate rand;

mod combinator;
mod value;

pub use combinator::*;
pub use value::*;

// many
// many1
// and
// or
// remote (call another production)
// ch
// byte
// bytes (matches byte slices; good for magic numbers)
// between
// not
// choice (one of n)
// option
// token
// string
// eof
// sep_by
// sep_by1
// at_most (up to n times)
// at_least (at least n times)
// exactly (exactly n times)

pub trait Generator {
    fn generate(&self) -> Vec<u8>;
}
