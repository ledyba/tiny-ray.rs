mod ray;
mod entity;
mod material;

use std::sync::Arc;
use palette::{Blend, LinSrgb, Mix};

use entity::{Entity, EntityCollection, Sphere};
use ray::Ray;
use crate::engine::material::Response;
use crate::img::Image;
use crate::math;
use crate::math::Vec3;

pub struct Camera {
  origin: Vec3,
  top_left_corner: Vec3,
  screen_vec_horizontal: Vec3,
  screen_vec_vertical: Vec3,
  x_unit: Vec3,
  y_unit: Vec3,
  z_unit: Vec3,
  lens_radius: f32,
}

impl Camera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    v_up: Vec3,
    v_fov: f32,
    aspect_ratio: (f32, f32),
    aperture: f32,
  ) -> Self {
    let theta = v_fov.to_radians();
    let h = (theta/2.0).tan();
    let screen_height = 2.0 * h;
    let screen_width = aspect_ratio.0 * screen_height / aspect_ratio.1;

    let origin = look_from;

    let gaze = look_at - look_from;
    let focus_dist = gaze.length();
    let z_unit = gaze.normalized();
    let x_unit = v_up.cross(-z_unit).normalized();
    let y_unit = (-z_unit).cross(x_unit); // already normalized

    let screen_vec_horizontal = focus_dist * screen_width * x_unit;
    let screen_vec_vertical = focus_dist * screen_height * y_unit;

    let top_left_corner = origin
      + focus_dist * z_unit
      - (screen_vec_horizontal / 2.0)
      + (screen_vec_vertical / 2.0);

     Self {
       origin,
       top_left_corner,
       screen_vec_horizontal,
       screen_vec_vertical,
       x_unit,
       y_unit,
       z_unit,
       lens_radius: aperture / 2.0,
     }
  }

  pub fn ray_at(&self, u: f32, v: f32) -> Ray {
    let (rx, ry) = math::random_disc(self.lens_radius);
    let depth_offset = self.x_unit * rx + self.y_unit * ry;

    let screen_position =
      self.top_left_corner
        + (self.screen_vec_horizontal * u)
        - (self.screen_vec_vertical * v);
    let origin = self.origin + depth_offset;
    let direction = screen_position - origin;
    Ray::new(
      origin,
      direction,
    )
  }
}

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
