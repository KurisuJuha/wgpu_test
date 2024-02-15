use wgpu::{Backends, InstanceDescriptor};
use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let size = window.inner_size();

    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        flags: (),
        dx12_shader_compiler: (),
        gles_minor_version: (),
    });

    event_loop
        .run(move |event, event_loop_window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close requested");
                event_loop_window_target.exit();
            }
            Event::AboutToWait => {
                // Youtubeみたいなやつならここでもredrawする感じ？なのかな？
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                window.request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
