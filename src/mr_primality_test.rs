extern crate num;
extern crate rand;

use rand::Rng;
use num::pow::pow;

pub fn is_prime(n: u64) -> bool {
    if n < 3 || n % 2 == 0 {
        return false;
    }
    let a = rand::thread_rng().gen_range(2, n-2);
    let x = pow(a, 3);
    
    return true;
}

#[test]
fn primes() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(11), true);
    assert_eq!(is_prime(5351), true);
    assert_eq!(is_prime(14867), true);
    assert_eq!(is_prime(36383), true);
    assert_eq!(is_prime(47699), true);
    assert_eq!(is_prime(99119), true);
}

#[test]
fn non_primes() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(10), false);
    assert_eq!(is_prime(67214), false);
    assert_eq!(is_prime(25230), false);
    assert_eq!(is_prime(99990), false);
}
