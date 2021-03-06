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

use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shader::Shader;
use crate::world::Interceptable;
use na::{Vector2, Vector3};

pub struct Plane {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
    pub c: Vector3<f64>,
    pub shader: Box<Shader>,
}

impl Interceptable for Plane {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let edge_ab = (self.a - self.b).normalize();
        let edge_ac = (self.a - self.c).normalize();
        let normal = edge_ab.cross(&edge_ac);
        let convergence_rate = ray.dir.into_inner().dot(&normal);
        if convergence_rate == 0.0 {
            return None;
        }

        let intersection_distance = normal.dot(&(self.a - ray.start)) / convergence_rate;
        if intersection_distance < 0.0 {
            return None;
        }

        let intersection_pos = intersection_distance * ray.dir.into_inner() + ray.start;
        let normal = if convergence_rate > 0.0 {
            -normal
        } else {
            normal
        };
        let intersection = Intersection {
            pos: intersection_pos,
            normal_at_surface: normal,
            shader: &self.shader,
            pos_on_surface: Vector2::new(
                (self.a - intersection_pos).dot(&edge_ab),
                (self.a - intersection_pos).dot(&edge_ac),
            ),
        };
        return Some((intersection_distance, intersection));
    }
}
