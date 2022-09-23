use std::{ops::Deref, slice};

use anyhow::Result;
use ash::{prelude::VkResult, vk};

use crate::backend::{util::debug_utils, Device};

#[derive(Copy, Clone, Debug)]
pub struct FenceDesc<'a> {
    pub signaled: bool,
    pub label: Option<&'a str>
}

pub struct Fence {
    fence: vk::Fence,
    device: Device
}

impl Fence {
    pub fn new(device: Device, desc: &FenceDesc) -> Result<Self> {
        let fence = unsafe {
            device.loader().create_fence(
                &vk::FenceCreateInfo::default().flags(if desc.signaled { vk::FenceCreateFlags::SIGNALED } else { Default::default() }),
                None
            )
        }?;

        if let Some(label) = desc.label {
            unsafe { debug_utils::set_object_name(&device, fence, label) }?;
        }

        Ok(Self { fence, device })
    }

    #[inline]
    pub unsafe fn reset(&self) -> VkResult<()> {
        self.device.loader().reset_fences(slice::from_ref(&self.fence))
    }

    #[inline]
    pub unsafe fn wait_for(&self, timeout: u64) -> VkResult<()> {
        self.device.loader().wait_for_fences(slice::from_ref(&self.fence), true, timeout)
    }
}

impl Deref for Fence {
    type Target = vk::Fence;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.fence
    }
}

impl Drop for Fence {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.device.loader().destroy_fence(self.fence, None);
        }
    }
}
