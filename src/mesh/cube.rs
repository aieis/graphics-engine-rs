use crate::geometry::vec3::Vec3;

use super::{mesh::Mesh, prism::make_prism};


pub fn make_cube(x: f32, y: f32, z: f32, length: f32, col: [f32; 3]) -> Mesh
{
    return make_prism(Vec3::new(x, y, z), Vec3::new(length, length, length,), Vec3::from_slice(col));
}
