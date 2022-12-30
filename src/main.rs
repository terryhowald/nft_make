use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use std::time::Duration;
use std::path::Path;

use rand::Rng;

// NFT Dimensions in pixels
const EIGHT_BIT: u32 = 256;
const NFT_WIDTH: u32 = EIGHT_BIT;
const NFT_HEIGHT: u32 = EIGHT_BIT;

fn render(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font, value: u32, color: sdl2::pixels::Color) -> Result<(), String> {

    // Set background color
    canvas.set_draw_color(color);
    canvas.clear();

    // Draw number value
    let value_text: String = format!("{value:08b}");
    //let value_text: String = value.to_string();
    let surface = font
        .render(&value_text)
        .blended(Color::RGB(255-color.r, 255-color.g, 255-color.b))
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(10 as i32, 0 as i32, 200 as u32, 100 as u32);
    canvas.copy(&texture, None, Some(target))?;

    canvas.present();

    Ok(())
    }

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
        .expect("Failed to initialize canvas");

    // Create texture creator for the new canvas
    let texture_creator = canvas.texture_creator();

    // Prepare fonts
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    // Initialize event pump
    let mut event_pump = sdl_context.event_pump()?;

    // Setup random number generator
    let mut rng = rand::thread_rng();

    // Loop through NFTs
    'running: loop {
        for i in 1..EIGHT_BIT {
            // Generate random index and retrieve RGB values
            let index = rng.gen_range(0..8);         
            let (r, g, b) = roygbiv[index];
            let color = Color::RGB(r, g, b); 
            let ib_str: String = format!("{i:08b}");
            println!("{} {} {} {} {}", i, r, g, b, ib_str);
         
            render(&mut canvas, &texture_creator, &font, i, color);            

            // See if any events are pending
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { 
                        keycode: Some(Keycode::Escape), 
                        .. 
                        } => break 'running,
                    _ => {}
                }
            } 

            // Sleep for 1 second to show NFT
            ::std::thread::sleep(Duration::from_secs(1));
        }
    }

    Ok(())
}

