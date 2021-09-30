use std::collections::HashMap;
use std::hash;

pub struct Cacher<T, U>
where
    T: Clone + Eq + hash::Hash,
{
    calculation: Box<dyn Fn(&T) -> U>,
    memory: HashMap<T, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Clone + Eq + hash::Hash,
{
    pub fn new(calculation: Box<dyn Fn(&T) -> U>) -> Cacher<T, U> {
        Cacher {
            calculation,
            memory: HashMap::new(),
        }
    }

    pub fn value(&mut self, input: T) -> &U {
        let i = input.clone();
        if !self.memory.contains_key(&input) {
            let res = (self.calculation)(&input);
            self.memory.insert(i, res);
        }
        self.memory.get(&input).unwrap()
    }
}

#[test]
fn caching_works_with_different_values() {
    let mut c: Cacher<u32, u32> = Cacher::new(Box::new(|a| *a));

    assert_eq!(*c.value(1), 1);
    assert_eq!(*c.value(2), 2);
    assert_eq!(*c.value(3), 3);
}

#[test]
fn caching_works_with_different_types() {
    let mut c: Cacher<u32, String> = Cacher::new(Box::new(|a| a.to_string()));

    assert_eq!(*c.value(1), String::from("1"));
    assert_eq!(*c.value(2), String::from("2"));
    assert_eq!(*c.value(3), String::from("3"));
}
