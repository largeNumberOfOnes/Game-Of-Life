mod hello_screen;
mod default;
mod game;

//? ///////////////////////////////////////////////////////////////////////

use sdl2::{image::{InitFlag, LoadTexture}, render::{TextureCreator}, surface::Surface};

use game::game_of_life::GameOfLife;
use game::ret::Ret;
use sdl2::video::WindowContext;
use sdl2::render::{Texture, WindowCanvas};
use hello_screen::hello_screen::draw_hello_screen;
use sdl2::Sdl;

use std::rc::Rc;
use std::rc;

use game::double_buf::DoubleBuf;
use default::{palette, arg_proc};

//? ///////////////////////////////////////////////////////////////////////

fn main_circle(
    // game: &mut GameOfLife,
    width: u32,
    height: u32,
    cellx: u32,
    celly: u32,
    sdl_context: &Sdl,
    mut canvas: WindowCanvas,
    mut textures_buf: DoubleBuf<Vec<Texture>>,
) -> Result<(), String> {
    let mut ret = Ret::Start;

    let mut game = GameOfLife::new(
        width,
        height,
        cellx as usize,
        celly as usize,
        sdl_context,
    )?;

    loop {
        match ret {
            Ret::Start => {
                ret = game.game_loop(&mut canvas, textures_buf.get_cur())?;
            },
            Ret::ChangeColorTheme => {
                textures_buf.switch();
                palette::set_other();
                ret = Ret::Start;
            },
            Ret::Help => {
                draw_hello_screen(&sdl_context, &mut canvas)?;
                ret = Ret::Start;
            },
            Ret::Unknown => {
                return Err("Unknown error".to_string());
            },
            Ret::Quit => {
                break;
            },
            Ret::Continue => { unreachable!(); }
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

fn init_textures<'a>(
    texture_creator: &'a TextureCreator<WindowContext>
) -> Result<DoubleBuf<Vec<Texture<'a>>>, String> {
    Ok(DoubleBuf::new(
        vec![
            texture_creator.load_texture("assets/dark/icon-play.png")?,
            texture_creator.load_texture("assets/dark/icon-pause.png")?,
            texture_creator.load_texture("assets/dark/icon-pencil.png")?,
            texture_creator.load_texture("assets/dark/icon-paint.png")?,
            texture_creator.load_texture("assets/dark/icon-broom.png")?,
            texture_creator.load_texture("assets/dark/icon-swap.png")?,
            texture_creator.load_texture("assets/dark/icon-home.png")?,
            texture_creator.load_texture("assets/dark/icon-help.png")?,
            ],
        vec![
            texture_creator.load_texture("assets/light/icon-play.png")?,
            texture_creator.load_texture("assets/light/icon-pause.png")?,
            texture_creator.load_texture("assets/light/icon-pencil.png")?,
            texture_creator.load_texture("assets/light/icon-paint.png")?,
            texture_creator.load_texture("assets/light/icon-broom.png")?,
            texture_creator.load_texture("assets/light/icon-swap.png")?,
            texture_creator.load_texture("assets/light/icon-home.png")?,
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

    let mut width = 1200;
    let mut height = 1000;
    let mut cellx = 20;
    let mut celly = 20;

    arg_proc::read_command_line_args(
        &mut width ,
        &mut height,
        &mut cellx ,
        &mut celly ,
    );

    let (sdl_context, canvas) = init_sdl(width, height)?;
    let texture_creator = init_image(&canvas)?;
    let textures_buf = init_textures(&texture_creator)?;

    palette::set_dark();

    main_circle(width, height, cellx, celly, &sdl_context, canvas, textures_buf)?;

    Ok(())
}

// TODO: shell args
// TODO: shortcuts
