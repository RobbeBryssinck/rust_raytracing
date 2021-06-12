use std::{fs::File, io::Write};

use rust_raytracing::utilities::{Vec3, Color, write_color};

fn main() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    file.write_all(header.as_bytes())?;

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = Vec3::new(i as f64 / (IMAGE_WIDTH-1) as f64, j as f64 / (IMAGE_HEIGHT-1) as f64, 0.25);
            file.write_all(write_color(pixel_color).as_bytes())?;
        }
    }

    println!("Done.");
    
    Ok(())
}
