use ash::{vk, LoadingError};
use log::LevelFilter;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::ops::BitXor;
use vk_llw::debug_report::{
    CreateDebugReportError, DebugReport, DebugReportBuilder, DebugReportResult,
};
use vk_llw::device::{pdevice_selectors, CreateDeviceError, DeviceBuilder};
use vk_llw::instance::{Instance, InstanceBuilder};
use vk_llw::memory::{MemAllocError, MemoryBuilder};

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

    let _some_memory = MemoryBuilder::new(256, 0).build(device)?;
    Ok(())
}

fn instance_layers(entry: ash::Entry) -> Vec<CString> {
    InstanceBuilder::debug_layers(entry)
}

fn instance_extensions() -> Vec<CString> {
    vec![ash::extensions::ext::DebugReport::name().into()]
}

pub fn debug_report(instance: Instance) -> DebugReportResult<Option<DebugReport>> {
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
    CreateDebugReportError(CreateDebugReportError),
    MemAllocError(MemAllocError),
}

impl Error for InitVkError {}

impl fmt::Display for InitVkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadVulkanError(e) => write!(f, "Can't load vk functions: {}", e),
            Self::CreateInstanceError(e) => write!(f, "Can't init vk instance: {}", e),
            Self::CreateDeviceError(e) => write!(f, "Can't create vk device: {}", e),
            Self::CreateDebugReportError(e) => write!(f, "Can't create vk debug report: {}", e),
            Self::MemAllocError(e) => write!(f, "Can't allocate memory: {}", e),
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

impl From<CreateDebugReportError> for InitVkError {
    fn from(e: CreateDebugReportError) -> Self {
        Self::CreateDebugReportError(e)
    }
}

impl From<MemAllocError> for InitVkError {
    fn from(e: MemAllocError) -> Self {
        Self::MemAllocError(e)
    }
}
