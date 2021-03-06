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

use crate::ray::Ray;
use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Unit, Vector2, Vector3};

pub struct SpecularShader {
    pub alpha: f64,
}

impl SpecularShader {
    pub fn new(alpha: f64) -> Box<Shader> {
        Box::new(SpecularShader { alpha })
    }
}

impl Shader for SpecularShader {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        _surface_normal: Vector3<f64>,
        world: &World,
        _surface_pos: Vector2<f64>,
        _recursion_depth: f64,
    ) -> Vector3<f64> {
        let mut i_specular = Vector3::new(0.0, 0.0, 0.0);

        for light in &world.lights {
            let shade_ray = Ray {
                dir: Unit::new_normalize(intersection_pos - light.pos),
                start: light.pos,
            };

            if let Some(shade_intersection) = world.next_intersection(&shade_ray) {
                if (shade_intersection.pos - intersection_pos).norm() < 0.1 {
                    let l_m = -shade_ray.dir.normalize();
                    let n_hat = shade_intersection.normal_at_surface.normalize();
                    let r_hat = (2.0 * l_m.dot(&n_hat) * n_hat - l_m).normalize();
                    let v_hat = -ray_dir.normalize();
                    //TODO: put shininess(Reflektionsfaktor) in intersection
                    let rv = r_hat.dot(&v_hat);
                    i_specular += (if rv > 0.0 { rv } else { 0.0 }).powf(self.alpha) * light.color;
                }
            }
        }
        i_specular
    }
}
