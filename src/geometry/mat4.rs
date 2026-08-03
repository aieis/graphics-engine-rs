use super::vec4::Vec4;

#[repr(C)]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4
}

impl Mat4 {
    pub const IDENT: Self = Self::new(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    pub const fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self { x, y, z, w }
    }
}
