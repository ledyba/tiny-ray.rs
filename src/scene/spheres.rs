use std::sync::Arc;
use palette::LinSrgb;
use crate::render::{Camera, Entity, World};
use crate::render::entity;
use crate::render::material;
use crate::util::img::Image;
use crate::util::math::Vec3;

pub fn spheres(canvas: &Image) -> World {
  let mut world = World::new();
  let lambert = Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5)));
  world.push(
    entity::Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, lambert.clone())
  );
  world.push(
    entity::Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, lambert.clone())
  );
  world.push(
    entity::Sphere::new(
      Vec3::new(-1.2, 0.0, 0.0), 0.5,
      Arc::new(material::Metal::new(LinSrgb::new(0.5, 0.0, 0.0), 0.1)))
  );
  world.push(
    entity::Sphere::new(
      Vec3::new(1.2, 0.0, 0.0), -0.49,
      Arc::new(material::Dielectric::new(1.5)))
  );
  world.push(
    entity::Sphere::new(
      Vec3::new(1.2, 0.0, 0.0), 0.5,
      Arc::new(material::Dielectric::new(1.5)))
  );

  let mut camera = world.camera();
  camera
    .look_from(Vec3::new(3.0, 2.0, 1.0))
    .look_at(Vec3::new(0.0, 0.0, 0.0))
    .v_up(Vec3::new(0.0, 1.0, 0.0))
    .v_fov(45.0)
    .aspect_ratio(canvas.aspect_ratio())
    .aperture(0.0);

  world
}
