#[macro_use]
extern crate failure;
extern crate lalrpop_util;
extern crate log;
extern crate synfuzz;

mod antlr4;
mod ast;
mod charset;

use ast::RuleType;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;

use synfuzz::{
    byte, register_rule, Any, CharLiteral, CharRange, Choice, Generator, JoinWith, Many, Many1,
    Not, Optional, Rule, Rules, Sequence, StringLiteral,
};

/// generate_rules takes the path to an ANTLR4 grammar file and returns a set of
/// rules that represent the parsed file
pub fn generate_rules(path: &str) -> Result<Arc<RwLock<Rules>>, AntlrError> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let parse_tree = antlr4::GrammarParser::new().parse(&buf)?;
    let rules = Arc::new(RwLock::new(HashMap::new()));

    for rule in parse_tree.rules().iter() {
        let parts = rule
            .body()
            .iter()
            .map(|part| translate_rule(part, rule.rule_type(), &rules))
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

fn translate_rule(
    operation: &ast::Operation,
    rule_type: ast::RuleType,
    rules: &Arc<RwLock<Rules>>,
) -> Box<dyn Generator> {
    match operation {
        ast::Operation::Alternate(op) => {
            let choices = op
                .iter()
                .map(|alternate| {
                    let parts = alternate
                        .iter()
                        .map(|part| translate_rule(part, rule_type, rules))
                        .collect::<Vec<Box<dyn Generator>>>();
                    match rule_type {
                        RuleType::Parser => Box::new(JoinWith {
                            generators: parts,
                            delimiter: Box::new(byte(0x20)),
                        }) as Box<dyn Generator>,
                        RuleType::Lexer | RuleType::Fragment => {
                            Box::new(Sequence { generators: parts }) as Box<dyn Generator>
                        }
                    }
                }).collect::<Vec<Box<dyn Generator>>>();
            return Box::new(Choice { choices: choices });
        }
        ast::Operation::Any => Box::new(Any {}),
        ast::Operation::CharacterClass(cc) => Box::new(Choice {
            choices: cc
                .iter()
                .map(|choice| translate_rule(choice, rule_type, rules))
                .collect(),
        }),
        ast::Operation::Group(op) => {
            let parts = op
                .iter()
                .map(|part| translate_rule(part, rule_type, rules))
                .collect::<Vec<Box<dyn Generator>>>();
            match rule_type {
                RuleType::Parser => Box::new(JoinWith {
                    generators: parts,
                    delimiter: Box::new(byte(0x20)),
                }) as Box<dyn Generator>,
                RuleType::Lexer | RuleType::Fragment => {
                    Box::new(Sequence { generators: parts }) as Box<dyn Generator>
                }
            }
        }
        ast::Operation::Optional(op) => Box::new(Optional {
            generator: translate_rule(op, rule_type, rules),
        }) as Box<dyn Generator>,
        ast::Operation::Plus(op) => Box::new(Many1 {
            generator: translate_rule(op, rule_type, rules),
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
            generator: translate_rule(op, rule_type, rules),
        }) as Box<dyn Generator>,
        ast::Operation::StringLiteral(s) => {
            Box::new(StringLiteral { s: s.clone() }) as Box<dyn Generator>
        }
        ast::Operation::Token(t) => Box::new(Rule {
            rules: rules.clone(),
            name: t.clone(),
        }) as Box<dyn Generator>,
        ast::Operation::CharRange((n, m)) => Box::new(CharRange { n: *n, m: *m }),
        ast::Operation::Char(c) => Box::new(CharLiteral { ch: *c }),
        ast::Operation::Not(op) => Box::new(Not {
            generator: translate_rule(op, rule_type, rules),
        }),
    }
}

#[derive(Debug, Fail)]
pub enum AntlrError {
    #[fail(display = "{}", _0)]
    IoError(std::io::Error),
    #[fail(display = "{}", _0)]
    ParseError(String),
}

impl From<std::io::Error> for AntlrError {
    fn from(other: std::io::Error) -> Self {
        AntlrError::IoError(other)
    }
}
impl<'a> From<lalrpop_util::ParseError<usize, antlr4::Token<'a>, &'a str>> for AntlrError {
    fn from(other: lalrpop_util::ParseError<usize, antlr4::Token<'a>, &'a str>) -> Self {
        AntlrError::ParseError(other.to_string())
    }
}

#[cfg(test)]
mod tests {}
