use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::image::{InitFlag, LoadTexture};

use std::time::Duration;
use std::path::Path;

use rand::Rng;

// Set constants for magic numbers
const EIGHT_BIT: u32 = 256;
const WHITE: u8 = (EIGHT_BIT-1) as u8;
const NFT_WIDTH: u32 = EIGHT_BIT;
const NFT_HEIGHT: u32 = EIGHT_BIT;

// Set constants for robot part indexes
const COLOR: usize = 0;
const ANTS: usize = 1;
const ARMS: usize = 2;
const HEAD: usize = 3;
const LEGS: usize = 4;
const TORSO: usize = 5;

// Set constant array for color tuples
const ROYGBIV: [(u8, u8, u8); 8] = [
    (0, 0, 0),      // black
    (255, 0, 0),    // red
    (255, 165, 0),  // orange
    (255, 255, 0),  // yellow
    (0, 128, 0),    // green
    (0, 0, 255),    // blue
    (75, 0, 130),   // indigo
    (238, 130, 238) // violet
]; 

fn render(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font, count: u32, rand_data: [i32;6]) -> Result<(), String> {      

    // Determine color
    let (r, g, b) = ROYGBIV[rand_data[COLOR] as usize];
    let color = Color::RGB(r, g, b); 

    // Set background color of canvas
    canvas.set_draw_color(color);
    canvas.clear();

    // Load robot head and draw on canvas
    let mut image_path = format!("img/head/head_0{}.png", rand_data[HEAD]);
    let mut texture = texture_creator.load_texture(image_path)
        .expect("Couldn't load image");
    let mut target = Rect::new(86 as i32, 0 as i32, 86 as u32, 86 as u32);
    canvas.copy(&texture, None, Some(target))?;

    // Load robot torso and draw on canvas
    image_path = format!("img/torso/torso_0{}.png", rand_data[TORSO]);
    texture = texture_creator.load_texture(image_path)
        .expect("Couldn't load image");
    target = Rect::new(86 as i32, 86 as i32, 86 as u32, 86 as u32);
    canvas.copy(&texture, None, Some(target))?;    

    // Draw count text on canvas
    let index_text: String = format!("{:08b}", count);
    let surface = font
        .render(&index_text)
        .blended(Color::RGB(WHITE-color.r, WHITE-color.g, WHITE-color.b))
        .map_err(|e| e.to_string())?;
    texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    target = Rect::new(86 as i32, 170 as i32, 80 as u32, 50 as u32);
    canvas.copy(&texture, None, Some(target))?;

    // Display canvas
    canvas.present();

    Ok(())
    }

fn main() -> Result<(), String> {

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
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // Prepare image
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    
    // Initialize event pump
    let mut event_pump = sdl_context.event_pump()?;

    // Setup random number generator
    let mut rng = rand::thread_rng();

    // Setup array to hold random values for color,
    // antennas, arms, head, legs, and torso
    let mut rand_arr: [i32; 6] = [0; 6];

    // Loop through NFTs
    'running: for index in 0..EIGHT_BIT {
        // Generate random values for NFT image
        rand_arr[COLOR] = rng.gen_range(0..8);  // RGB color
        rand_arr[ANTS] = rng.gen_range(0..8);  // Antennas
        rand_arr[ARMS] = rng.gen_range(0..8);  // Arms
        rand_arr[HEAD] = rng.gen_range(0..8);  // Head
        rand_arr[LEGS] = rng.gen_range(0..8);  // Legs
        rand_arr[TORSO] = rng.gen_range(0..8);  // Torso
  
        // Use random generated data to render new NFT
        render(&mut canvas, &texture_creator, &font, index, rand_arr)?;            

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

    Ok(())
}

