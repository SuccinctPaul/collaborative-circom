pub(crate) mod prover;
pub(crate) mod types;
pub(crate) mod verifier;

use super::polynomial::Polynomial;
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;

pub(crate) struct ZeroMorphOpeningClaim<F: PrimeField> {
    pub(crate) polynomial: Polynomial<F>,
    pub(crate) opening_pair: OpeningPair<F>,
}

pub(crate) struct OpeningPair<F: PrimeField> {
    pub(crate) challenge: F,
    pub(crate) evaluation: F,
}

pub(crate) struct ZeroMorphVerifierOpeningClaim<P: Pairing> {
    pub(crate) challenge: P::ScalarField,
    pub(crate) evaluation: P::ScalarField,
    pub(crate) commitment: P::G1,
}
