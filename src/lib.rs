use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

pub struct Commitment(u128, u128);

impl Commitment {
    pub fn new(a: u128, b: u128) -> Self {
        Commitment(a, b)
    }
}

pub fn pow(base: u128, exponent: BigUint) -> BigUint {
    if exponent.is_zero() {
        return BigUint::from(1u32);
    }

    let mut result = BigUint::from(1u32);
    let mut base = BigUint::from(base);
    let mut exponent = exponent.clone();

    while exponent > BigUint::from(0u32) {
        if exponent.clone() % BigUint::from(2u32) == BigUint::from(1u32) {
            result *= base.clone();
        }
        base = base.clone() * base.clone();
        exponent /= BigUint::from(2u32);
    }

    BigUint::from(result)
}

pub fn pow_mod(base: u128, exp: BigUint, modulo: u128) -> u128 {
    let p = pow(base, exp);
    (p % modulo).to_u128().unwrap()
}


pub mod verifier;
pub mod prover;
