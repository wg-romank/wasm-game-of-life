precision highp float;

uniform vec2 canvasSize;
uniform sampler2D state;

varying vec2 v_uv;

void main() {
    // vec2 v_uv = vec2(v_uv.x, 1.0 - v_uv.y);
    vec4 value = texture2D(state, v_uv);

    if (value.x > 0.0) {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}