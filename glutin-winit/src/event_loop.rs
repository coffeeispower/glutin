use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle};
use winit::error::OsError;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes};

use crate::private::Sealed;

/// [`ActiveEventLoop`] is the recommended way to interact with the event
/// loop, but for compatibility purposes [`EventLoop`] is also supported
/// although not recommended anymore as it has been deprecated by Winit.
pub trait GlutinEventLoop: Sealed {
    /// Create the window.
    ///
    /// See [`ActiveEventLoop::create_window`] for details.
    fn create_window(&self, window_attributes: WindowAttributes) -> Result<Box<dyn Window>, OsError>;

    /// Get a handle to the display controller of the windowing system.
    fn glutin_display_handle(&self) -> Result<DisplayHandle<'_>, HandleError>;
}

impl<T: ActiveEventLoop> Sealed for T {}

impl<T: ActiveEventLoop> GlutinEventLoop for T {
    fn create_window(&self, window_attributes: WindowAttributes) -> Result<Box<dyn Window>, OsError> {
        self.create_window(window_attributes)
    }

    fn glutin_display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.display_handle()
    }
}

impl Sealed for EventLoop {}

impl GlutinEventLoop for EventLoop {
    #[allow(deprecated)]
    fn create_window(&self, window_attributes: WindowAttributes) -> Result<Box<dyn Window>, OsError> {
        self.create_window(window_attributes)
    }

    fn glutin_display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.display_handle()
    }
}
