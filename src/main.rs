#![feature(const_fn_floating_point_arithmetic)]

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
#[macro_use]
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;

mod support;
mod scenes;
mod data;
mod theme;

use lazy_static::lazy_static;
use crate::{
	scenes::{SceneManager, Scene},
	data::{CharacterStats},
	theme::{Theme, ThemeManager},
};

use std::collections::HashMap;
use glium::Surface;

lazy_static! {
	static ref ASSETS_FOLDER: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 5).for_folder("assets").unwrap();
}

/* Get width and height from the command line arguments. If any of them is not present default to 720p */
fn get_dimensions() -> (u32, u32) {
	let args: Vec<String> = std::env::args().collect();
	if args.len() <= 2 {
		return (1600, 900);
	}

	let mut width: u32 = args[1].parse().unwrap();
	let mut height: u32 = args[2].parse().unwrap();

	if width < 600 {
		width = 600;
	}
	if height < 600 {
		height = 600;
	}
	(width, height)
}

fn load_fonts(fonts: &mut HashMap<&str, conrod_core::text::font::Id>, ui: &mut conrod_core::Ui) {
	[
		("lato", "Lato-Regular.ttf"),
		("firacode", "FiraCode-Retina.ttf"),
	]
	.iter()
	.map(|(k, v)| {
		let font_path = ASSETS_FOLDER.join(format!("fonts/{}", v));
		let font_id = ui.fonts.insert_from_file(font_path).unwrap();
		fonts.insert(k, font_id);
		font_id
	}).count();
}


fn main() {
	let (width, height) = get_dimensions();
	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
		.with_title("Hello, World")
		.with_resizable(false)
		.with_dimensions((width, height).into());
	let context = glium::glutin::ContextBuilder::new()
		.with_vsync(true)
		.with_multisampling(4);
	let display = glium::Display::new(window, context, &events_loop).unwrap();
	let display = support::GliumDisplayWinitWrapper(display);

	let mut ui = conrod_core::UiBuilder::new([width as f64, height as f64]).build();
	let mut fonts: HashMap<&str, conrod_core::text::font::Id> = HashMap::new();
	load_fonts(&mut fonts, &mut ui);

	let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();
	let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

	let mut scene_manager = SceneManager::new(&mut ui);
	scene_manager.set_starting_scene(SceneManager::MAIN_MENU);

	let mut event_loop = support::EventLoop::new();
	let event_loop_wakeup_proxy = events_loop.create_proxy();

	let mut is_light_theme = false;
	let mut theme_manager = ThemeManager::new();

	'main: loop {
		// Event handling loop
		for event in event_loop.next(&mut events_loop) {
			if let Some(event) = support::convert_event(event.clone(), &display) {
				ui.handle_event(event);
			}
			match event.clone() {
				glium::glutin::Event::WindowEvent { event, .. } => match event {
					glium::glutin::WindowEvent::CloseRequested
					| glium::glutin::WindowEvent::KeyboardInput {
						input:
							glium::glutin::KeyboardInput {
								virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
								..
							},
						..
					} => break 'main,
					glium::glutin::WindowEvent::KeyboardInput {
						input:
							glium::glutin::KeyboardInput {
								virtual_keycode: Some(glium::glutin::VirtualKeyCode::Space),
								state: glium::glutin::ElementState::Released,
								..
							},
						..
					} => {
						if is_light_theme {
							theme_manager.set_theme(theme::DARK_THEME);
						} else {
							theme_manager.set_theme(theme::LIGHT_THEME);
						}
						is_light_theme = !is_light_theme;
					}
					glium::glutin::WindowEvent::Resized(size) => {
						ui.win_w = size.width as f64;
						ui.win_h = size.height as f64;
						event_loop.needs_update();
					}
					_ => (),
				},
				_ => (),
			}
		}

		{
			let ui_cell = &mut ui.set_widgets();
			scene_manager.build(ui_cell, &fonts, &theme_manager, &event_loop_wakeup_proxy);
		}


		if let Some(primitives) = ui.draw_if_changed() {
			renderer.fill(&display.0, primitives, &image_map);
			let mut target = display.0.draw();
			target.clear_color(0.0, 0.0, 0.0, 1.0);
			renderer.draw(&display.0, &mut target, &image_map).unwrap();
			target.finish().unwrap();
		}
	}
}
