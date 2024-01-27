#version 450

#include "common.glsl"

layout (location = 0) out vec4 out_color;

layout (location = 0) flat in vec4 color;
layout (location = 1) flat in uint shadow_id;

layout (binding = 0) readonly buffer RenderBuffer {
    Shadow shadows[];
};

void main() {
    vec4 position = gl_FragCoord;

    Shadow shadow = shadows[shadow_id];
    vec2 origin = vec2(shadow.bounds.origin.x, shadow.bounds.origin.y);
    vec2 size = vec2(shadow.bounds.size.width, shadow.bounds.size.height);
    vec2 half_size = size / 2.0;
    vec2 center = origin + half_size;
    vec2 point = position.xy - center;
    float corner_radius = 0.0;

    if (point.x < 0.0) {
        if (point.y < 0.0) {
            corner_radius = shadow.corner_radii.top_left;
        } else {
            corner_radius = shadow.corner_radii.bottom_left;
        }
    } else {
        if (point.y < 0.0) {
            corner_radius = shadow.corner_radii.top_right;
        } else {
            corner_radius = shadow.corner_radii.bottom_right;
        }
    }

    // The signal is only non-zero in a limited range, so don't waste samples
    float low = point.y - half_size.y;
    float high = point.y + half_size.y;
    float start = clamp(-3.0 * shadow.blur_radius, low, high);
    float end = clamp(3.0 * shadow.blur_radius, low, high);

    // Accumulate samples (we can get away with surprisingly few samples)
    float step = (end - start) / 4.0;
    float y = start + step * 0.5;
    float alpha = 0.0;
    for (int i = 0; i < 4; i++) {
        alpha += blur_along_x(point.x, point.y - y, shadow.blur_radius, corner_radius, half_size) * gaussian(y, shadow.blur_radius) * step;
        y += step;
    }

    out_color = color * vec4(1.0, 1.0, 1.0, alpha);
}