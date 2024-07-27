use sdl2::{filesystem, mouse, EventPump};
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::Sdl;
use sdl2::video::Window;

use super::{button, field};
use super::toolbar::*;
use super::button::*;
use super::grid::Grid;
use super::cell::Cell;
use super::field::Field;
use super::palette::*;

use std::sync::Arc;
pub struct Renderer {
    width: u32,
    height: u32,
    canvas: WindowCanvas,
    // sdl_context: Sdl,
}

impl Renderer {
    pub fn new(width: u32, height: u32, canvas: WindowCanvas) -> Result<Renderer, String> {

        Ok(Self {
            width,
            height,
            canvas,
            // sdl_context,
            // texture_creator,
        })
    }

    // fn load_textures(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Vec<Texture<'a>>, String> {
    //     let textures = vec![
    //         texture_creator.load_texture("assets/icon-font-play.svg")?
    //     ];
        
    //     Ok(textures)
    //     // unimplemented!();
    // }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(PALETTE_BACKGROUND);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_button(&mut self, button: &Button) -> Result<(), String> {
        // let (x, y, w, h) = button.get_rect();

        // self.canvas.set_draw_color(PALETTE_ELEMENT);
        // if let Button::SwitchButton(button_directly) = button {
        //     if button_directly.is_pressed() {
        //         self.canvas.set_draw_color(PALETTE_ACTIVE);
        //     }
        // }

        // self.canvas.fill_rect(Rect::new(x, y, w, h))?;

        let (x, y, w, h) = button.get_rect();
        let rect = Rect::new(x, y, w, h);

        match button {
            Button::PressButton(e) => {
                self.canvas.copy(e.get_texture(), None, Some(rect))?;
            },
            Button::SwitchButton(e) => {
                match e.is_pressed() {
                    true  => self.canvas.copy(e.get_texture_2(), None, Some(rect))?,
                    false => self.canvas.copy(e.get_texture_1(), None, Some(rect))?,
                }
                
            },
        }

        Ok(())
    }

    pub fn draw_button_textured_switch(&mut self, button: &Button,
        texture1: &Texture, texture2: &Texture
    ) -> Result<(), String> {
        let (x, y, w, h) = button.get_rect();
        let rect = Rect::new(x, y, w, h);

        // self.canvas.set_draw_color(PALETTE_ELEMENT);
        self.canvas.copy(&texture1, None, Some(rect)).unwrap();
        // if let Button::SwitchButton(button_directly) = button {
        //     if button_directly.is_pressed() {
        //         self.canvas.set_draw_color(PALETTE_ACTIVE);
        //     }
        // }

        // self.canvas.fill_rect()?;

        Ok(())
    }

    pub fn draw_toolbar(&mut self, toolbar: &Toolbar) -> Result<(), String> {
        // const 
        const OUTLINING: u32 = 2;

        self.canvas.set_draw_color(PALETTE_THEME);
        self.canvas.fill_rect(Rect::new(0, 0, self.width, TOOLBAR_HEIGHT))?;
        
        self.canvas.set_draw_color(PALETTE_BORDER);
        self.canvas.draw_rect(Rect::new(0, 0, self.width, TOOLBAR_HEIGHT))?;
        // self.canvas.fill_rect(Rect::new(0, TOOLBAR_HEIGHT as i32, self.width, OUTLINING))?;

        for q in toolbar.into_iter() {
            self.draw_button(q)?;
        }
        // self.draw_button_textured_switch(toolbar.get_buttons()[0], &textures[0], &textures[1]);

        Ok(())
    }

    fn draw_cell(&mut self, cell: &Cell, field: &Field, size: u32,
                                row: u32, col: u32) -> Result<(), String> {

        const OUTLINING: i32 = 1;

        self.canvas.set_draw_color(PALETTE_CELL_BORDER);
        let a = ( (size as f32) * field.get_scale() ) as i32;

        self.canvas.fill_rect(Rect::new(
            (row as i32)*a + (field.get_xpos() as i32),
            (col as i32)*a + (field.get_ypos() as i32),
            a as u32, a as u32
        ))?;

        match cell {
            Cell::Alive => self.canvas.set_draw_color(PALETTE_CELL_ALIVE),
            Cell::Dead  => self.canvas.set_draw_color(PALETTE_CELL_DEAD),
        }

        self.canvas.fill_rect(Rect::new(
            (row as i32)*a + (field.get_xpos() as i32) + OUTLINING,
            (col as i32)*a + (field.get_ypos() as i32) + OUTLINING,
            std::cmp::max(0, (a - OUTLINING*2) as u32),
            std::cmp::max(0, (a - OUTLINING*2) as u32),
        ))?;

        Ok(())
    }

    // pub fn get_event_pump(&self) -> Result<EventPump, String> {
    //     self.sdl_context.event_pump()
    // }

    // pub fn get_canvas(&self) -> &Canvas<Window> {
    //     &self.canvas
    // }

    pub fn draw_grid(&mut self, grid: &Grid<Cell>, field: &Field, size: u32) -> Result<(), String> {
        // const TOOLBAR_HEIGHT: u32 = 40;
        // const 
        self.canvas.set_draw_color(Color::RGB(100, 0, 0));
        // self.canvas.fill_rect(Rect::new(
        //     field.get_xpos() as i32,
        //     field.get_ypos() as i32,
        //     (200.0*field.get_scale()) as u32,
        //     (100.0*field.get_scale()) as u32
        // ))?;

        let (rows, cols) = grid.size();
        for q in 0..rows {
            for w in 0..cols {
                // let cell: Cell = self.get_cur_grid().get(q, w).clone();
                self.draw_cell(grid.get(q, w), field, size, q as u32, w as u32)?;
            }
        }

        Ok(())
    }
}
