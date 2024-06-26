# Alternating Iterators

[![Latest Version](https://img.shields.io/crates/v/alternating-iter)](https://crates.io/crates/alternating-iter)

This crate aims to provide a convenient way to alternate between the items of two iterators. It allows you to iterate over two iterators in an alternating fashion, combining their elements into a single sequence.

For the easiest usage of this crate, bring the [`AlternatingExt`](crate::AlternatingExt) trait into scope

```rust, no_run
use alternating_iter::AlternatingExt;
```

and use the [`alternate_with_all`](AlternatingExt::alternate_with_all) method to create new alternating iterators.

```rust
use alternating_iter::AlternatingExt;

let a = [1, 2];
let b = [3, 4, 5];

let mut iter = a.iter().alternate_with_all(b.iter());

assert_eq!(iter.next(), Some(&1)); // `a` first
assert_eq!(iter.next(), Some(&3)); // `b`
assert_eq!(iter.next(), Some(&2)); // `a`
assert_eq!(iter.next(), Some(&4)); // `b`
assert_eq!(iter.next(), Some(&5)); // also `b`
assert_eq!(iter.next(), None);
```

By default, the `alternate_with_all` method creates an iterator that returns an element from `a` first, followed by an element from `b`, and so on until both are exhausted.

## Stopping after Exhaustion

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

## Alternating Even After Exhaustion

If the [`alternate_with_all`](AlternatingExt::alternate_with_all) behavior is not desirable and you want to continue alternation even after an iterator is exhausted, use [`alternate_with`](AlternatingExt::alternate_with), the simplest iterator of the three.

```rust
use alternating_iter::AlternatingExt;

let a = [1, 2];
let b = [3, 4, 5];

let mut iter = a.iter().alternate_with(b.iter());

assert_eq!(iter.next(), Some(&1)); // `a` first
assert_eq!(iter.next(), Some(&3)); // `b`
assert_eq!(iter.next(), Some(&2)); // `a`
assert_eq!(iter.next(), Some(&4)); // `b`
assert_eq!(iter.next(), None);     // `a` exhausted
assert_eq!(iter.next(), Some(&5)); // `b`
assert_eq!(iter.next(), None);     // `b` exhausted
```

The iterator will simply keep alternating blindly, so `Some` can appear between `None` if one of the input iterators is larger than the other.

# Changelog

- 0.2: Renamed methods on the extension trait and fixed erroneous `FusedIterator` implementation
- 0.3: Removed erroneous `FixedSizedIterator` implementations
