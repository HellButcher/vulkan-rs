# `vulkan_rs`
Vulkan bindings for the rust programming language.

[![Build Status](https://travis-ci.org/HellButcher/vulkan-rs.svg?branch=1.0-branch)](https://travis-ci.org/HellButcher/vulkan-rs)
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
vulkan_rs = "1.0.62"
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
