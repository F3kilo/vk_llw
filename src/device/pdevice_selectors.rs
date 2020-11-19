use crate::device::QueuesInfo;
use crate::instance::Instance;
use ash::version::InstanceV1_0;
use ash::vk;
use ash::vk::{PhysicalDevice, QueueFlags};
use std::error::Error;
use std::fmt;

pub type PhysicalDeviceResult = Result<PhysicalDeviceInfo, PhysicalDeviceError>;
pub trait PhysicalDeviceSelector: FnOnce(&Instance) -> PhysicalDeviceResult {}
impl<T: FnOnce(&Instance) -> PhysicalDeviceResult> PhysicalDeviceSelector for T {}

pub fn any_graphics(instance: &Instance) -> PhysicalDeviceResult {
    log::trace!("Selecting device with single graphics queue");
    first_with_flags(instance, vk::QueueFlags::GRAPHICS)
}

pub fn any_compute(instance: &Instance) -> PhysicalDeviceResult {
    log::trace!("Selecting device with single compute queue");
    first_with_flags(instance, vk::QueueFlags::COMPUTE)
}

pub fn first_with_flags(instance: &Instance, required_flags: QueueFlags) -> PhysicalDeviceResult {
    let (pdevice, family_index) = first_device_with_family_flags(&instance, required_flags)?;

    Ok(PhysicalDeviceInfo {
        pdevice,
        physical_device_features: Default::default(),
        queues_info: vec![QueuesInfo {
            family_index,
            count: 1,
        }],
    })
}

fn first_device_with_family_flags(
    instance: &Instance,
    flags: vk::QueueFlags,
) -> Result<(PhysicalDevice, u32), PhysicalDeviceError> {
    unsafe {
        let pdevices = instance.handle().enumerate_physical_devices()?;
        for pd in pdevices {
            let queue_props = instance
                .handle()
                .get_physical_device_queue_family_properties(pd);

            let suit_family = queue_props
                .iter()
                .enumerate()
                .find(|(_, props)| (props.queue_flags & flags == flags) && props.queue_count > 0);

            if let Some((index, _)) = suit_family {
                return Ok((pd, index as u32));
            }
        }
    }
    Err(PhysicalDeviceError::NotFound(format!(
        "Physical device with queue flags {:?} not found",
        flags
    )))
}

pub struct PhysicalDeviceInfo {
    pub pdevice: PhysicalDevice,
    pub queues_info: Vec<QueuesInfo>,
    pub physical_device_features: vk::PhysicalDeviceFeatures,
}

#[derive(Debug)]
pub enum PhysicalDeviceError {
    NotFound(String),
    VkError(vk::Result),
}

impl Error for PhysicalDeviceError {}

impl fmt::Display for PhysicalDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "No suit device: {}", msg),
            Self::VkError(e) => write!(f, "Vulkan error: {}", e),
        }
    }
}

impl From<vk::Result> for PhysicalDeviceError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}
