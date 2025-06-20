use super::super::default::textures::Textures;
use super::button::{self, *};

//? ///////////////////////////////////////////////////////////////////////

pub const TOOLBAR_HEIGHT: u32 = 40;
const TOOLBAR_BUTTON_INDENT: i32 = 5;
const TOOLBAR_BUTTON_WIDTH:  i32 = 30;

//? ///////////////////////////////////////////////////////////////////////

pub struct Toolbar {
    buttons: Vec<Button>,
}

impl Toolbar {
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
        event_id: u32,
        texture_1: Textures,
        texture_2: Textures
    ) -> Self {

        self.buttons.push(Button::SwitchButton(
            SwitchButton::new(
                TOOLBAR_BUTTON_INDENT + self.calc_shift(),
                TOOLBAR_BUTTON_INDENT,
                TOOLBAR_BUTTON_WIDTH as u32,
                TOOLBAR_BUTTON_WIDTH as u32,
                event_id,
                texture_1, texture_2
            )
        ));

        self
    }

    pub fn add_press_button(mut self, 
        event_id: u32,
        texture: Textures
    ) -> Self {

        self.buttons.push(Button::PressButton(
            PressButton::new(
                TOOLBAR_BUTTON_INDENT + self.calc_shift(),
                TOOLBAR_BUTTON_INDENT,
                TOOLBAR_BUTTON_WIDTH as u32,
                TOOLBAR_BUTTON_WIDTH as u32,
                event_id,
                texture,
            )
        ));

        self
    }

    pub fn into_iter(&self) -> std::slice::Iter<'_, button::Button> {
        self.buttons.iter()
    }

    pub fn get_buttons(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }
}
