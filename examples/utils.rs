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
use vulkan_rs::prelude::vk_version_1_0::*;
use vulkan_rs::prelude::vk_khr_surface::*;
use vulkan_rs::prelude::vk_khr_swapchain::*;

#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub use vulkan_rs::prelude::vk_khr_xlib_surface::*;

#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub use vulkan_rs::prelude::vk_khr_xcb_surface::*;

#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub use vulkan_rs::prelude::vk_khr_wayland_surface::*;

#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub use vulkan_rs::prelude::vk_khr_mir_surface::*;

#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub use vulkan_rs::prelude::vk_khr_android_surface::*;

#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub use vulkan_rs::prelude::vk_khr_win32_surface::*;

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
    render_pass: VkRenderPass,
    pipeline_layout: VkPipelineLayout,
    pipeline: VkPipeline,
    swapchain_framebuffers: Vec<VkFramebuffer>,
    command_pool: VkCommandPool,
    command_buffer: VkCommandBuffer,
    semaphore_image_available: VkSemaphore,
    semaphore_render_finished: VkSemaphore,
    extent: VkExtent2D,
    image_index: ::std::cell::Cell<u32>,
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
    let surface = vkCreateWin32SurfaceKHR(instance, &create_info, None)?;
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
        let surface = vkCreateWaylandSurfaceKHR(instance, &create_info, None)?;
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
        let surface = vkCreateXlibSurfaceKHR(instance, &create_info, None)?;
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
        let surface = vkCreateXcbSurfaceKHR(instance, &create_info, None)?;
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
    let surface = vkCreateAndroidSurfaceKHR(instance, &create_info, None)?;
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
                vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, i as u32, surface)?
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
    let instance = vkCreateInstance(&create_info, None)?;
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
    let devices = vkEnumeratePhysicalDevices(instance)?;
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
    let indices = choose_queue_family_indices(physical_device, surface)?;
    if indices[0] == INVALID_INDEX || (surface != vk_null_handle() && indices[1] == INVALID_INDEX) {
        debug!("device {:?} has no suitable queue family", physical_device);
        return Err(VK_ERROR_INITIALIZATION_FAILED.into());
    }
    let mut details : DeviceInitializationDetails = Default::default();
    details.queue_family_indices = indices;
    if ! check_device_extension_support(physical_device, exts)? {
        debug!("device {:?} doesn't support all required extensions", physical_device);
        return Err(VK_ERROR_INITIALIZATION_FAILED.into());
    }
    if surface != vk_null_handle() {
        details.surface_capabilities = vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, surface)?;
        details.surface_format = choose_surface_format(physical_device, surface)?;
        details.surface_present_mode = choose_surface_present_mode(physical_device, surface)?;
    }
    return Ok(details);
}

fn check_device_extension_support(physical_device: VkPhysicalDevice, exts: &[&str]) -> VkResultObj<bool> {
    let mut exts : ::std::collections::HashSet<&str> = exts.iter().cloned().collect();
    let extension_props = vkEnumerateDeviceExtensionProperties(physical_device, None)?;
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
    let available_formats = vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface)?;
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
    let available_modes = vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, surface)?;
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
    let device = vkCreateDevice(physical_device, &device_create_info, None)?;
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
        imageUsage: VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT.flags(),
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
    let swapchain = vkCreateSwapchainKHR(device, &create_info, None)?;
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
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.flags(),
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
        };
        image_views.push(vkCreateImageView(device, &create_info, None)?);
    }
    debug!("created {} image views", image_views.len());
    return Ok(image_views);
}

fn create_shader_module(device: VkDevice, data: &[u8]) -> VkResultObj<VkShaderModule> {
    let create_info = VkShaderModuleCreateInfo{
        sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        codeSize: data.len(),
        pCode: data.as_ptr() as *const u32,
    };
    let shader_module = vkCreateShaderModule(device, &create_info, None)?;
    debug!("created shader module {:?}", shader_module);
    return Ok(shader_module);
}

fn create_render_pass(device: VkDevice, format: VkFormat) -> VkResultObj<VkRenderPass> {
    let color_attachments = [
        VkAttachmentDescription{
            flags: 0,
            format: format,
            samples: VK_SAMPLE_COUNT_1_BIT,
            loadOp: VK_ATTACHMENT_LOAD_OP_CLEAR,
            storeOp: VK_ATTACHMENT_STORE_OP_STORE,
            stencilLoadOp: VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: VK_ATTACHMENT_STORE_OP_DONT_CARE,
            initialLayout: VK_IMAGE_LAYOUT_UNDEFINED,
            finalLayout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        },
    ];
    let color_attachment_refs1 = [
        VkAttachmentReference{
            attachment: 0,
            layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        },
    ];
    let subpasses = [
        VkSubpassDescription{
            flags: 0,
            pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
            inputAttachmentCount: 0,
            pInputAttachments: vk_null(),
            colorAttachmentCount: color_attachment_refs1.len() as u32,
            pColorAttachments: color_attachment_refs1.as_ptr(),
            pResolveAttachments: vk_null(),
            pDepthStencilAttachment: vk_null(),
            preserveAttachmentCount: 0,
            pPreserveAttachments: vk_null(),
        },
    ];
    let create_info = VkRenderPassCreateInfo{
        sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        attachmentCount: color_attachments.len() as u32,
        pAttachments: color_attachments.as_ptr(),
        subpassCount: subpasses.len() as u32,
        pSubpasses: subpasses.as_ptr(),
        dependencyCount: 0,
        pDependencies: vk_null(),
    };
    let render_pass = vkCreateRenderPass(device, &create_info, None)?;
    debug!("created render pass {:?}", render_pass);
    return Ok(render_pass);
}

fn create_pipeline_layout(device: VkDevice) -> VkResultObj<VkPipelineLayout> {
    let create_info = VkPipelineLayoutCreateInfo{
        sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        setLayoutCount: 0,
        pSetLayouts: vk_null(),
        pushConstantRangeCount: 0,
        pPushConstantRanges: vk_null(),
    };
    let pipeline_layout = vkCreatePipelineLayout(device, &create_info, None)?;
    debug!("created pipeline layout {:?}", pipeline_layout);
    return Ok(pipeline_layout);
}

fn create_graphics_pipeline(device: VkDevice, layout: VkPipelineLayout, render_pass: VkRenderPass, extent: VkExtent2D) -> VkResultObj<VkPipeline> {
    let vert_shader_module = try!(create_shader_module(device,
        include_bytes!(concat!(env!("OUT_DIR"), "/shader.vert.spv"))
    ));
    let frag_shader_module = try!(create_shader_module(device,
        include_bytes!(concat!(env!("OUT_DIR"), "/shader.frag.spv"))
    ));
    let shader_stage_create_infos = [
        VkPipelineShaderStageCreateInfo{
            sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: vk_null(),
            flags: 0,
            stage: VK_SHADER_STAGE_VERTEX_BIT,
            module: vert_shader_module,
            pName: "main\0".as_ptr() as *const c_char,
            pSpecializationInfo: vk_null(),
        },
        VkPipelineShaderStageCreateInfo{
            sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: vk_null(),
            flags: 0,
            stage: VK_SHADER_STAGE_FRAGMENT_BIT,
            module: frag_shader_module,
            pName: "main\0".as_ptr() as *const c_char,
            pSpecializationInfo: vk_null(),
        },
    ];
    let vertex_input_create_info = VkPipelineVertexInputStateCreateInfo{
        sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        vertexBindingDescriptionCount: 0,
        pVertexBindingDescriptions: vk_null(),
        vertexAttributeDescriptionCount: 0,
        pVertexAttributeDescriptions: vk_null(),
    };
    let input_asselbly_create_info = VkPipelineInputAssemblyStateCreateInfo{
        sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        topology: VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        primitiveRestartEnable: VK_FALSE,
    };
    let viewport_state_create_info = VkPipelineViewportStateCreateInfo{
        sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        viewportCount: 1,
        pViewports: &VkViewport{
            x: 0.0, y: 0.0,
            width: extent.width as f32, height: extent.height as f32,
            minDepth: 0.0, maxDepth: 1.0,
        },
        scissorCount: 1,
        pScissors: &VkRect2D{
            offset: VkOffset2D{x: 0, y: 0},
            extent: extent,
        },
    };
    let rasterizer_create_info = VkPipelineRasterizationStateCreateInfo{
        sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        depthClampEnable: VK_FALSE,
        rasterizerDiscardEnable: VK_FALSE,
        polygonMode: VK_POLYGON_MODE_FILL,
        cullMode: VK_CULL_MODE_BACK_BIT.flags(),
        frontFace: VK_FRONT_FACE_CLOCKWISE,
        depthBiasEnable: VK_FALSE,
        depthBiasConstantFactor: 0.0,
        depthBiasClamp: 0.0,
        depthBiasSlopeFactor: 0.0,
        lineWidth: 1.0,
    };
    let multisample_create_info = VkPipelineMultisampleStateCreateInfo{
        sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        rasterizationSamples: VK_SAMPLE_COUNT_1_BIT,
        sampleShadingEnable: VK_FALSE,
        minSampleShading: 0.0,
        pSampleMask: vk_null(),
        alphaToCoverageEnable: VK_FALSE,
        alphaToOneEnable: VK_FALSE,
    };
    let color_blend_attachements = [
        VkPipelineColorBlendAttachmentState{
            blendEnable: VK_FALSE,
            srcColorBlendFactor: VK_BLEND_FACTOR_ONE,
            dstColorBlendFactor: VK_BLEND_FACTOR_ZERO,
            colorBlendOp: VK_BLEND_OP_ADD,
            srcAlphaBlendFactor: VK_BLEND_FACTOR_ONE,
            dstAlphaBlendFactor: VK_BLEND_FACTOR_ZERO,
            alphaBlendOp: VK_BLEND_OP_ADD,
            colorWriteMask: VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT,
        },
    ];
    let color_blending_create_info = VkPipelineColorBlendStateCreateInfo{
        sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        logicOpEnable: VK_FALSE,
        logicOp: VK_LOGIC_OP_COPY,
        attachmentCount: color_blend_attachements.len() as u32,
        pAttachments: color_blend_attachements.as_ptr(),
        blendConstants: [0.0, 0.0, 0.0, 0.0],
    };
    let depth_stencli_create_info = VkPipelineDepthStencilStateCreateInfo{
        sType: VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        depthTestEnable: VK_FALSE,
        depthWriteEnable: VK_FALSE,
        depthCompareOp: VK_COMPARE_OP_LESS_OR_EQUAL,
        depthBoundsTestEnable: VK_FALSE,
        stencilTestEnable: VK_FALSE,
        front: VkStencilOpState{
            failOp: VK_STENCIL_OP_KEEP,
            passOp: VK_STENCIL_OP_KEEP,
            depthFailOp: VK_STENCIL_OP_KEEP,
            compareOp: VK_COMPARE_OP_ALWAYS,
            compareMask: 0,
            writeMask: 0,
            reference: 0,
        },
        back: VkStencilOpState{
            failOp: VK_STENCIL_OP_KEEP,
            passOp: VK_STENCIL_OP_KEEP,
            depthFailOp: VK_STENCIL_OP_KEEP,
            compareOp: VK_COMPARE_OP_ALWAYS,
            compareMask: 0,
            writeMask: 0,
            reference: 0,
        },
        minDepthBounds: 0.0,
        maxDepthBounds: 0.0,
    };
    let create_infos = [
        VkGraphicsPipelineCreateInfo{
            sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            pNext: vk_null(),
            flags: 0,
            stageCount: shader_stage_create_infos.len() as u32,
            pStages: shader_stage_create_infos.as_ptr(),
            pVertexInputState: &vertex_input_create_info,
            pInputAssemblyState: &input_asselbly_create_info,
            pTessellationState: vk_null(),
            pViewportState: &viewport_state_create_info,
            pRasterizationState: &rasterizer_create_info,
            pMultisampleState: &multisample_create_info,
            pDepthStencilState: &depth_stencli_create_info,
            pColorBlendState: &color_blending_create_info,
            pDynamicState: vk_null(),
            layout: layout,
            renderPass: render_pass,
            subpass: 0,
            basePipelineHandle: vk_null_handle(),
            basePipelineIndex: -1,
        },
    ];
    debug!("create graphics pipeline...");
    let res = vkCreateGraphicsPipelines(device, vk_null_handle(), &create_infos, None);
    debug!("destroy shader modules...");
    vkDestroyShaderModule(device, frag_shader_module, None);
    vkDestroyShaderModule(device, vert_shader_module, None);
    match res {
        Ok(p) => {
            let pipeline = p[0];
            debug!("created graphics pipeline {:?}", pipeline);
            Ok(pipeline)
        },
        Err(e) => Err(e),
    }
}

fn create_swapchain_framebuffers(device: VkDevice, render_pass: VkRenderPass, extent: VkExtent2D, image_views: &[VkImageView]) -> VkResultObj<Vec<VkFramebuffer>> {
    let mut framebuffers : Vec<VkFramebuffer> = Vec::with_capacity(image_views.len());
    for image_view in image_views {
        let attachments = [*image_view];
        let create_info = VkFramebufferCreateInfo{
            sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            pNext: vk_null(),
            flags: 0,
            renderPass: render_pass,
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            width: extent.width,
            height: extent.height,
            layers: 1,
        };
        framebuffers.push(vkCreateFramebuffer(device, &create_info, None)?);
    }
    debug!("created {} framebuffers", framebuffers.len());
    return Ok(framebuffers);
}

fn create_command_pool(device: VkDevice, graphics_family: u32) -> VkResultObj<VkCommandPool> {
    let create_info = VkCommandPoolCreateInfo{
        sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        pNext: vk_null(),
        flags: VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT|VK_COMMAND_POOL_CREATE_TRANSIENT_BIT,
        queueFamilyIndex: graphics_family,
    };
    let command_pool = vkCreateCommandPool(device, &create_info, None)?;
    debug!("created command pool {:?}", command_pool);
    return Ok(command_pool);
}

fn init_command_buffer(device: VkDevice, command_pool: VkCommandPool) -> VkResultObj<VkCommandBuffer> {
    let create_info = VkCommandBufferAllocateInfo{
        sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        pNext: vk_null(),
        commandPool: command_pool,
        level: VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        commandBufferCount: 1,
    };
    let commabd_buffer = vkAllocateCommandBuffers(device, &create_info)?;
    debug!("created command buffer {:?}", commabd_buffer);
    return Ok(commabd_buffer[0]);
}

fn create_semaphore(device: VkDevice) -> VkResultObj<VkSemaphore> {
    let create_info = VkSemaphoreCreateInfo{
        sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
    };
    let semaphore = vkCreateSemaphore(device, &create_info, None)?;
    debug!("created semaphore {:?}", semaphore);
    return Ok(semaphore);
}

impl Application {
    pub fn new(app_name: &str, window: &winit::Window) -> VkResultObj<Application> {
        debug!("initializingApplication...");
        let mut app : Application = Default::default();
        let instance_exts = get_required_instance_extensions(window);
        let device_exts = get_required_device_extensions();
        app.instance = create_instance(app_name, instance_exts.as_slice())?;
        app.surface = create_surface(app.instance, window)?;
        let (physical_device, details) = choose_physical_device(app.instance, device_exts.as_slice(), app.surface)?;
        app.physical_device = physical_device;
        //let mem_props = vk_get!(vkGetPhysicalDeviceMemoryProperties; app.physical_device; VkPhysicalDeviceMemoryProperties);
        //let dev_props = vk_get!(vkGetPhysicalDeviceProperties; app.physical_device; VkPhysicalDeviceProperties);
        //debug!("props: memoryTypeCount: {}, memoryHeapCount: {}, apiVersion: {}, driverVersion: {}, vendorID: {}, deviceID: {}", mem_props.memoryTypeCount, mem_props.memoryHeapCount, dev_props.apiVersion, dev_props.driverVersion, dev_props.vendorID, dev_props.deviceID);

        app.device = create_logical_device(app.physical_device, &details.queue_family_indices, device_exts.as_slice())?;
        app.queues = get_device_queues(app.device, &details.queue_family_indices);
        let window_size = match window.get_inner_size_pixels() {
            Some((width, height)) => VkExtent2D{width: width, height: height},
            None => VkExtent2D{width: 800, height: 600},
        };
        app.extent = choose_swap_extend_from_capabilities(&details.surface_capabilities, window_size);
        app.swapchain = create_swapchain(app.device, app.surface, &details, app.extent, vk_null_handle())?;
        app.swapchain_images = vkGetSwapchainImagesKHR(app.device, app.swapchain)?;
        app.swapchain_image_views = create_swapchain_image_views(app.device, &details, &app.swapchain_images)?;

        app.render_pass = create_render_pass(app.device, details.surface_format.format)?;
        app.pipeline_layout = create_pipeline_layout(app.device)?;
        app.pipeline = create_graphics_pipeline(app.device, app.pipeline_layout, app.render_pass, app.extent)?;

        app.swapchain_framebuffers = create_swapchain_framebuffers(app.device, app.render_pass, app.extent, &app.swapchain_image_views)?;

        app.command_pool = create_command_pool(app.device, details.queue_family_indices[GRAPHIC_QUEUE])?;
        app.command_buffer = init_command_buffer(app.device, app.command_pool)?;

        app.semaphore_image_available = create_semaphore(app.device)?;
        app.semaphore_render_finished = create_semaphore(app.device)?;

        debug!("Application initialized");
        return Ok(app);
    }

    pub fn begin(&self) -> VkResultObj<()> {

        let (image_index, _) = vkAcquireNextImageKHR(self.device, self.swapchain, u64::max_value(), self.semaphore_image_available, vk_null_handle())?;
        self.image_index.set(image_index);

        vkBeginCommandBuffer(self.command_buffer, &VkCommandBufferBeginInfo{
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: vk_null(),
            flags: 0,
            pInheritanceInfo: vk_null(),
        })?;

        vkCmdBeginRenderPass(self.command_buffer, &VkRenderPassBeginInfo{
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            pNext: vk_null(),
            renderPass: self.render_pass,
            framebuffer: self.swapchain_framebuffers[image_index as usize],
            renderArea: VkRect2D{
                offset: VkOffset2D{x: 0, y: 0},
                extent: self.extent,
            },
            clearValueCount: 1,
            pClearValues: &VkClearValue::from_color(VkClearColorValue::from_float32([0.0, 0.0, 0.0, 1.0])),
        }, VK_SUBPASS_CONTENTS_INLINE);

        vkCmdBindPipeline(self.command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, self.pipeline);

        Ok(())
    }

    #[inline]
    pub fn get_command_buffer(&self) -> VkCommandBuffer {
        self.command_buffer
    }

    pub fn end(&self) -> VkResultObj<()> {
        vkCmdEndRenderPass(self.command_buffer);
        vkEndCommandBuffer(self.command_buffer)?;

        let wait_semaphores = [self.semaphore_image_available];
        let wait_stages = [VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT.flags()];
        let signal_semaphores = [self.semaphore_render_finished];

        let submit_info = [
            VkSubmitInfo{
                sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
                pNext: vk_null(),
                waitSemaphoreCount: wait_semaphores.len() as u32,
                pWaitSemaphores: wait_semaphores.as_ptr(),
                pWaitDstStageMask: wait_stages.as_ptr(),
                commandBufferCount: 1,
                pCommandBuffers: &self.command_buffer,
                signalSemaphoreCount: signal_semaphores.len() as u32,
                pSignalSemaphores: signal_semaphores.as_ptr(),
            }
        ];
        vkQueueSubmit(self.queues[GRAPHIC_QUEUE], &submit_info, vk_null_handle())?;

        let image_index = self.image_index.get();

        let present_info = VkPresentInfoKHR {
            sType: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
            pNext: vk_null(),
            waitSemaphoreCount: signal_semaphores.len() as u32,
            pWaitSemaphores: signal_semaphores.as_ptr(),
            swapchainCount: 1,
            pSwapchains: &self.swapchain,
            pImageIndices: &image_index,
            pResults: vk_null(),
        };
        let _ = vkQueuePresentKHR(self.queues[PRESENT_QUEUE], &present_info)?; // ignore VK_SUBOPTIMAL_KHR

        vkQueueWaitIdle(self.queues[PRESENT_QUEUE])?;

        Ok(())
    }

    pub fn wait_idle(&self) -> VkResultObj<()> {
        vkDeviceWaitIdle(self.device)
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        debug!("Cleaning up Application...");
        if !self.semaphore_render_finished.is_null() {
            vkDestroySemaphore(self.device, self.semaphore_render_finished, None);
            self.semaphore_render_finished = vk_null_handle();
        }
        if !self.semaphore_image_available.is_null() {
            vkDestroySemaphore(self.device, self.semaphore_image_available, None);
            self.semaphore_image_available = vk_null_handle();
        }
        if !self.command_pool.is_null() {
            vkDestroyCommandPool(self.device, self.command_pool, None);
            self.command_pool = vk_null_handle();
        }
        while let Some(framebuffer) = self.swapchain_framebuffers.pop() {
            if !framebuffer.is_null() {
                vkDestroyFramebuffer(self.device, framebuffer, None);
            }
        }
        if !self.pipeline.is_null() {
            vkDestroyPipeline(self.device, self.pipeline, None);
            self.pipeline = vk_null_handle();
        }
        if !self.pipeline_layout.is_null() {
            vkDestroyPipelineLayout(self.device, self.pipeline_layout, None);
            self.pipeline_layout = vk_null_handle();
        }
        if !self.render_pass.is_null() {
            vkDestroyRenderPass(self.device, self.render_pass, None);
            self.render_pass = vk_null_handle();
        }
        while let Some(image_view) = self.swapchain_image_views.pop() {
            if !image_view.is_null() {
                vkDestroyImageView(self.device, image_view, None);
            }
        }
        if !self.swapchain.is_null() {
            vkDestroySwapchainKHR(self.device, self.swapchain, None);
            self.swapchain = vk_null_handle();
        }
        if !self.device.is_null() {
            vkDestroyDevice(self.device, None);
            self.device = vk_null_handle();
        }
        if !self.surface.is_null() {
            vkDestroySurfaceKHR(self.instance, self.surface, None);
            self.surface = vk_null_handle();
        }
        if !self.instance.is_null() {
            vkDestroyInstance(self.instance, None);
            self.instance = vk_null_handle();
        }
        self.swapchain_images.clear();
        self.queues = [vk_null_handle(), vk_null_handle()];
        self.physical_device = vk_null_handle();
        debug!("Application cleaned up");
    }
}
