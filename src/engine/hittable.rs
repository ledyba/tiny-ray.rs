mod sphere;
mod collection;

use crate::engine::ray::Ray;
use crate::math::Vec3;

pub use sphere::Sphere;
pub use collection::HittableCollection;

pub struct HitRecord {
  pub t: f32,
  pub point: Vec3,
  pub normal: Vec3,
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
