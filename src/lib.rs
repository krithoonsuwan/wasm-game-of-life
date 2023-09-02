mod utils;
use fixedbitset::FixedBitSet;
use js_sys;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        web_sys::console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        web_sys::console::time_end_with_label(self.name);
    }
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

fn gen_cells(height: u32, width: u32) -> FixedBitSet {
    let size = (width * height) as usize;
    let mut cells = FixedBitSet::with_capacity(size);
    for i in 0..size {
        cells.set(i, js_sys::Math::random() < 0.5);
    }
    cells
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();
        log!("Universe::new({}, {})", width, height);
        // panic!();

        let cells = gen_cells(height, width);

        Universe {
            width,
            height,
            cells,
        }
    }
    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = gen_cells(self.height, width);
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = gen_cells(height, self.width);
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, rows: &[u32], cols: &[u32]) {
        for (i, row) in rows.iter().cloned().enumerate() {
            let idx = self.get_index(row, cols[i]);
            self.cells.set(idx, true);
        }
    }
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");

        let mut next = {
            // let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        {
            // let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);
                    next.set(
                        idx,
                        match (cell, live_neighbors) {
                            (true, x) if x < 2 => false,
                            (true, 2) | (true, 3) => true,
                            (true, x) if x > 3 => false,
                            (false, 3) => true,
                            (otherwise, _) => otherwise,
                        },
                    );
                }
            }
        }

        // let _timer = Timer::new("free old cells");
        self.cells = next;
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        ((row * self.width) + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &_cell in line {
                let symbol = '◻'; //if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
