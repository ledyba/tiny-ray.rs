use std::sync::Arc;
use palette::LinSrgb;
use crate::render::{Entity, World};
use crate::render::entity;
use crate::render::material;
use crate::util::math::Vec3;

pub fn spheres() -> Box<dyn Entity> {
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
  world.build()
}