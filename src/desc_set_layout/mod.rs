pub mod binding;

use crate::device::Device;
use crate::sampler::Sampler;
use ash::version::DeviceV1_0;
use ash::vk;
use binding::BindingInfo;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

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

    pub fn build(self, device: Device) -> CreateDescriptorSetLayoutResult<DescriptorSetLayout> {
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

        let mut samplers = Vec::new();
        for binding in &self.bindings {
            samplers.extend(binding.samplers().clone());
        }

        unsafe { DescriptorSetLayout::new(&create_info, device, samplers) }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct DescriptorSetLayout {
    descriptor_set_layout: Arc<UniqueDescriptorSetLayout>,
}

impl DescriptorSetLayout {
    /// # Safety
    /// todo
    pub unsafe fn new(
        create_info: &vk::DescriptorSetLayoutCreateInfo,
        device: Device,
        samplers: Vec<Sampler>,
    ) -> CreateDescriptorSetLayoutResult<Self> {
        UniqueDescriptorSetLayout::new(create_info, device, samplers).map(|udsl| Self {
            descriptor_set_layout: Arc::new(udsl),
        })
    }

    /// # Safety
    /// todo
    pub unsafe fn handle(&self) -> &vk::DescriptorSetLayout {
        &self.descriptor_set_layout.handle()
    }

    pub fn device(&self) -> &Device {
        &self.descriptor_set_layout.device()
    }

    pub fn samplers(&self) -> &Vec<Sampler> {
        &self.descriptor_set_layout.samplers()
    }
}

struct UniqueDescriptorSetLayout {
    handle: vk::DescriptorSetLayout,
    device: Device,
    samplers: Vec<Sampler>,
}

impl UniqueDescriptorSetLayout {
    pub unsafe fn new(
        create_info: &vk::DescriptorSetLayoutCreateInfo,
        device: Device,
        samplers: Vec<Sampler>,
    ) -> CreateDescriptorSetLayoutResult<Self> {
        let handle = device
            .handle()
            .create_descriptor_set_layout(create_info, None)?;

        Ok(Self {
            handle,
            device,
            samplers,
        })
    }

    pub unsafe fn handle(&self) -> &vk::DescriptorSetLayout {
        &self.handle
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn samplers(&self) -> &Vec<Sampler> {
        &self.samplers
    }
}

impl Eq for UniqueDescriptorSetLayout {}

impl PartialEq for UniqueDescriptorSetLayout {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.handle() == other.handle() }
    }
}

pub type CreateDescriptorSetLayoutResult<T> = Result<T, CreateDescriptorSetLayoutError>;

#[derive(Debug)]
pub enum CreateDescriptorSetLayoutError {
    VkError(vk::Result),
}

impl Error for CreateDescriptorSetLayoutError {}

impl fmt::Display for CreateDescriptorSetLayoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VkError(e) => write!(f, "Can't create descriptor set layout: {}", e),
        }
    }
}

impl From<vk::Result> for CreateDescriptorSetLayoutError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
