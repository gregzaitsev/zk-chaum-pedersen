use num_bigint::BigUint;
use num_traits::ToPrimitive;
use chaum_pederson::prover::Prover;
use chaum_pederson::verifier::Verifier;
use chaum_pederson::{pow, pow_mod};


#[test]
fn test_pow_2_10() {
    let res = pow(2, BigUint::from(10u32));
    assert_eq!(res.to_u128(), Some(1024u128))
}

#[test]
fn test_pow_2_50() {
    let res = pow(2, BigUint::from(50u32));
    assert_eq!(res.to_string(), "1125899906842624".to_string())
}

#[test]
fn test_pow_mod_2_50() {
    let res = pow_mod(2, BigUint::from(50u32), 10000);
    assert_eq!(res.to_string(), "2624".to_string())
}

#[test]
fn test_full_workflow_correct_secret() {
    let prover = Prover::new(3, 10, 13, 10009);
    let mut verifier = Verifier::new(3, 10, 13, 10009);

    let secret = rand::random::<u128>() % 1000;
    let commitment = prover.generate_commitment(secret);

    let challenge = verifier.generate_challenge();
    let proof = prover.generate_proof(secret, challenge);

    assert_eq!(verifier.verify_proof(proof, challenge, &commitment), true);
}

#[test]
fn test_full_workflow_incorrect_secret() {
    let prover = Prover::new(3, 10, 13, 10009);
    let mut verifier = Verifier::new(3, 10, 13, 10009);

    let secret = rand::random::<u128>() % 1000;
    let wrong_secret = (secret + 1) % 1000;
    let commitment = prover.generate_commitment(secret);

    let challenge = verifier.generate_challenge();
    let proof = prover.generate_proof(wrong_secret, challenge);

    assert_eq!(verifier.verify_proof(proof, challenge, &commitment), false);
}
