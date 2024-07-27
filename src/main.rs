mod game;
// use cour::task_1::combinations::combinations1;
// use cour::tutorial::add::add;
use game::game_of_life::GameOfLife;

use sdl2::image::{self, LoadTexture, InitFlag};


fn main() -> Result<(), String> {

    let width = 1200;
    let height = 1000;

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
    let texture_creator = canvas.texture_creator();

    let textures = vec![
        texture_creator.load_texture("src/game/assets/icon-play.png")?,
        texture_creator.load_texture("src/game/assets/icon-pause.png")?,
        texture_creator.load_texture("src/game/assets/icon-pencil.png")?,
        texture_creator.load_texture("src/game/assets/icon-paint.png")?,
        texture_creator.load_texture("src/game/assets/icon-broom.png")?,
    ];

    GameOfLife::new(20, 20, width, height, sdl_context, canvas, &textures)?
        .start_game()?;

    Ok(())
}
