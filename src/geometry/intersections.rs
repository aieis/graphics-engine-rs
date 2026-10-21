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

pub struct Plane {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3
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

pub fn does_line_intersect_plane(l: Line, plane: Plane) -> bool {
    // The formula we are solving here
    // t*d - a*u - b*v = v0 - o

    // Important tricks here
    // u*(cross(u,v)) = 0 where * is dot
    // u*(cross(v,w)) = v*(cross(w,u))
    // cross(u,v) = -cross(v,u)


    let o  = Vec3 { x: l.b.x, y: l.b.y, z: 0.0 };
    let d  = Vec3 { x: l.m.x, y: l.m.y, z: 1.0 };
    let v0 = plane.v0;
    let u  = plane.v1 - plane.v0;
    let v  = plane.v2 - plane.v0;

    let p = v0 - o;

    let n = Vec3::cross(u, v);

    // From formula above:
    // t*d*n - a*u*n - b*v*n = p*n
    // t = p*n / (d*n)

    let f = Vec3::dot(d, n);
    if f < 1.0e-4 {
        return false;
    }

    // t*d*(d x v) - a*u*(d x v) - b*v*(d x v) = p*(d x v)
    // a = -p*(d x v) / (u*(d x v)) =>
    // a = -p*(d x v) / (d*(v x u)) =>
    // a =  p*(d x v) / (d*(u x v))

    let a = Vec3::dot(p, Vec3::cross(d,v)) / f;

    // b = -p*(d x u) / (v*(d x u))
    // b = -p*(d x u) / (d*(u x v))

    let b = -Vec3::dot(p, Vec3::cross(d, u)) / f;

    return a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0;
}
