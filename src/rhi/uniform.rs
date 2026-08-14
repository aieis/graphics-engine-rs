
use std::marker::PhantomData;

use crate::DeviceBundle;
use crate::BufferType;
use crate::Allocator;
use crate::BufferBundle;

use ash::vk;

pub struct Uniform<T> {
    staging: BufferBundle,
    uniform: BufferBundle,
    _marker: PhantomData<T>
}

impl<T> Uniform<T> {
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
            data_ptr.copy_from_nonoverlapping(val as *const T, self.staging.size as usize);
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
