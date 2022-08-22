use super::{HitRecord, Hittable};
use crate::engine::ray::Ray;
use crate::math::*;

pub struct Sphere {
  center: Vec3,
  radius: f32,
}

impl Sphere {
  pub fn new(
    center: Vec3,
    radius: f32,
  ) -> Self {
    Self {
      center,
      radius,
    }
  }
}

impl Hittable for Sphere {
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
        let normal = ray.at(t) - Vec3::new(0.0, 0.0, -1.0);
        return Some(HitRecord {
          t,
          point,
          normal,
        })
      }
    }
    None
  }
}