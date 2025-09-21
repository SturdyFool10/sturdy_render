use crate::renderer::vertex::Vertex;
use tracing::info;

pub struct Mesh {
    verts: Vec<Vertex>,
    indices: Vec<u32>,
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
}
