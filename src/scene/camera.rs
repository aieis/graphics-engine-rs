use crate::geometry::vec3::Vec3;


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

#[repr(C)]
pub struct CameraParams {
    pub location: Vec3,
    pub direction: Vec3,
    pub up: Vec3

}

const UP: Vec3 = Vec3::Y;

#[allow(non_upper_case_globals)]
const  PIx2        : f32 = std::f32::consts::PI * 2.0;
const  PI_4        : f32 = std::f32::consts::PI / 4.0;
const  PI_8        : f32 = std::f32::consts::PI / 8.0;
const  ANGLE_Y_MAX : f32 =  PI_4;
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
}

impl Camera {

    pub fn new(location: Vec3, x_angle: f32, y_angle: f32) -> Self {
        let y_sin   = y_angle.sin();
        let y_cos   = y_angle.cos();

        let x_sin   = x_angle.sin();
        let x_cos   = x_angle.cos();

        let direction = Self::calc_direction(x_sin, x_cos, y_sin, y_cos);
        let right = Self::calc_right(direction);


        Self {
            params: CameraParams { location, direction, up: Vec3::Y },
            right,
            x_angle,
            x_sin,
            x_cos,
            y_angle,
            y_sin,
            y_cos,
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
    }

    fn calc_direction(x_sin: f32, x_cos: f32, y_sin: f32, y_cos: f32) -> Vec3 {
        Vec3::norm(Vec3::new(y_cos * x_cos, y_sin, y_cos * x_sin))
    }

    fn calc_right(direction: Vec3) -> Vec3 {
        Vec3::norm(Vec3::cross(direction, UP))
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
