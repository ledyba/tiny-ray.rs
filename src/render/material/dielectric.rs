use palette::LinSrgb;

use crate::render::entity::HitRecord;
use crate::render::material::{Response, ResponseBuilder, Scattering};
use crate::render::ray::Ray;
use crate::util::physics;

pub struct Dielectric {
  refractive_index: f32,
}

impl Dielectric {
  pub fn new(
    refractive_index: f32,
  ) -> Self {
    Self {
      refractive_index,
    }
  }
}

fn schlick(cos: f32, refractive_index: f32) -> f32 {
  let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powf(2.0);
  r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

impl super::Material for Dielectric {
  fn hit(&self, ray: &Ray, hit: &HitRecord) -> Response {
    let dir = ray.direction().normalized();
    let theta = if hit.at_front_face {
       1.0 / self.refractive_index
    } else {
      self.refractive_index
    };
    let cos = f32::min(dir.dot(-hit.normal), 1.0);
    let sin = (1.0 - cos*cos).sqrt();
    if (theta * sin) > 1.0 || rand::random::<f32>() < schlick(cos, theta) {
      let reflect = physics::reflect(ray.direction(), hit.normal);
      Response::builder()
        .scatter(
          Ray::new(hit.point, reflect),
          LinSrgb::new(1.0, 1.0, 1.0),
        )
        .build()
    } else {
      let horizontal = theta * (dir + cos * hit.normal);
      let vertical = -(1.0 - f32::min(horizontal.length_squared(), 1.0)).sqrt() * hit.normal;
      Response::builder()
        .scatter(
          Ray::new(hit.point, horizontal + vertical),
          LinSrgb::new(1.0, 1.0, 1.0),
        )
        .build()
    }
  }
}
