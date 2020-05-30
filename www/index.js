import { setup_copy_program, setup_init_program, setup_compute_program, animation_webgl, setup_webgl } from "wasm-game-of-life";

let lastCall = 0;
let cum = 0;

let canvas = document.getElementById("game-of-life-canvas");
let brect = canvas.getBoundingClientRect();
canvas.setAttribute('width', brect.width);
canvas.setAttribute('height', brect.height);

let state = setup_webgl();
let init_program = setup_init_program();
let compute_program = setup_compute_program();
let copy_program = setup_copy_program();

// skip first 1000 iterations
for (let i = 0; i < 1000; i+= 1) {
  animation_webgl(init_program, compute_program, copy_program, state);
}


const renderLoop = (timestamp) => {
  const delta = timestamp - lastCall;
  lastCall = timestamp;
  cum += delta;

  let fps = document.getElementById("frames-per-second").value;
  if (cum > 1000 / fps) {
    animation_webgl(init_program, compute_program, copy_program, state);
    cum = 0;
  }

  requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);