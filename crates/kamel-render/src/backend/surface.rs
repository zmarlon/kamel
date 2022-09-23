use std::sync::Arc;

use anyhow::Result;
use ash::vk;
use kamel_bevy::ecs::{self as bevy_ecs, system::Resource};
use raw_window_handle::HasRawWindowHandle;

use crate::backend::Instance;

struct Inner {
    surface: vk::SurfaceKHR,
    instance: Instance
}

#[derive(Clone, Resource)]
pub struct Surface(Arc<Inner>);

impl Surface {
    pub fn new(instance: Instance, window: &impl HasRawWindowHandle) -> Result<Self> {
        unsafe {
            let surface = ash_window::create_surface(instance.entry_loader(), instance.loader(), window, None)?;
            Ok(Self(Arc::new(Inner { surface, instance })))
        }
    }

    #[inline]
    pub fn surface(&self) -> &vk::SurfaceKHR {
        &self.0.surface
    }
}

impl Drop for Inner {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.instance.surface_loader().destroy_surface(self.surface, None);
        }
    }
}
