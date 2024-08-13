precision mediump float;
varying vec2 v_texcoord;
uniform sampler2D tex;

void main()
{
    const float contrast = 1.5;

    // Invert value while preserving hue
    vec4 pixColor = texture2D(tex, v_texcoord);

    float max_g_b = max(pixColor.g, pixColor.b);

    float shift = pixColor.a - min(pixColor.r, max_g_b) - max(pixColor.r, max_g_b);

    pixColor = vec4(shift + pixColor.r, shift + pixColor.g, shift + pixColor.b, pixColor.a);

    // Increase contrast
    pixColor.rgb = (pixColor.rgb - 0.5) * contrast + 0.5;

    gl_FragColor = pixColor;
}