// Vertex shader

struct TransformUniform {
    m: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> view_proj: TransformUniform;

@group(1) @binding(0)
var<uniform> transform: TransformUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = vec3<f32>(1.0, 1.0, 1.0);
    out.clip_position = view_proj.m * transform.m * vec4<f32>(in.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
