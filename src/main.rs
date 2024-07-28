mod hello_screen;
mod game;

//? ///////////////////////////////////////////////////////////////////////

use sdl2::image::{LoadTexture, InitFlag};

use game::game_of_life::GameOfLife;
use game::ret::Ret;
use hello_screen::hello_screen::draw_hello_screen;

//? ///////////////////////////////////////////////////////////////////////

fn main() -> Result<(), String> {

    let width = 1200;
    let height = 1000;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Game of life", width, height)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let canvas = window.into_canvas()
        .build()
        .expect("could not make a canvas");

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let texture_creator = canvas.texture_creator();

    let mut textures = vec![
        texture_creator.load_texture("src/game/assets/icon-play.png")?,
        texture_creator.load_texture("src/game/assets/icon-pause.png")?,
        texture_creator.load_texture("src/game/assets/icon-pencil.png")?,
        texture_creator.load_texture("src/game/assets/icon-paint.png")?,
        texture_creator.load_texture("src/game/assets/icon-broom.png")?,
    ];

    GameOfLife::new(20, 20, width, height, sdl_context, canvas, &textures)?
        .start_game()?;

    // let mut ret = Ret::Start;
    // loop {
    //     match ret {
    //         Ret::Start => {
    //             ret = GameOfLife::new(20, 20, width, height, sdl_context, canvas, &textures)?
    //                 .start_game()?;
    //         },
    //         Ret::Help => {
    //             draw_hello_screen()?;
    //         },
    //         Ret::Unknown => {
    //             return Err("Unknown error".to_string());
    //         },
    //         Ret::Quit => {
    //             break;
    //         },
    //     }
    // }


    Ok(())
}
