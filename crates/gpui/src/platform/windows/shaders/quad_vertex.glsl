#version 450

#include "common.glsl"

layout (location = 0) flat out vec4 out_background_color;
layout (location = 1) flat out vec4 out_border_color;
layout (location = 2) flat out uint out_quad_id;

layout (binding = 0) readonly buffer RenderBuffer {
    Quad quads[];
};

layout (push_constant) uniform PushConstant {
    Size_DevicePixels viewport_size;
};

void main() {
    int unit_vertex_id = gl_VertexIndex;
    int quad_id = gl_InstanceIndex;
    
    vec2 unit_vertex = unit_vertices[unit_vertex_id];
    Quad quad = quads[quad_id];
    vec4 device_position = to_device_position(unit_vertex, quad.bounds, viewport_size);
    vec4 clip_distance = distance_from_clip_rect(unit_vertex, quad.bounds, quad.content_mask.bounds);
    vec4 background_color = hsla_to_rgba(quad.background);
    vec4 border_color = hsla_to_rgba(quad.border_color);

    gl_Position = device_position;
    out_background_color = background_color;
    out_border_color = border_color;
    out_quad_id = quad_id;
    gl_ClipDistance[0] = clip_distance.x;
    gl_ClipDistance[1] = clip_distance.y;
    gl_ClipDistance[2] = clip_distance.z;
    gl_ClipDistance[3] = clip_distance.w;
}