use super::super::default::textures::Textures;

//? ///////////////////////////////////////////////////////////////////////

pub struct PressButton {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    event_id: u32,
    texture: Textures
}

pub struct SwitchButton {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    event_id: u32,
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

    pub fn get_event_id(&mut self) -> u32 {
        match self {
            Button::PressButton(e)  => { e.event_id },
            Button::SwitchButton(e) => { e.event_id },
        }
    }

    pub fn switch_state(&mut self) {
        match self {
            Button::PressButton(_e)  => {},
            Button::SwitchButton(e) => { e.pressed = !e.pressed; },
        }
    }
}

impl PressButton {
    pub fn new(x: i32, y: i32, w: u32, h: u32,
        event_id: u32,
        texture: Textures
    ) -> Self {
        Self { x, y, w, h, event_id, texture }
    }

    pub fn get_texture(&self) -> Textures {
        self.texture
    }
}

impl SwitchButton {
    pub fn new(x: i32, y: i32, w: u32, h: u32,
        event_id: u32,
        texture_1: Textures,
        texture_2: Textures
    ) -> Self {
        Self { x, y, w, h,
            event_id,
            pressed: false,
            texture_1,
            texture_2,
        }
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
