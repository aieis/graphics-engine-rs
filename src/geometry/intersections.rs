use super::Vec3;

/// Line values are a function of z
pub struct Line {
    pub x_0: f32,
    pub y_0: f32,

    /// Slope on of x as a function of z
    pub x_m: f32,

    /// Slope on of y as a function of z
    pub y_m: f32
}


pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

pub fn does_line_intersect_sphere(line: Line, sphere: Sphere) -> bool {

    let x_s = sphere.center.x;
    let y_s = sphere.center.y;
    let z_s = sphere.center.z;

    let x_l = line.x_0;
    let x_m = line.x_m;
    let y_l = line.y_0;
    let y_m = line.y_m;

    let r_2 = sphere.radius * sphere.radius;


    let x = |z| { x_m * z + x_l };
    let y = |z| { y_m * z + y_l };
    let d = |z| { (x(z) - x_s) * (x(z) - x_s) + (y(z) - y_s) * (y(z) - y_s)  + (z - z_s) * (z - z_s) };

    // distance_function
    // d(z)   =       (x(z) - x_s)^2     +        (y(z) - y_s)^2  + (z - z_s)^2
    // d'(z)  = 2*x_m*(x(z) - x_s)  +  2*y_m*(y(z) - y_s) + 2*(z - z_s)
    // d''(z) = 2*x_m*x_m + 2*y_m*y_m + 2

    // f(z)   = d(z) ^ (1/2)

    // f'(z)  = d(z) ^ (-1/2) * d'(z)
    //        = d'(z) / sqrt( d(z) )

    // g(z)   = d(z) ^ (-1/2)

    // f'(z)  = g(z) * d'(z)

    // g'(z)  = (-1/2) * d(z) ^ (-3/2) * d'(z)

    // f''(z) = g'(z) * d'(z) + d''(z) * g(z)

    // Solve for
    // f'(z) = 0 which is where d'(z) == 0 or where it is undefined which is where d(z) == 0

    let z_v = (x_l - x_s + y_l - y_s - z_s) / (2.0*x_m*x_m + 2.0*y_m*y_m + 2.0);

     // d(z) = (x_m^2+y_m^2 + 1) * z^2 + (2*(x_l - x_s + y_l - y_s - z_s)) * z + ((x_l-x_s)^2 + (y_l - y_s)^2 + z_s^2)
    // d(z) = 0

    // QUARDRATIC FORMULA!
    // z   = (-b +/- sqrt( b^2 - 4ac)) / (2a)

    let a = x_m*x_m+y_m*y_m + 1.0;
    let b = 2.0*(x_l - x_s + y_l - y_s - z_s);
    let c = (x_l-x_s) * (x_l-x_s) + (y_l - y_s) * (y_l - y_s) + z_s * z_s;

    let desc = b*b - 4.0 * a * c;
    if desc < 0.0 {
        return  d(z_v) < r_2;
    }

    let q = desc.sqrt();

    let z_n = (-b - q) / (2.0 * a);
    let z_p = (-b + q) / (2.0 * a);

    return d(z_v) < r_2 || d(z_n) < r_2 || d(z_p) < r_2;
}
