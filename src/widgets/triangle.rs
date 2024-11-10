use cgmath::Vector4;
use wgpu::{util::DeviceExt, RenderPipeline};

use crate::render::context::{RenderContext, Renderable};

use super::Widget;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Uniform {
    view_proj: [[f32; 4]; 4],
}

impl Uniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, mat: &cgmath::Matrix4<f32>) {
        self.view_proj = (*mat * OPENGL_TO_WGPU_MATRIX).into();
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Default)]
pub struct Triangle {
    vertices: [Vertex; 3],
    render_pipeline: Option<RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    uniform_buffer: Option<wgpu::Buffer>,
    uniform_bind_group: Option<wgpu::BindGroup>,
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self {
            vertices,
            ..Default::default()
        }
    }
    fn matrix(&mut self, data: &super::WidgetData) -> cgmath::Matrix4<f32> {
        let mut mat = cgmath::Matrix4::from_translation(cgmath::Vector3 {
            x: (data.position.x + data.global_pos.x) / data.size.width as f32,
            y: (data.position.y + data.global_pos.y) / data.size.height as f32,
            z: data.position.z,
        });
        let proj = cgmath::Matrix4 {
            x: Vector4::new(1. / data.size.width as f32, 0., 0., 0.),
            y: Vector4::new(0., 1. / data.size.height as f32, 0., 0.),
            z: Vector4::new(0., 0., 1., 0.),
            w: Vector4::new(0., 0., 0., 1.),
        };
        mat = mat * proj;
        mat
    }
}

impl Widget for Triangle {
    fn init_widget(&mut self, render_context: &mut RenderContext, data: &super::WidgetData) {
        let shader = render_context
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("triangle_shader.wgsl").into()),
            });
        let mut uniform = Uniform::new();
        uniform.update_view_proj(&self.matrix(data));
        let uniform_buffer =
            render_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        let uniform_bind_group_layout =
            render_context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("uniform_bind_group_layout"),
                });
        let uniform_bind_group =
            render_context
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &uniform_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }],
                    label: Some("uniform_bind_group"),
                });
        let render_pipeline_layout =
            render_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&uniform_bind_group_layout],
                    push_constant_ranges: &[],
                });
        let render_pipeline =
            render_context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        buffers: &[Vertex::desc()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: render_context.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });
        let vertex_buffer =
            render_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&self.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        self.render_pipeline = Some(render_pipeline);
        self.vertex_buffer = Some(vertex_buffer);
        self.uniform_buffer = Some(uniform_buffer);
        self.uniform_bind_group = Some(uniform_bind_group);
    }
    fn update_data(&mut self, render_context: &mut RenderContext, data: &super::WidgetData) {
        let mut uniform = Uniform::new();

        uniform.update_view_proj(&self.matrix(data));
        render_context.queue.write_buffer(
            self.uniform_buffer
                .as_ref()
                .expect("Triangle is not inited"),
            0,
            bytemuck::cast_slice(&[uniform]),
        );
    }
    fn render_widget<'a>(&mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(
            self.render_pipeline
                .as_ref()
                .expect("Triangle is not inited"),
        );
        let vertex_buffer = self.vertex_buffer.as_ref().expect("Triangle is not inited");

        render_pass.set_bind_group(
            0,
            self.uniform_bind_group
                .as_ref()
                .expect("Triangle is not inited"),
            &[],
        );
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }
}
