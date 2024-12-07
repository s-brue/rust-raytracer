use hittable::*;
use ray::*;
use vec3::*;

extern crate rand;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitResult {
        let oc = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let b = oc * ray.direction;
        let c = (oc * oc) - (self.radius * self.radius);
        let discriminant = b * b - a * c;
        let temp = (-b - discriminant.sqrt()) / a;
        if temp < t_max && temp > t_min {
            let p = ray.point_at_parameter(temp);
            return HitResult {
                hit: true,
                t: temp,
                point: p,
                normal: (p - self.center).normalized(),
            };
        }
        let temp = (-b + discriminant.sqrt()) / a;
        if temp < t_max && temp > t_min {
            let p = ray.point_at_parameter(temp);
            return HitResult {
                hit: true,
                t: temp,
                point: p,
                normal: (p - self.center).normalized(),
            };
        }
        HitResult {
            hit: false,
            t: 0.,
            point: Vec3::to(0, 0, 0),
            normal: Vec3::to(0, 0, 0),
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::to(
            rand::thread_rng().gen_range(-1., 1.),
            rand::thread_rng().gen_range(-1., 1.),
            rand::thread_rng().gen_range(-1., 1.),
        );
        if p * p < 1.0 {
            break;
        }
    }
    p.normalized()
}
