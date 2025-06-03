use winit::{event::*, event_loop::{ControlFlow, EventLoop}};
use crate::{renderer::render::Renderer, world::world::World};

// this will call the render class
pub fn start_engine(world: World) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("GateVR");
    
    if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
        eprintln!("Failed to lock the cursor: {:?}", err);
    }
    window.set_cursor_visible(false);

    let mut renderer = pollster::block_on(Renderer::new(&window));    
    let render_start_time = std::time::Instant::now();

    let mut mouse_locked = true;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::Focused(focused) => {
                        if !focused {
                            mouse_locked = false;
                            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
                                eprintln!("Failed to unlock the cursor: {:?}", err);
                            }
                            window.set_cursor_visible(true);
                        }
                    }
                    WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        renderer.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        renderer.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - render_start_time;
                renderer.update(dt);

                match renderer.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.init.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}