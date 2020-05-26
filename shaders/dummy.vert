precision highp float;

attribute vec2 position;
attribute vec2 uv;

varying vec2 v_uv;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_uv = uv;
}