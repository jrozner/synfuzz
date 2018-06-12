use rand::thread_rng;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use super::Generator;

/// The maximum number of repetitions for the Many and Many1 Generators
const MANY_MAX: usize = 20;
/// The maximum number of repetitions for the SepBy and SepBy1 Generators
const SEP_BY_MAX: usize = 20;
/// The maximum number of repetitions for the negation of the RepeatN
/// Generator
const REPEAT_MAX: usize = 20;

/// Choice is a Generator that will pick one of the Generators specified in
/// its choices. Each call to the generate method may return a value from a
/// different Generator
pub struct Choice {
    choices: Vec<Box<Generator>>,
}

impl Generator for Choice {
    fn generate(&self) -> Vec<u8> {
        match thread_rng().choose(&self.choices) {
            Some(generator) => generator.generate(),
            None => panic!("no choices specified"),
        }
    }

    fn negate(&self) -> Vec<u8> {
        // must match any of of choices, except one, or anything not a choice
        match self.choices.len() {
            0 => vec![], // TODO: generate something non-empty
            1 => self.choices[0].negate(),
            _ => {
                let generate_nothing = thread_rng().gen();
                if generate_nothing {
                    vec![]
                } else {
                    vec![]
                }
            }
        }
    }
}

/// choice is a helper to create a Choice Generator. There is also a macro
/// that generates the Vec and Boxes the individual generators being passed
/// as choices for brevity and simplicity
pub fn choice(choices: Vec<Box<Generator>>) -> impl Generator {
    Choice { choices: choices }
}

/// Many is a Generator that will generate 0 or more values of its generator
pub struct Many {
    generator: Box<Generator>,
}

impl Generator for Many {
    fn generate(&self) -> Vec<u8> {
        let num = thread_rng().gen_range(0, MANY_MAX);
        (0..num).flat_map(|_| self.generator.generate()).collect()
    }

    fn negate(&self) -> Vec<u8> {
        // generate nothing or the negation of generator 0..MANY_MAX times
        let num = thread_rng().gen_range(0, MANY_MAX);
        (0..num).flat_map(|_| self.generator.negate()).collect()
    }
}

/// many is a helper to create a Many Generator
pub fn many(generator: impl Generator + 'static) -> impl Generator {
    Many {
        generator: Box::new(generator),
    }
}

/// Many1 is a Generator that will generate 1 or more values of its generator
pub struct Many1 {
    generator: Box<Generator>,
}

impl Generator for Many1 {
    fn generate(&self) -> Vec<u8> {
        let num = thread_rng().gen_range(1, MANY_MAX);
        (0..num).flat_map(|_| self.generator.generate()).collect()
    }

    fn negate(&self) -> Vec<u8> {
        // generate nothing or the negation of generator 0..MANY_MAX times
        let num: usize = thread_rng().gen_range(0, MANY_MAX);
        (0..num).flat_map(|_| self.generator.negate()).collect()
    }
}

/// many1 is a helper to create a Many1 Generator
pub fn many1(generator: impl Generator + 'static) -> impl Generator {
    Many1 {
        generator: Box::new(generator),
    }
}

/// Optional is a Generator that will optionally choose to generate exactly 1
/// of its generator or and empty value
pub struct Optional {
    generator: Box<Generator>,
}

impl Generator for Optional {
    fn generate(&self) -> Vec<u8> {
        if thread_rng().gen() {
            self.generator.generate()
        } else {
            vec![]
        }
    }

    fn negate(&self) -> Vec<u8> {
        if thread_rng().gen() {
            self.generator.negate()
        } else {
            vec![]
        }
    }
}

/// optional is a helper to create an Optional Generator
pub fn optional(generator: impl Generator + 'static) -> impl Generator {
    Optional {
        generator: Box::new(generator),
    }
}

/// Rule is a Generator for invoking a named rule Generator. This is useful
/// for implementing recursion and avoiding duplication of portions of a
/// grammar.
///
/// Only names that have already been registered should be used. If a
/// corresponding rule does not exist when generate is called it will panic.
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

    fn negate(&self) -> Vec<u8> {
        // invoke the negation of the rule
        let rules = self.rules.read().unwrap();
        match rules.get(&self.name) {
            Some(generator) => generator.negate(),
            None => panic!("rule '{}' does not exist", self.name),
        }
    }
}

/// rule is a helper to create a Rule Generator
pub fn rule<S>(name: S, rules: Arc<RwLock<Rules>>) -> impl Generator
where
    S: Into<String>,
{
    Rule {
        name: name.into(),
        rules: rules,
    }
}

/// register_rule associates a tree of Generators to a name that can be
/// later used with the Rule Generator. A rule must always be registered
/// before a Rule Generator is executed otherwise it will lead to a panic
/// due to an unknown rule. Rule names are case sensitive and must be unique.
/// Attempting to register two rules with the same name will result in the
/// last one being registered being used. This can lead to unexpected
/// behavior.
pub fn register_rule<S>(rules: &Arc<RwLock<Rules>>, name: S, rule: impl Generator + 'static)
where
    S: Into<String>,
{
    let mut rules = rules.write().unwrap();
    rules.insert(name.into(), Box::new(rule));
}

/// Rules stores a map of rule name to the tree of Generators. For
/// multithreaded applications this should be wrapped in an Arc<Mutex<T>>
/// to provide safe access. Realistically, as long as all rules are added
/// before generation begins, locking should be unecessary
pub type Rules = HashMap<String, Box<Generator>>;

/// Sequence is a Generator that generates all of its generators in the order
/// in which they are specified. This is useful for sequences of specific
/// bytes or chars but when multiple tokens are desired JoinWith is likely
/// more helpful
pub struct Sequence {
    generators: Vec<Box<Generator>>,
}

impl Generator for Sequence {
    fn generate(&self) -> Vec<u8> {
        self.generators.iter().flat_map(|g| g.generate()).collect()
    }

    fn negate(&self) -> Vec<u8> {
        unimplemented!()
    }
}

/// seq is a helper to create a Sequence Generator. This is also a macro that
/// handles creating the Vec and boxing the individual Generators being
/// specified for brevity and simplicity
pub fn seq(generators: Vec<Box<Generator>>) -> impl Generator {
    Sequence {
        generators: generators,
    }
}

/// RepeastN is a Generator that will product the specified Generator between
/// 0 and n times
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

    fn negate(&self) -> Vec<u8> {
        // repeats any number except n times
        let mut repetitions = thread_rng().gen_range(0, REPEAT_MAX);
        if repetitions == self.n {
            repetitions += 1;
        }

        (0..repetitions)
            .flat_map(|_| self.generator.negate())
            .collect()
    }
}

/// repeat_n is a helper to create a RepeatN Generator
pub fn repeat_n(generator: impl Generator + 'static, n: usize) -> impl Generator {
    RepeatN {
        n: n,
        generator: Box::new(generator),
    }
}

/// Range is a Generator that will produce the specified Generator between n
/// and m times
pub struct Range {
    n: usize,
    m: usize,
    generator: Box<Generator>,
}

impl Generator for Range {
    fn generate(&self) -> Vec<u8> {
        let times = thread_rng().gen_range(self.n, self.m);
        (self.n..times)
            .flat_map(|_| self.generator.generate())
            .collect()
    }

    fn negate(&self) -> Vec<u8> {
        unimplemented!()
    }
}

/// range is a helper to create a Range Generator
pub fn range(generator: impl Generator + 'static, n: usize, m: usize) -> impl Generator {
    Range {
        n: n,
        m: m,
        generator: Box::new(generator),
    }
}

/// JoinWith is a Generator that joins a list of Generators with the
/// specified Generator as the delimiter. In the case of the only one
/// Generator being specified in the list no delimiter will be added. This is
/// particularly useful when attempting to match tokens or desiring that
/// be a separator (eg. some whitespace) between them. In that case this
/// should be used instead of Sequence so specify the sequence of tokens for
/// a rule.
pub struct JoinWith {
    generators: Vec<Box<Generator>>,
    delimiter: Box<Generator>,
}

impl Generator for JoinWith {
    fn generate(&self) -> Vec<u8> {
        let mut first = true;
        self.generators
            .iter()
            .flat_map(|g| {
                let mut value = g.generate();
                if !first {
                    let mut d = self.delimiter.generate();
                    d.extend(value);
                    value = d;
                } else {
                    first = false;
                }
                value
            })
            .collect()
    }

    fn negate(&self) -> Vec<u8> {
        unimplemented!()
    }
}

/// join_with is a helper to create a JoinWith Generator. This is also a
/// macro that handles creating the Vec and boxing the individual Generators
/// being specified for brevity and simplicity

pub fn join_with(
    generators: Vec<Box<Generator>>,
    delimiters: impl Generator + 'static,
) -> impl Generator {
    JoinWith {
        generators: generators,
        delimiter: Box::new(delimiters),
    }
}

/// SepBy is a Generator that will repeat the generator 0 or more times
/// separated by the specified separator. A single match will result in no
/// separator being present, only there there is more than one
pub struct SepBy {
    generator: Box<Generator>,
    separator: Box<Generator>,
}

impl Generator for SepBy {
    fn generate(&self) -> Vec<u8> {
        let limit = thread_rng().gen_range(0, SEP_BY_MAX);

        let mut first = true;
        (0..limit)
            .flat_map(|_| {
                let mut value = self.generator.generate();
                if first {
                    first = false;
                } else {
                    let mut separator = self.separator.generate();
                    separator.extend(self.generator.generate());
                    value = separator;
                }

                value
            })
            .collect()
    }

    fn negate(&self) -> Vec<u8> {
        unimplemented!()
    }
}

/// sep_by is a helper to create a SepBy Generator
pub fn sep_by(
    generator: impl Generator + 'static,
    separator: impl Generator + 'static,
) -> impl Generator {
    SepBy {
        generator: Box::new(generator),
        separator: Box::new(separator),
    }
}

/// SepBy is a Generator that will repeat the generator 1 or more times
/// separated by the specified separator. A single match will result in no
/// separator being present, only there there is more than one
pub struct SepBy1 {
    generator: Box<Generator>,
    separator: Box<Generator>,
}

impl Generator for SepBy1 {
    fn generate(&self) -> Vec<u8> {
        let limit = thread_rng().gen_range(1, SEP_BY_MAX);

        let mut first = true;
        (0..limit)
            .flat_map(|_| {
                let mut value = self.generator.generate();
                if first {
                    first = false;
                } else {
                    let mut separator = self.separator.generate();
                    separator.extend(self.generator.generate());
                    value = separator;
                }

                value
            })
            .collect()
    }

    fn negate(&self) -> Vec<u8> {
        unimplemented!()
    }
}

/// sep_by1 is a helper to create a SepBy1 Generator
pub fn sep_by1(
    generator: impl Generator + 'static,
    separator: impl Generator + 'static,
) -> impl Generator {
    SepBy1 {
        generator: Box::new(generator),
        separator: Box::new(separator),
    }
}

/// Not is a generator that will return the negation of it's generator. The
/// implemenation is dependent on the negate implementation for all other
/// generators
pub struct Not {
    generator: Box<Generator>,
}

impl Generator for Not {
    fn generate(&self) -> Vec<u8> {
        self.generator.negate()
    }

    fn negate(&self) -> Vec<u8> {
        self.generator.generate()
    }
}

/// not is a helper to generate a Not Generator
pub fn not(generator: impl Generator + 'static) -> impl Generator {
    Not {
        generator: Box::new(generator),
    }
}

#[macro_export]
macro_rules! choice {
    ( $( $x:expr ),* ) => {
        choice(vec![
            $(Box::new($x)),*
        ]);
    };
}

#[macro_export]
macro_rules! seq {
    ( $( $x:expr ),* ) => {
        seq(vec![
            $(Box::new($x)),*
        ]);
    };
}

#[macro_export]
macro_rules! join_with {
    ( $delimiter:expr, $( $x:expr ),* ) => {
        join_with(vec![
            $(Box::new($x)),*
        ], $delimiter);
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use super::*;
    use value::byte;

    #[test]
    fn generate_choice() {
        let generator = choice!(byte(0x41), byte(0x42));
        let generated = generator.generate();
        assert!(generated == vec![0x41] || generated == vec![0x42]);
    }

    #[test]
    fn negate_choice() {
        let generator = choice!(byte(0x41), byte(0x42));
        let generated = generator.negate();
        assert!(generated != vec![0x41] && generated != vec![0x42]);
    }

    #[test]
    fn generate_many() {
        let generator = many(byte(0x41));
        let generated = generator.generate();
        let generated_string = String::from_utf8_lossy(&generated);
        // assured that the len will always be >= 0 due to usize constraints
        assert!(generated.len() < MANY_MAX);
        let r = Regex::new(r"\AA*\z").unwrap();
        assert!(r.is_match(&generated_string));
    }

    #[test]
    fn negate_many() {
        let generator = many(byte(0x41));
        let generated = generator.negate();
        let generated_string = String::from_utf8_lossy(&generated);
        // assured that the len will always be >= 0 due to usize constraints
        assert!(generated.len() < MANY_MAX);
        let r = Regex::new(r"\A[^A]*\z").unwrap();
        assert!(r.is_match(&generated_string));
    }

    #[test]
    fn generate_many1() {
        let generator = many1(byte(0x41));
        let generated = generator.generate();
        let generated_string = String::from_utf8_lossy(&generated);
        assert!(generated.len() > 0 && generated.len() < MANY_MAX);
        let r = Regex::new(r"\AA+\z").unwrap();
        assert!(r.is_match(&generated_string));
    }

    #[test]
    fn negate_many1() {
        let generator = many1(byte(0x41));
        let generated = generator.negate();
        let generated_string = String::from_utf8_lossy(&generated);
        // assured that the len will always be >= 0 due to usize constraints
        assert!(generated.len() < MANY_MAX);
        let r = Regex::new(r"\A[^A]*\z").unwrap();
        assert!(r.is_match(&generated_string));
    }

    #[test]
    fn generate_optional() {
        let generator = optional(byte(0x41));
        let generated = generator.generate();
        assert!(generated.len() < 2);
        if generated.len() > 0 {
            assert!(generated == vec![0x41]);
        }
    }

    #[test]
    fn negate_optional() {
        let generator = optional(byte(0x41));
        let generated = generator.negate();
        assert!(generated.len() < 2);
        if generated.len() > 0 {
            assert_ne!(generated, vec![0x41]);
        }
    }

    #[test]
    fn generate_rule() {
        let rules = Arc::new(RwLock::new(HashMap::new()));
        let the_rule = byte(0x41);
        register_rule(&rules, "rule", the_rule);
        let generator = rule("rule", rules);
        let generated = generator.generate();
        assert!(generated == vec![0x41]);
    }

    #[test]
    fn negate_rule() {
        let rules = Arc::new(RwLock::new(HashMap::new()));
        let the_rule = byte(0x41);
        register_rule(&rules, "rule", the_rule);
        let generator = rule("rule", rules);
        let generated = generator.negate();
        assert_ne!(generated, vec![0x41]);
    }

    #[test]
    fn generate_repeatn() {
        let generator = repeat_n(byte(0x41), 5);
        let generated = generator.generate();
        assert!(generated == vec![0x41, 0x41, 0x41, 0x41, 0x41]);
    }

    #[test]
    fn negate_repeatn() {
        let generator = repeat_n(byte(0x41), 5);
        let generated = generator.negate();
        assert_ne!(generated, vec![0x41, 0x41, 0x41, 0x41, 0x41]);
    }

    #[test]
    fn generate_not() {
        let generator = not(byte(0x41));
        let generated = generator.generate();
        assert_ne!(generated, vec![0x41]);
    }

    #[test]
    fn negate_not() {
        let generator = not(byte(0x41));
        let generated = generator.negate();
        assert!(generated == vec![0x41]);
    }
}
