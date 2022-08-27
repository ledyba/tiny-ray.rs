use crate::util::math::Vec3;

pub fn reflect(dir: Vec3, normal: Vec3) -> Vec3 {
  dir - 2.0 * (dir.dot(normal)) * normal
}
