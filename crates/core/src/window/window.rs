use std::sync::Arc;
use winit::{
    dpi::{PhysicalSize, Size},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

/// EngineWindow: Owns the window handle and related state, provides creation and event handling.
pub struct EngineWindow {
    pub window: Option<Arc<Window>>,
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
        let attrs = Self::default_attributes(title, width, height).with_visible(true);
        let window = el.create_window(attrs).expect("create window");
        let window = Arc::new(window);

        Self {
            window: Some(window),
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
                el.exit();
            }
            WindowEvent::Resized(_new_size) => {
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

    /// Example accessor: get window size
    pub fn get_size(&self) -> Option<PhysicalSize<u32>> {
        self.window.as_ref().map(|w| w.inner_size())
    }

    // Add more mutators/accessors as needed for full control.
}
