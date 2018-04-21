extern crate env_logger;
#[macro_use]
extern crate log;
extern crate vulkan_rs;
extern crate winit;

use vulkan_rs::prelude::*;
use vulkan_rs::prelude::vk_khr_swapchain::*;
use vulkan_rs::prelude::vk_khr_surface::*;
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
use vulkan_rs::prelude::vk_khr_wayland_surface::*;
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
use vulkan_rs::prelude::vk_khr_xcb_surface::*;
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
use vulkan_rs::prelude::vk_khr_win32_surface::*;

use vulkan_rs::utils::cstr_from_bytes_until_nul;

fn get_required_instance_extensions(el: &winit::EventsLoop) -> Vec<&'static str> {
  let mut exts = Vec::with_capacity(2);
  exts.push(VK_KHR_SURFACE_EXTENSION_NAME);
  #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
  {
    use winit::os::unix::EventsLoopExt;
    if el.is_wayland() {
      exts.push(VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME);
    }
  }
  #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
  {
    use winit::os::unix::EventsLoopExt;
    if el.is_x11() {
      exts.push(VK_KHR_XCB_SURFACE_EXTENSION_NAME);
    }
  }
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  {
    exts.push(VK_KHR_WIN32_SURFACE_EXTENSION_NAME);
  }
  #[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
  {
    exts.push(VK_KHR_ANDROID_SURFACE_EXTENSION_NAME);
  }
  exts
}

fn get_required_device_extensions() -> Vec<&'static str> {
  return vec![VK_KHR_SWAPCHAIN_EXTENSION_NAME];
}

fn create_instance(app_name: &str, exts: &[&str]) -> VkInstance {
  let app_name = cstr_from_bytes_until_nul(app_name);
  let exts_cstr: Vec<_> = exts.iter().map(cstr_from_bytes_until_nul).collect(); //TODO: better
  let exts_p: Vec<*const ::std::os::raw::c_char> = exts_cstr.iter().map(|s| s.as_ptr()).collect();
  let app_info = VkApplicationInfo::new()
    .set_application_name(Some(&app_name))
    .set_application_version(vk_make_version(1, 0, 0))
    .set_engine_name(Some(&app_name))
    .set_engine_version(vk_make_version(1, 0, 0))
    .set_api_version(VK_API_VERSION_1_0);
  let create_info = VkInstanceCreateInfo::new()
    .set_enabled_extension_names(&exts_p)
    .set_application_info(Some(&app_info));
  vkCreateInstance(&create_info, None).unwrap()
}

fn create_surface(instance: VkInstance, w: &winit::Window) -> VkSurfaceKHR {
  #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
  {
    use winit::os::unix::WindowExt;
    use vulkan_rs::platform::wsi::wayland::*;
    if let Some(wl_sfc) = w.get_wayland_surface() {
      let wl_dpy = w.get_wayland_display().unwrap();
      let create_info = VkWaylandSurfaceCreateInfoKHR::new()
        .set_display(wl_dpy as *mut wl_display)
        .set_surface(wl_sfc as *mut wl_surface);
      return vkCreateWaylandSurfaceKHR(instance, &create_info, None).unwrap();
    }
  }

  #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
  {
    use winit::os::unix::WindowExt;
    use vulkan_rs::platform::wsi::xcb::*;
    if let Some(xcb_win) = w.get_xlib_window() {
      let xcb_con = w.get_xcb_connection().unwrap();
      let create_info = VkXcbSurfaceCreateInfoKHR::new()
        .set_connection(xcb_con as *mut xcb_connection_t)
        .set_window(xcb_win as xcb_window_t);
      return vkCreateXcbSurfaceKHR(instance, &create_info, None).unwrap();
    }
  }

  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  {
    use kernel32;
    use winit::os::windows::WindowExt;
    let hinstance = kernel32::GetModuleHandleW(ptr::null());
    let create_info = VkWin32SurfaceCreateInfoKHR::new()
      .set_hinstance(hinstance)
      .set_window(w.get_hwnd());
    return vkCreateWin32SurfaceKHR(instance, &create_info, None).unwrap();
  }

  panic!("no window system available");
}

fn choose_queue_families(
  phys_dev: VkPhysicalDevice,
  surface: Option<VkSurfaceKHR>,
) -> Option<(u32, VkQueueFamilyProperties)> {
  let queue_props = vkGetPhysicalDeviceQueueFamilyProperties(phys_dev);
  assert!(queue_props.len() > 0);

  for (idx, props) in queue_props.into_iter().enumerate() {
    let idx = idx as u32;
    if !props.queueFlags.contains(VkQueueFlagBits::GRAPHICS_BIT) {
      continue;
    }
    if let Some(s) = surface {
      if !vkGetPhysicalDeviceSurfaceSupportKHR(phys_dev, idx, s).unwrap_or(false) {
        continue;
      }
    }
    return Some((idx, props));
  }
  // TODO: allow seperate graphics and present queue!
  None
}

fn choose_physical_device_and_queue_families(
  instance: VkInstance,
  surface: Option<VkSurfaceKHR>,
) -> (VkPhysicalDevice, u32, u32) {
  for (gpu_idx, gpu) in vkEnumeratePhysicalDevices(instance)
    .unwrap()
    .into_iter()
    .enumerate()
  {
    let gpu_idx = gpu_idx as u32;
    if let Some((queue_idx, _)) = choose_queue_families(gpu, surface) {
      return (gpu, gpu_idx, queue_idx);
    }
  }
  panic!("no device with suitable queuefamilies");
}

fn create_device(phys_device: VkPhysicalDevice, queue_idx: u32, exts: &[&str]) -> VkDevice {
  let exts_cstr: Vec<_> = exts.iter().map(cstr_from_bytes_until_nul).collect(); //TODO: better
  let exts_p: Vec<*const ::std::os::raw::c_char> = exts_cstr.iter().map(|s| s.as_ptr()).collect();
  let queue_create_info = &[
    VkDeviceQueueCreateInfo::new()
      .set_queue_family_index(queue_idx)
      .set_queue_priorities(&[0.0]),
  ];
  let create_info = VkDeviceCreateInfo::new()
    .set_queue_create_infos(queue_create_info)
    .set_enabled_extension_names(&exts_p);
  vkCreateDevice(phys_device, &create_info, None).unwrap()
}

fn create_command_pool(device: VkDevice, queue_idx: u32) -> VkCommandPool {
  let create_info = VkCommandPoolCreateInfo::new().set_queue_family_index(queue_idx);
  vkCreateCommandPool(device, &create_info, None).unwrap()
}

fn allocate_command_buffer(device: VkDevice, cmd_pool: VkCommandPool) -> VkCommandBuffer {
  let allc_info = VkCommandBufferAllocateInfo::new()
    .set_command_pool(cmd_pool)
    .set_level(VkCommandBufferLevel::E_PRIMARY)
    .set_command_buffer_count(1);
  vkAllocateCommandBuffers(device, &allc_info).unwrap()[0]
}

fn main() {
  env_logger::init_from_env(env_logger::Env::new().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

  info!("initializing...");

  // window system
  let mut events_loop = winit::EventsLoop::new();

  let instance_exts = get_required_instance_extensions(&events_loop);
  info!("required instance extensions: {:?}", instance_exts);
  let device_exts = get_required_device_extensions();
  info!("required device extensions: {:?}", device_exts);

  let instance = create_instance("example ctriangle", &instance_exts);
  info!("created instance {:?}", instance);
  let window = winit::Window::new(&events_loop).unwrap();
  info!("created window {:?}", window.id());
  let surface = create_surface(instance, &window);
  info!("created surface {:?}", surface);
  let (gpu, gpu_idx, queue_idx) = choose_physical_device_and_queue_families(instance, Some(surface));
  info!("choosed physical device {}: {:?}", gpu_idx, gpu);
  info!("choosed queue family index {:?}", queue_idx);
  let device = create_device(gpu, queue_idx, &device_exts);
  info!("created device {:?}", device);
  let cmd_pool = create_command_pool(device, queue_idx);
  info!("created command pool {:?}", cmd_pool);
  let cmd_buffer = allocate_command_buffer(device, cmd_pool);
  info!("allocated command buffer {:?}", cmd_buffer);
  //TODO: begin commad buffer
  //TODO: get device queue
  //TODO: create swapchain
  //TODO: create depthbuffer
  //TODO: create uniformbuffer
  //TODO: create descriptor and pipeline payout
  //TODO: create renderpass
  //TODO: create shaders
  //TODO: create framebuffers
  //TODO: create vertexbuffer
  //TODO: create descriptorpool
  //TODO: create descriptorset
  //TODO: create pipelinecache
  //TODO: create pipeline

  info!("done");
  info!("============================================");

  //TODO: get next image
  //TODO: begin renderpass
  //TODO: begin pipeline/descriptorset/vertexbuffer
  //TODO: update viewport and scissors
  //TODO: draw command
  //TODO: end renderpass
  //TODO: present image

  info!("============================================");
  info!("shutdown...");

  //TODO: destroy pipeline
  //TODO: destroy pipelinecache
  //TODO: destroy descriptorpool
  //TODO: destroy vertexbuffer
  //TODO: destroy framebuffers
  //TODO: destroy shaders
  //TODO: destroy renderpass
  //TODO: destroy descriptor and pipeline layouts
  //TODO: destroy uniformbuffer
  //TODO: destroy depthbuffer
  //TODO: destroy swapchain
  info!("freeing command buffer {:?}", cmd_buffer);
  vkFreeCommandBuffers(device, cmd_pool, &[cmd_buffer]);
  info!("destroying command pool {:?}", cmd_pool);
  vkDestroyCommandPool(device, Some(cmd_pool), None);
  info!("destroying device {:?}", device);
  vkDestroyDevice(device, None);
  info!("destroying surface {:?}", surface);
  vkDestroySurfaceKHR(instance, Some(surface), None);
  info!("destroying window {:?}", window.id());
  drop(window);
  info!("destroying instance {:?}", instance);
  vkDestroyInstance(instance, None);

  drop(events_loop);
  info!("done");
}
