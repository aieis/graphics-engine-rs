use ash::vk;

use crate::mesh::Mesh;
use crate::{DeviceBundle, GraphicsPipelineBundle};

use crate::rhi::{allocator::{Allocator, BufferType}, uniform::VariableDeviceBuffer};

pub struct DrawableCard {
    pub mesh: Mesh,
    pub vbo: VariableDeviceBuffer,
    pub col: VariableDeviceBuffer,
    pub ind: VariableDeviceBuffer,
    pub normals: VariableDeviceBuffer,
}

impl DrawableCard {

    pub fn new(mesh: Mesh, allocator: &mut Allocator) -> Self {

        let size_vrt = mesh.size_vrt() as u64;
        let size_col = mesh.size_col() as u64;
        let size_ind = mesh.size_ind() as u64;
        let size_normals = mesh.size_normals() as u64;

        let vbo     = VariableDeviceBuffer::new(allocator, size_vrt, BufferType::DeviceVertex);
        let col     = VariableDeviceBuffer::new(allocator, size_col, BufferType::DeviceVertex);
        let normals = VariableDeviceBuffer::new(allocator, size_normals, BufferType::DeviceVertex);
        let ind     = VariableDeviceBuffer::new(allocator, size_ind, BufferType::DeviceIndex);

        Self { mesh, vbo, col, ind, normals }
    }

    pub fn dirty(&self) -> bool {
        return self.mesh.dirty_colour || self.mesh.dirty_indices || self.mesh.dirty_vertices || self.mesh.dirty_normals;
    }

    pub fn update(device: &DeviceBundle, cb: vk::CommandBuffer, mesh_bundles: &mut [Self]) -> bool {
        let mut recorded = false;

        for m in mesh_bundles.iter_mut() {
            if !m.dirty() {
                continue;
            }

            recorded = true;

            if m.mesh.dirty_vertices {
                m.vbo.update(device, cb, &m.mesh.vertices);
            }

            if m.mesh.dirty_colour {
                m.col.update(device, cb, &m.mesh.colour);
            }

            if m.mesh.dirty_normals {
                m.normals.update(device, cb, &m.mesh.normals);
            }

            if m.mesh.dirty_indices {
                m.ind.update(device, cb, &m.mesh.indices);
            }

            m.mesh.dirty_colour   = false;
            m.mesh.dirty_vertices = false;
            m.mesh.dirty_indices  = false;
            m.mesh.dirty_normals  = false;
        }

        return recorded;
    }

    pub fn draw(device: &DeviceBundle, cb: vk::CommandBuffer, graphics_pipeline: &GraphicsPipelineBundle, mesh_bundles: &[Self])  {
        unsafe {
            device.logical.cmd_bind_pipeline(cb, vk::PipelineBindPoint::GRAPHICS, graphics_pipeline.graphics);

            for i in 0..mesh_bundles.len() {
                let m = &mesh_bundles[i];
                let bufs = [m.vbo.buffer.buffer, m.col.buffer.buffer, m.normals.buffer.buffer];
                let offs = [m.vbo.buffer.offset, m.col.buffer.offset, m.normals.buffer.offset];
                device.logical.cmd_bind_vertex_buffers(cb, 0, &bufs, &offs);
                device.logical.cmd_bind_index_buffer(cb, m.ind.buffer.buffer, 0, vk::IndexType::UINT16);
                device.logical.cmd_draw_indexed(cb, m.mesh.indices.len() as u32, 1, 0, 0, 0);
            }
        }
    }

    pub fn release(_device: &DeviceBundle, _mesh_bundles: &mut [Self]) {

    }
}
