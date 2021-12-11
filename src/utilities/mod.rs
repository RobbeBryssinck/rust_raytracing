pub mod vectors;
pub mod raytracing;

#[cfg(test)]
mod tests {
	use crate::utilities::vectors::{Vec3, write_color};

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
	fn add_vec3() {
		let first_vec = Vec3::new(5.0, 6.0, 3.0);
		let second_vec = Vec3::new(7.0, 4.0, 1.0);
		let result_vec = Vec3::new(12.0, 10.0, 4.0);

		let third_vec = &first_vec + &second_vec;

		assert_eq!(third_vec, result_vec);
	}

	#[test]
	fn length_vec3() {
		let vec3 = Vec3::new(2.0, 3.0, -2.0);

		assert_eq!(vec3.length(), 4.123105625617661);
	}

	#[test]
	fn write_color_test() {
		let pixel_color = Vec3::new(5.0, 2.0, 3.0);

		assert_eq!(write_color(pixel_color), "1279.995 511.998 767.997\n");
	}
}