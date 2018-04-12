#[macro_use]
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
use synfuzz::sep_by;
use synfuzz::seq;

fn main() {
    let rules = Arc::new(RwLock::new(HashMap::new()));

    let delimiters = ch(' ');

    let number = seq(vec![
        Box::new(choice!(
            ch('1'),
            ch('2'),
            ch('3'),
            ch('4'),
            ch('5'),
            ch('6'),
            ch('7'),
            ch('8'),
            ch('9')
        )),
        Box::new(many1(choice!(
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
        ))),
    ]);

    register_rule(&rules, "number", number);

    let operators = choice!(ch('*'), ch('/'), ch('+'), ch('-'));

    register_rule(&rules, "operators", operators);

    let expr = sep_by!(
        delimiters,
        rule("number", rules.clone()),
        rule("operators", rules.clone()),
        choice!(
            rule("expression", rules.clone()),
            rule("number", rules.clone())
        )
    );

    register_rule(&rules, "expression", expr);

    let root = rule("expression", rules.clone());

    let out = root.generate();
    println!("{}", String::from_utf8_lossy(&out));
}
