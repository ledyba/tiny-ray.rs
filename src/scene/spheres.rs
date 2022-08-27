use std::sync::Arc;

use palette::LinSrgb;

use crate::render::{Camera, Entity, Scene};
use crate::render::entity;
use crate::render::material;
use crate::util::img::Image;
use crate::util::math::Vec3;

pub fn spheres(canvas: &Image) -> Scene {
  let mut scene = Scene::new();
  let lambert = Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5)));
  scene.push(
    entity::Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, lambert.clone())
  );
  scene.push(
    entity::Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, lambert.clone())
  );
  scene.push(
    entity::Sphere::new(
      Vec3::new(-1.2, 0.0, 0.0), 0.5,
      Arc::new(material::Metal::new(LinSrgb::new(0.5, 0.0, 0.0), 0.1)))
  );
  scene.push(
    entity::Sphere::new(
      Vec3::new(1.2, 0.0, 0.0), -0.49,
      Arc::new(material::Dielectric::new(1.5)))
  );
  scene.push(
    entity::Sphere::new(
      Vec3::new(1.2, 0.0, 0.0), 0.5,
      Arc::new(material::Dielectric::new(1.5)))
  );

  let mut camera = scene.camera();
  camera
    .look_from(Vec3::new(3.0, 2.0, 1.0))
    .look_at(Vec3::new(0.0, 0.0, 0.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(45.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);

  scene
}
