use crate::device::Device;
use crate::generic::{DeviceHandle, UniqueDeviceHandle};
use ash::prelude::VkResult;
use ash::vk;

pub type ShaderModule = DeviceHandle<vk::ShaderModule>;

pub struct ShaderModuleBuilder {
    code: Vec<u32>,
}

impl ShaderModuleBuilder {
    pub fn new(code: Vec<u32>) -> Self {
        Self { code }
    }

    pub fn build(self, device: Device) -> VkResult<ShaderModule> {
        let create_info = vk::ShaderModuleCreateInfo {
            code_size: self.code.len() * 4,
            p_code: self.code.as_ptr(),
            ..Default::default()
        };

        unsafe {
            let unique = UniqueDeviceHandle::new(&create_info.into(), device, Vec::default(), ())?;
            Ok(ShaderModule::new(unique))
        }
    }
}
