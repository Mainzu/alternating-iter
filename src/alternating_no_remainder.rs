use core::iter;

use crate::utils::{checked, min_and_1, saturating};
#[allow(unused_imports)]
use crate::AlternatingExt;

/// Struct for alternating between the items of two iterators until one is exhausted.
///
/// This struct is created by the [`AlternatingExt::alternate_with_no_remainder`] method, see its documentation for more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlternatingNoRemainder<I, J> {
    i: I,
    j: J,
    last_i: bool,
}

impl<I, J> AlternatingNoRemainder<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    /// Create a new `AlternatingNoRemainder` iterator from two other iterables.
    ///
    /// Alternative to [`AlternatingExt::alternate_with_no_remainder`]. There is no  difference.
    pub fn new(i: impl IntoIterator<IntoIter = I>, j: impl IntoIterator<IntoIter = J>) -> Self {
        Self {
            i: i.into_iter(),
            j: j.into_iter(),
            last_i: false,
        }
    }
}

impl<I, J> Iterator for AlternatingNoRemainder<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.last_i {
            if let Some(item) = self.j.next() {
                self.last_i = false;
                Some(item)
            } else {
                None
            }
        } else {
            if let Some(item) = self.i.next() {
                self.last_i = true;
                Some(item)
            } else {
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (i_lower, i_upper) = self.i.size_hint();
        let (j_lower, j_upper) = self.j.size_hint();

        // The longest we can go without outputing consecutive elements
        // from the same iterator is twice the length of the shorter iterator.
        // We can squeeze 1 more if the other iterator is longer and
        // the last element was from the same iterator.

        let lower = saturating(min_and_1(i_lower, j_lower, self.last_i));
        let upper = match (i_upper, j_upper) {
            (Some(i_upper), Some(j_upper)) => checked(min_and_1(i_upper, j_upper, self.last_i)),
            (Some(i_upper), None) => checked((i_upper, self.last_i)),
            (None, Some(j_upper)) => checked((j_upper, !self.last_i)),
            // Since both have no upper bound, as far as we are concerned,
            // this mean they go on forever. Therefore, we don't have to worry
            // about one of them running out.
            (None, None) => None,
        };
        (lower, upper)
    }
}

// Deprecated: According to the documentation for ExactSizeIterator,
// "If an adapter makes an iterator longer, then it’s usually incorrect for
// that adapter to implement ExactSizeIterator."

// impl<I, J> iter::ExactSizeIterator for AlternatingNoRemainder<I, J>
// where
//     I: iter::ExactSizeIterator,
//     J: iter::ExactSizeIterator<Item = I::Item>,
// {
//     fn len(&self) -> usize {
//         saturating(min_and_1(self.i.len(), self.j.len(), self.last_i))
//     }
// }
impl<I, J> iter::FusedIterator for AlternatingNoRemainder<I, J>
where
    I: iter::FusedIterator,
    J: iter::FusedIterator<Item = I::Item>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_fused() {
        let a = 1..3;
        let b = iter::repeat(0);

        let mut iter = a.alternate_with_no_remainder(b);

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn same_lengths() {
        let a = [1, 2];
        let b = [3, 4];

        let mut iter = a.iter().alternate_with_no_remainder(b.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn different_lengths_1more() {
        let a = [1, 2];
        let b = [3, 4, 5];

        let mut iter = a.iter().alternate_with_no_remainder(b.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn different_lengths_2more() {
        let a = [1, 2];
        let b = [3, 4, 5, 6];

        let mut iter = a.iter().alternate_with_no_remainder(b.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty_iterators() {
        let a: [i32; 0] = [];
        let b: [i32; 0] = [];

        let mut iter = a.iter().alternate_with_no_remainder(b.iter());

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn one_empty_iterator() {
        let a = [1, 2, 3];
        let b: [i32; 0] = [];

        let mut iter = a.iter().alternate_with_no_remainder(b.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn same_iterator() {
        let a = [1, 2, 3];

        let mut iter = a.iter().alternate_with_no_remainder(a.iter());

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

        let iter = a.iter().alternate_with_no_remainder(b.iter());

        assert_eq!(iter.size_hint().1, Some(iter.count()));
    }
    #[test]
    fn size_hint() {
        let a = [1, 2, 3];
        let b = [4, 5];

        assert_eq!(a.iter().size_hint(), (3, Some(3)), "Sanity check failed");
        assert_eq!(b.iter().size_hint(), (2, Some(2)), "Sanity check failed");

        let iter = a.iter().alternate_with_no_remainder(b.iter());

        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.count(), 5, "Inaccurate size hint");
    }
    #[test]
    fn size_hint_unbounded_right() {
        let a = [1, 2, 3];
        let b = iter::repeat(&0);

        assert_eq!(a.iter().size_hint(), (3, Some(3)), "Sanity check failed");
        assert_eq!(b.size_hint(), (usize::MAX, None), "Sanity check failed");

        let iter = a.iter().alternate_with_no_remainder(b);

        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.count(), 6, "Inaccurate size hint");
    }
    #[test]
    fn size_hint_unbounded_left() {
        let a = iter::repeat(&0);
        let b = [1, 2, 3];

        assert_eq!(a.size_hint(), (usize::MAX, None), "Sanity check failed");
        assert_eq!(b.iter().size_hint(), (3, Some(3)), "Sanity check failed");

        let iter = a.alternate_with_no_remainder(b.iter());

        assert_eq!(iter.size_hint(), (7, Some(7)));
        assert_eq!(iter.count(), 7, "Inaccurate size hint");
    }
    #[test]
    fn size_hint_bound_exceed_max() {
        let a = 0..usize::MAX;
        let b = 0..usize::MAX;

        assert_eq!(
            a.size_hint(),
            (usize::MAX, Some(usize::MAX)),
            "Sanity check failed"
        );
        assert_eq!(
            b.size_hint(),
            (usize::MAX, Some(usize::MAX)),
            "Sanity check failed"
        );

        let iter = a.alternate_with_no_remainder(b);

        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }
    #[test]
    fn size_hint_bound_half_max_left() {
        let a = 0..usize::MAX / 2;
        let b = 0..usize::MAX / 2 + 1;

        assert_eq!(
            a.size_hint(),
            (usize::MAX / 2, Some(usize::MAX / 2)),
            "Sanity check failed"
        );
        assert_eq!(
            b.size_hint(),
            (usize::MAX / 2 + 1, Some(usize::MAX / 2 + 1)),
            "Sanity check failed"
        );

        let iter = a.alternate_with_no_remainder(b);

        assert_eq!(iter.size_hint(), (usize::MAX - 1, Some(usize::MAX - 1)));
    }
    #[test]
    fn size_hint_bound_half_max_right() {
        let a = 0..usize::MAX / 2 + 1;
        let b = 0..usize::MAX / 2;

        assert_eq!(
            a.size_hint(),
            (usize::MAX / 2 + 1, Some(usize::MAX / 2 + 1)),
            "Sanity check failed"
        );
        assert_eq!(
            b.size_hint(),
            (usize::MAX / 2, Some(usize::MAX / 2)),
            "Sanity check failed"
        );

        let iter = a.alternate_with_no_remainder(b);

        assert_eq!(iter.size_hint(), (usize::MAX, Some(usize::MAX)));
    }
    #[test]
    fn size_hint_both_unbounded() {
        let a = iter::repeat(0);
        let b = iter::repeat(0);

        assert_eq!(a.size_hint(), (usize::MAX, None), "Sanity check failed");
        assert_eq!(b.size_hint(), (usize::MAX, None), "Sanity check failed");

        let iter = a.alternate_with_no_remainder(b);

        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }
}
