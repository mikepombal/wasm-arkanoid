import { Universe, Cell } from "wasm";
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

// canvas.addEventListener("click", event => {
//   const boundingRect = canvas.getBoundingClientRect();

//   const scaleX = canvas.width / boundingRect.width;
//   const scaleY = canvas.height / boundingRect.height;

//   const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
//   const canvasTop = (event.clientY - boundingRect.top) * scaleY;

//   const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
//   const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

//   universe.toggle_cell(row, col);

//   // drawCells();
// });

document.addEventListener(
  "keypress",
  event => {
    if (event.key === "h") {
      universe.move_pad(false);
    } else if (event.key === "l") {
      universe.move_pad(true);
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

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

let animationId = null;

const isPaused = () => {
  return animationId === null;
};

const renderLoop = () => {
  clearPanel();
  drawPad();
  drawBall();
  // drawCells();

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
