use crate::render::Entity;
use crate::render::entity::collection::EntityCollection;
use crate::render::entity::volume_tree::{BoundingBox, VolumeTree, VolumeTreeLeaf};

pub struct VolumeTreeBuilder {
  entities: Vec<(Box<dyn Entity>, Option<BoundingBox>)>,
  bounding_box: Option<BoundingBox>,
}

impl VolumeTreeBuilder {
  pub fn new() -> Self {
    Self {
      entities: Vec::new(),
      bounding_box: None,
    }
  }
  fn from(entities: Vec<(Box<dyn Entity>, Option<BoundingBox>)>,) -> Self {
    let bounding_box = entities
      .iter()
      .map(|it| it.1.clone())
      .fold(None, BoundingBox::sum);
    Self {
      entities,
      bounding_box,
    }
  }
  pub fn push(&mut self, entity: impl Entity + 'static) {
    let bounding_box = entity.calc_bounding_box();
    self.bounding_box = BoundingBox::sum(self.bounding_box.clone(), bounding_box.clone());
    self.entities.push((Box::new(entity), bounding_box));
  }
  pub fn build(mut self) -> Box<dyn Entity> {
    if self.entities.is_empty() {
      return Box::new(EntityCollection::new());
    }
    if self.entities.len() == 1 {
      let (e, b) = self.entities.remove(0);
      let b = b.expect("No bounding box");
      return Box::new(VolumeTreeLeaf::new(e, b));
    }
    if self.entities.len() == 2 {
      let (e1, b1) = self.entities.remove(0);
      let (e2, b2) = self.entities.remove(0);
      let b1 = b1.expect("No bounding box");
      let b2 = b2.expect("No bounding box");
      let b = b1.clone().surrounding_with(&b2);
      return Box::new(VolumeTree::new(
        Box::new(VolumeTreeLeaf::new(e1, b1)),
        Box::new(VolumeTreeLeaf::new(e2, b2)),
        b
      ));
    }
    match rand::random::<usize>() % 3 {
      0 => {
        self.entities.sort_by(|(_, b1), (_, b2)| {
          let b1 = b1.as_ref().expect("No bounding box");
          let b2 = b2.as_ref().expect("No bounding box");
          b1.center().x.partial_cmp(&b2.center().x).expect("Invalid bounding box")
        });
      },
      1 => {
        self.entities.sort_by(|(_, b1), (_, b2)| {
          let b1 = b1.as_ref().expect("No bounding box");
          let b2 = b2.as_ref().expect("No bounding box");
          b1.center().y.partial_cmp(&b2.center().y).expect("Invalid bounding box")
        });
      },
      _ => {
        self.entities.sort_by(|(_, b1), (_, b2)| {
          let b1 = b1.as_ref().expect("No bounding box");
          let b2 = b2.as_ref().expect("No bounding box");
          b1.center().z.partial_cmp(&b2.center().z).expect("Invalid bounding box")
        });
      }
    }
    let mut left = self.entities;
    let right = left.split_off(left.len() / 2);
    let left = Self::from(left).build();
    let right = Self::from(right).build();
    let bounding_box = self.bounding_box.expect("Invalid bounding box");
    Box::new(VolumeTree::new(left, right, bounding_box))
  }
}