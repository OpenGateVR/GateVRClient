use cgmath::*;
use winit::window::Window;
use std::iter;
use wgpu::BindGroup;
use rust_embed::RustEmbed;
use image::{DynamicImage, GenericImageView};
use std::collections::HashMap;

use crate::interract::raycast::raycast_grab;
use crate::renderer::transforms;
use crate::renderer::vertex::Vertex;
use crate::setup::fonts::load_font_atlas;
use crate::world::object::{Object, ObjectType};
use crate::world::world::World;

pub struct TextureObject {
    texture: wgpu::Texture,
    texture_size: wgpu::Extent3d,
    texture_rgba: Vec<u8>,
    texture_width: u32,
    texture_height: u32
}
impl TextureObject {
    pub fn create(path: &str, init: &transforms::InitWgpu) -> Self {
        let texture_data = Assets::get(path).expect("Failed to load embedded texture");
        let img = image::load_from_memory(&texture_data.data).expect("Failed to load texture");
        println!("loaded {}", path);
        let texture_rgba = img.to_rgba8().to_vec();
        let (width, height) = img.dimensions();
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture: wgpu::Texture = init.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Self {
            texture,
            texture_size,
            texture_rgba,
            texture_width: width,
            texture_height: height
        }
    }

    pub fn load_from_dynamic_image(img: DynamicImage, init: &transforms::InitWgpu) -> Self {
        let texture_rgba = img.to_rgba8().to_vec();
        let (width, height) = img.dimensions();
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture: wgpu::Texture = init.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Self {
            texture,
            texture_size,
            texture_rgba,
            texture_width: width,
            texture_height: height
        }
    }
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub struct Renderer {
    pub init: transforms::InitWgpu,
    project_mat: Matrix4<f32>,

    pipeline_displacement: wgpu::RenderPipeline,

    frame: usize,
    previous_frame_time: std::time::Instant,

    vertex_buffer: Vec<wgpu::Buffer>,
    uniform_bind_group: Vec<wgpu::BindGroup>,
    num_vertices: Vec<u32>,

    uniform_bind_group_layout: wgpu::BindGroupLayout,
    vertex_uniform_buffer: wgpu::Buffer,
    model_uniform_buffers: Vec<wgpu::Buffer>,
    fragment_uniform_buffer: wgpu::Buffer,

    textures: HashMap<String, TextureObject>,

    // the client position and rotation
    camera_position: (f32, f32, f32),
    camera_rotation: (f32, f32, f32),
    camera_acceleration_walking: (f32, f32, f32),

    objects: Vec<Object>,
    cameras: Vec<Object>,
    current_camera: usize
}
impl Renderer {
    fn create_buffer_displacement(
        init: &transforms::InitWgpu, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout, 
        vertex_uniform_buffer: &wgpu::Buffer, fragment_uniform_buffer: &wgpu::Buffer, model_uniform_buffer: &wgpu::Buffer,
        displacement_texture: &wgpu::Texture, displacement_texture_size: wgpu::Extent3d, 
        displacement_rgba: &Vec<u8>, displacement_width: u32, displacement_height: u32,
        texture: &wgpu::Texture, texture_size: wgpu::Extent3d, rgba: &Vec<u8>, width: u32, height: u32, vertex_num: usize,
    ) -> (BindGroup, wgpu::Buffer) {
        init.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        init.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &displacement_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &displacement_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * displacement_width),
                rows_per_image: Some(displacement_height),
            },
            displacement_texture_size,
        );
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let displacement_texture_view = displacement_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = init.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: fragment_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&displacement_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: model_uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("Uniform Bind Group"),
        });

        let max_buffer_size = size_of::<Vertex>() * vertex_num; // 8MB buffer
        let vertex_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: max_buffer_size as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        return (uniform_bind_group, vertex_buffer)
    }

    pub async fn new(window: &Window) -> Self {
        let init =  transforms::InitWgpu::init_wgpu(window).await;

        let camera_position: (f32, f32, f32) = (-10.0, 4.0, 0.0);
        let camera_rotation: (f32, f32, f32) = (0.0, 0.0, 0.0);

        let (_, project_mat, _) = transforms::create_view_projection(
            camera_position.into(), camera_rotation.into(), cgmath::Vector3::unit_y(), 
            init.config.width as f32 / init.config.height as f32);

        let uniform_bind_group_layout: wgpu::BindGroupLayout = init.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("Uniform Bind Group Layout"),
        });

        let pipeline_layout = init.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader_displacement = init.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/displacement.wgsl").into()),
        });

        let pipeline_displacement = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_displacement,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_displacement,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: init.config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            //depth_stencil: None,
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None
        });

        let vertex_uniform_buffer: wgpu::Buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Vertex Uniform Buffer"),
            size: 192,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let fragment_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Fragment Uniform Buffer"),
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let model_mat = transforms::create_transforms(
            [0.0, 0.0, 0.0], 
            [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]
        );
        let normal_mat = (model_mat.invert().unwrap()).transpose();

        let model_ref:&[f32; 16] = model_mat.as_ref();
        let normal_ref:&[f32; 16] = normal_mat.as_ref();
        init.queue.write_buffer(&vertex_uniform_buffer, 0, bytemuck::cast_slice(model_ref));
        init.queue.write_buffer(&vertex_uniform_buffer, 128, bytemuck::cast_slice(normal_ref));

        let mut textures: HashMap<String, TextureObject> = HashMap::new();

        // create font atlasses
        textures.insert("fonts/NotoSansJP.ttf".to_string(), TextureObject::load_from_dynamic_image(load_font_atlas("fonts/NotoSansJP.ttf"), &init));

        textures.insert("textures/atlas.png".to_string(), TextureObject::create("textures/atlas.png", &init));
        textures.insert("textures/wood.jpg".to_string(), TextureObject::create("textures/wood.jpg", &init));
        textures.insert("textures/table.png".to_string(), TextureObject::create("textures/table.png", &init));
        textures.insert("textures/ground.jpg".to_string(), TextureObject::create("textures/ground.jpg", &init));
        textures.insert("textures/ground_displacement.png".to_string(), TextureObject::create("textures/ground_displacement.png", &init));
        textures.insert("textures/displacement.png".to_string(), TextureObject::create("textures/displacement.png", &init));
        textures.insert("textures/niko.png".to_string(), TextureObject::create("textures/niko.png", &init));
        textures.insert("textures/wall.jpg".to_string(), TextureObject::create("textures/wall.jpg", &init));
        textures.insert("textures/skybox_1.png".to_string(), TextureObject::create("textures/skybox_1.png", &init));
        textures.insert("textures/skybox_2.png".to_string(), TextureObject::create("textures/skybox_2.png", &init));
        textures.insert("textures/Selestia_costume.png".to_string(), TextureObject::create("textures/Selestia_costume.png", &init));

        let vertex_buffer = Vec::new();
        let uniform_bind_group = Vec::new();
        let num_vertices = Vec::new();

        let previous_frame_time = std::time::Instant::now();

        Self {
            init,
            project_mat,

            pipeline_displacement,

            frame: 0,
            previous_frame_time,

            vertex_buffer,
            uniform_bind_group,
            num_vertices,

            uniform_bind_group_layout,
            vertex_uniform_buffer,
            model_uniform_buffers: Vec::new(),
            fragment_uniform_buffer,

            textures,

            camera_position,
            camera_rotation,
            camera_acceleration_walking: (0.0, 0.0, 0.0),

            objects: Vec::new(),
            cameras: Vec::new(),
            current_camera: 0
        }
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.instance.poll_all(true);
            self.init.size = new_size;
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init.surface.configure(&self.init.device, &self.init.config);
            self.project_mat = transforms::create_projection(new_size.width as f32 / new_size.height as f32);
        }
    }
    pub fn update(&mut self, _dt: std::time::Duration, keys: [bool; 6], mouse: [f64; 2], menu_tablet_state: usize) {
        //self.camera_rotation.1 = dt.as_secs_f32();
        let current_time = std::time::Instant::now();
        let mut frame_time = current_time.duration_since(self.previous_frame_time).as_secs_f32() * 20.0;
        self.previous_frame_time = current_time;

        if frame_time > 5.0 {
            frame_time = 5.0
        }

        self.camera_rotation.1 -= mouse[0] as f32 * (frame_time * 0.02);
        self.camera_rotation.0 += mouse[1] as f32 * (frame_time * 0.02);
        self.camera_rotation.0 = self.camera_rotation.0.clamp(-std::f32::consts::FRAC_PI_2 / 1.01, std::f32::consts::FRAC_PI_2 / 1.01);

        let forward = Vector3::new(
            self.camera_rotation.1.cos() * self.camera_rotation.0.cos(),
            self.camera_rotation.0.sin(),
            self.camera_rotation.1.sin() * self.camera_rotation.0.cos(),
        ).normalize();
        let right = Vector3::new(
            self.camera_rotation.1.sin(),
            0.0,
            -self.camera_rotation.1.cos(),
        ).normalize();

        self.camera_acceleration_walking = (0.0, self.camera_acceleration_walking.1, 0.0).into();
        if keys[0] {
            self.camera_acceleration_walking.0 += frame_time * forward[0];
            self.camera_acceleration_walking.2 += frame_time * forward[2];
        }
        if keys[2] {
            self.camera_acceleration_walking.0 -= frame_time * forward[0];
            self.camera_acceleration_walking.2 -= frame_time * forward[2];
        }
        if keys[1] {
            self.camera_acceleration_walking.0 += frame_time * right[0];
            self.camera_acceleration_walking.2 += frame_time * right[2];
        }
        if keys[3] {
            self.camera_acceleration_walking.0 -= frame_time * right[0];
            self.camera_acceleration_walking.2 -= frame_time * right[2];
        }

        self.camera_position.0 += self.camera_acceleration_walking.0 * 1.0 * frame_time;
        self.camera_position.1 += self.camera_acceleration_walking.1 * 1.0 * frame_time;
        self.camera_position.2 += self.camera_acceleration_walking.2 * 1.0 * frame_time;

        if menu_tablet_state == 2 {
            for i in 0..self.objects.len() {
                if self.objects[i].get_object_type() == ObjectType::TabletMenu {
                    let model_mat = transforms::create_transforms(
                        [
                            self.camera_position.0 + forward.x * 2.0, 
                            self.camera_position.1 + forward.y * 2.0, 
                            self.camera_position.2 + forward.z * 2.0
                            ], 
                        [
                            -self.camera_rotation.0, 
                            -self.camera_rotation.1 + std::f32::consts::FRAC_PI_2, 
                            -self.camera_rotation.2
                            ], [1.0, 1.0, 1.0]
                    );
                    let normal_mat = (model_mat.invert().unwrap()).transpose();

                    let model_ref:&[f32; 16] = model_mat.as_ref();
                    let normal_ref:&[f32; 16] = normal_mat.as_ref();

                    self.init.queue.write_buffer(&self.model_uniform_buffers[i], 0, bytemuck::cast_slice(model_ref));
                    self.init.queue.write_buffer(&self.model_uniform_buffers[i], 64, bytemuck::cast_slice(normal_ref));
                }
            }
        } else if menu_tablet_state == 3 {
            for i in 0..self.objects.len() {
                if self.objects[i].get_object_type() == ObjectType::TabletMenu {
                    let model_mat = transforms::create_transforms(
                        [0.0, -10.0, 0.0], 
                        [
                            -self.camera_rotation.0, 
                            -self.camera_rotation.1 + std::f32::consts::FRAC_PI_2, 
                            -self.camera_rotation.2
                            ], [1.0, 1.0, 1.0]
                    );
                    let normal_mat = (model_mat.invert().unwrap()).transpose();

                    let model_ref:&[f32; 16] = model_mat.as_ref();
                    let normal_ref:&[f32; 16] = normal_mat.as_ref();

                    self.init.queue.write_buffer(&self.model_uniform_buffers[i], 0, bytemuck::cast_slice(model_ref));
                    self.init.queue.write_buffer(&self.model_uniform_buffers[i], 64, bytemuck::cast_slice(normal_ref));
                }
            }
        }

        // update skybox positions
        if self.frame % 10 == 0 {
            for i in 0..self.objects.len() {
                if self.objects[i].get_object_type() == ObjectType::Skybox {
                    let model_mat = transforms::create_transforms(
                        [self.camera_position.0, self.camera_position.1, self.camera_position.2], 
                        [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]
                    );
                    let normal_mat = (model_mat.invert().unwrap()).transpose();

                    let model_ref:&[f32; 16] = model_mat.as_ref();
                    let normal_ref:&[f32; 16] = normal_mat.as_ref();
                    let eye_position:&[f32; 3] = &self.camera_position.into();
                    self.init.queue.write_buffer(&self.fragment_uniform_buffer, 16, bytemuck::cast_slice(eye_position));
                    self.init.queue.write_buffer(&self.model_uniform_buffers[i], 0, bytemuck::cast_slice(model_ref));
                    self.init.queue.write_buffer(&self.model_uniform_buffers[i], 64, bytemuck::cast_slice(normal_ref));
                }
            }

            let grabbable_object_index = raycast_grab(&mut self.objects, self.camera_position, forward, 5);
            if grabbable_object_index > 0 {
                let y_rotation = self.objects[grabbable_object_index].get_rotation().1;
                self.objects[grabbable_object_index].set_rotation_y(y_rotation + 0.1);
                let model_mat = transforms::create_transforms(
                    [0.0, 0.0, 0.0], 
                    [0.0, y_rotation + 0.1, 0.0], [1.0, 1.0, 1.0]
                );
                let normal_mat = (model_mat.invert().unwrap()).transpose();

                let model_ref:&[f32; 16] = model_mat.as_ref();
                let normal_ref:&[f32; 16] = normal_mat.as_ref();
                let eye_position:&[f32; 3] = &self.camera_position.into();
                self.init.queue.write_buffer(&self.fragment_uniform_buffer, 16, bytemuck::cast_slice(eye_position));
                self.init.queue.write_buffer(&self.model_uniform_buffers[grabbable_object_index], 0, bytemuck::cast_slice(model_ref));
                self.init.queue.write_buffer(&self.model_uniform_buffers[grabbable_object_index], 64, bytemuck::cast_slice(normal_ref));
            }
        }

        let up_direction = cgmath::Vector3::unit_y();
        let (view_mat, project_mat, _) = transforms::create_view_rotation(
            self.camera_position.into(), self.camera_rotation.1, self.camera_rotation.0, 
            up_direction, self.init.config.width as f32 / self.init.config.height as f32);

        let view_project_mat = project_mat * view_mat;
        let view_projection_ref:&[f32; 16] = view_project_mat.as_ref();
        
        self.init.queue.write_buffer(&self.vertex_uniform_buffer, 64, bytemuck::cast_slice(view_projection_ref));

        let current_time_updated = std::time::Instant::now();
        let update_time = current_time_updated.duration_since(current_time).as_secs_f32();

        if false {
            println!("fps: {}", 1.0 / update_time);
        }

        self.frame += 1;
    }

    // replace all objects in the world
    pub fn set_objects(&mut self, world: &World) {
        let objects = world.get_objects().clone();
        let cameras = world.get_cameras().clone();

        self.vertex_buffer.clear();
        self.uniform_bind_group.clear();
        self.num_vertices.clear();

        for object in &objects {
            let vertices = object.get_vertices();

            if let Some(texture) = self.textures.get(object.get_texture()) {
                let model_uniform_buffer: wgpu::Buffer = self.init.device.create_buffer(&wgpu::BufferDescriptor{
                    label: Some("Vertex Uniform Buffer"),
                    size: 128,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

                let model_mat = transforms::create_transforms([
                    0.0, 0.0, 0.0], 
                    [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]
                );
                let normal_mat = (model_mat.invert().unwrap()).transpose();

                let model_ref:&[f32; 16] = model_mat.as_ref();
                let normal_ref:&[f32; 16] = normal_mat.as_ref();
                self.init.queue.write_buffer(&model_uniform_buffer, 0, bytemuck::cast_slice(model_ref));
                self.init.queue.write_buffer(&model_uniform_buffer, 64, bytemuck::cast_slice(normal_ref));

                if let Some(textue_displacement) = self.textures.get(object.get_displacement()) {
                    let (uniform_bind_group, vertex_buffer) = Self::create_buffer_displacement(
                        &self.init, &self.uniform_bind_group_layout, 
                        &self.vertex_uniform_buffer, &self.fragment_uniform_buffer,
                        &model_uniform_buffer, &textue_displacement.texture, textue_displacement.texture_size, 
                        &textue_displacement.texture_rgba, 
                        textue_displacement.texture_width, textue_displacement.texture_height,
                        &texture.texture, texture.texture_size, &texture.texture_rgba, 
                        texture.texture_width, texture.texture_height, vertices.len(),
                    );
                    
                    self.vertex_buffer.push(vertex_buffer);
                    self.uniform_bind_group.push(uniform_bind_group);

                    self.num_vertices.push(vertices.len() as u32);
                    self.init.queue.write_buffer(&self.vertex_buffer[self.vertex_buffer.len() - 1], 0, bytemuck::cast_slice(vertices));
                } else if let Some(textue_displacement) = self.textures.get("textures/displacement.png") {
                    let (uniform_bind_group, vertex_buffer) = Self::create_buffer_displacement(
                        &self.init, &self.uniform_bind_group_layout, 
                        &self.vertex_uniform_buffer, &self.fragment_uniform_buffer,
                        &model_uniform_buffer, &textue_displacement.texture, textue_displacement.texture_size, 
                        &textue_displacement.texture_rgba, 
                        textue_displacement.texture_width, textue_displacement.texture_height,
                        &texture.texture, texture.texture_size, &texture.texture_rgba, 
                        texture.texture_width, texture.texture_height, vertices.len(),
                    );
                    
                    self.vertex_buffer.push(vertex_buffer);
                    self.uniform_bind_group.push(uniform_bind_group);

                    self.num_vertices.push(vertices.len() as u32);
                    self.init.queue.write_buffer(&self.vertex_buffer[self.vertex_buffer.len() - 1], 0, bytemuck::cast_slice(vertices));
                }
                self.model_uniform_buffers.push(model_uniform_buffer);
            }
        }

        self.objects = objects;
        self.cameras = cameras;

        if self.cameras.len() > self.current_camera {
            let object_position = self.cameras[self.current_camera].get_position();
            self.camera_position.0 = object_position.0 as f32;
            self.camera_position.1 = object_position.1 as f32;
            self.camera_position.2 = object_position.2 as f32;
            self.camera_rotation = self.cameras[self.current_camera].get_rotation();
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //let output = self.init.surface.get_current_frame()?.output;
        let output = self.init.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.init.config.width,
                height: self.init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format:wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .init.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.247,
                            b: 0.314,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })], 
                //depth_stencil_attachment: None,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.pipeline_displacement);
            for i in 0..self.vertex_buffer.len() {
                render_pass.set_vertex_buffer(0, self.vertex_buffer[i].slice(..));           
                render_pass.set_bind_group(0, &self.uniform_bind_group[i], &[]);
                render_pass.draw(0..self.num_vertices[i], 0..1);
            }
        }

        self.init.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}