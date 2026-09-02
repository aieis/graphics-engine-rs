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

    pub fn new(location: Vec3, direction: Vec3) -> Self {

        let y_sin = direction.y;
        let y_angle = y_sin.asin();
        let y_cos   = y_angle.cos();

        let x_angle = (direction.x / y_cos).acos();
        let x_sin   = x_angle.sin();
        let x_cos   = x_angle.cos();

        Self {
            params: CameraParams { location, direction, up: Vec3::Y },
            right: Vec3::X,
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

                self.params.direction = Vec3::new(self.y_cos * self.x_cos, self.y_sin, self.y_cos * self.x_sin);
                self.right = Vec3::norm(Vec3::cross(self.params.direction, UP));
            },

            CameraAction::RotateY => {
                self.y_angle += delta;
                self.y_sin = self.y_angle.sin();
                self.y_cos = self.y_angle.cos();

                self.params.direction = Vec3::new(self.y_cos * self.x_cos, self.y_sin, self.y_cos * self.x_sin);
                self.right = Vec3::norm(Vec3::cross(self.params.direction, UP));

            },

            CameraAction::SnapDirX => todo!(),
            CameraAction::SnapDirY => todo!(),
            CameraAction::SnapPosX => todo!(),
            CameraAction::SnapPosY => todo!(),
        }
    }

}
