use crate::util::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
  x: f32,
  y: f32,
  z: f32,
  w: f32,
}

impl Quaternion {
  pub fn from_angle_axis(angle: f32, axis: Vec3) -> Self {
    let angle = angle.to_radians() / 2.0;
    let cos = angle.cos();
    let sin = angle.sin();
    let r = Self {
      x: axis.x * sin,
      y: axis.y * sin,
      z: axis.z * sin,
      w: cos,
    };
    r.normalized()
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
    let q = (*self) * Self::from_vec(v) * self.inverse();
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
  fn normalized(&self) -> Self {
    let length = (
      self.x * self.x +
        self.y * self.y +
        self.z * self.z +
        self.w * self.w
    ).sqrt();
    Self {
      x: self.x / length,
      y: self.y / length,
      z: self.z / length,
      w: self.w / length,
    }
  }
}

impl std::ops::Mul for Quaternion {
  type Output = Quaternion;
  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      x: (self.x * rhs.w) + (self.y * rhs.z) - (self.z * rhs.y) + (self.w * rhs.x),
      y: -(self.x * rhs.z) + (self.y * rhs.w) + (self.z * rhs.x) + (self.w * rhs.y),
      z: (self.x * rhs.y) - (self.y * rhs.x) + (self.z * rhs.w) + (self.w * rhs.z),
      w: -(self.x * rhs.x) - (self.y * rhs.y) - (self.z * rhs.z) + (self.w * rhs.w),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn ident() {
    let v = Vec3::new(1.0, 0.0, 0.0);
    let q = Quaternion::from_angle_axis(0.0, Vec3::new(0.0, 0.0, 1.0));
    let r = q.rotate(v).normalized();
    assert_eq!(Vec3::new(1.0, 0.0, 0.0), r);
  }

  #[test]
  fn basic_x() {
    let v = Vec3::new(0.0, 0.0, 1.0);

    let q1 = Quaternion::from_angle_axis(90.0, Vec3::new(1.0, 0.0, 0.0));
    let r = q1.rotate(v).normalized(); // FIXME: Float error
    assert_eq!(Vec3::new(0.0, -1.0, 0.0), r);
    let q2 = Quaternion::from_angle_axis(-90.0, Vec3::new(1.0, 0.0, 0.0));
    let r = q2.rotate(v).normalized(); // FIXME: Float error
    assert_eq!(Vec3::new(0.0, 1.0, 0.0), r);
  }

  #[test]
  fn basic_y() {
    let v = Vec3::new(1.0, 0.0, 0.0);

    let q1 = Quaternion::from_angle_axis(90.0, Vec3::new(0.0, 1.0, 0.0));
    let r = q1.rotate(v).normalized(); // FIXME: Float error
    assert_eq!(Vec3::new(0.0, 0.0, -1.0), r);
    let q2 = Quaternion::from_angle_axis(-90.0, Vec3::new(0.0, 1.0, 0.0));
    let r = q2.rotate(v).normalized(); // FIXME: Float error
    assert_eq!(Vec3::new(0.0, 0.0, 1.0), r);
  }

  #[test]
  fn basic_z() {
    let v = Vec3::new(1.0, 0.0, 0.0);

    let q1 = Quaternion::from_angle_axis(90.0, Vec3::new(0.0, 0.0, 1.0));
    let r = q1.rotate(v).normalized(); // FIXME: Float error
    assert_eq!(Vec3::new(0.0, 1.0, 0.0), r);
    let q2 = Quaternion::from_angle_axis(-90.0, Vec3::new(0.0, 0.0, 1.0));
    let r = q2.rotate(v).normalized(); // FIXME: Float error
    assert_eq!(Vec3::new(0.0, -1.0, 0.0), r);
  }

  #[test]
  fn basic_z45() {
    let v = Vec3::new(1.0, 0.0, 0.0);

    let q = Quaternion::from_angle_axis(30.0, Vec3::new(0.0, 0.0, 1.0));
    let r = q.rotate(v).normalized();
    assert_eq!(Vec3::new(0.86602545_f32, 0.50000006_f32, 0.0), r);
    let q = Quaternion::from_angle_axis(-30.0, Vec3::new(0.0, 0.0, 1.0));
    let r = q.rotate(v).normalized();
    assert_eq!(Vec3::new(0.86602545_f32, -0.50000006_f32, 0.0), r);
  }

  #[test]
  fn inverse() {
    let v = Vec3::new(1.0, 0.0, 0.0);
    let q = Quaternion::from_angle_axis(90.0, Vec3::new(0.0, 0.0, 1.0));
    let r = q.rotate(v);
    let r = q.inverse().rotate(r).normalized(); // FIXME: Float error
    assert_eq!(Vec3::new(1.0, 0.0, 0.0), r);
    let q = Quaternion::from_angle_axis(-90.0, Vec3::new(0.0, 0.0, 1.0));
    let r = q.rotate(v);
    let r = q.inverse().rotate(r).normalized(); // FIXME: Float error
    assert_eq!(Vec3::new(1.0, 0.0, 0.0), r);
  }
}
