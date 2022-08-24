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
    let cos = (dir * -hit.normal);
    let sin = theta * (1.0 - cos*cos).sqrt();
    if sin > 1.0 || rand::random::<f32>() < schlick(cos, theta) {
      let reflect = super::reflect(ray.direction(), hit.normal);
      Response::Scattering {
        scattering: Ray::new(hit.point, reflect),
        attenuation: LinSrgb::new(1.0, 1.0, 1.0),
      }
    } else {
      let horizontal = theta * (dir + cos * hit.normal);
      let vertical = -(1.0 - horizontal.length_squared()).sqrt() * hit.normal;
      Response::Scattering {
        scattering: Ray::new(hit.point, horizontal + vertical),
        attenuation: LinSrgb::new(1.0, 1.0, 1.0),
      }
    }
  }
}
