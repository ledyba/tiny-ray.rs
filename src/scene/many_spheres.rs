use std::sync::Arc;

use palette::LinSrgb;

use crate::render::Scene;
use crate::render::entity;
use crate::render::material;
use crate::render::material::Material;
use crate::util::img::Image;
use crate::util::math::Vec3;

pub fn many_spheres(canvas: &Image) -> Scene {
  use rand::prelude::*;
  let seed: [u8; 32] = [11; 32];
  let mut rng: StdRng = rand::SeedableRng::from_seed(seed);

  let mut scene = Scene::new();
  scene.push(entity::Sphere::new(
    Vec3::new(0.0, -100.0, 0.0),
    100.0,
    Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5)))
  ));
  for _ in 0..100 {
    let material: Arc<dyn Material> = match rng.gen::<u8>() % 3 {
      0 => {
        let albedo = LinSrgb::new(
          rng.gen_range(0.2..0.8),
          rng.gen_range(0.2..0.8),
          rng.gen_range(0.2..0.8),
        );
        Arc::new(material::Lambert::new(albedo))
      },
      1 => {
        Arc::new(material::Dielectric::new(1.0 + rng.gen::<f32>() * 2.0))
      },
      _ => {
        let albedo = LinSrgb::new(
          rng.gen_range(0.2..0.8),
          rng.gen_range(0.2..0.8),
          rng.gen_range(0.2..0.8),
        );
        Arc::new(material::Metal::new(albedo, rng.gen::<f32>() * 0.2))
      }
    };

    let x: f32 = rng.gen_range(-30.0..=30.0);

    let z: f32 = rng.gen_range(7.0..60.0);
    let r: f32 = rng.gen_range(0.5..3.5);

    let y: f32 = rng.gen_range(0.0..30.0);
    scene.push(entity::Sphere::new(
        Vec3::new(x, y, z), r,
        material
    ));
  }

  let camera = scene.camera();
  camera
    .look_from(Vec3::new(0.1, 0.5, -3.0))
    .look_at(Vec3::new(0.0, 0.75, -2.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(60.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);

  scene
}
