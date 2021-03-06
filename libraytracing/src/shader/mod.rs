/**
 * Copyright © 2019
 * Sami Shalayel <sami.shalayel@tutamail.com>,
 * Carl Schwan <carl@carlschwan.eu>,
 * Daniel Freiermuth <d_freiermu14@cs.uni-kl.de>
 *
 * This work is free. You can redistribute it and/or modify it under the
 * terms of the Do What The Fuck You Want To Public License, Version 2,
 * as published by Sam Hocevar. See the LICENSE file for more details.
 * 
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See the LICENSE
 * file for more details. **/

use crate::helpers::*;
use crate::shader::ambient_shader::AmbientShader;
use crate::shader::chess_shader::*;
use crate::shader::diffuse_shader::*;
use crate::shader::monochrome_shader::*;
use crate::shader::specular_shader::*;
use crate::world::World;
use image::Rgba;
use nalgebra::{Vector2, Vector3};

pub trait Shader {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        surface_pos: Vector2<f64>,
        recursion_depth: f64,
    ) -> Vector3<f64>;
    //default implementation to get a rgb<u8> (instead of a vector<f64>)
    fn get_color_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        surface_pos: Vector2<f64>,
        recursion_depth: f64,
    ) -> Rgba<u8> {
        let val = &self.get_appearance_for(
            intersection_pos,
            ray_dir,
            surface_normal,
            world,
            surface_pos,
            recursion_depth,
        );
        let u8val = val.map(|x| (x * 255.0).min(255.0).max(0.0) as u8);
        vector2color(&u8val)
    }
}

pub fn get_phong(color: Vector3<f64>) -> Box<Shader> {
    let diffuse_shader = DiffuseShader::new(color);
    let specular_shader = SpecularShader::new(10.0);
    let ambient_shader = AmbientShader::new(color);
    return 0.5 * diffuse_shader + specular_shader + 0.8 * ambient_shader;
}

pub fn get_bw_chess() -> Box<Shader> {
    let black_shader = Box::new(MonochromeShader {
        color: Vector3::new(0.0, 0.0, 0.0),
    });
    let white_shader = Box::new(MonochromeShader {
        color: Vector3::new(1.0, 1.0, 1.0),
    });
    Box::new(ChessShader {
        shader1: black_shader,
        shader2: white_shader,
        size: 1.0,
    })
}

pub mod additive_shader;
pub mod ambient_shader;
pub mod chess_shader;
pub mod diffuse_shader;
pub mod mirror_shader;
pub mod monochrome_shader;
pub mod multiplicative_shader;
pub mod specular_shader;
