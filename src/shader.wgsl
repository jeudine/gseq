// Vertex shader

struct Camera {
view_pos: vec4<f32>,
	view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct Light {
	position: vec3<f32>,
	color: vec3<f32>,
}

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
}

struct InstanceInput {
	@location(2) ambient: vec3<f32>,
	@location(3) diffuse: vec3<f32>,
	@location(4) spec: vec3<f32>,
	@location(5) shin: f32,
	@location(6) model_matrix_0: vec4<f32>,
	@location(7) model_matrix_1: vec4<f32>,
	@location(8) model_matrix_2: vec4<f32>,
	@location(9) model_matrix_3: vec4<f32>,
	@location(10) normal_matrix_0: vec3<f32>,
	@location(11) normal_matrix_1: vec3<f32>,
	@location(12) normal_matrix_2: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) ambient: vec3<f32>,
	@location(1) diffuse: vec3<f32>,
	@location(2) spec: vec3<f32>,
	@location(3) shin: f32,
	@location(4) world_normal: vec3<f32>,
	@location(5) world_position: vec3<f32>,
}

@vertex
fn vs_main(
	model: VertexInput,
	instance: InstanceInput,
	) -> VertexOutput {
	let model_matrix = mat4x4<f32>(
		instance.model_matrix_0,
		instance.model_matrix_1,
		instance.model_matrix_2,
		instance.model_matrix_3,
	);
	let normal_matrix = mat3x3<f32>(
		instance.normal_matrix_0,
		instance.normal_matrix_1,
		instance.normal_matrix_2,
	);
	var out: VertexOutput;
	out.ambient = instance.ambient;
	out.diffuse = instance.diffuse;
	out.spec = instance.spec;
	out.shin = instance.shin;
	out.world_normal = normal_matrix * model.normal;
	var world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
	out.world_position = world_position.xyz;
	out.clip_position = camera.view_proj * world_position;
	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return vec4<f32>(1.0,0.0,0.0, 1.0);
}
