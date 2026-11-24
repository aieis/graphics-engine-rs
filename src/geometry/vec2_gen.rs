use std::fmt;

use std::{ops::{self, Add, Sub, Mul, Div, Neg}};

pub trait Num:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{}

impl<T> Num for T where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
{}

#[allow(nonstandard_style)]
#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct Vec2_Gen<T: Num> {
    pub x: T,
    pub y: T
}

impl <T: Num> Vec2_Gen<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub const fn from_slice(v: [T; 2]) -> Self {
        Self { x: v[0], y: v[1] }
    }


    pub const fn of(v: T) -> Self {
        Self { x: v, y: v }
    }


    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y
    }

    pub fn length(&self) -> T {
        Self::dot(*self, *self).sqrt()
    }

    pub fn norm(v: Self) -> Self {
        let l = v.length();
        let l = if l > 1e-6 { l } else { 1.0 };
        Self { x: v.x / l, y: v.y / l }
    }
}



impl <T: Num> ops::Add<Vec2_Gen<T>> for Vec2_Gen<T> {
    type Output = Vec2_Gen<T>;

    fn add(self, rhs: Vec2_Gen<T>) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y : self.y + rhs.y }
    }
}

impl <T: Num> ops::AddAssign<Vec2_Gen<T>> for Vec2_Gen<T> {

    fn add_assign(&mut self, rhs: Vec2_Gen<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl <T: Num> ops::Sub<Vec2_Gen<T>> for Vec2_Gen<T> {
    type Output = Vec2_Gen<T>;

    fn sub(self, rhs: Vec2_Gen<T>) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y : self.y - rhs.y }
    }
}

impl <T: Num> ops::SubAssign<Vec2_Gen<T>> for Vec2_Gen<T> {
    fn sub_assign(&mut self, rhs: Vec2_Gen<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl <T: Num> ops::Mul<T> for Vec2_Gen<T> {
    type Output = Vec2_Gen<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output { x: self.x * rhs, y : self.y * rhs }
    }
}

impl <T: Num> ops::MulAssign<T> for Vec2_Gen<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl <T: Num> ops::Mul<Vec2_Gen<T>> for T {
    type Output = Vec2_Gen<T>;

    fn mul(self, rhs: Vec2_Gen<T>) -> Self::Output {
        Self::Output { x: self * rhs.x, y : self * rhs.y }
    }
}

impl <T: Num> ops::Neg for Vec2_Gen<T> {
    type Output = Vec2_Gen<T>;

    fn neg(self) -> Self::Output {
        Self::Output { x: -self.x, y: -self.y }
    }
}


impl <T: Num> fmt::Display for Vec2_Gen<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:>6.2}, {:>6.2})", self.x, self.y)
    }
}
