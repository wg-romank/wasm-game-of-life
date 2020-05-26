precision highp float;

attribute vec2 position;

uniform vec2 canvasSize;

varying vec2 vPosition;

void main() {
    vec2 zeroOne = position / canvasSize;
    vec2 clipSpace = zeroOne * 2.0 - 1.0;

    gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
    vPosition = position;
}
