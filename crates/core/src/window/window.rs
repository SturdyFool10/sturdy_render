use std::sync::Arc;
use tracing::info;
use winit::{
    dpi::{PhysicalSize, Size},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

/// EngineWindow: Owns the window handle and related state, provides creation and event handling.
pub struct EngineWindow {
    pub window: Option<Arc<Window>>,
    initialized: bool,
    intended_size: glm::UVec2,
    // Add other generic state as needed, but no graphics API handles
}

impl EngineWindow {
    pub fn default_attributes(title: &str, width: u32, height: u32) -> WindowAttributes {
        let mut attributes = WindowAttributes::default();
        attributes.title = title.into();
        attributes.inner_size = Some(Size::Physical(PhysicalSize::new(width, height)));
        attributes
    }

    /// Create the window and return EngineWindow.
    pub fn create(el: &ActiveEventLoop, title: &str, width: u32, height: u32) -> Self {
        info!(
            "Creating window: title='{}', size={}x{}",
            title, width, height
        );
        let attrs = Self::default_attributes(title, width, height).with_visible(true);
        let window = el.create_window(attrs).expect("create window");
        let window = Arc::new(window);

        Self {
            window: Some(window),
            initialized: false,
            intended_size: glm::uvec2(width, height),
        }
    }

    // No graphics API configuration helpers here.

    /// Handle window events and mutate window state as needed.
    pub fn handle_event(&mut self, el: &ActiveEventLoop, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                info!("Window close requested.");
                el.exit();
            }
            WindowEvent::Resized(new_size) => {
                // Only log and propagate resize after window is initialized to intended size
                if !self.initialized {
                    if new_size.width == self.intended_size.x
                        && new_size.height == self.intended_size.y
                    {
                        self.initialized = true;
                    } else {
                        // Suppress and stop propagation for weird initial resizes
                        return;
                    }
                }
                if self.initialized {
                    tracing::trace!("Window resized: {}x{}", new_size.width, new_size.height);
                }
                // Window resizing logic only, no graphics API calls
            }
            WindowEvent::RedrawRequested => {
                // No rendering here; engine will handle after window event
                window.request_redraw();
            }
            _ => {}
        }
    }

    /// Example mutator: set window title
    pub fn set_title(&self, title: &str) {
        if let Some(window) = self.window.as_ref() {
            window.set_title(title);
        }
    }

    pub fn get_ready(&self) -> bool {
        self.initialized
    }

    /// Example accessor: get window size
    pub fn get_size(&self) -> Option<PhysicalSize<u32>> {
        self.window.as_ref().map(|w| w.inner_size())
    }

    // Add more mutators/accessors as needed for full control.
}
