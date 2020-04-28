use wasm_bindgen::prelude::*;

use std::collections::HashSet;

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
    alive: HashSet<(u32, u32)>
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

    pub fn tick_many(&mut self, steps: u32) {
        for _ in 0..steps {
            self.tick();
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

                if next_cell != self.cells[idx] {
                    match next_cell {
                        Cell::Alive => self.alive.insert((row, col)),
                        Cell::Dead => self.alive.remove(&(row, col)),
                    };
                }

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn render(&self) -> String { self.to_string() }

    pub fn cell(&self, idx: usize) -> Cell { self.cells[idx] }

    pub fn alive_cells(&self) -> &HashSet<(u32, u32)> { &self.alive }
}


macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn new(size: u32) -> Universe {
        let width = size;
        let height = size;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 { Cell::Alive }
                else { Cell::Dead }
            }).collect();

        Universe { width, height, cells, alive: HashSet::new() }
    }

    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }
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

    println!("{}", universe.to_string());
    universe.toggle_cell(1, 1);
    // universe.tick();
    println!("{}", universe.to_string());
    panic!("A")
}