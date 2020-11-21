use crate::{get_c_str_pointers, ContainRawVkName};
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk::InstanceCreateInfo;
use ash::{vk, InstanceError};
use std::ffi::CString;
use std::sync::Arc;

pub struct InstanceBuilder {
    layers: Vec<CString>,
    extensions: Vec<CString>,
    entry: ash::Entry,
    app_info: vk::ApplicationInfo,
}

impl InstanceBuilder {
    pub fn new(entry: ash::Entry) -> Self {
        Self {
            entry,
            app_info: Default::default(),
            layers: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn with_api_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.app_info.api_version = vk::make_version(major, minor, patch);
        self
    }

    pub fn with_layers(mut self, layers: Vec<CString>) -> Self {
        self.layers = layers;
        self
    }

    pub fn with_extensions(mut self, extensions: Vec<CString>) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn build(self) -> Result<Instance, InstanceError> {
        let mut create_info = vk::InstanceCreateInfo::default();
        create_info.p_application_info = &self.app_info;

        create_info.enabled_layer_count = self.layers.len() as u32;
        let layers = get_c_str_pointers(&self.layers);
        create_info.pp_enabled_layer_names = layers.as_ptr();

        create_info.enabled_extension_count = self.extensions.len() as u32;
        let extensions = get_c_str_pointers(&self.extensions);
        create_info.pp_enabled_extension_names = extensions.as_ptr();

        unsafe { Instance::new(self.entry, &create_info) }
    }

    pub fn debug_layers(entry: ash::Entry) -> Vec<CString> {
        let layers = entry
            .enumerate_instance_layer_properties()
            .unwrap_or_default();

        let name = layers
            .into_iter()
            .map(|mut l| l.c_string_name())
            .find(|name| *name == CString::new("VK_LAYER_KHRONOS_validation").unwrap());
        name.into_iter().collect()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Instance {
    unique_instance: Arc<UniqueInstance>,
}

impl Instance {
    /// # Safety
    /// todo
    pub unsafe fn new(
        entry: ash::Entry,
        create_info: &InstanceCreateInfo,
    ) -> Result<Self, InstanceError> {
        UniqueInstance::new(entry, &create_info).map(|inst| Self {
            unique_instance: Arc::new(inst),
        })
    }

    /// # Safety
    /// TODO
    pub unsafe fn handle(&self) -> &ash::Instance {
        &self.unique_instance.handle()
    }

    pub fn entry(&self) -> &ash::Entry {
        &self.unique_instance.entry()
    }
}

struct UniqueInstance {
    handle: ash::Instance,
    entry: ash::Entry,
}

impl UniqueInstance {
    pub unsafe fn new(
        entry: ash::Entry,
        create_info: &InstanceCreateInfo,
    ) -> Result<Self, InstanceError> {
        log::trace!("Creating vulkan instance");
        let handle = entry.create_instance(create_info, None)?;
        Ok(Self { entry, handle })
    }

    pub unsafe fn handle(&self) -> &ash::Instance {
        &self.handle
    }

    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }
}

impl Drop for UniqueInstance {
    fn drop(&mut self) {
        log::trace!("Destroying vulkan instance");
        unsafe { self.handle.destroy_instance(None) }
    }
}

impl Eq for UniqueInstance {}

impl PartialEq for UniqueInstance {
    fn eq(&self, other: &Self) -> bool {
        self.handle.handle() == other.handle.handle()
    }
}
