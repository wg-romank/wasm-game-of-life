precision highp float;

uniform vec2 canvasSize;

varying vec2 vPosition;

void main() {
    float idx = canvasSize.x * vPosition.x + vPosition.y;

    if (mod(idx, 2.0) == 0.0 || mod(idx, 7.0) == 0.0) {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}