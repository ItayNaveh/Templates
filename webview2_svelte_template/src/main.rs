#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::unsync::OnceCell;
use std::mem;
use std::rc::Rc;
use webview2::Controller;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use winit::dpi::Size;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::windows::WindowExtWindows;
use winit::window::WindowBuilder;

fn main() {
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title("WebView2 - winit")
		.with_inner_size(Size::Logical((920, 640).into()))
		.build(&event_loop)
		.unwrap();

	let controller: Rc<OnceCell<Controller>> = Rc::new(OnceCell::new());

	let create_result = {
		let controller_clone = controller.clone();
		let hwnd = window.hwnd() as HWND;

		webview2::Environment::builder().build(move |env| {
			env.expect("env").create_controller(hwnd, move |controller| {
				let controller = controller.expect("create host");
				let webview = controller.get_webview().expect("get_webview");

				unsafe {
					let mut rect = mem::zeroed();
					GetClientRect(hwnd, &mut rect);
					controller.put_bounds(rect).expect("put_bounds");
				}

				let mut app = App::new();
				app.run(&controller, webview);

				controller_clone.set(controller).unwrap();
				Ok(())
			})
		})
	};
	if let Err(e) = create_result {
		eprintln!(
			"Failed to create webview environment: {}. Is the new edge browser installed?",
			e
		);
	}

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;

		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => {
					if let Some(webview_host) = controller.get() {
						webview_host.close().expect("close");
					}
					*control_flow = ControlFlow::Exit;
				}
				// Notify the webview when the parent window is moved.
				WindowEvent::Moved(_) => {
					if let Some(webview_host) = controller.get() {
						let _ = webview_host.notify_parent_window_position_changed();
					}
				}
				// Update webview bounds when the parent window is resized.
				WindowEvent::Resized(new_size) => {
					if let Some(webview_host) = controller.get() {
						let r = RECT {
							left: 0,
							top: 0,
							right: new_size.width as i32,
							bottom: new_size.height as i32,
						};
						webview_host.put_bounds(r).expect("put_bounds");
					}
				}
				_ => {}
			},
			Event::MainEventsCleared => {
				// Application update code.

				// Queue a RedrawRequested event.
				window.request_redraw();
			}
			Event::RedrawRequested(_) => {}
			_ => (),
		}
	});
}

struct App;

impl App {
	fn new() -> App {
		App {}
	}

	fn run(&mut self, _controller: &Controller, webview: webview2::WebView) {
		let settings = webview.get_settings().unwrap();
		settings.put_is_status_bar_enabled(false).unwrap();
		settings.put_are_default_context_menus_enabled(false).unwrap();
		settings.put_is_zoom_control_enabled(false).unwrap();

		#[cfg(debug_assertions)]
		webview.open_dev_tools_window().unwrap();
		
		#[cfg(debug_assertions)]
		webview.navigate_to_string(r#"
			<!DOCTYPE html>
			<html>
				<head>
					<script type="module" src="http://localhost:5000/bundle.js"></script>
				</head>
				<body></body>
			</html>
		"#).unwrap();

		#[cfg(not(debug_assertions))]
		webview.navigate_to_string(&format!(r#"
			<!DOCTYPE html>
			<html>
				<head>
					<script type="module">{}</script>
				</head>
				<body></body>
			</html>
		"#, include_str!("../target/release/client/bundle.js"))).unwrap();
		
		// webview.navigate_to_string(include_str!("../target/page.html")).unwrap();
		
		webview.add_web_message_received(|_, _| { println!("recieved web msg"); Ok(()) }).unwrap();
	}
}
