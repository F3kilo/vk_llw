use crate::device::Device;
use crate::generic::{DeviceHandle, UniqueDeviceHandle};
use ash::prelude::VkResult;
use ash::vk;

pub type Memory = DeviceHandle<vk::DeviceMemory>;

pub struct MemoryBuilder {
    size: u64,
    type_index: u32,
}

impl MemoryBuilder {
    pub fn new(size: u64, type_index: u32) -> Self {
        Self { size, type_index }
    }

    pub fn build(self, device: Device) -> VkResult<Memory> {
        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: self.size,
            memory_type_index: self.type_index,
            ..Default::default()
        };

        unsafe {
            let unique =
                UniqueDeviceHandle::new(&alloc_info.into(), device, Default::default(), ())?;
            Ok(Memory::new(unique))
        }
    }
}
