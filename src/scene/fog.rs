use std::sync::Arc;
use palette::LinSrgb;
use crate::render::{entity, Scene};
use crate::util::img::Image;
use crate::util::math::Vec3;

pub fn fog(canvas: &Image) -> Scene {
  use crate::render::sky_box;
  use crate::render::entity;
  use crate::render::material;
  use material::Material;;
  let mut scene = Scene::new();
  let white_material: Arc<dyn Material> = Arc::new(material::Lambert::new(LinSrgb::new(0.8, 0.8, 0.8)));
  let emission_material: Arc<dyn Material> = Arc::new(material::DiffuseLight::new(LinSrgb::new(1.0 * 25.0, 0.65 * 25.0, 0.0 * 25.0)));

  scene.sky_box(sky_box::BlueSky::new());
  scene.push(entity::Plane::new(Vec3::zero(), 1000.0, 1000.0, Arc::clone(&white_material)));
  //scene.push(entity::Cuboid::new(Vec3::new(0.0, 0.5, 0.0), 1.0, 1.0, 1.0, Arc::clone(&white_material)));
  scene.push(
    entity::Volume::new(
      entity::Cuboid::new(
        Vec3::new(0.0, 0.5, 0.0),
        1.0, 1.0, 1.0,
        Arc::clone(&white_material)),
      200.0,
      LinSrgb::new(0.8, 0.8, 0.8),
    )
  );
  //scene.push(entity::Sphere::new(Vec3::new(1.5, 3.0, -1.0), 0.5, Arc::clone(&emission_material)));
  scene.camera()
    .look_from(Vec3::new(3.0, 2.0, 4.0))
    .look_at(Vec3::new(0.0, 1.0, 0.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(45.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);
  scene
}