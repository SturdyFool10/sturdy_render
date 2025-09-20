//! WgpuRenderer: wgpu-specific implementation of the Renderer trait.

use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference,
    PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages, Trace,
};
use winit::window::Window;

pub struct WgpuRenderer {
    pub instance: Option<Instance>,
    pub surface: Option<Surface<'static>>,
    pub adapter: Option<Adapter>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub surface_config: Option<SurfaceConfiguration>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub pipeline: Option<wgpu::RenderPipeline>,
}

impl WgpuRenderer {
    pub fn new() -> Self {
        Self {
            instance: None,
            surface: None,
            adapter: None,
            device: None,
            queue: None,
            surface_config: None,
            vertex_buffer: None,
            pipeline: None,
        }
    }
}

impl WgpuRenderer {
    /// Initialize the renderer (internal setup only)
    pub fn init(&mut self) {
        self.instance = Some(Instance::default());
        // Device/queue will be created in create_surface after surface is available

        // If device, queue, and surface_config are ready, set up vertex buffer and pipeline
        if let (Some(device), Some(surface_config)) =
            (self.device.as_ref(), self.surface_config.as_ref())
        {
            // Vertex buffer
            let vertex_buffer = crate::renderer::wgpu::vertex::create_vertex_buffer(device);
            self.vertex_buffer = Some(vertex_buffer);

            // Pipeline
            let pipeline = crate::renderer::wgpu::pipeline::create_triangle_pipeline(
                device,
                surface_config,
                crate::renderer::wgpu::vertex::Vertex::desc(),
                "crates/core/src/renderer/wgpu/shaders/triangle.vert.wgsl",
                "crates/core/src/renderer/wgpu/shaders/triangle.frag.wgsl",
            );
            self.pipeline = Some(pipeline);
        }
    }

    /// Render the current frame (renderer manages its own device/queue/view)
    pub fn render(&mut self) {
        if let (
            Some(surface),
            Some(device),
            Some(queue),
            Some(surface_config),
            Some(pipeline),
            Some(vertex_buffer),
        ) = (
            self.surface.as_ref(),
            self.device.as_ref(),
            self.queue.as_ref(),
            self.surface_config.as_ref(),
            self.pipeline.as_ref(),
            self.vertex_buffer.as_ref(),
        ) {
            match surface.get_current_texture() {
                Ok(frame) => {
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Triangle Render Encoder"),
                        });

                    {
                        let mut render_pass =
                            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("Triangle Render Pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                            r: 0.1,
                                            g: 0.1,
                                            b: 0.1,
                                            a: 1.0,
                                        }),
                                        store: wgpu::StoreOp::Store,
                                    },
                                    depth_slice: None,
                                })],
                                depth_stencil_attachment: None,
                                occlusion_query_set: None,
                                timestamp_writes: None,
                            });

                        render_pass.set_pipeline(pipeline);
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass.draw(0..3, 0..1);
                    }

                    queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                Err(err) => {
                    eprintln!("Surface error: {:?}", err);
                    surface.configure(device, surface_config);
                }
            }
        }
    }

    /// Create the graphics API surface for the given window handle
    pub fn create_surface(&mut self, window_handle: *const std::ffi::c_void) {
        // Safety: window_handle must be a pointer to a winit::window::Window
        let window = unsafe { &*(window_handle as *const Window) };

        // Create instance if not already created
        if self.instance.is_none() {
            self.instance = Some(Instance::default());
        }
        let instance = self.instance.as_ref().unwrap();

        // Create surface
        let surface = instance
            .create_surface(window)
            .expect("create wgpu surface");
        self.surface = Some(surface);

        // Request adapter
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: self.surface.as_ref(),
        }))
        .expect("request adapter");
        self.adapter = Some(adapter);

        // Request device/queue
        let (device, queue) = pollster::block_on(self.adapter.as_ref().unwrap().request_device(
            &DeviceDescriptor {
                label: Some("wgpu-device"),
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::default(),
            },
        ))
        .expect("request device");
        self.device = Some(device);
        self.queue = Some(queue);

        // Configure surface
        let size = window.inner_size();
        let caps = self
            .surface
            .as_ref()
            .unwrap()
            .get_capabilities(self.adapter.as_ref().unwrap());
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| matches!(f, TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb))
            .unwrap_or(caps.formats[0]);
        let present_mode = caps
            .present_modes
            .iter()
            .copied()
            .find(|m| *m == PresentMode::Mailbox)
            .unwrap_or(PresentMode::Fifo);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        self.surface
            .as_ref()
            .unwrap()
            .configure(self.device.as_ref().unwrap(), &surface_config);
        self.surface_config = Some(surface_config);
    }

    /// Resize the surface (called when window is resized)
    pub fn resize_surface(&mut self, new_width: u32, new_height: u32) {
        if let (Some(surface), Some(device), Some(surface_config)) = (
            self.surface.as_ref(),
            self.device.as_ref(),
            self.surface_config.as_mut(),
        ) {
            surface_config.width = new_width.max(1);
            surface_config.height = new_height.max(1);
            surface.configure(device, &*surface_config);
        }
    }

    /// Detach the surface and clean up resources, but keep window alive
    pub fn detach_surface(&mut self) {
        self.surface = None;
        self.surface_config = None;
        self.adapter = None;
        self.device = None;
        self.queue = None;
    }
}

impl Drop for WgpuRenderer {
    fn drop(&mut self) {
        self.detach_surface();
    }
}

// Implement the Renderer trait for WgpuRenderer
impl crate::renderer::Renderer for WgpuRenderer {
    fn init(&mut self) {
        self.init();
    }

    fn render(&mut self) {
        self.render();
    }

    fn create_surface(&mut self, window_handle: *const std::ffi::c_void) {
        self.create_surface(window_handle);
    }

    fn resize_surface(&mut self, new_width: u32, new_height: u32) {
        self.resize_surface(new_width, new_height);
    }

    fn detach_surface(&mut self) {
        self.detach_surface();
    }
}
