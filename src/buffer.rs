use crate::device::Device;
use crate::generic::{DeviceHandle, UniqueDeviceHandle};
use ash::prelude::VkResult;
use ash::vk;

pub type Buffer = DeviceHandle<vk::Buffer>;

#[derive(Default)]
pub struct BufferBuilder {
    size: u64,
    usage: vk::BufferUsageFlags,
    sharing_mode: vk::SharingMode,
    flags: vk::BufferCreateFlags,
}

impl BufferBuilder {
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn with_usage(mut self, usage: vk::BufferUsageFlags) -> Self {
        self.usage = usage;
        self
    }

    pub fn with_sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.sharing_mode = sharing_mode;
        self
    }

    pub fn with_flags(mut self, flags: vk::BufferCreateFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn build(self, device: Device, queues_family_indices: &[u32]) -> VkResult<Buffer> {
        let create_info = vk::BufferCreateInfo {
            flags: self.flags,
            size: self.size,
            usage: self.usage,
            sharing_mode: self.sharing_mode,
            queue_family_index_count: queues_family_indices.len() as u32,
            p_queue_family_indices: queues_family_indices.as_ptr(),
            ..Default::default()
        };

        unsafe {
            let unique = UniqueDeviceHandle::new(&create_info.into(), device, Vec::default(), ())?;
            Ok(Buffer::new(unique))
        }
    }
}
