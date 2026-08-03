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
    ToggleType,
}

#[repr(C)]
pub struct CameraParams {
    pub location: Vec3,
    pub direction: Vec3,
    pub up: Vec3

}

pub struct Camera {
    pub params: CameraParams,


    right: Vec3,
}

impl Camera {

    pub fn new(location: Vec3, direction: Vec3) -> Self {

        Self {
            params: CameraParams { location, direction, up: Vec3::Y },
            right: Vec3::X
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
            CameraAction::RotateX => todo!(),
            CameraAction::RotateY => todo!(),
            CameraAction::SnapDirX => todo!(),
            CameraAction::SnapDirY => todo!(),
            CameraAction::SnapPosX => todo!(),
            CameraAction::SnapPosY => todo!(),
            CameraAction::ToggleType => todo!(),
        }
    }

}
