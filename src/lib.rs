//! This crate aims to provide a way to alternate between the items of two iterators.
//!
//! For the simplest usage, bring [`AlternatingExt`] into scope and use the [`alternate_with`](AlternatingExt::alternate_with) method:
//! ```
//! use alternating_iter::AlternatingExt;
//!
//! let a = [1, 2, 3];
//! let b = [4, 5];
//!
//! let mut alternating = a.iter().alternate_with(b.iter());
//!
//! assert_eq!(alternating.next(), Some(&1));
//! assert_eq!(alternating.next(), Some(&4));
//! assert_eq!(alternating.next(), Some(&2));
//! assert_eq!(alternating.next(), Some(&5));
//! assert_eq!(alternating.next(), Some(&3));
//! assert_eq!(alternating.next(), None);
//! ```
//!
//! By default, the iterator acquired from `alternate_with` method will
//! return the items from both iterators until both are exhausted.
//!
//! However, if you want to stop after either is exhuasted,
//! use [`alternate_with_no_remainder`](AlternatingExt::alternate_with_no_remainder).
//! The iterator acquired from this method will stop once it
//! has to return more than 1 item from a single iterator consecutively.

mod alternating;
mod alternating_no_remainder;

pub use alternating::Alternating;
pub use alternating_no_remainder::AlternatingNoRemainder;

/// Extension trait that provides methods for creating alternating iterators.
///
/// This trait can be `use`d to add the [`alternate_with`](AlternatingExt::alternate_with)
/// and [`alternate_with_no_remainder`](AlternatingExt::alternate_with_no_remainder) methods
/// to any iterator, allowing iteration over two iterators in an alternating fashion.
pub trait AlternatingExt: Iterator {
    /// Takes two iterators and creates a new iterator over both in in an alternating fashion.
    ///
    /// The left iterator will be the first in the sequence.
    /// Once one of the iterators is exhausted,
    /// the remaining items from the other iterator will be returned.
    ///
    /// Note that both iterators must have the same [`Item`](Iterator::Item) type.
    ///
    /// # Examples
    ///
    /// ```
    /// use alternating_iter::AlternatingExt;
    ///
    /// let a = [1, 2];
    /// let b = [3, 4, 5];
    ///
    /// let mut iter = a.iter().alternate_with(b.iter());
    ///
    /// assert_eq!(iter.next(), Some(&1)); // a first
    /// assert_eq!(iter.next(), Some(&3)); // b
    /// assert_eq!(iter.next(), Some(&2)); // a
    /// assert_eq!(iter.next(), Some(&4)); // b
    /// assert_eq!(iter.next(), Some(&5)); // also b
    /// assert_eq!(iter.next(), None);
    /// ```
    fn alternate_with<I>(self, other: I) -> Alternating<Self, I::IntoIter>
    where
        Self: Sized,
        I: IntoIterator<Item = Self::Item>,
    {
        Alternating::new(self, other)
    }

    /// Takes two iterators and creates a new iterator over both in an alternating fashion,
    /// with no remainder from the exhausted iterator.
    ///
    /// Different from [`alternate_with`][AlternatingExt::alternate_with] in that
    /// when one of the iterators is exhausted, only a single item from the other iterator
    /// is returned.
    ///
    /// Note that both iterators must have the same [`Item`](Iterator::Item) type.
    ///
    /// # Examples
    ///
    /// ```
    /// use alternating_iter::AlternatingExt;
    ///
    /// let a = [1, 2];
    /// let b = [3, 4, 5];
    ///
    /// let mut iter = a.iter().alternate_with_no_remainder(b.iter());
    ///
    /// assert_eq!(iter.next(), Some(&1)); // a first
    /// assert_eq!(iter.next(), Some(&3)); // b
    /// assert_eq!(iter.next(), Some(&2)); // a
    /// assert_eq!(iter.next(), Some(&4)); // b
    /// assert_eq!(iter.next(), None);     // remaining items from b are not returned
    /// ```
    ///
    /// Importantly, the order of the iterators matter:
    /// ```
    /// # use std::iter;
    /// # use alternating_iter::AlternatingExt;
    ///
    /// let small = [1, 2];
    /// let big = [3, 4, 5];
    ///
    /// assert_eq!(small.iter().alternate_with_no_remainder(big.iter()).count(), 4);
    /// assert_eq!(big.iter().alternate_with_no_remainder(small.iter()).count(), 5);
    /// ```
    ///
    /// This behavior can be depicted as follow:
    ///
    /// Here is when `small` is on the left,
    /// ```txt
    /// small: 1 2 None
    ///        |/|/
    ///   big: 3 4
    /// ```
    /// And here is when `big` is on the left:`
    /// ```txt
    /// small: 3 4 5
    ///        |/|/|
    ///   big: 1 2 None
    /// ```
    fn alternate_with_no_remainder<I>(self, other: I) -> AlternatingNoRemainder<Self, I::IntoIter>
    where
        Self: Sized,
        I: IntoIterator<Item = Self::Item>,
    {
        AlternatingNoRemainder::new(self, other)
    }
}

impl<I> AlternatingExt for I where I: Iterator {}
