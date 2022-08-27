use crate::render::ray::Ray;
use crate::render::scene::BoundingBox;

use super::{Entity, HitRecord};

pub struct EntityCollection {
  entities: Vec<Box<dyn Entity>>
}

impl EntityCollection {
  pub fn new() -> Self {
    Self {
      entities: Vec::new(),
    }
  }
  pub fn push(&mut self, target: impl Entity + 'static) {
    self.entities.push(Box::new(target));
  }
}

impl Entity for EntityCollection {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut far = t_max;
    let mut result = None;
    for target in &self.entities {
      if let Some(hit) = target.hit(ray, t_min, far) {
        far = hit.t;
        result = Some(hit);
      }
    }
    result
  }

  fn bounding_box(&self) -> Option<BoundingBox> {
    let mut b: Option<BoundingBox> = None;
    for entity in &self.entities {
      let a = entity.bounding_box();
      b = match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(b)) => Some(b.surrounding_with(&a)),
      };
    }
    b
  }
}
