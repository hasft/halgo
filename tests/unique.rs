extern crate halgo;
use halgo::Halgo;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[test]
fn test_next_unique()  {
    let data = vec![10, 20, 30, 20, 40, 10, 50];
    let mut m = data.into_iter().unique();
    let mut l = HashMap::new();
    while m.next().is_some() {
        println!("asas");
    }
    while let Some(v) = m.next() {
        match l.entry(v) {
            Entry::Vacant(entry) => {
                let elt = entry.key().clone();
                entry.insert(());
                println!("{:?}", elt);
            },
            _ => {}
        }        
    }

    
    assert_eq!(2,3);
}

#[test]
fn test_unique_by()  {
    let data = vec!["a", "bb", "aa", "c", "ccc"];
    let unique_by = data.into_iter().unique_by(|v| v.len());
    halgo::assert_equal(vec!["a", "bb", "ccc"], unique_by);
}

#[test]
fn test_unique()  {
    let data = vec![10, 20, 30, 20, 40, 10, 50];
    let unique:Vec<u32> = data.into_iter().unique().collect();
    halgo::assert_equal(vec![10,20,30,40,50], unique);
}
