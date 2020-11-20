use crate::command_pool::CommandPool;
use crate::device::Device;
use ash::version::DeviceV1_0;
use ash::vk;
use ash::vk::CommandBufferLevel;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

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

    pub fn build(
        self,
        pool: CommandPool,
        device: Device,
    ) -> AllocateCommandBuffersResult<CommandBuffers> {
        let alloc_info = vk::CommandBufferAllocateInfo {
            level: self.level,
            command_buffer_count: self.count,
            command_pool: unsafe { *pool.handle() },
            ..Default::default()
        };

        CommandBuffers::allocate(&alloc_info, device, pool)
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

#[derive(Clone, Eq, PartialEq)]
pub struct CommandBuffers {
    command_buffers: Arc<UniqueCommandBuffers>,
}

impl CommandBuffers {
    pub fn allocate(
        allocate_info: &vk::CommandBufferAllocateInfo,
        device: Device,
        pool: CommandPool,
    ) -> AllocateCommandBuffersResult<Self> {
        UniqueCommandBuffers::allocate(allocate_info, device, pool).map(|ucb| Self {
            command_buffers: Arc::new(ucb),
        })
    }

    pub fn pool(&self) -> &CommandPool {
        &self.command_buffers.pool()
    }

    pub fn device(&self) -> &Device {
        &self.command_buffers.device()
    }

    pub fn level(&self) -> &vk::CommandBufferLevel {
        &self.command_buffers.level()
    }

    /// # Safety
    ///
    pub unsafe fn handle(&self, index: usize) -> Option<&vk::CommandBuffer> {
        self.command_buffers.handle(index)
    }

    pub fn len(&self) -> usize {
        self.command_buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.command_buffers.is_empty()
    }
}

struct UniqueCommandBuffers {
    handles: Vec<vk::CommandBuffer>,
    pool: CommandPool,
    level: CommandBufferLevel,
    device: Device,
}

impl UniqueCommandBuffers {
    pub fn allocate(
        allocate_info: &vk::CommandBufferAllocateInfo,
        device: Device,
        pool: CommandPool,
    ) -> AllocateCommandBuffersResult<Self> {
        log::trace!(
            "Allocating {} command buffers with level: {:?}",
            allocate_info.command_buffer_count,
            allocate_info.level
        );
        let handles = unsafe { device.handle().allocate_command_buffers(allocate_info) }?;
        Ok({
            Self {
                handles,
                pool,
                device,
                level: allocate_info.level,
            }
        })
    }

    pub fn pool(&self) -> &CommandPool {
        &self.pool
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn level(&self) -> &vk::CommandBufferLevel {
        &self.level
    }

    pub unsafe fn handle(&self, index: usize) -> Option<&vk::CommandBuffer> {
        self.handles.get(index)
    }

    pub fn len(&self) -> usize {
        self.handles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.handles.len() == 0
    }
}

impl Drop for UniqueCommandBuffers {
    fn drop(&mut self) {
        log::trace!(
            "Destroying {} command buffers with level: {:?}",
            self.handles.len(),
            self.level
        );
        unsafe {
            self.device
                .handle()
                .free_command_buffers(*self.pool.handle(), &self.handles)
        }
    }
}

impl Eq for UniqueCommandBuffers {}

impl PartialEq for UniqueCommandBuffers {
    fn eq(&self, other: &Self) -> bool {
        self.handles == other.handles
    }
}

pub type AllocateCommandBuffersResult<T> = Result<T, AllocateCommandBuffersError>;

#[derive(Debug)]
pub enum AllocateCommandBuffersError {
    VkError(vk::Result),
}

impl Error for AllocateCommandBuffersError {}

impl fmt::Display for AllocateCommandBuffersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VkError(e) => write!(f, "Can't allocate command buffers: {}", e),
        }
    }
}

impl From<vk::Result> for AllocateCommandBuffersError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
