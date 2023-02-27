use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let vid_subsys = sdl_ctx.video().unwrap();
    let window = vid_subsys
        .window("test", 1200, 900)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    gl::load_with(|name| vid_subsys.gl_get_proc_address(name) as *const _);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.window().gl_set_context_to_current();
    unsafe {
        gl::ClearColor(0.6, 0.0, 0.8, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    canvas.present();

    

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    //let mut i = 0;
    /* */
    'mainloop: loop {
        //i = (i + 1) % 255;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        //canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

//gl::load_with()
