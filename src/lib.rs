mod audio;
mod camera;
mod display;
mod instance;
mod model;
mod pipeline;
mod texture;
mod vs_0;
use display::Display;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run(nb_displays: u32) {
    let event_loop = EventLoop::new();

    // Init audio
    //TODO: remove unwrap
    let (audio_data, _stream) = audio::init(2048, 20, 20000).unwrap();

    // Initialize the displays
    let mut displays = vec![];
    for _ in 0..nb_displays {
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let display: Result<Display, display::DisplayError> = Display::new(window).await;
        match display {
            Ok(d) => displays.push(d),
            Err(e) => {
                eprintln!("[ERROR] {e}");
                std::process::exit(1);
            }
        }
    }

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                for s in &mut displays {
                    if window_id == s.window().id() {
                        match event {
                            WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::R),
                                        ..
                                    },
                                ..
                            } => {}
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                s.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &mut so w have to dereference it twice
                                s.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                        break;
                    }
                }
            }

            Event::RedrawRequested(window_id) => {
                for d in &mut displays {
                    if window_id == d.window().id() {
                        d.update(&audio_data);
                        match d.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                d.resize(d.size)
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            // We're ignoring timeouts
                            Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                        }
                        break;
                    }
                }
            }

            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                for s in &mut displays {
                    s.window().request_redraw();
                }
            }
            _ => (),
        }
    });
}
