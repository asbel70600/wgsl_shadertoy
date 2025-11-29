#![warn(clippy::pedantic)]
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

use app::App;
use core::error::Error;
use tracing::{Level, span};
use winit::event_loop::EventLoop;

pub mod app;
pub mod config;
pub mod gpupipeline;
pub mod model;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let span = span!(Level::WARN, "MAIN_THREAD").entered();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App { componet: None };
    let _ = event_loop.run_app(&mut app);

    span.exit();
    Ok(())
}
