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
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameStatus {
    Start = 0,
    Playing = 1,
    Lost = 2,
    Won = 3,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    bricks: Vec<Brick>,
    pad: Pad,
    ball: Ball,
    status: GameStatus,
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
    pub fn tick(&mut self, rand: f32) {
        match self.status {
            GameStatus::Playing => (),
            _ => {
                return;
            }
        }
        let mut dummy: f32 = 1.0;
        for j in 0..100_000_000 as i32 {
            if j % 2 == 0 {
                dummy = dummy * rand;
            } else {
                dummy = dummy / rand;
            }
        }
        // log(&dummy.to_string());

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

            let mut count = 0;
            for index in 0..COLUMN_COUNT * ROW_COUNT {
                let brick = self.bricks[index as usize];
                if brick == Brick::Alive {
                    let colision_type = self.check_brick_colision(target.x, target.y, index);
                    if colision_type > 0 {
                        self.bricks[index as usize] = Brick::Dead;
                        match colision_type {
                            1 => self.ball.direction_up = true,
                            2 => self.ball.direction_right = true,
                            3 => self.ball.direction_up = false,
                            4 => self.ball.direction_right = false,
                            _ => (),
                        }
                    } else {
                        count += 1
                    }
                }
            }
            if count == 0 {
                self.status = GameStatus::Won;
                return;
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
                self.status = GameStatus::Lost
            }
        }
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

        let status = GameStatus::Start;

        Universe {
            width,
            height,
            bricks,
            pad,
            ball,
            status,
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
        match self.status {
            GameStatus::Playing => {
                if right {
                    self.pad.move_right()
                } else {
                    self.pad.move_left()
                }
            }
            _ => {
                return;
            }
        }
    }

    pub fn start_ball(&mut self) {
        self.status = GameStatus::Playing
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

    pub fn ball_x_position(&self) -> u32 {
        self.ball.x
    }

    pub fn ball_y_position(&self) -> u32 {
        self.ball.y
    }

    pub fn ball_radius(&self) -> u32 {
        self.ball.radius
    }

    pub fn get_status(&self) -> GameStatus {
        self.status
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
