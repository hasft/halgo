use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Clone)]
pub struct UniqueBy<I, V, F> {
    iter: I,
    used: HashMap<V, ()>,
    f: F
}

#[derive(Clone)]
pub struct Unique<I> where I: Iterator {
    iter: UniqueBy<I, I::Item, ()>
}

impl<I,V,F> fmt::Debug for UniqueBy<I, V, F> where I: Iterator + fmt::Debug, V: Eq + Hash + fmt::Debug {
    debug_fmt_fields!(UniqueBy, iter, used);
}

impl<I,V,F> Iterator for UniqueBy<I,V,F> where I: Iterator, V: Eq + Hash, F: FnMut(&I::Item) -> V {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        while let Some(v) = self.iter.next() {
            let key = (self.f)(&v);
            if self.used.insert(key, ()).is_none() {
                return Some(v);
            }
        }
        None
    }
}

impl<I> Iterator for Unique<I> where I: Iterator, I::Item: Eq + Hash + Clone {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        while let Some(v) = self.iter.iter.next() {
            if let Entry::Vacant(entry) = self.iter.used.entry(v) {
                let elt = entry.key().clone();
                entry.insert(());
                return Some(elt);
            }
        }
        None
    }
}

pub fn unique_by<I, V, F>(iter: I, f: F) -> UniqueBy<I, V, F> where I: Iterator, V: Eq + Hash, F: FnMut(&I::Item) -> V  {
    UniqueBy {
        iter: iter,
        used: HashMap::new(),
        f: f
    }
}

pub fn unique<I>(iter: I) -> Unique<I> where I: Iterator, I::Item: Eq + Hash  {
    Unique {
        iter: UniqueBy {
            iter: iter,
            used: HashMap::new(),
            f: ()
        }
    }
}
