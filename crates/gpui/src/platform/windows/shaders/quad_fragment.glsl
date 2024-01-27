#version 450

#include "common.glsl"

layout (location = 0) out vec4 out_color;

layout (location = 0) flat in vec4 background_color;
layout (location = 1) flat in vec4 border_color;
layout (location = 2) flat in uint quad_id;

layout (binding = 0) readonly buffer RenderBuffer {
    Quad quads[];
};

void main() {
    vec4 position = gl_FragCoord;

    Quad quad = quads[quad_id];
    vec2 half_size = vec2(quad.bounds.size.width, quad.bounds.size.height) / 2.0;
    vec2 center = vec2(quad.bounds.origin.x, quad.bounds.origin.y) + half_size;
    vec2 center_to_point = position.xy - center;
    float corner_radius = 0.0;

    if (center_to_point.x < 0.0) {
        if (center_to_point.y < 0.0) {
            corner_radius = quad.corner_radii.top_left;
        } else {
            corner_radius = quad.corner_radii.bottom_left;
        }
    } else {
        if (center_to_point.y < 0.0) {
            corner_radius = quad.corner_radii.top_right;
        } else {
            corner_radius = quad.corner_radii.bottom_right;
        }
    }

    vec2 rounded_edge_to_point = abs(center_to_point) - half_size + corner_radius;
    float distance = length(max(vec2(0.0), rounded_edge_to_point)) + min(0.0, max(rounded_edge_to_point.x, rounded_edge_to_point.y)) - corner_radius;

    float vertical_border = center_to_point.x <= 0.0 ? quad.border_widths.left : quad.border_widths.right;
    float horizontal_border = center_to_point.y <= 0.0 ? quad.border_widths.top : quad.border_widths.bottom;
    vec2 inset_size = half_size - corner_radius - vec2(vertical_border, horizontal_border);
    vec2 point_to_inset_corner = abs(center_to_point) - inset_size;
    float border_width = 0.0;

    if (point_to_inset_corner.x < 0.0 && point_to_inset_corner.y < 0.0) {
        border_width = 0.0;
    } else if (point_to_inset_corner.y > point_to_inset_corner.x) {
        border_width = horizontal_border;
    } else {
        border_width = vertical_border;
    }

    vec4 color = vec4(0.0);

    if (border_width == 0.0) {
        color = background_color;
    } else {
        float inset_distance = distance + border_width;
        // Blend the border on top of the background and then linearly interpolate
        // between the two as we slide inside the background.
        vec4 blended_border = over(background_color, border_color);
        color = mix(blended_border, background_color, clamp(0.5 - inset_distance, 0.0, 1.0));
    }

    out_color = color * vec4(1.0, 1.0, 1.0, clamp(0.5 - distance, 0.0, 1.0));
}