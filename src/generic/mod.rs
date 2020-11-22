pub mod raw;

use crate::device::Device;
use crate::generic::raw::RawDeviceHandle;
use ash::prelude::VkResult;
use std::sync::Arc;

pub struct UniqueDeviceHandle<T: RawDeviceHandle> {
    handle: T,
    device: Device,
    _dependencies: Vec<Box<dyn Dependence>>,
}

impl<T: RawDeviceHandle> UniqueDeviceHandle<T> {
    /// # Safety
    /// * Create info must be valid vulkan create info with valid pointers.
    /// * `dependencies` Vec must contain all handles, that must not be destroyed before destruction of `Self`.
    pub unsafe fn new(
        create_info: &T::CreateInfo,
        device: Device,
        dependencies: Vec<Box<dyn Dependence>>,
    ) -> VkResult<Self> {
        log::trace!("Creating {} with props: {}", T::name(), create_info);
        match T::create(create_info, device.handle()) {
            Ok(handle) => Ok(Self {
                handle,
                device,
                _dependencies: dependencies,
            }),
            Err(e) => {
                log::warn!(
                    "Creating {} with props: {} failed: {}",
                    T::name(),
                    create_info,
                    e
                );
                Err(e)
            }
        }
    }

    /// # Safety
    /// Copy of returned handle will become invalid after drop of `Self`.
    pub unsafe fn handle(&self) -> &T {
        &self.handle
    }
}

impl<T: RawDeviceHandle> Drop for UniqueDeviceHandle<T> {
    fn drop(&mut self) {
        log::trace!("Destroying {}", T::name());
        unsafe { self.handle.destroy(self.device.handle()) }
    }
}

impl<T: RawDeviceHandle> Eq for UniqueDeviceHandle<T> {}

impl<T: RawDeviceHandle> PartialEq for UniqueDeviceHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.handle() == other.handle() }
    }
}

pub trait Dependence {}

#[derive(Clone, Eq, PartialEq)]
pub struct DeviceHandle<T: RawDeviceHandle> {
    unique_handle: Arc<UniqueDeviceHandle<T>>,
}

impl<T: RawDeviceHandle> DeviceHandle<T> {
    /// # Safety
    /// Watch `UniqueDeviceHandle`.
    pub unsafe fn new(unique_handle: UniqueDeviceHandle<T>) -> Self {
        Self {
            unique_handle: Arc::new(unique_handle),
        }
    }

    /// # Safety
    /// Copy of returned handle will become invalid after drop of all clones of `Self`.
    pub unsafe fn handle(&self) -> &T {
        self.unique_handle.handle()
    }
}

impl<T: RawDeviceHandle> Dependence for DeviceHandle<T> {}
