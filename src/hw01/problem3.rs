use std::collections::HashSet;
use std::iter::FromIterator;

/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes: HashSet<u32> = (2..n).collect();

    for i in 2..n {
        // If the vector does not contain this number, then it is non-prime. Just skip.
        if !primes.contains(&i) {
            continue;
        }
        // Else, it is prime, and we remove all multiples of i from i * i to n
        let mut mult = i;
        while i * mult < n {
            primes.remove(&(i * mult));
            mult += 1;
        }
    }

    let mut primes_vec = Vec::from_iter(primes.into_iter());
    primes_vec.sort();
    return primes_vec;
}