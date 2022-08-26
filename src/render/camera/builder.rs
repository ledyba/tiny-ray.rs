use crate::render::Camera;
use crate::util::math::Vec3;

pub struct CameraBuilder {
  look_from: Vec3,
  look_at: Vec3,
  v_up: Vec3,
  v_fov: f32,
  aspect_ratio: (f32, f32),
  aperture: f32,
}

impl CameraBuilder {
  pub fn new() -> Self {
    Self {
      look_from: Vec3::new(0.0, 0.0, 1.0),
      look_at: Vec3::new(0.0, 0.0, 0.0),
      v_up: Vec3::new(0.0, 1.0, 0.0),
      v_fov: 45.0,
      aspect_ratio: (1.0, 1.0),
      aperture: 0.0,
    }
  }
  pub fn look_from(&mut self, v: Vec3) -> &mut Self {
    self.look_from = v;
    self
  }
  pub fn look_at(&mut self, v: Vec3) -> &mut Self {
    self.look_at = v;
    self
  }
  pub fn v_up(&mut self, v: Vec3) -> &mut Self {
    self.v_up = v;
    self
  }
  pub fn v_fov(&mut self, v: f32) -> &mut Self {
    self.v_fov = v;
    self
  }
  pub fn aspect_ratio(&mut self, v: (f32, f32)) -> &mut Self {
    self.aspect_ratio = v;
    self
  }
  pub fn aperture(&mut self, v: f32) -> &mut Self {
    self.aperture = v;
    self
  }
  pub fn build(&self) -> Camera {
    Camera::new(
      self.look_from,
      self.look_at,
      self.v_up,
      self.v_fov,
      self.aspect_ratio,
      self.aperture,
    )
  }
}
