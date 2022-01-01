mod hittable;
mod ray;
mod sphere;
mod vec3;

use ray::Ray;
use vec3::{unit_vector, Vec3};

fn write_color(pixel_color: Vec3) {
    let ir = 255.999 * pixel_color.r();
    let ig = 255.999 * pixel_color.g();
    let ib = 255.999 * pixel_color.b();
    println!("{} {} {}", ir.floor(), ig.floor(), ib.floor());
}

fn ray_color(ray: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let unit_direction = unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

// center + radius is the sphere
// does ray hit it?
// define quadratic and solve
fn hit_sphere(center: Vec3, radius: f32, ray: Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.squared_length();
    let half_b = oc.dot(ray.direction);
    let c = oc.squared_length() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;

    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f32) / aspect_ratio).round() as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f32) / ((image_width - 1) as f32);
            let v = (j as f32) / ((image_height - 1) as f32);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r);

            write_color(pixel_color)
        }
    }
    eprintln!("\nDone");
}
