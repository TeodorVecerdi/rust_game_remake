#![allow(dead_code)]
use glium;
use std;

pub struct GliumDisplayWinitWrapper(pub glium::backend::glutin::Display);

impl conrod_winit::WinitWindow for GliumDisplayWinitWrapper {
	fn get_inner_size(&self) -> Option<(u32, u32)> {
		self.0.gl_window().get_inner_size().map(Into::into)
	}
	fn hidpi_factor(&self) -> f32 {
		self.0.gl_window().get_hidpi_factor() as _
	}
}

pub struct EventLoop {
	ui_needs_update: bool,
	last_update: std::time::Instant,
}

impl EventLoop {
	pub fn new() -> Self {
		EventLoop {
			ui_needs_update: true,
			last_update: std::time::Instant::now(),
		}
	}

	pub fn needs_update(&mut self) {
		self.ui_needs_update = true;
	}

	pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> Vec<glium::glutin::Event> {
		// We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
		let last_update = self.last_update;
		let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
		let ms16 = std::time::Duration::from_millis(16);
		if duration_since_last_update < ms16 {
			std::thread::sleep(ms16 - duration_since_last_update);
		}

		let mut events = Vec::new();
		events_loop.poll_events(|event| events.push(event));

		if events.is_empty() && !self.ui_needs_update {
			events_loop.run_forever(|event| {
				events.push(event);
				glium::glutin::ControlFlow::Break
			});
		}

		self.ui_needs_update = false;
		self.last_update = std::time::Instant::now();

		events
	}
}

conrod_winit::conversion_fns!();
