use std::ops::Deref;

use anyhow::Result;
use ash::vk;

use crate::backend::{util::debug_utils, Device};

#[derive(Copy, Clone, Debug)]
pub struct BinarySemaphoreDesc<'a> {
    pub label: Option<&'a str>
}

pub struct BinarySemaphore {
    semaphore: vk::Semaphore,
    device: Device
}

impl BinarySemaphore {
    pub fn new(device: Device, desc: &BinarySemaphoreDesc) -> Result<Self> {
        let semaphore = unsafe { device.loader().create_semaphore(&vk::SemaphoreCreateInfo::default(), None) }?;

        if let Some(label) = desc.label {
            unsafe { debug_utils::set_object_name(&device, semaphore, label) }?;
        }

        Ok(Self { semaphore, device })
    }
}

impl Deref for BinarySemaphore {
    type Target = vk::Semaphore;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.semaphore
    }
}

impl Drop for BinarySemaphore {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.device.loader().destroy_semaphore(self.semaphore, None);
        }
    }
}
