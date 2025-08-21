use winit::{event::*, event_loop::{ControlFlow, EventLoop}};
use crate::{renderer::render::Renderer, world::world::World};

// this will call the render class
pub fn start_engine(world: World) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("GateVR");

    let mut renderer = pollster::block_on(Renderer::new(&window));    
    let render_start_time = std::time::Instant::now();

    //let mut mouse_locked = true;
    let mut frame = 0;

    let mut mouse_locked = false;

    let mut keys = [false; 5]; // keys: W A S D
    let mut mouse = [0.0; 2]; // mouse movement x and y

    event_loop.run(move |event, _, control_flow| {
        mouse[0] -= mouse[0] * 0.1;
        mouse[1] -= mouse[1] * 0.1;
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
                    WindowEvent::CursorMoved { position, .. } => {
                        if mouse_locked {
                            let window_size = window.inner_size();
                            let center_x = window_size.width as f64 / 2.0;
                            let center_y = window_size.height as f64 / 2.0;
                            mouse[0] = center_x - position.x;
                            mouse[1] = center_y - position.y;
                            window.set_cursor_position(winit::dpi::PhysicalPosition::new(center_x, center_y)).expect("Failed to set cursor position");
                        } else {
                            mouse[0] = 0.0;
                            mouse[1] = 0.0;
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => {
                        match key_state {
                            ElementState::Pressed => {
                                match &keycode {
                                    &VirtualKeyCode::W => { keys[0] = true }
                                    &VirtualKeyCode::A => { keys[1] = true }
                                    &VirtualKeyCode::S => { keys[2] = true }
                                    &VirtualKeyCode::D => { keys[3] = true }
                                    &VirtualKeyCode::Space => { keys[4] = true }
                                    &VirtualKeyCode::Escape | &VirtualKeyCode::LWin | &VirtualKeyCode::RWin => {
                                        mouse_locked = false;
                                        if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
                                            eprintln!("Failed to unlock the cursor: {:?}", err);
                                        }
                                        window.set_cursor_visible(true);
                                    }
                                    _ => {}
                                }
                            }
                            ElementState::Released => {
                                match &keycode {
                                    &VirtualKeyCode::W => { keys[0] = false }
                                    &VirtualKeyCode::A => { keys[1] = false }
                                    &VirtualKeyCode::S => { keys[2] = false }
                                    &VirtualKeyCode::D => { keys[3] = false }
                                    &VirtualKeyCode::Space => { keys[4] = false }
                                    _ => {}
                                }
                            }
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        match state {
                            ElementState::Pressed => {
                                match button {
                                    MouseButton::Left => {
                                        mouse_locked = true;
                                        if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                                            eprintln!("Failed to lock the cursor: {:?}", err);
                                        }
                                        window.set_cursor_visible(false);
                                        let window_size = window.inner_size();
                                        let center_x = window_size.width as f64 / 2.0;
                                        let center_y = window_size.height as f64 / 2.0;
                                        window.set_cursor_position(winit::dpi::PhysicalPosition::new(center_x, center_y)).expect("Failed to set cursor position");
                                    }
                                    _ => {}
                                }
                            }
                            ElementState::Released => {
                                return
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - render_start_time;

                renderer.update(dt, keys, mouse);

                if frame % 120 == 0 {
                    renderer.set_objects(&world);
                }

                match renderer.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.init.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }

                frame += 1;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}