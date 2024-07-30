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
use default::palette;

//? ///////////////////////////////////////////////////////////////////////

fn change_color_theme(textures_buf: &mut DoubleBuf<Vec<Texture>>) {
    palette::set_other();
    
}

fn main_circle(
    // game: &mut GameOfLife,
    width: u32,
    height: u32,
    cellx: u32,
    celly: u32,
    sdl_context: &Sdl,
    mut canvas: WindowCanvas,
    textures_dark: Vec<Texture>,
    textures_light: Vec<Texture>,
) -> Result<(), String> {
    let mut ret = Ret::Start;

    let mut textures: Vec<&Texture> = vec![];
    for q in 0..textures_dark.len() {
        textures.push(&(textures_dark[q]));
    }

    let mut game = GameOfLife::new(
        width,
        height,
        cellx as usize,
        celly as usize,
        sdl_context,
        &textures
    )?;

    loop {
        match ret {
            Ret::Start => {
                ret = game.game_loop(&mut canvas)?;
            },
            Ret::ChangeColorTheme => {
                // for q in 0..textures_dark.len() {
                //     // textures[q] = &(textures_dark[q]);
                // }
                textures = vec![];
                ret = Ret::Start;
            }
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
) -> Result<(Vec<Texture<'a>>, Vec<Texture<'a>>), String> {
    Ok((vec![
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
    let mut textures = init_textures(&texture_creator)?;

    palette::set_dark();
    // palette::set_light();

    main_circle(width, height, cellx, celly, &sdl_context, canvas, textures.0, textures.1)?;

    Ok(())
}

// TODO: change color theme
// TODO: shell args
