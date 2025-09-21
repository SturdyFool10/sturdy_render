use crate::renderer::vertex::Vertex;
use tracing::info;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages, Device};

pub struct Mesh {
    pub verts: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(verts: Vec<Vertex>, indices: Vec<u32>) -> Self {
        info!(
            "Mesh created: {} vertices, {} indices",
            verts.len(),
            indices.len()
        );
        Self { verts, indices }
    }

    /// Returns a sample quad mesh (square) with different colors at each corner.
    pub fn sample_quad() -> Self {
        let verts = vec![
            Vertex {
                position: [-0.5, 0.5],
                color: [1.0, 0.0, 0.0], // Top-left: Red
            },
            Vertex {
                position: [0.5, 0.5],
                color: [0.0, 1.0, 0.0], // Top-right: Green
            },
            Vertex {
                position: [0.5, -0.5],
                color: [0.0, 0.0, 1.0], // Bottom-right: Blue
            },
            Vertex {
                position: [-0.5, -0.5],
                color: [1.0, 1.0, 0.0], // Bottom-left: Yellow
            },
        ];
        let indices = vec![
            0, 2, 1, // First triangle (Top-left, Bottom-right, Top-right) - CCW
            0, 3, 2, // Second triangle (Top-left, Bottom-left, Bottom-right) - CCW
        ];
        Mesh::new(verts, indices)
    }

    /// Creates a vertex buffer from this mesh.
    pub fn create_vertex_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.verts),
            usage: BufferUsages::VERTEX,
        })
    }

    /// Creates an index buffer from this mesh.
    pub fn create_index_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: BufferUsages::INDEX,
        })
    }
}
