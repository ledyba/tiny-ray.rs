use palette::LinSrgb;

use crate::math;
use crate::render::entity::HitRecord;
use crate::render::material::Response;
use crate::render::ray::Ray;

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
    return Response::Scattering {
      scattering: Ray::new(hit.point, direction),
      attenuation: self.albedo,
    };
  }
}
