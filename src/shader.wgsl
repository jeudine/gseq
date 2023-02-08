// Vertex shader

struct TransformUniform {
    m: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> view_proj: TransformUniform;

@group(1) @binding(0)
var<uniform> transform: TransformUniform;

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(2) @binding(0)
var<uniform> light: Light;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
	@location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = view_proj.m * transform.m * vec4<f32>(in.position, 1.0);
    out.color = vec3<f32>(1.0, 1.0, 1.0);
    //out.world_normal = normal_matrix
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let object_color: vec4<f32> = vec4<f32>(in.color, 1.0);

	let ambient_strength = 0.1;
	let ambient_color = light.color * ambient_strength;

    let result = ambient_color * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}
