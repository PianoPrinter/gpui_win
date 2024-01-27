const float PI = 3.14159265359;

const uint AtlasTextureKind_Monochrome = 0;
const uint AtlasTextureKind_Polychrome = 1;
const uint AtlasTextureKind_Path = 2;

const uint PathRasterizationInputIndex_Vertices = 0;
const uint PathRasterizationInputIndex_AtlasTextureSize = 1;

const uint ShadowInputIndex_Shadows = 1;

const uint SpriteInputIndex_Sprites = 1;
const uint SpriteInputIndex_AtlasTextureSize = 3;
const uint SpriteInputIndex_AtlasTexture = 4;

const uint SurfaceInputIndex_Surfaces = 1;
const uint SurfaceInputIndex_TextureSize = 3;
const uint SurfaceInputIndex_YTexture = 4;
const uint SurfaceInputIndex_CbCrTexture = 5;

const uint UnderlineInputIndex_Underlines = 1;

const vec2 unit_vertices[6] = vec2[6](
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(0.0, 1.0),
    vec2(0.0, 1.0),
    vec2(1.0, 0.0),
    vec2(1.0, 1.0)
);

struct Point_f32 {
    float x;
    float y;
};

struct Hsla {
    float h;
    float s;
    float l;
    float a;
};

struct AtlasTextureId {
    uint index;
    uint kind;
};

struct TileId {
    uint _0;
};

#define DevicePixels int

struct Point_DevicePixels {
    DevicePixels x;
    DevicePixels y;
};

struct Size_DevicePixels {
    DevicePixels width;
    DevicePixels height;
};

struct Bounds_DevicePixels {
    Point_DevicePixels origin;
    Size_DevicePixels size;
};

struct AtlasTile {
    AtlasTextureId texture_id;
    TileId tile_id;
    Bounds_DevicePixels bounds;
};

#define ScaledPixels float

struct Point_ScaledPixels {
    ScaledPixels x;
    ScaledPixels y;
};

struct Size_ScaledPixels {
    ScaledPixels width;
    ScaledPixels height;
};

struct Bounds_ScaledPixels {
    Point_ScaledPixels origin;
    Size_ScaledPixels size;
};

struct ContentMask_ScaledPixels {
    Bounds_ScaledPixels bounds;
};

struct PathVertex_ScaledPixels {
    Point_ScaledPixels xy_position;
    Point_f32 st_position;
    ContentMask_ScaledPixels content_mask;
};

struct ViewId {
    uint low_bits;
    uint high_bits;
};

#define LayerId uint

#define DrawOrder uint

struct Corners_ScaledPixels {
    ScaledPixels top_left;
    ScaledPixels top_right;
    ScaledPixels bottom_right;
    ScaledPixels bottom_left;
};

struct Shadow {
    ViewId view_id;
    LayerId layer_id;
    DrawOrder order;
    Bounds_ScaledPixels bounds;
    Corners_ScaledPixels corner_radii;
    ContentMask_ScaledPixels content_mask;
    Hsla color;
    ScaledPixels blur_radius;
};

struct Underline {
    ViewId view_id;
    LayerId layer_id;
    DrawOrder order;
    Bounds_ScaledPixels bounds;
    ContentMask_ScaledPixels content_mask;
    ScaledPixels thickness;
    Hsla color;
    bool wavy;
};

struct Edges_ScaledPixels {
    ScaledPixels top;
    ScaledPixels right;
    ScaledPixels bottom;
    ScaledPixels left;
};

struct Quad {
    ViewId view_id;
    LayerId layer_id;
    DrawOrder order;
    Bounds_ScaledPixels bounds;
    ContentMask_ScaledPixels content_mask;
    Hsla background;
    Hsla border_color;
    Corners_ScaledPixels corner_radii;
    Edges_ScaledPixels border_widths;
};

struct MonochromeSprite {
    ViewId view_id;
    LayerId layer_id;
    DrawOrder order;
    Bounds_ScaledPixels bounds;
    ContentMask_ScaledPixels content_mask;
    Hsla color;
    AtlasTile tile;
};

struct PolychromeSprite {
    ViewId view_id;
    LayerId layer_id;
    DrawOrder order;
    Bounds_ScaledPixels bounds;
    ContentMask_ScaledPixels content_mask;
    Corners_ScaledPixels corner_radii;
    AtlasTile tile;
    bool grayscale;
};

struct PathSprite {
    Bounds_ScaledPixels bounds;
    Hsla color;
    AtlasTile tile;
};

struct SurfaceBounds {
    Bounds_ScaledPixels bounds;
    ContentMask_ScaledPixels content_mask;
};

vec4 hsla_to_rgba(Hsla hsla) {
    float h = hsla.h * 6.0;
    float s = hsla.s;
    float l = hsla.l;
    float a = hsla.a;

    float c = (1.0 - abs(2.0 * l - 1.0)) * s;
    float x = c * (1.0 - abs(mod(h, 2.0) - 1.0));
    float m = l - c / 2.0;

    float r = 0.0;
    float g = 0.0;
    float b = 0.0;

    if (h >= 0.0 && h < 1.0) {
        r = c;
        g = x;
        b = 0.0;
    } else if (h >= 1.0 && h < 2.0) {
        r = x;
        g = c;
        b = 0.0;
    } else if (h >= 2.0 && h < 3.0) {
        r = 0.0;
        g = c;
        b = x;
    } else if (h >= 3.0 && h < 4.0) {
        r = 0.0;
        g = x;
        b = c;
    } else if (h >= 4.0 && h < 5.0) {
        r = x;
        g = 0.0;
        b = c;
    } else {
        r = c;
        g = 0.0;
        b = x;
    }

    vec4 rgba = vec4(0.0);
    rgba.x = (r + m);
    rgba.y = (g + m);
    rgba.z = (b + m);
    rgba.w = a;

    return rgba;
}

vec4 to_device_position(vec2 unit_vertex, Bounds_ScaledPixels bounds, Size_DevicePixels input_viewport_size) {
    vec2 position = unit_vertex * vec2(bounds.size.width, bounds.size.height) + vec2(bounds.origin.x, bounds.origin.y);
    vec2 viewport_size = vec2(float(input_viewport_size.width), float(input_viewport_size.height));
    vec2 device_position = position / viewport_size * vec2(2.0, -2.0) + vec2(-1.0, 1.0);
    device_position.y = -device_position.y;

    return vec4(device_position, 0.0, 1.0);
}

vec2 to_tile_position(vec2 unit_vertex, AtlasTile tile, Size_DevicePixels atlas_size) {
    vec2 tile_origin = vec2(tile.bounds.origin.x, tile.bounds.origin.y);
    vec2 tile_size = vec2(tile.bounds.size.width, tile.bounds.size.height);

    return (tile_origin + unit_vertex * tile_size) / vec2(float(atlas_size.width), float(atlas_size.height));
}

float quad_sdf(vec2 point, Bounds_ScaledPixels bounds, Corners_ScaledPixels corner_radii) {
    vec2 half_size = vec2(bounds.size.width, bounds.size.height) / 2.0;
    vec2 center = vec2(bounds.origin.x, bounds.origin.y) + half_size;
    vec2 center_to_point = point - center;
    float corner_radius = 0.0;

    if (center_to_point.x < 0.0) {
        if (center_to_point.y < 0.0) {
            corner_radius = corner_radii.top_left;
        } else {
            corner_radius = corner_radii.bottom_left;
        }
    } else {
        if (center_to_point.y < 0.0) {
            corner_radius = corner_radii.top_right;
        } else {
            corner_radius = corner_radii.bottom_right;
        }
    }

    vec2 rounded_edge_to_point = abs(center_to_point) - half_size + corner_radius;
    float distance = length(max(vec2(0.0), rounded_edge_to_point)) + min(0.0, max(rounded_edge_to_point.x, rounded_edge_to_point.y)) - corner_radius;

    return distance;
}

float gaussian(float x, float sigma) {
    return exp(-(x * x) / (2.0 * sigma * sigma)) / (sqrt(2.0 * PI) * sigma);
}

vec2 erf(vec2 x) {
    vec2 s = sign(x);
    vec2 a = abs(x);
    x = 1. + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    x *= x;

    return s - s / (x * x);
}

float blur_along_x(float x, float y, float sigma, float corner, vec2 half_size) {
    float delta = min(half_size.y - corner - abs(y), 0.0);
    float curved = half_size.x - corner + sqrt(max(0.0, corner * corner - delta * delta));
    vec2 integral = 0.5 + 0.5 * erf((x + vec2(-curved, curved)) * (sqrt(0.5) / sigma));

    return integral.y - integral.x;
}

vec4 distance_from_clip_rect(vec2 unit_vertex, Bounds_ScaledPixels bounds, Bounds_ScaledPixels clip_bounds) {
    vec2 position = unit_vertex * vec2(bounds.size.width, bounds.size.height) + vec2(bounds.origin.x, bounds.origin.y);

    return vec4(
        position.x - clip_bounds.origin.x,
        clip_bounds.origin.x + clip_bounds.size.width - position.x,
        position.y - clip_bounds.origin.y,
        clip_bounds.origin.y + clip_bounds.size.height - position.y
    );
}

vec4 over(vec4 below, vec4 above) {
    vec4 result = vec4(0.0);
    float alpha = above.a + below.a * (1.0 - above.a);
    result.rgb = (above.rgb * above.a + below.rgb * below.a * (1.0 - above.a)) / alpha;
    result.a = alpha;

    return result;
}
