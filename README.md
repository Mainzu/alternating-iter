# Alternating Iterators

![Latest Version](https://img.shields.io/crates/v/alternating-iter)

This crate aims to provides a convenient way to alternate between the items of two iterators. It allows you to iterate over two iterators in an alternating fashion, combining their elements into a single sequence.

For the simplest usage of this crate, bring the [`AlternatingExt`](crate::AlternatingExt) trait into scope

```rust, no_run
use alternating_iter::AlternatingExt;
```

and use the [`alternate_with`](AlternatingExt::alternate_with) method to create new alternating iterators.

```rust
# use alternating_iter::AlternatingExt;

let a = [1, 2, 3];
let b = [4, 5];

let mut alternating = a.iter().alternate_with(b.iter());

assert_eq!(alternating.next(), Some(&1));
assert_eq!(alternating.next(), Some(&4));
assert_eq!(alternating.next(), Some(&2));
assert_eq!(alternating.next(), Some(&5));
assert_eq!(alternating.next(), Some(&3));
assert_eq!(alternating.next(), None);
```

By default the `alternate_with` method creates an iterator that returns an element from `a` first, followed by element from `b`, and so on until both are exhausted.

### Stopping after Exhaustion

If, however, you want the iteration to stop once either of the iterators is exhausted, you can use the [`alternate_with_no_remainder`](AlternatingExt::alternate_with_no_remainder) method, also provided by the `AlternatingExt` trait. This method returns an iterator that stops as soon as it needs to return more than one item consecutively from a single iterator.

```rust
use alternating_iter::AlternatingExt;

let a = [1, 2];
let b = [3, 4, 5];

let mut iter = a.iter().alternate_with_no_remainder(b.iter());

assert_eq!(iter.next(), Some(&1)); // `a` first
assert_eq!(iter.next(), Some(&3)); // `b`
assert_eq!(iter.next(), Some(&2)); // `a`
assert_eq!(iter.next(), Some(&4)); // `b`
assert_eq!(iter.next(), None);     // remaining items from `b` are not returned
```

The iteration stops after the fourth element because returning the fifth element from `b` would break the alternating pattern.
