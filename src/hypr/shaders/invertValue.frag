#version 300 es

precision mediump float;
in vec2 v_texcoord;
layout(location = 0) out vec4 fragColor;
uniform sampler2D tex;

void main()
{
    const float contrast = 1.0;

    // Invert value while preserving hue
    vec4 pixColor = texture(tex, v_texcoord);

    float max_g_b = max(pixColor.g, pixColor.b);

    float shift = pixColor.a - min(pixColor.r, max_g_b) - max(pixColor.r, max_g_b);

    pixColor = vec4(shift + pixColor.r, shift + pixColor.g, shift + pixColor.b, pixColor.a);

    // Increase contrast
    pixColor.rgb = (pixColor.rgb - 0.5) * contrast + 0.5;

    fragColor = pixColor;
}