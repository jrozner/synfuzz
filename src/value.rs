use super::Generator;

use rand::distributions::Alphanumeric;
use rand::distributions::Standard;
use rand::thread_rng;
use rand::Rng;
use std::iter;

const STRING_MAX: usize = 12;

/// CharLiteral is a Generator that will return the specified char for each
/// call of the generate method
#[derive(Debug)]
pub struct CharLiteral {
    pub ch: char,
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
#[derive(Debug)]
pub struct StringLiteral {
    pub s: String,
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
#[derive(Debug)]
pub struct ByteLiteral {
    pub byte: u8,
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

/// CharRange is a generator that will return bytes that represent a char between
/// n and m inclusively. This is useful for implementing ranges of chars such as
/// in a regular expression's character set
#[derive(Debug)]
pub struct CharRange {
    pub n: char,
    pub m: char,
}

impl Generator for CharRange {
    fn generate(&self) -> Vec<u8> {
        let c = thread_rng().gen_range(self.n as u8, self.m as u8) as char;
        let mut s = String::with_capacity(4);
        s.push(c);
        s.into_bytes()
    }

    fn negate(&self) -> Vec<u8> {
        unimplemented!()
    }
}

/// char_range is a helper to create a CharRange Generator
pub fn char_range(n: char, m: char) -> impl Generator {
    CharRange { n: n, m: m }
}

/// Any is a Generator that generates one character worth of value
#[derive(Debug)]
pub struct Any {}

impl Generator for Any {
    fn generate(&self) -> Vec<u8> {
        let mut rng = thread_rng();
        iter::repeat(())
            .map::<char, _>(|()| rng.sample::<char, Alphanumeric>(Alphanumeric))
            .take(1)
            .collect::<String>()
            .as_bytes()
            .to_owned()
    }

    fn negate(&self) -> Vec<u8> {
        unimplemented!()
    }
}

/// any is a helper to create an Any Generator
pub fn any() -> impl Generator {
    Any {}
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

    #[test]
    fn generate_char_range() {
        let generator = char_range('a', 'c');
        let generated = generator.generate();
        let c = generated[0];
        assert!(c >= 0x61 && c <= 0x63);
    }

    #[test]
    fn generate_any() {
        let generator = any();
        let generated = generator.generate();
        assert!(String::from_utf8(generated).is_ok());
    }
}
