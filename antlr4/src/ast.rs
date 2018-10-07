#[derive(Debug, Clone)]
pub struct Grammar {
    name: String,
    rules: Vec<Rule>,
}

impl Grammar {
    pub fn new(name: String, rules: Vec<Rule>) -> Grammar {
        Grammar { name, rules }
    }

    pub fn rules(&self) -> &Vec<Rule> {
        &self.rules
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RuleType {
    Lexer,
    Parser,
    Fragment,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Optional(Box<Operation>),
    Star(Box<Operation>),
    Plus(Box<Operation>),
    Group(Vec<Operation>),
    Alternate(Vec<Vec<Operation>>),
    Token(String),
    Rule(String),
    StringLiteral(String),
    Range((String, String)),
    Any,
    CharacterClass(Vec<Operation>),
    Char(char),
    CharRange((char, char)),
    Not(Box<Operation>),
}

#[derive(Debug, Clone)]
pub struct Rule {
    name: String,
    rule_type: RuleType,
    body: Vec<Operation>,
}

impl Rule {
    pub fn new(name: String, rule_type: RuleType, body: Vec<Operation>) -> Rule {
        Rule {
            name,
            rule_type,
            body,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rule_type(&self) -> RuleType {
        self.rule_type
    }

    pub fn body(&self) -> &Vec<Operation> {
        &self.body
    }
}

pub fn unroll_quantifier(quantifiers: Vec<String>, token: Operation) -> Operation {
    let mut ret = token;
    for quantifier in quantifiers {
        match quantifier.as_str() {
            "?" => ret = Operation::Optional(Box::new(ret)),
            "*" => ret = Operation::Star(Box::new(ret)),
            "+" => ret = Operation::Plus(Box::new(ret)),
            _ => panic!("noooooooo"),
        }
    }

    ret
}
