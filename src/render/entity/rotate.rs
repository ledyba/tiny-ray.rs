use crate::render::entity::HitRecord;
use crate::render::entity::volume_tree::BoundingBox;
use crate::render::ray::Ray;
use crate::util::math::Quaternion;

pub struct Rotate {
  rotate: Quaternion,
  entity: Box<dyn super::Entity>,
}

impl Rotate {
  pub fn new(
    rotate: Quaternion,
    entity: impl super::Entity + 'static,
  ) -> Self {
    Self {
      rotate,
      entity: Box::new(entity),
    }
  }
}

impl super::Entity for Rotate {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let inv = self.rotate.inverse();
    let rotated_ray = Ray::new(
      inv.rotate(ray.origin()),
      inv.rotate(ray.direction()),
    );
    let mut hit = if let Some(hit) = self.entity.hit(&rotated_ray, t_min, t_max) {
      hit
    } else {
      return None;
    };
    hit.point = self.rotate.rotate(hit.point);
    hit.normal = self.rotate.rotate(hit.normal);
    Some(hit)
  }

  fn calc_bounding_box(&self) -> Option<BoundingBox> {
    let original = if let Some(original) = self.entity.calc_bounding_box() {
      original
    } else {
      return None;
    };
    Some(BoundingBox::new(
      self.rotate.rotate(original.min()),
      self.rotate.rotate(original.max()),
    ))
  }
}
