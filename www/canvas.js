import { Brick, GameStatus } from "wasm";

const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const BACKGROUND_COLOUR = "#FFFFFF";
const BORDER_COLOUR = "#333333";
const MAIN_COLOUR = "#333333";

export const init = ({
  height,
  width,
  getPadPosition,
  getBallPosition,
  getBricksData,
  getGameStatus,
  tick,
  movePad,
  startGame
}) => {
  document.addEventListener(
    "keypress",
    event => {
      if (event.key === "h") {
        movePad(false);
      } else if (event.key === "l") {
        movePad(true);
      } else if (event.key === "k") {
        startGame();
      }
    },
    false
  );

  const canvas = document.getElementById("game");
  canvas.height = height;
  canvas.width = width;

  const ctx = canvas.getContext("2d");

  const clearPanel = () => {
    ctx.beginPath();
    ctx.fillStyle = BACKGROUND_COLOUR;
    ctx.fillRect(0, 0, width, height);
    ctx.stroke();
  };

  const drawPad = () => {
    const { top, left, height, width } = getPadPosition();
    ctx.beginPath();
    ctx.fillStyle = MAIN_COLOUR;
    ctx.fillRect(left, top, width, height);
    ctx.stroke();
  };

  const drawBall = () => {
    const { x, y, radius } = getBallPosition();
    ctx.beginPath();
    ctx.fillStyle = MAIN_COLOUR;
    ctx.arc(x, y, radius, 0, Math.PI * 2, true);
    ctx.fill();
  };

  const drawBricks = () => {
    const {
      rowCount,
      colCount,
      width,
      height,
      marginHeight,
      marginWidth,
      bricks
    } = getBricksData();
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
    const gameStatus = getGameStatus();
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

    tick();

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
};
