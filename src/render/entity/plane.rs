use std::sync::Arc;
use crate::render::entity::HitRecord;
use crate::render::entity::volume_tree::BoundingBox;
use crate::render::material::Material;
use crate::render::ray::Ray;
use crate::util::math::Vec3;

pub struct Plane {
  center: Vec3,
  half_width: f32,
  half_depth: f32,
  material: Arc<dyn Material>,
}

impl Plane {
  pub fn new(
    center: Vec3,
    width: f32,
    depth: f32,
    material: Arc<dyn Material>,
  ) -> Self {
    Self {
      center,
      half_width: width / 2.0,
      half_depth: depth / 2.0,
      material,
    }
  }
}

impl super::Entity for Plane {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let t = (self.center.y  - ray.origin().y) / ray.direction().y;
    if !(t_min <= t && t <= t_max) {
      return None;
    }
    let point = ray.at(t);
    let hit =
      self.center.x - self.half_width <= point.x && point.x <= self.center.x + self.half_width &&
        self.center.z - self.half_depth <= point.z && point.z <= self.center.z + self.half_depth;
    if !hit {
      return None;
    }
    let normal = if ray.origin().y < self.center.y {
      Vec3::new(0.0, -1.0, 0.0)
    } else {
      Vec3::new(0.0, 1.0, 0.0)
    };
    Some(HitRecord {
      t,
      point,
      normal,
      material: Arc::clone(&self.material),
      at_front_face: true,
    })
 }

  fn calc_bounding_box(&self) -> Option<BoundingBox> {
    Some(BoundingBox::new(
      Vec3::new(
        self.center.x - self.half_width,
        self.center.y - 0.001,
        self.center.z - self.half_depth,
      ),
      Vec3::new(
        self.center.x + self.half_width,
        self.center.y + 0.001,
        self.center.z + self.half_depth,
      ),
    ))
  }
}
