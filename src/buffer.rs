use crate::device::Device;
use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

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

    pub fn build(
        self,
        device: Device,
        queues_family_indices: &[u32],
    ) -> CreateBufferResult<Buffer> {
        let create_info = vk::BufferCreateInfo {
            flags: self.flags,
            size: self.size,
            usage: self.usage,
            sharing_mode: self.sharing_mode,
            queue_family_index_count: queues_family_indices.len() as u32,
            p_queue_family_indices: queues_family_indices.as_ptr(),
            ..Default::default()
        };

        Buffer::new(device, &create_info)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Buffer {
    unique_buffer: Arc<UniqueBuffer>,
}

impl Buffer {
    pub fn new(device: Device, create_info: &vk::BufferCreateInfo) -> CreateBufferResult<Self> {
        UniqueBuffer::new(device, create_info).map(|ub| Self {
            unique_buffer: Arc::new(ub),
        })
    }

    /// # Safety
    /// TODO
    pub unsafe fn handle(&self) -> &vk::Buffer {
        &self.unique_buffer.handle()
    }

    pub fn device(&self) -> &Device {
        &self.unique_buffer.device()
    }

    pub fn size(&self) -> u64 {
        self.unique_buffer.size()
    }

    pub fn usage(&self) -> vk::BufferUsageFlags {
        self.unique_buffer.usage()
    }
}

struct UniqueBuffer {
    handle: vk::Buffer,
    device: Device,
    size: u64,
    usage: vk::BufferUsageFlags,
}

impl UniqueBuffer {
    pub fn new(device: Device, create_info: &vk::BufferCreateInfo) -> CreateBufferResult<Self> {
        log::trace!(
            "Creating vk buffer with size: {} and usage: {:?}",
            create_info.size,
            create_info.usage
        );

        let handle = unsafe { device.handle().create_buffer(create_info, None)? };
        Ok(Self {
            handle,
            device,
            size: create_info.size,
            usage: create_info.usage,
        })
    }

    pub unsafe fn handle(&self) -> &vk::Buffer {
        &self.handle
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn usage(&self) -> vk::BufferUsageFlags {
        self.usage
    }
}

impl Drop for UniqueBuffer {
    fn drop(&mut self) {
        log::trace!(
            "Destroying buffer with size: {} and usage: {:?}",
            self.size,
            self.usage
        );

        unsafe { self.device.handle().destroy_buffer(self.handle, None) }
    }
}

impl Eq for UniqueBuffer {}

impl PartialEq for UniqueBuffer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.handle() == other.handle() }
    }
}

pub type CreateBufferResult<T> = Result<T, CreateBufferError>;

#[derive(Debug)]
pub enum CreateBufferError {
    VkError(vk::Result),
}

impl Error for CreateBufferError {}

impl fmt::Display for CreateBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VkError(e) => write!(f, "Can't create buffer: {}", e),
        }
    }
}

impl From<vk::Result> for CreateBufferError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
