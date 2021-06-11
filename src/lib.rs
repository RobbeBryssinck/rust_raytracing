use std::ops;

#[derive(Debug)]
pub struct Vec3 {
	x: f64,
	y: f64,
	z: f64,
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Vec3 {x, y, z}
	}

	pub fn print(&self) {
		println!("{} {} {}", self.x, self.y, self.z);
	}

	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f64 {
		self.x*self.x + self.y*self.y + self.z*self.z
	}

	pub fn dot(&self, rhs: &Vec3) -> f64 {
		self.x * rhs.x + 
		self.y * rhs.y + 
		self.z * rhs.z
	}

	pub fn cross(&self, rhs: &Vec3) -> Vec3 {
		Vec3::new(self.y * rhs.z - self.z * rhs.y,
				  self.z * rhs.x - self.x * rhs.z,
				  self.x * rhs.y - self.y * rhs.x)
	}
}

impl PartialEq for Vec3 {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x &&
		self.y == other.y &&
		self.z == other.z
	}
}

impl ops::Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Vec3 { x: -self.x, y: -self.y, z: -self.z }
	}
}

impl ops::AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Vec3) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

impl ops::MulAssign for Vec3 {
	fn mul_assign(&mut self, rhs: Vec3) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
	}
}

impl ops::DivAssign for Vec3 {
	fn div_assign(&mut self, rhs: Vec3) {
		self.x /= rhs.x;
		self.y /= rhs.y;
		self.z /= rhs.z;
	}
}

impl ops::Add for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Self::Output {
		Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl ops::Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: Vec3) -> Self::Output {
		Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}

impl ops::Mul for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Self::Output {
		Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
	}
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub mod utilities {
	use super::Color;

	pub fn write_color(pixel_color: Color) -> String {
		format!("{} {} {}\n", 255.999 * pixel_color.x as f64,
							255.999 * pixel_color.y as f64,
							255.999 * pixel_color.z as f64)
	}
}

#[cfg(test)]
mod tests {
	use crate::utilities::write_color;

use super::*;

	#[test]
	fn eq_vec3() {
		assert_eq!(Vec3{x: 5.0, y: 2.0, z: -3.0}, Vec3{x: 5.0, y: 2.0, z: -3.0});
	}

	#[test]
	fn neg_vec3() {
		assert_eq!(-Vec3{x: 5.0, y: 2.0, z: -3.0}, Vec3{x: -5.0, y: -2.0, z: 3.0});
	}

	#[test]
	fn add_assign_vec3() {
		let mut first_vec = Vec3::new(5.0, 6.0, 3.0);
		let second_vec = Vec3::new(7.0, 4.0, 1.0);
		let result_vec = Vec3::new(12.0, 10.0, 4.0);

		first_vec += second_vec;

		assert_eq!(first_vec, result_vec);
	}

	#[test]
	fn length_vec3() {
		let vec3 = Vec3::new(2.0, 3.0, -2.0);

		assert_eq!(vec3.length(), 4.123105625617661);
	}

	#[test]
	fn write_color_test() {
		let pixel_color = Vec3::new(5.0, 2.0, 3.0);

		assert_eq!(write_color(pixel_color), "1279.995 511.998 767.997");
	}
}
