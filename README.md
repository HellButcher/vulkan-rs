# vulkan-rs
Vulkan bindings for the rust programming language.

*EARLY STAGE!!!*

## Basic usage

```toml
[dependencies]
vulkan_rs = "1.0.33"
```

```rust
extern crate vulkan_rs;
[...]
use vulkan_rs::prelude::*;
[...]
fn main() {
  [...]
  let app_aame = CString::new("Application name").unwrap();
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
      enabledExtensionCount: 0,
      ppEnabledExtensionNames: vk_null(),
  };
  let instance = vkCreateInstance(&create_info, None).unwrap();
  [...]
}
```
