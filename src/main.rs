mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use material::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::{unit_vector, Vec3};

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f32) / aspect_ratio).round() as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let material_ground = Box::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center = Box::new(Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    });
    let material_left = Box::new(Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let material_right = Box::new(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    let spheres = vec![
        Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground,
        },
        Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: material_center,
        },
        Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left,
        },
        Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right,
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
        Some(rec) => match rec.material.scatter(ray, rec) {
            Some(scatter) => scatter.color * ray_color(scatter.ray, world, depth - 1),
            None => Vec3::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}
