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
  fn hit(&self, shape_ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let shape_hit = if let Some(hit) = self.shape.hit(shape_ray, t_min, t_max) {
      hit
    } else {
      return None;
    };
    let material = Arc::clone(&self.material);
    let density = material.density;
    if shape_hit.at_front_face {
      let ray = Ray::new(shape_hit.point, shape_ray.direction());
      let mut hit = if let Some(hit) = self.shape.hit(&ray, 0.001, f32::INFINITY) {
        if hit.at_front_face {
          return None;
        } else {
          hit
        }
      } else {
        return None;
      };
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
    } else {
      let mut hit = shape_hit;
      let distance = -rand::random::<f32>().log(std::f32::consts::E) / density;
      if hit.t < distance {
        return None;
      }
      hit.t = distance;
      hit.normal = util::math::random_direction(1.0);
      hit.point = shape_ray.at(distance);
      hit.at_front_face = true;
      hit.material = material;
      Some(hit)
    }
  }

  fn calc_bounding_box(&self) -> Option<BoundingBox> {
    self.shape.calc_bounding_box()
  }
}
