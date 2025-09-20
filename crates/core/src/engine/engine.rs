use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
};

use crate::renderer::Renderer;
use crate::renderer::WgpuRenderer;
use crate::window::window::EngineWindow;

/// Our top-level app. Implements `ApplicationHandler` for winit 0.30+.
pub struct Engine {
    window: EngineWindow,
    renderer: Box<dyn Renderer>,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            window: EngineWindow { window: None },
            renderer: Box::new(WgpuRenderer::new()),
        }
    }
}

impl Engine {
    /// Entry point: build an event loop and run.
    pub fn run() {
        let event_loop = EventLoop::new().expect("create EventLoop");
        event_loop.set_control_flow(ControlFlow::Poll);
        let mut app = Self::default();
        let _ = event_loop.run_app(&mut app);
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        // Use EngineWindow to create and own all window/wgpu handles
        self.window = EngineWindow::create(el, "SturdyRendererRS", 1280, 720);

        // Pass the raw window handle to the renderer for surface creation
        if let Some(window) = self.window.window.as_ref() {
            // This is a placeholder for a platform-agnostic raw window handle.
            // In real code, use raw-window-handle crate or similar.
            let raw_handle = window as *const _ as *const std::ffi::c_void;
            self.renderer.create_surface(raw_handle);
        }

        // Initialize the renderer (internal setup only)
        self.renderer.init();
    }

    fn window_event(
        &mut self,
        el: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Match event type before passing to handle_event to avoid borrow checker issues
        let _is_redraw = matches!(event, WindowEvent::RedrawRequested);

        // Clone the event for handle_event to avoid moving it
        let event_for_window = event.clone();

        // Match event type before passing to handle_event to avoid moved value error
        let mut resized_size: Option<(u32, u32)> = None;
        if let WindowEvent::Resized(new_size) = &event {
            resized_size = Some((new_size.width, new_size.height));
        }
        let is_redraw = matches!(event, WindowEvent::RedrawRequested);

        // Let the window handle the event and update its state
        self.window.handle_event(el, event_for_window);

        // After window event is processed, handle rendering if needed
        if is_redraw {
            self.renderer.render();
        }

        // Handle window resize event
        if let Some((width, height)) = resized_size {
            self.renderer.resize_surface(width, height);
        }
    }
}
