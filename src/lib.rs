pub mod action;
mod camera;
mod display;
mod fft;
mod group;
pub mod instance;
pub mod item;
mod light;
mod model;
mod texture;
use crate::model::Model;
pub use action::Action;
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

	// Create one state for each display
	let mut displays = vec![];
	while !items.is_empty() {
		let window = WindowBuilder::new().build(&event_loop).unwrap();
		let display = Display::new(window, items.pop().ok_or(()).unwrap()).await;
		displays.push(display);
	}

	#[allow(unused)]
	let (levels, stream) = fft::init(2048, 4, 20, 15000).unwrap();

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent {
				ref event,
				window_id,
			} => {
				for s in &mut displays {
					if window_id == s.window().id() {
						//TODO: remove input
						match event {
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
						d.update(&levels);
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
