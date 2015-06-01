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

macro_rules! expand {
	($pt:ident, $off:expr, $shift:expr, $mask:expr) => (
		( ($pt[$off + 0] as i64)
		| ($pt[$off + 1] as i64) << 8
		| ($pt[$off + 2] as i64) << 16
		| ($pt[$off + 3] as i64) << 24
		) >> $shift & $mask;
	);
}

impl Element {
	fn expand(point: &Point) -> Self {
		let &Point(pt) = point;

		Element([
			expand!(pt, 0, 0, 0x3ffffff),
			expand!(pt, 3, 2, 0x1ffffff),
			expand!(pt, 6, 3, 0x3ffffff),
			expand!(pt, 9, 5, 0x1ffffff),
			expand!(pt, 12, 6, 0x3ffffff),
			expand!(pt, 16, 0, 0x1ffffff),
			expand!(pt, 19, 1, 0x3ffffff),
			expand!(pt, 22, 3, 0x1ffffff),
			expand!(pt, 25, 4, 0x3ffffff),
			expand!(pt, 28, 6, 0x1ffffff),
		])
	}
}

impl Mul<Point> for Scalar {
	type Output = Point;

	fn mul(self, other: Point) -> Self::Output {
		let &Scalar(scalar) = &self;

		let mut e = scalar;

		e[0] &= 248;
		e[31] &= 127;
		e[31] |= 64;

		let bp = Element::expand(&other);

		// FIXME: dummy return value
		other
	}
}
