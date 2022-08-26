use std::sync::Arc;

use palette::{Blend, LinSrgb, Mix};

pub use camera::Camera;
pub use entity::Entity;
use ray::Ray;
pub use world::World;

use crate::util::img::Image;
use crate::util::math::Vec3;

mod ray;
pub mod entity;
pub mod material;
mod camera;
mod world;

pub struct Renderer {
  camera: Camera,
  world: Box<dyn Entity>,
}

impl Renderer {
  pub fn new(
    camera: Camera,
    world: Box<dyn Entity>,
  ) -> Self {
    Self {
      camera,
      world,
    }
  }
  pub fn render(&self, canvas: &mut Image, num_rays: usize) {
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;
    canvas.fill_from(|x, y| {
      let mut sum = LinSrgb::new(0.0, 0.0, 0.0);
      for _ in 0..num_rays {
        let x = (x as f32 + rand::random::<f32>()) / width;
        let y = (y as f32 + rand::random::<f32>()) / height;
        let ray = self.camera.ray_at(x, y);
        sum += self.ray_trace(&ray, 50)
      }
      sum / (num_rays as f32)
    })
  }
  pub fn ray_trace(&self, ray: &Ray, left: usize) -> LinSrgb {
    if left == 0 {
      return LinSrgb::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = self.world.hit(ray, 0.001, f32::MAX) {
      let resp = hit.material.hit(ray, &hit);
      return match resp {
        material::Response::Scattering {
          scattering,
          attenuation,
        } => {
          return self.ray_trace(&scattering, left - 1).multiply(attenuation);
        },
        material::Response::Absorption => {
          LinSrgb::new(0.0, 0.0, 0.0)
        },
      };
    }
    self.sky_box(ray)
  }
  fn sky_box(&self, ray: &Ray) -> LinSrgb {
    let t = 0.5 * (ray.direction().normalized().y + 1.0);
    let blue = LinSrgb::new(0.25, 0.5, 1.0);
    LinSrgb::new(1.0, 1.0, 1.0).mix(&blue, t.sqrt())
  }
}
