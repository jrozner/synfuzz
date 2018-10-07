extern crate synfuzz;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use synfuzz::ch;
use synfuzz::choice;
use synfuzz::join_with;
use synfuzz::many1;
use synfuzz::register_rule;
use synfuzz::rule;
use synfuzz::seq;
use synfuzz::Generator;

fn main() {
    let rules = Arc::new(RwLock::new(HashMap::new()));

    let delimiters = ch(' ');

    let number = seq!(
        choice!(
            ch('1'),
            ch('2'),
            ch('3'),
            ch('4'),
            ch('5'),
            ch('6'),
            ch('7'),
            ch('8'),
            ch('9')
        ),
        many1(choice!(
            ch('0'),
            ch('1'),
            ch('2'),
            ch('3'),
            ch('4'),
            ch('5'),
            ch('6'),
            ch('7'),
            ch('8'),
            ch('9')
        ))
    );

    register_rule(&rules, "number", number);

    let operators = choice!(ch('*'), ch('/'), ch('+'), ch('-'));

    register_rule(&rules, "operators", operators);

    let expr = choice!(
        join_with!(
            delimiters,
            rule("number", rules.clone()),
            rule("operators", rules.clone()),
            choice!(
                rule("expression", rules.clone()),
                rule("number", rules.clone())
            )
        ),
        seq!(ch('('), rule("expression", rules.clone()), ch(')'))
    );

    register_rule(&rules, "expression", expr);

    let root = rule("expression", rules.clone());

    let out = root.generate();
    println!("{}", String::from_utf8_lossy(&out));
}
