use image::Rgba;
use nalgebra::{Vector3, Vector2};
use crate::world::World;
use crate::shader::specular_shader::*;
use crate::shader::ambient_shader::*;
use crate::shader::diffuse_shader::*;
use crate::shader::additive_shader::*;
use crate::helpers::*;

pub trait Shader {
    fn get_appearance_for(&self, intersection_pos: Vector3<f64>, ray_dir: Vector3<f64>, surface_normal: Vector3<f64>,
                          world: &World, surface_pos: Vector2<f64>, recursion_depth: u64) -> Vector3<f64>;
    fn get_color_for(&self, intersection_pos: Vector3<f64>, ray_dir: Vector3<f64>, surface_normal: Vector3<f64>,
                          world: &World, surface_pos: Vector2<f64>, recursion_depth: u64) -> Rgba<u8> {
        let val = &self.get_appearance_for(intersection_pos, ray_dir, surface_normal, world, surface_pos, recursion_depth);
        let u8val = val.map(|x| (x * 255.0).min(255.0).max(0.0) as u8);
        vector2color(&u8val)
    }
}

pub fn get_phong(color: Vector3<f64>) -> Box<Shader> {
    let diffuse_shader = DiffuseShader { color: color, reflection: 0.5 };
    let specular_shader = SpecularShader { reflection: 1.0 , shininess: 1.0 , alpha: 10.0 };
    let ambient_shader = AmbientShader { reflection: 0.5, light: Vector3::new(0.1, 0.1, 0.1)};
    let comb1 = AdditiveShader { shader1: Box::new(diffuse_shader), shader2: Box::new(specular_shader), alpha1: 1.0, alpha2: 1.0};
    let comb2 = AdditiveShader { shader1: Box::new(ambient_shader), shader2: Box::new(comb1), alpha1: 1.0, alpha2: 1.0};
    return Box::new(comb2);
}

pub mod monochrome_shader;
pub mod additive_shader;
pub mod ambient_shader;
pub mod diffuse_shader;
pub mod specular_shader;
pub mod mirror_shader;
