let maxSteps: i32 = 64;
let hitThreshold: f32 = 0.01;
let minStep: f32 = 0.01;
let PI: f32 = 3.14159;
let translucentColor: vec4<f32> = vec4<f32>(1.2, 0.6, 0.3, 0.5);
fn difference(a: f32, b: f32) -> f32 {
    return max(a, -b);
} 

/*
fn noise(x: vec3<f32>) -> f32 {
    var p: vec3<f32> = floor(x);
    var f: vec3<f32> = fract(x);
    f = f * f * (3. - 2. * f);
    let uv: vec2<f32> = p.xy + vec2<f32>(37., 17.) * p.z + f.xy;
    let rg: vec2<f32> = textureSampleLevel(BUFFER_iChannel0, buffer_sampler, (uv + 0.5) / 256., f32(0.)).yx;
    return mix(rg.x, rg.y, f.z) * 2. - 1.;
} 

let m: mat3x3<f32> = mat3x3<f32>(0., 0.8, 0.6, -0.8, 0.36, -0.48, -0.6, -0.48, 0.64);
fn fbm(p: vec3<f32>) -> f32 {
    var p_var = p;
    var f: f32;
    f = 0.5 * noise(p_var);
    p_var = p_var * (m * 2.02);
    f = f + (0.25 * noise(p_var));
    p_var = p_var * (m * 2.03);
    f = f + (0.125 * noise(p_var));
    p_var = p_var * (m * 2.01);
    f = f + (0.0625 * noise(p_var));
    return f;
} 
*/

fn rotateX(p: vec3<f32>, a: f32) -> vec3<f32> {
    var sa: f32 = sin(a);
    var ca: f32 = cos(a);
    return vec3<f32>(p.x, ca * p.y - sa * p.z, sa * p.y + ca * p.z);
} 

fn rotateY(p: vec3<f32>, a: f32) -> vec3<f32> {
    let sa: f32 = sin(a);
    let ca: f32 = cos(a);
    return vec3<f32>(ca * p.x + sa * p.z, p.y, -sa * p.x + ca * p.z);
} 

fn sphere(p: vec3<f32>, r: f32) -> f32 {
    return length(p) - r;
} 

fn box(p: vec3<f32>, b: vec3<f32>) -> f32 {
    var d: vec3<f32> = abs(p) - b;
    return min(max(d.x, max(d.y, d.z)), 0.) + length(max(d, vec3<f32>(0.)));
} 

fn scene(p: vec3<f32>) -> f32 {
    var d: f32;
    d = sphere(p, 1.);
    d = difference(box(p, vec3<f32>(1.1)), d);
    d = min(d, sphere(p, 0.5));
    let np: vec3<f32> = p;
    return d;
} 

fn sceneNormal(pos: vec3<f32>) -> vec3<f32> {
    let eps: f32 = 0.05;
    var n: vec3<f32>;
    var d: f32 = scene(pos);
    n.x = scene(vec3<f32>(pos.x + eps, pos.y, pos.z)) - d;
    n.y = scene(vec3<f32>(pos.x, pos.y + eps, pos.z)) - d;
    n.z = scene(vec3<f32>(pos.x, pos.y, pos.z + eps)) - d;
    return normalize(n);
} 

fn trace(ro: vec3<f32>, rd: vec3<f32>, hit: ptr<function, bool>) -> vec3<f32> {
    (*hit) = false;
    var pos: vec3<f32> = ro;

    for (var i: i32 = 0; i < maxSteps; i = i + 1) {
        var d: f32 = scene(pos);
        if abs(d) < hitThreshold {
            (*hit) = true;
        }
        pos = pos + (d * rd);
    }

    return pos;
} 

fn traceInside(ro: vec3<f32>, rd: vec3<f32>, hit: ptr<function, bool>, insideDist: ptr<function, f32>) -> vec3<f32> {
    (*hit) = false;
    (*insideDist) = 0.;
    var pos: vec3<f32> = ro;
    var hitPos: vec3<f32> = pos;

    for (var i: i32 = 0; i < maxSteps; i = i + 1) {
        var d: f32 = scene(pos);
        d = max(abs(d), minStep) * sign(d);
        if d < hitThreshold && !(*hit) {
            hitPos = pos;
            (*hit) = true;
        }
        if d < 0. {
            (*insideDist) = (*insideDist) + (-d);
        }
        pos = pos + (abs(d) * rd);
    }

    return hitPos;
} 

fn background(rd: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(0.0);
} 


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
    out.position = vec4<f32>(model.position.xy, 0.9995, 1.0);
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x: f32 = in.position.x / f32(dimensions.x) * 2. - 1.;
    let y: f32 = in.position.y / f32(dimensions.y) * 2. - 1.;
    let asp = f32(dimensions.x) / f32(dimensions.y);
    let rd: vec3<f32> = normalize(vec3<f32>(asp * x, y, -1.5));
    let ro: vec3<f32> = vec3<f32>(0., 0., 2.5);
    var hit: bool;
    var dist: f32;
    let hitPos: vec3<f32> = traceInside(ro, rd, &hit, &dist);
    var rgba: vec4<f32> = vec4<f32>(0.);
    if hit {
        rgba = exp(-dist * dist * translucentColor);
    } else {
        rgba = vec4<f32>(0.0);
    }
    return rgba;
}
