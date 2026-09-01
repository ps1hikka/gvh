#version 100
precision mediump float;

varying vec2 uv;
varying vec4 color;

uniform float time;
uniform float thickness;
uniform float pulse;

void main() {
    vec2 p = uv;

    float amp = 0.012 + pulse * 0.03;
    p.x += sin((uv.y * 18.0) + time * 4.0) * amp;
    p.y += sin((uv.x * 14.0) - time * 3.2) * amp * 0.7;

    float slash = uv.x * 0.08 + uv.y * 0.04;
    p.x += slash * (0.15 + pulse * 0.2);

    float t = clamp(thickness * 0.02, 0.035, 0.12);
    float d = min(min(p.x, p.y), min(1.0 - p.x, 1.0 - p.y));

    float line = step(d, t) * step(0.0, d);
    if (line < 0.5) {
        discard;
    }

    vec3 rgb = color.rgb;
    if (pulse > 0.5) {
        rgb *= 1.15;
    }
    gl_FragColor = vec4(rgb, 1.0);
}