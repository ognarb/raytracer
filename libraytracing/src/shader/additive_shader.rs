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

use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Vector2, Vector3};
use std::ops::Add;

pub struct AdditiveShader {
    pub shader1: Box<Shader>,
    pub shader2: Box<Shader>,
}

impl Shader for AdditiveShader {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        surface_pos: Vector2<f64>,
        recursion_depth: f64,
    ) -> Vector3<f64> {
        let ap1 = self.shader1.get_appearance_for(
            intersection_pos,
            ray_dir,
            surface_normal,
            world,
            surface_pos,
            recursion_depth,
        );
        let ap2 = self.shader2.get_appearance_for(
            intersection_pos,
            ray_dir,
            surface_normal,
            world,
            surface_pos,
            recursion_depth,
        );
        ap1 + ap2
    }
}
// Add 2 Boxed Shader
impl Add for Box<Shader> {
    type Output = Box<Shader>;

    fn add(self, other: Box<Shader>) -> Box<Shader> {
        Box::new(AdditiveShader {
            shader1: self,
            shader2: other,
        })
    }
}

// Add unboxed Shader to boxed shader (boxing is done here)
impl<T: Shader + 'static> Add<T> for Box<Shader> {
    type Output = Box<Shader>;

    fn add(self, other: T) -> Box<Shader> {
        Box::new(AdditiveShader {
            shader1: self,
            shader2: Box::new(other),
        })
    }
}
