import init, { World, Direction, GameStatus } from "snake-game";
import { rnd } from "./utils/rnd";

init().then((wasm) => {
  window.addEventListener("keydown", (e) => {
    switch (e.code) {
      case "ArrowUp":
      case "KeyW":
        world.set_snake_direction(Direction.Up);
        break;
      case "ArrowDown":
      case "KeyS":
        world.set_snake_direction(Direction.Down);
        break;
      case "ArrowLeft":
      case "KeyA":
        world.set_snake_direction(Direction.Left);
        break;
      case "ArrowRight":
      case "KeyD":
        world.set_snake_direction(Direction.Right);
        break;
    }
  });

  const gameControlBtn = document.getElementById("game-control-btn");
  const gameStatus = document.getElementById("game-status");
  const points = document.getElementById("points");
  const CELL_SIZE = 25;
  const WORLD_WIDTH = 15;
  const snakeSpawnIndex = rnd(WORLD_WIDTH * WORLD_WIDTH);
  const FPS = 10;
  const world = World.new(WORLD_WIDTH, snakeSpawnIndex, 3);
  const canvas = document.getElementById("root-canvas") as HTMLCanvasElement;
  const ctx = canvas.getContext("2d");
  const canvasDimensions = {
    width: WORLD_WIDTH * CELL_SIZE,
    height: WORLD_WIDTH * CELL_SIZE,
  };

  canvas.height = canvasDimensions.height;
  canvas.width = canvasDimensions.width;

  gameControlBtn.addEventListener("click", (_) => {
    const gameStatus = world.get_game_status();
    if (gameStatus !== GameStatus.Played) {
      gameControlBtn.textContent = "Playing..";
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });

  const drawWorld = () => {
    ctx.beginPath();

    // draw vertical lines
    for (let x = 0; x <= WORLD_WIDTH; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, canvasDimensions.height);
    }

    // draw horizontal lines
    for (let y = 0; y <= WORLD_WIDTH; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(canvasDimensions.width, CELL_SIZE * y);
    }

    ctx.stroke();
  };

  const drawSnake = () => {
    const snakeCellPtr = world.get_snake_cells();
    const snakeLen = world.get_snake_length();
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      snakeCellPtr,
      snakeLen,
    );
    snakeCells
      .slice()
      .reverse()
      .forEach((cellIdx, i) => {
        const col = cellIdx % WORLD_WIDTH;
        const row = Math.floor(cellIdx / WORLD_WIDTH);
        ctx.fillStyle = i === snakeCells.length - 1 ? "#7878db" : "#000000";
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        ctx.stroke();
      });
  };

  function drawReward() {
    const reward_idx = world.get_reward_cell();
    const col = reward_idx % WORLD_WIDTH;
    const row = Math.floor(reward_idx / WORLD_WIDTH);
    ctx.beginPath();
    ctx.fillStyle = "#FF0000";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }
  function drawGameStatus() {
    gameStatus.textContent = world.game_status_text();
    points.textContent = world.get_points().toString();
  }

  const paint = () => {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  };

  const play = () => {
    const status = world.get_game_status();
    if (status == GameStatus.Won || status == GameStatus.Lost) {
      gameControlBtn.textContent = "Re-play";
      return;
    }

    setTimeout(() => {
      ctx.clearRect(0, 0, canvasDimensions.width, canvasDimensions.height);
      world.step();
      paint();
      requestAnimationFrame(play);
    }, 1000 / FPS);
  };

  paint();
});
