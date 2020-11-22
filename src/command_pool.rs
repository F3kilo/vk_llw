use crate::device::Device;
use crate::generic::{DeviceHandle, UniqueDeviceHandle};
use ash::prelude::VkResult;
use ash::vk;

pub type CommandPool = DeviceHandle<vk::CommandPool>;

pub struct CommandPoolBuilder {
    flags: vk::CommandPoolCreateFlags,
    queue_family_index: u32,
}

impl CommandPoolBuilder {
    pub fn new(queue_family_index: u32, flags: vk::CommandPoolCreateFlags) -> Self {
        Self {
            flags,
            queue_family_index,
        }
    }

    pub fn build(self, device: Device) -> VkResult<CommandPool> {
        let create_info = vk::CommandPoolCreateInfo {
            flags: self.flags,
            queue_family_index: self.queue_family_index,
            ..Default::default()
        };

        unsafe {
            let unique = UniqueDeviceHandle::new(&create_info.into(), device, Vec::default(), ())?;
            Ok(CommandPool::new(unique))
        }
    }
}
