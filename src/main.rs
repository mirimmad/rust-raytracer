mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use color::Color;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

    // delta u and delta v from pixel to pixel
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanning remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);
            color::write(pixel_color);
        }
    }
    eprint!("\nDone\n");
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let h = vec3::dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-h - f64::sqrt(discriminant)) / a
    }
}
