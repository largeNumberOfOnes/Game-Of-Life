use sdl2::mouse::MouseButton;

struct Lastdown {
    x: i32,
    y: i32,
    b: MouseButton,
}

impl Default for Lastdown {
    fn default() -> Self {
        Self { x: -1, y: -1, b: MouseButton::Unknown }
    }
}
