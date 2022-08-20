use crate::img::Image;

pub struct Engine {
  image: Image
}

impl Engine {
  pub fn new(
    image: Image,
  ) -> Self {
    Self {
      image,
    }
  }
  pub fn image(&self) -> &Image {
    &self.image
  }
}