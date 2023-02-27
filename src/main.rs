extern crate sdl2;
extern crate vulkano;

use std::borrow::Borrow;
use std::sync::Arc;
use std::{ffi::CString, time::Duration};

use bytemuck::{Pod, Zeroable};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, video::VkInstance};
use vulkano::buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::allocator::{CommandBufferAllocator, StandardCommandBufferAllocator};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
use vulkano::instance::{self, Instance, InstanceCreateInfo, InstanceExtensions};
use vulkano::memory::allocator::{FreeListAllocator, GenericMemoryAllocator};
use vulkano::swapchain::Surface;
use vulkano::{Handle, Version, VulkanLibrary, VulkanObject};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
struct Units {
    a: u32,
    b: u32,
}

fn main() -> Result<(), String> {
    //println!("Goodbye cruel world...");
    //::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 30));

    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    //
    //  Vulkan context set up BEGIN:
    //

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
    let data = Units { a: 5, b: 50 };
    let iter = (0..128).map(|_| 5u8);
    let genericdevice = GenericMemoryAllocator::<Arc<FreeListAllocator>>::new_default(device);

    let buffer = CpuAccessibleBuffer::from_iter(
        genericdevice.borrow(),
        BufferUsage {
            uniform_buffer: true,
            ..Default::default()
        },
        false,
        iter,
    )
    .unwrap();

    let mut content = buffer.write().unwrap();

    content[12] = 83;
    content[7] = 3;

    let source_content: Vec<i32> = (0..64).collect();
    let source = CpuAccessibleBuffer::from_iter(
        genericdevice.borrow(),
        BufferUsage {
            transfer_src: true,
            ..Default::default()
        },
        false,
        source_content,
    )
    .expect("failed to create source buffer");

    let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
    let destination = CpuAccessibleBuffer::from_iter(
        genericdevice.borrow(),
        BufferUsage {
            transfer_src: true,
            ..Default::default()
        },
        false,
        destination_content,
    )
    .expect("failed to create destination buffer");

    //let cmddevice = StandardCommandBufferAllocator::<Arc<CommandBufferAllocator>>::new_default()

    /* 
    let mut builder = AutoCommandBufferBuilder::primary(
        genericdevice.borrow(),
                queue_family_index,
                CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    */

    //
    //  Vulkan context set up END:
    //

    //
    //  SDL context set up BEGIN:
    //

    //let instance_extension = InstanceExtensions::from(instance_extensions_strings.iter().map(AsRef::as_ref));

    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    //
    //  SDL context set up END:
    //

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    //println!("{:?}", buffer.usage());

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
