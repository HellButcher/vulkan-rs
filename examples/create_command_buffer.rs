extern crate env_logger;
#[macro_use]
extern crate log;
extern crate vulkan_rs;

use std::ffi::CString;

use vulkan_rs::prelude::*;


fn create_instance(app_name: &str, _exts: &[&str]) -> VkInstance {
  let app_name = CString::new(app_name).unwrap();
  //TODO:let exts: Vec<CString> = exts.iter().map(|s| CString::new(*s).unwrap()).collect();
  //TODO:let exts_p: Vec<*const c_char> = exts.iter().map(|s| s.as_ptr()).collect();
  let app_info = VkApplicationInfo::new()
    .set_application_name(Some(&app_name))
    .set_application_version(vk_make_version(1, 0, 0))
    .set_engine_name(Some(&app_name))
    .set_engine_version(vk_make_version(1, 0, 0))
    .set_api_version(VK_API_VERSION_1_0);
  let create_info = VkInstanceCreateInfo::new()
                        //.set_pp_enabled_extension_names(exts_p)
                        .set_application_info(Some(&app_info));
  vkCreateInstance(&create_info, None).unwrap()
}

fn choose_physical_device(instance: VkInstance) -> VkPhysicalDevice {
    vkEnumeratePhysicalDevices(instance).unwrap()[0]
}

fn choose_queue_family(phys_dev: VkPhysicalDevice) -> (u32, VkQueueFamilyProperties) {
  let queue_props = vkGetPhysicalDeviceQueueFamilyProperties(phys_dev);
  assert!(queue_props.len() > 0);

  for (idx, props) in queue_props.iter().enumerate() {
    if props.queueFlags.contains(VkQueueFlagBits::GRAPHICS_BIT) {
      return (idx as u32, props.clone())
    }
  }

  panic!("no supported queue family available");
}

fn create_device(phys_device: VkPhysicalDevice, queue_idx: u32) -> VkDevice {
    let queue_create_info = &[
        VkDeviceQueueCreateInfo::new()
            .set_queue_family_index(queue_idx)
            .set_queue_priorities(&[0.0])
    ];
    let create_info = VkDeviceCreateInfo::new()
        .set_queue_create_infos(queue_create_info);
    vkCreateDevice(phys_device, &create_info, None).unwrap()
}

fn create_command_pool(device: VkDevice, queue_idx: u32) -> VkCommandPool {
  let create_info = VkCommandPoolCreateInfo::new()
                      .set_queue_family_index(queue_idx);
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
  env_logger::init().unwrap();

  let instance = create_instance("example ctriangle", &[]);
  info!("created instance {:?}", instance);
  let phys_device = choose_physical_device(instance);
  info!("choosed physical device {:?}", phys_device);
  let (queue_idx, queue_props) = choose_queue_family(phys_device);
  info!("choosed queue family index {:?}", queue_idx);
  let device = create_device(phys_device, queue_idx);
  info!("created device {:?}", device);
  let cmd_pool = create_command_pool(device, queue_idx);
  info!("created command pool {:?}", cmd_pool);
  let cmd_buffer = allocate_command_buffer(device, cmd_pool);
  info!("allocated command buffer {:?}", cmd_buffer);
  
  info!("============================================");

  info!("============================================");

  info!("freeing command buffer {:?}", cmd_buffer);
  vkFreeCommandBuffers(device, cmd_pool, &[cmd_buffer]);
  info!("destroying command pool {:?}", cmd_pool);
  vkDestroyCommandPool(device, Some(cmd_pool), None);
  info!("destroying device {:?}", device);
  vkDestroyDevice(device, None);
  info!("destroying instance {:?}", instance);
  vkDestroyInstance(instance, None);
  info!("done");
}
