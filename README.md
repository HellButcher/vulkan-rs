# `vulkan_rs`

Vulkan bindings for the rust programming language.

[![Version](https://img.shields.io/crates/v/vulkan_rs.svg)](https://crates.io/crates/vulkan_rs)
[![Docs](https://docs.rs/vulkan_rs/badge.svg)](https://docs.rs/vulkan_rs)
[![Build Status](https://travis-ci.org/HellButcher/vulkan-rs.svg?branch=master)](https://travis-ci.org/HellButcher/vulkan-rs)
[![License](https://img.shields.io/badge/License-BSD%202--Clause-orange.svg)](LICENSE)

## Discontinued ⚠

This crate is no longer under development: I recommend using [`ash`](https://crates.io/crates/ash) instead.

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

## Stability Notice

I'm a little bit experimenting with the API generator.
I might introduce non-backward compatible changes, for making the API more safe and sound.

Some topics, that might come in the future ([x] = already done):
 - safe commands:
   - [x] pass length- and array-pointer pairs as `slice`
   - [x] use references (and no pointers) where possible
   - [x] usage of `Option<T>` for optional parameters (especialy for references and handles)
   - [x] returning of "output-parameters" (`Result<T,VkResult>` when command returns `VkResult`)
   - [x] enumerating: returning `Vec<T>` or `Result<Vec<T>,VkResult>` (e.g. vkEnumeratePhysicalDevices)
   - [ ] simplify passing of `&str` (`&[&str]` not required?)
 - safe enums:
   - [x] usage of `enum` for enums and not `u32`
   - [x] usage of `crate bitflags` for bitmasks
   - [x] own `VkError` enum (VkResult items, but without `VK_SUCCESS`), `VkResult = Result<(),VkError>`
 - safe structs:
   - [x] hide length- and array-pointer pairs
     - provide safe setter with `slice` parameter
   - [x] hide `VkStructureType` and provide default
   - [x] use references (and no pointers) where possible (hide pointers)
   - [x] usage of `Option<T>` for optional fields
   - [ ] simplify passing of `&str` and `&[&str]`
     - setting string-arrays (`const char* const*`) not possible at the moment
   - [ ] setting of `const void* pNext` chain
   - :warning: Keep the structs binary compatible (same length, same padding)
 - safe handles:
   - [x] non-zero handles (use `Optional<T>` for zeroable handles)
   - [ ] Owned and Borrowed handles
     - `vkCreate*` commands return Owned handles
     - `vkDestroy*` commands consume Owned handles
     - everything else uses Borrowed handles
   - [ ] implement `Drop` (either panic or call `vkDestroy*`)
   - [ ] lifetimes: child-handle shouldn't outlive it's parent-handle

