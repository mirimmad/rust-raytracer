use crate::common;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write(pixel_color: Color) {
    let [r, g, b] = [pixel_color.x(), pixel_color.y(), pixel_color.z()].map(linear_to_gamma);

    let ri = (256.0 * common::clamp(r, 0.0, 0.999)) as i32;
    let gi = (256.0 * common::clamp(g, 0.0, 0.999)) as i32;
    let bi = (256.0 * common::clamp(b, 0.0, 0.999)) as i32;

    println!("{ri} {gi} {bi}");
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return f64::sqrt(linear_component);
    }

    0.0
}
