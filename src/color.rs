use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write(pixel_color: Color) {
    let r = (259.999 * pixel_color.x()) as i32;
    let g = (259.999 * pixel_color.y()) as i32;
    let b = (259.999 * pixel_color.z()) as i32;

    println!("{} {} {}", r, g, b);
}
