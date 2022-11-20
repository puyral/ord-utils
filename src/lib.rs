pub fn sort<A:Ord>(a: A, b:A) -> (A, A){
    if a < b {
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
        assert_eq!(sort(4,2) , (2,4));
        assert_eq!(sort(7,9) , (7,9));
    }
}
