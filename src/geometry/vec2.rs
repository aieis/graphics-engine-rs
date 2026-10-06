use std::{ops, fmt};

#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub const ZERO: Self = Self::of(0.0);

    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn from_slice(v: [f32; 2]) -> Self {
        Self { x: v[0], y: v[1] }
    }


    pub const fn of(v: f32) -> Self {
        Self { x: v, y: v }
    }


    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y
    }

    pub fn length(&self) -> f32 {
        Self::dot(*self, *self).sqrt()
    }

    pub fn norm(v: Self) -> Self {
        let l = v.length();
        let l = if l > 1e-6 { l } else { 1.0 };
        Self { x: v.x / l, y: v.y / l }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y : self.y + rhs.y }
    }
}

impl ops::AddAssign<Vec2> for Vec2 {

    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y : self.y - rhs.y }
    }
}

impl ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x * rhs, y : self.y * rhs }
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output { x: self * rhs.x, y : self * rhs.y }
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Self::Output { x: -self.x, y: -self.y }
    }
}


impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:>6.2}, {:>6.2})", self.x, self.y)
    }
}
