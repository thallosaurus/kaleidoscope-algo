/**
    Here we define the textures (gabor, voronoi etc)
*/



// MARK: - hä

fn kaleido_transformation(pos: vec2<f32>, repetitions: f32, scaling: f32, rotation: f32, pingpong: f32) -> vec2<f32> {
    //subtract
    let subt = (pos - vec2<f32>(0.5, 0.5)) * 2.0;

    let l = length(subt);
    let multip = scaling * l;

    let y = pingpong_port(multip, pingpong);

    //gradient texture
    let gradient = radial(subt);
    let trunc = truncate_port(repetitions);

    let m1 = (gradient * trunc);
    let m2 = pingpong_port(m1.r + 0.5, 0.5) - 0.25;

    let x = (rotation / 360.0) + m2;

    //return clamp(vec2(x, y), vec2<f32>(-1.0), vec2<f32>(1.0));
    return vec2(x, y);
}

fn wave_texture(uv: vec2<f32>, scale: f32, amp: f32) -> f32 {
    // einfache Sinus-Welle entlang X
    return 0.5 + 0.5 * sin(uv.x * scale * 6.2831853) * amp;
}

fn modf32(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
}

fn pingpong_port(value: f32, scale: f32) -> f32 {
    return scale - abs(modf32(value, 2.0*scale) - scale);
}

fn truncate_port(x: f32) -> f32 {
    return select(ceil(x), floor(x), x >= 0.0);
}

fn radial(uv: vec2<f32>) -> vec4<f32> {
    let val = gradient_radial(uv);
    return vec4<f32>(val, val, val, val);
}

fn gradient_radial(uv: vec2<f32>) -> f32 {
    // UV von [0..1] nach [-1..1] verschieben
    //let centered = uv * 2.0 - vec2<f32>(-1.0, 1.0);

    // atan(y, x) → Winkel im Bereich [-PI, PI]
    let angle = atan2(uv.y, uv.x);

    // Normalisieren auf [0..1]
    return (angle / (2.0 * 3.1415926535));
}

@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    //return vec4<f32>(1.0, 1.0, 0.0, 1.0);
    //let gradient = vec3<f32>(uv.x, uv.y, 1.0 - uv.x);
    //return vec4<f32>(gradient, 1.0);
    let newPos = kaleido_transformation(uv, 10.0, 3.8, 0.0, 1.0);
    //return vec4(newPos, newPos);

    let wave = wave_texture(newPos, 10.0, 1.0);

    return vec4(wave, wave, wave, wave);
}