use super::{Vec3, Vec2};

/// Line values are a function of z
#[derive(Copy, Clone)]
pub struct Line {
    /// Initial at z = 0
    pub b: Vec2,

    /// Slope as a function of z
    pub m: Vec2
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "< Line {{ {} + {}z }} >", self.b, self.m)
    }
}



pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl std::fmt::Display for Sphere {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "< Sphere {{ Center: {} Radius: {}}} >", self.center, self.radius)
    }
}


pub fn does_line_intersect_sphere(l: Line, sphere: Sphere) -> bool {

    let b = l.b;
    let m = l.m;

    let r = sphere.radius;
    let s = sphere.center;
    let r_2 = r*r;

    let x = |z| { m.x * z + b.x };
    let y = |z| { m.y * z + b.y };
    let d = |z| { (x(z) - s.x) * (x(z) - s.x) + (y(z) - s.y) * (y(z) - s.y)  + (z - s.z) * (z - s.z) };

    // distance function
    // d(z)   =       (x(z) - s.x)^2     +        (y(z) - s.y)^2  + (z - s.z)^2
    // d'(z)  = 2*m.x*(x(z) - s.x)  +  2*m.y*(y(z) - s.y) + 2*(z - s.z)

    // Solve for d'(z) = 0

    // 0 = 2 * m.x * (m.x*z + b.x - s.x) +
    //     2 * m.y * (m.y*z + b.y - s.y) +
    //     2 * 1   * (1  *z + 0   - s.z)

    // div 2

    // 0 = m.x*m.x*z + m.x*b.x - m.x*s.x +
    //     m.y*m.y*z + m.y*b.y - m.y*s.y +
    //             z           -     s.z

    // 0 = (m.x*m.x + m.y*m.y + 1) * z + ( m.x*b.x - m.x*s.x + m.y*b.y - m.y*s.y - s.z)
    // 0 = (m.x*m.x + m.y*m.y + 1) * z - (-m.x*b.x + m.x*s.x - m.y*b.y + m.y*s.y + s.z)
    // z = (-m.x*b.x + m.x*s.x -m.y*b.y + m.y*s.y + s.z) / (m.x*m.x + m.y*m.y + 1)

    let z = (s.z - (m.x*b.x+m.y*b.y) + (m.x*s.x + m.y*s.y)) / (m.x*m.x + m.y*m.y + 1.0);
    d(z) < r_2
}
