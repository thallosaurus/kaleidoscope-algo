/**
    Here we define the UV Transformations
*/
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn main(@location(0) pos: vec2<f32>) -> VertexOutput {



    
    var out: VertexOutput;
    out.position = vec4<f32>(
        pos,
        0.0, 1.0
        //pos,
    );

    out.uv = pos * 0.5 + vec2<f32>(0.5);

    //out.uv = pos;

    //return vec4<f32>(newPos, 0.0, 1.0);
    //return vec4<f32>(pos, 0.0, 1.0);
    return out;
}