pub use builder::CameraBuilder;

use crate::util::math;
use crate::util::math::Vec3;

use super::ray::Ray;

mod builder;

pub struct Camera {
  origin: Vec3,
  top_left_corner: Vec3,
  screen_vec_horizontal: Vec3,
  screen_vec_vertical: Vec3,
  x_unit: Vec3,
  y_unit: Vec3,
  z_unit: Vec3,
  lens_radius: f32,
}

impl Camera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    v_up: Vec3,
    v_fov: f32,
    aspect_ratio: (f32, f32),
    aperture: f32,
  ) -> Self {
    let theta = v_fov.to_radians();
    let screen_height = 2.0 * (theta/2.0).tan();
    let screen_width = aspect_ratio.0 * screen_height / aspect_ratio.1;

    let origin = look_from;

    let gaze = look_at - look_from;
    let focus_dist = gaze.length();
    let z_unit = gaze.normalized();
    let x_unit = v_up.cross(-z_unit).normalized();
    let y_unit = (-z_unit).cross(x_unit); // already normalized

    let screen_vec_horizontal = focus_dist * screen_width * x_unit;
    let screen_vec_vertical = focus_dist * screen_height * y_unit;

    let top_left_corner = origin
      + focus_dist * z_unit
      - (screen_vec_horizontal / 2.0)
      + (screen_vec_vertical / 2.0);

    Self {
      origin,
      top_left_corner,
      screen_vec_horizontal,
      screen_vec_vertical,
      x_unit,
      y_unit,
      z_unit,
      lens_radius: aperture / 2.0,
    }
  }

  pub fn ray_at(&self, nx: f32, ny: f32) -> Ray {
    let depth_offset = if self.lens_radius <= 0.0 {
      Vec3::zero()
    } else {
      let (rx, ry) = math::random_disc(self.lens_radius);
      (self.x_unit * rx + self.y_unit * ry) * rand::random::<f32>()
    };

    let screen_position =
      self.top_left_corner
        + (self.screen_vec_horizontal * nx)
        - (self.screen_vec_vertical * ny);
    let origin = self.origin + depth_offset;
    let direction = screen_position - origin;
    Ray::new(
      origin,
      direction,
    )
  }
}
