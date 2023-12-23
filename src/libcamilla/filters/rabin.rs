// Rabin fingerprint

use std::mem::size_of;

const POLYNOMIAL_BITS: usize = size_of::<usize>();

trait RabinPolynomial {
    /*
    Calculate the remainder of self divided by divisor in GF(2)
     */
    fn galois_remainder(&self, divisor: &Self) -> Self;
}

impl RabinPolynomial for usize {
    /*
    Euclidean algorithm
     */
    #[inline]
    fn galois_remainder(&self, divisor: &Self) -> Self {
        let mut res = *self;
        while res.leading_zeros() <= divisor.leading_zeros() {
            res ^= divisor << (divisor.leading_zeros() - res.leading_zeros());
        }
        res
    }
}