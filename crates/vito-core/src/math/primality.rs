pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let mut i = 3;
    while i <= n / i {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 2;
    }
    true
}

pub fn next_prime(n: u64) -> Option<u64> {
    (n.checked_add(1)?..=u64::MAX).find(|&c| is_prime(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_below_two_are_not_prime() {
        for n in [0, 1] {
            assert!(!is_prime(n), "{n} no es primo");
        }
    }

    #[test]
    fn small_primes_are_prime() {
        for n in [2, 3, 5, 7, 11, 13, 97] {
            assert!(is_prime(n), "{n} es primo");
        }
    }

    #[test]
    fn even_numbers_above_two_are_not_prime() {
        for n in [4, 6, 8, 100, 1_000_000] {
            assert!(!is_prime(n), "{n} es par y no es primo");
        }
    }

    #[test]
    fn squares_of_primes_are_not_prime() {
        for n in [4, 9, 25, 49, 121, 169, 961] {
            assert!(!is_prime(n), "{n} es un cuadrado perfecto");
        }
    }

    #[test]
    fn big_primes_are_prime() {
        for n in [7919, 999_983, 1_000_000_007] {
            assert!(is_prime(n), "{n} es primo");
        }
    }

    #[test]
    fn big_square_of_a_prime_is_not_prime() {
        let p: u64 = 999_983;
        assert!(!is_prime(p * p));
    }

    #[test]
    fn u64_max_does_not_overflow() {
        assert!(!is_prime(u64::MAX));
    }

    #[test]
    fn next_prime_of_small_numbers() {
        for (n, expected) in [
            (0, 2),
            (1, 2),
            (2, 3),
            (3, 5),
            (8, 11),
            (13, 17),
            (14, 17),
            (100, 101),
        ] {
            assert_eq!(next_prime(n), Some(expected), "next_prime({n})");
        }
    }

    #[test]
    fn next_prime_overflows_to_none() {
        assert_eq!(next_prime(u64::MAX), None);
        assert_eq!(next_prime(u64::MAX - 1), None);
    }

    #[test]
    fn next_prime_skips_no_primes() {
        for n in 0..2_000u64 {
            let p = next_prime(n).expect("hay primo siguiente");
            assert!(p > n && is_prime(p), "next_prime({n}) = {p}");
            for k in (n + 1)..p {
                assert!(!is_prime(k), "next_prime({n}) = {p}, pero {k} es primo");
            }
        }
    }
}
