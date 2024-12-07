use std::fmt::*;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Display for Vec3 {
    fn fmt(&self, fmtr: &mut Formatter) -> Result {
        write!(fmtr, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub fn norm_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.norm();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn normalized(self) -> Vec3 {
        let mut v = self;
        v.normalize();
        v
    }
}

impl Vec3 {
    pub fn to<U: Into<f64>, V: Into<f64>, T: Into<f64>>(x: U, y: V, z: T) -> Vec3 {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, o: Vec3) {
        self.x += o.x;
        self.y += o.y;
        self.z += o.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, o: Vec3) {
        self.x -= o.x;
        self.y -= o.y;
        self.z -= o.z;
    }
}

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, o: Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}

impl<T: Into<f64>> Mul<T> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: T) -> Vec3 {
        let sc = s.into();
        Vec3 {
            x: sc * self.x,
            y: sc * self.y,
            z: sc * self.z,
        }
    }
}

impl<T: Into<f64>> MulAssign<T> for Vec3 {
    fn mul_assign(&mut self, s: T) {
        let sc = s.into();
        self.x *= sc;
        self.y *= sc;
        self.z *= sc;
    }
}

impl<T: Into<f64>> Div<T> for Vec3 {
    type Output = Vec3;

    fn div(self, s: T) -> Vec3 {
        let sc = s.into();
        Vec3 {
            x: self.x / sc,
            y: self.y / sc,
            z: self.z / sc,
        }
    }
}

impl<T: Into<f64>> DivAssign<T> for Vec3 {
    fn div_assign(&mut self, s: T) {
        let sc = s.into();
        self.x /= sc;
        self.y /= sc;
        self.z /= sc;
    }
}

impl Rem for Vec3 {
    type Output = Vec3;

    fn rem(self, o: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.z * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }
}

impl Vec3 {
    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - (normal * 2. * (normal * self))
    }
}

impl Vec3 {
    pub fn do_sqrt(&mut self) {
        self.x = self.x.sqrt();
        self.y = self.y.sqrt();
        self.z = self.z.sqrt();
    }
}

// Just for testing purposes (TODO: fp comparison)
impl PartialEq for Vec3 {
    fn eq(&self, o: &Vec3) -> bool {
        self.x == o.x && self.y == o.y && self.z == o.z
    }
}

// Some Naive Tests

#[test]
fn norm_test() {
    assert_eq!(Vec3::to(0., 0., 1.).norm(), 1.);
    assert_eq!(Vec3::to(1., 1., 1.).norm_squared(), 3.);
}

#[test]
fn add_test() {
    assert_eq!(
        Vec3::to(1., 1., 1.) + Vec3::to(1., 2., 3.),
        Vec3::to(2., 3., 4.)
    );
}

#[test]
fn sub_test() {
    assert_eq!(
        Vec3::to(1., 1., 1.) - Vec3::to(1., 2., 3.),
        Vec3::to(0., -1., -2)
    );
}

#[test]
fn dot_test() {
    assert_eq!(Vec3::to(0., 0., 1.) * Vec3::to(0., 0., 1.), 1.);
    assert_eq!(Vec3::to(0., 0., 1.) * Vec3::to(1., 0., 0.), 0.);
}

#[test]
fn cross_test() {
    assert_eq!(
        Vec3::to(0., 0., 1.) % Vec3::to(1., 0., 0.),
        Vec3::to(0, 1, 0)
    );
}

#[test]
fn reflect_test() {
    assert_eq!(
        Vec3::to(1., 0., 0.).reflect(Vec3::to(-1., 0., 0.)),
        Vec3::to(-1, 0, 0)
    );
}
