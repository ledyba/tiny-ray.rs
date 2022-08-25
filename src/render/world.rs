use crate::render::entity::{Entity, Sphere};
use crate::render::entity::collection::EntityCollection;

mod bounding_box;

pub struct WorldBuilder {
  collection: EntityCollection,
}

impl WorldBuilder {
  pub fn new() -> Self {
    Self {
      collection: EntityCollection::new()
    }
  }
  pub fn push(&mut self, target: impl Entity + 'static) {
    self.collection.push(target);
  }
  pub fn build(self) -> Box<dyn Entity> {
    Box::new(self.collection)
  }
}
