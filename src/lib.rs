//! # `bls12_381`
//!
//! This crate provides an implementation of the BLS12-381 pairing-friendly elliptic
//! curve construction.
//!
//! * **This implementation has not been reviewed or audited. Use at your own risk.**
//! * This implementation targets Rust `1.36` or later.
//! * This implementation does not require the Rust standard library.
//! * All operations are constant time unless explicitly noted.

#![cfg_attr(not(feature = "std"), no_std)]
// Catch documentation errors caused by code changes.
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::many_single_char_names)]
// This lint is described at
// https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_arithmetic_impl
// In our library, some of the arithmetic involving extension fields will necessarily
// involve various binary operators, and so this lint is triggered unnecessarily.
#![allow(clippy::suspicious_arithmetic_impl)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
#[cfg(feature = "groups")]
mod tests;

#[macro_use]
mod util;

/// Notes about how the BLS12-381 elliptic curve is designed, specified
/// and implemented by this library.
pub mod notes {
    pub mod design;
    pub mod serialization;
}

mod choice;
mod scalar;

pub use scalar::Scalar as BlsScalar;
pub use scalar::{GENERATOR, ROOT_OF_UNITY, TWO_ADACITY};

#[cfg(feature = "rkyv-impl")]
pub use scalar::{ArchivedScalar as ArchivedBlsScalar, ScalarResolver as BlsScalarResolver};

#[cfg(feature = "groups")]
mod fp;
#[cfg(feature = "groups")]
mod fp2;
#[cfg(feature = "groups")]
mod g1;
#[cfg(feature = "groups")]
mod g2;

#[cfg(feature = "groups")]
pub use g1::{G1Affine, G1Projective};

#[cfg(all(feature = "groups", feature = "rkyv-impl"))]
pub use g1::{ArchivedG1Affine, G1AffineResolver};

#[cfg(feature = "groups")]
pub use g2::{G2Affine, G2Projective};

#[cfg(all(feature = "groups", feature = "rkyv-impl"))]
pub use g2::{ArchivedG2Affine, G2AffineResolver};

#[cfg(feature = "groups")]
mod fp12;
#[cfg(feature = "groups")]
mod fp6;

// The BLS parameter x for BLS12-381 is -0xd201000000010000
#[cfg(feature = "groups")]
const BLS_X: u64 = 0xd201000000010000;
#[cfg(feature = "groups")]
const BLS_X_IS_NEGATIVE: bool = true;

#[cfg(feature = "pairings")]
mod pairings;

#[cfg(feature = "pairings")]
pub use pairings::{pairing, Gt, MillerLoopResult};

#[cfg(all(feature = "pairings", feature = "rkyv-impl"))]
pub use pairings::{ArchivedGt, ArchivedMillerLoopResult, GtResolver, MillerLoopResultResolver};

#[cfg(all(feature = "pairings", feature = "alloc"))]
pub use pairings::{multi_miller_loop, G2Prepared};

#[cfg(all(feature = "pairings", feature = "rkyv-impl"))]
pub use pairings::{ArchivedG2Prepared, G2PreparedResolver};

#[cfg(all(feature = "groups", feature = "alloc"))]
pub mod multiscalar_mul;
