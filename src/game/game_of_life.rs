use sdl2::render::{Texture, WindowCanvas};
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::libc;
use sdl2::Sdl;

use std::time::Duration;

use super::super::default::textures::Textures;
use super::lastdown::Lastdown;
use super::cell::Cell;
use super::field::Field;
use super::grid::Grid;
use super::renderer::Renderer;
use super::toolbar::*;
use super::double_buf::DoubleBuf;
use super::ret::Ret;

//? ///////////////////////////////////////////////////////////////////////

const CELL_SIZE: u32 = 60;
const PRESS_VALUATE: i32 = 3;
const SIM_THRESHLD: u32 = 10;
const FPS: u32 = 60;
const SMALLEST_SCALE: f32 =  0.5;
const LARGEST_SCALE:  f32 = 10.0;

//? ///////////////////////////////////////////////////////////////////////

pub struct GameOfLife<'a> {
    width: u32,
    height: u32,
    sdl_context: &'a Sdl,
    buf: DoubleBuf<Grid<Cell>>,
    toolbar: Toolbar,
    field: Field,
    mousex: i32,
    mousey: i32,
    lastdown: Lastdown,
    play_state: bool,
    draw_state: bool,
    events: UserEvents,
}

fn count_of_alive(grid: &Grid<Cell>, row: usize, col: usize) -> usize {
    grid.neighbours(row, col)
        .iter()
        .filter(|a| match grid.get(a.0, a.1) {
            Cell::Alive => true,
            Cell::Dead  => false,
        })
        .count()
}

fn push_event(event_id: u32, sdl_context: &Sdl) {
    let event = sdl2::event::Event::User {
        timestamp: 0,
        window_id: 0,
        type_: event_id,
        code: 456,
        data1: 0x1234 as *mut libc::c_void,
        data2: 0x5678 as *mut libc::c_void,
    };
    sdl_context.event().unwrap().push_event(event).unwrap();
}

struct UserEvents {
    play               : u32,
    draw               : u32,
    clear              : u32,
    change_color_theme : u32,
    hemo_field         : u32,
    call_help          : u32,
}

impl<'a> GameOfLife<'a> {
    pub fn new(
        width: u32,
        height: u32,
        rows: usize,
        cols: usize,
        sdl_context: &'a Sdl,
    ) -> Result<Self, String>
    {

        let events = UserEvents {
            play               : unsafe { sdl_context.event()?.register_event()? },
            draw               : unsafe { sdl_context.event()?.register_event()? },
            clear              : unsafe { sdl_context.event()?.register_event()? },
            change_color_theme : unsafe { sdl_context.event()?.register_event()? },
            hemo_field         : unsafe { sdl_context.event()?.register_event()? },
            call_help          : unsafe { sdl_context.event()?.register_event()? },
        };

        let grid = Grid::new(rows, cols);
        Ok(Self {
            width,
            height,
            sdl_context,
            buf: DoubleBuf::new(grid.clone(), grid),
            // toolbar: std::cell::Cell::new(
            //     Some(Toolbar::new()
            //         .add_switch_button( // play
            //             event_play,
            //             Textures::Play,
            //             Textures::Pause
            //         )
            //         .add_switch_button( // draw
            //             event_draw,
            //             Textures::Pencil,
            //             Textures::Paint
            //         )
            //         .add_press_button( // clear
            //             event_clear,
            //             Textures::Broom
            //         )
            //         .add_switch_button( // change color theme
            //             event_change_color_theme,
            //             Textures::Swap,
            //             Textures::Swap
            //         )
            //         .add_press_button( // home field
            //             event_hemo_field,
            //             Textures::Home
            //         )
            //         .add_press_button( // call help
            //             event_call_help,
            //             Textures::Help
            //         )
            //     )
            // ),
            toolbar: Toolbar::new()
                .add_switch_button( // play
                    events.play,
                    Textures::Play,
                    Textures::Pause
                )
                .add_switch_button( // draw
                    events.draw,
                    Textures::Pencil,
                    Textures::Paint
                )
                .add_press_button( // clear
                    events.clear,
                    Textures::Broom
                )
                .add_press_button( // change color theme
                    events.change_color_theme,
                    Textures::Swap
                )
                .add_press_button( // home field
                    events.hemo_field,
                    Textures::Home
                )
                .add_press_button( // call help
                    events.call_help,
                    Textures::Help
                )
            ,
            field: Field::new(
                SMALLEST_SCALE,
                LARGEST_SCALE,
                0 as f32,
                TOOLBAR_HEIGHT as f32
            ),
            mousex: 0,
            mousey: 0,
            lastdown: Lastdown::default(),
            play_state: false,
            draw_state: false,
            events,
        })
    }

    // fn change_play_state(&mut self) {
    //     self.play_state = !self.play_state;
    // }

    // fn change_draw_state(&mut self) {
    //     self.draw_state = !self.draw_state;
    // }

    fn clear_grid(&mut self) {
        let (rows, cols) = self.buf.get_cur().size();
        for q in 0..rows {
            for w in 0..cols {
                self.buf.get_cur_mut().set(Cell::Dead, q, w);
            }
        }
    }

    // fn change_color_theme(&mut self) {
    //     push_event(self.change_color_theme_event, self.sdl_context);
    // }

    // fn field_home(&mut self) {
    //     self.field.home();
    // }

    // fn call_help(&mut self) {
    //     push_event(self.call_help_event, self.sdl_context);
    // }

    fn step(&mut self) {
        let (rows, cols) = self.buf.get_cur().size();
        let buf = &mut self.buf;
        for q in 0..rows {
            for w in 0..cols {
                let count_of_alive = count_of_alive(buf.get_cur(), q, w);
                let new_cell = match buf.get_cur().get(q, w) {
                    Cell::Dead => if count_of_alive == 3 
                            { Cell::Alive }
                        else
                            { Cell::Dead }
                    Cell::Alive =>
                        if 2 <= count_of_alive && count_of_alive <=3
                            { Cell::Alive }
                        else
                            { Cell::Dead }
                };
                buf.get_buf_mut().set(new_cell, q, w);
                
            }
        }
        buf.switch();
    }

    fn render(
        &mut self,
        canvas: &mut WindowCanvas,
        textures: & [Texture]
    ) ->Result<(), String> {
        let mut renderer = Renderer::new(self.width, self.height, canvas)?;

        renderer.clear();
        renderer.draw_grid(
            self.buf.get_cur(),
            &self.field,
            CELL_SIZE
        )?;
        // if let Some(ref toolbar) = self.toolbar.get_mut() {
            renderer.draw_toolbar(&self.toolbar, textures)?;
        // }

        renderer.present();
        Ok(())
    }

    fn get_pressed_cell(&self, x: i32, y: i32) -> Option<(usize, usize)> {

        let (rows, cols) = self.buf.get_cur().size();

        let xn = (((x as f32) - self.field.get_xpos())
            / ((CELL_SIZE as f32)*self.field.get_scale()) ).floor() as i32;
        let yn = (((y as f32) - self.field.get_ypos())
            / ((CELL_SIZE as f32)*self.field.get_scale()) ).floor() as i32;

        if xn < 0 || xn >= rows as i32 { return None; }
        if yn < 0 || yn >= cols as i32 { return None; }
        
        Some((xn as usize, yn as usize))
    }

    fn is_grid_area(&self, _x: i32, y: i32) -> bool {
        (TOOLBAR_HEIGHT as i32) < y
    }

    fn event_pump_processor(&mut self, event_pump: &mut EventPump) -> Ret {
        
        for event in event_pump.poll_iter() { match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                return Ret::Quit;
            },
            Event::User { type_, .. } if type_ == self.events.play => {
                self.play_state = !self.play_state;
                self.toolbar.get_buttons()[0].switch_state();
            },
            Event::User { type_, .. } if type_ == self.events.draw => {
                self.draw_state = !self.draw_state;
                self.toolbar.get_buttons()[1].switch_state();
            },
            Event::User { type_, .. } if type_ == self.events.clear => {
                self.clear_grid()
            },
            Event::User { type_, .. } if type_ == self.events.change_color_theme => {
                return Ret::ChangeColorTheme;
            },
            Event::User { type_, .. } if type_ == self.events.hemo_field => {
                self.field.home();
            },
            Event::User { type_, .. } if type_ == self.events.call_help => {
                return Ret::Help;
            },
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                self.lastdown = Lastdown { x: x, y: y, b: mouse_btn };
            },
            Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                if  (x - self.lastdown.x).abs() <= PRESS_VALUATE &&
                    (y - self.lastdown.y).abs() <= PRESS_VALUATE &&
                    mouse_btn == self.lastdown.b
                {
                    self.process_press_toolbar(x, y, mouse_btn);
                    self.process_press_grid(x, y, mouse_btn);
                }
                self.lastdown = Lastdown::default();
            },
            Event::MouseMotion { mousestate, x, y, xrel, yrel, .. } => {
                if mousestate.is_mouse_button_pressed(MouseButton::Left) {
                    if self.is_grid_area(x-xrel, y-yrel) {
                        self.field.shift(xrel as f32, yrel as f32);
                    }
                }
                self.mousex = x;
                self.mousey = y;
            },
            Event::MouseWheel { precise_y, mouse_x, mouse_y, .. } => {
                if self.is_grid_area(mouse_x, mouse_y) {
                    self.field.scale(
                        mouse_x as f32,
                        mouse_y as f32,
                        0.05*precise_y
                    )
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                push_event(self.events.play, self.sdl_context);
            }
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                push_event(self.events.draw, self.sdl_context);
            }
            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                push_event(self.events.clear, self.sdl_context);
            }
            Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                push_event(self.events.change_color_theme, self.sdl_context);
            }
            Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                push_event(self.events.hemo_field, self.sdl_context);
            }
            Event::KeyDown { keycode: Some(Keycode::F1), .. } => {
                push_event(self.events.call_help, self.sdl_context);
            }
            _ => {}
        }}

        Ret::Continue
    }

    fn process_press_toolbar(&mut self, x: i32, y: i32, b: MouseButton) {
        for q in self.toolbar.get_buttons() {
            if  q.on_button(self.lastdown.x, self.lastdown.y) &&
                q.on_button(x, y) &&
                // self.lastdown.b == b &&
                b == MouseButton::Left
            {
                push_event(q.get_event_id(), self.sdl_context);
            }
        }
    }

    fn process_press_grid(&mut self, x: i32, y: i32, b: MouseButton) {
        if !(
            self.draw_state &&
            self.is_grid_area(x, y) &&
            // self.lastdown.b == b &&
            b == MouseButton::Left 
        ) {
            return;
        }
        if let Some((q, w)) = self.get_pressed_cell(x, y) {
            let grid = self.buf.get_cur_mut();
            match grid.get(q, w) {
                Cell::Alive => grid.set(Cell::Dead,  q, w),
                Cell::Dead  => grid.set(Cell::Alive, q, w),
            }
        }
    }

    pub fn game_loop(
        &mut self, canvas:
        &mut WindowCanvas,
        textures: &[Texture]
        // textures: &'a Vec<Texture>
    ) -> Result<Ret, String> {

        let mut counter = 0;

        let ret;

        let mut event_pump = self.sdl_context.event_pump()?;
        loop {

            let ret_ = self.event_pump_processor(&mut event_pump);
            if let Ret::Continue = ret_ {} else {
                ret = ret_;
                break;
            }
            
            if self.play_state && counter > SIM_THRESHLD {
                self.step();
                counter = 0;
            }
            counter += 1;
            
            self.render(canvas, textures)?;

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }

        Ok(ret)
    }
}
