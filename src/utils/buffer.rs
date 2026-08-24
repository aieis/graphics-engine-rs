use anyhow::Result;

use ash::vk;

use crate::{BufferBundle, DeviceBundle};

use super::common::find_memory_type;

pub fn create_buffer_with_memory(device: &DeviceBundle, size: u64, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> Result<BufferBundle>{

    let buffer_create_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let buffer = unsafe { device.logical.create_buffer(&buffer_create_info, None)? };
    let mem_requirements = unsafe { device.logical.get_buffer_memory_requirements(buffer) };
    let memory_type = find_memory_type(mem_requirements.memory_type_bits, properties, device.mem_properties)?;

    let allocate_info = vk::MemoryAllocateInfo::default()
        .allocation_size(mem_requirements.size)
        .memory_type_index(memory_type);

    let memory = unsafe { device.logical.allocate_memory(&allocate_info, None)? };

    unsafe { device.logical.bind_buffer_memory(buffer, memory, 0)?; }


    Ok( BufferBundle { buffer, memory, offset: 0, size } )
}


pub fn bind_buffer_memory(device: &DeviceBundle, buffer: vk::Buffer, size: u64, offset: u64, properties: vk::MemoryPropertyFlags) -> Result<vk::DeviceMemory> {
    let mem_requirements = unsafe { device.logical.get_buffer_memory_requirements(buffer) };
    let memory_type = find_memory_type(mem_requirements.memory_type_bits, properties, device.mem_properties)?;

    let allocate_info = vk::MemoryAllocateInfo::default()
        .allocation_size(size)
        .memory_type_index(memory_type);

    let memory = unsafe { device.logical.allocate_memory(&allocate_info, None)? };
    unsafe { device.logical.bind_buffer_memory(buffer, memory, offset)?; }

    Ok ( memory )
}


pub fn create_staging_buffer(device: &DeviceBundle, size: u64) -> BufferBundle {
    let required_memory_properties = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    create_buffer_with_memory(device, size, vk::BufferUsageFlags::TRANSFER_SRC, required_memory_properties).unwrap()
}
