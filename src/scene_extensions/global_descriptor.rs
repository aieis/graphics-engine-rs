use crate::vk_bundles::DescSetBinding;

use ash::vk;

pub const GLOBAL_DESCRIPTOR_SET_BINDING: DescSetBinding = DescSetBinding {
    binding: 0,
    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
    descriptor_count: 1,
    stage_flags: vk::ShaderStageFlags::VERTEX,
};
