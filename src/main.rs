mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec3::{random_vector, unit_vector, Vec3};

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f32) / aspect_ratio).round() as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let spheres = vec![
        Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];
    let world: Vec<Box<dyn Hittable>> = spheres
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Hittable>)
        .collect();

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let camera = Camera {
        origin,
        horizontal,
        vertical,
        lower_left_corner,
    };

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f32 + rand::random::<f32>()) / ((image_width - 1) as f32);
                let v = (j as f32 + rand::random::<f32>()) / ((image_height - 1) as f32);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(pixel_color / samples_per_pixel as f32);
        }
    }
    eprintln!("\nDone");
}

fn write_color(pixel_color: Vec3) {
    // gamma-correct for gamma=2.0
    let r = pixel_color.r().sqrt();
    let g = pixel_color.g().sqrt();
    let b = pixel_color.b().sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)).floor(),
        (256.0 * clamp(g, 0.0, 0.999)).floor(),
        (256.0 * clamp(b, 0.0, 0.999)).floor()
    );
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

fn ray_color(ray: Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    match world.hit(ray, 0.001, f32::MAX) {
        Some(rec) => {
            let target = rec.p + rec.normal + random_unit_vector();
            0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, depth - 1)
        }
        None => {
            let unit_direction = unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vector(-1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
