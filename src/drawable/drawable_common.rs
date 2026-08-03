use ash::vk;

use crate::DeviceBundle;


#[allow(unused)]
pub trait Drawable {
    fn dirty(&self) -> bool;
    fn update(&self, device: &DeviceBundle);
    fn record_update(&self, device: &DeviceBundle, command_buffer: &vk::CommandBuffer);
}
