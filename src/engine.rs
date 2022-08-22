mod ray;

use log::info;
use palette::{LinSrgb, Mix, Srgb};
use crate::engine::ray::Ray;
use crate::img::Image;
use crate::math::Vec3;

pub struct Engine {
  origin: Vec3,
  screen_distance: f32,
  screen_height: f32,
}

pub trait Hittable {
  fn hit(ray: &Ray, t_min: f32, t_max: f32) -> bool;
}

impl Engine {
  pub fn new(
  ) -> Self {
    Self {
      origin: Vec3::zero(),
      screen_distance: 1.0,
      screen_height: 2.0,
    }
  }
  pub fn render(&mut self, canvas: &mut Image) {
    let top_left = Vec3::new(
      -(self.screen_height * canvas.width() as f32 / canvas.height() as f32)/2.0,
      self.screen_height / 2.0,
      -self.screen_distance,
    );
    let w = Vec3::new(self.screen_height * canvas.width() as f32 / canvas.height() as f32, 0.0, 0.0);
    let h = Vec3::new(0.0, self.screen_height, 0.0);
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;
    canvas.fill_from(|x, y| {
      let x = ((x as f32) + 0.5) / width;
      let y = ((y as f32) + 0.5) / height;
      let ray = Ray::new(
        self.origin,
        top_left + (w * x) - (h * y)
      );
      self.calc(&ray)
    })
  }
  pub fn calc(&self, ray: &Ray) -> LinSrgb {
    let t = self.hit_sphere(ray, Vec3::new(0.0, 0.0, -1.0), 0.5);
    if !t.is_nan() {
      let n = ray.at(t) - Vec3::new(0.0, 0.0, -1.0);
      return LinSrgb::new(
        (n.x + 1.0) / 2.0,
        (n.y + 1.0) / 2.0,
        (n.z + 1.0) / 2.0,
      );
    }
    self.sky(ray)
  }
  fn hit_sphere(&self, ray: &Ray, center: Vec3, radius: f32) -> f32 {
    let a = ray.direction() * ray.direction();
    let b = 2.0 * ray.direction() * (ray.origin() - center);
    let c = (ray.origin() - center) * (ray.origin() - center) - radius * radius;
    let discriminant = b * b  - 4.0 * a * c;
    if discriminant < 0.0 {
      f32::NAN
    } else {
      (-b - discriminant.sqrt()) / (2.0 * a)
    }
  }
  fn sky(&self, ray: &Ray) -> LinSrgb {
    let t = 0.5 * (ray.direction().normalized().y + 1.0);
    let blue = LinSrgb::new(0.0, 0.2, 1.0);
    LinSrgb::new(1.0, 1.0, 1.0).mix(&blue, t.sqrt())
  }
}
