enum Next {
    I,
    J,
    II,
    JJ,
}

/// See [`alternate_with`](AlternatingExt::alternate_with).
pub struct Alternating<I, J> {
    i: I,
    j: J,
    next: Next,
}

impl<I, J> Iterator for Alternating<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Next::I => {
                if let Some(i) = self.i.next() {
                    self.next = Next::J;
                    Some(i)
                } else {
                    self.next = Next::JJ;
                    self.j.next()
                }
            }
            Next::J => {
                if let Some(j) = self.j.next() {
                    self.next = Next::I;
                    Some(j)
                } else {
                    self.next = Next::II;
                    self.i.next()
                }
            }
            Next::II => self.i.next(),
            Next::JJ => self.j.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (i_lower, i_upper) = self.i.size_hint();
        let (j_lower, j_upper) = self.j.size_hint();
        (
            i_lower + j_lower,
            i_upper.and_then(|i| j_upper.map(|j| i + j)),
        )
    }
}

pub trait AlternatingExt: Iterator {
    /// Alternate between the items of two iterators. Once one of them run out, return only the other.
    ///
    /// Both iterators must have the same [`Item`](Iterator::Item) type.
    ///
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
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(4));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn alternate_with<J>(self, j: J) -> Alternating<Self, J>
    where
        Self: Sized,
        J: Iterator<Item = Self::Item>,
    {
        Alternating {
            i: self,
            j,
            next: Next::I,
        }
    }
}
impl<I> AlternatingExt for I where I: Iterator {}
