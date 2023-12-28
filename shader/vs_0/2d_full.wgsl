// Description : Array and textureless GLSL 2D/3D/4D simplex 
//               noise functions.
//      Author : Ian McEwan, Ashima Arts.
//  Maintainer : stegu
//     Lastmod : 20201014 (stegu)
//     License : Copyright (C) 2011 Ashima Arts. All rights reserved.
//               Distributed under the MIT License. See LICENSE file.
//               https://github.com/ashima/webgl-noise
//               https://github.com/stegu/webgl-noise
// 

fn mod289_3(x: vec3<f32>) -> vec3<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289_4(x: vec4<f32>) -> vec4<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn permute(x: vec4<f32>) -> vec4<f32> {
    return mod289_4(((x*34.0)+10.0)*x);
}

fn taylorInvSqrt(r: vec4<f32>) -> vec4<f32> {
    return 1.79284291400159 - 0.85373472095314 * r;
}

fn snoise(v: vec3<f32>) -> f32 { 
    let C = vec2<f32>(1.0/6.0, 1.0/3.0) ;
    let D = vec4<f32>(0.0, 0.5, 1.0, 2.0);

    // First corner
    var i  = floor(v + dot(v, C.yyy) );
    var x0 =   v - i + dot(i, C.xxx) ;

    // Other corners
    var g = step(x0.yzx, x0.xyz);
    var l = 1.0 - g;
    var i1 = min( g.xyz, l.zxy );
    var i2 = max( g.xyz, l.zxy );

    //   x0 = x0 - 0.0 + 0.0 * C.xxx;
    //   x1 = x0 - i1  + 1.0 * C.xxx;
    //   x2 = x0 - i2  + 2.0 * C.xxx;
    //   x3 = x0 - 1.0 + 3.0 * C.xxx;
    var x1 = x0 - i1 + C.xxx;
    var x2 = x0 - i2 + C.yyy; // 2.0*C.x = 1/3 = C.y
    var x3 = x0 - D.yyy;      // -1.0+3.0*C.x = -0.5 = -D.y

    // Permutations
    i = mod289_3(i); 
    var p = permute( permute( permute( 
        			i.z + vec4(0.0, i1.z, i2.z, 1.0 ))
        		+ i.y + vec4(0.0, i1.y, i2.y, 1.0 )) 
        	+ i.x + vec4(0.0, i1.x, i2.x, 1.0 ));

    // Gradients: 7x7 points over a square, mapped onto an octahedron.
    // The ring size 17*17 = 289 is close to a multiple of 49 (49*6 = 294)
    var n_ = 0.142857142857; // 1.0/7.0
    var ns = n_ * D.wyz - D.xzx;

    var j = p - 49.0 * floor(p * ns.z * ns.z);  //  mod(p,7*7)

    var x_ = floor(j * ns.z);
    var y_ = floor(j - 7.0 * x_ );    // mod(j,N)

    var x = x_ *ns.x + ns.yyyy;
    var y = y_ *ns.x + ns.yyyy;
    var h = 1.0 - abs(x) - abs(y);

    var b0 = vec4( x.xy, y.xy );
    var b1 = vec4( x.zw, y.zw );

    //vec4 s0 = vec4(lessThan(b0,0.0))*2.0 - 1.0;
    //vec4 s1 = vec4(lessThan(b1,0.0))*2.0 - 1.0;
    var s0 = floor(b0)*2.0 + 1.0;
    var s1 = floor(b1)*2.0 + 1.0;
    var sh = -step(h, vec4(0.0));

    var a0 = b0.xzyw + s0.xzyw*sh.xxyy ;
    var a1 = b1.xzyw + s1.xzyw*sh.zzww ;

    var p0 = vec3<f32>(a0.xy,h.x);
    var p1 = vec3<f32>(a0.zw,h.y);
    var p2 = vec3<f32>(a1.xy,h.z);
    var p3 = vec3<f32>(a1.zw,h.w);

    //Normalise gradients
    var norm = taylorInvSqrt(vec4<f32>(dot(p0,p0), dot(p1,p1), dot(p2, p2), dot(p3,p3)));
    p0 *= norm.x;
    p1 *= norm.y;
    p2 *= norm.z;
    p3 *= norm.w;

    // Mix final noise value
    var m = max(vec4<f32>(0.5, 0.5, 0.5, 0.5) - vec4<f32>(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3)), vec4<f32>(0.0, 0.0, 0.0, 0.0));
    m = m * m;
    return 105.0 * dot(m*m, vec4(dot(p0,x0), dot(p1,x1), dot(p2,x2), dot(p3,x3)));
}

struct Audio {
gain: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> audio: Audio;

@group(0) @binding(1)
var<uniform> time: f32;

@group(0) @binding(2)
var<uniform> dimensions: vec2<u32>;

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

	out.position =  model_matrix * vec4<f32>(model.position, 1.0);

	// To keep the aspect ratio
	let dims = vec2<f32>(dimensions);
	if (dimensions.x < dimensions.y) {
		out.position.x = out.position.x * dims.y / dims.x;
	} else {
		out.position.y = out.position.y * dims.x / dims.y;
	}

	out.color = instance.color;
	return out;
}

// Returns a 3D Simplex noise value between -1 and 1
fn layered_noise(v: vec3<f32>, n_layers: i32) -> f32 {
    let step = vec3<f32>(1.3, 1.7, 2.9);
	var f = 1.0;
	var ampl = 1.0;
	var n = 0.0;
	for (var i: i32 = 0; i < n_layers; i++) {
		n += ampl * snoise(f * v - f32(i) * step);
		ampl *= 0.5;
		f *= 2.0;
	}
	return n;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return in.color;
}
