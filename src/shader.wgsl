struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}

// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// --------------------TYPES---------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
const MAX_STEPS = 300;
const COLLITION_DIST = 0.001;
const MAX_VIEW_DEPTH: f32 = 50.0;

alias Color = vec3<f32>;
alias Point = vec3<f32>;
alias Direction = vec3<f32>;
alias Dimensions = vec3<f32>;
const PHI: f32 = 1.6180339887498948482046;

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
}

struct Material {
    color: Color,
    roughness: f32,
    transparency: f32,
}

struct Sphere {
    center: Point,
    radius: f32,
    material: Material,
}

struct Box {
    center: Point,
    size: Dimensions,
    material: Material,
}

// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// -------------------UTILITIES------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
fn sdSphere(p: Dimensions, r: f32) -> f32 {
    return length(p) - r;
}

fn sdBox(p: Point, b: vec3<f32>) -> f32 {
    let d = abs(p) - b;
    return min(max(d.x, max(d.y, d.z)), 0.0) + length(max(d, vec3<f32>(0.0)));
}

fn opUnion(d1: f32, d2: f32) -> f32 {
    return min(d1, d2);
}

fn opSubs(d1: f32, d2: f32) -> f32 {
    return max(-d1, d2);
}

fn opIntersec(d1: f32, d2: f32) -> f32 {
    return max(d1, d2);
}

fn opSmoothUnion(d1: f32, d2: f32, k: f32) -> f32 {
    let res = 0.5 + (0.5 * (d2 - d1) / k);
    let h = clamp(res, 0.0, 1.0);
    return mix(d2, d1, h) - k * h * (1.0 - h);
}

fn opSmoothSubs(d1: f32, d2: f32, k: f32) -> f32 {
    let res = 0.5 - (0.5 * (d2 + d1) / k);
    let h = clamp(res, 0.0, 1.0);
    return mix(d2, -d1, h) - k * h * (1.0 - h);
}

fn opSmoothIntersec(d1: f32, d2: f32, k: f32) -> f32 {
    let res = 0.5 - (0.5 * (d2 - d1) / k);
    let h = clamp(res, 0.0, 1.0);
    return mix(d2, -d1, h) - k * h * (1.0 - h);
}

fn rotate(point: Point, axis: Direction, angle: f32) -> Point {
    return mix(dot(axis, point) * axis, point, cos(angle)) + cross(axis, point) * sin(angle);
}

fn randomVec3FromTime() -> vec3<f32> {
    let t = uniforms.time;
    return vec3<f32>(fract(sin(t % 12.9898) * 43758.5453), fract(sin(t % 78.233) * 43758.5453), fract(sin(t % 45.164) * 43758.5453));
}


// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// -------------------ENGINE---------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
fn ray_march(ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> f32 {
    var origin_distance = 0.0;
    for (var i: i32 = 0; i < MAX_STEPS; i++) {
        let position = ray_origin + (ray_direction * origin_distance);
        let free_distance = free_distance(position);
        origin_distance += free_distance;

        if free_distance < COLLITION_DIST || origin_distance > MAX_VIEW_DEPTH { break; }
    }
    return origin_distance;
}

fn getNormal(point: vec3<f32>) -> vec3<f32> {
    let distance = free_distance(point);
    let epsilon = 0.01;

    let normal = distance - vec3(
        free_distance(point - vec3(epsilon, 0.0, 0.0)),
        free_distance(point - vec3(0.0, epsilon, 0.0)),
        free_distance(point - vec3(0.0, 0.0, epsilon)),
    );

    return normalize(normal);
}

fn diffuse_light_at(point: vec3<f32>) -> f32 {
    var lightPos = vec3<f32>(0.0, 5.0, 6.0);
    lightPos.x += 10.0 * sin(uniforms.time);
    lightPos.z += 10.0 * cos(uniforms.time);

    let light_direction = normalize(lightPos - point);
    let surface_normal = getNormal(point);

    var diffuse_light = clamp(dot(surface_normal, light_direction), 0f, 1f);

    let surface_point = point + surface_normal * 0.005;
    let distance_to_light = length(lightPos - surface_point);
    let marching_distance = ray_march(surface_point, light_direction);

    if marching_distance < distance_to_light { diffuse_light *= 0.1; }
    return diffuse_light;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = (in.clip_position.xy - (0.5 * uniforms.resolution)) / uniforms.resolution.y;
    let camera_position = vec3<f32>(0.0, 0.0, 5.0);
    let ray_direction = normalize(vec3<f32>(uv.x, -uv.y, -1.0));

    var color = vec3(0.0);
    var distance: f32 = ray_march(camera_position, ray_direction);
    let point = camera_position + (ray_direction * distance);
    let diff = diffuse_light_at(point);

    color = vec3(diff);
    //color = getNormal(point);

    let fragColor = vec4<f32>(color, 1.0);
    return fragColor;
}


// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
// ---------------------SCENE--------------------------------------------
// ----------------------------------------------------------------------
// ----------------------------------------------------------------------
fn free_distance(point: vec3<f32>) -> f32 {
    let position1 = vec3<f32>(sin(f32(uniforms.time)) * 2.0, 0.0, -0.5);
    let position2 = vec3<f32>(cos(f32(uniforms.time) * 1.3) * 2.0, sin(f32(uniforms.time) * 0.8) * 1.5, 0.0);
    let position3 = vec3<f32>(cos(f32(uniforms.time) * 1.3) * 2.0, sin(f32(uniforms.time) * 2.0) * 1.5, sin(f32(uniforms.time) * 0.8));

    let material1 = Material(randomVec3FromTime(), 1.0, 0.0);
    let material2 = Material(randomVec3FromTime(), 1.0, 0.0);
    let material3 = Material(randomVec3FromTime(), 1.0, 0.0);

    let sphere1 = Sphere(position1, 1.0, material1);
    let sphere2 = Sphere(position2, 0.8, material2);
    let sphere3 = Sphere(position3, 0.5, material3);

    let floor = Box(vec3<f32>(0.0, -2.0, 0.0), vec3<f32>(5.0, 0.1, 5.0), Material());
    let box = Box(vec3<f32>(3.0, -1.0, -3.0), vec3<f32>(0.5, 1.0, 1.0), Material());

    let d1 = sdSphere(point - sphere1.center, sphere1.radius);
    let d2 = sdSphere(point - sphere2.center, sphere2.radius);
    let d5 = sdSphere(point - sphere3.center, sphere3.radius);

    let d3 = sdBox(point - floor.center, floor.size);
    let d4 = sdBox(point - box.center, box.size);

    return min(min(min(min(d1, d2), d3), d4), d5);
}
