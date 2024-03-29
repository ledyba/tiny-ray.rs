use std::sync::Arc;

use palette::LinSrgb;

use crate::render::Scene;
use crate::render::entity;
use crate::render::material;
use crate::render::sky_box;
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
        let color = LinSrgb::new(
          1.0 + rng.gen::<f32>() * 20.0,
          1.0 + rng.gen::<f32>() * 20.0,
          1.0 + rng.gen::<f32>() * 20.0
        );
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

    let r: f32 = rng.gen_range(3.0..=10.0);
    let angle: f32 = rng.gen_range(0.0f32..=360.0f32).to_radians();
    let x: f32 = r * angle.sin();
    let y: f32 = r * angle.cos();

    let z: f32 = rng.gen_range(100.0..130.0);
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

  let wall_material: Arc<dyn material::Material> = Arc::new(material::Metal::new(LinSrgb::new(0.8, 0.8, 0.8), 0.0));
  let axis = Vec3::new(0.0, 0.0, 1.0);
  for i in 0..6 {
    scene.push(entity::Rotate::new(
      Quaternion::from_angle_axis((60 * i) as f32, axis),
      entity::Plane::new(
        Vec3::new(0.0, 12.0, 400.0),
        200.0,
        800.0,
        Arc::clone(&wall_material))));
  }

  scene.sky_box(sky_box::BlueSky::new());

  scene.camera()
    .look_from(Vec3::new(0.0, 0.0, -3.0))
    .look_at(Vec3::new(0.0, 0.0, 0.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(30.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);

  scene
}
