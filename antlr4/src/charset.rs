use ast::Operation;
use std::char;
use std::iter::Peekable;
use std::str::Chars;

// https://github.com/antlr/antlr4/blob/master/doc/lexer-rules.md

/// parse an ANTLR4 character set into a Choice of all possible generators
pub fn parse_charset(charset: &str) -> Operation {
    let mut chars = charset.chars().peekable();

    let mut choices = vec![];
    let mut state = State::Start;

    loop {
        let ch = match chars.next() {
            Some(ch) => ch,
            None => panic!("unexpected end of charset"),
        };

        if state == State::Start {
            if ch == '[' {
                state = State::InCharset;
            } else {
                panic!("expected `[` and got `{}`", ch);
            };
        } else if state == State::InCharset {
            if ch == ']' {
                state = State::End;
                break;
            } else {
                let first = match ch {
                    '\\' => escape(&mut chars),
                    _ => ch,
                };

                if chars.peek() == Some(&'-') {
                    chars.next();
                    let second_ch = chars.next().unwrap();
                    let second = match second_ch {
                        '\\' => escape(&mut chars),
                        _ => second_ch,
                    };
                    choices.push(Operation::CharRange((first, second)));
                } else {
                    choices.push(Operation::Char(first))
                }
            }
        } else {
            panic!("invalid state")
        }
    }

    Operation::CharacterClass(choices)
}

fn escape(chars: &mut Peekable<Chars>) -> char {
    // TODO: support unicode property names
    match chars.next() {
        Some('u') => {
            // TODO: support curly based code points for > U+FFFF
            let code_point = chars.take(4).collect::<String>();
            let numeric_value = u32::from_str_radix(&code_point, 16).unwrap();
            char::from_u32(numeric_value).expect("invalid codepoint specified")
        }
        Some('n') => '\n',
        Some('r') => '\r',
        Some('b') => 7 as char,
        Some('t') => '\t',
        Some('f') => 12 as char,
        Some(']') => ']',
        Some('\\') => '\\',
        Some('-') => '-',
        None => panic!("unexpected end of input in character set escape sequence"),
        _ => panic!("invalid escape sequence"),
    }
}

#[derive(PartialEq)]
enum State {
    Start,
    InCharset,
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_charset() {
        let tests = vec!["[a-zA-Z0-9]", r#"[\t\b\u0097]"#];

        for test in tests {
            let thing = parse_charset(test);
            println!("{}: {:?}", test, thing);
        }
    }
}
