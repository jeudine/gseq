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
@group(1) @binding(3)
var<uniform> light_3: Light;
@group(1) @binding(4)
var<uniform> light_4: Light;
@group(1) @binding(5)
var<uniform> light_5: Light;
@group(1) @binding(6)
var<uniform> light_6: Light;


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

	let ambient_color = in.ambient;

	let light_dir_0 = normalize(light_0.position - in.world_position);
	let diffuse_strength_0 = max(dot(in.world_normal, light_dir_0), 0.0);
	let diffuse_color_0 = light_0.color * diffuse_strength_0;

	let light_dir_1 = normalize(light_1.position - in.world_position);
	let diffuse_strength_1 = max(dot(in.world_normal, light_dir_1), 0.0);
	let diffuse_color_1 = light_1.color * diffuse_strength_1;

	let light_dir_2 = normalize(light_2.position - in.world_position);
	let diffuse_strength_2 = max(dot(in.world_normal, light_dir_2), 0.0);
	let diffuse_color_2 = light_2.color * diffuse_strength_2;

	let light_dir_3 = normalize(light_3.position - in.world_position);
	let diffuse_strength_3 = max(dot(in.world_normal, light_dir_3), 0.0);
	let diffuse_color_3 = light_3.color * diffuse_strength_3;

	let light_dir_4 = normalize(light_4.position - in.world_position);
	let diffuse_strength_4 = max(dot(in.world_normal, light_dir_4), 0.0);
	let diffuse_color_4 = light_4.color * diffuse_strength_4;

	let light_dir_5 = normalize(light_5.position - in.world_position);
	let diffuse_strength_5 = max(dot(in.world_normal, light_dir_5), 0.0);
	let diffuse_color_5 = light_5.color * diffuse_strength_5;

	let light_dir_6 = normalize(light_6.position - in.world_position);
	let diffuse_strength_6 = max(dot(in.world_normal, light_dir_6), 0.0);
	let diffuse_color_6 = light_6.color * diffuse_strength_6;


	let diffuse_color = in.diffuse * (diffuse_color_0 + diffuse_color_1 + diffuse_color_2 + diffuse_color_3 + diffuse_color_4 + diffuse_color_5 + diffuse_color_6);
	
	let view_dir = normalize(camera.view_pos.xyz - in.world_position);
	let half_dir_0 = normalize(view_dir + light_dir_0);
	let half_dir_1 = normalize(view_dir + light_dir_1);
	let half_dir_2 = normalize(view_dir + light_dir_2);
	let half_dir_3 = normalize(view_dir + light_dir_3);
	let half_dir_4 = normalize(view_dir + light_dir_4);
	let half_dir_5 = normalize(view_dir + light_dir_5);
	let half_dir_6 = normalize(view_dir + light_dir_6);
	
	let specular_strength_0 = pow(max(dot(in.world_normal, half_dir_0), 0.0), in.shin);
	let specular_strength_1 = pow(max(dot(in.world_normal, half_dir_1), 0.0), in.shin);
	let specular_strength_2 = pow(max(dot(in.world_normal, half_dir_2), 0.0), in.shin);
	let specular_strength_3 = pow(max(dot(in.world_normal, half_dir_3), 0.0), in.shin);
	let specular_strength_4 = pow(max(dot(in.world_normal, half_dir_4), 0.0), in.shin);
	let specular_strength_5 = pow(max(dot(in.world_normal, half_dir_5), 0.0), in.shin);
	let specular_strength_6 = pow(max(dot(in.world_normal, half_dir_6), 0.0), in.shin);

	let specular_color_0 = specular_strength_0 * light_0.color;
	let specular_color_1 = specular_strength_1 * light_1.color;
	let specular_color_2 = specular_strength_2 * light_2.color;
	let specular_color_3 = specular_strength_3 * light_3.color;
	let specular_color_4 = specular_strength_4 * light_4.color;
	let specular_color_5 = specular_strength_4 * light_5.color;
	let specular_color_6 = specular_strength_6 * light_6.color;

	let specular_color = in.spec * (specular_color_0 + specular_color_1 + specular_color_2 + specular_color_3 + specular_color_4 + specular_color_5 + specular_color_6);

	let result = ambient_color + diffuse_color + specular_color;

	return vec4<f32>(result, 1.0);
}
