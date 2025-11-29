#![warn(clippy::restriction)]
#![allow(clippy::separated_literal_suffix, reason = "WHTF")]
#![allow(
    clippy::question_mark,
    reason = "for a cleaner syntax errors are bubbled up"
)]
#![allow(
    clippy::question_mark_used,
    reason = "for a cleaner syntax errors are bubbled up"
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    reason = "i want to do it as it says"
)]
// #![warn(clippy::pedantic)]

extern crate alloc;
use crate::{
    config::{Configuration as _, DefaultConf as Conf},
    model,
};
use alloc::sync::Arc;
use core::error::Error;
use core::mem;
use std::time::{self, Instant};
use tracing::instrument;
use wgpu::util::DeviceExt as _;
use wgpu::{
    Adapter, Backends, BindGroup, BlendState, Buffer, BufferUsages, Color, ColorTargetState,
    ColorWrites, CommandEncoderDescriptor, CompositeAlphaMode, Device, FragmentState, IndexFormat,
    Instance, InstanceDescriptor, LoadOp, Operations, PipelineCompilationOptions, PipelineLayout,
    PipelineLayoutDescriptor, PresentMode, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, StoreOp, Surface, SurfaceConfiguration,
    TextureFormat, TextureUsages, TextureViewDescriptor, VertexState, include_wgsl,
};
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug)]
#[non_exhaustive]
pub struct State {
    pub adapter: Adapter,
    pub config: SurfaceConfiguration,
    pub device: Device,
    pub index_buffer: wgpu::Buffer,
    pub instance: Instance,
    pub num_indices: u32,
    pub num_vertices: u32,
    pub pipeline_layout: PipelineLayout,
    pub queue: Queue,
    pub render_pipeline: RenderPipeline,
    pub shader: ShaderModule,
    pub size: PhysicalSize<u32>,
    pub start_time: time::Instant,
    pub surface: Surface<'static>,
    pub surface_format: TextureFormat,
    pub uniforms: Uniforms,
    pub uniforms_bind_group: BindGroup,
    pub uniforms_buffer: Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub window: Arc<Window>,
}

impl State {
    #[instrument]
    pub async fn new(window: Arc<Window>) -> Result<State, Box<dyn Error>> {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let size = &window.inner_size();
        #[expect(clippy::expect_used, reason = "unrecoverable")]
        let surface = instance
            .create_surface(Arc::clone(&window))
            .expect("Can't use the current window to render");

        #[expect(clippy::expect_used, reason = "unrecoverable")]
        let adapter = instance
            .request_adapter(&Conf::adapter(&surface))
            .await
            .expect("can't request an adapter, shutting down");

        #[expect(clippy::expect_used, reason = "unrecoverable")]
        let (device, queue) = adapter
            .request_device(&Conf::device_dec(adapter.limits()))
            .await
            .expect("can't get a device representatio, shutting down");

        let config = Conf::surface_config(&adapter, &surface, size);
        surface.configure(&device, &config);

        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));

        #[expect(clippy::as_conversions, reason = "there is no usize bigger than u64")]
        let buffer = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<model::Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // positions
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // colors
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let uniforms = Uniforms {
            screen_size: [0.0, 0.0],
            time: 0.0,
            _padding: 0.0,
        };

        let uniforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms_buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniforms_bindgroup_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniforms_bindgroup_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniforms_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniforms_bind_group"),
            layout: &uniforms_bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&uniforms_bindgroup_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[buffer],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: Conf::primitives(),
            depth_stencil: None,
            multisample: Conf::multisample(),
            multiview: None,
            cache: None,
            label: Some("render_pipeline"),
        });

        let vertices = model::vertices();
        #[expect(clippy::expect_used, reason = "Unrecoverable error")]
        let num_vertices =
            u32::try_from(vertices.len()).expect("Couldn't convert from vertices.len() to u32");

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let indices = model::INDICES;
        #[expect(clippy::expect_used, reason = "Unrecoverable error")]
        let num_indices =
            u32::try_from(indices.len()).expect("Couldn't convert from indices.len() to u32");

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Ok(State {
            start_time: Instant::now(),
            uniforms,
            surface_format: config.format,
            instance,
            adapter,
            shader,
            pipeline_layout,
            uniforms_buffer,
            uniforms_bind_group,
            render_pipeline,
            size: *size,
            device,
            queue,
            surface,
            window,
            config,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
        })
    }

    pub fn configure_surface(&self) {
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    /// # Panics
    pub fn render(&mut self) {
        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(e) => {
                #[cfg(debug_assertions)]
                panic!("Hay dio quejesto:\n{e:#?}");
                #[expect(unused, reason = "")]
                return ();
            }
        };

        let view = frame.texture.create_view(&TextureViewDescriptor {
            format: Some(self.surface_format.add_srgb_suffix()),
            ..Default::default()
        });

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        let mut renderpass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: f64::from(109_i32 / 255_i32),
                        g: f64::from(208_i32 / 255_i32),
                        b: f64::from(250_i32 / 255_i32),
                        a: 0.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        let screen_size = [self.size.width as f32, self.size.height as f32];

        self.uniforms = Uniforms {
            screen_size: [screen_size[0], screen_size[1]],
            time: self.start_time.elapsed().as_secs_f32(),
            _padding: 0.0,
        };

        self.queue.write_buffer(
            &self.uniforms_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );

        renderpass.set_pipeline(&self.render_pipeline); // 2.
        renderpass.set_bind_group(0, &self.uniforms_bind_group, &[]);
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        renderpass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
        renderpass.draw_indexed(0..self.num_indices, 0, 0..1);

        drop(renderpass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        frame.present();
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;

        // reconfigure the surface
        self.configure_surface();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    screen_size: [f32; 2],
    time: f32,
    _padding: f32,
}

struct hay;
