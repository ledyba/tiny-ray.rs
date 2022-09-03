use std::sync::Arc;

use palette::LinSrgb;

use crate::render::Scene;
use crate::render::entity;
use crate::render::material;
use crate::render::material::Material;
use crate::util::img::Image;
use crate::util::math;
use crate::util::math::{Quaternion, Vec3};

pub fn many_boxes(canvas: &Image) -> Scene {
  use rand::prelude::*;
  let seed: [u8; 32] = [0; 32];
  let mut rng: StdRng = rand::SeedableRng::from_seed(seed);

  let mut scene = Scene::new();
  for _ in 0..100 {
    let material: Arc<dyn material::Material> = match rng.gen::<u8>() % 4 {
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
      2 => {
        let color = LinSrgb::new(1.0 + rng.gen::<f32>() * 8.0, 1.0 + rng.gen::<f32>() * 8.0, 1.0 + rng.gen::<f32>() * 8.0);
        Arc::new(material::DiffuseLight::new(color))
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

    let x: f32 = rng.gen_range(-10.0..=10.0);
    let y: f32 = rng.gen_range(-10.0..=10.0);

    let z: f32 = rng.gen_range(0.0..20.0);
    let size = 1.0;
    let angle = rng.gen_range(0.0..360.0);
    let axis = math::random_direction(1.0);
    scene.push(entity::Translate::new(
        Vec3::new(x, y, z),
        entity::Rotate::new(
          Quaternion::from_angle_axis(angle, axis),
          entity::Cuboid::new(
            Vec3::zero(),
            size,
            size,
            size,
            material,
          ))));
  }

  let wall_material: Arc<dyn Material> = Arc::new(material::Metal::new(LinSrgb::new(0.8, 0.8, 0.8), 0.1));
  scene.push(entity::Cuboid::new(
    Vec3::new(-11.5, 0.0, 25.0),
    1.0,
    40.0,
    80.0,
    Arc::clone(&wall_material)));
  scene.push(entity::Cuboid::new(
    Vec3::new(11.5, 0.0, 25.0),
    1.0,
    40.0,
    80.0,
    Arc::clone(&wall_material)));
  scene.push(entity::Cuboid::new(
    Vec3::new(0.0, -11.5, 25.0),
    40.0,
    1.0,
    80.0,
    Arc::clone(&wall_material)));
  scene.push(entity::Cuboid::new(
    Vec3::new(0.0, 11.5, 25.0),
    40.0,
    1.0,
    80.0,
    Arc::clone(&wall_material)));

  let camera = scene.camera();
  camera
    .look_from(Vec3::new(0.0, 0.0, -3.0))
    .look_at(Vec3::new(0.0, 0.0, 0.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(60.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);

  scene
}
