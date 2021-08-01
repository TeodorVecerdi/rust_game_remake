#[macro_use]
extern crate conrod;
extern crate find_folder;

use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Colorable, Positionable, Widget};
use std::collections::HashMap;

fn main() {
	const WIDTH: u32 = 1280;
	const HEIGHT: u32 = 720;

	// Window & Event loop setup
	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
		.with_title("Hello, World!")
		.with_dimensions(WIDTH, HEIGHT);
	let context = glium::glutin::ContextBuilder::new()
		.with_vsync(true)
		.with_multisampling(4);
	let display = glium::Display::new(window, context, &events_loop).unwrap();

	// Rendering setup
	let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
	let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
	let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

	widget_ids!(struct Ids { text });
	let ids = Ids::new(ui.widget_id_generator());

	let assets = find_folder::Search::KidsThenParents(3, 5)
		.for_folder("assets")
		.unwrap();
	let lato_font_path = assets.join("fonts/Lato-Regular.ttf");
	let fc_font_path = assets.join("fonts/FiraCode-Retina.ttf");

	let font_ids: HashMap<&str, conrod::text::font::Id> = [
		("lato", ui.fonts.insert_from_file(lato_font_path).unwrap()),
		("fira-code", ui.fonts.insert_from_file(fc_font_path).unwrap()),
	].iter().cloned().collect();

	let mut event_loop = EventLoop::new();

	// Main loop
	'main: loop {
		// Event handling
		for event in event_loop.next(&mut events_loop) {
			if let Some(event) = conrod::backend::winit::convert_event(
				event.clone(),
				&display
			) {
				ui.handle_event(event);
			}

			match event {
				glium::glutin::Event::WindowEvent { event, .. } => match event {
					glium::glutin::WindowEvent::Closed
					| glium::glutin::WindowEvent::KeyboardInput {
						input:
							glium::glutin::KeyboardInput {
								virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
								..
							},
						..
					} => break 'main,
					_ => (),
				},
				_ => (),
			}
		}

		// Rendering
		{
			let ui = &mut ui.set_widgets();
			widget::Text::new("Hello World!")
				.middle_of(ui.window)
				.color(conrod::color::WHITE)
				.font_id(font_ids["lato"])
				.font_size(32)
				.set(ids.text, ui);
		}

		// Render the `Ui` and then display it on the screen.
		if let Some(primitives) = ui.draw_if_changed() {
			renderer.fill(&display, primitives, &image_map);
			let mut target = display.draw();
			target.clear_color(0.1, 0.1, 0.1, 1.0);
			renderer.draw(&display, &mut target, &image_map).unwrap();
			target.finish().unwrap();
		}
	}
}

pub struct EventLoop {
	ui_needs_update: bool,
	last_update: std::time::Instant
}

impl EventLoop {
	pub fn new() -> Self {
		EventLoop { last_update: std::time::Instant::now(), ui_needs_update: true }
	}

	pub fn needs_update(&mut self) {
		self.ui_needs_update = true;
	}

	pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> Vec<glium::glutin::Event> {
		let last_update = self.last_update;
		let sixteen_ms = std::time::Duration::from_millis(16);
		let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
		if duration_since_last_update < sixteen_ms {
			std::thread::sleep(sixteen_ms - duration_since_last_update);
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