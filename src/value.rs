use super::Generator;

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
}

/// byte is a helper to create a ByteLiteral Generator
pub fn byte(byte: u8) -> impl Generator {
    ByteLiteral { byte: byte }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch() {
        let generator = ch('a');
        assert_eq!(generator.generate(), vec![0x61]);
    }

    #[test]
    fn test_string() {
        let generator = string("this is a test");
        assert_eq!(generator.generate(), "this is a test".as_bytes());
    }

    #[test]
    fn test_byte() {
        let generator = byte(0x42);
        assert_eq!(generator.generate(), vec![0x42]);
    }
}
