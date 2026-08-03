use ash::vk;

pub struct GraphicsPSO
{
    shader_id : usize,
    topology  : vk::PrimitiveTopology
}
