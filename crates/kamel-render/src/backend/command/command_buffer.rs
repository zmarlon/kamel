use core::slice;
use std::{ops::Deref, sync::Arc};

use anyhow::{Ok, Result};
use ash::vk;

use crate::backend::{command::CommandPool, util::debug_utils, Device};

#[derive(Copy, Clone, Debug)]
pub struct CommandBufferDesc<'a> {
    pub label: Option<&'a str>
}

pub struct CommandBuffer {
    command_buffer: vk::CommandBuffer,
    command_pool: Arc<CommandPool>,
    device: Device
}

impl CommandBuffer {
    pub fn new(device: Device, command_pool: Arc<CommandPool>, desc: &CommandBufferDesc) -> Result<Self> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default().command_pool(**command_pool).command_buffer_count(1);

        let command_buffer = unsafe { device.loader().allocate_command_buffers(&command_buffer_allocate_info)? }[0];

        if let Some(label) = desc.label {
            unsafe { debug_utils::set_object_name(&device, command_buffer, label) }?;
        }

        Ok(Self {
            command_buffer,
            command_pool,
            device
        })
    }
}

impl Deref for CommandBuffer {
    type Target = vk::CommandBuffer;

    fn deref(&self) -> &Self::Target {
        &self.command_buffer
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe { self.device.loader().free_command_buffers(**self.command_pool, slice::from_ref(&self.command_buffer)) }
    }
}
