extern crate synfuzz;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use synfuzz::Generator;
use synfuzz::and;
use synfuzz::ch;
use synfuzz::choice;
use synfuzz::many1;
use synfuzz::register_rule;
use synfuzz::remote;
use synfuzz::seq;

fn main() {
    let rules = Arc::new(RwLock::new(HashMap::new()));

    let number = and(
        choice(vec![
            Box::new(ch('1')),
            Box::new(ch('2')),
            Box::new(ch('3')),
            Box::new(ch('4')),
            Box::new(ch('5')),
            Box::new(ch('6')),
            Box::new(ch('7')),
            Box::new(ch('8')),
            Box::new(ch('9')),
        ]),
        many1(choice(vec![
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
        ])),
    );

    register_rule(&rules, "number", number);

    let operators = choice(vec![
        Box::new(ch('*')),
        Box::new(ch('/')),
        Box::new(ch('+')),
        Box::new(ch('-')),
    ]);

    register_rule(&rules, "operators", operators);

    let expr = seq(vec![
        Box::new(remote("number", rules.clone())),
        Box::new(remote("operators", rules.clone())),
        Box::new(choice(vec![
            Box::new(remote("expression", rules.clone())),
            Box::new(remote("number", rules.clone())),
        ])),
    ]);

    register_rule(&rules, "expression", expr);

    let root = remote("expression", rules.clone());

    let out = root.generate();
    println!("{}", String::from_utf8_lossy(&out));
}
