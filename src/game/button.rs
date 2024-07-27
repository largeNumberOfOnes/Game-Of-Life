use super::game_of_life::GameOfLife;

use sdl2::render::Texture;

//? ///////////////////////////////////////////////////////////////////////

pub struct PressButton<'a> {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
    texture: &'a Texture<'a>,
}

pub struct SwitchButton<'a> {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
    pressed: bool,
    texture_1: &'a Texture<'a>,
    texture_2: &'a Texture<'a>,
}

pub enum Button<'a> {
    PressButton(PressButton<'a>),
    SwitchButton(SwitchButton<'a>),
}

impl<'a> Button<'a> {
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

impl<'a> PressButton<'a> {
    pub fn new(x: i32, y: i32, w: u32, h: u32,
        on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
        texture: &'a Texture<'a>
    ) -> Self {
        Self { x, y, w, h, on_press_fn, texture }
    }

    pub fn on_press(&mut self, game: &mut GameOfLife) {
        (self.on_press_fn)(game);
    }

    pub fn get_texture(&self) -> &'a Texture<'a> {
        &self.texture
    }
}

impl<'a> SwitchButton<'a> {
    pub fn new(x: i32, y: i32, w: u32, h: u32,
        on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
        texture_1: &'a Texture<'a>,
        texture_2: &'a Texture<'a>
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

    pub fn get_texture_1(&self) -> &'a Texture<'a> {
        &self.texture_1
    }

    pub fn get_texture_2(&self) -> &'a Texture<'a> {
        &self.texture_2
    }
}
