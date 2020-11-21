use crate::device::Device;
use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

pub struct CommandPoolBuilder {
    flags: vk::CommandPoolCreateFlags,
    queue_family_index: u32,
}

impl CommandPoolBuilder {
    pub fn new(queue_family_index: u32) -> Self {
        Self {
            queue_family_index,
            flags: Default::default(),
        }
    }

    pub fn with_flags(mut self, flags: vk::CommandPoolCreateFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn build(self, device: Device) -> CreateCommandPoolResult<CommandPool> {
        let create_info = vk::CommandPoolCreateInfo {
            flags: self.flags,
            queue_family_index: self.queue_family_index,
            ..Default::default()
        };

        unsafe { CommandPool::new(device, &create_info) }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct CommandPool {
    unique_command_pool: Arc<UniqueCommandPool>,
}

impl CommandPool {
    /// # Safety
    /// todo
    pub unsafe fn new(
        device: Device,
        create_info: &vk::CommandPoolCreateInfo,
    ) -> CreateCommandPoolResult<Self> {
        UniqueCommandPool::new(device, create_info).map(|ucp| Self {
            unique_command_pool: Arc::new(ucp),
        })
    }

    /// # Safety
    /// TODO
    pub unsafe fn handle(&self) -> &vk::CommandPool {
        &self.unique_command_pool.handle()
    }

    pub fn device(&self) -> &Device {
        &self.unique_command_pool.device()
    }

    pub fn queue_family_index(&self) -> u32 {
        self.unique_command_pool.queue_family_index()
    }

    pub fn flags(&self) -> vk::CommandPoolCreateFlags {
        self.unique_command_pool.flags()
    }
}

#[derive(Eq, PartialEq)]
struct UniqueCommandPool {
    handle: vk::CommandPool,
    device: Device,
    queue_family_index: u32,
    flags: vk::CommandPoolCreateFlags,
}

impl UniqueCommandPool {
    pub unsafe fn new(
        device: Device,
        create_info: &vk::CommandPoolCreateInfo,
    ) -> CreateCommandPoolResult<Self> {
        log::trace!(
            "Creating command pool for queue family: {} and flags: {:?}",
            create_info.queue_family_index,
            create_info.flags
        );
        let handle = device.handle().create_command_pool(create_info, None)?;
        Ok(Self {
            handle,
            device,
            queue_family_index: create_info.queue_family_index,
            flags: create_info.flags,
        })
    }

    pub unsafe fn handle(&self) -> &vk::CommandPool {
        &self.handle
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    pub fn flags(&self) -> vk::CommandPoolCreateFlags {
        self.flags
    }
}

impl Drop for UniqueCommandPool {
    fn drop(&mut self) {
        log::trace!(
            "Creating command pool for queue family: {} and flags: {:?}",
            self.queue_family_index,
            self.flags
        );
        unsafe { self.device.handle().destroy_command_pool(self.handle, None) }
    }
}

pub type CreateCommandPoolResult<T> = Result<T, CreateCommandPoolError>;

#[derive(Debug)]
pub enum CreateCommandPoolError {
    VkError(vk::Result),
}

impl Error for CreateCommandPoolError {}

impl fmt::Display for CreateCommandPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VkError(e) => write!(f, "Can't create command pool: {}", e),
        }
    }
}

impl From<vk::Result> for CreateCommandPoolError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
