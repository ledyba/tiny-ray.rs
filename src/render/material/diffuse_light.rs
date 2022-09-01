use palette::LinSrgb;
use crate::render::entity::HitRecord;
use crate::render::material::Response;
use crate::render::ray::Ray;

pub struct DiffuseLight {
  color: LinSrgb,
}

impl DiffuseLight {
  pub fn new(color: LinSrgb) -> Self {
    Self {
      color,
    }
  }
}

impl super::Material for DiffuseLight {
  fn hit(&self, _ray: &Ray, _hit: &HitRecord) -> Response {
    Response::builder()
      .emit(self.color)
      .build()
  }
}
