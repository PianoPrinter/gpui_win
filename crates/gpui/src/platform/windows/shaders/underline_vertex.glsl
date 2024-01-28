#version 450

#include "common.glsl"

layout (location = 0) flat out vec4 out_color;
layout (location = 1) flat out uint out_underline_id;

layout (binding = 0) readonly buffer RenderBuffer {
    Underline underlines[];
};

layout (push_constant) uniform PushConstant {
    Size_DevicePixels viewport_size;
};

void main() {
    int unit_vertex_id = gl_VertexIndex;
    int underline_id = gl_InstanceIndex;

    vec2 unit_vertex = unit_vertices[unit_vertex_id];
    Underline underline = underlines[underline_id];
    vec4 device_position = to_device_position(unit_vertex, underline.bounds, viewport_size);
    vec4 clip_distance = distance_from_clip_rect(unit_vertex, underline.bounds, underline.content_mask.bounds);
    vec4 color = hsla_to_rgba(underline.color);

    gl_Position = device_position;
    out_color = color;
    out_underline_id = underline_id;
    gl_ClipDistance[0] = clip_distance.x;
    gl_ClipDistance[1] = clip_distance.y;
    gl_ClipDistance[2] = clip_distance.z;
    gl_ClipDistance[3] = clip_distance.w;
}