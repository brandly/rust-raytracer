mod ray;
mod vec3;

use ray::Ray;
use vec3::{unit_vector, Vec3};

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

fn write_color(pixel_color: Vec3) {
    let ir = (IMAGE_WIDTH as f32) * pixel_color.r();
    let ig = (IMAGE_WIDTH as f32) * pixel_color.g();
    let ib = (IMAGE_WIDTH as f32) * pixel_color.b();
    println!("{} {} {}", ir.round(), ig.round(), ib.round());
}

fn ray_color(r: Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let g = (j as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let b: f32 = 0.25;

            write_color(Vec3::new(r, g, b))
        }
    }
    eprintln!("\nDone");
}
