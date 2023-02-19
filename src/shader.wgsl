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

@group(1) @binding(0)
var<uniform> light_0: Light;
@group(1) @binding(1)
var<uniform> light_1: Light;
@group(1) @binding(2)
var<uniform> light_2: Light;

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
}

struct InstanceInput {
	@location(2) model_matrix_0: vec4<f32>,
	@location(3) model_matrix_1: vec4<f32>,
	@location(4) model_matrix_2: vec4<f32>,
	@location(5) model_matrix_3: vec4<f32>,
	@location(6) normal_matrix_0: vec3<f32>,
	@location(7) normal_matrix_1: vec3<f32>,
	@location(8) normal_matrix_2: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) color: vec3<f32>,
	@location(1) world_normal: vec3<f32>,
	@location(2) world_position: vec3<f32>,
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
	out.color = vec3<f32>(0.6, 0.6, 0.6);
	out.world_normal = normal_matrix * model.normal;
	var world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
	out.world_position = world_position.xyz;
	out.clip_position = camera.view_proj * world_position;
	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let object_color: vec4<f32> = vec4<f32>(in.color, 1.0);

	let ambient_strength = 0.1;
	let ambient_color = (light_0.color + light_1.color + light_2.color) * ambient_strength;

	let light_dir_0 = normalize(light_0.position - in.world_position);
	let diffuse_strength_0 = max(dot(in.world_normal, light_dir_0), 0.0);
	let diffuse_color_0 = light_0.color * diffuse_strength_0;

	let light_dir_1 = normalize(light_1.position - in.world_position);
	let diffuse_strength_1 = max(dot(in.world_normal, light_dir_1), 0.0);
	let diffuse_color_1 = light_1.color * diffuse_strength_1;

	let light_dir_2 = normalize(light_2.position - in.world_position);
	let diffuse_strength_2 = max(dot(in.world_normal, light_dir_2), 0.0);
	let diffuse_color_2 = light_2.color * diffuse_strength_2;

	let diffuse_color = diffuse_color_0 + diffuse_color_1 + diffuse_color_2;
	
	let view_dir = normalize(camera.view_pos.xyz - in.world_position);
	let half_dir_0 = normalize(view_dir + light_dir_0);
	let half_dir_1 = normalize(view_dir + light_dir_1);
	let half_dir_2 = normalize(view_dir + light_dir_2);
	
	let specular_strength_0 = pow(max(dot(in.world_normal, half_dir_0), 0.0), 256.0);
	let specular_strength_1 = pow(max(dot(in.world_normal, half_dir_1), 0.0), 256.0);
	let specular_strength_2 = pow(max(dot(in.world_normal, half_dir_2), 0.0), 256.0);
	let specular_color_0 = specular_strength_0 * light_0.color;
	let specular_color_1 = specular_strength_1 * light_1.color;
	let specular_color_2 = specular_strength_2 * light_2.color;

	let specular_color = specular_color_0 + specular_color_1 + specular_color_2;

	let result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;

	return vec4<f32>(result, object_color.a);
}
