//! This crate uses an extension trait to add a method `winit::window::Window` to allow for easy fullscreen toggling.
//!
//! ```rust
//! window.toggle_fullscreen()
//! ```
//!
//! Check the example for a minimal winit app that can toggle to fullscreen on pressing ENTER.
//!
//! # Future work
//! I plan to add an enum to allow a hint for exclusive or borderless.

#![warn(missing_docs)]

use winit::window::{Fullscreen, Window};

/// Extension trait for Window to add the `toggle_fullscreen` method.
pub trait WindowFullScreen {
    /// This method toggles the fullscreen state of a window.
    ///
    /// For platforms that support it well, it will use exclusive mode.  For other platforms, it will use borderless.  
    ///
    /// # Panics
    /// This function will not panic.  If any error occurs internally, it will be a no-op.
    ///
    fn toggle_fullscreen(&self);
}

impl WindowFullScreen for Window {
    fn toggle_fullscreen(&self) {
        if self.fullscreen().is_some() {
            self.set_fullscreen(None);
        } else {
            self.current_monitor().map(|monitor| {
                monitor.video_modes().next().map(|video_mode| {
                    if cfg!(any(target_os = "macos", unix)) {
                        self.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
                    } else {
                        self.set_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
                    }
                })
            });
        }
    }
}
