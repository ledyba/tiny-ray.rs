use palette::LinSrgb;

pub use dielectric::Dielectric;
pub use lambert::Lambert;
pub use metal::Metal;

use crate::math::Vec3;
use crate::render::entity::HitRecord;
use crate::render::ray::Ray;

mod lambert;
mod metal;
mod dielectric;

pub enum Response {
  Scattering {
    scattering: Ray,
    attenuation: LinSrgb,
  },
  Absorption,
}

pub trait Material: Send + Sync {
  fn hit(&self, ray: &Ray, hit: &HitRecord) -> Response;
}
