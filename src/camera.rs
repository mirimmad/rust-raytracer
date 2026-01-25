use crate::color::{self, Color};
use crate::common;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

const MAX_DEPTH: i32 = 50;
const LOOK_FROM: Vec3 = Vec3::new(13.0, 2.0, 3.0);
const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub struct Camera {
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(ascpect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        let image_height = (image_width as f64 / ascpect_ratio) as i32;
        let center = LOOK_FROM.clone();
        //let focal_length = (LOOK_FROM - LOOK_AT).length();
        let focus_dist = 10.0;
        let vfov = 20.0;
        let theta = common::degress_to_radians(vfov);
        let h = f64::tan(theta / 2.0);

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = vec3::unit_vector(LOOK_FROM - LOOK_AT);
        let u = vec3::unit_vector(vec3::cross(VUP, w));
        let v = vec3::cross(w, u);

        // Vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * v;

        // delta u and delta v from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // location of the upper left pixel
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_angle = 0.6;
        // calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * f64::tan(common::degress_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            eprint!("\rScanning remaining: {j}");
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, MAX_DEPTH, world);
                }
                color::write(pixel_color * (1.0 / self.samples_per_pixel as f64));
            }
        }
        eprint!("\nDone\n");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly
        // sampled point around pixel location i, j

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // Returns a vector to a random point in the [-.5, -.5] - [+.5, +.5] unit square
        Vec3::new(
            common::random_double() - 0.5,
            common::random_double() - 0.5,
            0.0,
        )
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // return a random point in the camera defocus disk
        let p = vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();
        if world.hit(r, 0.001, std::f64::INFINITY, &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if rec
                .mat
                .as_ref()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);

            //let direction = random_on_hemisphere(&rec.normal);
            /* let direction = rec.normal + vec3::random_unit_vector();
            return 0.5 * Self::ray_color(&Ray::new(rec.p, direction), depth - 1, world); */
            //return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
