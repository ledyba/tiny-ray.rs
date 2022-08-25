use crate::util::math::Vec3;

pub struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Ray {
  pub fn new(
    origin: Vec3,
    direction: Vec3,
  ) -> Self {
    Self {
      origin,
      direction,
    }
  }
  pub fn origin(&self) -> Vec3 {
    self.origin
  }
  pub fn direction(&self) -> Vec3 {
    self.direction
  }
  pub fn at(&self, t: f32) -> Vec3 {
    self.origin + (t * self.direction)
  }
}
