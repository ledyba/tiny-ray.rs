use std::sync::Arc;
use palette::LinSrgb;
use crate::render::Entity;
use crate::render::entity::HitRecord;
use crate::render::entity::volume_tree::BoundingBox;
use crate::render::material::IsotropicVolume;
use crate::render::ray::Ray;
use crate::util;

pub struct Volume {
  shape: Box<dyn Entity>,
  material: Arc<IsotropicVolume>,
}

impl Volume {
  pub fn new(
    shape: impl Entity + 'static,
    density: f32,
    color: LinSrgb,
  ) -> Self {
    Self {
      shape: Box::new(shape),
      material: Arc::new(IsotropicVolume::new(density, color)),
    }
  }
}

impl Entity for Volume {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let hit = if let Some(hit) = self.shape.hit(ray, t_min, t_max) {
      hit
    } else {
      return None;
    };
    let material = Arc::clone(&self.material);
    let density = material.density;
    if hit.at_front_face {
      let ray_internal = Ray::new(hit.point, ray.direction());
      let mut hit_internal = if let Some(hit) = self.shape.hit(&ray_internal, 0.001, f32::INFINITY) {
        if hit.at_front_face {
          return None;
        } else {
          hit
        }
      } else {
        return None;
      };
      let distance = -rand::random::<f32>().log(std::f32::consts::E) / density;
      if hit_internal.t < distance {
        return None;
      }
      hit_internal.t = distance;
      hit_internal.normal = util::math::random_direction(1.0);
      hit_internal.point = ray_internal.at(distance);
      hit_internal.at_front_face = true;
      hit_internal.material = material;
      Some(hit_internal)
    } else {
      let mut hit = hit;
      let distance = -rand::random::<f32>().log(std::f32::consts::E) / density;
      if hit.t < distance {
        return None;
      }
      hit.t = distance;
      hit.normal = util::math::random_direction(1.0);
      hit.point = ray.at(distance);
      hit.at_front_face = true;
      hit.material = material;
      Some(hit)
    }
  }

  fn calc_bounding_box(&self) -> Option<BoundingBox> {
    self.shape.calc_bounding_box()
  }
}
