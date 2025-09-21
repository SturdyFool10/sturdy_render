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

    /// Vertex buffer layout for InstanceRaw (mat4 + vec4, step_mode: Instance)
    pub fn instance_desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use wgpu::VertexStepMode;
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<crate::renderer::primitives::mesh::InstanceRaw>()
                as wgpu::BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                // mat4 as 4 vec4s
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 48,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // color as vec4
                wgpu::VertexAttribute {
                    offset: 64,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
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
