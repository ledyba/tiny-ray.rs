use palette::LinSrgb;
use crate::engine::entity::HitRecord;
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
    let reflect = super::reflect(ray.direction(), hit.normal);
    let scattering = Ray::new(hit.point, reflect + math::random_direction(self.fuzz));
    Response::Scattering {
      scattering,
      attenuation: self.albedo,
    }
  }
}