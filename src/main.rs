
extern crate sdl2;
extern crate vulkano;

use std::{ffi::CString, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, video::VkInstance};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::Surface;
use vulkano::{Handle, Version, VulkanObject};

fn main() -> Result<(), String> {
    //println!("Goodbye cruel world...");
    //::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 30));
    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;
    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    let window = video_subsys
        .window("Test", 800, 600)
        .vulkan()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let instance_extensions_strings: Vec<CString> = window
        .vulkan_instance_extensions()
        .unwrap()
        .iter()
        .map(|&v| CString::new(v).unwrap())
        .collect();

    
    let instance_extension = InstanceExtensions::from(instance_extensions_strings.iter().map(AsRef::as_ref))
    
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_ctx.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30)); // 30 fps
    }

    Ok(())
}
