use crate::render::entity::HitRecord;
use crate::render::entity::volume_tree::BoundingBox;
use crate::render::ray::Ray;
use crate::util::math::{Quaternion, Vec3};

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
    let origin = original.min();
    let size = original.max() - original.min();
    let points = vec![
      self.rotate.rotate(origin + Vec3::new(size.x, 0.0, 0.0)),
      self.rotate.rotate(origin + Vec3::new(0.0, size.y, 0.0)),
      self.rotate.rotate(origin + Vec3::new(0.0, 0.0, size.z)),
      self.rotate.rotate(origin + Vec3::new(0.0, size.y, size.z)),
      self.rotate.rotate(origin + Vec3::new(size.x, 0.0, size.z)),
      self.rotate.rotate(origin + Vec3::new(size.x, size.y, 0.0)),
    ];
    let fold = |init, g: &dyn Fn(f32, f32) -> f32| {
      Vec3::new(
        points.iter().map(|it| it.x).fold(init, g),
        points.iter().map(|it| it.y).fold(init, g),
        points.iter().map(|it| it.z).fold(init, g),
      )
    };
    let a = fold(f32::MAX, &f32::min);
    let b = fold(f32::MIN, &f32::max);
    Some(BoundingBox::new(a,b))
  }
}
