mod audio;
mod camera;
mod display;
pub mod instance;
pub mod item;
mod model;
mod pipeline;
mod texture;
use display::Display;
pub use instance::Instance;
pub use item::Item;

use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

/*
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Matrix4 {
	// We can't use cgmath with bytemuck directly so we'll have
	// to convert the Matrix4 into a 4x4 f32 array
	m: [[f32; 4]; 4],
}
*/

pub async fn run(mut items: Vec<Vec<Item>>) {
	let event_loop = EventLoop::new();

	// Init audio
	//TODO: remove unwrap
	let (audio_data, stream) = audio::init(2048, 20, 20000).unwrap();

	// Create one state for each display
	let mut displays = vec![];
	while !items.is_empty() {
		let window = WindowBuilder::new().build(&event_loop).unwrap();
		let display = Display::new(window, items.pop().ok_or(()).unwrap()).await;
		displays.push(display);
	}

	//Init the LED controller

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
							} => { /*TODO*/ }
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
