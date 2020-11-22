use crate::device::Device;
use crate::generic::{DeviceHandle, UniqueDeviceHandle};
use ash::prelude::VkResult;
use ash::vk;

pub type Sampler = DeviceHandle<vk::Sampler>;

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

    pub fn build(self, device: Device) -> VkResult<Sampler> {
        unsafe {
            let unique =
                UniqueDeviceHandle::new(&self.create_info.into(), device, Vec::default(), ())?;
            Ok(Sampler::new(unique))
        }
    }
}
