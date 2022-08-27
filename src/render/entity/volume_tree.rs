pub use bounding_box::BoundingBox;

use crate::render::Entity;

mod bounding_box;

pub struct VolumeTree {
  left: Box<VolumeTree>,
  right: Box<VolumeTree>,
}

pub struct VolumeTreeBuilder {
  entities: Vec<Box<dyn Entity>>,
}
