//! Renderer subsystem: stateless, immediate-mode graphics API layer.
//! Consumes graphics API resources internally; manages GPU resources and draw submission.

pub use crate::renderer::wgpu::wgpu_renderer::WgpuRenderer;

pub trait Renderer {
    /// Initialize the renderer (internal setup only)
    fn init(&mut self);

    /// Render the current frame (renderer manages its own device/queue/view)
    fn render(&mut self);

    /// Create the graphics API surface for the given window handle
    fn create_surface(&mut self, window_handle: *const std::ffi::c_void);

    /// Resize the surface (called when window is resized)
    fn resize_surface(&mut self, new_width: u32, new_height: u32);

    /// Detach the surface and clean up resources, but keep window alive
    fn detach_surface(&mut self);
}
