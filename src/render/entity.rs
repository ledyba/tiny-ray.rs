use std::sync::Arc;

pub use sphere::Sphere;

use crate::math::Vec3;
use crate::render::material::Material;
use crate::render::ray::Ray;

mod sphere;
pub mod collection;

pub struct HitRecord {
  pub t: f32,
  pub point: Vec3,
  pub normal: Vec3,
  pub material: Arc<dyn Material + Send + Sync>,
  pub at_front_face: bool,
}

pub trait Entity: Send + Sync {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
