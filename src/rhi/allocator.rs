use ash::vk;

use crate::{utils::buffer, vk_base::VkBase, vk_bundles::{BufferBundle, DeviceBundle}};


pub enum BufferType {

    /* Properties: HOST_VISIBLE | HOST_COHERENT
     * Usage: TRANSFER_SRC */
    Staging,


    /* Properties: DEVICE_LOCAL
     * Usage: TRANSFER_DST | VERTEX_BUFFER */
    DeviceVertex,

    /* Properties: DEVICE_LOCAL
     * Usage: TRANSFER_DST | INDEX_BUFFER */
    DeviceIndex,


    /* Properties: DEVICE_LOCAL
     * Usage: TRANSFER_DST | UNIFORM_BUFFER */
    Uniform,

}

pub struct AllocatorSizeInfo {
    pub staging        : u64,
    pub device_vertex  : u64,
    pub device_index   : u64,
    pub uniform_buffer : u64,
}

pub struct AllocatorHeap {
    pub heap   : BufferBundle,
    pub offset : u64,
    pub size   : u64,
    pub align  : u64,
    pub waste  : u64
}


pub struct Allocator {
    pub staging        : AllocatorHeap,
    pub device_vertex  : AllocatorHeap,
    pub device_index   : AllocatorHeap,
    pub uniform_buffer : AllocatorHeap,
}


impl Allocator {


    pub fn new(base: &VkBase, sizes: AllocatorSizeInfo) -> Self {

        let offset = 0;

        let staging = {
            let size = sizes.staging;
            let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
            let usage = vk::BufferUsageFlags::TRANSFER_SRC;
            let heap = buffer::create_buffer(&base.device, size, usage, required_memory_flags).expect("Failed to create buffer.");
            let mem_requirements = unsafe { base.device.logical.get_buffer_memory_requirements(heap.buffer) };
            let align = mem_requirements.alignment;
            AllocatorHeap { heap, offset, size, align, waste: 0 }
        };

        let device_vertex = {
            let size = sizes.device_vertex;
            let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
            let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER;
            let heap = buffer::create_buffer(&base.device, size, usage, required_memory_flags).expect("Failed to create buffer.");
            let mem_requirements = unsafe { base.device.logical.get_buffer_memory_requirements(heap.buffer) };
            let align = mem_requirements.alignment;
            AllocatorHeap { heap, offset, size, align, waste: 0 }
        };


        let device_index = {
            let size = sizes.device_index;
            let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
            let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER;
            let heap = buffer::create_buffer(&base.device, size, usage, required_memory_flags).expect("Failed to create buffer.");
            let mem_requirements = unsafe { base.device.logical.get_buffer_memory_requirements(heap.buffer) };
            let align = mem_requirements.alignment;
            AllocatorHeap { heap, offset, size, align, waste: 0 }
        };

        let uniform_buffer = {
            let size = sizes.uniform_buffer;
            let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
            let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::UNIFORM_BUFFER;
            let heap = buffer::create_buffer(&base.device, size, usage, required_memory_flags).expect("Failed to create buffer.");
            let mem_requirements = unsafe { base.device.logical.get_buffer_memory_requirements(heap.buffer) };
            let align = mem_requirements.alignment;
            AllocatorHeap { heap, offset, size, align, waste: 0 }
        };


        Self {
            staging,
            device_vertex,
            device_index,
            uniform_buffer
        }
    }

    pub fn alloc(&mut self, buffer_type: BufferType, size: u64) -> Result<BufferBundle, String> {

        let heap = match buffer_type {
            BufferType::Staging => &mut self.staging,
            BufferType::DeviceVertex => &mut self.device_vertex,
            BufferType::DeviceIndex => &mut self.device_index,
            BufferType::Uniform => &mut self.uniform_buffer,
        };

        if heap.offset + size >= heap.size {
            return Err(format!("Allocator: Requested {} bytes from heap of size {} with {} remaining", size, heap.size, heap.size - heap.offset));
        }

        let bundle = BufferBundle {
            buffer: heap.heap.buffer,
            memory: heap.heap.memory,
            offset: heap.offset,
            size
        };

        let incr = if heap.align > 0 { size.next_multiple_of(heap.align) } else { size };

        if incr != size {
            heap.waste += incr - size;
        }

        heap.offset += incr;

        Ok(bundle)
    }


    pub fn print_heap_stats(heap: &AllocatorHeap, name: &str) {

        println!("Heap '{}' stats: ", name);
        println!("\t Size: {}", heap.size);
        println!("\t Align: {}", heap.align);
        println!("\t Offset: {}", heap.offset);
        println!("\t Waste: {}", heap.waste);
        println!("\t Remaining: {}", heap.size - heap.offset);

    }


    pub fn release(&mut self, device: &DeviceBundle) {
        unsafe {
            Self::print_heap_stats(&self.staging, "Staging");
            device.logical.destroy_buffer(self.staging.heap.buffer, None);
            device.logical.free_memory(self.staging.heap.memory, None);
            self.staging.offset = 0;
            self.staging.size = 0;

            Self::print_heap_stats(&self.device_vertex, "Vertex");
            device.logical.destroy_buffer(self.device_vertex.heap.buffer, None);
            device.logical.free_memory(self.device_vertex.heap.memory, None);
            self.device_vertex.offset = 0;
            self.device_vertex.size = 0;

            Self::print_heap_stats(&self.device_index, "Index");
            device.logical.destroy_buffer(self.device_index.heap.buffer, None);
            device.logical.free_memory(self.device_index.heap.memory, None);
            self.device_index.offset = 0;
            self.device_index.size = 0;

            Self::print_heap_stats(&self.uniform_buffer, "Uniform_Buffer");
            device.logical.destroy_buffer(self.uniform_buffer.heap.buffer, None);
            device.logical.free_memory(self.uniform_buffer.heap.memory, None);
            self.uniform_buffer.offset = 0;
            self.uniform_buffer.size = 0;


        }
    }
}
