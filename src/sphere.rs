use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let h = vec3::dot(ray.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f64::sqrt(discriminant);

        // Nearest root that lies in the acceptable range
        let mut root = (-h - sqrt_d) / a;
        if root <= t_min || root >= t_max {
            root = (-h + sqrt_d) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        // root
        rec.t = root;

        // point of intersection
        rec.p = ray.at(rec.t);

        // normal (unit length)
        let outwards_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outwards_normal);
        true
    }
}
