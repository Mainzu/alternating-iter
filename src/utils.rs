pub(crate) fn min_and_1(i: usize, j: usize, last_i: bool) -> (usize, bool) {
    use core::cmp::Ordering;

    match i.cmp(&j) {
        Ordering::Less => (i, last_i),
        Ordering::Equal => (i, false),
        Ordering::Greater => (j, !last_i),
    }
}
pub(crate) fn saturating((min, add_one): (usize, bool)) -> usize {
    min.saturating_mul(2).saturating_add(add_one as usize)
}
pub(crate) fn checked((min, add_one): (usize, bool)) -> Option<usize> {
    min.checked_mul(2)
        .and_then(|min| min.checked_add(add_one as usize))
}
