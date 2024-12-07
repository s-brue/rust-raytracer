use vec3::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn from_points(origin: Vec3, other_point: Vec3) -> Ray {
        let mut r = Ray {
            origin,
            direction: other_point - origin,
        };
        r.direction.normalize();
        r
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}
