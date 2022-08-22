use crate::engine::ray::Ray;
use super::{Hittable, HitRecord};

pub struct HittableCollection {
  targets: Vec<Box<dyn Hittable + Send + Sync>>
}

impl HittableCollection {
  pub fn new() -> Self {
    Self {
      targets: Vec::new(),
    }
  }
  pub fn push(&mut self, target: impl Hittable + Send + Sync + 'static) {
    self.targets.push(Box::new(target));
  }
}

impl Hittable for HittableCollection {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut far = t_max;
    let mut rec = None;
    for target in &self.targets {
      if let Some(r) = target.hit(ray, t_min, far) {
        far = r.t;
        rec = Some(r);
      }
    }
    rec
  }
}
