use std::sync::Arc;

use palette::LinSrgb;

use crate::render::Scene;
use crate::render::entity;
use crate::render::material;
use crate::util::img::Image;
use crate::util::math::Vec3;

pub fn cornell(canvas: &Image) -> Scene {
  use material::Material;
  let mut scene = Scene::new();

  // http://www.graphics.cornell.edu/online/box/data.html
  // https://github.com/yumcyaWiz/glsl330-cornellbox/blob/b8f38475ba92e86e0340f81bbdf86c35ac2a7d70/src/scene.h
  let white_material:Arc<dyn Material> = Arc::new(material::Lambert::new(LinSrgb::new(0.8, 0.8, 0.8)));
  let green_material:Arc<dyn Material> = Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.8, 0.5)));
  let red_material:Arc<dyn Material> = Arc::new(material::Lambert::new(LinSrgb::new(0.8, 0.5, 0.5)));
  let light_material: Arc<dyn Material> = Arc::new(material::DiffuseLight::new(LinSrgb::new(34.0, 19.0, 10.0)));
  let glass_material: Arc<dyn Material> = Arc::new(material::Dielectric::new(1.5));

  // floor
  scene.push(entity::Cuboid::new(
    Vec3::new(
      278.0,
      -0.5,
      559.2 / 2.0,
    ),
    556.0,
    1.0,
    559.2,
    Arc::clone(&white_material),
  ));
  // ceiling
  scene.push(entity::Cuboid::new(
    Vec3::new(
      278.0,
      548.8 + 0.5,
      559.2 / 2.0,
    ),
    556.0,
    1.0,
    559.2,
    Arc::clone(&white_material),
  ));
  // back
  scene.push(entity::Cuboid::new(
    Vec3::new(
      278.0,
      273.0,
      559.2 + 0.5,
    ),
    278.0 * 2.0 + 2.0,
    273.0 * 2.0 + 2.0,
    1.0,
    Arc::clone(&white_material),
  ));
  // right
  scene.push(entity::Cuboid::new(
    Vec3::new(
      556.0+0.5,
      548.8/2.0,
      559.2/2.0,
    ),
    1.0,
    548.8,
    559.2,
    Arc::clone(&red_material),
  ));
  // left
  scene.push(entity::Cuboid::new(
    Vec3::new(
      -0.5,
      548.8/2.0,
      559.2/2.0,
    ),
    1.0,
    548.8,
    559.2,
    Arc::clone(&green_material),
  ));
  // light
  scene.push(entity::Cuboid::new(
    Vec3::new(
      (343.0 + 213.0)/2.0,
      548.8,
      (332.0 + 227.0)/2.0,
    ),
    343.0 - 213.0,
    0.1,
    332.0 - 227.0,
    Arc::clone(&light_material),
  ));

  // Items

  scene.push(entity::Translate::new(
    Vec3::new(
      400.0,
      (400.0 / 2.0),
      (559.2 / 2.0) + 70.0,
    ),
    entity::Cuboid::new(
      Vec3::zero(),
      180.0,
      400.0,
      80.0,
      Arc::clone(&white_material),
    )));

  scene.push(entity::Translate::new(
    Vec3::new(
      200.0,
      (150.0 / 2.0),
      (559.2 / 2.0) - 100.0,
    ),
    entity::Cuboid::new(
      Vec3::zero(),
      150.0,
      150.0,
      150.0,
      Arc::clone(&glass_material),
    ),
  ));

  let camera = scene.camera();
  camera
    .look_from(Vec3::new(278.0,273.0, -760.0))
    .look_at(Vec3::new(278.0, 273.0, 0.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(40.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);

  scene
}
