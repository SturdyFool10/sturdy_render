//! Pipeline setup for WgpuRenderer: loads shaders, creates pipeline layout and render pipeline.

use std::fs;
use std::path::Path;
use wgpu::{Device, RenderPipeline, ShaderModule, SurfaceConfiguration};

/// Loads a WGSL shader from a file path.
pub fn load_shader(device: &Device, path: &str) -> ShaderModule {
    let source = fs::read_to_string(Path::new(path))
        .expect(&format!("Failed to read shader file: {}", path));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(path),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}

/// Creates the render pipeline for a colored triangle.
pub fn create_triangle_pipeline(
    device: &Device,
    config: &SurfaceConfiguration,
    vertex_layout: wgpu::VertexBufferLayout,
    shader_vert_path: &str,
    shader_frag_path: &str,
) -> RenderPipeline {
    let vs_module = load_shader(device, shader_vert_path);
    let fs_module = load_shader(device, shader_frag_path);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Triangle Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Triangle Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vs_module,
            entry_point: Some("main"),
            buffers: &[vertex_layout],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_module,
            entry_point: Some("main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
