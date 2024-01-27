#version 450

#include "common.glsl"

layout (location = 0) flat out vec4 out_color;
layout (location = 1) flat out uint out_shadow_id;

layout (binding = 0) readonly buffer RenderBuffer {
    Shadow shadows[];
};

layout (push_constant) uniform PushConstant {
    Size_DevicePixels viewport_size;
};

void main() {
    int unit_vertex_id = gl_VertexIndex;
    int shadow_id = gl_InstanceIndex;

    vec2 unit_vertex = unit_vertices[unit_vertex_id];
    Shadow shadow = shadows[shadow_id];

    float margin = 3.0 * shadow.blur_radius;
    // Set the bounds of the shadow and adjust its size based on the shadow's
    // spread radius to achieve the spreading effect
    Bounds_ScaledPixels bounds = shadow.bounds;
    bounds.origin.x -= margin;
    bounds.origin.y -= margin;
    bounds.size.width += 2.0 * margin;
    bounds.size.height += 2.0 * margin;

    vec4 device_position = to_device_position(unit_vertex, bounds, viewport_size);
    vec4 clip_distance = distance_from_clip_rect(unit_vertex, bounds, shadow.content_mask.bounds);
    vec4 color = hsla_to_rgba(shadow.color);

    gl_Position = device_position;
    out_color = color;
    out_shadow_id = shadow_id;
    gl_ClipDistance[0] = clip_distance.x;
    gl_ClipDistance[1] = clip_distance.y;
    gl_ClipDistance[2] = clip_distance.z;
    gl_ClipDistance[3] = clip_distance.w;
}