/*
**  Copyright (c) 2016, Christoph Hommelsheim
**  All rights reserved.
**
**  Redistribution and use in source and binary forms, with or without
**  modification, are permitted provided that the following conditions are met:
**
**  * Redistributions of source code must retain the above copyright notice, this
**    list of conditions and the following disclaimer.
**
**  * Redistributions in binary form must reproduce the above copyright notice,
**    this list of conditions and the following disclaimer in the documentation
**    and/or other materials provided with the distribution.
**
**  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
**  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
**  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
**  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
**  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
**  DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
**  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
**  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
**  OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
**  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
**
*/
use std::ffi::CString;
use std::os::raw;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

use super::dispatch;
pub use super::dispatch::*; // some functions are re-defined here
use super::table;
use types::*;
use util;
use util::VkNullHandle;

#[cfg(not(target_os = "windows"))]
mod dl {
    use std::ptr;
    use std::io::{Error,ErrorKind,Result};
    use std::ffi::{CStr,OsStr};
    use std::os::unix::ffi::OsStrExt;
    use libc;
    use types::PFN_vkVoidFunction;

    pub struct Library(*mut libc::c_void);

    pub unsafe fn open<S: AsRef<OsStr>>(name: &S) -> Result<Library> {
        let name = name.as_ref();
        let mut buf: Vec<u8> = Vec::new();
        let name_cstr = if name.len() > 0 && name.as_bytes()[name.len() - 1] == 0 {
            CStr::from_bytes_with_nul_unchecked(name.as_bytes())
        } else {
            buf.extend_from_slice(name.as_bytes());
            buf.push(0);
            CStr::from_bytes_with_nul_unchecked(buf.as_slice())
        };
        libc::dlerror(); // clear errors
        let handle = libc::dlopen(name_cstr.as_ptr(), libc::RTLD_LOCAL | libc::RTLD_LAZY);
        if handle == ptr::null_mut() {
            let msg_cstr = CStr::from_ptr(libc::dlerror());
            return Err(Error::new(ErrorKind::Other, msg_cstr.to_string_lossy().to_string()));
        }
        Ok(Library(handle))
    }

    impl Library {
        pub unsafe fn get(&self, name: &str) -> Result<Option<PFN_vkVoidFunction>> {
            let mut v: Vec<u8> = Vec::new();
            let name_cstr = if name.len() > 0 && name.as_bytes()[name.len() - 1] == 0 {
                CStr::from_bytes_with_nul_unchecked(name.as_bytes())
            } else {
                v.extend_from_slice(name.as_bytes());
                v.push(0);
                CStr::from_bytes_with_nul_unchecked(v.as_slice())
            };
            libc::dlerror(); // clear errors
            let sym = libc::dlsym(self.0, name_cstr.as_ptr());
            if sym == ptr::null_mut() {
                let msg = libc::dlerror();
                if msg != ptr::null_mut() {
                    let msg_cstr = CStr::from_ptr(msg);
                    return Err(Error::new(ErrorKind::Other, msg_cstr.to_string_lossy().to_string()));
                }
            }
            Ok(::std::mem::transmute(sym))
        }
    }

}

#[cfg(target_os = "windows")]
mod dl {
    use winapi;
    use kernel32;
    use std::ptr;
    use std::io::{Error,Result};
    use std::os::raw;
    use std::ffi::{CStr,OsStr};
    use std::os::unix::ffi::OsStrExt;
    use types::PFN_vkVoidFunction;

    pub struct Library(winapi::HMODULE);

    pub unsafe fn open<S: AsRef<OsStr>>(name: &S) -> Result<Library> {
        let name_wide: Vec<u16> = name.as_ref().encode_wide().chain(Some(0)).collect();
        SetLastError(0); // clear errors
        let handle = kernel32::LoadLibraryW(name_wide.as_ptr());
        if handle == ptr::null_mut() {
            let err = kernel32::GetLastError();
            if err == 0 {
                return Err(Error::new(ErrorKind::Other, "LoadLibraryW didn't return any library handle"));
            } else {
                return Err(IoError::from_raw_os_error(err as i32));
            }
        }
        Ok(Library(handle))
    }

    impl Library {
        pub unsafe fn get(&self, name: &str) -> Result<Option<PFN_vkVoidFunction>> {
            let mut v: Vec<u8> = Vec::new();
            let name_cstr = if name.len() > 0 && name.as_bytes()[name.len() - 1] == 0 {
                CStr::from_bytes_with_nul_unchecked(name.as_bytes())
            } else {
                v.extend_from_slice(name.as_bytes());
                v.push(0);
                CStr::from_bytes_with_nul_unchecked(v.as_slice())
            };
            SetLastError(0); // clear errors
            let sym = GetProcAddress(self.0, name_cstr.as_ptr());
            if sym == ptr::null_mut() {
                let err = kernel32::GetLastError();
                if err != 0 {
                    return Err(IoError::from_raw_os_error(err as i32));
                }
            }
            Ok(::std::mem::transmute(sym))
        }
    }
}

#[inline]
fn wrap_proc_addr<T>(get_proc_addr: extern "system" fn (T,*const raw::c_char) -> Option<PFN_vkVoidFunction>, h: T, n: &str) -> util::VkResultObj<PFN_vkVoidFunction> {
    let cn = CString::new(n).unwrap();
    if let Some(f) = get_proc_addr(h, cn.as_ptr()) {
        debug!("loaded function '{}'", n);
        Ok(f)
    } else {
        warn!("Unable to load function '{}'", n);
        Err(VK_ERROR_EXTENSION_NOT_PRESENT)
    }
}

#[allow(non_camel_case_types)]
pub type PFN_vkGetInstanceProcAddr = extern "system" fn (VkInstance, *const raw::c_char) -> Option<PFN_vkVoidFunction>;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceProcAddr = extern "system" fn (VkDevice, *const raw::c_char) -> Option<PFN_vkVoidFunction>;

#[cfg(not(target_os = "windows"))]
const VULKAN_LIB_NAME : &str = "libvulkan.so";

#[cfg(target_os = "windows")]
const VULKAN_LIB_NAME : &str = "vulkan.dll";

lazy_static!{
    static ref LOADER_DATA: Option<(table::VkLoaderTable,PFN_vkGetInstanceProcAddr)> = {
        unsafe {
            match dl::open(&VULKAN_LIB_NAME) {
                Err(e) => {
                    error!("unable to load vulkan library: {}", e);
                    None
                },
                Ok(lib) => {
                    debug!("loaded {}", VULKAN_LIB_NAME);
                    let get_inst_proc_addr : Result<Option<PFN_vkGetInstanceProcAddr>,::std::io::Error> = ::std::mem::transmute(lib.get("vkGetInstanceProcAddr"));
                    match get_inst_proc_addr {
                        Ok(Some(get_inst_proc_addr)) => {
                            debug!("loaded vkGetInstanceProcAddr from library");
                            let mut tab : table::VkLoaderTable = ::std::mem::zeroed();
                            debug!("loading VkLoaderTable");
                            match tab.load(|n| wrap_proc_addr(get_inst_proc_addr, util::vk_null_handle(), n)) {
                                Ok(_) => Some((tab,get_inst_proc_addr)),
                                Err(e) => {
                                    error!("unable to load vulkan table (loader): {:?}", e);
                                    None
                                }
                            }
                        },
                        Ok(None) => {
                            error!("unable to find vkGetInstanceProcAddr function");
                            None
                        },
                        Err(e) => {
                            error!("unable to find vkGetInstanceProcAddr function: {}", e);
                            None
                        }
                    }
                }
            }
        }
    };
}

impl table::VkLoaderTable {
    pub unsafe fn get() -> Option<&'static table::VkLoaderTable> {
        if let Some((ref tab,_)) = *LOADER_DATA {
            Some(tab)
        } else {
            None
        }
    }
}

static mut INSTANCE_DATA : Option<(table::VkInstanceTable,VkInstance)> = None;
static mut INSTANCE_INITIALIZED : AtomicUsize = ATOMIC_USIZE_INIT; // 0=uninitialized, 1=initialized, 2=busy

impl table::VkInstanceTable {
    pub unsafe fn get() -> Option<&'static table::VkInstanceTable> {
        if INSTANCE_INITIALIZED.load(Ordering::Relaxed) == 1 {
            if let Some((ref t,_)) = INSTANCE_DATA {
                return Some(t);
            }
        }
        None
    }
    unsafe fn initialize_from<E>(instance: VkInstance, has_extension: E) where E: Fn(&str) -> bool {
        if let Some((_, mut get_inst_proc_addr)) = *LOADER_DATA {
            if let Ok(tmp) = wrap_proc_addr(get_inst_proc_addr, instance, "vkGetInstanceProcAddr") {
                get_inst_proc_addr = ::std::mem::transmute(tmp);
                let mut tab : table::VkInstanceTable = ::std::mem::zeroed();
                tab.vkGetInstanceProcAddr = get_inst_proc_addr;
                debug!("loading VkInstanceTable for {:?}", instance);
                if tab.load(|n| wrap_proc_addr(get_inst_proc_addr, instance, n), has_extension).is_ok() {
                    INSTANCE_DATA = Some((tab, instance));
                }
            }
        }
    }
}

static mut DEVICE_DATA : Option<(table::VkDeviceTable,VkDevice)> = None;
static mut DEVICE_INITIALIZED : AtomicUsize = ATOMIC_USIZE_INIT; // 0=uninitialized, 1=initialized, 2=busy

impl table::VkDeviceTable {
    pub unsafe fn get() -> Option<&'static table::VkDeviceTable> {
        if DEVICE_INITIALIZED.load(Ordering::Relaxed) == 1 {
            if let Some((ref t,_)) = DEVICE_DATA {
                return Some(t);
            }
        }
        None
    }
    unsafe fn initialize_from<E>(device: VkDevice, has_extension: E) where E: Fn(&str) -> bool {
        if INSTANCE_INITIALIZED.load(Ordering::Relaxed) != 1 {
            return;
        };
        if let Some((ref _it, instance)) = INSTANCE_DATA {
            let mut get_device_proc_addr : PFN_vkGetDeviceProcAddr;
            if let Ok(tmp) = wrap_proc_addr(_it.vkGetInstanceProcAddr, instance, "vkGetDeviceProcAddr") {
                get_device_proc_addr = ::std::mem::transmute(tmp);
                if let Ok(tmp) = wrap_proc_addr(get_device_proc_addr, device, "vkGetDeviceProcAddr") {
                    get_device_proc_addr = ::std::mem::transmute(tmp);
                    let mut tab : table::VkDeviceTable = ::std::mem::zeroed();
                    tab.vkGetDeviceProcAddr = get_device_proc_addr;
                    debug!("loading VkDeviceTable for {:?} and {:?}", instance, device);
                    if tab.load(|n| wrap_proc_addr(get_device_proc_addr, device, n), has_extension).is_ok() {
                        DEVICE_DATA = Some((tab, device));
                    }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
pub unsafe fn vkGetInstanceProcAddr (instance: VkInstance, p_name: *const raw::c_char) -> Option<PFN_vkVoidFunction> {
    if let Some(v) = dispatch::vkGetInstanceProcAddr(instance, p_name) {
        Some(v)
    } else if let Some((_, get_inst_proc_addr)) = *LOADER_DATA {
        get_inst_proc_addr(instance, p_name)
    } else {
        None
    }
}

#[allow(non_snake_case)]
pub unsafe fn vkCreateInstance (p_create_info: *const VkInstanceCreateInfo, p_allocator: *const VkAllocationCallbacks, p_instance: *mut VkInstance) -> VkResult {
    if p_instance != util::vk_null() {
        *p_instance = VkInstance::null();
    }
    if INSTANCE_INITIALIZED.compare_and_swap(0, 2, Ordering::Relaxed) != 0 {
        warn!("unable to call 'vkCreateInstance': multiple instances of 'VkInstance' are not supported!");
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let mut instance = VkInstance::null();
    let result = dispatch::vkCreateInstance(p_create_info, p_allocator, &mut instance);
    if result == VK_SUCCESS {
        if p_instance != util::vk_null() {
            *p_instance = instance;
        }
        let extensions = if !p_create_info.is_null() {
            util::extensions_list_to_set((*p_create_info).ppEnabledExtensionNames, (*p_create_info).enabledExtensionCount)
        } else {
            ::std::collections::BTreeSet::new()
        };
        table::VkInstanceTable::initialize_from(instance, |e| extensions.contains(e));
        INSTANCE_INITIALIZED.store(1, Ordering::Relaxed);
    } else {
        INSTANCE_INITIALIZED.store(0, Ordering::Relaxed);
    }
    result
}

#[allow(non_snake_case)]
pub unsafe fn vkCurrentInstance() -> Option<VkInstance> {
    if INSTANCE_INITIALIZED.load(Ordering::Relaxed) == 1 {
        if let Some((_,i)) = INSTANCE_DATA {
            return Some(i);
        }
    }
    None
}

#[allow(non_snake_case)]
pub unsafe fn vkDestroyInstance (instance: VkInstance, p_allocator: *const VkAllocationCallbacks) {
    dispatch::vkDestroyInstance(instance, p_allocator);
    if INSTANCE_INITIALIZED.compare_and_swap(1, 2, Ordering::Relaxed) == 1 {
        INSTANCE_DATA = None;
        DEVICE_INITIALIZED.store(0, Ordering::Relaxed);
    } else {
        warn!("unable to destroy VkInstanceTable, was not initialized");
    }
}

#[allow(non_snake_case)]
pub unsafe fn vkCreateDevice (physical_device: VkPhysicalDevice, p_create_info: *const VkDeviceCreateInfo, p_allocator: *const VkAllocationCallbacks, p_device: *mut VkDevice) -> VkResult {
    if p_device != util::vk_null() {
        *p_device = VkDevice::null();
    }
    if DEVICE_INITIALIZED.compare_and_swap(0, 2, Ordering::Relaxed) != 0 {
        warn!("unable to call 'vkCreateDevice': multiple instances of 'VkDevice' are not supported!");
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    let mut device = VkDevice::null();
    let result = dispatch::vkCreateDevice(physical_device, p_create_info, p_allocator, &mut device);
    if result == VK_SUCCESS {
        if p_device != util::vk_null() {
            *p_device = device;
        }
        let extensions = if !p_create_info.is_null() {
            util::extensions_list_to_set((*p_create_info).ppEnabledExtensionNames, (*p_create_info).enabledExtensionCount)
        } else {
            ::std::collections::BTreeSet::new()
        };
        table::VkDeviceTable::initialize_from(device, |e| extensions.contains(e));
        DEVICE_INITIALIZED.store(1, Ordering::Relaxed);
    } else {
        DEVICE_INITIALIZED.store(0, Ordering::Relaxed);
    }
    result
}

#[allow(non_snake_case)]
pub unsafe fn vkCurrentDevice<'l>() -> Option<VkDevice> {
    if INSTANCE_INITIALIZED.load(Ordering::Relaxed) == 1 {
        if let Some((_,d)) = DEVICE_DATA {
            return Some(d);
        }
    }
    None
}

#[allow(non_snake_case)]
pub unsafe fn vkDestroyDevice (device: VkDevice, p_allocator: *const VkAllocationCallbacks) {
    dispatch::vkDestroyDevice(device, p_allocator);
    if DEVICE_INITIALIZED.compare_and_swap(1, 2, Ordering::Relaxed) == 1 {
        DEVICE_DATA = None;
        DEVICE_INITIALIZED.store(0, Ordering::Relaxed);
    } else {
        warn!("unable to destroy VkDeviceTable, was not initialized");
    }
}
