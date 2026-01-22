mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::ray::Ray;
use crate::vec3::Point3;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH);
    camera.render(&world);
}
