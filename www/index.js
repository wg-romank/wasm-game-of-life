import('../pkg')
  .catch(console.error);
// import { Universe, Cell } from "wasm-game-of-life";
// import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// const CELL_SIZE = 5;
// const GRID_COLOR = "#CCCCCC";
// const DEAD_COLOR = "#FFFFFF";
// const ALIVE_COLOR = "#000000";

// const universe = Universe.new(64);
// const width = universe.width();
// const height = universe.height();

// const canvas = document.getElementById("game-of-life-canvas");
// canvas.width = (CELL_SIZE + 1) * width + 1;
// canvas.height = (CELL_SIZE + 1) * height + 1;

// canvas.addEventListener("click", event => {
//   const boundingRect = canvas.getBoundingClientRect();
//   const scaleX = canvas.width / boundingRect.width;
//   const scaleY = canvas.height / boundingRect.height;
  
//   const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
//   const canvasTop = (event.clientY - boundingRect.top) * scaleY;

//   const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
//   const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

//   universe.toggle_cell(row, col);
// })

// const ctx = canvas.getContext("2d");

// const drawGrid = (width, height, ctx) => {
//     ctx.beginPath();
//     ctx.strokeKey = GRID_COLOR;

//     for (let i = 0; i < width; i++) {
//         ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
//         ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height, 1);
//     }

//     for (let j = 0; j < height; j++) {
//         ctx.moveTo(0, j * (CELL_SIZE + 1) + 1, 0);
//         ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
//     }

//     ctx.stroke();
// }

// const drawCells = (memory, universe, ctx) => {
//     const width = universe.width();
//     const height = universe.height();
//     const cellsPtr = universe.cells();

//     const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

//     ctx.beginPath();

//     for (let row = 0; row < height; row++) {
//     for (let col = 0; col < width; col++) {
//       const idx = universe.get_index(row, col);

//       ctx.fillStyle = cells[idx] === Cell.Dead
//         ? DEAD_COLOR
//         : ALIVE_COLOR;

//       ctx.fillRect(
//         col * (CELL_SIZE + 1) + 1,
//         row * (CELL_SIZE + 1) + 1,
//         CELL_SIZE,
//         CELL_SIZE
//       );
//     }
//   }

//   ctx.stroke();
// }

// let lastCall = 0;
// let cum = 0;

// const renderLoop = (timestamp) => {
//     const delta = timestamp - lastCall;
//     lastCall = timestamp;
//     cum += delta;

//     let fps = document.getElementById("frames-per-second").value;
//     if (cum > 1000 / fps) {
//       const ticksPerFrame = document.getElementById("ticks-per-frame").value;
//       universe.tick_many(ticksPerFrame);

//       drawGrid(width, height, ctx);
//       drawCells(memory, universe, ctx);

//       cum = 0;
//     }

//     requestAnimationFrame(renderLoop);
// }

// requestAnimationFrame(renderLoop);