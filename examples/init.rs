use ash::prelude::VkResult;
use ash::{vk, LoadingError};
use log::LevelFilter;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::ops::BitXor;
use vk_llw::buffer::BufferBuilder;
use vk_llw::command_buffer::CommandBuffersBuilder;
use vk_llw::command_pool::CommandPoolBuilder;
use vk_llw::debug_report::{DebugReport, DebugReportBuilder};
use vk_llw::device::{pdevice_selectors, CreateDeviceError, DeviceBuilder};
use vk_llw::instance::{Instance, InstanceBuilder};
use vk_llw::memory::MemoryBuilder;
use vk_llw::queue::{GetQueueError, Queue};
use vk_llw::sampler::SamplerBuilder;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::max())
        .init();
    let init_result = init_vulkan();
    init_result.expect("Vulkan can't be initialized")
}

fn init_vulkan() -> InitVkResult<()> {
    let entry = ash::Entry::new()?;
    let layers = instance_layers(entry.clone());
    let instance = InstanceBuilder::new(entry)
        .with_api_version(1, 0, 0)
        .with_layers(layers)
        .with_extensions(instance_extensions())
        .build()?;

    let _debug_report = debug_report(instance.clone())?;

    let pdevice_selector = Box::new(pdevice_selectors::any_compute);
    let device = DeviceBuilder::new(pdevice_selector).build(instance)?;
    log::info!("Selected device: {}", device);

    let fam_index = device.queues_info()[0].family_index;
    let queue = Queue::get(device.clone(), fam_index, 0)?;
    let _memory = MemoryBuilder::new(256, 0).build(device.clone())?;

    let _buffer = BufferBuilder::default()
        .with_size(128)
        .with_usage(vk::BufferUsageFlags::TRANSFER_SRC)
        .build(device.clone(), &[queue.family_index()])?;

    let _command_buffers = {
        let command_pool =
            CommandPoolBuilder::new(queue.family_index(), vk::CommandPoolCreateFlags::TRANSIENT)
                .build(device.clone())?;

        CommandBuffersBuilder::default()
            .with_count(4)
            .build(command_pool, device.clone())?;
    };

    let _sampler = SamplerBuilder::default().build(device)?;

    // let binding_info = BindingInfo::new(
    //     0,
    //     BindingDescriptorType::UniformBuffer,
    //     1,
    //     vk::ShaderStageFlags::COMPUTE,
    // );
    // let _desc_set_layout = DescriptorSetLayoutBuilder::new(vec![binding_info]).build(device)?;

    Ok(())
}

fn instance_layers(entry: ash::Entry) -> Vec<CString> {
    InstanceBuilder::debug_layers(entry)
}

fn instance_extensions() -> Vec<CString> {
    vec![ash::extensions::ext::DebugReport::name().into()]
}

pub fn debug_report(instance: Instance) -> VkResult<Option<DebugReport>> {
    DebugReportBuilder::default()
        .with_callback(DebugReportBuilder::default_logger_callback())
        .with_flags(vk::DebugReportFlagsEXT::all().bitxor(vk::DebugReportFlagsEXT::INFORMATION))
        .build(instance)
        .map(Some)
}

pub type InitVkResult<T> = Result<T, InitVkError>;

#[derive(Debug)]
pub enum InitVkError {
    LoadVulkanError(ash::LoadingError),
    CreateInstanceError(ash::InstanceError),
    CreateDeviceError(CreateDeviceError),
    GetQueueError(GetQueueError),
    VkError(vk::Result),
}

impl Error for InitVkError {}

impl fmt::Display for InitVkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadVulkanError(e) => write!(f, "Can't load vk functions: {}", e),
            Self::CreateInstanceError(e) => write!(f, "Can't init vk instance: {}", e),
            Self::CreateDeviceError(e) => write!(f, "Can't create vk device: {}", e),
            Self::GetQueueError(e) => write!(f, "Can't get vk device queue: {}", e),
            Self::VkError(e) => write!(f, "Vulkan error: {}", e),
        }
    }
}

impl From<ash::LoadingError> for InitVkError {
    fn from(e: LoadingError) -> Self {
        Self::LoadVulkanError(e)
    }
}

impl From<ash::InstanceError> for InitVkError {
    fn from(e: ash::InstanceError) -> Self {
        Self::CreateInstanceError(e)
    }
}

impl From<CreateDeviceError> for InitVkError {
    fn from(e: CreateDeviceError) -> Self {
        Self::CreateDeviceError(e)
    }
}

impl From<GetQueueError> for InitVkError {
    fn from(e: GetQueueError) -> Self {
        Self::GetQueueError(e)
    }
}

impl From<vk::Result> for InitVkError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
