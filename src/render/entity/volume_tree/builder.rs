use crate::render::Entity;

pub struct VolumeTreeBuilder {
  entities: Vec<Box<dyn Entity>>,
}
