use std::sync::Arc;
use palette::LinSrgb;
use crate::render::Scene;
use crate::util::img::Image;
use crate::util::math::Vec3;

pub fn fog(canvas: &Image) -> Scene {
  use crate::render::entity;
  use crate::render::material;
  use material::Material;
  let mut scene = Scene::new();
  let white_material: Arc<dyn Material> = Arc::new(material::Lambert::new(LinSrgb::new(0.8, 0.8, 0.8)));
  let metal_material: Arc<dyn Material> = Arc::new(material::Metal::new(LinSrgb::new(1.0, 0.3, 0.3), 0.0));
  let emission_material: Arc<dyn Material> = Arc::new(material::DiffuseLight::new(LinSrgb::new(0.99 * 30.0, 0.84 * 30.0, 0.0 * 30.0)));

  scene.push(entity::Plane::new(Vec3::zero(), 1000.0, 1000.0, Arc::clone(&white_material)));
  scene.push(
    entity::Sphere::new(
      Vec3::new(-3.0, 1.0, -3.0),
      1.0,
      Arc::clone(&metal_material),
    ));
  scene.push(entity::Cuboid::new(Vec3::new(0.0, 0.5, 0.0), 1.0, 1.0, 1.0, Arc::clone(&white_material)));
  scene.push(
    entity::Volume::new(
      entity::Cuboid::new(
        Vec3::new(0.0, 50.0, 0.0),
        100.0, 100.0, 100.0,
        Arc::clone(&white_material)),
      0.5,
      LinSrgb::new(0.8, 0.8, 0.8),
    ));
  scene.push(
    entity::Sphere::new(
      Vec3::new(1.0, 3.0, -2.5),
      0.5,
      Arc::clone(&emission_material),
    ));
  scene.camera()
    .look_from(Vec3::new(3.0, 2.0, 4.0))
    .look_at(Vec3::new(0.0, 1.0, 0.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(45.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);
  scene
}