use ash::prelude::VkResult;
use ash::version::DeviceV1_0;
use ash::vk;
use std::fmt;

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

pub struct CreateInfoWrapper<T>(pub T);

impl<T> From<T> for CreateInfoWrapper<T> {
    fn from(create_info: T) -> Self {
        Self(create_info)
    }
}

// ----------------------------------------------------------
// ------------------------ DeviceMemory --------------------
// ----------------------------------------------------------

impl RawDeviceHandle for vk::DeviceMemory {
    type CreateInfo = CreateInfoWrapper<vk::MemoryAllocateInfo>;

    fn name() -> &'static str {
        "vulkan device memory"
    }

    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self> {
        device.allocate_memory(&create_info.0, None)
    }

    unsafe fn destroy(&self, device: &ash::Device) {
        device.free_memory(*self, None)
    }
}

impl fmt::Display for CreateInfoWrapper<vk::MemoryAllocateInfo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type index: {}; Size: {};",
            self.0.memory_type_index, self.0.allocation_size
        )
    }
}

// ----------------------------------------------------------
// ------------------------ Buffer --------------------------
// ----------------------------------------------------------

impl RawDeviceHandle for vk::Buffer {
    type CreateInfo = CreateInfoWrapper<vk::BufferCreateInfo>;

    fn name() -> &'static str {
        "vulkan buffer"
    }

    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self> {
        device.create_buffer(&create_info.0, None)
    }

    unsafe fn destroy(&self, device: &ash::Device) {
        device.destroy_buffer(*self, None)
    }
}

impl fmt::Display for CreateInfoWrapper<vk::BufferCreateInfo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Flags: {:?}; Usage: {:?}; Size: {}; Sharing mode: {:?};",
            self.0.flags, self.0.usage, self.0.size, self.0.sharing_mode
        )
    }
}
