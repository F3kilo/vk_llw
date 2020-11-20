use crate::device::Device;
use crate::sampler::Sampler;
use ash::version::DeviceV1_0;
use ash::vk;
use ash::vk::DescriptorSetLayoutCreateFlags;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

pub struct DescriptorSetLayoutBuilder {
    bindings: Vec<BindingInfo>,
    flags: DescriptorSetLayoutCreateFlags,
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
        for sampler in &self.bindings {
            for binding_sampler in sampler.samplers() {
                samplers.push(binding_sampler.clone())
            }
        }

        DescriptorSetLayout::new(&create_info, device, samplers)
    }
}

pub enum BindingDescriptorType {
    Sampler(Vec<Sampler>),
    CombinedImageSampler(Vec<Sampler>),
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
    AccelerationStructureKhr,
}

impl BindingDescriptorType {
    pub fn to_vk_descriptor_type(&self) -> vk::DescriptorType {
        match &self {
            BindingDescriptorType::Sampler(_) => vk::DescriptorType::SAMPLER,
            BindingDescriptorType::CombinedImageSampler(_) => {
                vk::DescriptorType::COMBINED_IMAGE_SAMPLER
            }
            BindingDescriptorType::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
            BindingDescriptorType::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
            BindingDescriptorType::UniformTexelBuffer => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
            BindingDescriptorType::StorageTexelBuffer => vk::DescriptorType::STORAGE_TEXEL_BUFFER,
            BindingDescriptorType::UniformBuffer => vk::DescriptorType::UNIFORM_BUFFER,
            BindingDescriptorType::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
            BindingDescriptorType::UniformBufferDynamic => {
                vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
            }
            BindingDescriptorType::StorageBufferDynamic => {
                vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
            }
            BindingDescriptorType::InputAttachment => vk::DescriptorType::INPUT_ATTACHMENT,
            BindingDescriptorType::AccelerationStructureKhr => {
                vk::DescriptorType::ACCELERATION_STRUCTURE_KHR
            }
        }
    }

    pub fn has_samplers(&self) -> bool {
        match &self {
            BindingDescriptorType::Sampler(_) => true,
            BindingDescriptorType::CombinedImageSampler(_) => true,
            _ => false,
        }
    }
}

pub struct BindingInfo {
    samplers: Vec<Sampler>,
    raw_samplers: Vec<vk::Sampler>,
    raw_binding: vk::DescriptorSetLayoutBinding,
}

impl BindingInfo {
    pub fn new(
        index: u32,
        descriptor_type: BindingDescriptorType,
        descriptors_count: u32,
        stage_flags: vk::ShaderStageFlags,
    ) -> Self {
        let vk_descriptor_type = descriptor_type.to_vk_descriptor_type();
        let has_samplers = descriptor_type.has_samplers();
        let samplers = Self::get_samplers_vec(descriptor_type);
        let raw_samplers: Vec<vk::Sampler> =
            samplers.iter().map(|s| unsafe { *s.handle() }).collect();

        if has_samplers && descriptors_count > raw_samplers.len() as u32 {
            panic!("Descriptors count must be less or equal to samplers count")
        }

        let raw_binding = vk::DescriptorSetLayoutBinding {
            descriptor_type: vk_descriptor_type,
            descriptor_count: descriptors_count,
            binding: index,
            p_immutable_samplers: raw_samplers.as_ptr(),
            stage_flags,
        };

        Self {
            samplers,
            raw_samplers,
            raw_binding,
        }
    }

    fn get_samplers_vec(desc_types: BindingDescriptorType) -> Vec<Sampler> {
        match desc_types {
            BindingDescriptorType::Sampler(samplers) => samplers,
            BindingDescriptorType::CombinedImageSampler(samplers) => samplers,
            _ => Default::default(),
        }
    }

    pub fn samplers(&self) -> &Vec<Sampler> {
        &self.samplers
    }

    /// # Safety
    ///
    pub unsafe fn immutable_samplers(&self) -> &Vec<vk::Sampler> {
        &self.raw_samplers
    }

    /// # Safety
    ///
    pub unsafe fn raw_binding(&self) -> vk::DescriptorSetLayoutBinding {
        self.raw_binding
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct DescriptorSetLayout {
    descriptor_set_layout: Arc<UniqueDescriptorSetLayout>,
}

impl DescriptorSetLayout {
    pub fn new(
        create_info: &vk::DescriptorSetLayoutCreateInfo,
        device: Device,
        samplers: Vec<Sampler>,
    ) -> CreateDescriptorSetLayoutResult<Self> {
        UniqueDescriptorSetLayout::new(create_info, device, samplers).map(|udsl| Self {
            descriptor_set_layout: Arc::new(udsl),
        })
    }

    /// # Safety
    ///
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
    pub fn new(
        create_info: &vk::DescriptorSetLayoutCreateInfo,
        device: Device,
        samplers: Vec<Sampler>,
    ) -> CreateDescriptorSetLayoutResult<Self> {
        let handle = unsafe {
            device
                .handle()
                .create_descriptor_set_layout(create_info, None)?
        };
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
            Self::VkError(e) => write!(f, "Can't create buffer: {}", e),
        }
    }
}

impl From<vk::Result> for CreateDescriptorSetLayoutError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
