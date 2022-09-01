use std::sync::Arc;
use crate::render::entity::HitRecord;
use crate::render::entity::volume_tree::BoundingBox;
use crate::render::material::Material;
use crate::render::ray::Ray;
use crate::util::math::Vec3;

pub struct Cuboid {
  center: Vec3,
  half_width: f32,
  half_height: f32,
  half_depth: f32,
  material: Arc<dyn Material>,
}

impl Cuboid {
  pub fn new(
    center: Vec3,
    width: f32,
    height: f32,
    depth: f32,
    material: Arc<dyn Material>,
  ) -> Self {
    Self {
      center,
      half_width: width / 2.0,
      half_height: height / 2.0,
      half_depth: depth / 2.0,
      material,
    }
  }
  fn hit_x(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let t1 = ((self.center.x + self.half_width) - ray.origin().x) / ray.direction().x;
    let t0 = ((self.center.x - self.half_width) - ray.origin().x) / ray.direction().x;
    let mut t = f32::NAN;
    if t_min <= t0 && t0 <= t_max {
      t = t0;
    }
    if t_min <= t1 && t1 < t {
      t = t1;
    }
    if t.is_nan() {
      return None;
    }
    let point = ray.at(t);
    let hit =
      self.center.z - self.half_depth <= point.z && point.z <= self.center.z + self.half_depth &&
        self.center.y - self.half_height <= point.y && point.y <= self.center.y + self.half_height;
    if !hit {
      return None;
    }
    let normal = if t == t0 {
      Vec3::new(-1.0, 0.0, 0.0)
    } else {
      Vec3::new(1.0, 0.0, 0.0)
    };
    Some(HitRecord {
      t,
      point,
      normal,
      material: Arc::clone(&self.material),
      at_front_face: true,
    })
  }

  fn hit_y(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let t0 = ((self.center.y - self.half_height) - ray.origin().y) / ray.direction().y;
    let t1 = ((self.center.y + self.half_height) - ray.origin().y) / ray.direction().y;
    let mut t = f32::NAN;
    if t_min <= t0 && t0 <= t_max {
      t = t0;
    }
    if t_min <= t1 && t1 < t {
      t = t1;
    }
    if t.is_nan() {
      return None;
    }
    let point = ray.at(t);
    let hit =
      self.center.x - self.half_width <= point.x && point.x <= self.center.x + self.half_width &&
        self.center.z - self.half_depth <= point.z && point.z <= self.center.z + self.half_depth;
    if !hit {
      return None;
    }
    let normal = if t == t0 {
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

  fn hit_z(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let t0 = ((self.center.z - self.half_depth) - ray.origin().z) / ray.direction().z;
    let t1 = ((self.center.z + self.half_depth) - ray.origin().z) / ray.direction().z;
    let mut t = f32::NAN;
    if t_min <= t0 && t0 <= t_max {
      t = t0;
    }
    if t_min <= t1 && t1 < t {
      t = t1;
    }
    if t.is_nan() {
      return None;
    }
    let point = ray.at(t);
    let hit =
      self.center.x - self.half_width <= point.x && point.x <= self.center.x + self.half_width &&
        self.center.y - self.half_height <= point.y && point.y <= self.center.y + self.half_height;
    if !hit {
      return None;
    }
    let normal = if t == t0 {
      Vec3::new(0.0, 0.0, -1.0)
    } else {
      Vec3::new(0.0, 0.0, 1.0)
    };
    Some(HitRecord {
      t,
      point,
      normal,
      material: Arc::clone(&self.material),
      at_front_face: true,
    })
  }
}

impl super::Entity for Cuboid {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut far = t_max;
    let mut result: Option<HitRecord> = None;
    if let Some(hit) = self.hit_x(ray, t_min, far) {
      far = hit.t;
      result = Some(hit)
    }
    if let Some(hit) = self.hit_y(ray, t_min, far) {
      far = hit.t;
      result = Some(hit)
    }
    if let Some(hit) = self.hit_z(ray, t_min, far) {
      result = Some(hit)
    }
    result
  }

  fn calc_bounding_box(&self) -> Option<BoundingBox> {
    Some(BoundingBox::new(
      Vec3::new(
        self.center.x - self.half_width,
        self.center.y - self.half_height,
        self.center.z - self.half_depth,
      ),
      Vec3::new(
        self.center.x + self.half_width,
        self.center.y + self.half_height,
        self.center.z + self.half_depth,
      ),
    ))
  }
}
