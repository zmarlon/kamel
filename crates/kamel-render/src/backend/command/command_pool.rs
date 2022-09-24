use std::ops::Deref;

use anyhow::Result;
use ash::vk;

use crate::backend::{util::debug_utils, Device};

#[derive(Copy, Clone, Debug)]
pub struct CommandPoolDesc<'a> {
    pub flags: vk::CommandPoolCreateFlags,
    pub family_index: u32,
    pub label: Option<&'a str>
}

pub struct CommandPool {
    command_pool: vk::CommandPool,
    device: Device
}

impl CommandPool {
    pub fn new(device: Device, desc: &CommandPoolDesc) -> Result<Self> {
        let command_pool_create_info = vk::CommandPoolCreateInfo::default().flags(desc.flags).queue_family_index(desc.family_index);

        let command_pool = unsafe { device.loader().create_command_pool(&command_pool_create_info, None)? };

        if let Some(label) = desc.label {
            unsafe { debug_utils::set_object_name(&device, command_pool, label) }?;
        }

        Ok(Self { command_pool, device })
    }
}

impl Deref for CommandPool {
    type Target = vk::CommandPool;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.command_pool
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe {
            self.device.loader().destroy_command_pool(self.command_pool, None);
        }
    }
}
