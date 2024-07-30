use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

use super::super::default::palette::*;
use super::toolbar::*;
use super::button::*;
use super::grid::Grid;
use super::cell::Cell;
use super::field::Field;

//? ///////////////////////////////////////////////////////////////////////

pub struct Renderer<'a> {
    width: u32,
    _height: u32,
    canvas: &'a mut WindowCanvas,
}

impl<'a> Renderer<'a> {
    pub fn new(width: u32, height: u32, canvas: &'a mut WindowCanvas) -> Result<Renderer, String> {

        Ok(Self {
            width,
            _height: height,
            canvas,
        })
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(palette_background());
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_button(&mut self, button: &Button) -> Result<(), String> {
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

    pub fn draw_toolbar(&mut self, toolbar: &Toolbar) -> Result<(), String> {
        const OUTLINING: u32 = 2;

        self.canvas.set_draw_color(palette_theme());
        self.canvas.fill_rect(Rect::new(0, 0, self.width, TOOLBAR_HEIGHT))?;
        
        self.canvas.set_draw_color(palette_border());
        self.canvas.draw_rect(Rect::new(0, 0, self.width, TOOLBAR_HEIGHT))?;
        self.canvas.fill_rect(Rect::new(0, TOOLBAR_HEIGHT as i32, self.width, OUTLINING))?;

        for q in toolbar.into_iter() {
            self.draw_button(q)?;
        }

        Ok(())
    }

    fn draw_cell(&mut self, cell: &Cell, field: &Field, size: u32,
                                row: u32, col: u32) -> Result<(), String> {

        const OUTLINING: i32 = 1;

        self.canvas.set_draw_color(palette_cell_border());
        let a = ( (size as f32) * field.get_scale() ) as i32;

        self.canvas.fill_rect(Rect::new(
            (row as i32)*a + (field.get_xpos() as i32),
            (col as i32)*a + (field.get_ypos() as i32),
            a as u32, a as u32
        ))?;

        match cell {
            Cell::Alive => self.canvas.set_draw_color(palette_cell_alive()),
            Cell::Dead  => self.canvas.set_draw_color(palette_cell_dead()),
        }

        self.canvas.fill_rect(Rect::new(
            (row as i32)*a + (field.get_xpos() as i32) + OUTLINING,
            (col as i32)*a + (field.get_ypos() as i32) + OUTLINING,
            std::cmp::max(0, (a - OUTLINING*2) as u32),
            std::cmp::max(0, (a - OUTLINING*2) as u32),
        ))?;

        Ok(())
    }

    pub fn draw_grid(&mut self, grid: &Grid<Cell>, field: &Field, size: u32
    ) -> Result<(), String> {

        let (rows, cols) = grid.size();
        for q in 0..rows {
            for w in 0..cols {
                self.draw_cell(grid.get(q, w), field, size, q as u32, w as u32)?;
            }
        }

        Ok(())
    }
}
