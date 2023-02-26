extern crate sdl2;
extern crate vulkano;

use std::{ffi::CString, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, video::VkInstance};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
use vulkano::instance::{self, Instance, InstanceCreateInfo, InstanceExtensions};
use vulkano::swapchain::Surface;
use vulkano::{Handle, Version, VulkanLibrary, VulkanObject};

fn main() -> Result<(), String> {
    //println!("Goodbye cruel world...");
    //::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 30));
    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;
    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    let library = VulkanLibrary::new().expect("no local Vulkan library");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    let physical = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");

    let window = video_subsys
        .window("Test", 1200, 900)
        .vulkan()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    for family in physical.queue_family_properties() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queue_count
        );
    }

    let queue_family_index = physical
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, q)| q.queue_flags.graphics)
        .expect("couldn't find a graphical queue family") as u32;

    let instance_extensions_strings: Vec<CString> = window
        .vulkan_instance_extensions()
        .unwrap()
        .iter()
        .map(|&v| CString::new(v).unwrap())
        .collect();

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    //let instance_extension = InstanceExtensions::from(instance_extensions_strings.iter().map(AsRef::as_ref));
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

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
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30)); // 30 fps
    }

    Ok(())
}
