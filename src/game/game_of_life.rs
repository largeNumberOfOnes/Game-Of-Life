use sdl2::controller::Button;
use sdl2::libc::YESEXPR;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;
use sdl2::event::{self, Event};
use sdl2::EventPump;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::Sdl;
use sdl2::render::{TextureCreator};
use sdl2::video::WindowContext;

use super::{button, toolbar};
// use 
use super::cell::Cell;
use super::field::Field;
use super::grid::Grid;
use super::renderer::Renderer;
use super::toolbar::{Toolbar, TOOLBAR_HEIGHT};
use super::double_buf::DoubleBuf;


use std::time::Duration;
use std::sync::Arc;

////////////////////////////////////////////////////////////////////////////////

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

const CELL_SIZE: u32 = 60;
const SIM_THRESHLD: u32 = 10;
const FPS: u32 = 60;
/// ////////////////////////////////////////////////////////////////////////////////

pub struct GameOfLife<'a> {
    buf: DoubleBuf<Grid<Cell>>,
    // textures: &'a Vec<Texture<'a>>,
    toolbar: std::cell::Cell<Option<Toolbar<'a>>>,
    renderer: Renderer,
    field: Field,
    mousex: i32,
    mousey: i32,
    lastdown: Lastdown,
    sdl_context: Sdl,
    play_state: bool,
    draw_state: bool,
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

impl<'a> GameOfLife<'a> {
    pub fn new(rows: usize, cols: usize, width: u32, height: u32,
        sdl_context: Sdl,
        canvas: WindowCanvas,
        textures: &'a Vec<Texture>
    ) -> Result<Self, String> 
    {
        let grid = Grid::new(rows, cols);
        Ok(Self {
            buf: DoubleBuf::new(grid.clone(), grid),
            toolbar: std::cell::Cell::new(
                Some(Toolbar::new()
                    .add_switch_button(
                        Box::new(|game| game.change_play_state()),
                        &textures[0],
                        &textures[1]
                    )
                    .add_switch_button(
                        Box::new(|game| game.change_draw_state()),
                        &textures[2],
                        &textures[3]
                    )
                    .add_press_button(
                        Box::new(|game| game.clear_grid() ),
                        &textures[4]
                    )
                )
            ),
            renderer: Renderer::new(width, height, canvas)?,
            field: Field::new(0.5, 5.0),
            mousex: 0,
            mousey: 0,
            lastdown: Lastdown::default(),
            sdl_context: sdl_context,
            play_state: false,
            draw_state: false,
        })
    }

    fn change_play_state(&mut self) {
        self.play_state = !self.play_state;
    }

    fn change_draw_state(&mut self) {
        self.draw_state = !self.draw_state;
    }

    fn clear_grid(&mut self) {
        // println!("This is clear_grid(). Mouse pos ({}, {})", self.mousex, self.mousey);
        let (rows, cols) = self.buf.get_cur().size();
        for q in 0..rows {
            for w in 0..cols {
                self.buf.get_cur_mut().set(Cell::Dead, q, w);
            }
        }
    }

    fn step(&mut self) {
        // // println!("STEP");
        let (rows, cols) = self.buf.get_cur().size();
        let buf = &mut self.buf;
        for q in 0..rows {
            for w in 0..cols {
                let count_of_alive = count_of_alive(buf.get_cur(), q, w);
                match buf.get_cur().get(q, w) {
                    Cell::Alive => {
                        if !(2 <= count_of_alive && count_of_alive <=3) {
                            buf.get_buf_mut().set(Cell::Dead, q, w);
                        } else {
                            buf.get_buf_mut().set(Cell::Alive, q, w);
                        }
                    },
                    Cell::Dead => {
                        if count_of_alive == 3 {
                            buf.get_buf_mut().set(Cell::Alive, q, w);
                        } else {
                            buf.get_buf_mut().set(Cell::Dead, q, w);
                        }
                    },
                }
                // let new_cell = match *self.get_cur_grid().get(q, w) {
                //     Cell::Dead => 
                //         if count_of_alive == 3 
                //             {Cell::Alive} else {Cell::Dead}
                //     Cell::Alive =>
                //         if !(2 <= count_of_alive && count_of_alive <=3) 
                //             {Cell::Alive} else {Cell::Dead}
                // };
                // self.get_buf_grid().set(new_cell, q, w);
                
            }
        }
        buf.switch();
    }

    fn render(&mut self) ->Result<(), String> {
        self.renderer.clear();
        self.renderer.draw_grid(
            self.buf.get_cur(),
            &self.field,
            CELL_SIZE
        )?;
        if let Some(ref toolbar) = self.toolbar.get_mut() {
            self.renderer.draw_toolbar(toolbar)?;
        }

        self.renderer.present();
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

    fn is_grid_area(&self, x: i32, y: i32) -> bool {
        (TOOLBAR_HEIGHT as i32) < y
    }

    fn event_pump_processor(&mut self, mut event_pump: EventPump) -> bool {
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    // break 'running;
                    return true;
                },
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    self.lastdown = Lastdown { x: x, y: y, b: mouse_btn };
    //                 if self.home_button.on_button(
    //                     (x as f32)/(width as f32),
    //                     (y as f32)/(height as f32)
    //                 ) {
    //                     self.field.home();
    //                 }
    //                 if self.stop_button.on_button(
    //                     (x as f32)/(width as f32),
    //                     (y as f32)/(height as f32)
    //                 ) {
    //                     self.play_stay = !self.play_stay;
    //                 }
    //                 if self.rest_button.on_button(
    //                     (x as f32)/(width as f32),
    //                     (y as f32)/(height as f32)
    //                 ) {
    //                     for q in 0..rows {
    //                         for w in 0..cols {
    //                             self.grid[0].set(Cell::Dead, q, w);
    //                             self.grid[1].set(Cell::Dead, q, w);
    //                         }
    //                     }
    //                 }
    //                 if !self.play_stay && mouse_btn == MouseButton::Right {
    //                     let field = &self.field;
    //                     match Self::get_pressed_cell(rows, cols, field, x, y, width, height) {
    //                         Some((x, y)) => {
    //                             println!("{}, {}", x, y);
    //                             let cell = self.grid[0].get(x as usize, y as usize);
    //                             let cell = match cell {
    //                                 Cell::Alive => Cell::Dead ,
    //                                 Cell::Dead  => Cell::Alive,
    //                             };
    //                             self.grid[0].set(cell.clone(), x as usize, y as usize);
    //                             self.grid[1].set(cell.clone(), x as usize, y as usize);
    //                         },
    //                         None => println!("None"),
    //                     }
    //                 }
                },
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    self.process_press_toolbar(x, y, mouse_btn);
                    self.process_press_grid(x, y, mouse_btn);
                    self.lastdown = Lastdown::default();
                },
                Event::MouseMotion { mousestate, x, y, xrel, yrel, .. } => {
                    // println!("mouse motion: ({}, {}), ({}, {})", x, y, xrel, yrel);
                    if mousestate.is_mouse_button_pressed(MouseButton::Left) {
                        if self.is_grid_area(x-xrel, y-yrel) {
                            self.field.shift(xrel as f32, yrel as f32);
                        }
                    }
                    self.mousex = x;
                    self.mousey = y;
                },
                Event::MouseWheel { timestamp, window_id, which, x, y, direction, precise_x, precise_y, mouse_x, mouse_y } => {
                    // println!("mouse wheel: ({}, {}), {:?}, ({}, {}), ({}, {})", x, y, direction, precise_x, precise_y, mouse_x, mouse_y);
                    if self.is_grid_area(mouse_x, mouse_y) {
                        self.field.scale(
                            mouse_x as f32,
                            mouse_y as f32,
                            0.05*precise_y
                        )
                    }
                }
                _ => {}
            }

        }

        false
    }

    fn process_press_toolbar(&mut self, x: i32, y: i32, b: MouseButton) {
        let mut toolbar_opt = self.toolbar.replace(None);
        if let Some(mut toolbar) = toolbar_opt {
            for q in toolbar.get_buttons() {
                if q.on_button(self.lastdown.x, self.lastdown.y) &&
                q.on_button(x, y) &&
                self.lastdown.b == b &&
                b == MouseButton::Left
                {
                    q.on_press(self);
                }
            }
            toolbar_opt = Some(toolbar);
        } else {
            panic!("Something strange!");
        }
        self.toolbar.replace(toolbar_opt);
    }

    fn process_press_grid(&mut self, x: i32, y: i32, b: MouseButton) {
        if !(
            self.draw_state &&
            self.is_grid_area(x, y) &&
            self.lastdown.b == b &&
            b == MouseButton::Right 
        ) { return; }
        if let Some((q, w)) = self.get_pressed_cell(x, y) {
            let grid = self.buf.get_cur_mut();
            match grid.get(q, w) {
                Cell::Alive => grid.set(Cell::Dead,  q, w),
                Cell::Dead  => grid.set(Cell::Alive, q, w),
            }
        }
    }

    pub fn start_game(&mut self) -> Result<(), String> {

        let mut counter = 0;

        loop {
            
            let event_pump = self.sdl_context.event_pump()?;

            if self.event_pump_processor(event_pump) {
                break;
            }
            
            if self.play_state && counter > 10 {
                self.step();
                counter = 0;
            }
            counter += 1;
            // println!("mouse pos: ({}, {})", self.mousex, self.mousey);
            // println!("cell: ({:?})", self.get_pressed_cell(self.mousex, self.mousey));
            
            self.render();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }

        Ok(())
    }
}
