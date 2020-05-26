precision highp float;

uniform vec2 canvasSize;

varying vec2 v_uv;

void main() {
    vec2 vPosition = v_uv * canvasSize;

    float idx = canvasSize.x * vPosition.y + vPosition.x;

    // if (mod(idx, 2.0) == 0.0 || mod(idx, 7.0) == 0.0) {
    //     gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    // } else {
    //     gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    // }
    gl_FragColor = vec4(idx, 0.0, 1.0, 1.0);
}