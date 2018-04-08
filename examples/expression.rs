extern crate synfuzz;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use synfuzz::Generator;
use synfuzz::ch;
use synfuzz::choice;
use synfuzz::many1;
use synfuzz::register_rule;
use synfuzz::rule;
use synfuzz::seq;

fn main() {
    let rules = Arc::new(RwLock::new(HashMap::new()));

    let number = seq(vec![
        Box::new(choice(vec![
            Box::new(ch('1')),
            Box::new(ch('2')),
            Box::new(ch('3')),
            Box::new(ch('4')),
            Box::new(ch('5')),
            Box::new(ch('6')),
            Box::new(ch('7')),
            Box::new(ch('8')),
            Box::new(ch('9')),
        ])),
        Box::new(many1(choice(vec![
            Box::new(ch('0')),
            Box::new(ch('1')),
            Box::new(ch('2')),
            Box::new(ch('3')),
            Box::new(ch('4')),
            Box::new(ch('5')),
            Box::new(ch('6')),
            Box::new(ch('7')),
            Box::new(ch('8')),
            Box::new(ch('9')),
        ]))),
    ]);

    register_rule(&rules, "number", number);

    let operators = choice(vec![
        Box::new(ch('*')),
        Box::new(ch('/')),
        Box::new(ch('+')),
        Box::new(ch('-')),
    ]);

    register_rule(&rules, "operators", operators);

    let expr = seq(vec![
        Box::new(rule("number", rules.clone())),
        Box::new(rule("operators", rules.clone())),
        Box::new(choice(vec![
            Box::new(rule("expression", rules.clone())),
            Box::new(rule("number", rules.clone())),
        ])),
    ]);

    register_rule(&rules, "expression", expr);

    let root = rule("expression", rules.clone());

    let out = root.generate();
    println!("{}", String::from_utf8_lossy(&out));
}
