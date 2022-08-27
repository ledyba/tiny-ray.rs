pub use bounding_box::BoundingBox;

use crate::render::Entity;

mod bounding_box;
mod builder;
pub use builder::VolumeTreeBuilder;
use crate::render::entity::HitRecord;
use crate::render::ray::Ray;

pub struct VolumeTree {
  left: Box<dyn Entity>,
  right: Box<dyn Entity>,
  bounding_box: BoundingBox,
}

impl VolumeTree {
  pub fn new(
    left: Box<dyn Entity>,
    right: Box<dyn Entity>,
    bounding_box: BoundingBox,
  ) -> Self {
    Self {
      left,
      right,
      bounding_box,
    }
  }
}

impl Entity for VolumeTree {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    if !self.bounding_box.hit(ray, t_min, t_max) {
      return None;
    }
    let mut result = None;
    let mut far = t_max;
    if let Some(hit) = self.left.hit(ray, t_min, far) {
      far = hit.t;
      result = Some(hit);
    }
    if let Some(hit) = self.right.hit(ray, t_min, far) {
      far = hit.t;
      result = Some(hit);
    }
    result
  }

  fn bounding_box(&self) -> Option<BoundingBox> {
    Some(self.bounding_box.clone())
  }
}
