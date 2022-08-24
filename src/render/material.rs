mod lambert;
mod metal;
mod dielectric;

use palette::LinSrgb;
use crate::render::entity::HitRecord;
use crate::render::ray::Ray;
use crate::math::Vec3;

pub use lambert::Lambert;
pub use metal::Metal;
pub use dielectric::Dielectric;

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

fn reflect(dir: Vec3, normal: Vec3) -> Vec3 {
  dir - 2.0 * (dir * normal) * normal
}