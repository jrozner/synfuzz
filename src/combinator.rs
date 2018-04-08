use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use super::Generator;

const MANY_MAX: usize = 20;

pub struct Choice {
    choices: Vec<Box<Generator>>,
}

impl Generator for Choice {
    fn generate(&self) -> Vec<u8> {
        let num: usize = thread_rng().gen();
        let i = num % self.choices.len();
        self.choices[i].generate()
    }
}

// TODO: create macro choice! that takes n Generators and converts
// them into a Vec<Box<Generator>>
pub fn choice(choices: Vec<Box<Generator>>) -> impl Generator {
    Choice { choices: choices }
}

pub struct Many {
    generator: Box<Generator>,
}

impl Generator for Many {
    fn generate(&self) -> Vec<u8> {
        let num: usize = thread_rng().gen();
        (0..(num % MANY_MAX))
            .flat_map(|_| self.generator.generate())
            .collect()
    }
}

pub fn many(generator: impl Generator + 'static) -> impl Generator {
    Many {
        generator: Box::new(generator),
    }
}

pub struct Many1 {
    generator: Box<Generator>,
}

impl Generator for Many1 {
    fn generate(&self) -> Vec<u8> {
        let num: usize = thread_rng().gen();
        (1..(num % MANY_MAX))
            .flat_map(|_| self.generator.generate())
            .collect()
    }
}

pub fn many1(generator: impl Generator + 'static) -> impl Generator {
    Many1 {
        generator: Box::new(generator),
    }
}

pub struct Optional {
    generator: Box<Generator>,
}

impl Generator for Optional {
    fn generate(&self) -> Vec<u8> {
        let num: usize = thread_rng().gen();
        if num % 2 == 0 {
            self.generator.generate()
        } else {
            vec![]
        }
    }
}

pub fn optional(generator: impl Generator + 'static) -> impl Generator {
    Optional {
        generator: Box::new(generator),
    }
}

pub struct Rule {
    rules: Arc<RwLock<HashMap<String, Box<Generator>>>>,
    name: String,
}

impl Generator for Rule {
    fn generate(&self) -> Vec<u8> {
        let rules = self.rules.read().unwrap();
        match rules.get(&self.name) {
            Some(generator) => generator.generate(),
            None => panic!("rule '{}' does not exist", self.name),
        }
    }
}

pub fn rule<S>(name: S, rules: Arc<RwLock<Rules>>) -> impl Generator
where
    S: Into<String>,
{
    Rule {
        name: name.into(),
        rules: rules,
    }
}

pub fn register_rule<S>(rules: &Arc<RwLock<Rules>>, name: S, rule: impl Generator + 'static)
where
    S: Into<String>,
{
    let mut rules = rules.write().unwrap();
    rules.insert(name.into(), Box::new(rule));
}

pub type Rules = HashMap<String, Box<Generator>>;

pub struct Sequence {
    generators: Vec<Box<Generator>>,
}

impl Generator for Sequence {
    fn generate(&self) -> Vec<u8> {
        self.generators.iter().flat_map(|g| g.generate()).collect()
    }
}

// TODO: create macro seq! that takes n Generators and converts
// them into a Vec<Box<Generator>>
pub fn seq(generators: Vec<Box<Generator>>) -> impl Generator {
    Sequence {
        generators: generators,
    }
}

pub struct RepeatN {
    n: usize,
    generator: Box<Generator>,
}

impl Generator for RepeatN {
    fn generate(&self) -> Vec<u8> {
        (0..self.n)
            .flat_map(|_| self.generator.generate())
            .collect()
    }
}

pub fn repeat_n(generator: impl Generator + 'static, n: usize) -> impl Generator {
    RepeatN {
        n: n,
        generator: Box::new(generator),
    }
}
