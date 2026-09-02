use crate::ShaderMesh;
use crate::ShaderRect;
use crate::ShaderTexture;
use crate::components::sliding_texture::SlidingTexture;
use crate::devices::record_player::RecordPlayer;
use crate::drawable::{drawable_mesh::DrawableMesh, drawable_tex::DrawableTexture, drawable2d::Drawable2d};
use crate::mesh::{ RectMesh, cube};
use crate::primitives::{rect::Rect, image::{PixelFormat, Image}};
use crate::utils::{image::{begin_single_time_command, end_single_time_command}};
use crate::vk_base::VkBase;

use ash::vk;

const USE_AUDIO_BUFFER: bool = false;

pub struct DemoScene {
    video_device: RecordPlayer,
    rect_bundles: Vec<Drawable2d>,
    mesh_bundles: Vec<DrawableMesh>,
    textures: Vec<DrawableTexture>,
    sliding_textures: Vec<SlidingTexture>,
}

impl DemoScene {
    pub fn new(base: &VkBase) -> Self {

        let mut video_device = if USE_AUDIO_BUFFER {
            RecordPlayer::from_buffer(include_bytes!("../../assets/recordings/record1.rdbin")).unwrap()
        } else {
            RecordPlayer::new("./assets/recordings/record1.rdbin").unwrap()
        };

        video_device.reset_player();

        let rect_bundles = vec![
            Drawable2d::new(&base.device, RectMesh::new(-0.9, -0.9, 0.5, 0.5, [1.0, 0.0, 0.0])),
            Drawable2d::new(&base.device, RectMesh::new(0.0, 0.0, 0.5, 0.5, [0.0, 0.0, 1.0])),
            Drawable2d::new(&base.device, RectMesh::new(-0.25, -0.25, 0.5, 0.5, [0.0, 1.0, 1.0]))
        ];

        let mesh_bundles = vec![
            DrawableMesh::new(&base.device, cube::make_cube(0.0, 0.0, 0.25, 0.5, [1.0, 0.2, 1.0]))
        ];

        let data = unsafe { video_device.current_frame[0..video_device.size() / 2].align_to::<u8>().1.to_vec() };
        let texture = Image::new(data, video_device.width(), video_device.height(), video_device.format());

        let cb = begin_single_time_command(&base.device, base.spare_command.pool);

        let textures = vec![
            DrawableTexture::new(&base, cb, Rect::new(0.75, -1.0, 0.25, 0.25), Rect::new(0.0, 0.0, 1.0, 1.0), texture)
        ];

        let data = unsafe { video_device.current_frame[0..video_device.size() / 2].align_to::<u8>().1.to_vec() };
        let texture = Image::new(data, video_device.width(), video_device.height(), video_device.format());

        let atlas = farbfeld_image::load_ff("assets/fonts/Atlas-Iosevka-Regular.ff").expect("Could not find font atlas.");
        let atlas_texture = Image::new(atlas.data, atlas.w, atlas.h, PixelFormat::RGBA);

        let sliding_textures = vec![
            SlidingTexture::new(DrawableTexture::new(&base, cb, Rect::new(0.00, -1.0, 0.4, 0.4), Rect::new(0.0, 0.0, 0.2, 0.2), texture), 5.0),
            SlidingTexture::new(DrawableTexture::new(&base, cb, Rect::new(0.25, -1.0, 0.4, 0.4), Rect::new(0.0, 0.0, 0.2, 0.2), atlas_texture), 5.0)
        ];

        end_single_time_command(&base.device, base.spare_command.pool, base.device.present_queue, cb);

        Self {
            video_device,
            rect_bundles,
            mesh_bundles,
            textures,
            sliding_textures,
        }
    }

    pub fn update(&mut self, base: &VkBase, cb: vk::CommandBuffer, delta_time: f32) {
        for mesh_bundle in self.rect_bundles.iter_mut() {
            mesh_bundle.mesh.transform(0.001, [0.0, 0.0]);
        }

        Drawable2d::update(&base.device, cb, &mut self.rect_bundles);
        DrawableMesh::update(&base.device, cb, &mut self.mesh_bundles);

        if let Some(new_frame) = self.video_device.poll() {
            self.textures[0].texture_data.copy_to_data(&new_frame);
        }

        DrawableTexture::update(&base.device, cb, &mut self.textures);
        SlidingTexture::update(&base.device, cb, delta_time, &mut self.sliding_textures);
    }

    pub fn draw(&self, base: &VkBase, cb: vk::CommandBuffer) {
        DrawableTexture::draw(&base.device, cb, &base.graphics_pipelines[ShaderTexture::ID], base.current_frame, &self.textures);
        SlidingTexture::draw(&base.device, cb, &base.graphics_pipelines[ShaderTexture::ID], base.current_frame, &self.sliding_textures);
        Drawable2d::draw(&base.device, cb, &base.graphics_pipelines[ShaderRect::ID], &self.rect_bundles);
        DrawableMesh::draw(&base.device, cb, &base.graphics_pipelines[ShaderMesh::ID], &self.mesh_bundles);
    }

    pub fn release(&mut self, base: &VkBase) {
        Drawable2d::release(&base.device, &mut self.rect_bundles);
        self.rect_bundles.clear();

        DrawableMesh::release(&base.device, &mut self.mesh_bundles);
        self.mesh_bundles.clear();

        DrawableTexture::release(&base.device, &mut self.textures);
        self.textures.clear();

        SlidingTexture::release(&base.device, &mut self.sliding_textures);
        self.sliding_textures.clear();
    }
}
