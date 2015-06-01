use std::cmp::{Eq, PartialEq};

/// Curve25519 scalar
pub struct Scalar([u8; 32]);

/// Curve25519 point
pub struct Point([u8; 32]);

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