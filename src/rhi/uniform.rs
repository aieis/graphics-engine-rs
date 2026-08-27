
use std::marker::PhantomData;

use crate::DeviceBundle;
use crate::BufferType;
use crate::Allocator;
use crate::BufferBundle;

use ash::vk;

pub struct StaticUniform<T> {
    pub staging: BufferBundle,
    pub uniform: BufferBundle,
    _marker: PhantomData<T>
}

impl<T> StaticUniform<T> {
    pub fn new(allocator: &mut Allocator) -> Self{
        let staging = allocator.alloc(BufferType::Staging, std::mem::size_of::<T>() as u64).unwrap();
        let uniform = allocator.alloc(BufferType::Uniform, std::mem::size_of::<T>() as u64).unwrap();

        Self {
            staging,
            uniform,
            _marker: PhantomData::<T> {}
        }
    }

    pub fn update(&mut self, device: &DeviceBundle, cb: vk::CommandBuffer, val: &T)  {
        unsafe {
            let data_ptr = device.logical.map_memory(self.staging.memory, self.staging.offset, self.staging.size, vk::MemoryMapFlags::empty()).unwrap() as *mut T;
            data_ptr.copy_from_nonoverlapping(val as *const T, 1);
            device.logical.unmap_memory(self.staging.memory);

            let copy_region = [
                vk::BufferCopy::default()
                    .src_offset(self.staging.offset)
                    .dst_offset(self.uniform.offset)
                    .size(self.staging.size)
            ];

            device.logical.cmd_copy_buffer(cb, self.staging.buffer, self.uniform.buffer, &copy_region);
        }
    }
}

pub struct VariableUniform {
    pub staging: BufferBundle,
    pub uniform: BufferBundle,
}

impl VariableUniform {

    pub fn new(allocator: &mut Allocator, max_size: u64) -> Self{
        let staging = allocator.alloc(BufferType::Staging, max_size).unwrap();
        let uniform = allocator.alloc(BufferType::Uniform, max_size).unwrap();

        Self {
            staging,
            uniform,
         }
    }

    pub fn update<T>(&mut self, device: &DeviceBundle, cb: vk::CommandBuffer, val: &[T])  {
        unsafe {
            let data_ptr = device.logical.map_memory(self.staging.memory, self.staging.offset, self.staging.size, vk::MemoryMapFlags::empty()).unwrap() as *mut T;
            data_ptr.copy_from_nonoverlapping(val.as_ptr(), val.len());
            device.logical.unmap_memory(self.staging.memory);

            let copy_region = [
                vk::BufferCopy::default()
                    .src_offset(self.staging.offset)
                    .dst_offset(self.uniform.offset)
                    .size(self.staging.size)
            ];

            device.logical.cmd_copy_buffer(cb, self.staging.buffer, self.uniform.buffer, &copy_region);
        }
    }

}
