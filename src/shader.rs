use std::{
    path::PathBuf, str::FromStr, time::SystemTime
};

use ash::vk;
use comptime_register_macro::{register_shader, shaders_registry};

use crate::{geometry::vec3::Vec3, vk_bundles::{DescSetBinding, DeviceBundle, PipelineDescriptor}};

#[register_shader("mesh")]
pub struct ShaderMesh { }

impl ShaderMesh  {
    const GLOBAL_UNIFORMS: bool = false;

    pub fn pipeline_descriptor() -> PipelineDescriptor {
        let ubo_layout_bindings = vec![];

        let vertex_bindings = vec![
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<[f32; 3]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),

            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<[f32; 3]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),

            vk::VertexInputBindingDescription::default()
                .binding(2)
                .stride(std::mem::size_of::<[f32; 3]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ];

        let vertex_attributes = vec![
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT),

            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT),

            vk::VertexInputAttributeDescription::default()
                .binding(2)
                .location(2)
                .format(vk::Format::R32G32B32_SFLOAT),

        ];

        PipelineDescriptor {
            ubo_layout_bindings,
            vertex_bindings,
            vertex_attributes,
        }
    }
}

#[register_shader("special_mesh")]
pub struct ShaderSpecialMesh { }
impl ShaderSpecialMesh  {
    const GLOBAL_UNIFORMS: bool = true;

    pub fn pipeline_descriptor() -> PipelineDescriptor {
        let ubo_layout_bindings = vec![
            DescSetBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::VERTEX,
            }
        ];

        let vertex_bindings = vec![
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<Vec3>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),

            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<Vec3>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),

            vk::VertexInputBindingDescription::default()
                .binding(2)
                .stride(std::mem::size_of::<Vec3>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ];

        let vertex_attributes = vec![
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32B32_SFLOAT),

            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT),

            vk::VertexInputAttributeDescription::default()
                .binding(2)
                .location(2)
                .format(vk::Format::R32G32B32_SFLOAT),

        ];

        PipelineDescriptor {
            ubo_layout_bindings,
            vertex_bindings,
            vertex_attributes,
        }
    }
}


#[register_shader("triangle")]
pub struct ShaderRect {}
impl ShaderRect {
    const GLOBAL_UNIFORMS: bool = false;

    pub fn pipeline_descriptor() -> PipelineDescriptor {
        let ubo_layout_bindings = vec![];

        let vertex_bindings = vec![
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),
            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<[f32; 3]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ];

        let vertex_attributes = vec![
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT),
            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
        ];

        PipelineDescriptor {
            ubo_layout_bindings,
            vertex_bindings,
            vertex_attributes,
        }
    }
}

#[register_shader("texture")]
pub struct ShaderTexture {}

impl ShaderTexture {
    const GLOBAL_UNIFORMS: bool = false;

    pub fn pipeline_descriptor() -> PipelineDescriptor {
        let ubo_layout_bindings = vec![
            DescSetBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::FRAGMENT,
            }
        ];

        let vertex_bindings = vec![
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),

            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ];

        let vertex_attributes = vec![
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT),
            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32_SFLOAT)
        ];

        PipelineDescriptor {
            ubo_layout_bindings,
            vertex_bindings,
            vertex_attributes,
        }
    }
}

pub struct StaticShader {
    pub vert_path: PathBuf,
    pub frag_path: PathBuf,

    pub vert_mt: SystemTime,
    pub frag_mt: SystemTime,

    pub vert_spv_path: PathBuf,
    pub frag_spv_path: PathBuf,

    pub vert_code: Vec<u8>,
    pub frag_code: Vec<u8>,

    pub descriptor: PipelineDescriptor,

    pub id: usize,

    pub global_uniforms: bool
}

pub struct CompiledShader {
    pub details: StaticShader,
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
}

impl StaticShader {

    pub fn reload_and_compile_glsl(device: &DeviceBundle, glsl_path: &PathBuf, spv_path: &PathBuf) -> Option<(Vec<u8>, vk::ShaderModule, SystemTime)> {

        let mt = match StaticShader::file_mt(&glsl_path) {
            Some(mt) => mt,
            None => return None
        };


        if !StaticShader::generate_spv(glsl_path, spv_path) {
            println!("Error: Failed to generate the spv file for shader: {:?}.", glsl_path);
            return None;
        }

        let (code, module) = match StaticShader::reload_and_compile_spv(device, spv_path) {
            Some((code, module)) => (code, module),
            None => return None
        };

        return Some((code, module, mt));
    }

    pub fn reload_and_compile_spv(device: &DeviceBundle, spv_path: &PathBuf) -> Option<(Vec<u8>, vk::ShaderModule)> {
        let code = match StaticShader::read_file(spv_path) {
            Some(code) => code,
            None => {
                println!("Error: Failed to read the spv file for shader: {:?}.", spv_path);
                return None;
            }
        };

        let create_info = vk::ShaderModuleCreateInfo {
            code_size: code.len(),
            p_code: code.as_ptr() as *const u32,
            ..Default::default()
        };

        let module = unsafe { device.logical.create_shader_module(&create_info, None).unwrap() };
        return Some((code, module));
    }


    pub fn outdated(&self) -> bool {
        let mut err = false;
        let mut new = false;

        if let Some(mt) = StaticShader::file_mt(&self.vert_path) {
            new = new || mt != self.vert_mt;
        } else {
            err = true;
        }

        if let Some(mt) = StaticShader::file_mt(&self.frag_path) {
            new = new || mt != self.frag_mt;
        } else {
            err = true;
        }

        return new && !err;
    }

    pub fn changed(file: &PathBuf, file_mt: SystemTime) -> bool {
        if let Some(mt) = StaticShader::file_mt(&file) {
            return mt != file_mt;
        }

        return false;
    }

    fn read_file(file: &PathBuf) -> Option<Vec<u8>> {
        match std::fs::read(file) {
            Ok(data) => Some(data),
            Err(e) => {
                println!(
                    "Error: Failed to read bytes from file: '{:?}: \n\t {}'",
                    file, e
                );
                None
            }
        }
    }

    fn file_mt(file: &PathBuf) -> Option<SystemTime> {
        match std::fs::metadata(file) {
            Ok(res) => match res.modified() {
                Ok(mt) => Some(mt),

                Err(e) => {
                    println!(
                        "Failed to get modified time from path: '{:?}' \n\t {}",
                        file, e
                    );
                    None
                }
            },
            Err(e) => {
                println!("Failed to get metadata from path: '{:?}' \n\t {}", file, e);
                None
            }
        }
    }

    fn generate_spv(src: &PathBuf, dst: &PathBuf) -> bool {
        let mut cmd = std::process::Command::new("glslc");
        cmd.arg(src)
            .arg("-o")
            .arg(dst);

        match cmd.output() {
            Ok(res) => {

                if res.status.success() { true } else {
                    println!("Error: Spv command failed with exit code: {:?}", res.status.code());
                    false
                }


            }
            Err(e) => {
                println!("Error: Spv command failed to execute. \n Cmd: \n\t {:?} \n Err: \n\t {}.", cmd, e);
                false
            }
        }
    }
}

impl CompiledShader {
    pub fn reload_and_compile(&mut self, device: &DeviceBundle) -> bool {

        let ve = std::fs::exists(&self.details.vert_path);
        let ve = if let Ok(v) = ve { v } else { false };

        let fe = std::fs::exists(&self.details.frag_path);
        let fe = if let Ok(v) = fe { v } else { false };

        if !ve {
            println!("Error: File '{:?}' does not exist!", self.details.vert_path);
        }
        if !fe {
            println!("Error: File '{:?}' does not exist!", self.details.frag_path);
        }

        if !ve || !fe {
            return false;
        }

        if StaticShader::changed(&self.details.vert_path, self.details.vert_mt) {

            if let Some((code, module, mt)) = StaticShader::reload_and_compile_glsl(device, &self.details.vert_path, &self.details.vert_spv_path) {
                unsafe { device.logical.destroy_shader_module(self.vert_module, None) };
                self.vert_module = module;
                self.details.vert_code = code;
                self.details.vert_mt = mt;
            }

        }


        if StaticShader::changed(&self.details.frag_path, self.details.frag_mt) {

            if let Some((code, module, mt)) = StaticShader::reload_and_compile_glsl(device, &self.details.frag_path, &self.details.frag_spv_path) {
                unsafe { device.logical.destroy_shader_module(self.frag_module, None) };
                self.frag_module = module;
                self.details.frag_code = code;
                self.details.frag_mt = mt;
            }

        }

        // recompile
        return true;
    }

    pub fn load_from_details(device: &DeviceBundle, details: &StaticShader) -> Result<CompiledShader, String> {

        let vert_mt = StaticShader::file_mt(&details.vert_path);
        let ve = vert_mt.is_some();

        let frag_mt = StaticShader::file_mt(&details.frag_path);
        let fe = frag_mt.is_some();

        if !ve || !fe {
            let mut msg = "Error: The follwing files do not exist!".to_string();
            if !ve {
                msg.push_str(&format!("\n\t {:?}", details.vert_path));
            }

            if !fe {
                msg.push_str(&format!("\n\t {:?}", details.frag_path));
            }

            return Err(msg);
        }

        let vert_mt = match vert_mt {
            Some(mt) => mt,
            None => { return Err("ERROR: LOAD_FROM_DETAILS: UNEXPECTED".to_string()); }
        };

        let frag_mt = match frag_mt {
            Some(mt) => mt,
            None => { return Err("ERROR: LOAD_FROM_DETAILS: UNEXPECTED".to_string()); }
        };


        let vert_result = StaticShader::reload_and_compile_spv(device, &details.vert_spv_path);
        let frag_result = StaticShader::reload_and_compile_spv(device, &details.frag_spv_path);


        if !vert_result.is_some() || !frag_result.is_some() {
            // cleanup and exit

            if let Some((_code, module)) = vert_result {
                unsafe { device.logical.destroy_shader_module(module, None); }
            }

            if let Some((_code, module)) = frag_result {
                unsafe { device.logical.destroy_shader_module(module, None); }
            }

            return Err("Could not create shader modules".to_string());
        }

        match (vert_result, frag_result) {
            (None, None) => {
                Err("Could not create either shader modules".to_string())
            }

            (None, Some((_code, module))) => {
                unsafe { device.logical.destroy_shader_module(module, None); }
                Err("Could not create vertex shader module".to_string())
            }

            (Some((_code, module)), None) => {
                unsafe { device.logical.destroy_shader_module(module, None); }
                Err("Could not create fragment shader module".to_string())
            }

            (Some((vert_code, vert_module)), Some((frag_code, frag_module))) => {
                let details = StaticShader {
                    vert_path: details.vert_path.clone(),
                    frag_path: details.frag_path.clone(),
                    vert_spv_path: details.vert_spv_path.clone(),
                    frag_spv_path: details.frag_spv_path.clone(),
                    vert_mt,
                    frag_mt,
                    vert_code,
                    frag_code,
                    descriptor: details.descriptor.clone(),
                    id: details.id,
                    global_uniforms: details.global_uniforms
                };

                Ok( Self {
                    details,
                    vert_module,
                    frag_module
                })
            }
        }
    }

}

#[shaders_registry]
pub struct ShaderRegistry {

    pub static_shaders: [CompiledShader; ShaderRegistry::SHADER_DETAILS.len()],

}

impl ShaderRegistry {

    pub fn new(device: &DeviceBundle, asset_dir: &str) -> Self {

        let asset_dir = match PathBuf::from_str(asset_dir) {
            Ok(asset_dir) => asset_dir,
            Err(e) => panic!("ShaderRegistry: Failed to get asset dir as path: {}.", e)
        };

        let static_shaders = ShaderRegistry::SHADER_DETAILS.map(|shader_info| {
            ShaderRegistry::get_compiled_shader(device, &asset_dir, shader_info.0, shader_info.1, shader_info.2(), shader_info.3)
        });

        Self {
            static_shaders
        }
    }

    pub fn get_compiled_shader(device: &DeviceBundle, asset_dir: &PathBuf, name: &str, id: usize, descriptor: PipelineDescriptor, global_uniforms: bool) -> CompiledShader {

        let name = name.to_string();

        let vert_name = name.clone() + ".vert";
        let vert_path = asset_dir.join(vert_name);
        let vert_spv_name = name.clone() + ".vert.spv";
        let vert_spv_path = asset_dir.join(vert_spv_name);
        let vert_mt = match StaticShader::file_mt(&vert_path) {
            Some(mt) => mt,
            None => SystemTime::UNIX_EPOCH
        };

        let frag_name = name.clone() + ".frag";
        let frag_path = asset_dir.join(frag_name);
        let frag_spv_name = name.clone() + ".frag.spv";
        let frag_spv_path = asset_dir.join(frag_spv_name);
        let frag_mt = match StaticShader::file_mt(&frag_path) {
            Some(mt) => mt,
            None => SystemTime::UNIX_EPOCH
        };

        let details = StaticShader {
            vert_path,
            frag_path,
            vert_mt,
            frag_mt,
            vert_spv_path,
            frag_spv_path,
            vert_code: Vec::new(),
            frag_code: Vec::new(),
            descriptor,
            id,
            global_uniforms
        };

        let compiled_shader = match CompiledShader::load_from_details(device, &details) {
            Ok(compiled_shader) => compiled_shader,
            Err(e) => panic!("ShaderRegistry: Failed to compile the shader ({}) : {}.", name, e)
        };

        return compiled_shader;
    }


    pub fn describe_registed_shaders() {

        // created by the shader_registry proc_macro_attribute of the struct
        println!();
        println!("Shader Registry - {} Shaders Registered:", ShaderRegistry::SHADER_DETAILS.len());
        for (name, id, _, _) in ShaderRegistry::SHADER_DETAILS {
            println!("\t Shader {} ({})", name, id);
        }
        println!();

    }

}
