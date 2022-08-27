mod bounding_box;
pub use bounding_box::BoundingBox;
use crate::render::Entity;

pub struct VolumeTree {
  left: Box<VolumeTree>,
  right: Box<VolumeTree>,
}

pub struct VolumeTreeBuilder {
  entities: Vec<Box<dyn Entity>>,
}
