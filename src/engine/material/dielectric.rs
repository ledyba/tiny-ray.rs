use crate::engine::hittable::HitRecord;
use crate::engine::material::Response;
use crate::engine::ray::Ray;

pub struct Dielectric {
  refractive_index: f32,
}

impl Dielectric {
  pub fn new(
    refractive_index: f32,
  ) -> Self {
    Self {
      refractive_index,
    }
  }
}

impl super::Material for Dielectric {
  fn hit(&self, ray: &Ray, hit: &HitRecord) -> Response {
    Response::Absorption
  }
}
