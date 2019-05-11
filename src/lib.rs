use std::hash::Hash;
use std::fmt;

#[macro_use]
mod impl_macros;

pub use unique_impl::{UniqueBy, Unique};
mod unique_impl;

pub trait Halgo: Iterator {
    /// Return an iterator adaptor that filters out elements that have
    /// already been produced once during the iteration.
    ///
    /// Duplicates are detected by comparing the key they map to
    /// with the keying function `f` by hash and equality.
    /// The keys are stored in a hash set in the iterator.
    ///
    /// ```
    /// use halgo::Halgo;
    ///
    /// let data = vec!["a", "bb", "aa", "c", "ccc"];
    /// halgo::assert_equal(data.into_iter().unique_by(|s| s.len()),
    ///                         vec!["a", "bb", "ccc"]);
    /// ```
    fn unique_by<V, F>(self, f: F) -> UniqueBy<Self, V, F> where Self: Sized, V: Eq + Hash, F: FnMut(&Self::Item) -> V {
        unique_impl::unique_by(self, f)
    }

    /// Return an iterator adaptor that filters out elements that have
    /// already been produced once during the iteration. Duplicates
    /// are detected using hash and equality.
    ///
    /// Clones of visited elements are stored in a hash set in the
    /// iterator.
    ///
    /// ```
    /// use halgo::Halgo;
    ///
    /// let data = vec![10, 20, 30, 20, 40, 10, 50];
    /// halgo::assert_equal(data.into_iter().unique(),
    ///                         vec![10, 20, 30, 40, 50]);
    /// ```
    fn unique(self) -> Unique<Self> where Self: Sized, Self::Item: Clone + Eq + Hash  {
        unique_impl::unique(self)
    }
}

/// Assert that two iterables produce equal sequences, with the same
/// semantics as *equal(a, b)*.
///
/// **Panics** on assertion failure with a message that shows the
/// two iteration elements.
///
/// ```ignore
/// assert_equal("exceed".split('c'), "excess".split('c'));
/// // ^PANIC: panicked at 'Failed assertion Some("eed") == Some("ess") for iteration 1',
/// ```
pub fn assert_equal<I, J>(a: I, b: J)
    where I: IntoIterator,
          J: IntoIterator,
          I::Item: fmt::Debug + PartialEq<J::Item>,
          J::Item: fmt::Debug,
{
    let mut ia = a.into_iter();
    let mut ib = b.into_iter();
    let mut i = 0;
    loop {
        match (ia.next(), ib.next()) {
            (None, None) => return,
            (a, b) => {
                let equal = match (&a, &b) {
                    (&Some(ref a), &Some(ref b)) => a == b,
                    _ => false,
                };
                assert!(equal, "Failed assertion {a:?} == {b:?} for iteration {i}",
                        i=i, a=a, b=b);
                i += 1;
            }
        }
    }
}

impl<T: ?Sized> Halgo for T where T: Iterator {}
