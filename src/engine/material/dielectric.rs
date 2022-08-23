use log::warn;
use palette::LinSrgb;
use crate::engine::hittable::HitRecord;
use crate::engine::material::Response;
use crate::engine::ray::Ray;

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

impl super::Material for Dielectric {
  fn hit(&self, ray: &Ray, hit: &HitRecord) -> Response {
    let dir = ray.direction().normalized();
    let theta = if hit.at_front_face {
       1.0 / self.refractive_index
    } else {
      self.refractive_index
    };
    let horizontal = theta * (dir - (hit.normal * dir) * hit.normal);
    let v = 1.0 - horizontal.length_squared();
    if v >= 0.0 {
      let vertical = -v.sqrt() * hit.normal;
      Response::Scattering {
        scattering: Ray::new(hit.point, horizontal + vertical),
        attenuation: LinSrgb::new(1.0, 1.0, 1.0),
      }
    } else {
      Response::Scattering {
        scattering: Ray::new(hit.point, hit.normal),
        attenuation: LinSrgb::new(1.0, 1.0, 1.0),
      }
    }
  }
}
