use ash::vk;

use crate::rhi::allocator::{Allocator, BufferType};
use crate::rhi::uniform::VariableDeviceBuffer;
use crate::{DeviceBundle, GraphicsPipelineBundle};

pub struct DrawablePcl {

    proj: VariableDeviceBuffer,
    dist: VariableDeviceBuffer,
    col: VariableDeviceBuffer,

    count: usize,
    internal_data: Vec<f32>

}

impl DrawablePcl {

    pub fn new(allocator: &mut Allocator, count: usize) -> Self {

        let internal_data = vec![0.0; count as usize];

        let proj = VariableDeviceBuffer::new(allocator, count as u64 * std::mem::size_of::<f32>() as u64 * 3, BufferType::DeviceVertex);
        let col  = VariableDeviceBuffer::new(allocator, count as u64 * std::mem::size_of::<f32>() as u64 * 3, BufferType::DeviceVertex);
        let dist = VariableDeviceBuffer::new(allocator, count as u64 * std::mem::size_of::<f32>() as u64, BufferType::DeviceVertex);


        DrawablePcl {
            proj,
            dist,
            col,
            count,
            internal_data
        }
    }

    pub fn init(&mut self, device: &DeviceBundle, cb: vk::CommandBuffer, proj_data: &[f32], col_data: &[f32]) {

        if proj_data.len() != self.count * 3 || col_data.len() != self.count * 3 {
            eprintln!("DarawablePcl: Could not initialize data.");
            dbg!(proj_data.len());
            dbg!(col_data.len());
            dbg!(self.count*3);
            return;
        }

        self.proj.update(device, cb, proj_data);
        self.col.update(device, cb, col_data);
    }

    pub fn update(&mut self, device: &DeviceBundle, cb: vk::CommandBuffer, pcl_data: &[u16]) {


        if pcl_data.len() != self.internal_data.len() {
            eprintln!("DrawablePcl: Input data does not match expected. Expected len {} got {}", self.internal_data.len(), pcl_data.len());
            return;
        }


        for i in 0..self.internal_data.len() {
            self.internal_data[i] = pcl_data[i] as f32;
        }

        self.dist.update(device, cb, &self.internal_data);
    }


    pub fn draw(device: &DeviceBundle, cb: vk::CommandBuffer, graphics_pipeline: &GraphicsPipelineBundle, mesh_bundles: &[Self])  {
        unsafe {
            device.logical.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, graphics_pipeline.graphics);

            for i in 0..mesh_bundles.len() {
                let m = &mesh_bundles[i];
                device.logical.cmd_bind_vertex_buffers(cb, 0,
                                                       &[m.proj.buffer.buffer, m.col.buffer.buffer, m.dist.buffer.buffer],
                                                       &[m.proj.buffer.offset, m.col.buffer.offset, m.dist.buffer.offset]);

                device.logical.cmd_draw(cb, m.count as u32, 1, 0, 0);
            }
        }
    }

    pub fn release(_device: &DeviceBundle, _mesh_bundles: &mut [Self]) {

    }
}
