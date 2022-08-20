use crate::img::Image;

pub struct Engine {
  canvas: Image
}

impl Engine {
  pub fn new(
    canvas: Image,
  ) -> Self {
    Self {
      canvas,
    }
  }
  pub fn canvas(&self) -> &Image {
    &self.image
  }
}