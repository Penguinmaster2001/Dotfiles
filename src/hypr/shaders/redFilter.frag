precision mediump float;
varying vec2 v_texcoord;
uniform sampler2D tex;

void main() {

    vec4 pixColor = texture2D(tex, v_texcoord);

    pixColor[0] *= 1.25;
    pixColor[1] *= 1.0;
    pixColor[2] *= 0.75;

    gl_FragColor = pixColor;
}