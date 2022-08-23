use palette::LinSrgb;
use crate::engine::hittable::HitRecord;
use crate::engine::ray::Ray;
use crate::math::Vec3;

pub enum Response {
  Reflection {
    scattering: Vec3,
    attenuation: LinSrgb,
  },
  Absorption,
}

pub trait Material {
  fn hit(ray: &Ray, hit: HitRecord) -> Response;
}
