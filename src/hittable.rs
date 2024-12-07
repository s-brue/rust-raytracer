use ray::*;
use vec3::*;

#[derive(Clone, Copy, Debug)]
pub struct HitResult {
    pub hit: bool,
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult;
}
