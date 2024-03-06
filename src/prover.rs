use num_bigint::BigUint;
use num_traits::ToPrimitive;
use crate::Commitment;

pub struct Prover {
    g: u128,
    a: u128,
    b: u128,
    q: u128,
}

impl Prover {

    pub fn new(g: u128, a: u128, b: u128, q: u128) -> Self {
        Prover {
            g, a, b, q,
        }
    }

    pub fn generate_commitment(&self, secret: u128) -> Commitment {
        let c1 = crate::pow_mod(self.g, BigUint::from(secret), self.q);
        let g_b = crate::pow_mod(self.g, BigUint::from(self.b), self.q);
        let c2 = crate::pow_mod(g_b, BigUint::from(secret), self.q);
        Commitment(c1, c2)
    }

    pub fn generate_proof(&self, secret: u128, challenge: u128) -> u128 {
        let secret_bn = BigUint::from(secret);
        let challenge_bn = BigUint::from(challenge);
        ((challenge_bn * (self.a) + secret_bn) % self.q).to_u128().unwrap()
    }
}
