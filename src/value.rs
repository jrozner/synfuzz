use super::Generator;

pub struct Ch {
    ch: char,
}

pub fn ch(ch: char) -> impl Generator {
    Ch { ch: ch }
}

impl Generator for Ch {
    fn generate(&self) -> Vec<u8> {
        let mut s = String::with_capacity(4);
        s.push(self.ch);
        s.into_bytes()
    }
}

pub struct StringLiteral {
    s: String,
}

impl Generator for StringLiteral {
    fn generate(&self) -> Vec<u8> {
        Vec::from(self.s.as_bytes())
    }
}

pub fn string<S>(s: S) -> impl Generator
where
    S: Into<String>,
{
    StringLiteral { s: s.into() }
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
}
