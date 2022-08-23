mod ray;
mod hittable;
mod material;

use palette::{LinSrgb, Mix, Shade, Srgb};
use crate::engine::hittable::{Hittable, HittableCollection, Sphere};
use crate::engine::ray::Ray;
use crate::img::Image;
use crate::math;
use crate::math::Vec3;

pub struct Engine {
  origin: Vec3,
  screen_distance: f32,
  screen_height: f32,
  world: HittableCollection,
}

impl Engine {
  pub fn new(
  ) -> Self {
    let mut world = HittableCollection::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    Self {
      origin: Vec3::zero(),
      screen_distance: 1.0,
      screen_height: 2.0,
      world,
    }
  }
  pub fn render(&self, canvas: &mut Image) {
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
      let mut sum = LinSrgb::new(0.0, 0.0, 0.0);
      for _ in 0..10 {
        let x = (x as f32 + rand::random::<f32>()) / width;
        let y = (y as f32 + rand::random::<f32>()) / height;
        let ray = Ray::new(
          self.origin,
          top_left + (w * x) - (h * y)
        );
        sum += self.ray_trace(&ray, 50)
      }
      sum / 10.0
    })
  }
  pub fn ray_trace(&self, ray: &Ray, left: usize) -> LinSrgb {
    if left == 0 {
      return LinSrgb::new(0.0, 0.0, 0.0);
    }
    if let Some(r) = self.world.hit(ray, 0.001, f32::MAX) {
      let target = r.point + r.normal + math::random_direction(1.0);
      return self.ray_trace(&Ray::new(r.point, target - r.point), left - 1).darken(0.5);
    }
    self.sky_box(ray)
  }
  fn sky_box(&self, ray: &Ray) -> LinSrgb {
    let t = 0.5 * (ray.direction().normalized().y + 1.0);
    let blue = LinSrgb::new(0.25, 0.5, 1.0);
    LinSrgb::new(1.0, 1.0, 1.0).mix(&blue, t.sqrt())
  }
}
