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

/// Curve25519 base point
pub static BASE: Point = Point([
	9, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0
]);

impl PartialEq for Scalar {
	fn eq(&self, other: &Self) -> bool {
		let &Scalar(lhs) = self;
		let &Scalar(rhs) = other;
		lhs == rhs
	}
}

impl Eq for Scalar {}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		let &Point(lhs) = self;
		let &Point(rhs) = other;
		lhs == rhs
	}
}

impl Eq for Point {}
