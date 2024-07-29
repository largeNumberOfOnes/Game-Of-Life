mod hello_screen;
mod default;
mod game;

//? ///////////////////////////////////////////////////////////////////////

use sdl2::{image::{InitFlag, LoadTexture}, render::TextureCreator, surface::Surface};

use game::game_of_life::GameOfLife;
use game::ret::Ret;
use sdl2::video::WindowContext;
use sdl2::render::{Texture, WindowCanvas};
use hello_screen::hello_screen::draw_hello_screen;
use sdl2::Sdl;

use game::double_buf::DoubleBuf;

//? ///////////////////////////////////////////////////////////////////////

fn change_color_theme() {

}

fn main_circle(
    // game: &mut GameOfLife,
    cellx: u32,
    celly: u32,
    width: u32,
    height: u32,
    sdl_context: Sdl,
    mut canvas: WindowCanvas,
    textures: &Vec<&Texture>,
) -> Result<(), String>  {
    let mut ret = Ret::Start;
    // let mut ret = Ret::Help;

    let mut game = GameOfLife::new(
        cellx as usize,
        celly as usize,
        width,
        height,
        &sdl_context,
        &mut canvas,
        textures
    )?;

    loop {
        match ret {
            Ret::Start => {
                ret = game.start_game()?;
            },
            Ret::ChangeColorTheme => {
                change_color_theme();
            }
            Ret::Help => {
                // draw_hello_screen(&sdl_context, &mut canvas)?;
                ret = Ret::Start;
                break;
            },
            Ret::Unknown => {
                return Err("Unknown error".to_string());
            },
            Ret::Quit => {
                break;
            },
        }
    }

    Ok(())
}

fn init_sdl(width: u32, height: u32)
    -> Result<(Sdl, WindowCanvas), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Game of life", width, height)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let canvas = window.into_canvas()
        .build()
        .expect("could not make a canvas");

    Ok((sdl_context, canvas))
}

fn init_textures_buf<'a>(
    texture_creator: &'a TextureCreator<WindowContext>
) -> Result<DoubleBuf<Vec<Texture<'a>>>, String> {
    Ok(DoubleBuf::new(vec![
            texture_creator.load_texture("assets/dark/icon-play.png")?,
            texture_creator.load_texture("assets/dark/icon-pause.png")?,
            texture_creator.load_texture("assets/dark/icon-pencil.png")?,
            texture_creator.load_texture("assets/dark/icon-paint.png")?,
            texture_creator.load_texture("assets/dark/icon-broom.png")?,
            texture_creator.load_texture("assets/dark/icon-swap.png")?,
            texture_creator.load_texture("assets/dark/icon-help.png")?,
        ],
        vec![
            texture_creator.load_texture("assets/light/icon-play.png")?,
            texture_creator.load_texture("assets/light/icon-pause.png")?,
            texture_creator.load_texture("assets/light/icon-pencil.png")?,
            texture_creator.load_texture("assets/light/icon-paint.png")?,
            texture_creator.load_texture("assets/light/icon-broom.png")?,
            texture_creator.load_texture("assets/light/icon-swap.png")?,
            texture_creator.load_texture("assets/light/icon-help.png")?,
        ]
    ))
}

fn init_image(canvas: &WindowCanvas)
    -> Result<TextureCreator<WindowContext>, String> {
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    Ok(canvas.texture_creator())
}

fn main() -> Result<(), String> {

    let width = 1200;
    let height = 1000;
    let cellx = 20;
    let celly = 20;

    let (sdl_context, canvas) = init_sdl(width, height)?;
    let texture_creator = init_image(&canvas)?;
    let textures_buf = init_textures_buf(&texture_creator)?;

    let mut textures: Vec<&Texture> = vec![];
    for q in textures_buf.get_cur() {
        textures.push(q);
    }

    main_circle(cellx, celly, width, height, sdl_context, canvas, &textures)?;

    Ok(())
}
