use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Vec3};

pub trait Material {
    fn scatter(&self, ray_in: Ray, rec: HitRecord) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray: Ray,
    pub color: Vec3,
}

pub struct Lambertian {
    pub albedo: Vec3, // color
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, rec: HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(Scatter {
            ray: Ray::new(rec.p, scatter_direction),
            color: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3, // color
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, rec: HitRecord) -> Option<Scatter> {
        let reflected = reflect(unit_vector(ray_in.direction), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some(Scatter {
                ray: scattered,
                color: self.albedo,
            })
        } else {
            None
        }
    }
}
