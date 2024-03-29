struct Camera {
	view_pos: vec4<f32>,
	view_proj: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
	@location(0) position: vec3<f32>,
}

struct InstanceInput {
    @location(1) color: vec4<f32>,
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
};

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>,
}

@vertex
fn vs_main(
		model: VertexInput,
		instance: InstanceInput,
		) -> VertexOutput {

	var out: VertexOutput;
	let model_matrix = mat4x4<f32>(
		instance.model_matrix_0,
		instance.model_matrix_1,
		instance.model_matrix_2,
		instance.model_matrix_3,
	);


	let world_position =  model_matrix * vec4<f32>(model.position, 1.0);
	out.position = camera.view_proj * world_position;

	
	out.color = instance.color;
	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return in.color;
}
