use crate::render::camera;
use crate::render::camera::Camera;
use crate::render::entity::Entity;
use crate::render::entity::VolumeTreeBuilder;
use crate::render::sky_box::SkyBox;

pub struct Scene {
  camera: camera::Builder,
  collection: VolumeTreeBuilder,
  sky_box: Option<Box<dyn SkyBox>>,
}

impl Scene {
  pub fn new() -> Self {
    Self {
      camera: Camera::builder(),
      collection: VolumeTreeBuilder::new(),
      sky_box: None,
    }
  }
  pub fn camera(&mut self) -> &mut camera::Builder {
    &mut self.camera
  }
  pub fn push(&mut self, target: impl Entity + 'static) {
    self.collection.push(target);
  }
  pub fn sky_box(&mut self, target: impl SkyBox + 'static) {
    self.sky_box = Some(Box::new(target));
  }
  pub fn build(self) -> (Camera, Box<dyn Entity>, Option<Box<dyn SkyBox>>) {
    (
      self.camera.build(),
      self.collection.build(),
      self.sky_box,
    )
  }
}
