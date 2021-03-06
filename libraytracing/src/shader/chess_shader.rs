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

pub struct ChessShader {
    pub shader1: Box<Shader>,
    pub shader2: Box<Shader>,
    pub size: f64, //size of squares
}

fn my_mod(a: f64, b: f64) -> f64 {
    let mut x = a;
    while x > b {
        x -= b;
    }
    while x < 0.0 {
        x += b;
    }
    x
}

impl Shader for ChessShader {
    fn get_appearance_for(
        &self,
        _intersection_pos: Vector3<f64>,
        _ray_dir: Vector3<f64>,
        _surface_normal: Vector3<f64>,
        _world: &World,
        _surface_pos: Vector2<f64>,
        _recursion_depth: f64,
    ) -> Vector3<f64> {
        let modulo = (_surface_pos / self.size).map(|x| if my_mod(x, 2.0) <= 1.0 { 0 } else { 1 });
        let chooser = modulo.dot(&modulo);
        return if chooser == 0 || chooser == 2 {
            self.shader1.get_appearance_for(
                _intersection_pos,
                _ray_dir,
                _surface_normal,
                _world,
                _surface_pos,
                _recursion_depth,
            )
        } else {
            self.shader2.get_appearance_for(
                _intersection_pos,
                _ray_dir,
                _surface_normal,
                _world,
                _surface_pos,
                _recursion_depth,
            )
        };
    }
}
