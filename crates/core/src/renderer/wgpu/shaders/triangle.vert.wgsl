// Vertex shader for instanced mesh rendering with per-instance transform and color.

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct InstanceInput {
    @location(2) transform_0: vec4<f32>,
    @location(3) transform_1: vec4<f32>,
    @location(4) transform_2: vec4<f32>,
    @location(5) transform_3: vec4<f32>,
    @location(6) instance_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var output: VertexOutput;
    let pos = vec4<f32>(vertex.position, 0.0, 1.0);
    let transform = mat4x4<f32>(
        instance.transform_0,
        instance.transform_1,
        instance.transform_2,
        instance.transform_3
    );
    output.position = transform * pos;
    output.color = instance.instance_color;
    return output;
}
