use std::collections::HashMap;

use winit::dpi::PhysicalSize;

use crate::render::context::{RenderContext, Renderable};

pub mod triangle;

pub struct WidgetDesc {
    pub widget: Box<dyn Widget>,
    pub data: WidgetData,
}
pub struct WidgetData {
    pub args: HashMap<String, String>,
    pub position: cgmath::Vector3<f32>,
    pub global_pos: cgmath::Vector3<f32>,
    pub size: PhysicalSize<u32>,
}

impl Renderable for WidgetDesc {
    fn init(&mut self, render_context: &mut crate::render::context::RenderContext) {
        self.widget.init_widget(render_context, &self.data);
    }
    fn render<'a>(&mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.widget.render_widget(render_pass);
    }
}

pub trait Widget {
    fn init_widget(&mut self, render_context: &mut RenderContext, data: &WidgetData);
    fn update_data(&mut self, render_context: &mut RenderContext, data: &WidgetData);
    fn render_widget<'a>(&mut self, render_pass: &mut wgpu::RenderPass<'a>);
}
