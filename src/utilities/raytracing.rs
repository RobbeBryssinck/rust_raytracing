use super::vectors::{Color, Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &self.direction.multiply(t)
    }

    pub fn ray_color(&self) -> Color {
        let unit_direction = Vec3::unit_vector(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0).multiply(1.0 - t) + Color::new(0.5, 0.7, 1.0).multiply(t)
    }
}