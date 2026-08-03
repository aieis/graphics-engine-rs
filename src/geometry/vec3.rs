use std::{ops, fmt};

#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::of(0.0);

    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn from_slice(v: [f32; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }


    pub const fn of(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }


    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Self {
            x: a.y*b.z - a.z*b.y,
            y: a.z*b.x - a.x*b.z,
            z: a.x*b.y - a.y*b.x
        }
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn norm(v: &Vec3) -> Vec3 {
        let l = v.length();
        let l = if l > 1e-6 { l } else { 1.0 };
        Self { x: v.x / l, y: v.y / l, z: v.z / l}
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y : self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {

    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}


impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y : self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x * rhs, y : self.y * rhs, z: self.z * rhs }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output { x: self * rhs.x, y : self * rhs.y, z: self * rhs.z }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:>6.2}, {:>6.2}, {:>6.2})", self.x, self.y, self.z)
    }
}
