@group(0) @binding(2)
var<uniform> dimensions: vec2<u32>;

@group(2) @binding(2)
var t_f: texture_2d<f32>;
@group(2) @binding(3)
var s_f: sampler;
@group(2) @binding(4)
var t_p: texture_2d<f32>;
@group(2) @binding(5)
var s_p: sampler;
@group(2) @binding(6)
var t_g: texture_2d<f32>;
@group(2) @binding(7)
var s_g: sampler;
@group(2) @binding(8)
var t_a: texture_2d<f32>;
@group(2) @binding(9)
var s_a: sampler;

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
	@location(2) idx: i32,
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

	if (model.position.x == -0.5) {
		out.text.x = 0.0;
	} else if (model.position.x == 0.5) {
		out.text.x = 1.0;
	}

	if (model.position.y == -0.7) {
		out.text.y = 1.0;
	} else if (model.position.y == 0.7) {
		out.text.y = 0.0;
	}
	
	if (instance.model_matrix_3.z == 0.0) {
		out.idx = 0;
	} else if (instance.model_matrix_3.z == 1.0) {
		out.idx = 1;
	} else if (instance.model_matrix_3.z == 2.0){
		out.idx = 2;
	} else {
		out.idx = 3;
	}

	out.color = instance.color;

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	var out : vec4<f32>;

	let dx = dpdx(in.text);
	let dy = dpdx(in.text);

	if (in.idx == 0) {
		out = textureSampleGrad(t_f, s_f, in.text, dx, dy);
	} else if (in.idx == 1){
		out = textureSampleGrad(t_p, s_p, in.text, dx, dy);
	} else if (in.idx == 2) {
		out = textureSampleGrad(t_g, s_g, in.text, dx, dy);
	} else {
		out = textureSampleGrad(t_a, s_a, in.text, dx, dy);
	}
	out.x = in.color.x;
	out.y = in.color.y;
	out.z = in.color.z;
	if (in.color.a < 0.5) {
		out.a = 1.0 - out.a;
	}
	return out;
}
