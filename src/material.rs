use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, Vec3};

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
