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

extern crate vulkan_rs;
extern crate winit;
use std::os::raw::c_char;
use std::ffi::CString;
pub use vulkan_rs::prelude::*;

const FENCE_TIMEOUT: u64 = 100000000u64;

#[derive(Default)]
pub struct SampleInfo {
    inst: VkInstance,
    surface: VkSurfaceKHR,
    gpus: Vec<VkPhysicalDevice>,
    device: VkDevice,
    graphics_queue: VkQueue,
    present_queue: VkQueue,
    graphics_queue_family_index: u32,
    present_queue_family_index: u32,
    gpu_props: VkPhysicalDeviceProperties,
    memory_properties: VkPhysicalDeviceMemoryProperties,
    queue_props: Vec<VkQueueFamilyProperties>,
    format: VkFormat,
    cmd_pool: VkCommandPool,
    cmd: VkCommandBuffer,
}

macro_rules! vk_try {
    ( $($e:expr );+ ) => {
        $(
            let res = $e ;
            if res != VK_SUCCESS {
                println!("e @ {} = {}\n{}", line!(), res, stringify!($e));
                return Err(res);
            }
        )+
    };
}

macro_rules! vk_unsafe {
    ( $($e:expr );+ ) => {
        $(
            vk_try!(unsafe { $e });
        )+
    };
}

#[cfg(target_os = "windows")]
fn platform_extensions(w: &winit::Window) -> Vec<&'static str> {
    return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                VK_KHR_WIN32_SURFACE_EXTENSION_NAME,
                VK_KHR_SWAPCHAIN_EXTENSION_NAME];
}

#[cfg(target_os = "linux")]
fn platform_extensions(w: &winit::Window) -> Vec<&'static str> {
    use winit::os::unix::WindowExt;
    if let Some(_) = (w as &WindowExt).get_wayland_display() {
        return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                    VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME,
                    VK_KHR_SWAPCHAIN_EXTENSION_NAME];
    } else if let Some(_) = (w as &WindowExt).get_xcb_connection() {
        return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                    VK_KHR_XCB_SURFACE_EXTENSION_NAME,
                    VK_KHR_SWAPCHAIN_EXTENSION_NAME];
    } else {
        return vec![VK_KHR_SURFACE_EXTENSION_NAME,
                    VK_KHR_SWAPCHAIN_EXTENSION_NAME];
    }
}

fn init_instance(info: &mut SampleInfo, exts: &[*const c_char]) -> VkResultObj<()> {
    let app_name = CString::new("triangles").unwrap();
    info.inst = try!(VkInstance::create(&VkInstanceCreateInfo {
                                             sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                                             pNext: vk_null(),
                                             flags: 0,
                                             pApplicationInfo:
                                                 &VkApplicationInfo {
                                                      sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
                                                      pNext: vk_null(),
                                                      pApplicationName: app_name.as_ptr(),
                                                      applicationVersion: 1,
                                                      pEngineName: app_name.as_ptr(),
                                                      engineVersion: 1,
                                                      apiVersion: VK_API_VERSION_1_0,
                                                  },
                                             enabledLayerCount: 0,
                                             ppEnabledLayerNames: vk_null(),
                                             enabledExtensionCount: exts.len() as u32,
                                             ppEnabledExtensionNames: exts.as_ptr(),
                                         }));
    return Ok(());
}

#[cfg(target_os = "windows")]
fn init_surface_wsi(info: &mutSampleInfo, w: &winit::Window) -> VkResultObj<()> {
    use kernel32;
    let create_info = VkWin32SurfaceCreateInfoKHR {
        sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
        pNext: vk_null(),
        flags: 0,
        hinstance: kernel32::GetModuleHandleW(ptr::null()),
        hwnd: w.get_hwnd(),
    };
    vk_unsafe!(vkCreateWin32SurfaceKHR(info.inst, create_info, vk_null(), &mut info.surface));
    return Ok(());
}

#[cfg(target_os = "linux")]
fn init_surface_wsi(info: &mut SampleInfo, w: &winit::Window) -> VkResultObj<()> {
    use winit::os::unix::WindowExt;
    if let Some(display) = (w as &WindowExt).get_wayland_display() {
        let create_info = VkWaylandSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
            pNext: vk_null(),
            flags: 0,
            display: display as *mut vk_platform::wayland::wl_display,
            surface: (w as &WindowExt).get_wayland_surface().unwrap() as
                     *mut vk_platform::wayland::wl_surface,
        };
        vk_unsafe!(vkCreateWaylandSurfaceKHR(info.inst,
                                             &create_info,
                                             vk_null(),
                                             &mut info.surface));
        return Ok(());
    } else if let Some(connection) = (w as &WindowExt).get_xcb_connection() {
        let create_info = VkXcbSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: vk_null(),
            flags: 0,
            connection: connection as *mut vk_platform::xcb::xcb_connection_t,
            window: (w as &WindowExt).get_xlib_window().unwrap() as vk_platform::xcb::xcb_window_t,
        };
        vk_unsafe!(vkCreateXcbSurfaceKHR(info.inst, &create_info, vk_null(), &mut info.surface));
        return Ok(());
    } else {
        return Err(VK_ERROR_EXTENSION_NOT_PRESENT);
    }
}


#[cfg(target_os = "android")]
fn init_surface_wsi(info: &mutSampleInfo, w: &winit::Window) -> VkResultObj<()> {
    use kernel32;
    let create_info = VkAndroidSurfaceCreateInfoKHR {
        sType: VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR,
        pNext: vk_null(),
        flags: 0,
        window: (w as &WindowExt).get_native_window() as *mut vk_platform::android::ANativeWindow,
    };
    vk_unsafe!(vkCreateAndroidSurfaceKHR(info.inst, create_info, vk_null(), &mut info.surface));
    return Ok(());
}

fn init_surface(info: &mut SampleInfo) -> VkResultObj<()> {
    let invalid_index = (-1i32) as u32;
    {
        // Iterate over each queue to learn whether it supports presenting:
        let queue_family_count = info.queue_props.len();
        let mut supports_present = Vec::<bool>::with_capacity(queue_family_count);
        for i in 0..queue_family_count {
            let mut val: VkBool32 = VK_FALSE;
            vk_unsafe!(vkGetPhysicalDeviceSurfaceSupportKHR(info.gpus[0],
                                                            i as u32,
                                                            info.surface,
                                                            &mut val));
            supports_present.push(val != VK_FALSE)
        }

        // Search for a graphics and a present queue in the array of queue
        // families, try to find one that supports both
        info.graphics_queue_family_index = invalid_index;
        info.present_queue_family_index = invalid_index;
        for i in 0..queue_family_count {
            if (info.queue_props[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) != 0 {
                if info.graphics_queue_family_index == invalid_index {
                    info.graphics_queue_family_index = i as u32;
                }
                if supports_present[i] {
                    info.graphics_queue_family_index = i as u32;
                    info.present_queue_family_index = i as u32;
                    break;
                }
            }
        }

        if info.present_queue_family_index == invalid_index {
            // If didn't find a queue that supports both graphics and present, then
            // find a separate present queue.
            for i in 0..queue_family_count {
                if supports_present[i] {
                    info.present_queue_family_index = i as u32;
                    break;
                }
            }
        }
    }
    {
        // Generate error if could not find queues that support graphics
        // and present
        if info.graphics_queue_family_index == invalid_index ||
           info.present_queue_family_index == invalid_index {
            panic!("Could not find a queues for both graphics and present");
        }

        // Get the list of VkFormats that are supported:
        let mut format_count: u32 = 0;
        vk_unsafe!(vkGetPhysicalDeviceSurfaceFormatsKHR(info.gpus[0],
                                                        info.surface,
                                                        &mut format_count,
                                                        vk_null()));
        let mut surf_formats = Vec::<VkSurfaceFormatKHR>::with_capacity(format_count as usize);
        vk_unsafe!(vkGetPhysicalDeviceSurfaceFormatsKHR(info.gpus[0],
                                                        info.surface,
                                                        &mut format_count,
                                                        surf_formats.as_mut_ptr()));
        unsafe { surf_formats.set_len(format_count as usize) };

        // If the format list includes just one entry of VK_FORMAT_UNDEFINED,
        // the surface has no preferred format.  Otherwise, at least one
        // supported format will be returned.
        if surf_formats.len() == 1 && surf_formats[0].format == VK_FORMAT_UNDEFINED {
            info.format = VK_FORMAT_B8G8R8A8_UNORM;
        } else {
            assert!(surf_formats.len() >= 1);
            info.format = surf_formats[0].format;
        }
    }
    return Ok(());
}

fn init_enumerate_device(info: &mut SampleInfo) -> VkResultObj<()> {
    info.gpus = try!(info.inst.get_physical_devices());
    if info.gpus.len() <= 0 {
        return Err(VK_ERROR_INITIALIZATION_FAILED);
    }
    let first_gpu = info.gpus[0];
    info.queue_props = first_gpu.get_query_family_properties();
    if info.queue_props.len() <= 0 {
        return Err(VK_ERROR_INITIALIZATION_FAILED);
    }
    first_gpu.get_memory_properties(&mut info.memory_properties);
    return Ok(());
}

fn init_device(info: &mut SampleInfo, exts: &[*const c_char]) -> VkResultObj<()> {
    let queue_priorities: [f32; 1] = [0.0];
    info.device = try!(info.gpus[0].create_device(&VkDeviceCreateInfo{
        sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
        queueCreateInfoCount: 1,
        pQueueCreateInfos: &VkDeviceQueueCreateInfo{
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: vk_null(),
            flags: 0,
            queueCount: queue_priorities.len() as u32,
            pQueuePriorities: queue_priorities.as_ptr(),
            queueFamilyIndex: info.graphics_queue_family_index,
        },
        enabledLayerCount: 0,
        ppEnabledLayerNames: vk_null(),
        enabledExtensionCount: exts.len() as u32,
        ppEnabledExtensionNames: exts.as_ptr(),
        pEnabledFeatures: vk_null(),
    }));
    return Ok(());
}

fn init_command_pool(info: &mut SampleInfo) -> VkResultObj<()> {
    let cmd_pool_info = VkCommandPoolCreateInfo {
        sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        pNext: vk_null(),
        queueFamilyIndex: info.graphics_queue_family_index,
        flags: VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
    };
    vk_unsafe!(vkCreateCommandPool(info.device, &cmd_pool_info, vk_null(), &mut info.cmd_pool));
    return Ok(());
}

fn init_command_buffer(info: &mut SampleInfo) -> VkResultObj<()> {
    let cmd_info = VkCommandBufferAllocateInfo {
        sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        pNext: vk_null(),
        commandPool: info.cmd_pool,
        level: VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        commandBufferCount: 1,
    };
    vk_unsafe!(vkAllocateCommandBuffers(info.device, &cmd_info, &mut info.cmd));
    return Ok(());
}

pub fn execute_begin_command_buffer(info: &SampleInfo) -> VkResultObj<()> {
    let cmd_buf_info = VkCommandBufferBeginInfo {
        sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        pNext: vk_null(),
        flags: 0,
        pInheritanceInfo: vk_null(),
    };
    vk_unsafe!(vkBeginCommandBuffer(info.cmd, &cmd_buf_info));
    return Ok(());
}

pub fn execute_end_command_buffer(info: &SampleInfo) -> VkResultObj<()> {
    vk_unsafe!(vkEndCommandBuffer(info.cmd));
    return Ok(());
}

pub fn execute_queue_command_buffer(info: &SampleInfo) -> VkResultObj<()> {

    /* Queue the command buffer for execution */
    let cmd_bufs: [VkCommandBuffer; 1] = [info.cmd];
    let mut drawFence: VkFence = vk_null_handle();
    let fenceInfo = VkFenceCreateInfo {
        sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
        pNext: vk_null(),
        flags: 0,
    };
    vk_unsafe!(vkCreateFence(info.device, &fenceInfo, vk_null(), &mut drawFence));
    let pipe_stage_flags: VkPipelineStageFlags = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
    let submit_infos: [VkSubmitInfo; 1] = [VkSubmitInfo {
                                               pNext: vk_null(),
                                               sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
                                               waitSemaphoreCount: 0,
                                               pWaitSemaphores: vk_null(),
                                               pWaitDstStageMask: &pipe_stage_flags,
                                               commandBufferCount: cmd_bufs.len() as u32,
                                               pCommandBuffers: cmd_bufs.as_ptr(),
                                               signalSemaphoreCount: 0,
                                               pSignalSemaphores: vk_null(),
                                           }];
    vk_unsafe!(vkQueueSubmit(info.graphics_queue,
                             submit_infos.len() as u32,
                             submit_infos.as_ptr(),
                             drawFence));

    {
        let mut res;
        loop {
            res = unsafe { vkWaitForFences(info.device, 1, &drawFence, VK_TRUE, FENCE_TIMEOUT) };
            if res != VK_TIMEOUT {
                break;
            }
        }
        vk_try!(res);
    }
    unsafe {
        vkDestroyFence(info.device, drawFence, vk_null());
    }
    return Ok(());
}

fn init_device_queue(info: &mut SampleInfo) -> VkResultObj<()> {
    unsafe {
        vkGetDeviceQueue(info.device,
                         info.graphics_queue_family_index,
                         0,
                         &mut info.graphics_queue);
    }
    if info.graphics_queue_family_index == info.present_queue_family_index {
        info.present_queue = info.graphics_queue;
    } else {
        unsafe {
            vkGetDeviceQueue(info.device,
                             info.present_queue_family_index,
                             0,
                             &mut info.present_queue);
        }
    }
    return Ok(());
}

fn init_swap_chain(info: &mut SampleInfo) -> VkResultObj<()> {
    // TODO: implementat
    /*

    VkResult U_ASSERT_ONLY res;
    VkSurfaceCapabilitiesKHR surfCapabilities;

    res = vkGetPhysicalDeviceSurfaceCapabilitiesKHR(info.gpus[0], info.surface,
                                                    &surfCapabilities);
    assert(res == VK_SUCCESS);

    uint32_t presentModeCount;
    res = vkGetPhysicalDeviceSurfacePresentModesKHR(info.gpus[0], info.surface,
                                                    &presentModeCount, NULL);
    assert(res == VK_SUCCESS);
    VkPresentModeKHR *presentModes =
        (VkPresentModeKHR *)malloc(presentModeCount * sizeof(VkPresentModeKHR));
    assert(presentModes);
    res = vkGetPhysicalDeviceSurfacePresentModesKHR(
        info.gpus[0], info.surface, &presentModeCount, presentModes);
    assert(res == VK_SUCCESS);

    VkExtent2D swapchainExtent;
    // width and height are either both 0xFFFFFFFF, or both not 0xFFFFFFFF.
    if (surfCapabilities.currentExtent.width == 0xFFFFFFFF) {
        // If the surface size is undefined, the size is set to
        // the size of the images requested.
        swapchainExtent.width = info.width;
        swapchainExtent.height = info.height;
        if (swapchainExtent.width < surfCapabilities.minImageExtent.width) {
            swapchainExtent.width = surfCapabilities.minImageExtent.width;
        } else if (swapchainExtent.width >
                   surfCapabilities.maxImageExtent.width) {
            swapchainExtent.width = surfCapabilities.maxImageExtent.width;
        }

        if (swapchainExtent.height < surfCapabilities.minImageExtent.height) {
            swapchainExtent.height = surfCapabilities.minImageExtent.height;
        } else if (swapchainExtent.height >
                   surfCapabilities.maxImageExtent.height) {
            swapchainExtent.height = surfCapabilities.maxImageExtent.height;
        }
    } else {
        // If the surface size is defined, the swap chain size must match
        swapchainExtent = surfCapabilities.currentExtent;
    }

    // The FIFO present mode is guaranteed by the spec to be supported
    // Also note that current Android driver only supports FIFO
    VkPresentModeKHR swapchainPresentMode = VK_PRESENT_MODE_FIFO_KHR;

    // Determine the number of VkImage's to use in the swap chain.
    // We need to acquire only 1 presentable image at at time.
    // Asking for minImageCount images ensures that we can acquire
    // 1 presentable image as long as we present it before attempting
    // to acquire another.
    uint32_t desiredNumberOfSwapChainImages = surfCapabilities.minImageCount;

    VkSurfaceTransformFlagBitsKHR preTransform;
    if (surfCapabilities.supportedTransforms &
        VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR) {
        preTransform = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
    } else {
        preTransform = surfCapabilities.currentTransform;
    }

    VkSwapchainCreateInfoKHR swapchain_ci = {};
    swapchain_ci.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    swapchain_ci.pNext = NULL;
    swapchain_ci.surface = info.surface;
    swapchain_ci.minImageCount = desiredNumberOfSwapChainImages;
    swapchain_ci.imageFormat = info.format;
    swapchain_ci.imageExtent.width = swapchainExtent.width;
    swapchain_ci.imageExtent.height = swapchainExtent.height;
    swapchain_ci.preTransform = preTransform;
    swapchain_ci.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
    swapchain_ci.imageArrayLayers = 1;
    swapchain_ci.presentMode = swapchainPresentMode;
    swapchain_ci.oldSwapchain = VK_NULL_HANDLE;
#ifndef __ANDROID__
    swapchain_ci.clipped = true;
#else
    swap_chain.clipped = false;
#endif
    swapchain_ci.imageColorSpace = VK_COLORSPACE_SRGB_NONLINEAR_KHR;
    swapchain_ci.imageUsage = usageFlags;
    swapchain_ci.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
    swapchain_ci.queueFamilyIndexCount = 0;
    swapchain_ci.pQueueFamilyIndices = NULL;
    uint32_t queueFamilyIndices[2] = {
        (uint32_t)info.graphics_queue_family_index,
        (uint32_t)info.present_queue_family_index};
    if (info.graphics_queue_family_index != info.present_queue_family_index) {
        // If the graphics and present queues are from different queue families,
        // we either have to explicitly transfer ownership of images between the
        // queues, or we have to create the swapchain with imageSharingMode
        // as VK_SHARING_MODE_CONCURRENT
        swapchain_ci.imageSharingMode = VK_SHARING_MODE_CONCURRENT;
        swapchain_ci.queueFamilyIndexCount = 2;
        swapchain_ci.pQueueFamilyIndices = queueFamilyIndices;
    }

    res = vkCreateSwapchainKHR(info.device, &swapchain_ci, NULL,
                               &info.swap_chain);
    assert(res == VK_SUCCESS);

    res = vkGetSwapchainImagesKHR(info.device, info.swap_chain,
                                  &info.swapchainImageCount, NULL);
    assert(res == VK_SUCCESS);

    VkImage *swapchainImages =
        (VkImage *)malloc(info.swapchainImageCount * sizeof(VkImage));
    assert(swapchainImages);
    res = vkGetSwapchainImagesKHR(info.device, info.swap_chain,
                                  &info.swapchainImageCount, swapchainImages);
    assert(res == VK_SUCCESS);

    for (uint32_t i = 0; i < info.swapchainImageCount; i++) {
        swap_chain_buffer sc_buffer;

        VkImageViewCreateInfo color_image_view = {};
        color_image_view.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        color_image_view.pNext = NULL;
        color_image_view.format = info.format;
        color_image_view.components.r = VK_COMPONENT_SWIZZLE_R;
        color_image_view.components.g = VK_COMPONENT_SWIZZLE_G;
        color_image_view.components.b = VK_COMPONENT_SWIZZLE_B;
        color_image_view.components.a = VK_COMPONENT_SWIZZLE_A;
        color_image_view.subresourceRange.aspectMask =
            VK_IMAGE_ASPECT_COLOR_BIT;
        color_image_view.subresourceRange.baseMipLevel = 0;
        color_image_view.subresourceRange.levelCount = 1;
        color_image_view.subresourceRange.baseArrayLayer = 0;
        color_image_view.subresourceRange.layerCount = 1;
        color_image_view.viewType = VK_IMAGE_VIEW_TYPE_2D;
        color_image_view.flags = 0;

        sc_buffer.image = swapchainImages[i];

        color_image_view.image = sc_buffer.image;

        res = vkCreateImageView(info.device, &color_image_view, NULL,
                                &sc_buffer.view);
        info.buffers.push_back(sc_buffer);
        assert(res == VK_SUCCESS);
    }
    free(swapchainImages);
    info.current_buffer = 0;

    if (NULL != presentModes) {
        free(presentModes);
    }
    */

    return Ok(());
}

fn init_depth_buffer(info: &mut SampleInfo) -> VkResultObj<()> {
    // TODO: implement
    /*
    VkResult U_ASSERT_ONLY res;
    bool U_ASSERT_ONLY pass;
    VkImageCreateInfo image_info = {};

    /* allow custom depth formats */
    if (info.depth.format == VK_FORMAT_UNDEFINED)
    info.depth.format = VK_FORMAT_D16_UNORM;

    #ifdef __ANDROID__
    // Depth format needs to be VK_FORMAT_D24_UNORM_S8_UINT on Android.
    const VkFormat depth_format = VK_FORMAT_D24_UNORM_S8_UINT;
    #else
    const VkFormat depth_format = info.depth.format;
    #endif
    VkFormatProperties props;
    vkGetPhysicalDeviceFormatProperties(info.gpus[0], depth_format, &props);
    if (props.linearTilingFeatures &
    VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT) {
    image_info.tiling = VK_IMAGE_TILING_LINEAR;
    } else if (props.optimalTilingFeatures &
           VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT) {
    image_info.tiling = VK_IMAGE_TILING_OPTIMAL;
    } else {
    /* Try other depth formats? */
    std::cout << "depth_format " << depth_format << " Unsupported.\n";
    exit(-1);
    }

    image_info.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
    image_info.pNext = NULL;
    image_info.imageType = VK_IMAGE_TYPE_2D;
    image_info.format = depth_format;
    image_info.extent.width = info.width;
    image_info.extent.height = info.height;
    image_info.extent.depth = 1;
    image_info.mipLevels = 1;
    image_info.arrayLayers = 1;
    image_info.samples = NUM_SAMPLES;
    image_info.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    image_info.queueFamilyIndexCount = 0;
    image_info.pQueueFamilyIndices = NULL;
    image_info.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    image_info.usage = VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;
    image_info.flags = 0;

    VkMemoryAllocateInfo mem_alloc = {};
    mem_alloc.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    mem_alloc.pNext = NULL;
    mem_alloc.allocationSize = 0;
    mem_alloc.memoryTypeIndex = 0;

    VkImageViewCreateInfo view_info = {};
    view_info.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    view_info.pNext = NULL;
    view_info.image = VK_NULL_HANDLE;
    view_info.format = depth_format;
    view_info.components.r = VK_COMPONENT_SWIZZLE_R;
    view_info.components.g = VK_COMPONENT_SWIZZLE_G;
    view_info.components.b = VK_COMPONENT_SWIZZLE_B;
    view_info.components.a = VK_COMPONENT_SWIZZLE_A;
    view_info.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT;
    view_info.subresourceRange.baseMipLevel = 0;
    view_info.subresourceRange.levelCount = 1;
    view_info.subresourceRange.baseArrayLayer = 0;
    view_info.subresourceRange.layerCount = 1;
    view_info.viewType = VK_IMAGE_VIEW_TYPE_2D;
    view_info.flags = 0;

    if (depth_format == VK_FORMAT_D16_UNORM_S8_UINT ||
    depth_format == VK_FORMAT_D24_UNORM_S8_UINT ||
    depth_format == VK_FORMAT_D32_SFLOAT_S8_UINT) {
    view_info.subresourceRange.aspectMask |= VK_IMAGE_ASPECT_STENCIL_BIT;
    }

    VkMemoryRequirements mem_reqs;

    /* Create image */
    res = vkCreateImage(info.device, &image_info, NULL, &info.depth.image);
    assert(res == VK_SUCCESS);

    vkGetImageMemoryRequirements(info.device, info.depth.image, &mem_reqs);

    mem_alloc.allocationSize = mem_reqs.size;
    /* Use the memory properties to determine the type of memory required */
    pass = memory_type_from_properties(info, mem_reqs.memoryTypeBits,
                                   0, /* No requirements */
                                   &mem_alloc.memoryTypeIndex);
    assert(pass);

    /* Allocate memory */
    res = vkAllocateMemory(info.device, &mem_alloc, NULL, &info.depth.mem);
    assert(res == VK_SUCCESS);

    /* Bind memory */
    res = vkBindImageMemory(info.device, info.depth.image, info.depth.mem, 0);
    assert(res == VK_SUCCESS);

    /* Set the image layout to depth stencil optimal */
    set_image_layout(info, info.depth.image,
                 view_info.subresourceRange.aspectMask,
                 VK_IMAGE_LAYOUT_UNDEFINED,
                 VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

    /* Create image view */
    view_info.image = info.depth.image;
    res = vkCreateImageView(info.device, &view_info, NULL, &info.depth.view);
    assert(res == VK_SUCCESS);
    */
    return Ok(());
}

pub fn init(info: &mut SampleInfo, window: &winit::Window) -> VkResultObj<()> {
    let exts: Vec<CString> = platform_extensions(window)
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect();
    let exts_p: Vec<*const c_char> = exts.iter().map(|s| s.as_ptr()).collect();
    try!(init_instance(info, exts_p.as_slice()));
    try!(init_enumerate_device(info));
    try!(init_surface_wsi(info, window));
    try!(init_surface(info));
    try!(init_device(info, exts_p.as_slice()));
    try!(init_command_pool(info));
    try!(init_command_buffer(info));
    try!(execute_begin_command_buffer(info));
    try!(init_device_queue(info));
    try!(init_swap_chain(info));
    try!(init_depth_buffer(info));
    return Ok(());
}

fn destroy_depth_buffer(info: &mut SampleInfo) {
    // TODO: implement
    /*
    vkDestroyImageView(info.device, info.depth.view, NULL);
    vkDestroyImage(info.device, info.depth.image, NULL);
    vkFreeMemory(info.device, info.depth.mem, NULL);
    */

}

fn destroy_swap_chain(info: &mut SampleInfo) {
    // TODO: implement
    /*
    for (uint32_t i = 0; i < info.swapchainImageCount; i++) {
        vkDestroyImageView(info.device, info.buffers[i].view, NULL);
    }
    vkDestroySwapchainKHR(info.device, info.swap_chain, NULL);
    */

}

pub fn destroy(info: &mut SampleInfo) {
    destroy_depth_buffer(info);
    destroy_swap_chain(info);
    info.device
        .free_command_buffers(info.cmd_pool, &[info.cmd]);
    info.device.destroy_command_pool(info.cmd_pool);
    info.device.destroy();
    info.inst.destroy();
}
