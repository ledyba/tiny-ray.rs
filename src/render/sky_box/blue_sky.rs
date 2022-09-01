use palette::{LinSrgb, Mix};
use crate::render::ray::Ray;
use super::SkyBox;

pub struct BlueSky;

impl BlueSky {
  pub fn new() -> Self {
    Self {}
  }
}

impl SkyBox for BlueSky {
  fn from_ray(&self, ray: &Ray) -> LinSrgb {
    let t = 0.5 * (ray.direction().normalized().y + 1.0);
    let blue = LinSrgb::new(0.25, 0.5, 1.0);
    LinSrgb::new(1.0, 1.0, 1.0).mix(&blue, t.sqrt())
  }
}