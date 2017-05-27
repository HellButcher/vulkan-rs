/*
Copyright (c) 2016, Christoph Hommelsheim
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use winit;
use std::os::raw::c_char;
use std::ffi::{CStr,CString};
use vulkan_rs::prelude::*;

macro_rules! vk_drop {
    ( $f:path ; [ $($p:expr),* ] $e:expr $(, $r:expr)* ) => {
        if $e != vk_null_handle() {
            debug!("dropping: {} @ line {}", stringify!($f), line!());
            $f ( $($p,)* $e $(, $r)* );
            $e = vk_null_handle();
        }
    };
}

const QUEUE_COUNT : usize = 2;
const GRAPHIC_QUEUE: usize = 0;
const PRESENT_QUEUE: usize = 1;
const INVALID_INDEX : u32 = (-1i32) as u32;

#[derive(Default)]
pub struct Application {
    instance: VkInstance,
    surface: VkSurfaceKHR,
    physical_device: VkPhysicalDevice,
    queues: [VkQueue; QUEUE_COUNT],
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    swapchain_images: Vec<VkImage>,
    swapchain_image_views: Vec<VkImageView>,
}

#[cfg(target_os = "windows")]
fn get_required_instance_extensions(w: &winit::Window) -> Vec<&'static str> {
    return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                VK_KHR_WIN32_SURFACE_EXTENSION_NAME];
}

#[cfg(target_os = "linux")]
fn get_required_instance_extensions(w: &winit::Window) -> Vec<&'static str> {
    use winit::os::unix::WindowExt;
    if let Some(_) = (w as &WindowExt).get_wayland_display() {
        return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                    VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME];
    } else if let Some(_) = (w as &WindowExt).get_xlib_display() {
        return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                    VK_KHR_XLIB_SURFACE_EXTENSION_NAME];
    } /* else if let Some(_) = (w as &WindowExt).get_xcb_connection() {
        return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                    VK_KHR_XCB_SURFACE_EXTENSION_NAME];
    } */ else {
        return vec![VK_KHR_SURFACE_EXTENSION_NAME];
    }
}

#[cfg(target_os = "android")]
fn get_required_instance_extensions(w: &winit::Window) -> Vec<&'static str> {
    return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                VK_KHR_ANDROID_SURFACE_EXTENSION_NAME];
}

fn get_required_device_extensions() -> Vec<&'static str> {
    return vec![VK_KHR_SWAPCHAIN_EXTENSION_NAME];
}

#[cfg(target_os = "windows")]
fn create_surface(instance: VkInstance, w: &winit::Window) -> VkResultObj<VkSurfaceKHR> {
    use kernel32;
    let create_info = VkWin32SurfaceCreateInfoKHR {
        sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
        pNext: vk_null(),
        flags: 0,
        hinstance: kernel32::GetModuleHandleW(ptr::null()),
        hwnd: w.get_hwnd(),
    };
    let surface = try!(vkCreateWin32SurfaceKHR(instance, &create_info, None));
    debug!("created windows surface {:?}", surface);
    return Ok(surface);
}

#[cfg(target_os = "linux")]
fn create_surface(instance: VkInstance, w: &winit::Window) -> VkResultObj<VkSurfaceKHR> {
    use winit::os::unix::WindowExt;
    if let Some(wayland_display) = w.get_wayland_display() {
        let wayland_surface = w.get_wayland_surface().unwrap();
        debug!("wayland display is {}; wayland surface is {}", wayland_display as usize, wayland_surface as usize);
        let create_info = VkWaylandSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
            pNext: vk_null(),
            flags: 0,
            display: wayland_display as *mut vk_platform::wayland::wl_display,
            surface: wayland_surface as *mut vk_platform::wayland::wl_surface,
        };
        let surface = try!(vkCreateWaylandSurfaceKHR(instance, &create_info, None));
        debug!("created wayland surface {:?}", surface);
        return Ok(surface);
    } else if let Some(xlib_display) = w.get_xlib_display() {
        let xlib_window = w.get_xlib_window().unwrap();
        debug!("xlib display is {}; xlib window is {}", xlib_display as usize, xlib_window as usize);
        let create_info = VkXlibSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
            pNext: vk_null(),
            flags: 0,
            dpy: xlib_display as *mut vk_platform::xlib::Display,
            window: xlib_window as vk_platform::xlib::Window,
        };
        let surface = try!(vkCreateXlibSurfaceKHR(instance, &create_info, None));
        debug!("created xlib surface {:?}", surface);
        return Ok(surface);
    } /* else if let Some(xcb_connection) = w.get_xcb_connection() {
        let xcb_window = w.get_xlib_window().unwrap();
        debug!("xcb connection is {}; xcb window is {}", xcb_connection as usize, xcb_window as usize);
        let create_info = VkXcbSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: vk_null(),
            flags: 0,
            connection: xcb_connection as *mut vk_platform::xcb::xcb_connection_t,
            window: xcb_window as vk_platform::xcb::xcb_window_t,
        };
        let surface = try!(vkCreateXcbSurfaceKHR(instance, &create_info, None));
        debug!("created xcb surface {:?}", surface);
        return Ok(surface);
    } */ else {
        return Err(VK_ERROR_EXTENSION_NOT_PRESENT.into());
    }
}


#[cfg(target_os = "android")]
fn create_surface(instance: VkInstance, w: &winit::Window) -> VkResultObj<VkSurfaceKHR> {
    use kernel32;
    let create_info = VkAndroidSurfaceCreateInfoKHR {
        sType: VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR,
        pNext: vk_null(),
        flags: 0,
        window: w.get_native_window() as *mut vk_platform::android::ANativeWindow,
    };
    let surface = try!(vkCreateAndroidSurfaceKHR(instance, &create_info, None));
    debug!("created android surface {:?}", surface);
    return Ok(surface);
}

fn choose_queue_family_indices(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR) -> VkResultObj<[u32;2]> {
    let mut result : [u32;QUEUE_COUNT] = [INVALID_INDEX, INVALID_INDEX];
    let queue_family_props = vkGetPhysicalDeviceQueueFamilyProperties(physical_device);
    debug!("got {} queue family properties", queue_family_props.len());

    for (i, props) in queue_family_props.iter().enumerate() {
        debug!("querying queue family {}: queues={}, flags={}", i, props.queueCount, props.queueFlags);
        if props.queueCount > 0 {
            let has_surface_support = if surface != vk_null_handle() {
                try!(vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, i as u32, surface))
            } else {
                VK_FALSE
            };
            debug!("has_surface_support={}", has_surface_support);

            if (props.queueFlags&VK_QUEUE_GRAPHICS_BIT)!=0 {
                if result[GRAPHIC_QUEUE] == INVALID_INDEX {
                    result[GRAPHIC_QUEUE] = i as u32;
                }
                if has_surface_support != VK_FALSE {
                    result[GRAPHIC_QUEUE] = i as u32;
                    result[PRESENT_QUEUE] = i as u32;
                    break;
                }
            }

            if has_surface_support != VK_FALSE && result[PRESENT_QUEUE] == INVALID_INDEX {
                result[PRESENT_QUEUE] = i as u32;
            }
        }
    }
    return Ok(result);
}

fn get_device_queues(device: VkDevice, queue_family_indices: &[u32; QUEUE_COUNT]) -> [VkQueue; QUEUE_COUNT] {
    let mut result : [VkQueue; QUEUE_COUNT] = [vk_null_handle(), vk_null_handle()];
    for i in 0..QUEUE_COUNT {
        if queue_family_indices[i] != INVALID_INDEX {
            result[i] = vkGetDeviceQueue(device, queue_family_indices[i], 0);
            debug!("created device queue {:?} for index {}", result[i], i);
        }
    }
    return result;
}

fn create_instance(app_aame: &str, exts: &[&str]) -> VkResultObj<VkInstance> {
    let app_aame = CString::new(app_aame).unwrap();
    let exts: Vec<CString> = exts.iter().map(|s| CString::new(*s).unwrap()).collect();
    let exts_p: Vec<*const c_char> = exts.iter().map(|s| s.as_ptr()).collect();
    let app_info = VkApplicationInfo {
        sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: vk_null(),
        pApplicationName: app_aame.as_ptr(),
        applicationVersion: 1,
        pEngineName: app_aame.as_ptr(),
        engineVersion: 1,
        apiVersion: VK_API_VERSION_1_0,
    };
    let create_info = VkInstanceCreateInfo {
        sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: vk_null(),
        enabledExtensionCount: exts_p.len() as u32,
        ppEnabledExtensionNames: exts_p.as_ptr(),
    };
    let instance = try!(vkCreateInstance(&create_info, None));
    debug!("created instalce {:?}", instance);
    return Ok(instance);
}

#[derive(Default)]
struct DeviceInitializationDetails {
    queue_family_indices: [u32; QUEUE_COUNT],
    surface_format: VkSurfaceFormatKHR,
    surface_present_mode: VkPresentModeKHR,
    surface_capabilities: VkSurfaceCapabilitiesKHR,
}

fn choose_physical_device(instance: VkInstance, exts: &[&str], surface: VkSurfaceKHR) -> VkResultObj<(VkPhysicalDevice,DeviceInitializationDetails)> {
    let devices = try!(vkEnumeratePhysicalDevices(instance));
    if devices.len() <= 0 {
        warn!("there are no physical devices!");
        return Err(VK_ERROR_INITIALIZATION_FAILED.into());
    }
    for (i, physical_device) in devices.into_iter().enumerate() {
        debug!("check physical device {:?} with index {}", physical_device, i);
        match check_device(physical_device, exts, surface) {
            Ok(details) => {
                debug!("choosed physical device {:?} with index {}", physical_device, i);
                return Ok((physical_device, details));
            },
            Err(e) => {
                debug!("physical device {:?} with index {} not choosed: {}", physical_device, i, e);
            },
        }
    }
    warn!("no suitable physical device found!");
    return Err(VK_ERROR_INITIALIZATION_FAILED.into());
}

fn check_device(physical_device: VkPhysicalDevice, exts: &[&str], surface: VkSurfaceKHR) -> VkResultObj<DeviceInitializationDetails> {
    let indices = try!(choose_queue_family_indices(physical_device, surface));
    if indices[0] == INVALID_INDEX || (surface != vk_null_handle() && indices[1] == INVALID_INDEX) {
        debug!("device {:?} has no suitable queue family", physical_device);
        return Err(VK_ERROR_INITIALIZATION_FAILED.into());
    }
    let mut details : DeviceInitializationDetails = Default::default();
    details.queue_family_indices = indices;
    if ! try!(check_device_extension_support(physical_device, exts)) {
        debug!("device {:?} doesn't support all required extensions", physical_device);
        return Err(VK_ERROR_INITIALIZATION_FAILED.into());
    }
    if surface != vk_null_handle() {
        details.surface_capabilities = try!(vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, surface));
        details.surface_format = try!(choose_surface_format(physical_device, surface));
        details.surface_present_mode = try!(choose_surface_present_mode(physical_device, surface));
    }
    return Ok(details);
}

fn check_device_extension_support(physical_device: VkPhysicalDevice, exts: &[&str]) -> VkResultObj<bool> {
    let mut exts : ::std::collections::HashSet<&str> = exts.iter().cloned().collect();
    let extension_props = try!(vkEnumerateDeviceExtensionProperties(physical_device, None));
    for extension in extension_props.into_iter() {
        let ext_name = unsafe{ CStr::from_ptr(extension.extensionName.as_ptr()) };
        match ext_name.to_str() {
            Ok(ext_name) => { exts.remove(ext_name); },
            Err(_) => {},
        };
    }
    debug!("checked physical device {:?}. missing extensions: {:?}", physical_device, exts);
    return Ok(exts.is_empty());
}

fn choose_surface_format(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR) -> VkResultObj<VkSurfaceFormatKHR> {
    if surface == vk_null_handle() {
        return Err(VK_ERROR_EXTENSION_NOT_PRESENT.into());
    }
    let available_formats = try!(vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface));
    if available_formats.len() <= 0 {
        return Err(VK_ERROR_EXTENSION_NOT_PRESENT.into());
    }
    let first_format = VkSurfaceFormatKHR{ format: available_formats[0].format, colorSpace: available_formats[0].colorSpace };
    if available_formats.len() == 1 && first_format.format == VK_FORMAT_UNDEFINED {
        return Ok(VkSurfaceFormatKHR{
            format: VK_FORMAT_B8G8R8A8_UNORM,
            colorSpace: VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        });
    }

    for available_format in available_formats {
        if available_format.format == VK_FORMAT_B8G8R8A8_UNORM && available_format.colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR {
            return Ok(available_format);
        }
    }

    return Ok(first_format);
}

fn choose_surface_present_mode(physical_device: VkPhysicalDevice, surface: VkSurfaceKHR) -> VkResultObj<VkPresentModeKHR> {
    if surface == vk_null_handle() {
        return Err(VK_ERROR_EXTENSION_NOT_PRESENT.into());
    }
    let available_modes = try!(vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, surface));
    let mut best_mode = VK_PRESENT_MODE_FIFO_KHR;
    for available_mode in available_modes {
        if available_mode == VK_PRESENT_MODE_MAILBOX_KHR {
            return Ok(available_mode);
        } else if available_mode == VK_PRESENT_MODE_IMMEDIATE_KHR {
            best_mode = available_mode;
        }
    }
    return Ok(best_mode);
}

fn choose_swap_extend_from_capabilities(capabilities: &VkSurfaceCapabilitiesKHR, window_size: VkExtent2D) -> VkExtent2D {
    if capabilities.currentExtent.width != (-1i32) as u32 {
        return VkExtent2D{ width: capabilities.currentExtent.width, height: capabilities.currentExtent.height };
    }
    let mut actual_extent = window_size;
    if actual_extent.width > capabilities.maxImageExtent.width {
        actual_extent.width = capabilities.maxImageExtent.width;
    } else if actual_extent.width < capabilities.minImageExtent.width {
        actual_extent.width = capabilities.minImageExtent.width;
    }
    if actual_extent.height > capabilities.maxImageExtent.height {
        actual_extent.height = capabilities.maxImageExtent.height;
    } else if actual_extent.height < capabilities.minImageExtent.height {
        actual_extent.height = capabilities.minImageExtent.height;
    }
    return actual_extent;
}

fn create_logical_device(physical_device: VkPhysicalDevice, queue_family_indices: &[u32], exts: &[&str]) -> VkResultObj<VkDevice> {
    let queue_family_indices : ::std::collections::HashSet<u32> = queue_family_indices.iter().cloned().collect(); // make unique
    let exts: Vec<CString> = exts.iter().map(|s| CString::new(*s).unwrap()).collect();
    let exts_p: Vec<*const c_char> = exts.iter().map(|s| s.as_ptr()).collect();

    let queue_priorities = [0.0f32];
    let queue_create_info : Vec<VkDeviceQueueCreateInfo> = queue_family_indices.iter()
        .map(|family_index| VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: vk_null(),
            flags: 0,
            queueCount: queue_priorities.len() as u32,
            pQueuePriorities: queue_priorities.as_ptr(),
            queueFamilyIndex: *family_index,
        }).collect();
    let device_create_info = VkDeviceCreateInfo{
        sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        queueCreateInfoCount: queue_create_info.len() as u32,
        pQueueCreateInfos: queue_create_info.as_ptr(),
        enabledLayerCount: 0,
        ppEnabledLayerNames: vk_null(),
        enabledExtensionCount: exts_p.len() as u32,
        ppEnabledExtensionNames: exts_p.as_ptr(),
        pEnabledFeatures: vk_null(),
    };
    let device = try!(vkCreateDevice(physical_device, &device_create_info, None));
    debug!("created device {:?}", device);
    return Ok(device);
}

fn create_swapchain(device: VkDevice, surface: VkSurfaceKHR, details: &DeviceInitializationDetails, extent: VkExtent2D, old_swapchain: VkSwapchainKHR) -> VkResultObj<VkSwapchainKHR> {
    let mut image_count = details.surface_capabilities.minImageCount + 1;
    if details.surface_capabilities.maxImageCount > 0 && image_count > details.surface_capabilities.maxImageCount {
        image_count = details.surface_capabilities.maxImageCount;
    }
    let mut create_info = VkSwapchainCreateInfoKHR{
        sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
        pNext: vk_null(),
        flags: 0,
        surface: surface,
        minImageCount: image_count,
        imageFormat: details.surface_format.format,
        imageColorSpace: details.surface_format.colorSpace,
        imageExtent: extent,
        imageArrayLayers: 1,
        imageUsage: VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
        imageSharingMode: VK_SHARING_MODE_EXCLUSIVE,
        queueFamilyIndexCount: 0,
        pQueueFamilyIndices: vk_null(),
        preTransform: details.surface_capabilities.currentTransform,
        compositeAlpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
        presentMode: details.surface_present_mode,
        clipped: VK_TRUE,
        oldSwapchain: old_swapchain,
    };
    if details.queue_family_indices[0] != details.queue_family_indices[1] {
        create_info.imageSharingMode = VK_SHARING_MODE_CONCURRENT;
        create_info.queueFamilyIndexCount = 2;
        create_info.pQueueFamilyIndices = details.queue_family_indices.as_ptr();
    }
    let swapchain = try!(vkCreateSwapchainKHR(device, &create_info, None));
    debug!("created swapchain {:?}", swapchain);
    return Ok(swapchain);
}

fn create_swapchain_image_views(device: VkDevice, details: &DeviceInitializationDetails, images: &[VkImage]) -> VkResultObj<Vec<VkImageView>> {
    let mut image_views : Vec<VkImageView> = Vec::with_capacity(images.len());
    for image in images {
        let create_info = VkImageViewCreateInfo{
            sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            pNext: vk_null(),
            flags: 0,
            image: *image,
            viewType: VK_IMAGE_VIEW_TYPE_2D,
            format: details.surface_format.format,
            components: VkComponentMapping{
                r: VK_COMPONENT_SWIZZLE_IDENTITY,
                g: VK_COMPONENT_SWIZZLE_IDENTITY,
                b: VK_COMPONENT_SWIZZLE_IDENTITY,
                a: VK_COMPONENT_SWIZZLE_IDENTITY,
            },
            subresourceRange: VkImageSubresourceRange{
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
        };
        image_views.push(try!(vkCreateImageView(device, &create_info, None)));
    }
    return Ok(image_views);
}

impl Application {
    pub fn new(app_name: &str, window: &winit::Window) -> VkResultObj<Application> {
        debug!("initializingApplication...");
        let mut app : Application = Default::default();
        let instance_exts = get_required_instance_extensions(window);
        let device_exts = get_required_device_extensions();
        app.instance = try!(create_instance(app_name, instance_exts.as_slice()));
        app.surface = try!(create_surface(app.instance, window));
        let (physical_device, details) = try!(choose_physical_device(app.instance, device_exts.as_slice(), app.surface));
        app.physical_device = physical_device;
        //let mem_props = vk_get!(vkGetPhysicalDeviceMemoryProperties; app.physical_device; VkPhysicalDeviceMemoryProperties);
        //let dev_props = vk_get!(vkGetPhysicalDeviceProperties; app.physical_device; VkPhysicalDeviceProperties);
        //debug!("props: memoryTypeCount: {}, memoryHeapCount: {}, apiVersion: {}, driverVersion: {}, vendorID: {}, deviceID: {}", mem_props.memoryTypeCount, mem_props.memoryHeapCount, dev_props.apiVersion, dev_props.driverVersion, dev_props.vendorID, dev_props.deviceID);

        app.device = try!(create_logical_device(app.physical_device, &details.queue_family_indices, device_exts.as_slice()));
        app.queues = get_device_queues(app.device, &details.queue_family_indices);
        let window_size = match window.get_inner_size_pixels() {
            Some((width, height)) => VkExtent2D{width: width, height: height},
            None => VkExtent2D{width: 800, height: 600},
        };
        let extent = choose_swap_extend_from_capabilities(&details.surface_capabilities, window_size);
        app.swapchain = try!(create_swapchain(app.device, app.surface, &details, extent, vk_null_handle()));
        app.swapchain_images = try!(vkGetSwapchainImagesKHR(app.device, app.swapchain));
        app.swapchain_image_views = try!(create_swapchain_image_views(app.device, &details, &app.swapchain_images));
        debug!("Application initialized");
        return Ok(app);
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        debug!("Cleaning up Application...");
        for image_view in self.swapchain_image_views.iter_mut() {
            vk_drop!(vkDestroyImageView; [self.device] *image_view, None);
        }
        self.swapchain_image_views.clear();
        vk_drop!(vkDestroySwapchainKHR; [self.device] self.swapchain, None);
        vk_drop!(vkDestroyDevice; [] self.device, None);
        vk_drop!(vkDestroySurfaceKHR; [self.instance] self.surface, None);
        vk_drop!(vkDestroyInstance; [] self.instance, None);
        self.swapchain_images.clear();
        self.queues = [vk_null_handle(), vk_null_handle()];
        self.physical_device = vk_null_handle();
        debug!("Application cleaned up");
    }
}
