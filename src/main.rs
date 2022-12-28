use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {

    // Initialize the sdl2 library
    let sdl_context = sdl2::init()?;

    // Initialize the video subsystem
    let video_subsystem = sdl_context.video()?;

    // Initialize a new window
    let window = video_subsystem.window("NFT Make", 800, 600)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    // Initialize a new canvas
    let mut canvas = window.into_canvas().build()
        .expect("Could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    // Initialize event pump
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

