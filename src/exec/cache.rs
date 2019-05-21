use super::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Lookup<T: Clone + Debug>: Debug {
    fn lookup(&self, index: usize) -> Option<&T>;
}

#[derive(Debug)]
pub struct CodeLookup(pub Rc<ty::ClassFile>);

impl Lookup<attr::Code> for CodeLookup {
    fn lookup(&self, index: usize) -> Option<&attr::Code> {
        self.0.methods.get(index).and_then(ty::Method::get_code)
    }
}

#[derive(Debug)]
pub struct Cache<T> {
    map: RefCell<HashMap<usize, Rc<T>>>,
    lookup: Box<dyn Lookup<T>>,
}

impl<T: Clone + Debug> Cache<T> {
    pub fn new(lookup: Box<dyn Lookup<T>>) -> Self {
        Self {
            map: RefCell::new(HashMap::default()),
            lookup,
        }
    }

    pub fn get(&self, index: usize) -> Rc<T> {
        let mut map = self.map.borrow_mut();

        if let Some(code) = map.get(&index).map(Rc::clone) {
            return code;
        }

        if let Some(method) = self.lookup.lookup(index).map(Clone::clone).map(Rc::new) {
            map.insert(index, Rc::clone(&method));
            return method;
        }

        unreachable!("item must exist in cache")
    }
}
