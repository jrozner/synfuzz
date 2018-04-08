use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use super::Generator;

const MANY_MAX: usize = 20;

pub struct And {
    left: Box<Generator>,
    right: Box<Generator>,
}

impl Generator for And {
    fn generate(&self) -> Vec<u8> {
        let mut generated = self.left.generate();
        generated.append(&mut self.right.generate());
        generated
    }
}

pub fn and(left: impl Generator + 'static, right: impl Generator + 'static) -> impl Generator {
    And {
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub struct Or {
    left: Box<Generator>,
    right: Box<Generator>,
}

impl Generator for Or {
    fn generate(&self) -> Vec<u8> {
        let num: u32 = thread_rng().gen();
        if num % 2 == 0 {
            self.left.generate()
        } else {
            self.right.generate()
        }
    }
}

pub fn or(left: impl Generator + 'static, right: impl Generator + 'static) -> impl Generator {
    Or {
        left: Box::new(left),
        right: Box::new(right),
    }
}

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

pub struct Remote {
    generators: Arc<Mutex<HashMap<String, Box<Generator>>>>,
    name: String,
}

impl Generator for Remote {
    fn generate(&self) -> Vec<u8> {
        let generators = self.generators.lock().unwrap();
        match generators.get(&self.name) {
            Some(generator) => generator.generate(),
            None => panic!("rule '{}' does not exist", self.name),
        }
    }
}

pub fn remote<S>(name: S, generators: Arc<Mutex<Rules>>) -> impl Generator
where
    S: Into<String>,
{
    Remote {
        name: name.into(),
        generators: generators,
    }
}

pub fn register_rule<S>(rules: &Arc<Mutex<Rules>>, name: S, rule: impl Generator + 'static)
where
    S: Into<String>,
{
    let mut rules = rules.lock().unwrap();
    rules.insert(name.into(), Box::new(rule));
}

pub type Rules = HashMap<String, Box<Generator>>;
