use crate::instance::Instance;
use ash::extensions::ext;
use ash::vk;
use std::ffi::CStr;
use std::fmt;
use std::os::raw::{c_char, c_void};
use std::sync::Arc;
use ash::prelude::VkResult;

#[derive(Debug, Copy, Clone)]
pub enum MessageLevel {
    Information,
    Warning,
    Perfomance,
    Error,
    Debug,
}

impl From<vk::DebugReportFlagsEXT> for MessageLevel {
    fn from(flags: vk::DebugReportFlagsEXT) -> Self {
        if flags.contains(vk::DebugReportFlagsEXT::ERROR) {
            return Self::Error;
        }

        if flags.contains(vk::DebugReportFlagsEXT::WARNING) {
            return Self::Warning;
        }

        if flags.contains(vk::DebugReportFlagsEXT::PERFORMANCE_WARNING) {
            return Self::Perfomance;
        }

        if flags.contains(vk::DebugReportFlagsEXT::DEBUG) {
            return Self::Debug;
        }

        if flags.contains(vk::DebugReportFlagsEXT::INFORMATION) {
            return Self::Information;
        }

        Self::Error
    }
}

impl fmt::Display for MessageLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            MessageLevel::Information => write!(f, "INFO"),
            MessageLevel::Warning => write!(f, "WARN"),
            MessageLevel::Perfomance => write!(f, "PERF"),
            MessageLevel::Error => write!(f, "ERRO"),
            MessageLevel::Debug => write!(f, "DEBG"),
        }
    }
}

impl From<MessageLevel> for log::Level {
    fn from(ml: MessageLevel) -> Self {
        match ml {
            MessageLevel::Information => log::Level::Info,
            MessageLevel::Warning => log::Level::Warn,
            MessageLevel::Perfomance => log::Level::Warn,
            MessageLevel::Error => log::Level::Error,
            MessageLevel::Debug => log::Level::Debug,
        }
    }
}

pub struct Callback(pub Box<dyn Fn(String, MessageLevel) + 'static>);

pub struct DebugReportBuilder {
    callback: Callback,
    flags: vk::DebugReportFlagsEXT,
}

impl Default for DebugReportBuilder {
    fn default() -> Self {
        let callback = |msg, level| println!("Vulkan callback report [{}]: {}", level, msg);
        Self {
            callback: Callback(Box::new(callback)),
            flags: vk::DebugReportFlagsEXT::all(),
        }
    }
}

impl DebugReportBuilder {
    pub fn with_flags(mut self, flags: vk::DebugReportFlagsEXT) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_callback(mut self, callback: Callback) -> Self {
        self.callback = callback;
        self
    }

    pub fn build(self, instance: Instance) -> VkResult<DebugReport> {
        let cb = Box::new(self.callback);
        let ud = Box::leak(cb) as *mut Callback;
        let ud_void = ud as *mut c_void;

        let create_info = vk::DebugReportCallbackCreateInfoEXT {
            flags: self.flags,
            pfn_callback: Some(debug_report_callback),
            p_user_data: ud_void,
            ..Default::default()
        };

        unsafe { DebugReport::new(instance, &create_info, ud) }
    }

    pub fn default_logger_callback() -> Callback {
        let callback = |msg, level: MessageLevel| {
            log::log!(level.into(), "Vulkan report: {}", msg);
        };
        Callback(Box::new(callback))
    }
}

unsafe extern "system" fn debug_report_callback(
    flags: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const c_char,
    p_message: *const c_char,
    p_user_data: *mut c_void,
) -> vk::Bool32 {
    let callback: *mut Callback = p_user_data.cast();
    let callback_ref = callback.as_ref();
    let msg = CStr::from_ptr(p_message);
    let str = msg.to_string_lossy();
    let level = flags.into();
    match callback_ref {
        Some(cb) => cb.0(format!("{}", str), level),
        None => eprintln!("Can't dereference vk debug report callback pointer"),
    }

    vk::FALSE
}

#[derive(Clone, Eq, PartialEq)]
pub struct DebugReport {
    unique_debug_report: Arc<UniqueDebugReport>,
}

impl DebugReport {
    /// # Safety
    /// todo
    pub unsafe fn new(
        instance: Instance,
        create_info: &vk::DebugReportCallbackCreateInfoEXT,
        callback: *mut Callback,
    ) -> VkResult<Self> {
        UniqueDebugReport::new(instance, create_info, callback).map(|uniq| Self {
            unique_debug_report: Arc::new(uniq),
        })
    }

    ///# Safety
    /// TODO
    pub unsafe fn handle(&self) -> &vk::DebugReportCallbackEXT {
        &self.unique_debug_report.handle()
    }

    pub fn instance(&self) -> &Instance {
        &self.unique_debug_report.instance()
    }
}

struct UniqueDebugReport {
    instance: Instance,
    debug_report: ext::DebugReport,
    handle: vk::DebugReportCallbackEXT,
    callback: *mut Callback,
}

impl UniqueDebugReport {
    pub unsafe fn new(
        instance: Instance,
        create_info: &vk::DebugReportCallbackCreateInfoEXT,
        callback: *mut Callback,
    ) -> VkResult<Self> {
        let level: MessageLevel = create_info.flags.into();
        log::trace!("Creating vk debug report with level: {}", level);

        let instance_raw = instance.handle().clone();
        let debug_report = ext::DebugReport::new(instance.entry(), &instance_raw);
        let handle = debug_report.create_debug_report_callback(create_info, None)?;

        Ok(Self {
            debug_report,
            handle,
            instance,
            callback,
        })
    }

    pub unsafe fn handle(&self) -> &vk::DebugReportCallbackEXT {
        &self.handle
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}

impl Drop for UniqueDebugReport {
    fn drop(&mut self) {
        log::trace!("Destroying vk debug report with it's callback");
        unsafe {
            self.debug_report
                .destroy_debug_report_callback(self.handle, None);
            let _cb = Box::from_raw(self.callback);
        }
    }
}

impl Eq for UniqueDebugReport {}

impl PartialEq for UniqueDebugReport {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.handle() == other.handle() }
    }
}