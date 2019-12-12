import { Universe, Brick, GameStatus } from "wasm";
import { memory, universe_pad_height } from "wasm/wasm_bg";
import { init } from "./canvas";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const movePad = right => universe.move_pad(right);
const startGame = () => universe.start_ball();

const getPadPosition = () => ({
  top: universe.pad_top_position(),
  left: universe.pad_left_position(),
  height: universe.pad_height(),
  width: universe.pad_width()
});

const getBallPosition = () => ({
  x: universe.ball_x_position(),
  y: universe.ball_y_position(),
  radius: universe.ball_radius()
});

const getBricksData = () => {
  const rowCount = universe.row_count();
  const colCount = universe.column_count();
  const bricksPtr = universe.bricks();

  return {
    rowCount,
    colCount,
    width: universe.brick_width(),
    height: universe.brick_height(),
    bricksPtr,
    marginHeight: universe.margin_height(),
    marginWidth: universe.margin_width(),
    bricks: new Uint8Array(memory.buffer, bricksPtr, rowCount * colCount)
  };
};

const getGameStatus = () => universe.get_status();

const tick = () => {
  universe.tick(Math.random());
};

init({
  height,
  width,
  getPadPosition,
  getBallPosition,
  getBricksData,
  getGameStatus,
  tick,
  movePad,
  startGame
});
