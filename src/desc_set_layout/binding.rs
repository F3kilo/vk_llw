use crate::sampler::Sampler;
use ash::vk;

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
    /// todo
    pub unsafe fn immutable_samplers(&self) -> &Vec<vk::Sampler> {
        &self.raw_samplers
    }

    /// # Safety
    /// todo
    pub unsafe fn raw_binding(&self) -> vk::DescriptorSetLayoutBinding {
        self.raw_binding
    }
}
