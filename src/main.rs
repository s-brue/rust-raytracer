extern crate rand;
use rand::Rng;

use std::boxed::Box;

pub mod vec3;
use vec3::*;

pub mod ppm;
use ppm::*;

pub mod ray;
use ray::*;

pub mod hittable;
use hittable::*;

pub mod sphere;
use sphere::Sphere;

pub mod camera;
use camera::*;

fn color(objects: &Vec<Box<dyn Hittable>>, ray: &Ray) -> Vec3 {
    for object in objects {
        let res = object.hit(&ray, 0., 10000000.);
        if res.hit {
            let target = res.normal + res.point + sphere::random_in_unit_sphere();
            return color(&objects, &Ray::from_points(res.point, target)) * 0.5;
        }
    }

    let u: Vec3 = ray.direction.normalized();
    let t = 0.5 * (u.y + 1.0);
    (Vec3::to(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::to(0.5, 0.7, 1.0) * t)
}

fn main() {
    let w = 1000;
    let h = 500;

    let mut p = PPMFile::create("render.ppm", w, h);

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let camera = Camera::cam(
        Vec3::to(0, 0, 0),
        Vec3::to(-2, -1, -1),
        Vec3::to(4, 0, 0),
        Vec3::to(0, 2, 0),
    );

    objects.push(Box::new(Sphere {
        center: Vec3::to(0, 0, -1),
        radius: 0.5,
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::to(-1, 0, -1),
        radius: 0.5,
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::to(1, 0, -1),
        radius: 0.5,
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::to(0, -100.5, -1),
        radius: 100.,
    }));

    let n_samples: u16 = 4;

    for y in (0..h).rev() {
        for x in 0..w {
            let mut col = Vec3::to(0, 0, 0);
            for _sample in 0..n_samples {
                let r1: f64 = rand::thread_rng().gen_range(0., 1.);
                let r2: f64 = rand::thread_rng().gen_range(0., 1.);

                let u = (x as f64 + r1) / w as f64;
                let v = (y as f64 + r2) / h as f64;

                let ray = camera.get_ray(u, v);
                col += color(&objects, &ray);
            }
            col /= n_samples as f64;
            col.do_sqrt();
            col *= 255.99;

            p.write_next_pixel(col.x as u8, col.y as u8, col.z as u8);
        }
    }
}
