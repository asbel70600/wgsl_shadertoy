use crate::gpupipeline::State;

use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};

#[derive(Debug)]
pub struct App {
    pub componet: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_transparent(true))
                .unwrap(),
        );
        let res = pollster::block_on(State::new(window.clone()));

        self.componet = Some(res.unwrap());
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.componet.as_mut().unwrap();
        // let input_handler = InputHandler{&state};

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                state.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                // input_handler::handleKyeboard(e,state);
                ()
            }
            WindowEvent::CursorMoved { position, .. } => {
                // input_handler::handle_mouse(position, state);
                ()
            }
            _ => (),
        }
    }
}
