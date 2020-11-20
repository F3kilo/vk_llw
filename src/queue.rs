use crate::device::Device;
use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::fmt;

#[derive(Clone)]
pub struct Queue {
    handle: vk::Queue,
    device: Device,
    family_index: u32,
    queue_index: u32,
}

impl Queue {
    pub fn get(device: Device, family_index: u32, queue_index: u32) -> Result<Self, GetQueueError> {
        log::trace!(
            "Getting queue #{} with family #{} from device",
            queue_index,
            family_index
        );

        let family_info = device
            .queues_info()
            .iter()
            .find(|inf| inf.family_index == family_index);

        if let Some(family_info) = family_info {
            if queue_index < family_info.count {
                let handle = unsafe { device.handle().get_device_queue(family_index, queue_index) };
                return Ok(Self {
                    handle,
                    device,
                    family_index,
                    queue_index,
                });
            }
            return Err(GetQueueError::BadQueueIndex);
        }
        Err(GetQueueError::BadFamilyIndex)
    }

    /// # Safety
    /// TODO
    pub unsafe fn handle(&self) -> &vk::Queue {
        &self.handle
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn family_index(&self) -> u32 {
        self.family_index
    }

    pub fn queue_index(&self) -> u32 {
        self.queue_index
    }
}

impl Eq for Queue {}

impl PartialEq for Queue {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.handle() == other.handle() }
    }
}

pub type GetQueueResult<T> = Result<T, GetQueueError>;

#[derive(Debug)]
pub enum GetQueueError {
    BadFamilyIndex,
    BadQueueIndex,
}

impl Error for GetQueueError {}

impl fmt::Display for GetQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BadFamilyIndex => write!(
                f,
                "Device must be created with queues with specefied family index"
            ),
            Self::BadQueueIndex => write!(
                f,
                "Queue index must be less or equal to count of queues in specified family"
            ),
        }
    }
}
