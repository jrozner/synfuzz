use super::Generator;

use rand::distributions::Alphanumeric;
use rand::distributions::Standard;
use rand::thread_rng;
use rand::Rng;
use std::iter;

const STRING_MAX: usize = 12;

/// CharLiteral is a Generator that will return the specified char for each
/// call of the generate method
pub struct CharLiteral {
    ch: char,
}

/// ch is a helper to create a CharLiteral Generator
pub fn ch(ch: char) -> impl Generator {
    CharLiteral { ch: ch }
}

impl Generator for CharLiteral {
    fn generate(&self) -> Vec<u8> {
        let mut s = String::with_capacity(4);
        s.push(self.ch);
        s.into_bytes()
    }

    fn negate(&self) -> Vec<u8> {
        iter::repeat(())
            .map(|_| thread_rng().sample::<char, Standard>(Standard))
            .filter(|x| *x != self.ch)
            .take(1)
            .collect::<String>()
            .into_bytes()
    }
}

/// StringLiteral is a Generator that will return the specified string for
/// each call of the generate method
pub struct StringLiteral {
    s: String,
}

impl Generator for StringLiteral {
    fn generate(&self) -> Vec<u8> {
        Vec::from(self.s.as_bytes())
    }

    fn negate(&self) -> Vec<u8> {
        // TODO: Find a good way to get full unicode with good ut8 only
        let mut rng = thread_rng();
        let chars = rng.gen_range(0, STRING_MAX);
        loop {
            let generated = iter::repeat(())
                .map::<char, _>(|()| rng.sample::<char, Alphanumeric>(Alphanumeric))
                .take(chars)
                .collect::<String>();

            if generated != self.s {
                return generated.as_bytes().to_owned();
            }
        }
    }
}

/// string is a helper to create a StringLiteral Generator
pub fn string<S>(s: S) -> impl Generator
where
    S: Into<String>,
{
    StringLiteral { s: s.into() }
}

/// ByteLiteral is a generator that will return the specified byte for each
/// call of the generate method
pub struct ByteLiteral {
    byte: u8,
}

impl Generator for ByteLiteral {
    fn generate(&self) -> Vec<u8> {
        vec![self.byte]
    }

    fn negate(&self) -> Vec<u8> {
        iter::repeat(())
            .map(|_| thread_rng().sample(Standard))
            .filter(|x| *x != self.byte)
            .take(1)
            .collect()
    }
}

/// byte is a helper to create a ByteLiteral Generator
pub fn byte(byte: u8) -> impl Generator {
    ByteLiteral { byte: byte }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_ch() {
        let generator = ch('a');
        assert_eq!(generator.generate(), vec![0x61]);
    }

    #[test]
    fn negate_ch() {
        let generator = ch('a');
        assert_ne!(generator.negate(), vec![0x61]);
    }

    #[test]
    fn generate_string() {
        let generator = string("this is a test");
        assert_eq!(generator.generate(), "this is a test".as_bytes());
    }

    #[test]
    fn negate_string() {
        let generator = string("this is a test");
        let generated = generator.negate();
        assert_ne!(generated, "this is a test".as_bytes());
        assert!(generated.len() < STRING_MAX);
    }

    #[test]
    fn generate_byte() {
        let generator = byte(0x42);
        assert_eq!(generator.generate(), vec![0x42]);
    }

    #[test]
    fn negate_byte() {
        let generator = byte(0x42);
        assert_ne!(generator.negate(), vec![0x42]);
    }
}
