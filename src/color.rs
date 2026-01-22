use crate::common;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write(pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let ri = (256.0 * common::clamp(r, 0.0, 0.999)) as i32;
    let gi = (256.0 * common::clamp(g, 0.0, 0.999)) as i32;
    let bi = (256.0 * common::clamp(b, 0.0, 0.999)) as i32;

    println!("{} {} {}", ri, gi, bi);
}
