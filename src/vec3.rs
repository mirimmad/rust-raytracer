use crate::common;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn random() -> Self {
        Vec3::new(
            common::random_double(),
            common::random_double(),
            common::random_double(),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            common::random_double_range(min, max),
            common::random_double_range(min, max),
            common::random_double_range(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        [self.x(), self.y(), self.z()].iter().all(|x| *x < EPS)
    }
}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1.0e-160 < lensq && lensq <= 1.0 {
            return p / f64::sqrt(lensq);
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, *normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}
