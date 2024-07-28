use sdl2::pixels::Color;

//? ///////////////////////////////////////////////////////////////////////

pub const PALETTE_BACKGROUND: Color = Color::RGB(50, 50, 50);
pub const PALETTE_THEME:   Color = Color::RGB(50, 50, 50);
pub const PALETTE_ELEMENT: Color = Color::RGB(127,  86, 231);
pub const PALETTE_SELECT:  Color = Color::RGB(133,  94, 230);
pub const PALETTE_ACTIVE:  Color = Color::RGB(133,  0, 0);
pub const PALETTE_ACTIVE_SELECT:  Color = Color::RGB(133,  0, 0);
pub const PALETTE_BORDER: Color = Color::RGB( 10,   10,  10);

pub const PALETTE_CELL_ALIVE:  Color = Color::RGB(140,  140, 140);
pub const PALETTE_CELL_DEAD:   Color = Color::RGB( 70,   70,  70);
pub const PALETTE_CELL_BORDER: Color = Color::RGB( 30,   30,  30);
