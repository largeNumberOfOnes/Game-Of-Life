use sdl2::pixels::Color;

//? ///////////////////////////////////////////////////////////////////////

pub fn palette_background    () -> Color { unsafe { SD[0] } }
pub fn palette_theme         () -> Color { unsafe { SD[1] } }
// pub fn palette_element       () -> Color { unsafe { sd[2] } }
// pub fn palette_select        () -> Color { unsafe { sd[3] } }
// pub fn palette_active        () -> Color { unsafe { sd[4] } }
// pub fn palette_active_select () -> Color { unsafe { sd[5] } }
pub fn palette_border        () -> Color { unsafe { SD[6] } }
pub fn palette_cell_alive    () -> Color { unsafe { SD[7] } }
pub fn palette_cell_dead     () -> Color { unsafe { SD[8] } }
pub fn palette_cell_border   () -> Color { unsafe { SD[9] } }
pub fn palette_text          () -> Color { unsafe { SD[10] } }

//? ///////////////////////////////////////////////////////////////////////

// DARK
static SD_DARK: [Color; 11] = [
    Color::RGB( 50,  50,  50), // BACKGROUND
    Color::RGB( 50,  50,  50), // THEME
    Color::RGB(127,  86, 231), // ELEMENT
    Color::RGB(133,  94, 230), // SELECT
    Color::RGB(133,   0,   0), // ACTIVE
    Color::RGB(133,   0,   0), // ACTIVE_SELECT
    Color::RGB( 10,  10,  10), // BORDER

    Color::RGB(140, 140, 140), // CELL_ALIVE
    Color::RGB( 70,  70,  70), // CELL_DEAD
    Color::RGB( 30,  30,  30), // CELL_BORDER
    
    Color::RGB(240, 240, 240), // TEXT
];

// LIGHT
static SD_LIGHT: [Color; 11] = [
    Color::RGB(255, 255, 255), // BACKGROUND
    Color::RGB(220, 220, 220), // THEME
    Color::RGB(127,  86, 231), // ELEMENT
    Color::RGB(133,  94, 230), // SELECT
    Color::RGB(133,   0,   0), // ACTIVE
    Color::RGB(133,   0,   0), // ACTIVE_SELECT
    Color::RGB( 10,  10,  10), // BORDER

    Color::RGB(140, 140, 140), // CELL_ALIVE
    Color::RGB( 70,  70,  70), // CELL_DEAD
    Color::RGB( 30,  30,  30), // CELL_BORDER
    
    Color::RGB(240, 240, 240), // TEXT
];

static mut SD: &[Color; 11] = &SD_DARK;
//? ///////////////////////////////////////////////////////////////////////

fn set(sdp: &'static[Color; 11]) {
    unsafe { SD = sdp; }
}

pub fn set_dark() {
    set(&SD_DARK);
}

pub fn set_light() {
    set(&SD_LIGHT);
}

enum Status {
    DARK,
    LIGHT,
}

static mut STATUS: Status = Status::DARK;
pub fn set_other() {
    unsafe {
        match STATUS {
            Status::DARK => {
                set_light();
                STATUS = Status::LIGHT;
            },
            Status::LIGHT => {
                set_dark();
                STATUS = Status::DARK;
            },
        }
    }
}
