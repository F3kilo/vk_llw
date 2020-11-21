use crate::device::Device;
use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

pub struct SamplerBuilder {
    create_info: vk::SamplerCreateInfo,
}

impl SamplerBuilder {
    pub fn with_min_mag_filters(mut self, min: vk::Filter, mag: vk::Filter) -> Self {
        self.create_info.min_filter = min;
        self.create_info.mag_filter = mag;
        self
    }

    pub fn with_mip_map_mode(mut self, mode: vk::SamplerMipmapMode) -> Self {
        self.create_info.mipmap_mode = mode;
        self
    }

    pub fn with_address_modes(
        mut self,
        u: vk::SamplerAddressMode,
        v: vk::SamplerAddressMode,
        w: vk::SamplerAddressMode,
    ) -> Self {
        self.create_info.address_mode_u = u;
        self.create_info.address_mode_v = v;
        self.create_info.address_mode_w = w;
        self
    }

    pub fn with_mip_lod_bias(mut self, bias: f32) -> Self {
        self.create_info.mip_lod_bias = bias;
        self
    }

    pub fn with_anisotropy(mut self, anisotropy_enable: vk::Bool32) -> Self {
        self.create_info.anisotropy_enable = anisotropy_enable;
        self
    }

    pub fn with_max_anisotropy(mut self, max_anisotropy: f32) -> Self {
        self.create_info.max_anisotropy = max_anisotropy;
        self
    }

    pub fn with_compare_op(mut self, compare_op: Option<vk::CompareOp>) -> Self {
        match compare_op {
            None => {
                self.create_info.compare_enable = vk::FALSE;
            }
            Some(compare_op) => {
                self.create_info.compare_enable = vk::TRUE;
                self.create_info.compare_op = compare_op;
            }
        }
        self
    }

    pub fn with_min_max_lod(mut self, min: f32, max: f32) -> Self {
        self.create_info.min_lod = min;
        self.create_info.max_lod = max;
        self
    }

    pub fn with_border_color(mut self, border_color: vk::BorderColor) -> Self {
        self.create_info.border_color = border_color;
        self
    }

    pub fn with_unnormolized_coordinates(mut self, unnormolized_coordinates: vk::Bool32) -> Self {
        self.create_info.unnormalized_coordinates = unnormolized_coordinates;
        self
    }

    pub fn build(self, device: Device) -> CreateSamplerResult<Sampler> {
        unsafe { Sampler::new(&self.create_info, device) }
    }
}

impl Default for SamplerBuilder {
    fn default() -> Self {
        SamplerBuilder {
            create_info: Default::default(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Sampler {
    sampler: Arc<UniqueSampler>,
}

impl Sampler {
    /// # Safety
    /// todo
    pub unsafe fn new(
        create_info: &vk::SamplerCreateInfo,
        device: Device,
    ) -> CreateSamplerResult<Self> {
        UniqueSampler::new(create_info, device).map(|s| Self {
            sampler: Arc::new(s),
        })
    }

    /// # Safety
    /// todo
    pub unsafe fn handle(&self) -> &vk::Sampler {
        &self.sampler.handle()
    }

    pub fn device(&self) -> &Device {
        &self.sampler.device()
    }
}

struct UniqueSampler {
    handle: vk::Sampler,
    device: Device,
}

impl UniqueSampler {
    unsafe fn new(
        create_info: &vk::SamplerCreateInfo,
        device: Device,
    ) -> CreateSamplerResult<Self> {
        log::trace!("Creating vulkan sampler");
        let handle = device.handle().create_sampler(create_info, None)?;
        Ok(Self { handle, device })
    }

    pub unsafe fn handle(&self) -> &vk::Sampler {
        &self.handle
    }

    pub fn device(&self) -> &Device {
        &self.device
    }
}

impl Drop for UniqueSampler {
    fn drop(&mut self) {
        log::trace!("Destroying vulkan sampler");
        unsafe { self.device.handle().destroy_sampler(self.handle, None) }
    }
}

impl Eq for UniqueSampler {}

impl PartialEq for UniqueSampler {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.handle() == other.handle() }
    }
}

pub type CreateSamplerResult<T> = Result<T, CreateSamplerError>;

#[derive(Debug)]
pub enum CreateSamplerError {
    VkError(vk::Result),
}

impl Error for CreateSamplerError {}

impl fmt::Display for CreateSamplerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VkError(e) => write!(f, "Can't create vk sampler: {}", e),
        }
    }
}

impl From<vk::Result> for CreateSamplerError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
