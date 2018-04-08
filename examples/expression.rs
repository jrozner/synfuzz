extern crate synfuzz;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use synfuzz::Generator;
use synfuzz::and;
use synfuzz::ch;
use synfuzz::choice;
use synfuzz::many1;
use synfuzz::register_rule;
use synfuzz::remote;

fn main() {
    let rules = Arc::new(Mutex::new(HashMap::new()));

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

    let expr = and(
        remote("number", rules.clone()),
        and(
            remote("operators", rules.clone()),
            remote("number", rules.clone()),
        ),
    );

    let out = expr.generate();
    println!("{}", String::from_utf8_lossy(&out));
}
