use crate::color::{self, Color};
use crate::common;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Camera {
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
}

impl Camera {
    pub fn new(ascpect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        let image_height = (image_width as f64 / ascpect_ratio) as i32;
        let center = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

        // delta u and delta v from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    pub fn render(&self, world: &HittableList) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            eprint!("\rScanning remaining: {}", j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for p in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, world);
                }
                color::write(pixel_color * (1.0 / self.samples_per_pixel as f64));
            }
        }
        eprint!("\nDone\n");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly
        // sampled point around pixel location i, j

        let offset = Self::sampe_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center.clone();
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sampe_square() -> Vec3 {
        // Returns a vector to a random point in the [-.5, -.5] - [+.5, +.5] unit square
        Vec3::new(
            common::random_double() - 0.5,
            common::random_double() - 0.5,
            0.0,
        )
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(r, 0.0, std::f64::INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
