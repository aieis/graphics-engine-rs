use super::vec4::Vec4;

#[repr(C, align(16))]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4
}

impl Mat4 {
    #[allow(unused)]
    pub const IDENT: Self = Self::new(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    /// This is in row-column order so the parameters are rows
    pub const fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self { x, y, z, w }
    }


    /// `v` is a column vector here
    pub fn mul_vec(&self, v: Vec4) -> Vec4 {
        Vec4::new(
            v.x * self.x.x + v.y * self.x.y + v.z * self.x.z + v.w * self.x.w,
            v.x * self.y.x + v.y * self.y.y + v.z * self.y.z + v.w * self.y.w,
            v.x * self.z.x + v.y * self.z.y + v.z * self.z.z + v.w * self.z.w,
            v.x * self.w.x + v.y * self.w.y + v.z * self.w.z + v.w * self.w.w,
        )
    }

    pub const fn transpose(m: Mat4) -> Mat4 {
        Mat4::new(
            Vec4::new(m.x.x, m.y.x, m.z.x, m.w.x),
            Vec4::new(m.x.y, m.y.y, m.z.y, m.w.y),
            Vec4::new(m.x.z, m.y.z, m.z.z, m.w.z),
            Vec4::new(m.x.w, m.y.w, m.z.w, m.w.w),
        )
    }
}
