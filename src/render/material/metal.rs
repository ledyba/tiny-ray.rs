use palette::LinSrgb;

use crate::util::math;
use crate::util::physics;
use crate::render::entity::HitRecord;
use crate::render::material::Response;
use crate::render::ray::Ray;

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
    let reflect = physics::reflect(ray.direction(), hit.normal);
    let scattering = Ray::new(hit.point, reflect + math::random_direction(self.fuzz));
    Response::Scattering {
      scattering,
      attenuation: self.albedo,
    }
  }
}
