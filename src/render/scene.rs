use crate::render::camera::{Camera, CameraBuilder};
use crate::render::entity::{Entity, Sphere};
use crate::render::entity::collection::EntityCollection;

mod bounding_box;

pub struct Scene {
  camera: CameraBuilder,
  collection: EntityCollection,
}

impl Scene {
  pub fn new() -> Self {
    Self {
      camera: CameraBuilder::new(),
      collection: EntityCollection::new()
    }
  }
  pub fn camera(&mut self) -> &mut CameraBuilder {
    &mut self.camera
  }
  pub fn push(&mut self, target: impl Entity + 'static) {
    self.collection.push(target);
  }
  pub fn build(self) -> (Camera, Box<dyn Entity>) {
    (
      self.camera.build(),
      Box::new(self.collection),
    )
  }
}
