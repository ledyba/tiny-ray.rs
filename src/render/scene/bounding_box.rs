use crate::util::math::Vec3;
use crate::render::ray::Ray;

pub struct BoundingBox {
  min: Vec3,
  max: Vec3,
}

impl BoundingBox {
  pub fn new(
    a: Vec3,
    b: Vec3,
  ) -> Self {
    Self {
      min: Vec3::new(
        f32::min(a.x, b.x),
        f32::min(a.y, b.y),
        f32::min(a.z, b.z),
      ),
      max: Vec3::new(
        f32::max(a.x, b.x),
        f32::max(a.y, b.y),
        f32::max(a.z, b.z),
      ),
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
  pub fn surrounding_with(&self, b: &Self) -> Self {
    Self {
      min: Vec3::new(
        f32::min(self.min.x, b.min.x),
        f32::min(self.min.y, b.min.y),
        f32::min(self.min.z, b.min.z),
      ),
      max: Vec3::new(
        f32::max(self.max.x, b.max.x),
        f32::max(self.max.y, b.max.y),
        f32::max(self.max.z, b.max.z),
      ),
    }
  }
}
