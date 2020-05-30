precision highp float;

uniform vec2 canvasSize;
uniform sampler2D state;

varying vec2 v_uv;

const float DEAD_COLOR = 0.0;
const float ALIVE_COLOR = 0.1;
const float STILL_ALIVE_COLOR = 0.2;

vec4 textureOffset(vec2 uv, vec2 offset) {
    return texture2D(state, (uv * canvasSize + offset) / canvasSize);
}

float num(vec4 v) {
    if (v.x > 0.0) {
        return 1.0;
    } else {
        return 0.0;
    }
}

float numNeighboors(vec2 v_uv) {
    float left = num(textureOffset(v_uv, vec2(1, 0)));
    float right = num(textureOffset(v_uv, vec2(-1, 0)));
    float up = num(textureOffset(v_uv, vec2(0, -1)));
    float down = num(textureOffset(v_uv, vec2(0, 1)));

    float leftUp = num(textureOffset(v_uv, vec2(1, -1)));
    float rightUp = num(textureOffset(v_uv, vec2(-1, -1)));
    float leftDown = num(textureOffset(v_uv, vec2(1, 1)));
    float rightDown = num(textureOffset(v_uv, vec2(-1, 1)));

    return left + right + up + down + leftUp + leftDown + rightUp + rightDown;
}

const vec3 bitEnc = vec3(1.,255.,65025.);
const vec3 bitDec = 1./bitEnc;

vec3 dead() {
    return vec3(0.0, 0.0, 0.0);
}

vec3 deadInc(vec3 previous) {
    vec3 res = previous;
    res.x += 0.01;
    return res;
}

void main() {
    // vec2 v_uv = vec2(v_uv.x, 1.0 - v_uv.y);

    float neighboors = numNeighboors(v_uv);
    vec4 myColor = texture2D(state, v_uv);
    float myValue = num(myColor);

    if (myValue == 1.0 && neighboors < 2.0) {
        gl_FragColor = vec4(DEAD_COLOR, dead());
    } else if (myValue == 1.0 && neighboors == 2.0 || myValue == 1.0 && neighboors == 3.0) {
        gl_FragColor = vec4(STILL_ALIVE_COLOR, 0.0, 0.0, 0.0);
    } else if (myValue == 1.0 && neighboors > 3.0) {
        gl_FragColor = vec4(DEAD_COLOR, deadInc(myColor.gba));
    } else if (myValue == 0.0 && neighboors == 3.0) {
        gl_FragColor = vec4(ALIVE_COLOR, 0.0, 0.0, 0.0);
    } else {
        if (myValue == 1.0) {
            gl_FragColor = vec4(STILL_ALIVE_COLOR, 0.0, 0.0, 0.0);
        } else {
            gl_FragColor = vec4(DEAD_COLOR, deadInc(myColor.gba));
        }
    }
}