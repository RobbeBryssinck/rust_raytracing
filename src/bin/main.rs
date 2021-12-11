use std::{fs::File, io::Write};

use std::env;
use log::info;
use log::debug;
use log::trace;

use rust_raytracing::utilities::raytracing::Ray;
use rust_raytracing::utilities::vectors::{Vec3, Color, Point3};
use rust_raytracing::utilities::vectors;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "TRACE");
    env_logger::init();

    let mut file = File::create("image.ppm")?;

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    debug!("Ratio: {}, Width: {}, Height: {}", ASPECT_RATIO, IMAGE_WIDTH, IMAGE_HEIGHT);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    debug!("Focal length: {}, VP width: {}, VP height: {}", focal_length, viewport_width, viewport_height);

    let origin = Point3::new(0.0, 0.0, 0.0);
    debug!("origin: {:?}", origin);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    debug!("horizontal: {:?}", horizontal);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    debug!("vertical: {:?}", vertical);
    let lower_left_corner = origin - horizontal.divide(2.0) - vertical.divide(2.0) - Vec3::new(0.0, 0.0, focal_length);
    debug!("lower_left_corner: {:?}", lower_left_corner);

    let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    file.write_all(header.as_bytes())?;

    info!("Starting image generation.");
    for j in (0..IMAGE_HEIGHT).rev() {
        trace!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT-1) as f64;

            let r = Ray::new(origin, lower_left_corner + horizontal.multiply(u) + vertical.multiply(v) - origin);

            let pixel_color: Color = r.ray_color();
            trace!("pixel_color: {:?}", pixel_color);

            file.write_all(vectors::write_color(pixel_color).as_bytes())?;
        }
    }

    info!("Done.");
    
    Ok(())
}
