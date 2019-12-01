mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate js_sys;

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
    cells: Vec<Cell>,
    pad: Pad,
    ball: Ball
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        // let mut next = self.cells.clone();

        // for row in 0..self.height {
        //     for col in 0..self.width {
        //         let idx = self.get_index(row, col);
        //         let cell = self.cells[idx];
        //         let live_neighbors = self.live_neighbor_count(row, col);

        //         let next_cell = match (cell, live_neighbors) {
        //             // Rule 1: Any live cell with fewer than two live neighbours
        //             // dies, as if caused by underpopulation.
        //             (Cell::Alive, x) if x < 2 => Cell::Dead,
        //             // Rule 2: Any live cell with two or three live neighbours
        //             // lives on to the next generation.
        //             (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
        //             // Rule 3: Any live cell with more than three live
        //             // neighbours dies, as if by overpopulation.
        //             (Cell::Alive, x) if x > 3 => Cell::Dead,
        //             // Rule 4: Any dead cell with exactly three live neighbours
        //             // becomes a live cell, as if by reproduction.
        //             (Cell::Dead, 3) => Cell::Alive,
        //             // All other cells remain in the same state.
        //             (otherwise, _) => otherwise,
        //         };

        //         next[idx] = next_cell;
        //     }
        // }

        // self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    //     let mut count = 0;
    //     for delta_row in [self.height - 1, 0, 1].iter().cloned() {
    //         for delta_col in [self.width - 1, 0, 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }

    //             let neighbor_row = (row + delta_row) % self.height;
    //             let neighbor_col = (column + delta_col) % self.width;
    //             let idx = self.get_index(neighbor_row, neighbor_col);
    //             count += self.cells[idx] as u8;
    //         }
    //     }
    //     count
    // }

    pub fn new() -> Universe {
        let width = 700;
        let height = 800;

        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() < 0.5  {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let pad = Pad { x: width / 2 };

        let ball = Ball { x: width / 2, y: height / 2 };

        Universe {
            width,
            height,
            cells,
            pad,
            ball,
        }
    }

    // pub fn render(&self) -> String {
    //     self.to_string()
    // }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pad_position(&self) -> u32 {
        self.pad.position()
    }

    pub fn move_pad(&mut self, right: bool) {
        if right {
            self.pad.move_right()
        } else {
            self.pad.move_left()
        }
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn ball_x_position(&self) -> u32 {
        self.ball.x_position()
    }

    pub fn ball_y_position(&self) -> u32 {
        self.ball.y_position()
    }

}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

struct Pad {
    x: u32
}

impl Pad {
    fn position(&self) -> u32 {
        self.x
    }

    fn move_left(&mut self) {
        self.x = self.x - 30
    }

    fn move_right(&mut self) {
        self.x = self.x + 30
    }
}

struct Ball {
    x: u32,
    y: u32
}

impl Ball {
    fn x_position(&self) -> u32 {
        self.x
    }

    fn y_position(&self) -> u32 {
        self.y
    }

}
