use std::cmp::{Eq, PartialEq};
use std::ops::{Add,Sub,Mul};

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

/// Curve25519 field element
struct Element([i64; 10]);

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

impl PartialEq for Element {
	fn eq(&self, other: &Self) -> bool {
		let &Element(lhs) = self;
		let &Element(rhs) = other;
		lhs == rhs
	}
}

impl Eq for Element {}

impl Add for Element {
	type Output = Element;

	fn add(self, other: Element) -> Self::Output {
		let &Element(lhs) = &self;
		let &Element(rhs) = &other;

		Element([
			lhs[0] + rhs[0], lhs[1] + rhs[1],
			lhs[2] + rhs[2], lhs[3] + rhs[3],
			lhs[4] + rhs[4], lhs[5] + rhs[5],
			lhs[6] + rhs[6], lhs[7] + rhs[7],
			lhs[8] + rhs[8], lhs[9] + rhs[9],
		])
	}
}

impl Sub for Element {
	type Output = Element;

	fn sub(self, other: Element) -> Self::Output {
		let &Element(lhs) = &self;
		let &Element(rhs) = &other;

		Element([
			lhs[0] - rhs[0], lhs[1] - rhs[1],
			lhs[2] - rhs[2], lhs[3] - rhs[3],
			lhs[4] - rhs[4], lhs[5] - rhs[5],
			lhs[6] - rhs[6], lhs[7] - rhs[7],
			lhs[8] - rhs[8], lhs[9] - rhs[9],
		])
	}
}
