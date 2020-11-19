use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use crate::device::Device;

pub struct MemoryBuilder {
    size: u64,
    type_index: u32,
}

impl MemoryBuilder {
    pub fn new(size: u64, type_index: u32) -> Self {
        Self { size, type_index }
    }

    pub fn build(self, device: Device) -> MemAllocResult<Memory> {
        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: self.size,
            memory_type_index: self.type_index,
            ..Default::default()
        };

        Memory::new(device, &alloc_info)
    }
}

#[derive(Clone)]
pub struct Memory {
    unique_memory: Arc<UniqueMemory>,
}

impl Memory {
    pub fn new(device: Device, allocate_info: &vk::MemoryAllocateInfo) -> MemAllocResult<Self> {
        UniqueMemory::new(device, allocate_info).map(|um| Self {
            unique_memory: Arc::new(um),
        })
    }

    /// # Safety
    /// TODO
    pub unsafe fn handle(&self) -> &vk::DeviceMemory {
        &self.unique_memory.handle()
    }

    pub fn device(&self) -> &Device {
        &self.unique_memory.device()
    }
}

struct UniqueMemory {
    device: Device,
    handle: vk::DeviceMemory,
}

impl UniqueMemory {
    pub fn new(device: Device, allocate_info: &vk::MemoryAllocateInfo) -> MemAllocResult<Self> {
        log::trace!(
            "Allocating vk device memory; size: {}; type_index: {}",
            allocate_info.allocation_size,
            allocate_info.memory_type_index
        );
        let handle = unsafe { device.handle().allocate_memory(allocate_info, None)? };
        Ok(Self { handle, device })
    }

    pub unsafe fn handle(&self) -> &vk::DeviceMemory {
        &self.handle
    }

    pub fn device(&self) -> &Device {
        &self.device
    }
}

impl Drop for UniqueMemory {
    fn drop(&mut self) {
        log::trace!("Freeing vk device memory");
        unsafe { self.device.handle().free_memory(self.handle, None) }
    }
}

pub type MemAllocResult<T> = Result<T, MemAllocError>;

#[derive(Debug)]
pub enum MemAllocError {
    VkError(vk::Result),
}

impl Error for MemAllocError {}

impl fmt::Display for MemAllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemAllocError::VkError(e) => write!(f, "Vulkan memory allocation failed: {}", e),
        }
    }
}

impl From<vk::Result> for MemAllocError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
