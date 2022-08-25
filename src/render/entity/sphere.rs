use std::sync::Arc;

use crate::util::math::*;
use crate::render::material::Material;
use crate::render::ray::Ray;

use super::{Entity, HitRecord};

pub struct Sphere {
  center: Vec3,
  radius: f32,
  material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
  pub fn new(
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material + Send + Sync>,
  ) -> Self {
    Self {
      center,
      radius,
      material,
    }
  }
}

impl Entity for Sphere {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let a = ray.direction() * ray.direction();
    let b = 2.0 * ray.direction() * (ray.origin() - self.center);
    let c = (ray.origin() - self.center) * (ray.origin() - self.center) - self.radius * self.radius;
    let discriminant = b * b  - 4.0 * a * c;
    if discriminant < 0.0 {
      return None;
    }
    let ts: [f32; 2] = [
      (-b - discriminant.sqrt()) / (2.0 * a),
      (-b + discriminant.sqrt()) / (2.0 * a),
    ];
    for t in ts {
      if t_min <= t && t <= t_max {
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        let at_front_face = ray.direction() * normal < 0.0;
        return if at_front_face {
          Some(HitRecord {
            t,
            point,
            normal,
            material: Arc::clone(&self.material),
            at_front_face,
          })
        } else {
          Some(HitRecord {
            t,
            point,
            normal: -normal,
            material: Arc::clone(&self.material),
            at_front_face,
          })
        }
      }
    }
    None
  }
}