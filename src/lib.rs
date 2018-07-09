#[macro_use]
extern crate failure;
extern crate log;
extern crate synfuzz;

pub mod antlr4;
pub mod ast;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;

use synfuzz::{
    byte, register_rule, CharRange, Choice, Generator, JoinWith, Many, Many1, Optional, Rule,
    Rules, Sequence, StringLiteral,
};

/// generate_rules takes the path to an ANTLR4 grammar file and returns a set of
/// rules that represent the parsed file
pub fn generate_rules(path: &str) -> Result<Arc<RwLock<Rules>>, AntlrError> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let parse_tree = match antlr4::GrammarParser::new().parse(&buf) {
        Ok(t) => t,
        Err(_) => return Err(AntlrError::ParseError),
    };
    let rules = Arc::new(RwLock::new(HashMap::new()));

    for rule in parse_tree.rules().iter() {
        let parts = rule
            .body()
            .iter()
            .map(|part| translate_rule(part, &rules))
            .collect::<Vec<Box<dyn Generator>>>();
        match rule.rule_type() {
            ast::RuleType::Parser => {
                let root = JoinWith {
                    generators: parts,
                    delimiter: Box::new(byte(0x20)),
                };
                register_rule(&rules, rule.name(), root);
            }
            ast::RuleType::Lexer | ast::RuleType::Fragment => {
                let root = Sequence { generators: parts };
                register_rule(&rules, rule.name(), root);
            }
        };
    }

    Ok(rules)
}

fn translate_rule(operation: &ast::Operation, rules: &Arc<RwLock<Rules>>) -> Box<dyn Generator> {
    match operation {
        ast::Operation::Alternate(op) => {
            let choices = op
                .iter()
                .map(|alternate| {
                    let parts = alternate
                        .iter()
                        .map(|part| translate_rule(part, rules))
                        .collect::<Vec<Box<dyn Generator>>>();
                    Box::new(JoinWith {
                        generators: parts,
                        delimiter: Box::new(byte(0x20)),
                    }) as Box<dyn Generator>
                }).collect::<Vec<Box<dyn Generator>>>();
            return Box::new(Choice { choices: choices });
        }
        ast::Operation::Any => unimplemented!(),
        ast::Operation::CharacterClass(_) => unimplemented!(),
        ast::Operation::Group(op) => {
            let parts = op
                .iter()
                .map(|part| translate_rule(part, rules))
                .collect::<Vec<Box<dyn Generator>>>();
            Box::new(JoinWith {
                generators: parts,
                delimiter: Box::new(byte(0x20)),
            }) as Box<dyn Generator>
        }
        ast::Operation::Optional(op) => Box::new(Optional {
            generator: translate_rule(op, rules),
        }) as Box<dyn Generator>,
        ast::Operation::Plus(op) => Box::new(Many1 {
            generator: translate_rule(op, rules),
        }) as Box<dyn Generator>,
        ast::Operation::Range((l, r)) => {
            let n = l.chars().next().unwrap();
            let m = r.chars().next().unwrap();
            Box::new(CharRange { n: n, m: m }) as Box<dyn Generator>
        }
        ast::Operation::Rule(op) => Box::new(Rule {
            rules: rules.clone(),
            name: op.clone(),
        }) as Box<dyn Generator>,
        ast::Operation::Star(op) => Box::new(Many {
            generator: translate_rule(op, rules),
        }) as Box<dyn Generator>,
        ast::Operation::StringLiteral(s) => {
            Box::new(StringLiteral { s: s.clone() }) as Box<dyn Generator>
        }
        ast::Operation::Token(t) => Box::new(Rule {
            rules: rules.clone(),
            name: t.clone(),
        }) as Box<dyn Generator>,
    }
}

/*
fn parse_charset(charset: &str) -> Box<dyn Generator> {
    let length = charset.
    let chars = charset.chars();

    let mut choices = vec![];
    let mut i = 0;

    for _ in i < chars.size() {}

    match c {
        '\\' => ,
        '[' => ,
        ']' => ,
        _ => 
    }

    unimplemented!()
}*/

#[derive(Debug, Fail)]
pub enum AntlrError {
    #[fail(display = "{}", _0)]
    IoError(std::io::Error),
    #[fail(display = "failed to parse")]
    ParseError
}

impl From<std::io::Error> for AntlrError {
    fn from(other: std::io::Error) -> Self {
        AntlrError::IoError(other)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // "/Users/joe/src/grammars-v4/bnf/bnf.g4"

        //let r = rules.read().unwrap();
        //let root = r.get("rulelist").unwrap();
        //let generated = root.generate();
        //let s = String::from_utf8_lossy(&generated);
        //println!("{}", s);
    }
}
