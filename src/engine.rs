mod ray;
mod entity;
mod material;
mod camera;

use std::sync::Arc;
use palette::{Blend, LinSrgb, Mix};

use crate::img::Image;
use crate::math::Vec3;

use entity::{Entity, EntityCollection, Sphere};
use ray::Ray;
pub use camera::Camera;

pub struct Engine {
  camera: Camera,
  world: EntityCollection,
}

impl Engine {
  pub fn new(
    camera: Camera,
  ) -> Self {
    let mut world = EntityCollection::new();
    let lambert = Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5)));
    world.push(
      Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, lambert.clone())
    );
    world.push(
      Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, lambert.clone())
    );
    world.push(
      Sphere::new(
        Vec3::new(-1.2, 0.0, 0.0), 0.5,
        Arc::new(material::Metal::new(LinSrgb::new(0.5, 0.0, 0.0), 0.1)))
    );
    world.push(
      Sphere::new(
        Vec3::new(1.2, 0.0, 0.0), -0.49,
        Arc::new(material::Dielectric::new(1.5)))
    );
    world.push(
      Sphere::new(
        Vec3::new(1.2, 0.0, 0.0), 0.5,
        Arc::new(material::Dielectric::new(1.5)))
    );
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
