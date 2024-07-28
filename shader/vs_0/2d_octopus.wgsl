fn gyr(p: vec3<f32>) -> f32 {
	return dot(sin(p.xyz), cos(p.zxy));
}
fn map(p: vec3<f32>, time: f32) -> f32 {
	return (1. + 0.2 * sin(p.y * 600.)) * dot(sin((p * 10. + 0.8 * gyr(p * 8.)).xyz), cos((p * 10. + 0.8 * gyr(p * 8.)).zxy)) * (1. + sin(time + length(p.xy) * 10.)) + 0.3 * sin(time * 0.15 + p.z * 5. + p.y) * (2. + dot(sin((p * (sin(time * 0.2 + p.z * 3.) * 350. + 250.)).xyz), cos((p * (sin(time * 0.2 + p.z * 3.) * 350. + 250.)).zxy)));
}

fn norm(p: vec3<f32>, time: f32) -> vec3<f32> {
	let m: f32 = map(p, time);
	var d: vec2<f32> = vec2<f32>(0.06 + 0.06 * sin(p.z), 0.);
	return map(p, time) - vec3<f32>(map(p - d.xyy, time), map(p - d.yxy, time), map(p - d.yyx, time));
}

fn octopus(v: vec3<f32>) -> vec4<f32> {
	let norm_s: f32 = v.x * v.x + v.y * v.y;
	var out :vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
	if (norm_s < 1.0) {
		out = vec4<f32>(1.0, 1.0, 1.0, 1.0);
	}
	return out;
}

@group(0) @binding(1)
var<uniform> time: f32;

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
	out.position = vec4<f32>(model.position.xy, 0.999 , 1.0);
	out.color = instance.color;
	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let v = octopus(vec3<f32>(in.position.xy * 0.005, time));
	return v;
}
