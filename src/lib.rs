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
const MARGIN_HEIGHT: u32 = 100;
const MARGIN_WIDTH: u32 = 50;

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

struct Target {
    x: u32,
    y: u32,
}

enum BrickColision {
    No = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Left = 4,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for _step in 0..self.ball.speed {
            let target = match (self.ball.direction_right, self.ball.direction_up) {
                (true, true) => Target {
                    x: self.ball.x + 1,
                    y: self.ball.y - 1,
                },
                (true, false) => Target {
                    x: self.ball.x + 1,
                    y: self.ball.y + 1,
                },
                (false, true) => Target {
                    x: self.ball.x - 1,
                    y: self.ball.y - 1,
                },
                (false, false) => Target {
                    x: self.ball.x - 1,
                    y: self.ball.y + 1,
                },
            };

            for index in 0..COLUMN_COUNT * ROW_COUNT {
                let brick = self.bricks[index as usize];
                if brick == Brick::Alive && self.check_brick_colision(target.x, target.y, index) > 0
                {
                    self.bricks[index as usize] = Brick::Dead;
                }
            }

            if self.ball.direction_right {
                if target.x > WIDTH - self.ball.radius {
                    self.ball.direction_right = false
                } else {
                    self.ball.x += 1;
                }
            } else {
                self.ball.x -= 1;
            }
            if self.ball.direction_up {
                if target.y < self.ball.radius {
                    self.ball.direction_up = false
                } else {
                    self.ball.y -= 1;
                }
            } else {
                self.ball.y += 1;
            }
            if self.ball.x >= WIDTH - self.ball.radius {
                self.ball.direction_right = false
            }
            if self.ball.x <= self.ball.radius {
                self.ball.direction_right = true
            }
            if self.ball.y <= self.ball.radius {
                self.ball.direction_up = false
            }
            if self.ball.y >= HEIGHT - 2 * PAD_HEIGHT - self.ball.radius
                && self.ball.x > self.pad.left
                && self.ball.x < self.pad.left + PAD_WIDTH
            {
                self.ball.direction_up = true
            } else if self.ball.y >= HEIGHT - self.ball.radius {
                self.ball.speed = 0
            }
        }

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

    fn check_brick_colision(&self, ball_x: u32, ball_y: u32, index: u32) -> u8 {
        let brick_x = MARGIN_WIDTH + (index % COLUMN_COUNT) * BRICK_WIDTH;
        let brick_y = MARGIN_HEIGHT + (index / COLUMN_COUNT) * BRICK_HEIGHT;
        // temporary variables to set edges for testing
        let mut test_x = ball_x;
        let mut test_y = ball_y;

        // which edge is closest?
        if ball_x < brick_x {
            test_x = brick_x // left edge
        } else if ball_x > brick_x + BRICK_WIDTH {
            test_x = brick_x + BRICK_WIDTH // right edge
        }
        if ball_y < brick_y {
            test_y = brick_y
        } else if ball_y > brick_y + BRICK_HEIGHT {
            test_y = brick_y + BRICK_HEIGHT
        }

        // get distance from closest edges
        let dist_x = std::cmp::max(ball_x, test_x) - std::cmp::min(ball_x, test_x);
        let dist_y = std::cmp::max(ball_y, test_y) - std::cmp::min(ball_y, test_y);
        let distance = ((dist_x ^ 2) + (dist_y ^ 2)) ^ (1 / 2);

        if distance < BALL_RADIUS {
            if dist_x > dist_y {
                if ball_x > test_x {
                    return BrickColision::Right as u8;
                } else if ball_x < test_x {
                    return BrickColision::Left as u8;
                }
            }
            if ball_y > test_y {
                return BrickColision::Bottom as u8;
            } else if ball_y < test_y {
                return BrickColision::Top as u8;
            }
        }
        BrickColision::No as u8
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

    pub fn margin_height(&self) -> u32 {
        MARGIN_HEIGHT
    }

    pub fn margin_width(&self) -> u32 {
        MARGIN_WIDTH
    }

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
