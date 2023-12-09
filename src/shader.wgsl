// Vertex shader

struct Camera {
view_pos: vec4<f32>,
	view_proj: mat4x4<f32>,
}

/*
@group(0) @binding(0)
var<uniform> camera: Camera;
*/

struct VertexInput {
	@location(0) position: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(
	model: VertexInput,
	) -> VertexOutput {
	var out: VertexOutput;
	out.position = vec4<f32>(model.position, 1.0);
	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return vec4<f32>(1.0,1.0,1.0, 1.0);
}
