use palette::LinSrgb;
use crate::render::entity::HitRecord;
use crate::render::material::Response;
use crate::render::ray::Ray;
use crate::util::math;

pub struct IsotropicVolume {
  pub density: f32,
  pub color: LinSrgb,
}

impl IsotropicVolume {
  pub fn new(
    density: f32,
    color: LinSrgb,
  ) -> Self {
    Self {
      density,
      color,
    }
  }
}

impl super::Material for IsotropicVolume {
  fn hit(&self, _ray: &Ray, hit: &HitRecord) -> Response {
    Response::builder()
      .scatter(Ray::new(hit.point, math::random_direction(1.0)), self.color)
      .build()
  }
}
