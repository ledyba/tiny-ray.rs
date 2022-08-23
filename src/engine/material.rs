mod lambert;
mod metal;

use palette::LinSrgb;
use crate::engine::hittable::HitRecord;
use crate::engine::ray::Ray;
use crate::math::Vec3;

pub use lambert::Lambert;
pub use metal::Metal;

pub enum Response {
  Reflection {
    scattering: Ray,
    attenuation: LinSrgb,
  },
  Absorption,
}

pub trait Material: Send + Sync {
  fn hit(&self, ray: &Ray, hit: &HitRecord) -> Response;
}
