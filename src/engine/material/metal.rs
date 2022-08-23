use palette::LinSrgb;
use crate::engine::hittable::HitRecord;
use crate::engine::material::Response;
use crate::engine::ray::Ray;
use crate::math;

pub struct Metal {
  albedo: LinSrgb,
  fuzz: f32,
}

impl Metal {
  pub fn new(
    albedo: LinSrgb,
    fuzz: f32,
  ) -> Self {
    Self {
      albedo,
      fuzz,
    }
  }
}

impl super::Material for Metal {
  fn hit(&self, ray: &Ray, hit: &HitRecord) -> Response {
    let reflect = ray.direction() - 2.0 * (ray.direction() * hit.normal) * hit.normal;
    let scattering = Ray::new(hit.point, reflect + math::random_direction(self.fuzz));
    Response::Reflection {
      scattering,
      attenuation: self.albedo,
    }
  }
}