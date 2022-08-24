mod ray;
mod hittable;
mod material;

use std::sync::Arc;
use palette::{Blend, LinSrgb, Mix};

use hittable::{Hittable, HittableCollection, Sphere};
use ray::Ray;
use crate::engine::material::Response;
use crate::img::Image;
use crate::math::Vec3;

pub struct Camera {
  origin: Vec3,
  top_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
}

impl Camera {
  pub fn new(
    look_from: Vec3,
    look_to: Vec3,
    v_up: Vec3,
    v_fov: f32,
    aspect_ratio: (f32, f32),
  ) -> Self {
    let theta = v_fov.to_radians();
    let focal_length = 10.0;
    let h = focal_length * (theta/2.0).tan();
    let screen_height = h * 2.0;
    let screen_width = aspect_ratio.0 * screen_height / aspect_ratio.1;

    let origin = look_from;

    let w = (look_from - look_to).normalized();
    let u = Vec3::cross(&v_up, w).normalized();
    let v = Vec3::cross(&w, u); // already normalized

    let horizontal = screen_width * u;
    let vertical = screen_height * v;

    let top_left_corner = origin
      - Vec3::new(0.0, 0.0, focal_length)
      - (horizontal / 2.0)
      + (vertical / 2.0)
      - w;

     Self {
       origin,
       top_left_corner,
       horizontal,
       vertical,
     }
  }

  pub fn ray_at(&self, u: f32, v: f32) -> Ray {
    let screen_position = self.top_left_corner + (self.horizontal * u) - (self.vertical * v);
    let direction = screen_position - self.origin;
    Ray::new(
      self.origin,
      direction,
    )
  }
}

pub struct Engine {
  camera: Camera,
  world: HittableCollection,
}

impl Engine {
  pub fn new(
    camera: Camera,
  ) -> Self {
    let mut world = HittableCollection::new();
    let lambert = Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5)));
    world.push(
      Sphere::new(Vec3::new(0.0, 0.0, -10.0), 0.5, lambert.clone())
    );
    world.push(
      Sphere::new(Vec3::new(0.0, -100.5, -10.0), 100.0, lambert.clone())
    );
    world.push(
      Sphere::new(
        Vec3::new(-1.2, 0.0, -10.0), 0.5,
        Arc::new(material::Metal::new(LinSrgb::new(0.5, 0.0, 0.0), 0.1)))
    );
    world.push(
      Sphere::new(
        Vec3::new(1.2, 0.0, -10.0), 0.5,
        Arc::new(material::Dielectric::new(1.5)))
    );
    Self {
      camera,
      world,
    }
  }
  pub fn render(&self, canvas: &mut Image) {
    const NUM_RAYS: usize = 30;
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;
    canvas.fill_from(|x, y| {
      let mut sum = LinSrgb::new(0.0, 0.0, 0.0);
      for _ in 0..NUM_RAYS {
        let x = (x as f32 + rand::random::<f32>()) / width;
        let y = (y as f32 + rand::random::<f32>()) / height;
        let ray = self.camera.ray_at(x, y);
        sum += self.ray_trace(&ray, 50)
      }
      sum / (NUM_RAYS as f32)
    })
  }
  pub fn ray_trace(&self, ray: &Ray, left: usize) -> LinSrgb {
    if left == 0 {
      return LinSrgb::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = self.world.hit(ray, 0.001, f32::MAX) {
      let resp = hit.material.hit(ray, &hit);
      return match resp {
        Response::Scattering { scattering, attenuation } => {
          return self.ray_trace(&scattering, left - 1).multiply(attenuation);
        },
        Response::Absorption => LinSrgb::new(0.0, 0.0, 0.0),
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
