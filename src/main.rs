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
    sdl_context: Sdl,
    mut canvas: WindowCanvas,
    textures: &Vec<&Texture>,
) -> Result<(), String>  {
    let mut ret = Ret::Start;
    // let mut ret = Ret::Help;

    let width = 1200;
    let height = 1000;
    let cellx = 20;
    let celly = 20;
    let mut game = GameOfLife::new(cellx, celly, width, height, &sdl_context, &mut canvas, textures)?;

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

fn init_textures_buf<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Result<DoubleBuf<Vec<Texture<'a>>>, String> {
    Ok(DoubleBuf::new(vec![
            texture_creator.load_texture("assets/dark/icon-play.png")?,
            texture_creator.load_texture("assets/dark/icon-pause.png")?,
            texture_creator.load_texture("assets/dark/icon-pencil.png")?,
            texture_creator.load_texture("assets/dark/icon-paint.png")?,
            texture_creator.load_texture("assets/dark/icon-broom.png")?,
            texture_creator.load_texture("assets/dark/icon-swap.png")?,
        ],
        vec![
            texture_creator.load_texture("assets/light/icon-play.png")?,
            texture_creator.load_texture("assets/light/icon-pause.png")?,
            texture_creator.load_texture("assets/light/icon-pencil.png")?,
            texture_creator.load_texture("assets/light/icon-paint.png")?,
            texture_creator.load_texture("assets/light/icon-broom.png")?,
            texture_creator.load_texture("assets/light/icon-swap.png")?,
        ]
    ))
}

fn main() -> Result<(), String> {

    let width = 1200;
    let height = 1000;
    let cellx = 20;
    let celly = 20;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Game of life", width, height)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas()
        .build()
        .expect("could not make a canvas");

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let  texture_creator = canvas.texture_creator();

    let textures_buf = init_textures_buf(&texture_creator)?;

    let mut textures: Vec<&Texture> = vec![];
    // let mut w: usize = 0;
    for q in textures_buf.get_cur() {
        textures.push(q);
        // w += 1;
    }

    main_circle(sdl_context, canvas, &textures);

    Ok(())
}
