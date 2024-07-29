use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;
use sdl2::ttf::{self, Font};
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::path::Path;
use sdl2::Sdl;

use super::super::game::palette::*;
use super::super::default::default::*;


pub fn draw_hello_screen(sdl_context: &Sdl, canvas: &mut WindowCanvas)
                                                    -> Result<(), String> {

    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let font_path = Path::new(FONT_PATH);
    let font_size = 36;
    let font: Font = ttf_context.load_font(font_path, font_size)?;

    let texture_creator = canvas.texture_creator();
    let surface = font.render(HELLO_STRING)
        .blended(PALETTE_TEXT)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;


    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {

        for event in event_pump.poll_iter() { 
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    break 'running;
                },
                _ => {},
            }
        }

        canvas.set_draw_color(PALETTE_BACKGROUND);
        canvas.clear();
        let text_query = texture.query();
        canvas.copy(&texture, None, Rect::new(
            0, 0,
            text_query.width, text_query.height)
        )?;
        canvas.present();

    }

    Ok(())
}
