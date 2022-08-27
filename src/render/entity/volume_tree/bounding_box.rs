use crate::render::ray::Ray;
use crate::util::math::Vec3;

#[derive(Debug, Clone)]
pub struct BoundingBox {
  min: Vec3,
  max: Vec3,
}

impl BoundingBox {
  pub fn new(
    a: Vec3,
    b: Vec3,
  ) -> Self {
    Self {
      min: Vec3::new(
        f32::min(a.x, b.x),
        f32::min(a.y, b.y),
        f32::min(a.z, b.z),
      ),
      max: Vec3::new(
        f32::max(a.x, b.x),
        f32::max(a.y, b.y),
        f32::max(a.z, b.z),
      ),
    }
  }
  pub fn min(&self) -> Vec3 {
    self.min
  }
  pub fn max(&self) -> Vec3 {
    self.max
  }
  pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
    self.hit_naive(ray, t_min, t_max)
  }
  pub fn hit_naive(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
    fn range(min: f32, max: f32, r_origin: f32, r_weight: f32) -> (f32, f32) {
      let t0 = (min - r_origin) / r_weight;
      let t1 = (max - r_origin) / r_weight;
      (f32::min(t0, t1), f32::max(t0, t1))
    }
    fn calc_overlap((t0, t1): (f32, f32), (u0, u1): (f32, f32)) -> (f32, f32) {
      (f32::max(t0, u0), f32::min(t1, u1))
    }
    let (t_min, t_max) = calc_overlap(
      (t_min, t_max),
      range(
        self.min.x, self.max.x,
        ray.origin().x, ray.direction().x,
      )
    );
    if t_max < t_min {
      return false;
    }
    let (t_min, t_max) = calc_overlap(
      (t_min, t_max),
      range(
        self.min.y, self.max.y,
        ray.origin().y, ray.direction().y,
      )
    );
    if t_max < t_min {
      return false;
    }
    let (t_min, t_max) = calc_overlap(
      (t_min, t_max),
      range(
        self.min.z, self.max.z,
        ray.origin().z, ray.direction().z,
      )
    );
    t_min <= t_max
  }
  pub fn surrounding_with(&self, b: &Self) -> Self {
    Self {
      min: Vec3::new(
        f32::min(self.min.x, b.min.x),
        f32::min(self.min.y, b.min.y),
        f32::min(self.min.z, b.min.z),
      ),
      max: Vec3::new(
        f32::max(self.max.x, b.max.x),
        f32::max(self.max.y, b.max.y),
        f32::max(self.max.z, b.max.z),
      ),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn basic() {
    let b = BoundingBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::zero());
    let ray = Ray::new(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(false, b.hit_naive(&ray, 0.0, 1.0));
    let ray = Ray::new(Vec3::zero(), Vec3::new(1.0, 1.0, 0.0));
    assert_eq!(false, b.hit_naive(&ray, 0.0, 1.0));
    let ray = Ray::new(Vec3::zero(), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(true, b.hit_naive(&ray, 0.0, 1.0));
    let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(true, b.hit_naive(&ray, 0.0, 1.0));
    let ray = Ray::new(Vec3::new(0.5, 0.5, 0.5), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(true, b.hit_naive(&ray, 0.0, 1.0));
    let ray = Ray::new(Vec3::new(0.5, 0.5, 0.5), -Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(true, b.hit_naive(&ray, 0.0, 1.0));
    let ray = Ray::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(false, b.hit_naive(&ray, 0.0, 1.0));
  }

  #[test]
  fn surrounding() {
    let b = BoundingBox::new(Vec3::zero(), Vec3::zero());
    assert_eq!(Vec3::zero(), b.min);
    assert_eq!(Vec3::zero(), b.max);
    let b = b.surrounding_with(&BoundingBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::zero()));
    assert_eq!(Vec3::zero(), b.min);
    assert_eq!(Vec3::new(1.0, 1.0, 1.0), b.max);
  }
}
