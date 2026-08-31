use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ffi::{CStr, CString};
use std::ptr;
use std::io;

// From hardware/hardware.h (simplified)
#[repr(C)]
pub struct hw_module_t {
    pub tag: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub id: *const c_char,
    pub name: *const c_char,
    pub author: *const c_char,
    pub methods: *const c_void, // hw_module_methods_t*
    pub dso: *mut c_void,
    pub reserved: [c_char; 8],
}

#[repr(C)]
pub struct boot_control_module {
    pub common: hw_module_t,
    pub init: Option<unsafe extern "C" fn(*mut boot_control_module)>,
    pub getNumberSlots: Option<unsafe extern "C" fn(*mut boot_control_module) -> c_uint>,
    pub getCurrentSlot: Option<unsafe extern "C" fn(*mut boot_control_module) -> c_uint>,
    pub markBootSuccessful: Option<unsafe extern "C" fn(*mut boot_control_module) -> c_int>,
    pub setActiveBootSlot: Option<unsafe extern "C" fn(*mut boot_control_module, c_uint) -> c_int>,
    pub setSlotAsUnbootable: Option<unsafe extern "C" fn(*mut boot_control_module, c_uint) -> c_int>,
    pub isSlotBootable: Option<unsafe extern "C" fn(*mut boot_control_module, c_uint) -> c_int>,
    pub getSuffix: Option<unsafe extern "C" fn(*mut boot_control_module, c_uint) -> *const c_char>,
    pub isSlotMarkedSuccessful: Option<unsafe extern "C" fn(*mut boot_control_module, c_uint) -> c_int>,
    pub getActiveBootSlot: Option<unsafe extern "C" fn(*mut boot_control_module) -> c_uint>,
    pub reserved: [*mut c_void; 30],
}

pub type boot_control_module_t = boot_control_module;


// for loading func

const BOOT_CONTROL_HARDWARE_MODULE_ID: &str = "bootctrl\0";

#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *mut c_char;
}

const RTLD_NOW: c_int = 2;

pub struct BootControlHal {
    lib_handle: *mut c_void,
    module: *mut boot_control_module_t,
}

impl BootControlHal {
    pub fn load() -> io::Result<Self> {
        unsafe {
            // Load libhardware.so
            let libhardware = CString::new("libhardware.so").unwrap();
            let handle = dlopen(libhardware.as_ptr(), RTLD_NOW);
            if handle.is_null() {
                return Err(io::Error::new(io::ErrorKind::Other, "dlopen failed"));
            }

            // Get hw_get_module symbol
            let sym = CString::new("hw_get_module").unwrap();
            let hw_get_module_ptr = dlsym(handle, sym.as_ptr());
            if hw_get_module_ptr.is_null() {
                dlclose(handle);
                return Err(io::Error::new(io::ErrorKind::Other, "dlsym hw_get_module failed"));
            }

            let hw_get_module: extern "C" fn(*const c_char, *mut *mut c_void) -> c_int
                = std::mem::transmute(hw_get_module_ptr);

            // Call hw_get_module("bootctrl", &mut module)
            let module_id = CString::new(BOOT_CONTROL_HARDWARE_MODULE_ID).unwrap();
            let mut module_ptr: *mut c_void = ptr::null_mut();
            let ret = hw_get_module(module_id.as_ptr(), &mut module_ptr);
            if ret != 0 {
                dlclose(handle);
                return Err(io::Error::new(io::ErrorKind::Other, format!("hw_get_module failed: {}", ret)));
            }

            Ok(BootControlHal {
                lib_handle: handle,
                module: module_ptr as *mut boot_control_module_t,
            })
        }
    }

    pub unsafe fn module(&self) -> &boot_control_module_t {
        &*self.module
    }
}

impl Drop for BootControlHal {
    fn drop(&mut self) {
        unsafe {
            if !self.lib_handle.is_null() {
                dlclose(self.lib_handle);
            }
        }
    }
}

