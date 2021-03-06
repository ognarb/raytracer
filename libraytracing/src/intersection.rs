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
use image::Rgba;
use na::{Vector2, Vector3};

pub struct Intersection<'a> {
    pub pos: Vector3<f64>,
    pub normal_at_surface: Vector3<f64>,
    pub pos_on_surface: Vector2<f64>,
    pub shader: &'a Box<Shader>,
}

impl<'a> Intersection<'a> {
    pub fn get_color(
        &self,
        ray_dir: Vector3<f64>,
        world: &World,
        recursion_depth: f64,
    ) -> Rgba<u8> {
        self.shader.get_color_for(
            self.pos,
            ray_dir,
            self.normal_at_surface,
            world,
            self.pos_on_surface,
            recursion_depth,
        )
    }
    pub fn get_appearance(
        &self,
        ray_dir: Vector3<f64>,
        world: &World,
        recursion_depth: f64,
    ) -> Vector3<f64> {
        self.shader.get_appearance_for(
            self.pos,
            ray_dir,
            self.normal_at_surface,
            world,
            self.pos_on_surface,
            recursion_depth,
        )
    }
}
