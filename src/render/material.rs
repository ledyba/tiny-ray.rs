use palette::LinSrgb;

pub use dielectric::Dielectric;
pub use lambert::Lambert;
pub use metal::Metal;

use crate::render::entity::HitRecord;
use crate::render::ray::Ray;

mod lambert;
mod metal;
mod dielectric;

pub struct Response {
  scattering: Option<Scattering>,
  emission: Option<LinSrgb>,
}

impl Response {
  pub fn builder() -> builder::Builder {
    builder::Builder::new()
  }
  pub fn scattering(&self) -> Option<&Scattering> {
    self.scattering.as_ref()
  }
  pub fn emission(&self) -> Option<LinSrgb> {
    self.emission
  }
}

mod builder {
  use super::*;

  pub struct Builder {
    scattering: Option<Scattering>,
    emission: Option<LinSrgb>,
  }

  impl Builder {
    pub fn new() -> Self {
      Self {
        scattering: None,
        emission: None,
      }
    }
    pub fn scatter(
      mut self,
      direction: Ray,
      attenuation: LinSrgb,
    ) -> Self {
      self.scattering = Some(Scattering {
        direction,
        attenuation
      });
      self
    }
    pub fn emit(
      mut self,
      emission: LinSrgb,
    ) -> Self {
      self.emission = Some(emission);
      self
    }
    pub fn build(self) -> Response {
      Response {
        scattering: self.scattering,
        emission: self.emission,
      }
    }
  }
}

pub struct Scattering {
  direction: Ray,
  attenuation: LinSrgb,
}

impl Scattering {
  pub fn direction(&self) -> &Ray {
    &self.direction
  }
  pub fn attenuation(&self) -> LinSrgb {
    self.attenuation
  }
}

pub trait Material: Send + Sync {
  fn hit(&self, ray: &Ray, hit: &HitRecord) -> Response;
}
