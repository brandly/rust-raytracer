use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;

        for hittable in self {
            if let Some(candidate_hit) = hittable.hit(ray, t_min, t_max) {
                match hit {
                    None => hit = Some(candidate_hit),
                    Some(prev) => {
                        if candidate_hit.t < prev.t {
                            hit = Some(candidate_hit);
                        }
                    }
                }
            }
        }

        hit
    }
}
