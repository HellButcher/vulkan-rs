# `vulkan_rs`

Vulkan bindings for the rust programming language.

[![Build Status](https://travis-ci.org/HellButcher/vulkan-rs.svg?branch=master)](https://travis-ci.org/HellButcher/vulkan-rs)
[![License](https://img.shields.io/badge/License-BSD%202--Clause-orange.svg)](LICENSE)

## Overview

* Crate `vulkan_rs`

  [![Version](https://img.shields.io/crates/v/vulkan_rs.svg)](https://crates.io/crates/vulkan_rs)
  [![Docs](https://docs.rs/vulkan_rs/badge.svg)](https://docs.rs/vulkan_rs)

* Crate `vulkan_rs_generator`

  [![Version](https://img.shields.io/crates/v/vulkan_rs_generator.svg)](https://crates.io/crates/vulkan_rs_generator)
  [![Docs](https://docs.rs/vulkan_rs_generator/badge.svg)](https://docs.rs/vulkan_rs_generator)


## Basic usage

```toml
[dependencies]
vulkan_rs = "0.4"
```

```rust
extern crate vulkan_rs;

use vulkan_rs::prelude::*;

fn main() {
  let app_name = CString::new("Application name").unwrap();
  let app_info = VkApplicationInfo::new()
    .set_application_name(Some(&app_name))
    .set_application_version(vk_make_version(1, 0, 0))
    .set_engine_name(Some(&app_name))
    .set_engine_version(vk_make_version(1, 0, 0))
    .set_api_version(VK_API_VERSION_1_0);
  let create_info = VkInstanceCreateInfo::new()
    .set_application_info(Some(&app_info));
  let instance = vkCreateInstance(&create_info, None).unwrap();
  let phys_devices = vkEnumeratePhysicalDevices(instance).unwrap();
  println!("There are {} physical devices", phys_devices.len());
  // [...]
  vkDestroyInstance(instance, None);
}
```
