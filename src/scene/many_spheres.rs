use std::sync::Arc;

use palette::LinSrgb;

use crate::render::{Camera, Entity, Scene};
use crate::render::entity;
use crate::render::material;
use crate::render::material::Material;
use crate::util::img::Image;
use crate::util::math::Vec3;

pub fn many_spheres(canvas: &Image) -> Scene {
  let mut scene = Scene::new();
  scene.push(entity::Sphere::new(
    Vec3::new(0.0, -100.5, 0.0),
    100.0,
    Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5)))
  ));
  for _ in 0..3 {
    let material: Arc<dyn Material> = match rand::random::<usize>() % 3 {
      0 => {
        let albedo = LinSrgb::new(
          rand::random::<f32>(),
          rand::random::<f32>(),
          rand::random::<f32>(),
        );
        Arc::new(material::Lambert::new(albedo))
      },
      1 => {
        Arc::new(material::Dielectric::new(1.0 + rand::random::<f32>() * 2.0))
      },
      _ => {
        let albedo = LinSrgb::new(
          rand::random::<f32>(),
          rand::random::<f32>(),
          rand::random::<f32>(),
        );
        Arc::new(material::Metal::new(albedo, rand::random::<f32>() * 0.3))
      }
    };

    let x = (rand::random::<f32>() - 0.5) * 60.0;

    let z = 3.0 + rand::random::<f32>() * 30.0;
    let r = z + rand::random::<f32>() * 10.0;

    let y = r + rand::random::<f32>() * 20.0;
    scene.push(entity::Sphere::new(
        Vec3::new(x, y, z), r,
        material
    ));
  }

  let mut camera = scene.camera();
  camera
    .look_from(Vec3::new(0.0, 0.0, 1.0))
    .look_at(Vec3::new(0.0, 0.0, 0.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(60.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);

  scene
}
