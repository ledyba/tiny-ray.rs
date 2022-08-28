use palette::LinSrgb;

use crate::render::entity::HitRecord;
use crate::render::material::{Response, Scattering};
use crate::render::ray::Ray;
use crate::util::math;

pub struct Lambert {
  albedo: LinSrgb,
}

impl Lambert {
  pub fn new(albedo: LinSrgb) -> Self {
    Self {
      albedo,
    }
  }
}

impl super::Material for Lambert {
  fn hit(&self, _ray: &Ray, hit: &HitRecord) -> Response {
    let direction = hit.normal + math::random_direction(1.0);
    Response::builder()
      .scatter(
        Ray::new(hit.point, direction),
        self.albedo,
      )
      .build()
  }
}
