pub mod pdevice_selectors;
use crate::device::pdevice_selectors::PhysicalDeviceSelector;
use crate::instance::Instance;
use crate::{get_c_str_pointers, raw_name_to_c_string};
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::vk;
use pdevice_selectors::{PhysicalDeviceError, PhysicalDeviceInfo};
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

pub struct DeviceBuilder {
    pdevice_selector: Box<dyn PhysicalDeviceSelector>,
    layers: Vec<CString>,
    extensions: Vec<CString>,
}

impl DeviceBuilder {
    pub fn new(pdevice_selector: Box<dyn PhysicalDeviceSelector>) -> Self {
        Self {
            pdevice_selector,
            layers: vec![],
            extensions: vec![],
        }
    }

    pub fn with_layers(mut self, layers: Vec<CString>) -> Self {
        self.layers = layers;
        self
    }

    pub fn with_extensions(mut self, extensions: Vec<CString>) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn build(self, instance: Instance) -> Result<Device, CreateDeviceError> {
        let mut create_info = vk::DeviceCreateInfo::default();

        let layers = get_c_str_pointers(&self.layers);
        create_info.pp_enabled_layer_names = layers.as_ptr();
        create_info.enabled_layer_count = self.layers.len() as u32;

        let extensions = get_c_str_pointers(&self.extensions);
        create_info.pp_enabled_extension_names = extensions.as_ptr();
        create_info.enabled_extension_count = self.extensions.len() as u32;

        let selector = self.pdevice_selector;
        let pdevice_info = selector(&instance)?;

        let mut queues_info_builder = QueueCreateInfosBuilder::new(pdevice_info.queues_info.iter());
        let queue_infos = queues_info_builder.build();
        create_info.p_queue_create_infos = queue_infos.as_ptr();
        create_info.queue_create_info_count = queue_infos.len() as u32;

        create_info.p_enabled_features = &pdevice_info.physical_device_features;

        Device::new(instance, pdevice_info, &create_info)
    }
}

#[derive(Clone, Debug)]
pub struct Device {
    unique_device: Arc<UniqueDevice>,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.unique_device, f)
    }
}

impl Device {
    pub fn new(
        instance: Instance,
        pdevice_info: PhysicalDeviceInfo,
        create_info: &vk::DeviceCreateInfo,
    ) -> Result<Self, CreateDeviceError> {
        let unique_device = Arc::new(UniqueDevice::new(instance, pdevice_info, create_info)?);
        log::trace!("Device created: {}", unique_device);
        Ok(Self { unique_device })
    }

    /// # Safety
    /// TODO
    pub unsafe fn handle(&self) -> &ash::Device {
        &self.unique_device.handle()
    }

    /// # Safety
    /// TODO
    pub unsafe fn pdevice_info(&self) -> &PhysicalDeviceInfo {
        &self.unique_device.pdevice_info()
    }

    pub fn instance(&self) -> &Instance {
        &self.unique_device.instance()
    }
}

struct UniqueDevice {
    instance: Instance,
    pdevice_info: PhysicalDeviceInfo,
    handle: ash::Device,
}

impl UniqueDevice {
    pub fn new(
        instance: Instance,
        pdevice_info: PhysicalDeviceInfo,
        create_info: &vk::DeviceCreateInfo,
    ) -> Result<Self, CreateDeviceError> {
        log::trace!("Creating device");

        let handle = unsafe {
            instance
                .handle()
                .create_device(pdevice_info.pdevice, create_info, None)?
        };
        Ok(Self {
            instance,
            pdevice_info,
            handle,
        })
    }

    pub unsafe fn handle(&self) -> &ash::Device {
        &self.handle
    }

    pub unsafe fn pdevice_info(&self) -> &PhysicalDeviceInfo {
        &self.pdevice_info
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}

impl Drop for UniqueDevice {
    fn drop(&mut self) {
        log::trace!("Destroying vulkan device");
        unsafe { self.handle.destroy_device(None) }
    }
}

impl fmt::Debug for UniqueDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let device_info = unsafe {
            self.instance
                .handle()
                .get_physical_device_properties(self.pdevice_info.pdevice)
        };
        write!(f, "Device: {:?}", device_info)
    }
}

impl fmt::Display for UniqueDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let device_info = unsafe {
            self.instance
                .handle()
                .get_physical_device_properties(self.pdevice_info.pdevice)
        };
        let mut name_raw = device_info.device_name;
        let name = raw_name_to_c_string(&mut name_raw);
        write!(f, "Device: {}", name.to_string_lossy())
    }
}

pub struct QueueInfo {
    pub family_index: u32,
    pub count: u32,
}

struct QueueCreateInfosBuilder {
    prioreties: Vec<f32>,
    queue_infos: Vec<vk::DeviceQueueCreateInfo>,
}

impl QueueCreateInfosBuilder {
    pub fn new<'a>(infos: impl Iterator<Item = &'a QueueInfo>) -> Self {
        let queue_infos = infos
            .map(|info| vk::DeviceQueueCreateInfo {
                queue_count: info.count,
                queue_family_index: info.family_index,
                ..Default::default()
            })
            .collect();
        Self {
            queue_infos,
            prioreties: Default::default(),
        }
    }

    pub fn build(&mut self) -> &Vec<vk::DeviceQueueCreateInfo> {
        let max_queue_count = self
            .queue_infos
            .iter()
            .map(|info| info.queue_count)
            .max()
            .unwrap_or_default();
        self.prioreties = vec![1f32; max_queue_count as usize];
        for inf in &mut self.queue_infos {
            inf.p_queue_priorities = self.prioreties.as_ptr()
        }
        &self.queue_infos
    }
}

#[derive(Debug)]
pub enum CreateDeviceError {
    VkError(vk::Result),
    PhysicalDeviceError(PhysicalDeviceError),
}

impl Error for CreateDeviceError {}

impl fmt::Display for CreateDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreateDeviceError::VkError(e) => write!(f, "Vulkan device creation failed: {}", e),
            CreateDeviceError::PhysicalDeviceError(e) => {
                write!(f, "Physical device selection failed: {}", e)
            }
        }
    }
}

impl From<vk::Result> for CreateDeviceError {
    fn from(e: vk::Result) -> Self {
        Self::VkError(e)
    }
}

impl From<PhysicalDeviceError> for CreateDeviceError {
    fn from(e: PhysicalDeviceError) -> Self {
        Self::PhysicalDeviceError(e)
    }
}
