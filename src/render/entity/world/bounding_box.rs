use crate::math::Vec3;
use crate::render::ray::Ray;

pub struct BoundingBox {
  min: Vec3,
  max: Vec3,
}

impl BoundingBox {
  pub fn new(
    min: Vec3,
    max: Vec3,
  ) -> Self {
    Self {
      min,
      max,
    }
  }
  pub fn min(&self) -> Vec3 {
    self.min
  }
  pub fn max(&self) -> Vec3 {
    self.max
  }
  pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
    self.hit_naive(ray, t_min, t_max)
  }
  pub fn hit_naive(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
    todo!()
  }
}
