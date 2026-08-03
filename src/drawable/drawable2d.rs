use ash::vk;

use crate::mesh::Rect;
use crate::vk_bundles::BufferBundle;
use crate::{utils::buffer, DeviceBundle, GraphicsPipelineBundle};

pub struct Drawable2d {
    pub mesh: Rect,
    pub vbo: BufferBundle,
    pub col: BufferBundle,
    pub ind: BufferBundle,
    pub staging: BufferBundle,
}

impl Drawable2d {

    pub fn new(device: &DeviceBundle, mesh: Rect) -> Self {
        let size_vrt = mesh.size_vrt() as u64;
        let size_col = mesh.size_col() as u64;
        let size_ind = mesh.size_ind() as u64;

        let size_staging = size_vrt + size_col + size_ind;

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        let usage = vk::BufferUsageFlags::TRANSFER_SRC;
        let staging = buffer::create_buffer(device, size_staging, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER;
        let vbo = buffer::create_buffer(device, size_vrt, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER;
        let col = buffer::create_buffer(device, size_col, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER;
        let ind = buffer::create_buffer(device, size_ind, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        Drawable2d { mesh, vbo, col, ind, staging}
    }

    pub fn dirty(&self) -> bool {
        return self.mesh.dirty_colour || self.mesh.dirty_indices || self.mesh.dirty_vertices;
    }

    pub fn update(device: &DeviceBundle, command_buffer: &vk::CommandBuffer, mesh_bundles: &mut Vec<Self>) -> bool {

        let mut recorded = false;

        for mesh_bundle in mesh_bundles.iter_mut() {
            if !mesh_bundle.dirty() {
                continue;
            }

            recorded = true;

            let size_vrt = mesh_bundle.mesh.size_vrt() as u64;
            let size_col = mesh_bundle.mesh.size_col() as u64;
            let size_ind = mesh_bundle.mesh.size_ind() as u64;

            unsafe {
                if mesh_bundle.mesh.dirty_vertices {
                    let data_ptr = device.logical.map_memory(mesh_bundle.staging.memory, 0, size_vrt, vk::MemoryMapFlags::empty()).unwrap() as *mut [f32; 2];
                    data_ptr.copy_from_nonoverlapping(mesh_bundle.mesh.vertices.as_ptr(), mesh_bundle.mesh.vertices.len());
                    device.logical.unmap_memory(mesh_bundle.staging.memory);

                    let copy_region = [
                        vk::BufferCopy::default().size(size_vrt)
                    ];

                    device.logical.cmd_copy_buffer(*command_buffer, mesh_bundle.staging.buffer, mesh_bundle.vbo.buffer, &copy_region);
                }

                if mesh_bundle.mesh.dirty_colour {
                    let data_ptr = device.logical.map_memory(mesh_bundle.staging.memory, size_vrt, size_col, vk::MemoryMapFlags::empty()).unwrap() as *mut [f32; 3];
                    data_ptr.copy_from_nonoverlapping(mesh_bundle.mesh.colour.as_ptr(), mesh_bundle.mesh.colour.len());
                    device.logical.unmap_memory(mesh_bundle.staging.memory);

                    let copy_region = [
                        vk::BufferCopy::default()
                            .src_offset(size_vrt)
                            .size(size_col)
                    ];

                    device.logical.cmd_copy_buffer(*command_buffer, mesh_bundle.staging.buffer, mesh_bundle.col.buffer, &copy_region);
                }

                if mesh_bundle.mesh.dirty_indices {
                    let data_ptr = device.logical.map_memory(mesh_bundle.staging.memory, size_vrt+size_col, size_ind, vk::MemoryMapFlags::empty()).unwrap() as *mut u16;
                    data_ptr.copy_from_nonoverlapping(mesh_bundle.mesh.indices.as_ptr(), mesh_bundle.mesh.indices.len());
                    device.logical.unmap_memory(mesh_bundle.staging.memory);

                    let copy_region = [
                        vk::BufferCopy::default()
                            .src_offset(size_vrt+size_col)
                            .size(size_ind as u64)
                    ];

                    device.logical.cmd_copy_buffer(*command_buffer, mesh_bundle.staging.buffer, mesh_bundle.ind.buffer, &copy_region);

                }
            }

            mesh_bundle.mesh.dirty_colour = false;
            mesh_bundle.mesh.dirty_vertices = false;
            mesh_bundle.mesh.dirty_indices = false;
        }

        return recorded;
    }

    pub fn draw(device: &DeviceBundle, command_buffer: &vk::CommandBuffer, graphics_pipeline: &GraphicsPipelineBundle, mesh_bundles: &[Self])  {
        let command_buffer = *command_buffer;
        unsafe {
            device.logical.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, graphics_pipeline.graphics);
            for i in 0..mesh_bundles.len() {
                device.logical.cmd_bind_vertex_buffers(command_buffer, 0, &[mesh_bundles[i].vbo.buffer, mesh_bundles[i].col.buffer], &[0, 0]);
                device.logical.cmd_bind_index_buffer(command_buffer, mesh_bundles[i].ind.buffer, 0, vk::IndexType::UINT16);
                device.logical.cmd_draw_indexed(command_buffer, mesh_bundles[i].mesh.indices.len() as u32, 1, 0, 0, 0);
            }
        }
    }

    pub fn release(device: &DeviceBundle, mesh_bundles: &mut [Self])
    {
        unsafe
        {
            for mesh in mesh_bundles.iter()
            {
                device.logical.destroy_buffer(mesh.vbo.buffer, None);
                device.logical.free_memory(mesh.vbo.memory, None);
                device.logical.destroy_buffer(mesh.staging.buffer, None);
                device.logical.free_memory(mesh.staging.memory, None);
                device.logical.destroy_buffer(mesh.col.buffer, None);
                device.logical.free_memory(mesh.col.memory, None);
                device.logical.destroy_buffer(mesh.ind.buffer, None);
                device.logical.free_memory(mesh.ind.memory, None);
            }
        }
    }
}
