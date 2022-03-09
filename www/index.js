import * as wg from "wasm-game-of-life";

import ZingTouch from 'zingtouch';

const canvas = document.getElementById("game-of-life-canvas");
const brect = canvas.getBoundingClientRect();
canvas.setAttribute('width', brect.width);
canvas.setAttribute('height', brect.height);

const cellsPerInch = 100;
const ppi = window.devicePixelRatio * 96;
const mWidth = Math.floor(brect.width / ppi * cellsPerInch);
const mHeight = Math.floor(brect.height / ppi * cellsPerInch)

const r = wg.Render.new(mWidth, mHeight);

let activeRegion = ZingTouch.Region(canvas);

activeRegion.bind(canvas, 'tap', () => {
  r.toggle_color();
});

let lastCall = 0;
let cum = 0;

// skip first 50 iterations
for (var i = 0; i < 50; i++) {
  r.frame();
}

const renderLoop = (timestamp) => {
  const delta = timestamp - lastCall;
  lastCall = timestamp;
  cum += delta;

  let fps = 60;
  if (cum > 1000 / fps) {
    r.frame()
    cum = 0;
  }

  requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
