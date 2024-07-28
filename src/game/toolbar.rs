use super::game_of_life::GameOfLife;
use super::button::{self, *};

use sdl2::render::Texture;

//? ///////////////////////////////////////////////////////////////////////

pub const TOOLBAR_HEIGHT: u32 = 40;
const TOOLBAR_BUTTON_INDENT: i32 = 5;
const TOOLBAR_BUTTON_WIDTH:  i32 = 30;

//? ///////////////////////////////////////////////////////////////////////

pub struct Toolbar<'a> {
    buttons: Vec<Button<'a>>,
}

impl<'a> Toolbar<'a> {
    pub fn new() -> Self {
        Self {
            buttons: vec![]
        }
    }

    fn calc_shift(&self) -> i32 {
        (TOOLBAR_BUTTON_WIDTH + TOOLBAR_BUTTON_INDENT) * 
                                            (self.buttons.len() as i32)
    }

    pub fn add_switch_button(mut self, 
        on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
        texture_1: &'a Texture<'a>,
        texture_2: &'a Texture<'a>
    ) -> Self {

        self.buttons.push(Button::SwitchButton(
            SwitchButton::new(
                TOOLBAR_BUTTON_INDENT + self.calc_shift(),
                TOOLBAR_BUTTON_INDENT,
                TOOLBAR_BUTTON_WIDTH as u32,
                TOOLBAR_BUTTON_WIDTH as u32,
                on_press_fn,
                texture_1, texture_2
            )
        ));

        self
    }

    pub fn add_press_button(mut self, 
        on_press_fn: Box<dyn FnMut(&mut GameOfLife)>,
        texture: &'a Texture<'a>
    ) -> Self {

        self.buttons.push(Button::PressButton(
            PressButton::new(
                TOOLBAR_BUTTON_INDENT + self.calc_shift(),
                TOOLBAR_BUTTON_INDENT,
                TOOLBAR_BUTTON_WIDTH as u32,
                TOOLBAR_BUTTON_WIDTH as u32,
                on_press_fn,
                texture,
            )
        ));

        self
    }

    pub fn into_iter(&self) -> std::slice::Iter<'_, button::Button> {
        self.buttons.iter()
    }

    pub fn get_buttons(&mut self) -> &mut Vec<Button<'a>> {
        &mut self.buttons
    }
}
