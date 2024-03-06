use num_bigint::BigUint;
use num_traits::ToPrimitive;
use crate::Commitment;

pub struct Verifier {
    g: u128,
    a: u128,
    b: u128,
    q: u128,
}

impl Verifier {
    pub fn new(g: u128, a: u128, b: u128, q: u128) -> Self {
        Verifier {
            g, a, b, q,
        }
    }

    pub fn generate_challenge(&mut self) -> u128 {
        rand::random::<u128>() % 1000
    }

    pub fn verify_proof(&self, proof: u128, challenge: u128, commitment: &Commitment) -> bool {
        let a = crate::pow_mod(self.g, BigUint::from(self.a), self.q);
        let b = crate::pow_mod(self.g, BigUint::from(self.b), self.q);
        let ab = crate::pow_mod(self.g, BigUint::from(self.a * self.b), self.q);

        let chal_comm0 = (crate::pow(a, BigUint::from(challenge)) * BigUint::from(commitment.0) % self.q).to_u128().unwrap();
        let chal_comm1 = (crate::pow(ab, BigUint::from(challenge)) * BigUint::from(commitment.1) % self.q).to_u128().unwrap();

        let condition1 = crate::pow_mod(self.g, BigUint::from(proof), self.q) == chal_comm0;
        let condition2 = crate::pow_mod(b, BigUint::from(proof), self.q) == chal_comm1;

        condition1 && condition2
    }
}
