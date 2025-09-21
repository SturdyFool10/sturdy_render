use bytemuck::{Pod, Zeroable};
use wgpu::{Buffer, BufferUsages, Device};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2, // position
        1 => Float32x3  // color
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Returns the vertex data for a colored triangle.
pub fn triangle_vertices() -> [Vertex; 3] {
    [
        Vertex {
            position: [0.0, 0.8],
            color: [1.0, 0.0, 0.0],
        }, // Top (Red)
        Vertex {
            position: [-0.8, -0.8],
            color: [0.0, 1.0, 0.0],
        }, // Left (Green)
        Vertex {
            position: [0.8, -0.8],
            color: [0.0, 0.0, 1.0],
        }, // Right (Blue)
    ]
}

/// Creates a vertex buffer for the triangle.
pub fn create_vertex_buffer(device: &Device) -> Buffer {
    let vertices = triangle_vertices();
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: BufferUsages::VERTEX,
    })
}

// Ensure wgpu::util is available
#[allow(unused_imports)]
use wgpu::util::DeviceExt;
