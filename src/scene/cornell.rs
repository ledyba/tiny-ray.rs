use std::sync::Arc;

use palette::LinSrgb;

use crate::render::Scene;
use crate::render::entity;
use crate::render::material;
use crate::render::sky_box;
use crate::util::img::Image;
use crate::util::math::Vec3;

pub fn cornell(canvas: &Image) -> Scene {
  let mut scene = Scene::new();
  scene.push(entity::Cuboid::new(
    Vec3::new(0.0, 0.0, 0.0),
    1.0,
    1.0,
    1.0,
    Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5))),
  ));

  scene.push(entity::Cuboid::new(
    Vec3::new(0.0, 0.0, -1.0),
    10.0,
    10.0,
    1.0,
    Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5))),
  ));

  scene.push(entity::Cuboid::new(
    Vec3::new(0.0, 0.0, 3.0),
    1.0,
    1.0,
    1.0,
    Arc::new(material::DiffuseLight::new(LinSrgb::new(6.0, 6.0, 6.0))),
  ));

  let camera = scene.camera();
  camera
    .look_from(Vec3::new(2.0, 2.0, 2.0))
    .look_at(Vec3::new(0.0, 0.0, 0.0))
    .v_up(Vec3::new(0.0, 0.0, 1.0))
    .v_fov(45.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);

  scene
}
