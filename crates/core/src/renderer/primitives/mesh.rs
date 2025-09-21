use crate::renderer::vertex::Vertex;
use glm::{Mat4, Vec4};
use tracing::info;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages, Device};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Instance {
    pub transform: Mat4, // glm 4x4 matrix
    pub color: Vec4,     // glm 4D vector (RGBA)
}

/// Creates a column-major 4x4 translation matrix as a flat [f32; 16].
pub fn translation_matrix_flat(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0, // Column 0
        0.0, 1.0, 0.0, 0.0, // Column 1
        0.0, 0.0, 1.0, 0.0, // Column 2
        x, y, z, 1.0, // Column 3 (translation)
    ]
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub transform: [f32; 16],
    pub color: [f32; 4],
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let m = &self.transform;
        InstanceRaw {
            transform: [
                m.c0.x, m.c0.y, m.c0.z, m.c0.w, m.c1.x, m.c1.y, m.c1.z, m.c1.w, m.c2.x, m.c2.y,
                m.c2.z, m.c2.w, m.c3.x, m.c3.y, m.c3.z, m.c3.w,
            ],
            color: [self.color.x, self.color.y, self.color.z, self.color.w],
        }
    }
}

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

    /// Creates an instance buffer from a slice of Instance data.
    pub fn create_instance_buffer(instances: &[Instance], device: &Device) -> Buffer {
        let raw_instances: Vec<InstanceRaw> = instances.iter().map(|inst| inst.to_raw()).collect();
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&raw_instances),
            usage: BufferUsages::VERTEX,
        })
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
