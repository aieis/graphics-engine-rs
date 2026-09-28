use ash::vk;

use crate::geometry::Vec3;
use crate::utils::colours;
use crate::{DeviceBundle, GraphicsPipelineBundle};

use crate::shader::ShaderLightColumn_Params;

pub struct DrawableLightColumn {
    pub params: ShaderLightColumn_Params,
}

const SEGMENT_COUNT: usize = 8;
const TRI_COUNT: usize = SEGMENT_COUNT * 2 + 2;
impl DrawableLightColumn {

    pub fn new() -> Self {

        let params = ShaderLightColumn_Params {
            point_a: Vec3::new(0.0, 0.0, 0.0),
            point_b: Vec3::new(0.0, 0.0, -1.0),
            colour: colours::YELLOW
        };

        Self { params }
    }

    pub fn draw(device: &DeviceBundle, cb: vk::CommandBuffer, pso: &GraphicsPipelineBundle, mesh_bundles: &[Self])  {
        unsafe {
            device.logical.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, pso.graphics);

            for i in 0..mesh_bundles.len() {
                let m = &mesh_bundles[i];
                device.logical.cmd_push_constants(cb, pso.layout, vk::ShaderStageFlags::VERTEX, 0, std::slice::from_raw_parts(&m.params as *const _ as *const u8, std::mem::size_of::<ShaderLightColumn_Params>()));
                device.logical.cmd_draw(cb, TRI_COUNT as u32 * 3, 1, 0, 0);
            }
        }
    }

}
