mod sphere;
mod collection;

use std::sync::Arc;
use crate::engine::ray::Ray;
use crate::engine::material::Material;
use crate::math::Vec3;

pub use sphere::Sphere;
pub use collection::HittableCollection;

pub struct HitRecord {
  pub t: f32,
  pub point: Vec3,
  pub normal: Vec3,
  pub material: Arc<dyn Material + Send + Sync>,
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
