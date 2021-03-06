import { setup_copy_program, setup_display_monochrome_program, setup_display_program, setup_compute_program, animation_webgl, setup_webgl } from "wasm-game-of-life";

import ZingTouch from 'zingtouch';

const canvas = document.getElementById("game-of-life-canvas");
const brect = canvas.getBoundingClientRect();
canvas.setAttribute('width', brect.width);
canvas.setAttribute('height', brect.height);

const color_program = setup_display_program();
const monochrome_program = setup_display_monochrome_program();
const compute_program = setup_compute_program();
const copy_program = setup_copy_program();

var draw_program = color_program;

let activeRegion = ZingTouch.Region(canvas);

activeRegion.bind(canvas, 'tap', () => {
  if (draw_program == color_program) {
    draw_program = monochrome_program;
  } else {
    draw_program = color_program;
  }
});

let main = (cellsPerInch) => {
  const ppi = window.devicePixelRatio * 96;
  const mWidth = Math.floor(brect.width / ppi * cellsPerInch);
  const mHeight = Math.floor(brect.height / ppi * cellsPerInch)
  
  const state = setup_webgl(mWidth, mHeight);

  let lastCall = 0;
  let cum = 0;

  // skip first 100 iterations
  for (let i = 0; i < 100; i+= 1) {
    animation_webgl(draw_program, compute_program, copy_program, mWidth, mHeight, state);
  }
  
  const renderLoop = (timestamp) => {
    const delta = timestamp - lastCall;
    lastCall = timestamp;
    cum += delta;
  
    let fps = 60;
    if (cum > 1000 / fps) {
      animation_webgl(draw_program, compute_program, copy_program, mWidth, mHeight, state);
      cum = 0;
    }
  
    requestAnimationFrame(renderLoop);
  }

  requestAnimationFrame(renderLoop);
}

main(50);