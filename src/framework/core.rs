use std::collections::HashMap;

use crate::render::context::RenderContext;
use crate::widgets::triangle::{Triangle, Vertex};
use crate::widgets::{WidgetData, WidgetDesc};
use pollster::block_on;
use wgpu::Color;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub struct Framework;

impl Framework {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        block_on(self.async_run())
    }

    pub async fn async_run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let mut renderables: Vec<WidgetDesc> = Vec::new();
        renderables.push(WidgetDesc {
            widget: Box::new(Triangle::new(VERTICES.clone())),
            data: WidgetData {
                args: HashMap::new(),
                position: cgmath::Vector3::<f32>::new(400., -400., 0.),
                global_pos: cgmath::Vector3::<f32>::new(0., 0., 0.),
                size: window.inner_size(),
            },
        });
        renderables.push(WidgetDesc {
            widget: Box::new(Triangle::new(VERTICES.clone())),
            data: WidgetData {
                args: HashMap::new(),
                position: cgmath::Vector3::<f32>::new(0., 0., 0.),
                global_pos: cgmath::Vector3::<f32>::new(0., 0., 0.),
                size: window.inner_size(),
            },
        });
        let mut render_context = RenderContext::new(&window, renderables, Color::WHITE).await;

        match event_loop.run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == render_context.window().id() => {
                //
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::Resized(physical_size) => {
                        render_context.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        // This tells winit that we want another frame after this one
                        render_context.window().request_redraw();

                        // Updating
                        match render_context.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                render_context.resize(render_context.size)
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                log::error!("OutOfMemory");
                                control_flow.exit();
                            }

                            // This happens when the a frame takes too long to present
                            Err(wgpu::SurfaceError::Timeout) => {
                                log::warn!("Surface timeout")
                            }
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }) {
            Ok(()) => {}
            Err(e) => {
                log::error!("Running error: {e}")
            }
        };
    }
}

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [0.0, 0.5 * 800., 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5 * 800., -0.5 * 800., 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5 * 800., -0.5 * 800., 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
const VERTICES2: [Vertex; 3] = [
    Vertex {
        position: [0.0, 1., 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0., 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0., 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
