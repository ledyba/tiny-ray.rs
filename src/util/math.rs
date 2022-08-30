mod quaternion;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self {
      x,
      y,
      z,
    }
  }
  pub fn zero() -> Self {
    Self::new(0.0, 0.0, 0.0)
  }
  pub fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }
  pub fn length_squared(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }
  pub fn normalized(&self) -> Self {
    *self / self.length()
  }
  pub fn dot(&self, other: Self) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
  pub fn cross(&self, other: Self) -> Self {
    Self::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x,
    )
  }
}

impl std::ops::Add for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Self) -> Self::Output {
    Vec3::new(
      self.x + rhs.x,
      self.y + rhs.y,
      self.z + rhs.z,
    )
  }
}

impl std::ops::AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}

impl std::ops::Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Self) -> Self::Output {
    Vec3::new(
      self.x - rhs.x,
      self.y - rhs.y,
      self.z - rhs.z,
    )
  }
}

impl std::ops::SubAssign for Vec3 {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

impl std::ops::Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self {
    Vec3 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl std::ops::Mul<f32> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f32) -> Self::Output {
    Vec3::new(
      self.x * rhs,
      self.y * rhs,
      self.z * rhs,
    )
  }
}

impl std::ops::MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, rhs: f32) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
  }
}

impl std::ops::Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
    Vec3::new(
      self * rhs.x,
      self * rhs.y,
      self * rhs.z,
    )
  }
}

impl std::ops::Div<f32> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f32) -> Self::Output {
    Vec3::new(
      self.x / rhs,
      self.y / rhs,
      self.z / rhs,
    )
  }
}

impl std::ops::DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, rhs: f32) {
    self.x /= rhs;
    self.y /= rhs;
    self.z /= rhs;
  }
}

pub fn random_direction(len: f32) -> Vec3 {
  let z = f32::min((rand::random::<f32>() - 0.5) * 2.0, 1.0);
  let r = (1.0 - z * z).sqrt();
  let theta = (rand::random::<f32>() * 360.0).to_radians();
  let cos = theta.cos();
  let sin = theta.sin();
  let n = Vec3::new(
    r * cos,
    r * sin,
    z,
  );
  len * n
}

pub fn random_disc(r: f32) -> (f32, f32) {
  let theta = (rand::random::<f32>() * 360.0).to_radians();
  let cos = theta.cos();
  let sin = theta.sin();
  (r * cos, r * sin)
}
