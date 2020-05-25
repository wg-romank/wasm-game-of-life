import { Universe, setup_canvas, get_cell_size, animation_webgl, setup_webgl } from "wasm-game-of-life";

const universe = Universe.new(64);

let lastCall = 0;
let cum = 0;

setup_canvas(universe);
let state = setup_webgl();

const renderLoop = (timestamp) => {
    const delta = timestamp - lastCall;
    lastCall = timestamp;
    cum += delta;

    let fps = document.getElementById("frames-per-second").value;
    if (cum > 1000 / fps) {
      const ticksPerFrame = document.getElementById("ticks-per-frame").value;
      animation_webgl(state, universe, ticksPerFrame);
      cum = 0;
    }

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);

const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
const CELL_SIZE = get_cell_size();

canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);
})