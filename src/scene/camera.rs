use crate::geometry::{Vec3, Vec4, Mat4};

pub enum CameraAction {
    Right,
    Left,
    Up,
    Down,
    Forward,
    Backward,
    RotateX,
    RotateY,
    SnapDirX,
    SnapDirY,
    SnapPosX,
    SnapPosY,
}

#[repr(C, align(16))]
pub struct CameraParams {
    pub location: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub view: Mat4,
    pub projection: Mat4
}

const UP: Vec3 = Vec3::Y;

#[allow(non_upper_case_globals)]
const  PIx2        : f32 = std::f32::consts::PI * 2.0;
const  PI_2        : f32 = std::f32::consts::PI / 2.0;
const  PI_4        : f32 = std::f32::consts::PI / 4.0;
const  PI_8        : f32 = std::f32::consts::PI / 8.0;
const  ANGLE_Y_MAX : f32 =  PI_2 - PI_8;
const  ANGLE_Y_MIN : f32 = -ANGLE_Y_MAX;

pub struct Camera {
    pub params: CameraParams,

    x_angle: f32,
    x_sin: f32,
    x_cos: f32,

    y_angle: f32,
    y_sin: f32,
    y_cos: f32,

    right: Vec3,

    width: f32,
    height: f32,
    fov: f32

}

impl Camera {

    pub fn new(location: Vec3, x_angle: f32, y_angle: f32, width: f32, height: f32, fov: f32) -> Self {
        let y_sin   = y_angle.sin();
        let y_cos   = y_angle.cos();

        let x_sin   = x_angle.sin();
        let x_cos   = x_angle.cos();

        let direction = Self::calc_direction(x_sin, x_cos, y_sin, y_cos);
        let right = Self::calc_right(direction);
        let up = Vec3::Y;

        let view = Self::create_view_matrix(location, direction, up);
        let projection = Self::create_projection_matrix(fov, if height <= 0.0 { width / height } else { 1.0 }) ;


        let params = CameraParams {
            location,
            direction,
            up,
            view,
            projection
        };


        println!(
            "Camera Calculated Direction: \n\t x_angle: {} \n\t y_angle: {} \n\t direction: {}",
            x_angle * 360.0 / PIx2,
            y_angle * 360.0 / PIx2,
            direction
        );

        Self {
            params,
            right,
            x_angle,
            x_sin,
            x_cos,
            y_angle,
            y_sin,
            y_cos,

            width,
            height,
            fov,
        }
    }


    pub fn update(&mut self, action: CameraAction, delta: f32) {
        match action {
            CameraAction::Right => self.params.location += delta * self.right,
            CameraAction::Left => self.params.location -= delta * self.right,
            CameraAction::Up => self.params.location += delta * self.params.up,
            CameraAction::Down => self.params.location -= delta * self.params.up,
            CameraAction::Forward =>  self.params.location += delta * self.params.direction,
            CameraAction::Backward => self.params.location -= delta * self.params.direction,
            CameraAction::RotateX => {
                self.x_angle += delta;
                self.x_sin = self.x_angle.sin();
                self.x_cos = self.x_angle.cos();

                self.params.direction = Self::calc_direction(self.x_sin, self.x_cos, self.y_sin, self.y_cos);
                self.right = Self::calc_right(self.params.direction);
            },

            CameraAction::RotateY => {
                self.y_angle += delta;
                self.y_angle  = self.y_angle.clamp(ANGLE_Y_MIN, ANGLE_Y_MAX);
                self.y_sin = self.y_angle.sin();
                self.y_cos = self.y_angle.cos();

                self.params.direction = Self::calc_direction(self.x_sin, self.x_cos, self.y_sin, self.y_cos);
                self.right = Self::calc_right(self.params.direction);

            },

            CameraAction::SnapDirX => {
                self.x_angle = angle_to_closest_pi_div_2(self.x_angle);

                self.x_sin = self.x_angle.sin();
                self.x_cos = self.x_angle.cos();
                self.params.direction = Self::calc_direction(self.x_sin, self.x_cos, self.y_sin, self.y_cos);
                self.right = Self::calc_right(self.params.direction);
            },

            CameraAction::SnapDirY => {
                self.y_angle = angle_to_closest_pi_div_2(self.y_angle);
                self.y_angle  = self.y_angle.clamp(ANGLE_Y_MIN, ANGLE_Y_MAX);

                self.y_sin = self.y_angle.sin();
                self.y_cos = self.y_angle.cos();
                self.params.direction = Self::calc_direction(self.x_sin, self.x_cos, self.y_sin, self.y_cos);
                self.right = Self::calc_right(self.params.direction);
            }

            CameraAction::SnapPosX => {
                self.params.location  += delta * self.right;
                self.params.location.x = round_to_nearest(self.params.location.x, delta);
            }

            CameraAction::SnapPosY => {
                self.params.location  += delta * UP;
                self.params.location.y = round_to_nearest(self.params.location.y, delta);
            }
        }

        self.params.view = Self::create_view_matrix(self.params.location, self.params.direction, self.params.up);
        self.params.projection = Self::create_projection_matrix(self.fov, if self.height <= 0.0 { self.width / self.height } else { 1.0 });
    }

    fn calc_direction(x_sin: f32, x_cos: f32, y_sin: f32, y_cos: f32) -> Vec3 {
        Vec3::norm(Vec3::new(y_cos * x_cos, y_sin, y_cos * x_sin))
    }

    fn calc_right(direction: Vec3) -> Vec3 {
        Vec3::norm(Vec3::cross(direction, UP))
    }

    fn create_projection_matrix(fov: f32, aspect: f32) -> Mat4 {
        let F: f32 = 50.0;
        let N: f32 = 0.1;
        let C: f32 = 1.0 / (fov/2.0).tan();

        let X: f32 = C / aspect;

        // z' = Az + B
        // z'' = z' / -z
        // So we need an A and B such that z' gets mapped to 0 when z==N and 1 when at z==F
        // (A*N+B) / (-N) = 0 and (A*F+B)/(-F) = 1
        let A: f32 = -F/(F-N);
        let B: f32 = -(N*F)/(F-N);

        let proj: Mat4 = Mat4::new(
            Vec4::new(X  ,  0.0,  0.0,  0.0),
            Vec4::new(0.0, -C  ,  0.0,  0.0),
            Vec4::new(0.0,  0.0,  A  ,  B),
            Vec4::new(0.0,  0.0, -1.0,  0.0)
        );

        return Mat4::transpose(proj);
    }


    fn create_view_matrix(pos: Vec3, dir: Vec3, up: Vec3) -> Mat4 {

        /*
         * The premise as follows the component of a vector v onto a basis can be derived as follows
         * cos(t) = c' / |v| because the vector forms the hypotenuse (imagine vector (1, 1) on the cartesian grid)
         * -> c' = |v| * cos(t)
         * -> c' = 1 * |v| * cos(t) so if you are projecting onto a vector 'a' of length 1 then we get
         * -> c' = |a| * |v| * cos(t) which is the dot product
         * -> c' = dot(a,v)
         */

        let f: Vec3 =  Vec3::norm(dir);
        let r: Vec3 =  Vec3::norm(Vec3::cross(f, up));
        let u: Vec3 = -Vec3::norm(Vec3::cross(f, r));
        let b: Vec3 = -f;

        let disp = Vec3::new(-Vec3::dot(r,pos), -Vec3::dot(u,pos), -Vec3::dot(b,pos));

        let view = Mat4::new(
            Vec4::new(r.x, r.y, r.z, disp.x),
            Vec4::new(u.x, u.y, u.z, disp.y),
            Vec4::new(b.x, b.y, b.z, disp.z),
            Vec4::new(0.0, 0.0, 0.0, 1.0)
        );

        return Mat4::transpose(view);
    }


}


// Closest angle to Pi / 2 (45 degrees)
fn angle_to_closest_pi_div_2(angle: f32) -> f32 {
    let angle = ((angle % PIx2) + PIx2) % PIx2;
    let mut i = 0;
    while i < 9 {
        let a = PI_4 * i as f32;
        let d = angle - a;

        if d <= PI_8 && d >= -PI_8 {
            return a;
        }

        i+=1;
    }

    angle
}

fn round_to_nearest(v: f32, d: f32) -> f32 {
    if d > 1.0e-3 { (v / d).round() * d } else { v }
}
