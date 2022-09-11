use palette::{Blend, LinSrgb};
use crate::util::img::Image;

pub use camera::Camera;
pub use entity::Entity;
use ray::Ray;
pub use scene::Scene;
pub use sky_box::SkyBox;

mod ray;
pub mod entity;
pub mod material;
mod camera;
mod scene;
pub mod sky_box;

pub struct Renderer {
  camera: Camera,
  world: Box<dyn Entity>,
  sky_box: Option<Box<dyn SkyBox>>,
}

impl Renderer {
  pub fn new(
    scene: Scene,
  ) -> Self {
    let (camera, world, sky_box) = scene.build();
    Self {
      camera,
      world,
      sky_box,
    }
  }
  pub fn render(&self, canvas: &mut Image, num_rays: usize, num_reflections: usize) {
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;
    canvas.fill_by(|x, y| {
      let mut sum = LinSrgb::new(0.0, 0.0, 0.0);
      for _ in 0..num_rays {
        let x = (x as f32 + rand::random::<f32>()) / width;
        let y = (y as f32 + rand::random::<f32>()) / height;
        let ray = self.camera.ray_at(x, y);
        sum += self.trace_ray(&ray, num_reflections)
      }
      sum / (num_rays as f32)
    })
  }
  pub fn trace_ray(&self, ray: &Ray, left_num_reflections: usize) -> LinSrgb {
    if left_num_reflections == 0 {
      return LinSrgb::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = self.world.hit(ray, 0.001, f32::MAX) {
      let resp = hit.material.hit(ray, &hit);
      let mut color = LinSrgb::new(0.0, 0.0, 0.0);
      if let Some(emission) = resp.emission() {
        color += emission;
      }
      if let Some(scattering) = resp.scattering() {
        color += self
          .trace_ray(scattering.direction(), left_num_reflections - 1)
          .multiply(scattering.attenuation());
      }
      return color;
    }
    if let Some(sky_box) = &self.sky_box {
      sky_box.from_ray(ray)
    } else {
      LinSrgb::new(0.0, 0.0, 0.0)
    }
  }
}
