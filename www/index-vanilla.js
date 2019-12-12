import { Brick, GameStatus } from "wasm";
import { init } from "./canvas";

const WIDTH = 700;
const HEIGHT = 800;
const PAD_WIDTH = 100;
const PAD_HEIGHT = 20;
const BALL_RADIUS = 10;
const BALL_SPEED = 4;
const ROW_COUNT = 7;
const COLUMN_COUNT = 12;
const BRICK_HEIGHT = 30;
const BRICK_WIDTH = 50;
const MARGIN_HEIGHT = 100;
const MARGIN_WIDTH = 50;

const BrickColision = {
  No: 0,
  Top: 1,
  Right: 2,
  Bottom: 3,
  Left: 4
};

const pad = {
  left: WIDTH / 2 - PAD_WIDTH / 2,
  top: HEIGHT - 2 * PAD_HEIGHT
};

const ball = {
  radius: BALL_RADIUS,
  x: WIDTH / 2,
  y: HEIGHT - 2 * PAD_HEIGHT - BALL_RADIUS,
  speed: BALL_SPEED,
  direction_right: true,
  direction_up: true
};

const bricks = new Uint8Array(ROW_COUNT * COLUMN_COUNT).map(() => 1);

let status = GameStatus.Start;

const movePad = right => {
  if (status !== GameStatus.Playing) {
    return;
  }

  if (right) {
    if (pad.left >= WIDTH - PAD_WIDTH) {
      pad.left = WIDTH - PAD_WIDTH;
    } else {
      pad.left += 30;
    }
  } else {
    if (pad.left < 30) {
      pad.left = 0;
    } else {
      pad.left -= 30;
    }
  }
};
const startGame = () => {
  status = GameStatus.Playing;
};

const getPadPosition = () => ({
  top: pad.top,
  left: pad.left,
  height: PAD_HEIGHT,
  width: PAD_WIDTH
});

const getBallPosition = () => ({
  x: ball.x,
  y: ball.y,
  radius: ball.radius
});

const getBricksData = () => {
  return {
    rowCount: ROW_COUNT,
    colCount: COLUMN_COUNT,
    width: BRICK_WIDTH,
    height: BRICK_HEIGHT,
    marginHeight: MARGIN_HEIGHT,
    marginWidth: MARGIN_WIDTH,
    bricks
  };
};

const getGameStatus = () => status;

const tick = () => {
  if (status !== GameStatus.Playing) {
    return;
  }

  const rand = Math.random();
  let dummy = 1.0;
  for (let j = 0; j < 100000000; j++) {
    if (j % 2 === 0) {
      dummy = dummy * rand;
    } else {
      dummy = dummy / rand;
    }
  }
  //   console.log(dummy);

  for (let i = 0; i < BALL_SPEED; i++) {
    const getTarget = () => {
      if ((ball.direction_right, ball.direction_up)) {
        return {
          x: ball.x + 1,
          y: ball.y - 1
        };
      } else if ((ball.direction_right, !ball.direction_up)) {
        return {
          x: ball.x + 1,
          y: ball.y + 1
        };
      } else if ((!ball.direction_right, ball.direction_up)) {
        return {
          x: ball.x - 1,
          y: ball.y - 1
        };
      } else {
        return {
          x: ball.x - 1,
          y: ball.y + 1
        };
      }
    };
    const target = getTarget();

    let count = 0;
    for (let index = 0; index < COLUMN_COUNT * ROW_COUNT; index++) {
      let brick = bricks[index];
      if (brick === Brick.Alive) {
        const colision_type = checkBrickCollision(target.x, target.y, index);
        if (colision_type > 0) {
          bricks[index] = Brick.Dead;
          switch (colision_type) {
            case BrickColision.Top: {
              ball.direction_up = true;
              break;
            }
            case BrickColision.Right: {
              ball.direction_right = true;
              break;
            }
            case BrickColision.Bottom: {
              ball.direction_up = false;
              break;
            }
            case BrickColision.Left: {
              ball.direction_right = false;
              break;
            }
          }
        } else {
          count += 1;
        }
      }
    }
    if (count === 0) {
      status = GameStatus.Won;
      return;
    }

    if (ball.direction_right) {
      if (target.x > WIDTH - ball.radius) {
        ball.direction_right = false;
      } else {
        ball.x += 1;
      }
    } else {
      if (ball.x <= ball.radius) {
        ball.direction_right = true;
      } else {
        ball.x -= 1;
      }
    }
    if (ball.direction_up) {
      if (target.y < ball.radius) {
        ball.direction_up = false;
      } else {
        ball.y -= 1;
      }
    } else {
      if (
        ball.y >= HEIGHT - 2 * PAD_HEIGHT - ball.radius &&
        ball.x > pad.left &&
        ball.x < pad.left + PAD_WIDTH
      ) {
        ball.direction_up = true;
      } else {
        ball.y += 1;
      }
    }
    if (ball.y >= HEIGHT - ball.radius) {
      status = GameStatus.Lost;
    }
  }
};

function checkBrickCollision(ball_x, ball_y, index) {
  const brick_x = MARGIN_WIDTH + (index % COLUMN_COUNT) * BRICK_WIDTH;
  const brick_y = MARGIN_HEIGHT + parseInt(index / COLUMN_COUNT) * BRICK_HEIGHT;
  // temporary variables to set edges for testing
  let test_x = ball_x;
  let test_y = ball_y;

  // which edge is closest?
  if (ball_x < brick_x) {
    test_x = brick_x; // left edge
  } else if (ball_x > brick_x + BRICK_WIDTH) {
    test_x = brick_x + BRICK_WIDTH; // right edge
  }
  if (ball_y < brick_y) {
    test_y = brick_y;
  } else if (ball_y > brick_y + BRICK_HEIGHT) {
    test_y = brick_y + BRICK_HEIGHT;
  }

  // get distance from closest edges
  const dist_x = Math.max(ball_x, test_x) - Math.min(ball_x, test_x);
  const dist_y = Math.max(ball_y, test_y) - Math.min(ball_y, test_y);
  const distance = Math.sqrt(Math.pow(dist_x, 2) + Math.pow(dist_y, 2));

  if (distance < BALL_RADIUS) {
    if (dist_x > dist_y) {
      if (ball_x > test_x) {
        return BrickColision.Right;
      } else if (ball_x < test_x) {
        return BrickColision.Left;
      }
    }
    if (ball_y > test_y) {
      return BrickColision.Bottom;
    } else if (ball_y < test_y) {
      return BrickColision.Top;
    }
  }
  return BrickColision.No;
}

init({
  height: HEIGHT,
  width: WIDTH,
  getPadPosition,
  getBallPosition,
  getBricksData,
  getGameStatus,
  tick,
  movePad,
  startGame
});
