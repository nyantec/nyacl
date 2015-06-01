use std::cmp::{Eq, PartialEq};

/// Curve25519 scalar
#[derive(Clone, Copy)]
pub struct Scalar([u8; 32]);

/// Curve25519 point
#[derive(Clone, Copy)]
pub struct Point([u8; 32]);

/// Curve25519 secret key
pub type SecretKey = Scalar;

/// Curve25519 public key
pub type PublicKey = Point;

impl PartialEq for Scalar {
	fn eq(&self, rhs: &Self) -> bool {
		let &Scalar(lhs_array) = self;
		let &Scalar(rhs_array) = rhs;
		lhs_array == rhs_array
	}
}

impl Eq for Scalar {}

impl PartialEq for Point {
	fn eq(&self, rhs: &Self) -> bool {
		let &Point(lhs_array) = self;
		let &Point(rhs_array) = rhs;
		lhs_array == rhs_array
	}
}

impl Eq for Point {}
