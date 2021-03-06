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
use crate::storage::primitive_storage::PrimitiveStorage;
use crate::storage::Bounded;
use crate::world::Interceptable;
use nalgebra::Vector3;
use crate::helpers::*;
use std::f64;

pub struct BVStorage {
    left: Box<Interceptable>,
    right: Box<Interceptable>,
    min: Vector3<f64>,
    max: Vector3<f64>,
}

impl BVStorage {
	fn from(elements: Vec<Box<Bounded>>, min: Vector3<f64>, max:Vector3<f64>) -> Self {
		return BVStorage {
				min,
				max,
				left: Box::new(PrimitiveStorage { elements: bounded2interceptable(elements)}),
				right: Box::new(PrimitiveStorage { elements: Vec::new()}),
			}
	}
	
    pub fn new(elements: Vec<Box<Bounded>>) -> Self {
        let box_min = pointwise_min_list(elements.iter().map(|v| v.get_min()).collect());
        let box_max = pointwise_max_list(elements.iter().map(|v| v.get_max()).collect());

        let mins_max = pointwise_max_list(elements.iter().map(|v| v.get_min()).collect());

        let split_dimension = (mins_max - box_min).iamax();
        let split_point = (mins_max[split_dimension] + box_min[split_dimension]) / 2.0;

        let mut lower_elements = Vec::new();
        let mut upper_elements = Vec::new();
        for element in elements {
            if element.get_min()[split_dimension] < split_point {
                lower_elements.push(element);
            } else {
                upper_elements.push(element);
            }
        }

		if lower_elements.len() == 0 {
			return Self::from(upper_elements, box_min, box_max);
		}
		if upper_elements.len() == 0 {
			return Self::from(lower_elements, box_min, box_max);
		}

        let left: Box<Interceptable> = if lower_elements.len() < 4 {
            Box::new(PrimitiveStorage { elements: bounded2interceptable(lower_elements)})
        } else {
            Box::new(BVStorage::new(lower_elements))
        };
        let right: Box<Interceptable> = if upper_elements.len() < 4 {
            Box::new(PrimitiveStorage { elements: bounded2interceptable(upper_elements)})
        } else {
            Box::new(BVStorage::new(upper_elements))
        };

        BVStorage {
            min: box_min,
            max: box_max,
            left,
            right,
        }
    }
}

impl Interceptable for BVStorage {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let xspeed = if ray.dir.x == 0.0 {
            f64::MIN_POSITIVE
        } else {
            ray.dir.x
        };
        let yspeed = if ray.dir.y == 0.0 {
            f64::MIN_POSITIVE
        } else {
            ray.dir.y
        };
        let zspeed = if ray.dir.z == 0.0 {
            f64::MIN_POSITIVE
        } else {
            ray.dir.z
        };
        let (txm, tx_m) = if ray.dir.x > 0.0 {
            (
                (self.min.x - ray.start.x) / xspeed,
                (self.max.x - ray.start.x) / xspeed,
            )
        } else {
            (
                (self.max.x - ray.start.x) / xspeed,
                (self.min.x - ray.start.x) / xspeed,
            )
        };

        let (tym, ty_m) = if ray.dir.y > 0.0 {
            (
                (self.min.y - ray.start.y) / yspeed,
                (self.max.y - ray.start.y) / yspeed,
            )
        } else {
            (
                (self.max.y - ray.start.y) / yspeed,
                (self.min.y - ray.start.y) / yspeed,
            )
        };

        let (tzm, tz_m) = if ray.dir.z > 0.0 {
            (
                (self.min.z - ray.start.z) / zspeed,
                (self.max.z - ray.start.z) / zspeed,
            )
        } else {
            (
                (self.max.z - ray.start.z) / zspeed,
                (self.min.z - ray.start.z) / zspeed,
            )
        };

        if txm.max(tym).max(tzm) > tx_m.min(ty_m).min(tz_m) {
            return None;
        }
        return match (self.left.intercept(ray), self.right.intercept(ray)) {
            (None, x) => x,
            (x, None) => x,
            (Some((dist1, int1)), Some((dist2, int2))) => {
                if dist1 < dist2 {
                    Some((dist1, int1))
                } else {
                    Some((dist2, int2))
                }
            }
        };
    }
}

fn pointwise_min_list(vectors: Vec<Vector3<f64>>) -> Vector3<f64> {
    let mut res = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    for vector in vectors {
        res = pointwise_min(res, vector);
    }
    res
}

fn pointwise_max_list(vectors: Vec<Vector3<f64>>) -> Vector3<f64> {
    let mut res = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for vector in vectors {
        res = pointwise_max(res, vector);
    }
    res
}

fn pointwise_min(v1: Vector3<f64>, v2: Vector3<f64>) -> Vector3<f64> {
    let x = v1.x.min(v2.x);
    let y = v1.y.min(v2.y);
    let z = v1.z.min(v2.z);
    Vector3::new(x, y, z)
}

fn pointwise_max(v1: Vector3<f64>, v2: Vector3<f64>) -> Vector3<f64> {
    let x = v1.x.max(v2.x);
    let y = v1.y.max(v2.y);
    let z = v1.z.max(v2.z);
    Vector3::new(x, y, z)
}
