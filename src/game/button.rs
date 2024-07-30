use super::super::default::textures::Textures;
use super::game_of_life::GameOfLife;

use sdl2::render::Texture;

//? ///////////////////////////////////////////////////////////////////////

pub struct PressButton {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
    texture: Textures
}

pub struct SwitchButton {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
    pressed: bool,
    texture_1: Textures,
    texture_2: Textures,
}

pub enum Button {
    PressButton(PressButton),
    SwitchButton(SwitchButton),
}

impl Button {
    pub fn get_rect(&self) -> (i32, i32, u32, u32) {
        match &self {
            Button::PressButton(e)  => { (e.x, e.y, e.w, e.h) },
            Button::SwitchButton(e) => { (e.x, e.y, e.w, e.h) },
        }
    }

    pub fn on_button(&self, mousex: i32, mousey: i32) -> bool {
        let (x, y, w, h) = self.get_rect();
        x < mousex && mousex < x + (w as i32) &&
        y < mousey && mousey < y + (h as i32)
    }

    pub fn on_press(&mut self, game: &mut GameOfLife) {
        match self {
            Button::PressButton(e)  => { e.on_press(game) },
            Button::SwitchButton(e) => { e.on_press(game) },
        }
    }
}

impl PressButton {
    pub fn new(x: i32, y: i32, w: u32, h: u32,
        on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
        texture: Textures
    ) -> Self {
        Self { x, y, w, h, on_press_fn, texture }
    }

    pub fn on_press(&mut self, game: &mut GameOfLife) {
        (self.on_press_fn)(game);
    }

    pub fn get_texture(&self) -> Textures {
        self.texture
    }
}

impl SwitchButton {
    pub fn new(x: i32, y: i32, w: u32, h: u32,
        on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
        texture_1: Textures,
        texture_2: Textures
    ) -> Self {
        Self { x, y, w, h,
            on_press_fn,
            pressed: false,
            texture_1,
            texture_2,
        }
    }

    pub fn on_press(&mut self, game: &mut GameOfLife) {
        (self.on_press_fn)(game);
        self.pressed = !self.pressed;
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    pub fn get_texture_1(&self) -> Textures {
        self.texture_1
    }

    pub fn get_texture_2(&self) -> Textures {
        self.texture_2
    }
}
