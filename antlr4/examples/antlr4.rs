extern crate synfuzz_antlr4;

use synfuzz_antlr4::generate_rules;

use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("{} <path> <rule>", args[0]);
        exit(1);
    }

    let rules = match generate_rules(&args[1]) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    let r = rules.read().unwrap();
    let root = r.get(&args[2]).unwrap();
    let generated = root.generate();
    let s = String::from_utf8_lossy(&generated);
    println!("{}", s);
}
