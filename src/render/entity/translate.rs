use crate::render::entity::HitRecord;
use crate::render::entity::volume_tree::BoundingBox;
use crate::render::ray::Ray;
use crate::util::math::Vec3;

pub struct Translate {
  trans: Vec3,
  entity: Box<dyn super::Entity>,
}

impl Translate {
  pub fn new(
    trans: Vec3,
    entity: impl super::Entity + 'static,
  ) -> Self {
    Self {
      trans,
      entity: Box::new(entity),
    }
  }
}

impl super::Entity for Translate {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let moved_ray = Ray::new(
      ray.origin() - self.trans,
      ray.direction()
    );
    let mut hit = if let Some(hit) = self.entity.hit(&moved_ray, t_min, t_max) {
      hit
    } else {
      return None;
    };
    hit.point += self.trans;
    Some(hit)
  }

  fn calc_bounding_box(&self) -> Option<BoundingBox> {
    let original = if let Some(original) = self.entity.calc_bounding_box() {
      original
    } else {
      return None;
    };
    Some(BoundingBox::new(
      original.min() + self.trans,
      original.max() + self.trans,
    ))
  }
}
