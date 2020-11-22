use std::fmt;
use ash::prelude::VkResult;

pub trait RawDeviceHandle: Sized + Eq {
    type CreateInfo: fmt::Display;

    fn name() -> &'static str;

    /// # Safety
    /// * `create_info` must be valid vulkan create info with valid pointers.
    /// * `device` must be valid and created vulkan device.
    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self>;

    /// # Safety
    /// * `device` must be valid and created vulkan device.
    /// * Must not be called more then once.
    unsafe fn destroy(&self, device: &ash::Device);
}
