use crate::util::math::Vec3;

pub struct Quaternion {
  x: f32,
  y: f32,
  z: f32,
  w: f32,
}

impl Quaternion {
  pub fn from_angle_axis(angle: f32, axis: Vec3) -> Self {
    let angle = (angle / 2.0).to_radians();
    let cos = angle.cos();
    let sin = angle.sin();
    let axis = axis.normalized();
    Self {
      x: axis.x * sin,
      y: axis.y * sin,
      z: axis.z * sin,
      w: cos,
    }
  }
  fn from_vec(v: Vec3) -> Self {
    Self {
      x: v.x,
      y: v.y,
      z: v.z,
      w: 0.0,
    }
  }
  pub fn rotate(&self, v: Vec3) -> Vec3 {
    let q = self.mul(Self::from_vec(v)).mul(self.inverse());
    Vec3::new(
      q.x,
      q.y,
      q.z,
    )
  }
  pub fn inverse(&self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: self.w,
    }
  }
  pub fn mul(&self, other: Self) -> Self {
    todo!()
  }
}
