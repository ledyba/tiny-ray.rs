mod sphere;
mod collection;

use std::sync::Arc;
use crate::render::ray::Ray;
use crate::render::material::Material;
use crate::math::Vec3;

pub use sphere::Sphere;
pub use collection::EntityCollection;

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
