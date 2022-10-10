#![deny(clippy::all)]
#![forbid(unsafe_code)]
use std::time::SystemTime;

use crate::gui::Framework;
use egui_wgpu::wgpu::{RequestAdapterOptions, PowerPreference, PresentMode};
use log::error;
use pixels::{Error, SurfaceTexture, PixelsBuilder};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;
mod canvas;
mod tools;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rustpaint")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        //v1
        //let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        let pixels = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
            .request_adapter_options(RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .present_mode(PresentMode::Immediate)
            .enable_vsync(false)
            .build()?;
        let framework =
            Framework::new(window_size.width, window_size.height, scale_factor, &pixels);

        (pixels, framework)
    };
    let mut canvas = canvas::Canvas::new(HEIGHT, WIDTH);
    let mut tool = tools::Tool::new();
    let tool_size: i32 = 5; // todo make dyn 

    //let mut time = SystemTime::now();
    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::P){
                tool.mode = tools::ToolType::Pen;
            }

            if input.key_pressed(VirtualKeyCode::E){
                tool.mode = tools::ToolType::Ereaser;
            }

            if input.mouse_released(0) {
                tool.state = tools::ToolState::Up;
            }

            if input.mouse_pressed(0) {
                tool.state = tools::ToolState::Down;
                // let pos = input.mouse().unwrap();
                // let x = (pos.0 / window.scale_factor() as f32) as i32;
                // let y = (pos.1 / window.scale_factor() as f32) as i32;
                // tools::draw(&mut canvas, &tool, tool_size, (x, y));
            }

            if input.mouse_held(0) {
                // let pos = input.mouse().unwrap();
                // let x = (pos.0 / window.scale_factor() as f32) as i32;
                // let y = (pos.1 / window.scale_factor() as f32) as i32;
                // tools::draw(&mut canvas, &tool, tool_size, (x, y));
            }

            if input.mouse().is_some() {
                let pos = input.mouse().unwrap();
                let x = pos.0.round() as u32;
                let y = pos.1.round() as u32;
            }
            
            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                framework.resize(size.width, size.height);
            }
            
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                let start_time = SystemTime::now();
                // Draw the world
                canvas.canvas_to_frame(pixels.get_frame());

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if render_result
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }

                //println!("Rendering took: {:?}", SystemTime::now().duration_since(start_time));
            }
            _ => (),
        }
    });
}