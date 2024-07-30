use sdl2::render::{Texture, WindowCanvas};
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::{set_error, EventPump};
use sdl2::libc;
use sdl2::Sdl;

use std::time::Duration;

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
    toolbar: std::cell::Cell<Option<Toolbar<'a>>>,
    field: Field,
    mousex: i32,
    mousey: i32,
    lastdown: Lastdown,
    play_state: bool,
    draw_state: bool,
    change_color_theme_event: u32,
    call_help_event: u32,
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

impl<'a> GameOfLife<'a> {
    pub fn new(
        width: u32,
        height: u32,
        rows: usize,
        cols: usize,
        sdl_context: &'a Sdl,
        textures: &'a Vec<&Texture>
    ) -> Result<Self, String>
    {

        let change_color_theme_event = unsafe {
            sdl_context.event()?.register_event()?
        };
        let call_help_event = unsafe {
            sdl_context.event()?.register_event()?
        };

        let grid = Grid::new(rows, cols);
        Ok(Self {
            width,
            height,
            sdl_context,
            buf: DoubleBuf::new(grid.clone(), grid),
            toolbar: std::cell::Cell::new(
                Some(Toolbar::new()
                    .add_switch_button( // play
                        Box::new(|game| game.change_play_state()),
                        &textures[0],
                        &textures[1]
                    )
                    .add_switch_button( // draw
                        Box::new(|game| game.change_draw_state()),
                        &textures[2],
                        &textures[3]
                    )
                    .add_press_button( // clear
                        Box::new(|game| game.clear_grid() ),
                        &textures[4]
                    )
                    .add_switch_button( // change color theme
                        Box::new(|game| game.change_color_theme()),
                        &textures[5],
                        &textures[5]
                    )
                    .add_press_button( // call help
                        Box::new(|game| game.call_help()),
                        &textures[6],
                    )
                )
            ),
            field: Field::new(SMALLEST_SCALE, LARGEST_SCALE),
            mousex: 0,
            mousey: 0,
            lastdown: Lastdown::default(),
            play_state: false,
            draw_state: false,
            change_color_theme_event: change_color_theme_event,
            call_help_event: call_help_event,
        })
    }

    fn change_play_state(&mut self) {
        self.play_state = !self.play_state;
    }

    fn change_draw_state(&mut self) {
        self.draw_state = !self.draw_state;
    }

    fn clear_grid(&mut self) {
        let (rows, cols) = self.buf.get_cur().size();
        for q in 0..rows {
            for w in 0..cols {
                self.buf.get_cur_mut().set(Cell::Dead, q, w);
            }
        }
    }

    fn change_color_theme(&mut self) {
        push_event(self.change_color_theme_event, self.sdl_context);
    }

    fn call_help(&mut self) {
        push_event(self.call_help_event, self.sdl_context);
    }

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

    fn render(&mut self, canvas: &mut WindowCanvas) ->Result<(), String> {
        let mut renderer = Renderer::new(self.width, self.height, canvas)?;

        renderer.clear();
        renderer.draw_grid(
            self.buf.get_cur(),
            &self.field,
            CELL_SIZE
        )?;
        if let Some(ref toolbar) = self.toolbar.get_mut() {
            renderer.draw_toolbar(toolbar)?;
        }

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
            Event::User { type_, .. } if type_ == self.change_color_theme_event => {
                return Ret::ChangeColorTheme;
            },
            Event::User { type_, .. } if type_ == self.call_help_event => {
                return Ret::Help;
            },
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                self.lastdown = Lastdown { x: x, y: y, b: mouse_btn };
            },
            Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                self.process_press_toolbar(x, y, mouse_btn);
                self.process_press_grid(x, y, mouse_btn);
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
            }
            _ => {}
        }}

        Ret::Continue
    }

    fn process_press_toolbar(&mut self, x: i32, y: i32, b: MouseButton) {
        let mut toolbar_opt = self.toolbar.replace(None);
        if let Some(mut toolbar) = toolbar_opt {
            for q in toolbar.get_buttons() {
                if  q.on_button(self.lastdown.x, self.lastdown.y) &&
                    q.on_button(x, y) &&
                    self.lastdown.b == b &&
                    b == MouseButton::Left
                {
                    q.on_press(self);
                }
            }
            toolbar_opt = Some(toolbar);
        }
        self.toolbar.replace(toolbar_opt);
    }

    fn process_press_grid(&mut self, x: i32, y: i32, b: MouseButton) {
        if !(
            self.draw_state &&
            self.is_grid_area(x, y) &&
            self.lastdown.b == b &&
            b == MouseButton::Right 
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

    pub fn game_loop(&mut self, canvas: &mut WindowCanvas) -> Result<Ret, String> {

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
            
            self.render(canvas)?;

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }

        Ok(ret)
    }
}
