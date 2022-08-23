use palette::LinSrgb;
use crate::engine::hittable::HitRecord;
use crate::engine::material::Response;
use crate::engine::ray::Ray;
use crate::math;

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
    return Response::Reflection {
      scattering: Ray::new(hit.point, direction),
      attenuation: self.albedo,
    };
  }
}
