use crate::render::camera::{Camera, CameraBuilder};
use crate::render::entity::Entity;
use crate::render::entity::volume_tree::VolumeTreeBuilder;

pub struct Scene {
  camera: CameraBuilder,
  collection: VolumeTreeBuilder,
}

impl Scene {
  pub fn new() -> Self {
    Self {
      camera: CameraBuilder::new(),
      collection: VolumeTreeBuilder::new()
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
      self.collection.build(),
    )
  }
}
