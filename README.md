# Halgo
Extended Iterator lib. This is lib is made to learn rust iterator and some list manipulation algorithm. Some function taken from itertools.

## UniqueBy

Duplicates are detected by comparing the key they map to with the keying function `f` by hash and equality. The keys are stored in a hash set in the iterator.
	
``` rust
// get from itertool

impl<I,V,F> Iterator for UniqueBy<I,V,F> where I: Iterator, V: Eq + Hash, F: FnMut(&I::Item) -> V {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
	
		// below fn same as:
		// match self.iter.next() { Some(v) => {...}, _ => None }
		//
		// means:
		// as long as self.iter.next() match Some(v)
		// set key with predicate facing Item referenced. e.g key for "abc" is 3 for |v| v.len()
		// if HashMap not have a current key then return Some(v)
		// note: HashMap never duplicate a key
		
        while let Some(v) = self.iter.next() {
            let key = (self.f)(&v);
            if self.used.insert(key, ()).is_none() {
                return Some(v);
            }
        }
        None		
    }
}
```

## Unique

Clones of visited elements are stored in a hash set in the iterator.

``` rust
impl<I> Iterator for Unique<I> where I: Iterator, I::Item: Eq + Hash + Clone {
    type Item = I::Item;
	
	// if Some(v) match enum of used.entry(v)

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
```
