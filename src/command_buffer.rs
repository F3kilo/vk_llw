use crate::command_pool::CommandPool;
use crate::device::Device;
use crate::generic::{DeviceHandle, UniqueDeviceHandle};
use ash::prelude::VkResult;
use ash::vk;

pub type CommandBuffers = DeviceHandle<Vec<vk::CommandBuffer>>;

pub struct CommandBuffersBuilder {
    level: vk::CommandBufferLevel,
    count: u32,
}

impl CommandBuffersBuilder {
    pub fn with_level(mut self, level: vk::CommandBufferLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    pub fn build(self, pool: CommandPool, device: Device) -> VkResult<CommandBuffers> {
        let raw_pool = unsafe { *pool.handle() };

        let alloc_info = vk::CommandBufferAllocateInfo {
            level: self.level,
            command_buffer_count: self.count,
            command_pool: raw_pool,
            ..Default::default()
        };

        unsafe {
            let unique = UniqueDeviceHandle::new(
                &alloc_info.into(),
                device,
                vec![Box::new(pool)],
                raw_pool,
            )?;
            Ok(CommandBuffers::new(unique))
        }
    }
}

impl Default for CommandBuffersBuilder {
    fn default() -> Self {
        CommandBuffersBuilder {
            level: vk::CommandBufferLevel::PRIMARY,
            count: 1,
        }
    }
}
