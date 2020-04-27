use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

const CELL_SIZE: u32 = 5;

pub fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
    let document = web_sys::window()?.document()?;
    let canvas = document.get_element_by_id("game-of-life-canvas")?;

    canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok()
}

pub fn setup_canvas(universe: &Universe, context: &str) -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
    let canvas = get_canvas().ok_or(JsValue::from_str("Failed getting canvas"))?;
    canvas.set_width((CELL_SIZE + 1) * universe.width() + 1);
    canvas.set_height((CELL_SIZE + 1) * universe.height() + 1);

    let ctx = canvas.get_context(context)?.ok_or(JsValue::from_str("Failed getting ctx"))?;

    ctx.dyn_into::<web_sys::CanvasRenderingContext2d>().map_err(|e| JsValue::from(e))
}

pub fn draw_grid(ctx: &web_sys::CanvasRenderingContext2d, universe: &Universe) -> Result<(), JsValue> {
    ctx.begin_path();
    ctx.set_stroke_style(&JsValue::from_str("gray"));

    let float_width = universe.width() as f64;
    let float_height = universe.height() as f64;
    let csf = CELL_SIZE as f64;

    for i in 0..universe.width() + 1 {
        let fi = i as f64;
        ctx.move_to(fi * (csf + 1.) + 1., 0.);
        ctx.line_to(fi * (csf + 1.) + 1., (csf + 1.) * float_height + 1.);
    }

    for j in 0..universe.height() + 1 {
        let fj = j as f64;
        ctx.move_to(0., fj * (csf + 1.) + 1.);
        ctx.line_to((csf + 1.) * float_width + 1., fj * (csf + 1.) + 1.);
    }

    ctx.stroke();

    Ok(())
}

pub fn draw_cells(ctx: &web_sys::CanvasRenderingContext2d, universe: &Universe) -> Result<(), JsValue> {
    let fcs = CELL_SIZE as f64;

    ctx.begin_path();

    for i in 0..universe.height() {
        for j in 0..universe.width() {
            let idx = universe.get_index(i, j);

            let stroke_style = match universe.cells[idx] {
                Cell::Dead => "white",
                Cell::Alive => "black"
            };

            ctx.set_fill_style(&JsValue::from(stroke_style));
            ctx.fill_rect((i as f64) * (fcs + 1.) + 1., (j as f64) * (fcs + 1.) + 1., fcs, fcs);
        }
    }

    ctx.stroke();

    Ok(())
}

pub fn animation_loop(ctx: &web_sys::CanvasRenderingContext2d, mut universe: Universe) -> Result<Universe, JsValue> {
    universe.tick();
    draw_grid(&ctx, &universe);
    draw_cells(&ctx, &universe);

    // web_sys::window()?.request_animation_frame(
    //     || animation_loop(&ctx, &universe)
    // )
    Ok(universe)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut universe = Universe::new(64);
    let ctx = setup_canvas(&universe, "2d")?;

    for _ in 0..100 {
        universe = animation_loop(&ctx, universe)?;
    }

    Ok(())
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn live_neighboors_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighboor_row = (row + delta_row) % self.height;
                let neighboor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighboor_row, neighboor_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn tick_many(&mut self, steps: u32) {
        for _ in 0..steps {
            self.tick()
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighboors = self.live_neighboors_count(row, col);

                let next_cell = match (cell, live_neighboors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new(size: u32) -> Universe {
        let width = size;
        let height = size;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 { Cell::Alive }
                else { Cell::Dead }
            }).collect();

        Universe { width, height, cells }
    }

    pub fn render(&self) -> String { self.to_string() }

    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }

    pub fn cells(&self) -> *const Cell { self.cells.as_ptr() }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }    
}

#[test]
fn unreachable_executed() {
    let mut universe = Universe::new(4);

    for _ in 0..10 {
        println!("{}", universe.to_string());
        universe.tick();
    }
    println!("{}", universe.to_string());
}