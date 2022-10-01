use palette::LinSrgb;
use crate::render::Entity;
use crate::render::entity::HitRecord;
use crate::render::entity::volume_tree::BoundingBox;
use crate::render::ray::Ray;

pub struct Volume {
  shape: Box<dyn Entity>,
  albedo: LinSrgb,
  density: f32,
}

impl Volume {
  pub fn new(
    shape: impl Entity + 'static,
    albedo: LinSrgb,
    density: f32,
  ) -> Self {
    Self {
      shape: Box::new(shape),
      albedo,
      density,
    }
  }
}

impl Entity for Volume {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let hit = if let Some(hit) = self.shape.hit(ray, t_min, t_max) {
      hit
    } else {
      return None;
    };

    None
  }

  fn calc_bounding_box(&self) -> Option<BoundingBox> {
    self.shape.calc_bounding_box()
  }
}
