use wgpu::{
    Backends, Instance, InstanceDescriptor, InstanceFlags, MemoryBudgetThresholds, Surface,
};
use winit::window::Window;

/// Represents the GPU/WebGPU concern for the engine.
pub struct GpuSurface {
    pub instance: Instance,
    pub surface: Surface<'static>,
}

impl GpuSurface {
    /// Creates a new wgpu instance and surface from a winit window.
    ///
    /// # Safety
    /// This function will panic if the surface cannot be created.
    pub fn new(window: &'static Box<Window>) -> Self {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            flags: InstanceFlags::default(),
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: Default::default(),
        });
        let surface = instance
            .create_surface(&**window)
            .expect("Failed to create wgpu surface");
        Self { instance, surface }
    }
}
