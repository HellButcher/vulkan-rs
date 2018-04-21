extern crate env_logger;
#[macro_use]
extern crate log;
extern crate vulkan_rs;

use std::ffi::CString;

use vulkan_rs::prelude::*;

fn create_instance(app_name: &str, _exts: &[&str]) -> Result<VkInstance, VkResult> {
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
  let r = vkCreateInstance(&create_info, None);
  match r {
    Ok(instance) => {
      debug!("created instance {:?}", instance);
      return Ok(instance);
    }
    Err(e) => {
      debug!("unable to create instance: {:?}", e);
      return Err(e);
    }
  }
}

fn main() {
  env_logger::init_from_env(env_logger::Env::new().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

  let instance = create_instance("example create_instance", &[]).unwrap();

  let r = vkEnumeratePhysicalDevices(instance);
  match r {
    Ok(phys_devs) => {
      println!("There are {} devices", phys_devs.len());
    }
    Err(e) => {
      debug!("unable to enumerate devices: {:?}", e);
    }
  }
  vkDestroyInstance(instance, None);
}
