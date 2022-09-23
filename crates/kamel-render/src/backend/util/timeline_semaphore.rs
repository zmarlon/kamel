use std::{ffi::CString, ops::Deref, slice};

use ash::{prelude::VkResult, vk, vk::Handle};

use crate::backend::Device;

#[derive(Copy, Clone, Debug)]
pub struct TimelineSemaphoreDesc<'a> {
    pub initial_value: u64,
    pub label: Option<&'a str>
}

pub struct TimelineSemaphore {
    semaphore: vk::Semaphore,
    device: Device
}

impl TimelineSemaphore {
    pub fn new(device: Device, desc: &TimelineSemaphoreDesc) -> VkResult<Self> {
        let mut semaphore_type_create_info = vk::SemaphoreTypeCreateInfo::default()
            .semaphore_type(vk::SemaphoreType::TIMELINE)
            .initial_value(desc.initial_value);

        let semaphore_create_info = vk::SemaphoreCreateInfo::default().push_next(&mut semaphore_type_create_info);

        let semaphore = unsafe { device.loader().create_semaphore(&semaphore_create_info, None) }?;

        if let Some(label) = desc.label {
            if device.instance().extensions().ext_debug_utils() {
                let object_name = CString::new(label).unwrap();

                let debug_utils_object_name_info = vk::DebugUtilsObjectNameInfoEXT::default()
                    .object_type(vk::ObjectType::SEMAPHORE)
                    .object_handle(semaphore.as_raw())
                    .object_name(&object_name);

                unsafe {
                    device
                        .instance()
                        .debug_utils_loader()
                        .debug_utils_set_object_name(device.loader().handle(), &debug_utils_object_name_info)
                }?;
            }
        }

        Ok(Self { semaphore, device })
    }

    #[inline]
    pub fn value(&self) -> VkResult<u64> {
        unsafe { self.device.loader().get_semaphore_counter_value(self.semaphore) }
    }

    #[inline]
    pub fn set_value(&self, value: u64) -> VkResult<()> {
        let semaphore_signal_info = vk::SemaphoreSignalInfo::default().semaphore(self.semaphore).value(value);

        unsafe { self.device.loader().signal_semaphore(&semaphore_signal_info) }
    }

    #[inline]
    pub fn wait_for_value(&self, value: u64, timeout: u64) -> VkResult<()> {
        let semaphore_wait_info = vk::SemaphoreWaitInfo::default().semaphores(slice::from_ref(&self.semaphore)).values(slice::from_ref(&value));

        unsafe { self.device.loader().wait_semaphores(&semaphore_wait_info, timeout) }
    }
}

impl Deref for TimelineSemaphore {
    type Target = vk::Semaphore;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.semaphore
    }
}

impl Drop for TimelineSemaphore {
    fn drop(&mut self) {
        todo!()
    }
}
