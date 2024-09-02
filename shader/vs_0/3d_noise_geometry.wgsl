@group(0) @binding(1)
var<uniform> time: f32;

@group(0) @binding(2)
var<uniform> dimensions: vec2<u32>;

let maxSteps: i32 = 32;
let hitThreshold: f32 = 0.01;
let minStep: f32 = 0.01;
let PI: f32 = 3.14159;
let translucentColor: vec4<f32> = vec4<f32>(1.0, 0.6, 0.3, 0.5);

fn mod289_3(x: vec3<f32>) -> vec3<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289_4(x: vec4<f32>) -> vec4<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn permute(x: vec4<f32>) -> vec4<f32> {
    return mod289_4(((x * 34.0) + 10.0) * x);
}

fn taylorInvSqrt(r: vec4<f32>) -> vec4<f32> {
    return 1.79284291400159 - 0.85373472095314 * r;
}

fn snoise(v: vec3<f32>) -> f32 {
    let C = vec2<f32>(1.0 / 6.0, 1.0 / 3.0) ;
    let D = vec4<f32>(0.0, 0.5, 1.0, 2.0);

    // First corner
    var i = floor(v + dot(v, C.yyy));
    var x0 = v - i + dot(i, C.xxx) ;

    // Other corners
    var g = step(x0.yzx, x0.xyz);
    var l = 1.0 - g;
    var i1 = min(g.xyz, l.zxy);
    var i2 = max(g.xyz, l.zxy);

    //   x0 = x0 - 0.0 + 0.0 * C.xxx;
    //   x1 = x0 - i1  + 1.0 * C.xxx;
    //   x2 = x0 - i2  + 2.0 * C.xxx;
    //   x3 = x0 - 1.0 + 3.0 * C.xxx;
    var x1 = x0 - i1 + C.xxx;
    var x2 = x0 - i2 + C.yyy; // 2.0*C.x = 1/3 = C.y
    var x3 = x0 - D.yyy;      // -1.0+3.0*C.x = -0.5 = -D.y

    // Permutations
    i = mod289_3(i);
    var p = permute(permute(permute(
        i.z + vec4(0.0, i1.z, i2.z, 1.0)
    ) + i.y + vec4(0.0, i1.y, i2.y, 1.0)) + i.x + vec4(0.0, i1.x, i2.x, 1.0));

    // Gradients: 7x7 points over a square, mapped onto an octahedron.
    // The ring size 17*17 = 289 is close to a multiple of 49 (49*6 = 294)
    var n_ = 0.142857142857; // 1.0/7.0
    var ns = n_ * D.wyz - D.xzx;

    var j = p - 49.0 * floor(p * ns.z * ns.z);  //  mod(p,7*7)

    var x_ = floor(j * ns.z);
    var y_ = floor(j - 7.0 * x_);    // mod(j,N)

    var x = x_ * ns.x + ns.yyyy;
    var y = y_ * ns.x + ns.yyyy;
    var h = 1.0 - abs(x) - abs(y);

    var b0 = vec4(x.xy, y.xy);
    var b1 = vec4(x.zw, y.zw);

    //vec4 s0 = vec4(lessThan(b0,0.0))*2.0 - 1.0;
    //vec4 s1 = vec4(lessThan(b1,0.0))*2.0 - 1.0;
    var s0 = floor(b0) * 2.0 + 1.0;
    var s1 = floor(b1) * 2.0 + 1.0;
    var sh = -step(h, vec4(0.0));

    var a0 = b0.xzyw + s0.xzyw * sh.xxyy ;
    var a1 = b1.xzyw + s1.xzyw * sh.zzww ;

    var p0 = vec3<f32>(a0.xy, h.x);
    var p1 = vec3<f32>(a0.zw, h.y);
    var p2 = vec3<f32>(a1.xy, h.z);
    var p3 = vec3<f32>(a1.zw, h.w);

    //Normalise gradients
    var norm = taylorInvSqrt(vec4<f32>(dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)));
    p0 *= norm.x;
    p1 *= norm.y;
    p2 *= norm.z;
    p3 *= norm.w;

    // Mix final noise value
    var m = max(vec4<f32>(0.5, 0.5, 0.5, 0.5) - vec4<f32>(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)), vec4<f32>(0.0, 0.0, 0.0, 0.0));
    m = m * m;
    return 105.0 * dot(m * m, vec4(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)));
}

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

fn difference(a: f32, b: f32) -> f32 {
    return max(a, -b);
} 

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
    let np: vec3<f32> = vec3<f32>(p.xy, time);
    d += layered_noise(np, 3) * 0.1;
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
	@location(1) pos: vec2<f32>,
	@location(2) rotation: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(model.position.xy, 0.9995, 1.0);
    out.color = instance.color;
    out.pos = vec2<f32>(0.4, 0.);
    out.rotation = vec2<f32>(0.4, 0.);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos: vec2<f32> = (in.position.xy / vec2<f32>(dimensions.xy) * 2. - 1.) - in.pos;
    let asp = f32(dimensions.x) / f32(dimensions.y);
    var rd: vec3<f32> = normalize(vec3<f32>(asp * pos.x, pos.y, -1.5));
    var ro: vec3<f32> = vec3<f32>(0., 0., 4.5);
    var hit: bool;
    var dist: f32;

    rd = rotateX(rd, time * in.rotation.x);
    ro = rotateX(ro, time * in.rotation.x);
    rd = rotateY(rd, time * in.rotation.y);
    ro = rotateY(ro, time * in.rotation.y);
    let hitPos: vec3<f32> = traceInside(ro, rd, &hit, &dist);
    var rgba: vec4<f32> = vec4<f32>(0.);
    if hit {
        rgba = exp(-dist * dist * translucentColor);
    } else {
        rgba = vec4<f32>(0.0);
    }
    return rgba;
}
