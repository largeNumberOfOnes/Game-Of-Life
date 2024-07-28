// use sdl2::pixels::Color;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::rect::Point;
// use sdl2::render::{Canvas, Texture, TextureCreator};
// use sdl2::ttf::{Font, Sdl2TtfContext};
// use sdl2::video::Window;
use std::path::Path;
use sdl2::ttf::{self, Font};



pub fn draw_hello_screen() -> Result<(), String> {

    let ttf_context = ttf::init().unwrap();
    let font_path = Path::new("path/to/your/font.ttf");
    let font_size = 36;
    let font: Font = ttf_context.load_font(font_path, font_size)?;

    // let texture_creator: TextureCreator<_> = canvas.texture_creator();
    // let surface = font.render("Hello, SDL!")
    //     .blended(Color::RGB(255, 255, 255))?
    //     .map_err(|e| e.to_string())?;
    // let texture = texture_creator.create_texture_from_surface(&surface)?;

    loop {



    }

}
