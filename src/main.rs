use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;

// NFT Dimensions in pixels
const EIGHT_BIT: u32 = 256;
const NFT_WIDTH: u32 = EIGHT_BIT;
const NFT_HEIGHT: u32 = EIGHT_BIT;

fn main() -> Result<(), String> {

    // Setup array of RGB tuples
    let roygbiv = [
        (0, 0, 0),      // black
        (255, 0, 0),    // red
        (255, 165, 0),  // orange
        (255, 255, 0),  // yellow
        (0, 128, 0),    // green
        (0, 0, 255),    // blue
        (75, 0, 130),   // indigo
        (238, 130, 238) // violet
    ];    

    // Initialize the sdl2 library
    let sdl_context = sdl2::init()?;

    // Initialize the video subsystem
    let video_subsystem = sdl_context.video()?;

    // Initialize a new window
    let window = video_subsystem.window("NFT Make", NFT_WIDTH, NFT_HEIGHT)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    // Initialize a new canvas
    let mut canvas = window.into_canvas().build()
        .expect("Could not make a canvas");

    // Clear canvas with drawing color (black) and display
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Initialize event pump
    let mut event_pump = sdl_context.event_pump()?;

    // Setup random number generator
    let mut rng = rand::thread_rng();

    // Loop through NFTs
    'running: for i in 1..EIGHT_BIT {
        // Generate random index and retrieve RGB values
        let index = rng.gen_range(0..8);
        let (r, g, b) = roygbiv[index];
        println!("{} {} {} {}", i, r, g, b);

        // Clear canvas with new drawing color
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.clear();

        // See if any events are pending
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }        

        // Display newly constructed NFT
        canvas.present();

        // Sleep for 1 second to show NFT
        ::std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

