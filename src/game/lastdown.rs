use sdl2::mouse::MouseButton;

pub struct Lastdown {
    pub x: i32,
    pub y: i32,
    pub b: MouseButton,
}

impl Default for Lastdown {
    fn default() -> Self {
        Self { x: -1, y: -1, b: MouseButton::Unknown }
    }
}
