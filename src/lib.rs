use ash::vk;
use std::ffi::{CStr, CString};

pub mod buffer;
pub mod command_buffer;
pub mod command_pool;
pub mod debug_report;
pub mod desc_set_layout;
pub mod device;
pub mod instance;
pub mod memory;
pub mod queue;
pub mod sampler;
pub mod generic;

fn get_c_str_pointers(strs: &[CString]) -> Vec<*const i8> {
    let mut ptrs = Vec::with_capacity(strs.len());
    for layer in strs {
        ptrs.push(layer.as_ptr());
    }
    ptrs
}

pub fn raw_name_to_c_string(raw: &mut [i8]) -> CString {
    if raw.is_empty() {
        return CString::new("").unwrap();
    }
    if let Some(last) = raw.last_mut() {
        *last = 0 // To ensure that EOL symbol present
    }
    let c_str = unsafe { CStr::from_ptr(raw.as_ptr()) };
    c_str.to_owned()
}

pub trait ContainRawVkName {
    fn get_name(&mut self) -> &mut [i8];
    fn c_string_name(&mut self) -> CString {
        raw_name_to_c_string(self.get_name())
    }
}

impl ContainRawVkName for vk::LayerProperties {
    fn get_name(&mut self) -> &mut [i8] {
        self.layer_name.as_mut()
    }
}

impl ContainRawVkName for vk::ExtensionProperties {
    fn get_name(&mut self) -> &mut [i8] {
        self.extension_name.as_mut()
    }
}
