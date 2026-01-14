#version 300 es

precision mediump float;
in vec2 v_texcoord;
uniform sampler2D tex;
layout(location = 0) out vec4 fragColor;

void main() {

    vec4 pixColor = texture(tex, v_texcoord);

    pixColor[0] *= 1.25;
    pixColor[1] *= 1.0;
    pixColor[2] *= 0.75;

    fragColor = pixColor;
}