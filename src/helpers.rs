use na::{Vector3,angle, Matrix};
use image::{Rgba, Rgb, Pixel};

// It's easier to do everything with Vectors, and then convert them to colors
//pub fn color2vector(c : &Rgba<f64>) -> Vector3<f64> {
//    let (r,g,b,_a) = c.channels4();
//    return Vector3::new(r,g,b);
//}

pub fn vector2color<T: na::Scalar + image::Primitive>(v: &Vector3<T>) -> Rgba<T> {
    Rgb::from_slice(v.as_slice()).to_rgba()
}

pub fn polar2vector(gamma: f64, phi: f64) -> Vector3<f64> {
    let y = gamma.cos();
    let x = gamma.sin() * phi.sin();
    let z = gamma.sin() * phi.cos();
    Vector3::new(x, y, z)
}

pub fn vector2polar<T: na::Real>(v: &Vector3<T>) -> (T, T) {
    let vertical_angle = Matrix::angle(&Vector3::y_axis().into_inner(), v);
    let projected_to_flat = Vector3::new(v.x, T::pi() - T::pi(), v.z);
    let horizontal_angle = Matrix::angle(&Vector3::x_axis().into_inner(), &projected_to_flat);
    (
        vertical_angle,
        if v.z > T::pi() - T::pi() {
            horizontal_angle
        } else {
            -horizontal_angle
        },
    )
}
