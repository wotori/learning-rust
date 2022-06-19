pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    // implementing greatest common divisor function
    // using Euclidean algorithm
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(1 * 2 * 3, 10 * 11 * 13), 2);
}
