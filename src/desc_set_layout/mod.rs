pub mod binding;

use crate::device::Device;
use crate::generic::{Dependence, DeviceHandle, UniqueDeviceHandle};
use ash::prelude::VkResult;
use ash::vk;
use binding::BindingInfo;

pub type DescriptorSetLayout = DeviceHandle<vk::DescriptorSetLayout>;

pub struct DescriptorSetLayoutBuilder {
    bindings: Vec<BindingInfo>,
    flags: vk::DescriptorSetLayoutCreateFlags,
}

impl DescriptorSetLayoutBuilder {
    pub fn new(bindings: Vec<BindingInfo>) -> Self {
        Self {
            bindings,
            flags: Default::default(),
        }
    }

    pub fn build(self, device: Device) -> VkResult<DescriptorSetLayout> {
        let binding_ptrs: Vec<vk::DescriptorSetLayoutBinding> = self
            .bindings
            .iter()
            .map(|b| unsafe { b.raw_binding() })
            .collect();

        let create_info = vk::DescriptorSetLayoutCreateInfo {
            binding_count: binding_ptrs.len() as u32,
            p_bindings: binding_ptrs.as_ptr(),
            flags: self.flags,
            ..Default::default()
        };

        let mut samplers: Vec<Box<dyn Dependence>> = Vec::new();
        for binding in &self.bindings {
            for sampler in binding.samplers() {
                samplers.push(Box::new(sampler.clone()))
            }
        }

        unsafe {
            let unique = UniqueDeviceHandle::new(&create_info.into(), device, samplers, ())?;
            Ok(DescriptorSetLayout::new(unique))
        }
    }
}
