mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const WIDTH: u32 = 700;
const HEIGHT: u32 = 800;
const BALL_RADIUS: u32 = 10;
const BALL_SPEED: u32 = 4;
const PAD_WIDTH: u32 = 100;
const PAD_HEIGHT: u32 = 20;
const ROW_COUNT: u32 = 7;
const COLUMN_COUNT: u32 = 12;
const BRICK_HEIGHT: u32 = 30;
const BRICK_WIDTH: u32 = 50;

extern crate js_sys;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brick {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    bricks: Vec<Brick>,
    pad: Pad,
    ball: Ball,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.ball.tick(self.pad.left);
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

    // fn get_index(&self, row: u32, column: u32) -> usize {
    //     (row * self.width + column) as usize
    // }

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
        let width = WIDTH;
        let height = HEIGHT;

        let bricks = (0..COLUMN_COUNT * ROW_COUNT)
            .map(|_i| Brick::Alive)
            .collect();

        let pad = Pad {
            left: WIDTH / 2 - PAD_WIDTH / 2,
            top: HEIGHT - 2 * PAD_HEIGHT,
        };

        let ball = Ball {
            radius: BALL_RADIUS,
            x: WIDTH / 2,
            y: HEIGHT - 2 * PAD_HEIGHT - BALL_RADIUS,
            speed: BALL_SPEED,
            direction_right: true,
            direction_up: true,
        };

        Universe {
            width,
            height,
            bricks,
            pad,
            ball,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pad_top_position(&self) -> u32 {
        self.pad.top
    }

    pub fn pad_left_position(&self) -> u32 {
        self.pad.left
    }

    pub fn pad_width(&self) -> u32 {
        PAD_WIDTH
    }

    pub fn pad_height(&self) -> u32 {
        PAD_HEIGHT
    }

    pub fn move_pad(&mut self, right: bool) {
        if right {
            self.pad.move_right()
        } else {
            self.pad.move_left()
        }
    }

    pub fn bricks(&self) -> *const Brick {
        self.bricks.as_ptr()
    }

    pub fn bricks_count(&self) -> usize {
        self.bricks.len()
    }

    pub fn row_count(&self) -> u32 {
        ROW_COUNT
    }

    pub fn column_count(&self) -> u32 {
        COLUMN_COUNT
    }

    pub fn brick_height(&self) -> u32 {
        BRICK_HEIGHT
    }

    pub fn brick_width(&self) -> u32 {
        BRICK_WIDTH
    }

    // pub fn cells(&self) -> *const Cell {
    //     self.cells.as_ptr()
    // }

    // pub fn toggle_cell(&mut self, row: u32, column: u32) {
    //     let idx = self.get_index(row, column);
    //     self.cells[idx].toggle();
    // }

    pub fn ball_x_position(&self) -> u32 {
        self.ball.x
    }

    pub fn ball_y_position(&self) -> u32 {
        self.ball.y
    }

    pub fn ball_radius(&self) -> u32 {
        self.ball.radius
    }
}

struct Pad {
    top: u32,
    left: u32,
}

impl Pad {
    fn move_left(&mut self) {
        if self.left < 30 {
            self.left = 0
        } else {
            self.left -= 30
        }
    }

    fn move_right(&mut self) {
        if self.left >= WIDTH - PAD_WIDTH {
            self.left = WIDTH - PAD_WIDTH
        } else {
            self.left += 30
        }
    }
}

struct Ball {
    radius: u32,
    x: u32,
    y: u32,
    speed: u32,
    direction_up: bool,
    direction_right: bool,
}

impl Ball {
    fn tick(&mut self, left_pad: u32) {
        if self.direction_right {
            self.x += self.speed;
        } else {
            self.x -= self.speed;
        }
        if self.direction_up {
            self.y -= self.speed;
        } else {
            self.y += self.speed;
        }
        if self.x >= WIDTH - self.radius {
            self.direction_right = false
        }
        if self.x <= self.radius {
            self.direction_right = true
        }
        if self.y <= self.radius {
            self.direction_up = false
        }
        if self.y >= HEIGHT - 2 * PAD_HEIGHT - self.radius
            && self.x > left_pad
            && self.x < left_pad + PAD_WIDTH
        {
            self.direction_up = true
        } else if self.y >= HEIGHT - self.radius {
            self.speed = 0
        }
    }
}
