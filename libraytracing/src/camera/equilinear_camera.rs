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

use crate::camera::*;
use crate::helpers::*;
use crate::ray::Ray;
use crate::world::World;
use image::{DynamicImage, GenericImage};
use na::{Rotation3, Unit, Vector3};
use std::f64;
use indicatif::ProgressBar;

pub struct EquilinearCamera {
    pub height: u32,
    pub width: u32,
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub pos: Vector3<f64>,
    pub vertical_viewangle: f64,
}

impl EquilinearCamera {
    pub fn point_at(&mut self, object: Vector3<f64>) -> () {
        self.yaw = 0.0;
        let (vert_angle, hort_angle) = vector2polar(&(object - self.pos));
        self.roll = vert_angle - f64::consts::FRAC_PI_2;
        self.pitch = -hort_angle + f64::consts::FRAC_PI_2;
    }
}

impl Camera for EquilinearCamera {
    fn render(&self, world: &World, progress: bool) -> DynamicImage {
        // algorithm for direction taken from https://www.scratchapixel.com/code.php?id=3&origin=/lessons/3d-basic-rendering/introduction-to-ray-tracing
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        let inv_width = 1.0 / self.width as f64;
        let inv_height = 1.0 / self.height as f64;
        let aspectratio = self.width as f64 / self.height as f64;
        let vertical_half_canvas_size =
            (f64::consts::FRAC_PI_2 * self.vertical_viewangle / 180.0).tan();
        let rot_matrix = Rotation3::from_euler_angles(self.roll, self.pitch, self.yaw);
        let bar = if progress {
            Some(ProgressBar::new((self.width * self.height).into()))
        } else {
            None
        };
        for x in 0..self.width {
            for y in 0..self.height {
                let xx = (2.0 * ((x as f64 + 0.5) * inv_width) - 1.0)
                    * vertical_half_canvas_size
                    * aspectratio;
                let yy = (2.0 * ((y as f64 + 0.5) * inv_height) - 1.) * vertical_half_canvas_size;
                let dir = rot_matrix * Vector3::new(xx, yy, 1.0).normalize();
                let ray = Ray {
                    dir: Unit::new_normalize(dir),
                    start: self.pos,
                };
                let rgb = world.color(ray, 10.0);

                img.put_pixel(x, self.height - y - 1, rgb);
                if let Some(bar) = &bar {
                    bar.inc(1);
                }

            }
        }
        if let Some(bar) = bar {
            bar.finish();
        }
        img
    }
}
