use core::iter;

#[allow(unused_imports)]
use crate::AlternatingExt;

/// Struct for alternating between the items of two iterators while handling size difference intuitively.
///
/// This struct is created by the [`AlternatingExt::alternate_with_all`] method, see its documentation for more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlternatingAll<I, J> {
    i: I,
    j: J,
    next: Next,
}

/// Represent the next iterator to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Next {
    I,
    J,
    /// Marks that iterator `j` has been exhausted
    IAlways,
    /// Marks that iterato `i` has been exhausted
    JAlways,
}

impl<I, J> AlternatingAll<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    /// Create a new `AlternatingAll` iterator from two other iterables.
    ///
    /// Alternative to [`AlternatingExt::alternate_with_all`]. There is no significant difference.
    pub fn new(i: impl IntoIterator<IntoIter = I>, j: impl IntoIterator<IntoIter = J>) -> Self {
        Self {
            i: i.into_iter(),
            j: j.into_iter(),
            next: Next::I,
        }
    }
}

impl<I, J> Iterator for AlternatingAll<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Next::I => {
                if let Some(item) = self.i.next() {
                    self.next = Next::J;
                    Some(item)
                } else {
                    self.next = Next::JAlways;
                    self.j.next()
                }
            }
            Next::J => {
                if let Some(item) = self.j.next() {
                    self.next = Next::I;
                    Some(item)
                } else {
                    self.next = Next::IAlways;
                    self.i.next()
                }
            }
            Next::IAlways => self.i.next(),
            Next::JAlways => self.j.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (i_lower, i_upper) = self.i.size_hint();
        let (j_lower, j_upper) = self.j.size_hint();
        (
            usize::saturating_add(i_lower, j_lower),
            i_upper.and_then(|i| j_upper.and_then(|j| usize::checked_add(i, j))),
        )
    }
}

impl<I, J> iter::ExactSizeIterator for AlternatingAll<I, J>
where
    I: iter::ExactSizeIterator,
    J: iter::ExactSizeIterator<Item = I::Item>,
{
    fn len(&self) -> usize {
        self.i.len() + self.j.len()
    }
}
impl<I, J> iter::FusedIterator for AlternatingAll<I, J>
where
    I: iter::FusedIterator,
    J: iter::FusedIterator<Item = I::Item>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_lengths() {
        let a = [1, 2, 3];
        let b = [4, 5, 6];

        let mut iter = a.iter().alternate_with_all(b.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn different_lengths() {
        let a = [1, 2, 3];
        let b = [4, 5];

        let mut iter = a.iter().alternate_with_all(b.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty_iterators() {
        let a: [i32; 0] = [];
        let b: [i32; 0] = [];

        let mut iter = a.iter().alternate_with_all(b.iter());

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn one_empty_iterator() {
        let a = [1, 2, 3];
        let b: [i32; 0] = [];

        let mut iter = a.iter().alternate_with_all(b.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn same_iterator() {
        let a = [1, 2, 3];

        let mut iter = a.iter().alternate_with_all(a.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn size_hint_accurate() {
        let a = [1, 2, 3];
        let b = [4, 5];

        assert_eq!(
            a.iter().size_hint().1,
            Some(a.iter().count()),
            "Sanity check failed"
        );
        assert_eq!(
            b.iter().size_hint().1,
            Some(b.iter().count()),
            "Sanity check failed"
        );

        let iter = a.iter().alternate_with_all(b.iter());

        assert_eq!(iter.size_hint().1, Some(iter.count()));
    }

    #[test]
    fn size_hint() {
        let a = [1, 2, 3];
        let b = [4, 5];

        assert_eq!(a.iter().size_hint(), (3, Some(3)), "Sanity check failed");
        assert_eq!(b.iter().size_hint(), (2, Some(2)), "Sanity check failed");

        let iter = a.iter().alternate_with_all(b.iter());

        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.count(), 5, "Inaccurate size hint");
    }
    #[test]
    fn size_hint_unbounded_right() {
        let a = [1, 2, 3];
        let b = iter::repeat(&0);

        assert_eq!(a.iter().size_hint(), (3, Some(3)), "Sanity check failed");
        assert_eq!(b.size_hint(), (usize::MAX, None), "Sanity check failed");

        let iter = a.iter().alternate_with_all(b);

        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }
    #[test]
    fn size_hint_unbounded_left() {
        let a = iter::repeat(&0);
        let b = [1, 2, 3];

        assert_eq!(a.size_hint(), (usize::MAX, None), "Sanity check failed");
        assert_eq!(b.iter().size_hint(), (3, Some(3)), "Sanity check failed");

        let iter = a.alternate_with_all(b.iter());

        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }
    #[test]
    fn size_hint_bound_exceed_max() {
        let a = 0..usize::MAX;
        let b = 0..3;

        assert_eq!(
            a.size_hint(),
            (usize::MAX, Some(usize::MAX)),
            "Sanity check failed"
        );
        assert_eq!(b.size_hint(), (3, Some(3)), "Sanity check failed");

        let iter = a.alternate_with_all(b);

        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }
    #[test]
    fn size_hint_bound_exactly_max() {
        let a = 0..usize::MAX;
        let b = 0..0;

        assert_eq!(
            a.size_hint(),
            (usize::MAX, Some(usize::MAX)),
            "Sanity check failed"
        );
        assert_eq!(b.size_hint(), (0, Some(0)), "Sanity check failed");

        let iter = a.alternate_with_all(b);

        assert_eq!(iter.size_hint(), (usize::MAX, Some(usize::MAX)));
    }
}
