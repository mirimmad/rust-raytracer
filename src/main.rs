mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::rc::Rc;

use crate::camera::Camera;

use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use vec3::Point3;

use hittable_list::HittableList;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const SAMPLES_PER_PIXEL: i32 = 100;

fn main() {
    // World
    let mut world = HittableList::new();
    //world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    //world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.50));
    let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL);
    camera.render(&world);
}
