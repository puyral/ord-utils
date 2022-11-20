#[inline(always)]
pub fn sort<A: PartialOrd>(a: A, b: A) -> (A, A) {
    if PartialOrd::partial_cmp(&a, &b) == Some(std::cmp::Ordering::Less) {
        (a, b)
    } else {
        (b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sort(4, 2), (2, 4));
        assert_eq!(sort(7, 9), (7, 9));
    }
}
