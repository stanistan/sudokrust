use std::collections::HashSet;

use config::{is_valid};

pub trait Validatable {
    fn is_valid(&self) -> bool;
}

impl Validatable for i8 {
    fn is_valid(&self) -> bool {
        is_valid(*self)
    }
}

pub fn are_many_valid<T: Validatable>(validatables: &Vec<T>) -> bool {
    for validatable in validatables {
        if !validatable.is_valid() {
            return false;
        }
    }
    true
}

pub trait AsSet {
    fn as_set(&self) -> HashSet<i8>;
}

impl AsSet for i8 {
    fn as_set(&self) -> HashSet<i8> {
        let mut set = HashSet::new();
        set.insert(*self);
        set
    }
}

impl AsSet for Vec<i8> {
    fn as_set(&self) -> HashSet<i8> {
        self.iter().cloned().collect()
    }
}

impl AsSet for HashSet<i8> {
    fn as_set(&self) -> HashSet<i8> {
        self.clone()
    }
}

pub fn set_intersection<T: AsSet>(a: &T, b: &T) -> HashSet<i8> {
    let a_set: HashSet<_> = a.as_set();
    let b_set: HashSet<_> = b.as_set();

    // there needs to be a better way, this is painful
    let mut set = HashSet::new();
    for value in a_set.intersection(&b_set) {
        set.insert(*value);
    }
    set
}

pub fn are_sets_equal<T: AsSet>(a: &T, b: &T) -> bool {
    let len = a.as_set().len();
    let intersection = set_intersection(a, b);
    intersection.len() == len
}
