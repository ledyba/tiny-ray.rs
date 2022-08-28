use std::sync::Arc;

pub use sphere::Sphere;

use crate::render::entity::volume_tree::BoundingBox;
use crate::render::material::Material;
use crate::render::ray::Ray;
use crate::util::math::Vec3;

mod sphere;
pub mod collection;
pub mod volume_tree;

pub struct HitRecord {
  pub t: f32,
  pub point: Vec3,
  pub normal: Vec3,
  pub material: Arc<dyn Material>,
  pub at_front_face: bool,
}

pub trait Entity: Send + Sync {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
  fn calc_bounding_box(&self) -> Option<BoundingBox>;
}
