@group(0) @binding(2)
var<uniform> dimensions: vec2<u32>;

@group(1) @binding(0)
var t_image: texture_2d<f32>;
@group(1) @binding(1)
var s_image: sampler;

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
	@location(0) text: vec2<f32>,
	@location(1) color: vec4<f32>,
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

	out.position =  model_matrix * vec4<f32>(model.position, 1.0);
	out.position.z = 0.99;

	// To keep the aspect ratio
	let dims = vec2<f32>(dimensions);
	if (dimensions.x < dimensions.y) {
		out.position.x = out.position.x * dims.y / dims.x;
	} else {
		out.position.y = out.position.y * dims.x / dims.y;
	}

	if (model.position.x == -1.0) {
		out.text.x = 0.0;
	} else if (model.position.x == 1.0) {
		out.text.x = 1.0;
	}

	if (model.position.y == -1.0) {
		out.text.y = 0.0;
	} else if (model.position.y == 1.0) {
		out.text.y = 1.0;
	}

	out.color = instance.color;

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	var out = textureSample(t_image, s_image, in.text);
	out.xyz = in.color.xyz;
	if (in.color.a < 0.5) {
		out.a = 1.0 - out.a;
	}
	return out;
}
