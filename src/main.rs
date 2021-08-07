#![feature(const_fn_floating_point_arithmetic)]
#![feature(type_name_of_val)]

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
#[macro_use]
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

mod support;
mod scenes;
mod data;
mod theme;

use crate::{
	scenes::{SceneManager, Scene}, 
	theme::ThemeManager
};

use lazy_static::lazy_static;
use std::collections::HashMap;
use glium::Surface;
use clap::{Arg, ArgSettings};
use dark_light;

lazy_static! {
	static ref ASSETS_FOLDER: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 5).for_folder("assets").unwrap();
}

fn get_cli_options() -> clap::ArgMatches {
	clap::App::new("MyApp")
		.arg(
			Arg::new("fullscreen")
			.long("fullscreen")
			.short('f')
			.about("Run in fullscreen mode")
		)
		.arg(
			Arg::new("resolution")
			.long("resolution")
			.short('r')
			.about("Set the resolution of the window")
			.setting(ArgSettings::RequireDelimiter)
			.setting(ArgSettings::MultipleValues)
			.value_delimiter("x")
			.number_of_values(2)
			.default_values(&["1600", "900"])
		)
		.get_matches()
}

/* Get width, height and fullscreen mode from the command line arguments. If any of them is not present default to 720p, false */
fn get_args(args: clap::ArgMatches) -> (u32, u32, bool) {
	let resolution: Vec<u32> = args.values_of_t("resolution").unwrap();
	
	let width = std::cmp::max(*resolution.get(0).unwrap(), 600);
	let height = std::cmp::max(*resolution.get(1).unwrap(), 600);
	let fullscreen = args.is_present("fullscreen");

	(width, height, fullscreen)
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
	})
	.count();
}

fn load_image(display: &glium::Display, path: &str) -> glium::texture::SrgbTexture2d {
	let image = image::open(&std::path::Path::new(&ASSETS_FOLDER.join("textures").join(path))).unwrap().to_rgba8();
	let dimensions = image.dimensions();
	let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
	glium::texture::SrgbTexture2d::new(display, raw_image).unwrap()
}

fn get_display(events_loop: &glium::glutin::EventsLoop, width: u32, height: u32, fullscreen: bool, title: &str) -> glium::Display {
	let window = glium::glutin::WindowBuilder::new()
		.with_title(title)
		.with_resizable(false)
		.with_dimensions((width, height).into());
	
	let context = glium::glutin::ContextBuilder::new()
		.with_vsync(true)
		.with_hardware_acceleration(Some(true))
		.with_multisampling(4);

	let display = glium::Display::new(window, context, &events_loop).unwrap();
	
	if !fullscreen {
		return display;
	}
	
	let window = glium::glutin::WindowBuilder::new()
		.with_title(title)
		.with_resizable(false)
		.with_dimensions((width, height).into())
		.with_fullscreen(Some(display.gl_window().get_primary_monitor()));

	let context = glium::glutin::ContextBuilder::new()
		.with_vsync(true)
		.with_multisampling(4);
	
	glium::Display::new(window, context, &events_loop).unwrap()
}


fn main() {
	let (width, height,fullscreen) = get_args(get_cli_options());

	let mut events_loop = glium::glutin::EventsLoop::new();
	
	let display = support::GliumDisplayWinitWrapper(get_display(
		&events_loop, 
		width, 
		height, 
		fullscreen, 
		"Hello, World",
	));

	let mut ui = conrod_core::UiBuilder::new([width as f64, height as f64]).build();
	let mut fonts: HashMap<&str, conrod_core::text::font::Id> = HashMap::new();
	load_fonts(&mut fonts, &mut ui);

	let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();
	let mut image_map = conrod_core::image::Map::<glium::texture::SrgbTexture2d>::new();
	let images = load_images(&mut image_map, &display.0);

	let mut event_loop = support::EventLoop::new();
	let event_loop_wakeup_proxy = events_loop.create_proxy();

	let mut scene_manager = SceneManager::new(&event_loop_wakeup_proxy, &mut ui);
	scene_manager.set_starting_scene(SceneManager::MAIN_MENU);


	let mut data_store = data::DataStore::new();
	let mut is_light_theme = false;
	let mut theme_manager = ThemeManager::new();
	match dark_light::detect() {
		dark_light::Mode::Light => {
			is_light_theme = true;
			theme_manager.set_theme(theme::LIGHT_THEME);
		},
		_ => {}
	}
	data_store.set("is_light_theme", is_light_theme);
	let mut last_frame_time = std::time::Instant::now();

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

		if data_store.has("should_toggle_theme") {
			data_store.remove("should_toggle_theme");
			if is_light_theme {
				theme_manager.set_theme(theme::DARK_THEME);
			} else {
				theme_manager.set_theme(theme::LIGHT_THEME);
			}
			is_light_theme = !is_light_theme;
			data_store.set("is_light_theme", is_light_theme);
		}

		{
			let ui_cell = &mut ui.set_widgets();
			scene_manager.build(ui_cell, &images, &fonts, &theme_manager, &mut data_store);
		}


		if let Some(primitives) = ui.draw_if_changed() {
			renderer.fill(&display.0, primitives, &image_map);
			let mut target = display.0.draw();
			target.clear_color(0.0, 0.0, 0.0, 1.0);
			renderer.draw(&display.0, &mut target, &image_map).unwrap();
			target.finish().unwrap();
		}

		let now = std::time::Instant::now();
		let delta: std::time::Duration = now - last_frame_time;
		last_frame_time = now;
		
		display.0.gl_window().window().set_title(format!("CPP Remake | {} FPS | {:.2}ms", (1.0_f64 / delta.as_secs_f64()) as u64, delta.as_secs_f64() * 1000.0).as_str());
	}
}

fn load_images(
	image_map: &mut conrod_core::image::Map<glium::texture::SrgbTexture2d>, 
	display: &glium::Display
) -> HashMap<String, conrod_core::image::Id> 
{
    let mut images: HashMap<String, _> = vec![
		("dark_mode_icon", "misc/DarkModeIcon.png"),
		("light_mode_icon", "misc/LightModeIcon.png"),
	]
	.into_iter()
	.map(|(name, path)| {
		let id = image_map.insert(load_image(display, path));
		(String::from(name), id)
	})
	.collect();
	
	for &class in data::ALL_CHARACTER_TYPES {
		for &state in data::ALL_CHARACTER_STATES {
			let id = image_map.insert(load_image(display, format!("characters/{}/head_{}.png", class, state).as_str()));
			images.insert(format!("{}_{}", class, state), id);
		}
	}

	images
}
