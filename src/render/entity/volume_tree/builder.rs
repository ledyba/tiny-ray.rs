use crate::render::Entity;
use crate::render::entity::collection::EntityCollection;

pub struct VolumeTreeBuilder {
  entities: Vec<Box<dyn Entity>>,
}

impl VolumeTreeBuilder {
  pub fn new() -> Self {
    Self {
      entities: Vec::new(),
    }
  }
  pub fn push(&mut self, entity: impl Entity + 'static) {
    self.entities.push(Box::new(entity));
  }
  pub fn build(mut self) -> Box<dyn Entity> {
    if self.entities.is_empty() {
      return Box::new(EntityCollection::new());
    }
    if self.entities.len() == 1 {
      return self.entities.remove(0);
    }
    todo!()
  }
}