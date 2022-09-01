mod blue_sky;
pub use blue_sky::BlueSky;

use palette::LinSrgb;
use super::Ray;

pub trait SkyBox: Send + Sync {
  fn from_ray(&self, ray: &Ray) -> LinSrgb;
}
