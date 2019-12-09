import { Universe, Brick, GameStatus } from "wasm";
import { memory, universe_pad_height } from "wasm/wasm_bg";

const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const BACKGROUND_COLOUR = "#FFFFFF";
const BORDER_COLOUR = "#333333";
const MAIN_COLOUR = "#333333";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game");
canvas.height = height;
canvas.width = width;

document.addEventListener(
  "keypress",
  event => {
    if (event.key === "h") {
      universe.move_pad(false);
    } else if (event.key === "l") {
      universe.move_pad(true);
    } else if (event.key === "k") {
      universe.start_ball();
    }
  },
  false
);

const ctx = canvas.getContext("2d");

const getIndex = (row, column) => {
  return row * width + column;
};

const clearPanel = () => {
  ctx.beginPath();
  ctx.fillStyle = BACKGROUND_COLOUR;
  ctx.fillRect(0, 0, width, height);
  ctx.stroke();
};

const drawPad = () => {
  const top = universe.pad_top_position();
  const left = universe.pad_left_position();
  const height = universe.pad_height();
  const width = universe.pad_width();

  ctx.beginPath();
  ctx.fillStyle = MAIN_COLOUR;
  ctx.fillRect(left, top, width, height);
  ctx.stroke();
};

const drawBall = () => {
  const x = universe.ball_x_position();
  const y = universe.ball_y_position();
  const radius = universe.ball_radius();

  ctx.beginPath();
  ctx.fillStyle = MAIN_COLOUR;
  ctx.arc(x, y, radius, 0, Math.PI * 2, true);
  ctx.fill();
};

const drawBricks = () => {
  const rowCount = universe.row_count();
  const colCount = universe.column_count();
  const width = universe.brick_width();
  const height = universe.brick_height();
  const bricksPtr = universe.bricks();
  const marginHeight = universe.margin_height();
  const marginWidth = universe.margin_width();
  const bricks = new Uint8Array(memory.buffer, bricksPtr, rowCount * colCount);

  ctx.beginPath();
  ctx.fillStyle = MAIN_COLOUR;
  for (let row = 0; row < rowCount; row++) {
    for (let col = 0; col < colCount; col++) {
      if (bricks[row * colCount + col] === Brick.Alive) {
        ctx.fillRect(
          marginWidth + 1 + width * col,
          marginHeight + 1 + height * row,
          width - 2,
          height - 2
        );
      }
    }
  }
  ctx.stroke();
};

let animationId = null;

const isPaused = () => {
  return animationId === null;
};

const renderLoop = () => {
  const gameStatus = universe.get_status();
  if (gameStatus === GameStatus.Lost || gameStatus === GameStatus.Won) {
    const overlay = document.getElementById("overlay");
    overlay.style.visibility = "visible";
    const header = document.getElementById("header");
    header.textContent =
      gameStatus === GameStatus.Lost ? "Loser!!!" : "Winner!!!";
  }

  clearPanel();
  drawPad();
  drawBall();
  drawBricks();

  universe.tick();

  animationId = requestAnimationFrame(renderLoop);
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

play();
