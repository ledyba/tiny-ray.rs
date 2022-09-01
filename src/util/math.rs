mod quaternion;
mod vec3;

pub use vec3::Vec3;
pub use quaternion::Quaternion;

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
