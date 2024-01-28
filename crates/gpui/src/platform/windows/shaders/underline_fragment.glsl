#version 450

#include "common.glsl"

layout (location = 0) out vec4 out_color;

layout (location = 0) flat in vec4 color;
layout (location = 1) flat in uint underline_id;

layout (binding = 0) readonly buffer RenderBuffer {
    Underline underlines[];
};

void main() {
    vec4 position = gl_FragCoord;

    Underline underline = underlines[underline_id];
    if (underline.wavy) {
        float half_thickness = underline.thickness * 0.5;
        vec2 origin = vec2(underline.bounds.origin.x, underline.bounds.origin.y);
        vec2 st = ((position.xy - origin) / underline.bounds.size.height) - vec2(0.0, 0.5);
        float frequency = (PI * (3.0 * underline.thickness)) / 8.0;
        float amplitude = 1.0 / (2.0 * underline.thickness);
        float sine = sin(st.x * frequency) * amplitude;
        float dSine = cos(st.x * frequency) * amplitude * frequency;
        float distance = (st.y - sine) / sqrt(1.0 + dSine * dSine);
        float distance_in_pixels = distance * underline.bounds.size.height;
        float distance_from_top_border = distance_in_pixels - half_thickness;
        float distance_from_bottom_border = distance_in_pixels + half_thickness;
        float alpha = clamp(0.5 - max(-distance_from_bottom_border, distance_from_top_border), 0.0, 1.0);
        out_color = color * vec4(1.0, 1.0, 1.0, alpha);
    } else {
        out_color = color;
    }
}