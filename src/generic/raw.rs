use ash::prelude::VkResult;
use ash::version::DeviceV1_0;
use ash::vk;
use std::fmt;

pub trait RawDeviceHandle: Sized + Eq {
    type CreateInfo: fmt::Display;
    type DestroyInfo: Sized;

    fn name() -> &'static str;

    /// # Safety
    /// * `create_info` must be valid vulkan create info with valid pointers.
    /// * `device` must be valid and created vulkan device.
    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self>;

    /// # Safety
    /// * `device` must be valid and created vulkan device.
    /// * Must not be called more then once.
    unsafe fn destroy(&self, device: &ash::Device, destroy_info: &Self::DestroyInfo);
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
    type DestroyInfo = ();

    fn name() -> &'static str {
        "vulkan device memory"
    }

    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self> {
        device.allocate_memory(&create_info.0, None)
    }

    unsafe fn destroy(&self, device: &ash::Device, _destroy_info: &Self::DestroyInfo) {
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
    type DestroyInfo = ();

    fn name() -> &'static str {
        "vulkan buffer"
    }

    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self> {
        device.create_buffer(&create_info.0, None)
    }

    unsafe fn destroy(&self, device: &ash::Device, _destroy_info: &Self::DestroyInfo) {
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

// ----------------------------------------------------------
// ------------------------ Command pool --------------------
// ----------------------------------------------------------

impl RawDeviceHandle for vk::CommandPool {
    type CreateInfo = CreateInfoWrapper<vk::CommandPoolCreateInfo>;
    type DestroyInfo = ();

    fn name() -> &'static str {
        "vulkan command pool"
    }

    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self> {
        device.create_command_pool(&create_info.0, None)
    }

    unsafe fn destroy(&self, device: &ash::Device, _destroy_info: &Self::DestroyInfo) {
        device.destroy_command_pool(*self, None)
    }
}

impl fmt::Display for CreateInfoWrapper<vk::CommandPoolCreateInfo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Flags: {:?}; Queue family index: {};",
            self.0.flags, self.0.queue_family_index,
        )
    }
}

// ----------------------------------------------------------
// ------------------------ Command buffers -----------------
// ----------------------------------------------------------

impl RawDeviceHandle for Vec<vk::CommandBuffer> {
    type CreateInfo = CreateInfoWrapper<vk::CommandBufferAllocateInfo>;
    type DestroyInfo = vk::CommandPool;

    fn name() -> &'static str {
        "vulkan command buffers"
    }

    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self> {
        device.allocate_command_buffers(&create_info.0)
    }

    unsafe fn destroy(&self, device: &ash::Device, destroy_info: &Self::DestroyInfo) {
        device.free_command_buffers(*destroy_info, self.as_slice())
    }
}

impl fmt::Display for CreateInfoWrapper<vk::CommandBufferAllocateInfo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Count: {}; Level: {:?};",
            self.0.command_buffer_count, self.0.level,
        )
    }
}

// ----------------------------------------------------------
// ------------------------ Sampler -------------------------
// ----------------------------------------------------------

impl RawDeviceHandle for vk::Sampler {
    type CreateInfo = CreateInfoWrapper<vk::SamplerCreateInfo>;
    type DestroyInfo = ();

    fn name() -> &'static str {
        "vulkan sampler"
    }

    unsafe fn create(create_info: &Self::CreateInfo, device: &ash::Device) -> VkResult<Self> {
        device.create_sampler(&create_info.0, None)
    }

    unsafe fn destroy(&self, device: &ash::Device, _destroy_info: &Self::DestroyInfo) {
        device.destroy_sampler(*self, None)
    }
}

impl fmt::Display for CreateInfoWrapper<vk::SamplerCreateInfo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Flags: {:?}; Mag filter: {:?}; Min filter: {:?}; Mip map mode: {:?}; Address mode (u,v,w): ({:?},{:?},{:?}); Mip lod bias: {}; Anisotropy enable: {}; Max anisotropy: {}; Compare enable: {}; Compare op: {:?}; Min lod: {}; Max lod: {}; Border color: {:?}; Unnormolized coordinates: {};",
            self.0.flags,
            self.0.mag_filter,
            self.0.min_filter,
            self.0.mipmap_mode,
            self.0.address_mode_u,
            self.0.address_mode_v,
            self.0.address_mode_w,
            self.0.mip_lod_bias,
            self.0.anisotropy_enable,
            self.0.max_anisotropy,
            self.0.compare_enable,
            self.0.compare_op,
            self.0.min_lod,
            self.0.max_lod,
            self.0.border_color,
            self.0.unnormalized_coordinates,
        )
    }
}
