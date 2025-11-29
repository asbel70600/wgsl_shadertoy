use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};

pub struct State;

#[derive(Debug)]
pub struct App {
    pub componet: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {}
            WindowEvent::Resized(size) => {}
            WindowEvent::KeyboardInput { event, .. } => {}
            WindowEvent::CursorMoved { position, .. } => {}
            _ => (),
        }
    }
}
