use ash::vk;
use ash::khr;

pub struct SurfaceBundle {
    pub surface: vk::SurfaceKHR,
    pub loader: khr::surface::Instance
}

pub struct DeviceBundle {
    pub logical: ash::Device,
    pub physical: vk::PhysicalDevice,
    pub queue_family_index: u32,
    pub present_queue: vk::Queue,
    pub mem_properties: vk::PhysicalDeviceMemoryProperties
}

pub struct SwapchainBundle {
    pub swapchain: vk::SwapchainKHR,
    pub loader: khr::swapchain::Device,
    pub images: Vec<vk::Image>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

#[derive(Clone)]
pub struct DescSetBinding {
    pub binding: u32,
    pub descriptor_type: vk::DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: vk::ShaderStageFlags,
}


#[derive(Clone)]
pub struct PipelineDescriptor {
    pub ubo_layout_bindings: Vec<DescSetBinding>,
    pub vertex_bindings: Vec<vk::VertexInputBindingDescription>,
    pub vertex_attributes: Vec<vk::VertexInputAttributeDescription>
}

pub struct GraphicsPipelineDescSetData {
    pub has_global_layout: bool,
    pub layouts: Vec<vk::DescriptorSetLayout>,
}

pub struct GraphicsPipelineBundle {
    pub id: usize,
    pub graphics: vk::Pipeline,
    pub layout: vk::PipelineLayout,
    pub ubo: Option<GraphicsPipelineDescSetData>,
}

pub struct SyncObjectsBundle {
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub spare_fences: Vec<vk::Fence>,
}

pub struct BufferBundle {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub offset: u64,
    pub size: u64
}

pub struct ImageBundle {
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
    pub format: vk::Format,
}


pub struct CommandBundle {
    pub pool: vk::CommandPool,
    pub buffers: Vec<vk::CommandBuffer>
}

pub struct TextureBundle {
    pub resource: ImageBundle,
    pub sampler: vk::Sampler,
    pub image_view: vk::ImageView,
    pub aspect_flags: vk::ImageAspectFlags,
    pub staging: BufferBundle,
}
