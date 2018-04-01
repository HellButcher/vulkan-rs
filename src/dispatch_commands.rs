/* GENERATED FILE */

#![allow(non_snake_case)]

use AsRaw;
use dispatch_table::*;
use enums::VkResult;
use platform::*;
use types::*;

// feature: VK_VERSION_1_0

/// Create a new Vulkan instance
///
///   - `pCreateInfo` points to an instance of `VkInstanceCreateInfo` controlling
///     creation of the instance.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pInstance` points a `VkInstance` handle in which the resulting instance is
///     returned.
///
/// `vkCreateInstance` verifies that the requested layers exist. If not,
/// `vkCreateInstance` will return `VK_ERROR_LAYER_NOT_PRESENT`. Next
/// `vkCreateInstance` verifies that the requested extensions are supported (e.g. in
/// the implementation or in any enabled instance layer) and if any requested
/// extension is not supported, `vkCreateInstance` must: return
/// `VK_ERROR_EXTENSION_NOT_PRESENT`. After verifying and enabling the instance
/// layers and extensions the `VkInstance` object is created and returned to the
/// application. If a requested extension is only supported by a layer, both the
/// layer and the extension need to be specified at `vkCreateInstance` time for the
/// creation to succeed.
///
pub fn vkCreateInstance(
  pCreateInfo: &VkInstanceCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkInstance, VkResult> {
  unsafe {
    let mut pInstance: VkInstance = ::std::mem::zeroed();
    VkLoaderDispatchTable::with(|_t| {
      let _r = _t.vkCreateInstance.unwrap()(
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pInstance).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      VkInstanceDispatchTable::create(pCreateInfo, pAllocator, pInstance);
      return Ok(pInstance);
    })
  }
}

/// Destroy an instance of Vulkan
///
///   - `instance` is the handle of the instance to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyInstance(instance: VkInstance, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkInstanceDispatchTable::with(instance, |_t| {
      _t.vkDestroyInstance.unwrap()(instance.as_raw(), pAllocator.as_raw())
    });
    VkInstanceDispatchTable::destroy(instance);
  }
}

/// Enumerates the physical devices accessible to a Vulkan instance
///
/// To retrieve a list of physical device objects representing the physical devices
/// installed in the system.
///
///   - `instance` is a handle to a Vulkan instance previously created with
///     `vkCreateInstance`.
///
///   - `pPhysicalDeviceCount` is a pointer to an integer related to the number of
///     physical devices available or queried, as described below.
///
///   - `pPhysicalDevices` is either `NULL` or a pointer to an array of
///     `VkPhysicalDevice` handles.
///
/// If `pPhysicalDevices` is `NULL`, then the number of physical devices available
/// is returned in `pPhysicalDeviceCount`. Otherwise, `pPhysicalDeviceCount` must:
/// point to a variable set by the user to the number of elements in the
/// `pPhysicalDevices` array, and on return the variable is overwritten with the
/// number of handles actually written to `pPhysicalDevices`. If
/// `pPhysicalDeviceCount` is less than the number of physical devices available, at
/// most `pPhysicalDeviceCount` structures will be written. If
/// `pPhysicalDeviceCount` is smaller than the number of physical devices available,
/// `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to indicate that not
/// all the available physical devices were returned.
///
pub fn vkEnumeratePhysicalDevices(instance: VkInstance) -> Result<Vec<VkPhysicalDevice>, VkResult> {
  unsafe {
    let mut pPhysicalDeviceCount: u32 = 0;
    let mut pPhysicalDevices: Vec<VkPhysicalDevice> = Vec::new();
    VkInstanceDispatchTable::with(instance, |_t| loop {
      let _r = _t.vkEnumeratePhysicalDevices.unwrap()(
        instance.as_raw(),
        &mut pPhysicalDeviceCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPhysicalDeviceCount == 0 {
        return Ok(pPhysicalDevices);
      }
      pPhysicalDevices = Vec::with_capacity(pPhysicalDeviceCount as usize);
      let _r = _t.vkEnumeratePhysicalDevices.unwrap()(
        instance.as_raw(),
        &mut pPhysicalDeviceCount,
        pPhysicalDevices.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pPhysicalDevices.set_len(pPhysicalDeviceCount as usize);
      return Ok(pPhysicalDevices);
    })
  }
}

/// Reports capabilities of a physical device
///
///   - `physicalDevice` is the physical device from which to query the supported
///     features.
///
///   - `pFeatures` is a pointer to a `VkPhysicalDeviceFeatures` structure in which
///     the physical device features are returned. For each feature, a value of
///     `VK_TRUE` indicates that the feature is supported on this physical device,
///     and `VK_FALSE` indicates that the feature is not supported.
///
pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice) -> VkPhysicalDeviceFeatures {
  unsafe {
    let mut pFeatures: VkPhysicalDeviceFeatures = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceFeatures.unwrap()(physicalDevice.as_raw(), (&mut pFeatures).as_raw());
      return pFeatures;
    })
  }
}

/// Lists physical device's format capabilities
///
/// To query supported format features which are properties of the physical device.
///
///   - `physicalDevice` is the physical device from which to query the format
///     properties.
///
///   - `format` is the format whose properties are queried.
///
///   - `pFormatProperties` is a pointer to a `VkFormatProperties` structure in
///     which physical device properties for `format` are returned.
///
pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat) -> VkFormatProperties {
  unsafe {
    let mut pFormatProperties: VkFormatProperties = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceFormatProperties.unwrap()(
        physicalDevice.as_raw(),
        format,
        (&mut pFormatProperties).as_raw(),
      );
      return pFormatProperties;
    })
  }
}

/// Lists physical device's image format capabilities
///
///   - `physicalDevice` is the physical device from which to query the image
///     capabilities.
///
///   - `format` is a `VkFormat` value specifying the image format, corresponding to
///     `VkImageCreateInfo::format`.
///
///   - `type` is a `VkImageType` value specifying the image type, corresponding to
///     `VkImageCreateInfo::imageType`.
///
///   - `tiling` is a `VkImageTiling` value specifying the image tiling,
///     corresponding to `VkImageCreateInfo::tiling`.
///
///   - `usage` is a bitmask of `VkImageUsageFlagBits` specifying the intended usage
///     of the image, corresponding to `VkImageCreateInfo::usage`.
///
///   - `flags` is a bitmask of `VkImageCreateFlagBits` specifying additional
///     parameters of the image, corresponding to `VkImageCreateInfo::flags`.
///
///   - `pImageFormatProperties` points to an instance of the
///     `VkImageFormatProperties` structure in which capabilities are returned.
///
/// The `format`, `type`, `tiling`, `usage`, and `flags` parameters correspond to
/// parameters that would be consumed by `vkCreateImage` (as members of
/// `VkImageCreateInfo`).
///
/// If `format` is not a supported image format, or if the combination of `format`,
/// `type`, `tiling`, `usage`, and `flags` is not supported for images, then
/// `vkGetPhysicalDeviceImageFormatProperties` returns
/// `VK_ERROR_FORMAT_NOT_SUPPORTED`.
///
/// The limitations on an image format that are reported by
/// `vkGetPhysicalDeviceImageFormatProperties` have the following property: if
/// `usage1` and `usage2` of type `VkImageUsageFlags` are such that the bits set in
/// `usage1` are a subset of the bits set in `usage2`, and `flags1` and `flags2` of
/// type `VkImageCreateFlags` are such that the bits set in `flags1` are a subset of
/// the bits set in `flags2`, then the limitations for `usage1` and `flags1` must:
/// be no more strict than the limitations for `usage2` and `flags2`, for all values
/// of `format`, `type`, and `tiling`.
///
pub fn vkGetPhysicalDeviceImageFormatProperties(
  physicalDevice: VkPhysicalDevice,
  format: VkFormat,
  eType: VkImageType,
  tiling: VkImageTiling,
  usage: VkImageUsageFlags,
  flags: VkImageCreateFlags,
) -> Result<VkImageFormatProperties, VkResult> {
  unsafe {
    let mut pImageFormatProperties: VkImageFormatProperties = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceImageFormatProperties.unwrap()(
        physicalDevice.as_raw(),
        format,
        eType,
        tiling,
        usage,
        flags,
        (&mut pImageFormatProperties).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pImageFormatProperties);
    })
  }
}

/// Returns properties of a physical device
///
///   - `physicalDevice` is the handle to the physical device whose properties will
///     be queried.
///
///   - `pProperties` points to an instance of the `VkPhysicalDeviceProperties`
///     structure, that will be filled with returned information.
///
pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice) -> VkPhysicalDeviceProperties {
  unsafe {
    let mut pProperties: VkPhysicalDeviceProperties = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceProperties.unwrap()(physicalDevice.as_raw(), (&mut pProperties).as_raw());
      return pProperties;
    })
  }
}

/// Reports properties of the queues of the specified physical device
///
///   - `physicalDevice` is the handle to the physical device whose properties will
///     be queried.
///
///   - `pQueueFamilyPropertyCount` is a pointer to an integer related to the number
///     of queue families available or queried, as described below.
///
///   - `pQueueFamilyProperties` is either `NULL` or a pointer to an array of
///     `VkQueueFamilyProperties` structures.
///
/// If `pQueueFamilyProperties` is `NULL`, then the number of queue families
/// available is returned in `pQueueFamilyPropertyCount`. Otherwise,
/// `pQueueFamilyPropertyCount` must: point to a variable set by the user to the
/// number of elements in the `pQueueFamilyProperties` array, and on return the
/// variable is overwritten with the number of structures actually written to
/// `pQueueFamilyProperties`. If `pQueueFamilyPropertyCount` is less than the number
/// of queue families available, at most `pQueueFamilyPropertyCount` structures will
/// be written.
///
pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice) -> Vec<VkQueueFamilyProperties> {
  unsafe {
    let mut pQueueFamilyPropertyCount: u32 = 0;
    let mut pQueueFamilyProperties: Vec<VkQueueFamilyProperties> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceQueueFamilyProperties.unwrap()(
        physicalDevice.as_raw(),
        &mut pQueueFamilyPropertyCount,
        ::std::ptr::null_mut(),
      );
      if pQueueFamilyPropertyCount == 0 {
        return pQueueFamilyProperties;
      }
      pQueueFamilyProperties = Vec::with_capacity(pQueueFamilyPropertyCount as usize);
      _t.vkGetPhysicalDeviceQueueFamilyProperties.unwrap()(
        physicalDevice.as_raw(),
        &mut pQueueFamilyPropertyCount,
        pQueueFamilyProperties.as_mut_slice().as_raw(),
      );
      pQueueFamilyProperties.set_len(pQueueFamilyPropertyCount as usize);
      return pQueueFamilyProperties;
    })
  }
}

/// Reports memory information for the specified physical device
///
///   - `physicalDevice` is the handle to the device to query.
///
///   - `pMemoryProperties` points to an instance of
///     `VkPhysicalDeviceMemoryProperties` structure in which the properties are
///     returned.
///
pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice) -> VkPhysicalDeviceMemoryProperties {
  unsafe {
    let mut pMemoryProperties: VkPhysicalDeviceMemoryProperties = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceMemoryProperties.unwrap()(physicalDevice.as_raw(), (&mut pMemoryProperties).as_raw());
      return pMemoryProperties;
    })
  }
}

/// Create a new device instance
///
/// A logical device is created as a *connection* to a physical device.
///
///   - `physicalDevice` must: be one of the device handles returned from a call to
///     `vkEnumeratePhysicalDevices` (see [Physical Device
///     Enumeration](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-physical-device-enumeration)).
///
///   - `pCreateInfo` is a pointer to a `VkDeviceCreateInfo` structure containing
///     information about how to create the device.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pDevice` points to a handle in which the created `VkDevice` is returned.
///
/// `vkCreateDevice` verifies that extensions and features requested in the
/// `ppEnabledExtensionNames` and `pEnabledFeatures` members of `pCreateInfo`,
/// respectively, are supported by the implementation. If any requested extension is
/// not supported, `vkCreateDevice` must: return `VK_ERROR_EXTENSION_NOT_PRESENT`.
/// If any requested feature is not supported, `vkCreateDevice` must: return
/// `VK_ERROR_FEATURE_NOT_PRESENT`. Support for extensions can: be checked before
/// creating a device by querying `vkEnumerateDeviceExtensionProperties`. Support
/// for features can: similarly be checked by querying
/// `vkGetPhysicalDeviceFeatures`.
///
/// After verifying and enabling the extensions the `VkDevice` object is created and
/// returned to the application. If a requested extension is only supported by a
/// layer, both the layer and the extension need to be specified at
/// `vkCreateInstance` time for the creation to succeed.
///
/// Multiple logical devices can: be created from the same physical device. Logical
/// device creation may: fail due to lack of device-specific resources (in addition
/// to the other errors). If that occurs, `vkCreateDevice` will return
/// `VK_ERROR_TOO_MANY_OBJECTS`.
///
pub fn vkCreateDevice(
  physicalDevice: VkPhysicalDevice,
  pCreateInfo: &VkDeviceCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkDevice, VkResult> {
  unsafe {
    let mut pDevice: VkDevice = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkCreateDevice.unwrap()(
        physicalDevice.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pDevice).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      VkDeviceDispatchTable::create(physicalDevice, pCreateInfo, pAllocator, pDevice);
      return Ok(pDevice);
    })
  }
}

/// Destroy a logical device
///
///   - `device` is the logical device to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
/// To ensure that no work is active on the device, `vkDeviceWaitIdle` can: be used
/// to gate the destruction of the device. Prior to destroying a device, an
/// application is responsible for destroying/freeing any Vulkan objects that were
/// created using that device as the first parameter of the corresponding
/// ftext:vkCreate\* or ftext:vkAllocate\* command.
///
/// > **Note**
/// >
/// > The lifetime of each of these objects is bound by the lifetime of the
/// > `VkDevice` object. Therefore, to avoid resource leaks, it is critical that an
/// > application explicitly free all of these resources prior to calling
/// > `vkDestroyDevice`.
///
pub fn vkDestroyDevice(device: VkDevice, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyDevice.unwrap()(device.as_raw(), pAllocator.as_raw())
    });
    VkDeviceDispatchTable::destroy(device);
  }
}

/// Returns up to requested number of global extension properties
///
///   - `pLayerName` is either `NULL` or a pointer to a null-terminated UTF-8 string
///     naming the layer to retrieve extensions from.
///
///   - `pPropertyCount` is a pointer to an integer related to the number of
///     extension properties available or queried, as described below.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkExtensionProperties` structures.
///
/// When `pLayerName` parameter is `NULL`, only extensions provided by the Vulkan
/// implementation or by implicitly enabled layers are returned. When `pLayerName`
/// is the name of a layer, the instance extensions provided by that layer are
/// returned.
///
/// If `pProperties` is `NULL`, then the number of extensions properties available
/// is returned in `pPropertyCount`. Otherwise, `pPropertyCount` must: point to a
/// variable set by the user to the number of elements in the `pProperties` array,
/// and on return the variable is overwritten with the number of structures actually
/// written to `pProperties`. If `pPropertyCount` is less than the number of
/// extension properties available, at most `pPropertyCount` structures will be
/// written. If `pPropertyCount` is smaller than the number of extensions available,
/// `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to indicate that not
/// all the available properties were returned.
///
/// Because the list of available layers may change externally between calls to
/// `vkEnumerateInstanceExtensionProperties`, two calls may retrieve different
/// results if a `pLayerName` is available in one call but not in another. The
/// extensions supported by a layer may also change between two calls, e.g. if the
/// layer implementation is replaced by a different version between those calls.
///
pub fn vkEnumerateInstanceExtensionProperties(
  pLayerName: Option<&AsRef<CStr>>,
) -> Result<Vec<VkExtensionProperties>, VkResult> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkExtensionProperties> = Vec::new();
    VkLoaderDispatchTable::with(|_t| loop {
      let _r = _t.vkEnumerateInstanceExtensionProperties.unwrap()(
        pLayerName.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkEnumerateInstanceExtensionProperties.unwrap()(
        pLayerName.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Returns properties of available physical device extensions
///
///   - `physicalDevice` is the physical device that will be queried.
///
///   - `pLayerName` is either `NULL` or a pointer to a null-terminated UTF-8 string
///     naming the layer to retrieve extensions from.
///
///   - `pPropertyCount` is a pointer to an integer related to the number of
///     extension properties available or queried, and is treated in the same
///     fashion as the `vkEnumerateInstanceExtensionProperties::pPropertyCount`
///     parameter.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkExtensionProperties` structures.
///
/// When `pLayerName` parameter is `NULL`, only extensions provided by the Vulkan
/// implementation or by implicitly enabled layers are returned. When `pLayerName`
/// is the name of a layer, the device extensions provided by that layer are
/// returned.
///
pub fn vkEnumerateDeviceExtensionProperties(
  physicalDevice: VkPhysicalDevice,
  pLayerName: Option<&AsRef<CStr>>,
) -> Result<Vec<VkExtensionProperties>, VkResult> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkExtensionProperties> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkEnumerateDeviceExtensionProperties.unwrap()(
        physicalDevice.as_raw(),
        pLayerName.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkEnumerateDeviceExtensionProperties.unwrap()(
        physicalDevice.as_raw(),
        pLayerName.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Returns up to requested number of global layer properties
///
///   - `pPropertyCount` is a pointer to an integer related to the number of layer
///     properties available or queried, as described below.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkLayerProperties` structures.
///
/// If `pProperties` is `NULL`, then the number of layer properties available is
/// returned in `pPropertyCount`. Otherwise, `pPropertyCount` must: point to a
/// variable set by the user to the number of elements in the `pProperties` array,
/// and on return the variable is overwritten with the number of structures actually
/// written to `pProperties`. If `pPropertyCount` is less than the number of layer
/// properties available, at most `pPropertyCount` structures will be written. If
/// `pPropertyCount` is smaller than the number of layers available, `VK_INCOMPLETE`
/// will be returned instead of `VK_SUCCESS`, to indicate that not all the available
/// layer properties were returned.
///
/// The list of available layers may change at any time due to actions outside of
/// the Vulkan implementation, so two calls to `vkEnumerateInstanceLayerProperties`
/// with the same parameters may: return different results, or retrieve different
/// `pPropertyCount` values or `pProperties` contents. Once an instance has been
/// created, the layers enabled for that instance will continue to be enabled and
/// valid for the lifetime of that instance, even if some of them become unavailable
/// for future instances.
///
pub fn vkEnumerateInstanceLayerProperties() -> Result<Vec<VkLayerProperties>, VkResult> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkLayerProperties> = Vec::new();
    VkLoaderDispatchTable::with(|_t| loop {
      let _r = _t.vkEnumerateInstanceLayerProperties.unwrap()(&mut pPropertyCount, ::std::ptr::null_mut());
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkEnumerateInstanceLayerProperties.unwrap()(&mut pPropertyCount, pProperties.as_mut_slice().as_raw());
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Returns properties of available physical device layers
///
///   - `pPropertyCount` is a pointer to an integer related to the number of layer
///     properties available or queried.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkLayerProperties` structures.
///
/// If `pProperties` is `NULL`, then the number of layer properties available is
/// returned in `pPropertyCount`. Otherwise, `pPropertyCount` must: point to a
/// variable set by the user to the number of elements in the `pProperties` array,
/// and on return the variable is overwritten with the number of structures actually
/// written to `pProperties`. If `pPropertyCount` is less than the number of layer
/// properties available, at most `pPropertyCount` structures will be written. If
/// `pPropertyCount` is smaller than the number of layers available, `VK_INCOMPLETE`
/// will be returned instead of `VK_SUCCESS`, to indicate that not all the available
/// layer properties were returned.
///
/// The list of layers enumerated by `vkEnumerateDeviceLayerProperties` must: be
/// exactly the sequence of layers enabled for the instance. The members of
/// `VkLayerProperties` for each enumerated layer must: be the same as the
/// properties when the layer was enumerated by
/// `vkEnumerateInstanceLayerProperties`.
///
pub fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice) -> Result<Vec<VkLayerProperties>, VkResult> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkLayerProperties> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkEnumerateDeviceLayerProperties.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkEnumerateDeviceLayerProperties.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Get a queue handle from a device
///
///   - `device` is the logical device that owns the queue.
///
///   - `queueFamilyIndex` is the index of the queue family to which the queue
///     belongs.
///
///   - `queueIndex` is the index within this queue family of the queue to retrieve.
///
///   - `pQueue` is a pointer to a `VkQueue` object that will be filled with the
///     handle for the requested queue.
///
pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32) -> VkQueue {
  unsafe {
    let mut pQueue: VkQueue = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetDeviceQueue.unwrap()(
        device.as_raw(),
        queueFamilyIndex,
        queueIndex,
        (&mut pQueue).as_raw(),
      );
      return pQueue;
    })
  }
}

/// Submits a sequence of semaphores or command buffers to a queue
///
///   - `queue` is the queue that the command buffers will be submitted to.
///
///   - `submitCount` is the number of elements in the `pSubmits` array.
///
///   - `pSubmits` is a pointer to an array of `VkSubmitInfo` structures, each
///     specifying a command buffer submission batch.
///
///   - `fence` is an optional: handle to a fence to be signaled once all submitted
///     command buffers have completed execution. If `fence` is not
///     `VK_NULL_HANDLE`, it defines a [fence signal
///     operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-fences-signaling).
///
/// > **Note**
/// >
/// > Submission can be a high overhead operation, and applications should: attempt
/// > to batch work together into as few calls to `vkQueueSubmit` as possible.
///
/// `vkQueueSubmit` is a [queue submission command](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-submission), with
/// each batch defined by an element of `pSubmits` as an instance of the
/// `VkSubmitInfo` structure. Batches begin execution in the order they appear in
/// `pSubmits`, but may: complete out of order.
///
/// Fence and semaphore operations submitted with `vkQueueSubmit` have additional
/// ordering constraints compared to other submission commands, with dependencies
/// involving previous and subsequent queue operations. Information about these
/// additional constraints can be found in the
/// [semaphore](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores) and [fence](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-fences)
/// sections of [the synchronization chapter](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization).
///
/// Details on the interaction of `pWaitDstStageMask` with synchronization are
/// described in the [semaphore wait operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-waiting)
/// section of [the synchronization chapter](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization).
///
/// The order that batches appear in `pSubmits` is used to determine [submission
/// order](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-submission-order), and thus all the [implicit ordering
/// guarantees](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-implicit) that respect it. Other than these
/// implicit ordering guarantees and any [explicit synchronization
/// primitives](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization), these batches may: overlap or otherwise execute
/// out of order.
///
/// If any command buffer submitted to this queue is in the [executable
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle), it is moved to the [pending
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle). Once execution of all submissions of a
/// command buffer complete, it moves from the [pending
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle), back to the [executable
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle). If a command buffer was recorded with the
/// `VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT` flag, it instead moves back to the
/// [invalid state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
/// If `vkQueueSubmit` fails, it may: return `VK_ERROR_OUT_OF_HOST_MEMORY` or
/// `VK_ERROR_OUT_OF_DEVICE_MEMORY`. If it does, the implementation must: ensure
/// that the state and contents of any resources or synchronization primitives
/// referenced by the submitted command buffers and any semaphores referenced by
/// `pSubmits` is unaffected by the call or its failure. If `vkQueueSubmit` fails in
/// such a way that the implementation can: not make that guarantee, the
/// implementation must: return `VK_ERROR_DEVICE_LOST`. See [Lost
/// Device](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-lost-device).
///
pub fn vkQueueSubmit(queue: VkQueue, pSubmits: &[VkSubmitInfo], fence: Option<VkFence>) -> VkResult {
  unsafe {
    let submitCount = pSubmits.len() as u32;
    VkDeviceDispatchTable::with(queue, |_t| {
      _t.vkQueueSubmit.unwrap()(
        queue.as_raw(),
        submitCount,
        pSubmits.as_raw(),
        fence.as_raw(),
      )
    })
  }
}

/// Wait for a queue to become idle
///
/// To wait on the host for the completion of outstanding queue operations for a
/// given queue.
///
///   - `queue` is the queue on which to wait.
///
/// `vkQueueWaitIdle` is equivalent to submitting a fence to a queue and waiting
/// with an infinite timeout for that fence to signal.
///
pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult {
  unsafe { VkDeviceDispatchTable::with(queue, |_t| _t.vkQueueWaitIdle.unwrap()(queue.as_raw())) }
}

/// Wait for a device to become idle
///
/// To wait on the host for the completion of outstanding queue operations for all
/// queues on a given logical device.
///
///   - `device` is the logical device to idle.
///
/// `vkDeviceWaitIdle` is equivalent to calling `vkQueueWaitIdle` for all queues
/// owned by `device`.
///
pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult {
  unsafe { VkDeviceDispatchTable::with(device, |_t| _t.vkDeviceWaitIdle.unwrap()(device.as_raw())) }
}

/// Allocate GPU memory
///
///   - `device` is the logical device that owns the memory.
///
///   - `pAllocateInfo` is a pointer to an instance of the `VkMemoryAllocateInfo`
///     structure describing parameters of the allocation. A successful returned
///     allocation must: use the requested parameters — no substitution is permitted
///     by the implementation.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pMemory` is a pointer to a `VkDeviceMemory` handle in which information
///     about the allocated memory is returned.
///
/// Allocations returned by `vkAllocateMemory` are guaranteed to meet any alignment
/// requirement of the implementation. For example, if an implementation requires
/// 128 byte alignment for images and 64 byte alignment for buffers, the device
/// memory returned through this mechanism would be 128-byte aligned. This ensures
/// that applications can: correctly suballocate objects of different types (with
/// potentially different alignment requirements) in the same memory object.
///
/// When memory is allocated, its contents are undefined.
///
/// The maximum number of valid memory allocations that can: exist simultaneously
/// within a `VkDevice` may: be restricted by implementation- or platform-dependent
/// limits. If a call to `vkAllocateMemory` would cause the total number of
/// allocations to exceed these limits, such a call will fail and must: return
/// `VK_ERROR_TOO_MANY_OBJECTS`. The
/// [`maxMemoryAllocationCount`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits-maxMemoryAllocationCount) feature
/// describes the number of allocations that can: exist simultaneously before
/// encountering these internal limits.
///
/// Some platforms may: have a limit on the maximum size of a single allocation. For
/// example, certain systems may: fail to create allocations with a size greater
/// than or equal to 4GB. Such a limit is implementation-dependent, and if such a
/// failure occurs then the error `VK_ERROR_OUT_OF_DEVICE_MEMORY` must: be returned.
///
pub fn vkAllocateMemory(
  device: VkDevice,
  pAllocateInfo: &VkMemoryAllocateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkDeviceMemory, VkResult> {
  unsafe {
    let mut pMemory: VkDeviceMemory = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAllocateMemory.unwrap()(
        device.as_raw(),
        pAllocateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pMemory).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pMemory);
    })
  }
}
pub fn vkFreeMemory(device: VkDevice, memory: Option<VkDeviceMemory>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkFreeMemory.unwrap()(device.as_raw(), memory.as_raw(), pAllocator.as_raw())
    })
  }
}
pub fn vkMapMemory(
  device: VkDevice,
  memory: VkDeviceMemory,
  offset: VkDeviceSize,
  size: VkDeviceSize,
  flags: VkMemoryMapFlags,
) -> Result<*mut c_void, VkResult> {
  unsafe {
    let mut ppData: *mut c_void = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkMapMemory.unwrap()(
        device.as_raw(),
        memory.as_raw(),
        offset,
        size,
        flags,
        &mut ppData,
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(ppData);
    })
  }
}
pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkUnmapMemory.unwrap()(device.as_raw(), memory.as_raw())
    })
  }
}
pub fn vkFlushMappedMemoryRanges(device: VkDevice, pMemoryRanges: &[VkMappedMemoryRange]) -> VkResult {
  unsafe {
    let memoryRangeCount = pMemoryRanges.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkFlushMappedMemoryRanges.unwrap()(device.as_raw(), memoryRangeCount, pMemoryRanges.as_raw())
    })
  }
}
pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, pMemoryRanges: &[VkMappedMemoryRange]) -> VkResult {
  unsafe {
    let memoryRangeCount = pMemoryRanges.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkInvalidateMappedMemoryRanges.unwrap()(device.as_raw(), memoryRangeCount, pMemoryRanges.as_raw())
    })
  }
}
pub fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory) -> VkDeviceSize {
  unsafe {
    let mut pCommittedMemoryInBytes: VkDeviceSize = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetDeviceMemoryCommitment.unwrap()(
        device.as_raw(),
        memory.as_raw(),
        &mut pCommittedMemoryInBytes,
      );
      return pCommittedMemoryInBytes;
    })
  }
}

/// Bind device memory to a buffer object
///
///   - `device` is the logical device that owns the buffer and memory.
///
///   - `buffer` is the buffer to be attached to memory.
///
///   - `memory` is a `VkDeviceMemory` object describing the device memory to
///     attach.
///
///   - `memoryOffset` is the start offset of the region of `memory` which is to be
///     bound to the buffer. The number of bytes returned in the
///     `VkMemoryRequirements::size` member in `memory`, starting from
///     `memoryOffset` bytes, will be bound to the specified buffer.
///
/// `vkBindBufferMemory` is equivalent to passing the same parameters through
/// `VkBindBufferMemoryInfoKHR` to `vkBindBufferMemory2KHR`.
///
pub fn vkBindBufferMemory(
  device: VkDevice,
  buffer: VkBuffer,
  memory: VkDeviceMemory,
  memoryOffset: VkDeviceSize,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindBufferMemory.unwrap()(
        device.as_raw(),
        buffer.as_raw(),
        memory.as_raw(),
        memoryOffset,
      )
    })
  }
}

/// Bind device memory to an image object
///
/// To attach memory to a `VkImage` object created without the
/// `VK_IMAGE_CREATE_DISJOINT_BIT_KHR` set.
///
///   - `device` is the logical device that owns the image and memory.
///
///   - `image` is the image.
///
///   - `memory` is the `VkDeviceMemory` object describing the device memory to
///     attach.
///
///   - `memoryOffset` is the start offset of the region of `memory` which is to be
///     bound to the image. The number of bytes returned in the
///     `VkMemoryRequirements::size` member in `memory`, starting from
///     `memoryOffset` bytes, will be bound to the specified image.
///
/// `vkBindImageMemory` is equivalent to passing the same parameters through
/// `VkBindImageMemoryInfoKHR` to `vkBindImageMemory2KHR`.
///
pub fn vkBindImageMemory(
  device: VkDevice,
  image: VkImage,
  memory: VkDeviceMemory,
  memoryOffset: VkDeviceSize,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindImageMemory.unwrap()(
        device.as_raw(),
        image.as_raw(),
        memory.as_raw(),
        memoryOffset,
      )
    })
  }
}

/// Returns the memory requirements for specified Vulkan object
///
///   - `device` is the logical device that owns the buffer.
///
///   - `buffer` is the buffer to query.
///
///   - `pMemoryRequirements` points to an instance of the `VkMemoryRequirements`
///     structure in which the memory requirements of the buffer object are
///     returned.
///
pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer) -> VkMemoryRequirements {
  unsafe {
    let mut pMemoryRequirements: VkMemoryRequirements = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetBufferMemoryRequirements.unwrap()(
        device.as_raw(),
        buffer.as_raw(),
        (&mut pMemoryRequirements).as_raw(),
      );
      return pMemoryRequirements;
    })
  }
}

/// Returns the memory requirements for specified Vulkan object
///
/// To determine the memory requirements for an image resource which is not created
/// with the `VK_IMAGE_CREATE_DISJOINT_BIT_KHR` flag set.
///
///   - `device` is the logical device that owns the image.
///
///   - `image` is the image to query.
///
///   - `pMemoryRequirements` points to an instance of the `VkMemoryRequirements`
///     structure in which the memory requirements of the image object are returned.
///
pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage) -> VkMemoryRequirements {
  unsafe {
    let mut pMemoryRequirements: VkMemoryRequirements = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetImageMemoryRequirements.unwrap()(
        device.as_raw(),
        image.as_raw(),
        (&mut pMemoryRequirements).as_raw(),
      );
      return pMemoryRequirements;
    })
  }
}

/// Query the memory requirements for a sparse image
///
///   - `device` is the logical device that owns the image.
///
///   - `image` is the `VkImage` object to get the memory requirements for.
///
///   - `pSparseMemoryRequirementCount` is a pointer to an integer related to the
///     number of sparse memory requirements available or queried, as described
///     below.
///
///   - `pSparseMemoryRequirements` is either `NULL` or a pointer to an array of
///     `VkSparseImageMemoryRequirements` structures.
///
/// If `pSparseMemoryRequirements` is `NULL`, then the number of sparse memory
/// requirements available is returned in `pSparseMemoryRequirementCount`.
/// Otherwise, `pSparseMemoryRequirementCount` must: point to a variable set by the
/// user to the number of elements in the `pSparseMemoryRequirements` array, and on
/// return the variable is overwritten with the number of structures actually
/// written to `pSparseMemoryRequirements`. If `pSparseMemoryRequirementCount` is
/// less than the number of sparse memory requirements available, at most
/// `pSparseMemoryRequirementCount` structures will be written.
///
/// If the image was not created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` then
/// `pSparseMemoryRequirementCount` will be set to zero and
/// `pSparseMemoryRequirements` will not be written to.
///
/// > **Note**
/// >
/// > It is legal for an implementation to report a larger value in
/// > `VkMemoryRequirements::size` than would be obtained by adding together memory
/// > sizes for all `VkSparseImageMemoryRequirements` returned by
/// > `vkGetImageSparseMemoryRequirements`. This may: occur when the hardware
/// > requires unused padding in the address range describing the resource.
///
pub fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage) -> Vec<VkSparseImageMemoryRequirements> {
  unsafe {
    let mut pSparseMemoryRequirementCount: u32 = 0;
    let mut pSparseMemoryRequirements: Vec<VkSparseImageMemoryRequirements> = Vec::new();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetImageSparseMemoryRequirements.unwrap()(
        device.as_raw(),
        image.as_raw(),
        &mut pSparseMemoryRequirementCount,
        ::std::ptr::null_mut(),
      );
      if pSparseMemoryRequirementCount == 0 {
        return pSparseMemoryRequirements;
      }
      pSparseMemoryRequirements = Vec::with_capacity(pSparseMemoryRequirementCount as usize);
      _t.vkGetImageSparseMemoryRequirements.unwrap()(
        device.as_raw(),
        image.as_raw(),
        &mut pSparseMemoryRequirementCount,
        pSparseMemoryRequirements.as_mut_slice().as_raw(),
      );
      pSparseMemoryRequirements.set_len(pSparseMemoryRequirementCount as usize);
      return pSparseMemoryRequirements;
    })
  }
}

/// Retrieve properties of an image format applied to sparse images
///
/// `vkGetPhysicalDeviceSparseImageFormatProperties` returns an array of
/// `VkSparseImageFormatProperties`. Each element will describe properties for one
/// set of image aspects that are bound simultaneously in the image. This is usually
/// one element for each aspect in the image, but for interleaved depth/stencil
/// images there is only one element describing the combined aspects.
///
///   - `physicalDevice` is the physical device from which to query the sparse image
///     capabilities.
///
///   - `format` is the image format.
///
///   - `type` is the dimensionality of image.
///
///   - `samples` is the number of samples per texel as defined in
///     `VkSampleCountFlagBits`.
///
///   - `usage` is a bitmask describing the intended usage of the image.
///
///   - `tiling` is the tiling arrangement of the data elements in memory.
///
///   - `pPropertyCount` is a pointer to an integer related to the number of sparse
///     format properties available or queried, as described below.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkSparseImageFormatProperties` structures.
///
/// If `pProperties` is `NULL`, then the number of sparse format properties
/// available is returned in `pPropertyCount`. Otherwise, `pPropertyCount` must:
/// point to a variable set by the user to the number of elements in the
/// `pProperties` array, and on return the variable is overwritten with the number
/// of structures actually written to `pProperties`. If `pPropertyCount` is less
/// than the number of sparse format properties available, at most `pPropertyCount`
/// structures will be written.
///
/// If `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` is not supported for the given
/// arguments, `pPropertyCount` will be set to zero upon return, and no data will be
/// written to `pProperties`.
///
/// Multiple aspects are returned for depth/stencil images that are implemented as
/// separate planes by the implementation. The depth and stencil data planes each
/// have unique `VkSparseImageFormatProperties` data.
///
/// Depth/stencil images with depth and stencil data interleaved into a single plane
/// will return a single `VkSparseImageFormatProperties` structure with the
/// `aspectMask` set to `VK_IMAGE_ASPECT_DEPTH_BIT` | `VK_IMAGE_ASPECT_STENCIL_BIT`.
///
pub fn vkGetPhysicalDeviceSparseImageFormatProperties(
  physicalDevice: VkPhysicalDevice,
  format: VkFormat,
  eType: VkImageType,
  samples: VkSampleCountFlagBits,
  usage: VkImageUsageFlags,
  tiling: VkImageTiling,
) -> Vec<VkSparseImageFormatProperties> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkSparseImageFormatProperties> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceSparseImageFormatProperties.unwrap()(
        physicalDevice.as_raw(),
        format,
        eType,
        samples,
        usage,
        tiling,
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if pPropertyCount == 0 {
        return pProperties;
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      _t.vkGetPhysicalDeviceSparseImageFormatProperties.unwrap()(
        physicalDevice.as_raw(),
        format,
        eType,
        samples,
        usage,
        tiling,
        &mut pPropertyCount,
        pProperties.as_mut_slice().as_raw(),
      );
      pProperties.set_len(pPropertyCount as usize);
      return pProperties;
    })
  }
}

/// Bind device memory to a sparse resource object
///
///   - `queue` is the queue that the sparse binding operations will be submitted
///     to.
///
///   - `bindInfoCount` is the number of elements in the `pBindInfo` array.
///
///   - `pBindInfo` is an array of `VkBindSparseInfo` structures, each specifying a
///     sparse binding submission batch.
///
///   - `fence` is an optional: handle to a fence to be signaled. If `fence` is not
///     `VK_NULL_HANDLE`, it defines a [fence signal
///     operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-fences-signaling).
///
/// `vkQueueBindSparse` is a [queue submission command](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-submission),
/// with each batch defined by an element of `pBindInfo` as an instance of the
/// `VkBindSparseInfo` structure. Batches begin execution in the order they appear
/// in `pBindInfo`, but may: complete out of order.
///
/// Within a batch, a given range of a resource must: not be bound more than once.
/// Across batches, if a range is to be bound to one allocation and offset and then
/// to another allocation and offset, then the application must: guarantee (usually
/// using semaphores) that the binding operations are executed in the correct order,
/// as well as to order binding operations against the execution of command buffer
/// submissions.
///
/// As no operation to `vkQueueBindSparse` causes any pipeline stage to access
/// memory, synchronization primitives used in this command effectively only define
/// execution dependencies.
///
/// Additional information about fence and semaphore operation is described in [the
/// synchronization chapter](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization).
///
pub fn vkQueueBindSparse(queue: VkQueue, pBindInfo: &[VkBindSparseInfo], fence: Option<VkFence>) -> VkResult {
  unsafe {
    let bindInfoCount = pBindInfo.len() as u32;
    VkDeviceDispatchTable::with(queue, |_t| {
      _t.vkQueueBindSparse.unwrap()(
        queue.as_raw(),
        bindInfoCount,
        pBindInfo.as_raw(),
        fence.as_raw(),
      )
    })
  }
}

/// Create a new fence object
///
///   - `device` is the logical device that creates the fence.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkFenceCreateInfo`
///     structure which contains information about how the fence is to be created.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pFence` points to a handle in which the resulting fence object is returned.
///
pub fn vkCreateFence(
  device: VkDevice,
  pCreateInfo: &VkFenceCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkFence, VkResult> {
  unsafe {
    let mut pFence: VkFence = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateFence.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pFence).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pFence);
    })
  }
}

/// Destroy a fence object
///
///   - `device` is the logical device that destroys the fence.
///
///   - `fence` is the handle of the fence to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyFence(device: VkDevice, fence: Option<VkFence>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyFence.unwrap()(device.as_raw(), fence.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Resets one or more fence objects
///
///   - `device` is the logical device that owns the fences.
///
///   - `fenceCount` is the number of fences to reset.
///
///   - `pFences` is a pointer to an array of fence handles to reset.
///
/// If any member of `pFences` currently has its [payload
/// imported](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-fences-importing) with temporary permanence, that
/// fence’s prior permanent payload is first restored. The remaining operations
/// described therefore operate on the restored payload.
///
/// When `vkResetFences` is executed on the host, it defines a *fence unsignal
/// operation* for each fence, which resets the fence to the unsignaled state.
///
/// If any member of `pFences` is already in the unsignaled state when
/// `vkResetFences` is executed, then `vkResetFences` has no effect on that fence.
///
pub fn vkResetFences(device: VkDevice, pFences: &[VkFence]) -> VkResult {
  unsafe {
    let fenceCount = pFences.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetFences.unwrap()(device.as_raw(), fenceCount, pFences.as_raw())
    })
  }
}

/// Return the status of a fence
///
///   - `device` is the logical device that owns the fence.
///
///   - `fence` is the handle of the fence to query.
///
/// Upon success, `vkGetFenceStatus` returns the status of the fence object, with
/// the following return codes:
///
/// <table>
/// <caption>Fence Object Status Codes</caption>
/// <colgroup>
/// <col width="50%" />
/// <col width="50%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Status</th>
/// <th align="left">Meaning</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"><p><code>VK_SUCCESS</code></p></td>
/// <td align="left"><p>The fence specified by <code>fence</code> is signaled.</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><code>VK_NOT_READY</code></p></td>
/// <td align="left"><p>The fence specified by <code>fence</code> is unsignaled.</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p><code>VK_ERROR_DEVICE_LOST</code></p></td>
/// <td align="left"><p>The device has been lost. See <a href="#devsandqueues-lost-device">Lost Device</a>.</p></td>
/// </tr>
/// </tbody>
/// </table>
///
/// If a [queue submission](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-submission) command is pending execution,
/// then the value returned by this command may: immediately be out of date.
///
/// If the device has been lost (see [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-lost-device)),
/// `vkGetFenceStatus` may: return any of the above status codes. If the device has
/// been lost and `vkGetFenceStatus` is called repeatedly, it will eventually return
/// either `VK_SUCCESS` or `VK_ERROR_DEVICE_LOST`.
///
pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetFenceStatus.unwrap()(device.as_raw(), fence.as_raw())
    })
  }
}

/// Wait for one or more fences to become signaled
///
/// To wait for one or more fences to enter the signaled state on the host, call.
///
///   - `device` is the logical device that owns the fences.
///
///   - `fenceCount` is the number of fences to wait on.
///
///   - `pFences` is a pointer to an array of `fenceCount` fence handles.
///
///   - `waitAll` is the condition that must: be satisfied to successfully unblock
///     the wait. If `waitAll` is `VK_TRUE`, then the condition is that all fences
///     in `pFences` are signaled. Otherwise, the condition is that at least one
///     fence in `pFences` is signaled.
///
///   - `timeout` is the timeout period in units of nanoseconds. `timeout` is
///     adjusted to the closest value allowed by the implementation-dependent
///     timeout accuracy, which may: be substantially longer than one nanosecond,
///     and may: be longer than the requested period.
///
/// If the condition is satisfied when `vkWaitForFences` is called, then
/// `vkWaitForFences` returns immediately. If the condition is not satisfied at the
/// time `vkWaitForFences` is called, then `vkWaitForFences` will block and wait up
/// to `timeout` nanoseconds for the condition to become satisfied.
///
/// If `timeout` is zero, then `vkWaitForFences` does not wait, but simply returns
/// the current state of the fences. `VK_TIMEOUT` will be returned in this case if
/// the condition is not satisfied, even though no actual wait was performed.
///
/// If the specified timeout period expires before the condition is satisfied,
/// `vkWaitForFences` returns `VK_TIMEOUT`. If the condition is satisfied before
/// `timeout` nanoseconds has expired, `vkWaitForFences` returns `VK_SUCCESS`.
///
/// If device loss occurs (see [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-lost-device)) before the
/// timeout has expired, `vkWaitForFences` must: return in finite time with either
/// `VK_SUCCESS` or `VK_ERROR_DEVICE_LOST`.
///
/// > **Note**
/// >
/// > While we guarantee that `vkWaitForFences` must: return in finite time, no
/// > guarantees are made that it returns immediately upon device loss. However, the
/// > client can reasonably expect that the delay will be on the order of seconds
/// > and that calling `vkWaitForFences` will not result in a permanently (or
/// > seemingly permanently) dead process.
///
pub fn vkWaitForFences(device: VkDevice, pFences: &[VkFence], waitAll: VkBool32, timeout: u64) -> VkResult {
  unsafe {
    let fenceCount = pFences.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkWaitForFences.unwrap()(
        device.as_raw(),
        fenceCount,
        pFences.as_raw(),
        waitAll,
        timeout,
      )
    })
  }
}

/// Create a new queue semaphore object
///
///   - `device` is the logical device that creates the semaphore.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkSemaphoreCreateInfo`
///     structure which contains information about how the semaphore is to be
///     created.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pSemaphore` points to a handle in which the resulting semaphore object is
///     returned.
///
/// When created, the semaphore is in the unsignaled state.
///
pub fn vkCreateSemaphore(
  device: VkDevice,
  pCreateInfo: &VkSemaphoreCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSemaphore, VkResult> {
  unsafe {
    let mut pSemaphore: VkSemaphore = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateSemaphore.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSemaphore).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSemaphore);
    })
  }
}

/// Destroy a semaphore object
///
///   - `device` is the logical device that destroys the semaphore.
///
///   - `semaphore` is the handle of the semaphore to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroySemaphore(
  device: VkDevice,
  semaphore: Option<VkSemaphore>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySemaphore.unwrap()(device.as_raw(), semaphore.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new event object
///
///   - `device` is the logical device that creates the event.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkEventCreateInfo`
///     structure which contains information about how the event is to be created.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pEvent` points to a handle in which the resulting event object is returned.
///
/// When created, the event object is in the unsignaled state.
///
pub fn vkCreateEvent(
  device: VkDevice,
  pCreateInfo: &VkEventCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkEvent, VkResult> {
  unsafe {
    let mut pEvent: VkEvent = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateEvent.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pEvent).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pEvent);
    })
  }
}

/// Destroy an event object
///
///   - `device` is the logical device that destroys the event.
///
///   - `event` is the handle of the event to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyEvent(device: VkDevice, event: Option<VkEvent>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyEvent.unwrap()(device.as_raw(), event.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Retrieve the status of an event object
///
///   - `device` is the logical device that owns the event.
///
///   - `event` is the handle of the event to query.
///
/// Upon success, `vkGetEventStatus` returns the state of the event object with the
/// following return codes:
///
/// <table>
/// <caption>Event Object Status Codes</caption>
/// <colgroup>
/// <col width="50%" />
/// <col width="50%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Status</th>
/// <th align="left">Meaning</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"><p><code>VK_EVENT_SET</code></p></td>
/// <td align="left"><p>The event specified by <code>event</code> is signaled.</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><code>VK_EVENT_RESET</code></p></td>
/// <td align="left"><p>The event specified by <code>event</code> is unsignaled.</p></td>
/// </tr>
/// </tbody>
/// </table>
///
/// If a `vkCmdSetEvent` or `vkCmdResetEvent` command is in a command buffer that is
/// in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle), then the value returned by
/// this command may: immediately be out of date.
///
/// The state of an event can: be updated by the host. The state of the event is
/// immediately changed, and subsequent calls to `vkGetEventStatus` will return the
/// new state. If an event is already in the requested state, then updating it to
/// the same state has no effect.
///
pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetEventStatus.unwrap()(device.as_raw(), event.as_raw())
    })
  }
}

/// Set an event to signaled state
///
///   - `device` is the logical device that owns the event.
///
///   - `event` is the event to set.
///
/// When `vkSetEvent` is executed on the host, it defines an *event signal
/// operation* which sets the event to the signaled state.
///
/// If `event` is already in the signaled state when `vkSetEvent` is executed, then
/// `vkSetEvent` has no effect, and no event signal operation occurs.
///
pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkSetEvent.unwrap()(device.as_raw(), event.as_raw())
    })
  }
}

/// Reset an event to non-signaled state
///
///   - `device` is the logical device that owns the event.
///
///   - `event` is the event to reset.
///
/// When `vkResetEvent` is executed on the host, it defines an *event unsignal
/// operation* which resets the event to the unsignaled state.
///
/// If `event` is already in the unsignaled state when `vkResetEvent` is executed,
/// then `vkResetEvent` has no effect, and no event unsignal operation occurs.
///
pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetEvent.unwrap()(device.as_raw(), event.as_raw())
    })
  }
}

/// Create a new query pool object
///
///   - `device` is the logical device that creates the query pool.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkQueryPoolCreateInfo`
///     structure containing the number and type of queries to be managed by the
///     pool.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pQueryPool` is a pointer to a `VkQueryPool` handle in which the resulting
///     query pool object is returned.
///
pub fn vkCreateQueryPool(
  device: VkDevice,
  pCreateInfo: &VkQueryPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkQueryPool, VkResult> {
  unsafe {
    let mut pQueryPool: VkQueryPool = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateQueryPool.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pQueryPool).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pQueryPool);
    })
  }
}

/// Destroy a query pool object
///
///   - `device` is the logical device that destroys the query pool.
///
///   - `queryPool` is the query pool to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyQueryPool(
  device: VkDevice,
  queryPool: Option<VkQueryPool>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyQueryPool.unwrap()(device.as_raw(), queryPool.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Copy results of queries in a query pool to a host memory region
///
///   - `device` is the logical device that owns the query pool.
///
///   - `queryPool` is the query pool managing the queries containing the desired
///     results.
///
///   - `firstQuery` is the initial query index.
///
///   - `queryCount` is the number of queries. `firstQuery` and `queryCount`
///     together define a range of queries. For pipeline statistics queries, each
///     query index in the pool contains one integer value for each bit that is
///     enabled in `VkQueryPoolCreateInfo::pipelineStatistics` when the pool is
///     created.
///
///   - `dataSize` is the size in bytes of the buffer pointed to by `pData`.
///
///   - `pData` is a pointer to a user-allocated buffer where the results will be
///     written
///
///   - `stride` is the stride in bytes between results for individual queries
///     within `pData`.
///
///   - `flags` is a bitmask of `VkQueryResultFlagBits` specifying how and when
///     results are returned.
///
/// If no bits are set in `flags`, and all requested queries are in the available
/// state, results are written as an array of 32-bit unsigned integer values. The
/// behavior when not all queries are available, is described
/// [below](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-wait-bit-not-set).
///
/// If `VK_QUERY_RESULT_64_BIT` is not set and the result overflows a 32-bit value,
/// the value may: either wrap or saturate. Similarly, if `VK_QUERY_RESULT_64_BIT`
/// is set and the result overflows a 64-bit value, the value may: either wrap or
/// saturate.
///
/// If `VK_QUERY_RESULT_WAIT_BIT` is set, Vulkan will wait for each query to be in
/// the available state before retrieving the numerical results for that query. In
/// this case, `vkGetQueryPoolResults` is guaranteed to succeed and return
/// `VK_SUCCESS` if the queries become available in a finite time (i.e. if they have
/// been issued and not reset). If queries will never finish (e.g. due to being
/// reset but not issued), then `vkGetQueryPoolResults` may: not return in finite
/// time.
///
/// If `VK_QUERY_RESULT_WAIT_BIT` and `VK_QUERY_RESULT_PARTIAL_BIT` are both not set
/// then no result values are written to `pData` for queries that are in the
/// unavailable state at the time of the call, and `vkGetQueryPoolResults` returns
/// `VK_NOT_READY`. However, availability state is still written to `pData` for
/// those queries if `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` is set.
///
/// > **Note**
/// >
/// > Applications must: take care to ensure that use of the
/// > `VK_QUERY_RESULT_WAIT_BIT` bit has the desired effect.
/// >
/// > For example, if a query has been used previously and a command buffer records
/// > the commands `vkCmdResetQueryPool`, `vkCmdBeginQuery`, and `vkCmdEndQuery` for
/// > that query, then the query will remain in the available state until the
/// > `vkCmdResetQueryPool` command executes on a queue. Applications can: use
/// > fences or events to ensure that a query has already been reset before checking
/// > for its results or availability status. Otherwise, a stale value could be
/// > returned from a previous use of the query.
/// >
/// > The above also applies when `VK_QUERY_RESULT_WAIT_BIT` is used in combination
/// > with `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT`. In this case, the returned
/// > availability status may: reflect the result of a previous use of the query
/// > unless the `vkCmdResetQueryPool` command has been executed since the last use
/// > of the query.
///
/// > **Note**
/// >
/// > Applications can: double-buffer query pool usage, with a pool per frame, and
/// > reset queries at the end of the frame in which they are read.
///
/// If `VK_QUERY_RESULT_PARTIAL_BIT` is set, `VK_QUERY_RESULT_WAIT_BIT` is not set,
/// and the query’s status is unavailable, an intermediate result value between zero
/// and the final result value is written to `pData` for that query.
///
/// `VK_QUERY_RESULT_PARTIAL_BIT` must: not be used if the pool’s `queryType` is
/// `VK_QUERY_TYPE_TIMESTAMP`.
///
/// If `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` is set, the final integer value
/// written for each query is non-zero if the query’s status was available or zero
/// if the status was unavailable. When `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` is
/// used, implementations must: guarantee that if they return a non-zero
/// availability value then the numerical results must: be valid, assuming the
/// results are not reset by a subsequent command.
///
/// > **Note**
/// >
/// > Satisfying this guarantee may: require careful ordering by the application,
/// > e.g. to read the availability status before reading the results.
///
pub fn vkGetQueryPoolResults(
  device: VkDevice,
  queryPool: VkQueryPool,
  firstQuery: u32,
  queryCount: u32,
  pData: &mut [u8],
  stride: VkDeviceSize,
  flags: VkQueryResultFlags,
) -> VkResult {
  unsafe {
    let dataSize = pData.len() as usize;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetQueryPoolResults.unwrap()(
        device.as_raw(),
        queryPool.as_raw(),
        firstQuery,
        queryCount,
        dataSize,
        pData.as_raw() as *mut c_void,
        stride,
        flags,
      )
    })
  }
}

/// Create a new buffer object
///
///   - `device` is the logical device that creates the buffer object.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkBufferCreateInfo`
///     structure containing parameters affecting creation of the buffer.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pBuffer` points to a `VkBuffer` handle in which the resulting buffer object
///     is returned.
///
pub fn vkCreateBuffer(
  device: VkDevice,
  pCreateInfo: &VkBufferCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkBuffer, VkResult> {
  unsafe {
    let mut pBuffer: VkBuffer = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateBuffer.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pBuffer).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pBuffer);
    })
  }
}

/// Destroy a buffer object
///
///   - `device` is the logical device that destroys the buffer.
///
///   - `buffer` is the buffer to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyBuffer(device: VkDevice, buffer: Option<VkBuffer>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyBuffer.unwrap()(device.as_raw(), buffer.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new buffer view object
///
///   - `device` is the logical device that creates the buffer view.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkBufferViewCreateInfo`
///     structure containing parameters to be used to create the buffer.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pView` points to a `VkBufferView` handle in which the resulting buffer view
///     object is returned.
///
pub fn vkCreateBufferView(
  device: VkDevice,
  pCreateInfo: &VkBufferViewCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkBufferView, VkResult> {
  unsafe {
    let mut pView: VkBufferView = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateBufferView.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pView).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pView);
    })
  }
}

/// Destroy a buffer view object
///
///   - `device` is the logical device that destroys the buffer view.
///
///   - `bufferView` is the buffer view to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyBufferView(
  device: VkDevice,
  bufferView: Option<VkBufferView>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyBufferView.unwrap()(device.as_raw(), bufferView.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new image object
///
///   - `device` is the logical device that creates the image.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkImageCreateInfo`
///     structure containing parameters to be used to create the image.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pImage` points to a `VkImage` handle in which the resulting image object is
///     returned.
///
pub fn vkCreateImage(
  device: VkDevice,
  pCreateInfo: &VkImageCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkImage, VkResult> {
  unsafe {
    let mut pImage: VkImage = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateImage.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pImage).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pImage);
    })
  }
}

/// Destroy an image object
///
///   - `device` is the logical device that destroys the image.
///
///   - `image` is the image to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyImage(device: VkDevice, image: Option<VkImage>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyImage.unwrap()(device.as_raw(), image.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Retrieve information about an image subresource
///
/// To query the host access layout of an image subresource, for an image created
/// with linear tiling.
///
///   - `device` is the logical device that owns the image.
///
///   - `image` is the image whose layout is being queried.
///
///   - `pSubresource` is a pointer to a `VkImageSubresource` structure selecting a
///     specific image for the image subresource.
///
///   - `pLayout` points to a `VkSubresourceLayout` structure in which the layout is
///     returned.
///
/// If the `VkFormat` of `image` is a [multi-planar
/// format](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion),
/// `vkGetImageSubresourceLayout` describes one plane of the image.
///
/// `vkGetImageSubresourceLayout` is invariant for the lifetime of a single image.
///
pub fn vkGetImageSubresourceLayout(
  device: VkDevice,
  image: VkImage,
  pSubresource: &VkImageSubresource,
) -> VkSubresourceLayout {
  unsafe {
    let mut pLayout: VkSubresourceLayout = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetImageSubresourceLayout.unwrap()(
        device.as_raw(),
        image.as_raw(),
        pSubresource.as_raw(),
        (&mut pLayout).as_raw(),
      );
      return pLayout;
    })
  }
}

/// Create an image view from an existing image
///
///   - `device` is the logical device that creates the image view.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkImageViewCreateInfo`
///     structure containing parameters to be used to create the image view.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pView` points to a `VkImageView` handle in which the resulting image view
///     object is returned.
///
/// Some of the image creation parameters are inherited by the view. In particular,
/// image view creation inherits the implicit parameter `usage` specifying the
/// allowed usages of the image view that, by default, takes the value of the
/// corresponding `usage` parameter specified in `VkImageCreateInfo` at image
/// creation time. This implicit parameter can: be overriden by chaining a
/// `VkImageViewUsageCreateInfoKHR` structure through the `pNext` member to
/// `VkImageViewCreateInfo` as described later in this section.
///
/// The remaining parameters are contained in the `pCreateInfo`.
///
pub fn vkCreateImageView(
  device: VkDevice,
  pCreateInfo: &VkImageViewCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkImageView, VkResult> {
  unsafe {
    let mut pView: VkImageView = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateImageView.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pView).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pView);
    })
  }
}

/// Destroy an image view object
///
///   - `device` is the logical device that destroys the image view.
///
///   - `imageView` is the image view to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyImageView(
  device: VkDevice,
  imageView: Option<VkImageView>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyImageView.unwrap()(device.as_raw(), imageView.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Creates a new shader module object
///
///   - `device` is the logical device that creates the shader module.
///
///   - `pCreateInfo` parameter is a pointer to an instance of the
///     `VkShaderModuleCreateInfo` structure.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pShaderModule` points to a `VkShaderModule` handle in which the resulting
///     shader module object is returned.
///
/// Once a shader module has been created, any entry points it contains can: be used
/// in pipeline shader stages as described in [Compute
/// Pipelines](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-compute) and [Graphics Pipelines](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-graphics).
///
/// If the shader stage fails to compile `VK_ERROR_INVALID_SHADER_NV` will be
/// generated and the compile log will be reported back to the application by  if
/// enabled.
///
pub fn vkCreateShaderModule(
  device: VkDevice,
  pCreateInfo: &VkShaderModuleCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkShaderModule, VkResult> {
  unsafe {
    let mut pShaderModule: VkShaderModule = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateShaderModule.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pShaderModule).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pShaderModule);
    })
  }
}

/// Destroy a shader module module
///
///   - `device` is the logical device that destroys the shader module.
///
///   - `shaderModule` is the handle of the shader module to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
/// A shader module can: be destroyed while pipelines created using its shaders are
/// still in use.
///
pub fn vkDestroyShaderModule(
  device: VkDevice,
  shaderModule: Option<VkShaderModule>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyShaderModule.unwrap()(device.as_raw(), shaderModule.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Creates a new pipeline cache
///
///   - `device` is the logical device that creates the pipeline cache object.
///
///   - `pCreateInfo` is a pointer to a `VkPipelineCacheCreateInfo` structure that
///     contains the initial parameters for the pipeline cache object.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pPipelineCache` is a pointer to a `VkPipelineCache` handle in which the
///     resulting pipeline cache object is returned.
///
/// > **Note**
/// >
/// > Applications can: track and manage the total host memory size of a pipeline
/// > cache object using the `pAllocator`. Applications can: limit the amount of
/// > data retrieved from a pipeline cache object in `vkGetPipelineCacheData`.
/// > Implementations should: not internally limit the total number of entries added
/// > to a pipeline cache object or the total host memory consumed.
///
/// Once created, a pipeline cache can: be passed to the `vkCreateGraphicsPipelines`
/// and `vkCreateComputePipelines` commands. If the pipeline cache passed into these
/// commands is not `VK_NULL_HANDLE`, the implementation will query it for possible
/// reuse opportunities and update it with new content. The use of the pipeline
/// cache object in these commands is internally synchronized, and the same pipeline
/// cache object can: be used in multiple threads simultaneously.
///
/// > **Note**
/// >
/// > Implementations should: make every effort to limit any critical sections to
/// > the actual accesses to the cache, which is expected to be significantly
/// > shorter than the duration of the `vkCreateGraphicsPipelines` and
/// > `vkCreateComputePipelines` commands.
///
pub fn vkCreatePipelineCache(
  device: VkDevice,
  pCreateInfo: &VkPipelineCacheCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkPipelineCache, VkResult> {
  unsafe {
    let mut pPipelineCache: VkPipelineCache = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreatePipelineCache.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pPipelineCache).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pPipelineCache);
    })
  }
}

/// Destroy a pipeline cache object
///
///   - `device` is the logical device that destroys the pipeline cache object.
///
///   - `pipelineCache` is the handle of the pipeline cache to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyPipelineCache(
  device: VkDevice,
  pipelineCache: Option<VkPipelineCache>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyPipelineCache.unwrap()(device.as_raw(), pipelineCache.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Get the data store from a pipeline cache
///
/// Data can: be retrieved from a pipeline cache object using the command.
///
///   - `device` is the logical device that owns the pipeline cache.
///
///   - `pipelineCache` is the pipeline cache to retrieve data from.
///
///   - `pDataSize` is a pointer to a value related to the amount of data in the
///     pipeline cache, as described below.
///
///   - `pData` is either `NULL` or a pointer to a buffer.
///
/// If `pData` is `NULL`, then the maximum size of the data that can: be retrieved
/// from the pipeline cache, in bytes, is returned in `pDataSize`. Otherwise,
/// `pDataSize` must: point to a variable set by the user to the size of the buffer,
/// in bytes, pointed to by `pData`, and on return the variable is overwritten with
/// the amount of data actually written to `pData`.
///
/// If `pDataSize` is less than the maximum size that can: be retrieved by the
/// pipeline cache, at most `pDataSize` bytes will be written to `pData`, and
/// `vkGetPipelineCacheData` will return `VK_INCOMPLETE`. Any data written to
/// `pData` is valid and can: be provided as the `pInitialData` member of the
/// `VkPipelineCacheCreateInfo` structure passed to `vkCreatePipelineCache`.
///
/// Two calls to `vkGetPipelineCacheData` with the same parameters must: retrieve
/// the same data unless a command that modifies the contents of the cache is called
/// between them.
///
/// Applications can: store the data retrieved from the pipeline cache, and use
/// these data, possibly in a future run of the application, to populate new
/// pipeline cache objects. The results of pipeline compiles, however, may: depend
/// on the vendor ID, device ID, driver version, and other details of the device. To
/// enable applications to detect when previously retrieved data is incompatible
/// with the device, the initial bytes written to `pData` must: be a header
/// consisting of the following members:
///
/// <table>
/// <caption>Layout for pipeline cache header version <code>VK_PIPELINE_CACHE_HEADER_VERSION_ONE</code></caption>
/// <colgroup>
/// <col width="8%" />
/// <col width="21%" />
/// <col width="70%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Offset</th>
/// <th align="left">Size</th>
/// <th align="left">Meaning</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"><p>0</p></td>
/// <td align="left"><p>4</p></td>
/// <td align="left"><p>length in bytes of the entire pipeline cache header written as a stream of bytes, with the least significant byte first</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p>4</p></td>
/// <td align="left"><p>4</p></td>
/// <td align="left"><p>a <code>VkPipelineCacheHeaderVersion</code> value written as a stream of bytes, with the least significant byte first</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p>8</p></td>
/// <td align="left"><p>4</p></td>
/// <td align="left"><p>a vendor ID equal to <code>VkPhysicalDeviceProperties::vendorID</code> written as a stream of bytes, with the least significant byte first</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p>12</p></td>
/// <td align="left"><p>4</p></td>
/// <td align="left"><p>a device ID equal to <code>VkPhysicalDeviceProperties::deviceID</code> written as a stream of bytes, with the least significant byte first</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p>16</p></td>
/// <td align="left"><p><code>VK_UUID_SIZE</code></p></td>
/// <td align="left"><p>a pipeline cache ID equal to <code>VkPhysicalDeviceProperties::pipelineCacheUUID</code></p></td>
/// </tr>
/// </tbody>
/// </table>
///
/// The first four bytes encode the length of the entire pipeline cache header, in
/// bytes. This value includes all fields in the header including the pipeline cache
/// version field and the size of the length field.
///
/// The next four bytes encode the pipeline cache version, as described for
/// `VkPipelineCacheHeaderVersion`. A consumer of the pipeline cache should: use the
/// cache version to interpret the remainder of the cache header.
///
/// If `pDataSize` is less than what is necessary to store this header, nothing will
/// be written to `pData` and zero will be written to `pDataSize`.
///
pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache) -> Result<Vec<u8>, VkResult> {
  unsafe {
    let mut pDataSize: usize = 0;
    let mut pData: Vec<u8> = Vec::new();
    VkDeviceDispatchTable::with(device, |_t| loop {
      let _r = _t.vkGetPipelineCacheData.unwrap()(
        device.as_raw(),
        pipelineCache.as_raw(),
        &mut pDataSize,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pDataSize == 0 {
        return Ok(pData);
      }
      pData = Vec::with_capacity(pDataSize as usize);
      let _r = _t.vkGetPipelineCacheData.unwrap()(
        device.as_raw(),
        pipelineCache.as_raw(),
        &mut pDataSize,
        pData.as_mut_slice().as_raw() as *mut c_void,
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pData.set_len(pDataSize as usize);
      return Ok(pData);
    })
  }
}

/// Combine the data stores of pipeline caches
///
/// Pipeline cache objects can: be merged using the command.
///
///   - `device` is the logical device that owns the pipeline cache objects.
///
///   - `dstCache` is the handle of the pipeline cache to merge results into.
///
///   - `srcCacheCount` is the length of the `pSrcCaches` array.
///
///   - `pSrcCaches` is an array of pipeline cache handles, which will be merged
///     into `dstCache`. The previous contents of `dstCache` are included after the
///     merge.
///
/// > **Note**
/// >
/// > The details of the merge operation are implementation dependent, but
/// > implementations should: merge the contents of the specified pipelines and
/// > prune duplicate entries.
///
pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, pSrcCaches: &[VkPipelineCache]) -> VkResult {
  unsafe {
    let srcCacheCount = pSrcCaches.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkMergePipelineCaches.unwrap()(
        device.as_raw(),
        dstCache.as_raw(),
        srcCacheCount,
        pSrcCaches.as_raw(),
      )
    })
  }
}

/// Create graphics pipelines
///
///   - `device` is the logical device that creates the graphics pipelines.
///
///   - `pipelineCache` is either `VK_NULL_HANDLE`, indicating that pipeline caching
///     is disabled; or the handle of a valid [pipeline cache](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-cache)
///     object, in which case use of that cache is enabled for the duration of the
///     command.
///
///   - `createInfoCount` is the length of the `pCreateInfos` and `pPipelines`
///     arrays.
///
///   - `pCreateInfos` is an array of `VkGraphicsPipelineCreateInfo` structures.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pPipelines` is a pointer to an array in which the resulting graphics
///     pipeline objects are returned.
///
/// The `VkGraphicsPipelineCreateInfo` structure includes an array of shader create
/// info structures containing all the desired active shader stages, as well as
/// creation info to define all relevant fixed-function stages, and a pipeline
/// layout.
///
pub fn vkCreateGraphicsPipelines(
  device: VkDevice,
  pipelineCache: Option<VkPipelineCache>,
  pCreateInfos: &[VkGraphicsPipelineCreateInfo],
  pAllocator: Option<&VkAllocationCallbacks>,
  pPipelines: &mut [VkPipeline],
) -> VkResult {
  unsafe {
    let createInfoCount = pCreateInfos.len() as u32;
    assert!(createInfoCount as usize == pPipelines.len());
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkCreateGraphicsPipelines.unwrap()(
        device.as_raw(),
        pipelineCache.as_raw(),
        createInfoCount,
        pCreateInfos.as_raw(),
        pAllocator.as_raw(),
        pPipelines.as_raw(),
      )
    })
  }
}

/// Creates a new compute pipeline object
///
///   - `device` is the logical device that creates the compute pipelines.
///
///   - `pipelineCache` is either `VK_NULL_HANDLE`, indicating that pipeline caching
///     is disabled; or the handle of a valid [pipeline cache](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-cache)
///     object, in which case use of that cache is enabled for the duration of the
///     command.
///
///   - `createInfoCount` is the length of the `pCreateInfos` and `pPipelines`
///     arrays.
///
///   - `pCreateInfos` is an array of `VkComputePipelineCreateInfo` structures.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pPipelines` is a pointer to an array in which the resulting compute
///     pipeline objects are returned.
///
pub fn vkCreateComputePipelines(
  device: VkDevice,
  pipelineCache: Option<VkPipelineCache>,
  pCreateInfos: &[VkComputePipelineCreateInfo],
  pAllocator: Option<&VkAllocationCallbacks>,
  pPipelines: &mut [VkPipeline],
) -> VkResult {
  unsafe {
    let createInfoCount = pCreateInfos.len() as u32;
    assert!(createInfoCount as usize == pPipelines.len());
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkCreateComputePipelines.unwrap()(
        device.as_raw(),
        pipelineCache.as_raw(),
        createInfoCount,
        pCreateInfos.as_raw(),
        pAllocator.as_raw(),
        pPipelines.as_raw(),
      )
    })
  }
}

/// Destroy a pipeline object
///
///   - `device` is the logical device that destroys the pipeline.
///
///   - `pipeline` is the handle of the pipeline to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyPipeline(device: VkDevice, pipeline: Option<VkPipeline>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyPipeline.unwrap()(device.as_raw(), pipeline.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Creates a new pipeline layout object
///
///   - `device` is the logical device that creates the pipeline layout.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkPipelineLayoutCreateInfo` structure specifying the state of the pipeline
///     layout object.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pPipelineLayout` points to a `VkPipelineLayout` handle in which the
///     resulting pipeline layout object is returned.
///
pub fn vkCreatePipelineLayout(
  device: VkDevice,
  pCreateInfo: &VkPipelineLayoutCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkPipelineLayout, VkResult> {
  unsafe {
    let mut pPipelineLayout: VkPipelineLayout = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreatePipelineLayout.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pPipelineLayout).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pPipelineLayout);
    })
  }
}

/// Destroy a pipeline layout object
///
///   - `device` is the logical device that destroys the pipeline layout.
///
///   - `pipelineLayout` is the pipeline layout to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyPipelineLayout(
  device: VkDevice,
  pipelineLayout: Option<VkPipelineLayout>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyPipelineLayout.unwrap()(
        device.as_raw(),
        pipelineLayout.as_raw(),
        pAllocator.as_raw(),
      )
    })
  }
}

/// Create a new sampler object
///
///   - `device` is the logical device that creates the sampler.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkSamplerCreateInfo`
///     structure specifying the state of the sampler object.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pSampler` points to a `VkSampler` handle in which the resulting sampler
///     object is returned.
///
pub fn vkCreateSampler(
  device: VkDevice,
  pCreateInfo: &VkSamplerCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSampler, VkResult> {
  unsafe {
    let mut pSampler: VkSampler = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateSampler.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSampler).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSampler);
    })
  }
}

/// Destroy a sampler object
///
///   - `device` is the logical device that destroys the sampler.
///
///   - `sampler` is the sampler to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroySampler(device: VkDevice, sampler: Option<VkSampler>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySampler.unwrap()(device.as_raw(), sampler.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new descriptor set layout
///
///   - `device` is the logical device that creates the descriptor set layout.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkDescriptorSetLayoutCreateInfo` structure specifying the state of the
///     descriptor set layout object.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pSetLayout` points to a `VkDescriptorSetLayout` handle in which the
///     resulting descriptor set layout object is returned.
///
pub fn vkCreateDescriptorSetLayout(
  device: VkDevice,
  pCreateInfo: &VkDescriptorSetLayoutCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkDescriptorSetLayout, VkResult> {
  unsafe {
    let mut pSetLayout: VkDescriptorSetLayout = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateDescriptorSetLayout.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSetLayout).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSetLayout);
    })
  }
}

/// Destroy a descriptor set layout object
///
///   - `device` is the logical device that destroys the descriptor set layout.
///
///   - `descriptorSetLayout` is the descriptor set layout to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyDescriptorSetLayout(
  device: VkDevice,
  descriptorSetLayout: Option<VkDescriptorSetLayout>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyDescriptorSetLayout.unwrap()(
        device.as_raw(),
        descriptorSetLayout.as_raw(),
        pAllocator.as_raw(),
      )
    })
  }
}

/// Creates a descriptor pool object
///
///   - `device` is the logical device that creates the descriptor pool.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkDescriptorPoolCreateInfo` structure specifying the state of the
///     descriptor pool object.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pDescriptorPool` points to a `VkDescriptorPool` handle in which the
///     resulting descriptor pool object is returned.
///
/// `pAllocator` controls host memory allocation as described in the [Memory
/// Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
/// The created descriptor pool is returned in `pDescriptorPool`.
///
pub fn vkCreateDescriptorPool(
  device: VkDevice,
  pCreateInfo: &VkDescriptorPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkDescriptorPool, VkResult> {
  unsafe {
    let mut pDescriptorPool: VkDescriptorPool = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateDescriptorPool.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pDescriptorPool).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pDescriptorPool);
    })
  }
}

/// Destroy a descriptor pool object
///
///   - `device` is the logical device that destroys the descriptor pool.
///
///   - `descriptorPool` is the descriptor pool to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
/// When a pool is destroyed, all descriptor sets allocated from the pool are
/// implicitly freed and become invalid. Descriptor sets allocated from a given pool
/// do not need to be freed before destroying that descriptor pool.
///
pub fn vkDestroyDescriptorPool(
  device: VkDevice,
  descriptorPool: Option<VkDescriptorPool>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyDescriptorPool.unwrap()(
        device.as_raw(),
        descriptorPool.as_raw(),
        pAllocator.as_raw(),
      )
    })
  }
}

/// Resets a descriptor pool object
///
/// To return all descriptor sets allocated from a given pool to the pool, rather
/// than freeing individual descriptor sets.
///
///   - `device` is the logical device that owns the descriptor pool.
///
///   - `descriptorPool` is the descriptor pool to be reset.
///
///   - `flags` is reserved for future use.
///
/// Resetting a descriptor pool recycles all of the resources from all of the
/// descriptor sets allocated from the descriptor pool back to the descriptor pool,
/// and the descriptor sets are implicitly freed.
///
pub fn vkResetDescriptorPool(
  device: VkDevice,
  descriptorPool: VkDescriptorPool,
  flags: VkDescriptorPoolResetFlags,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetDescriptorPool.unwrap()(device.as_raw(), descriptorPool.as_raw(), flags)
    })
  }
}

/// Allocate one or more descriptor sets
///
///   - `device` is the logical device that owns the descriptor pool.
///
///   - `pAllocateInfo` is a pointer to an instance of the
///     `VkDescriptorSetAllocateInfo` structure describing parameters of the
///     allocation.
///
///   - `pDescriptorSets` is a pointer to an array of `VkDescriptorSet` handles in
///     which the resulting descriptor set objects are returned.
///
/// The allocated descriptor sets are returned in `pDescriptorSets`.
///
/// When a descriptor set is allocated, the initial state is largely uninitialized
/// and all descriptors are undefined. However, the descriptor set can: be bound in
/// a command buffer without causing errors or exceptions. All entries that are
/// statically used by a pipeline in a drawing or dispatching command must: have
/// been populated before the descriptor set is bound for use by that command.
/// Entries that are not statically used by a pipeline can: have uninitialized
/// descriptors or descriptors of resources that have been destroyed, and executing
/// a draw or dispatch with such a descriptor set bound does not cause undefined
/// behavior. This means applications need not populate unused entries with dummy
/// descriptors.
///
/// If a call to `vkAllocateDescriptorSets` would cause the total number of
/// descriptor sets allocated from the pool to exceed the value of
/// `VkDescriptorPoolCreateInfo::maxSets` used to create
/// `pAllocateInfo`→\`descriptorPool\`, then the allocation may: fail due to lack
/// of space in the descriptor pool. Similarly, the allocation may: fail due to lack
/// of space if the call to `vkAllocateDescriptorSets` would cause the number of any
/// given descriptor type to exceed the sum of all the `descriptorCount` members of
/// each element of `VkDescriptorPoolCreateInfo::pPoolSizes` with a `member` equal
/// to that type. If the allocation fails due to no more space in the descriptor
/// pool, and not because of system or device memory exhaustion, then
/// `VK_ERROR_OUT_OF_POOL_MEMORY_KHR` must: be returned.
///
/// `vkAllocateDescriptorSets` can: be used to create multiple descriptor sets. If
/// the creation of any of those descriptor sets fails, then the implementation
/// must: destroy all successfully created descriptor set objects from this command,
/// set all entries of the `pDescriptorSets` array to `VK_NULL_HANDLE` and return
/// the error.
///
pub fn vkAllocateDescriptorSets(
  device: VkDevice,
  pAllocateInfo: &VkDescriptorSetAllocateInfo,
  pDescriptorSets: &mut [VkDescriptorSet],
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkAllocateDescriptorSets.unwrap()(
        device.as_raw(),
        pAllocateInfo.as_raw(),
        pDescriptorSets.as_raw(),
      )
    })
  }
}

/// Free one or more descriptor sets
///
///   - `device` is the logical device that owns the descriptor pool.
///
///   - `descriptorPool` is the descriptor pool from which the descriptor sets were
///     allocated.
///
///   - `descriptorSetCount` is the number of elements in the `pDescriptorSets`
///     array.
///
///   - `pDescriptorSets` is an array of handles to `VkDescriptorSet` objects.
///
/// After a successful call to `vkFreeDescriptorSets`, all descriptor sets in
/// `pDescriptorSets` are invalid.
///
pub fn vkFreeDescriptorSets(
  device: VkDevice,
  descriptorPool: VkDescriptorPool,
  pDescriptorSets: &[VkDescriptorSet],
) -> VkResult {
  unsafe {
    let descriptorSetCount = pDescriptorSets.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkFreeDescriptorSets.unwrap()(
        device.as_raw(),
        descriptorPool.as_raw(),
        descriptorSetCount,
        pDescriptorSets.as_raw(),
      )
    })
  }
}

/// Update the contents of a descriptor set object
///
/// Once allocated, descriptor sets can: be updated with a combination of write and
/// copy operations.
///
///   - `device` is the logical device that updates the descriptor sets.
///
///   - `descriptorWriteCount` is the number of elements in the `pDescriptorWrites`
///     array.
///
///   - `pDescriptorWrites` is a pointer to an array of `VkWriteDescriptorSet`
///     structures describing the descriptor sets to write to.
///
///   - `descriptorCopyCount` is the number of elements in the `pDescriptorCopies`
///     array.
///
///   - `pDescriptorCopies` is a pointer to an array of `VkCopyDescriptorSet`
///     structures describing the descriptor sets to copy between.
///
/// The operations described by `pDescriptorWrites` are performed first, followed by
/// the operations described by `pDescriptorCopies`. Within each array, the
/// operations are performed in the order they appear in the array.
///
/// Each element in the `pDescriptorWrites` array describes an operation updating
/// the descriptor set using descriptors for resources specified in the structure.
///
/// Each element in the `pDescriptorCopies` array is a `VkCopyDescriptorSet`
/// structure describing an operation copying descriptors between sets.
///
/// If the `dstSet` member of any element of `pDescriptorWrites` or
/// `pDescriptorCopies` is bound, accessed, or modified by any command that was
/// recorded to a command buffer which is currently in the [recording or executable
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle), that command buffer becomes
/// [invalid](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
pub fn vkUpdateDescriptorSets(
  device: VkDevice,
  pDescriptorWrites: &[VkWriteDescriptorSet],
  pDescriptorCopies: &[VkCopyDescriptorSet],
) {
  unsafe {
    let descriptorWriteCount = pDescriptorWrites.len() as u32;
    let descriptorCopyCount = pDescriptorCopies.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkUpdateDescriptorSets.unwrap()(
        device.as_raw(),
        descriptorWriteCount,
        pDescriptorWrites.as_raw(),
        descriptorCopyCount,
        pDescriptorCopies.as_raw(),
      )
    })
  }
}

/// Create a new framebuffer object
///
///   - `device` is the logical device that creates the framebuffer.
///
///   - `pCreateInfo` points to a `VkFramebufferCreateInfo` structure which
///     describes additional information about framebuffer creation.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pFramebuffer` points to a `VkFramebuffer` handle in which the resulting
///     framebuffer object is returned.
///
pub fn vkCreateFramebuffer(
  device: VkDevice,
  pCreateInfo: &VkFramebufferCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkFramebuffer, VkResult> {
  unsafe {
    let mut pFramebuffer: VkFramebuffer = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateFramebuffer.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pFramebuffer).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pFramebuffer);
    })
  }
}

/// Destroy a framebuffer object
///
///   - `device` is the logical device that destroys the framebuffer.
///
///   - `framebuffer` is the handle of the framebuffer to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyFramebuffer(
  device: VkDevice,
  framebuffer: Option<VkFramebuffer>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyFramebuffer.unwrap()(device.as_raw(), framebuffer.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new render pass object
///
///   - `device` is the logical device that creates the render pass.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkRenderPassCreateInfo`
///     structure that describes the parameters of the render pass.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pRenderPass` points to a `VkRenderPass` handle in which the resulting
///     render pass object is returned.
///
pub fn vkCreateRenderPass(
  device: VkDevice,
  pCreateInfo: &VkRenderPassCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkRenderPass, VkResult> {
  unsafe {
    let mut pRenderPass: VkRenderPass = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateRenderPass.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pRenderPass).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pRenderPass);
    })
  }
}

/// Destroy a render pass object
///
///   - `device` is the logical device that destroys the render pass.
///
///   - `renderPass` is the handle of the render pass to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
pub fn vkDestroyRenderPass(
  device: VkDevice,
  renderPass: Option<VkRenderPass>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyRenderPass.unwrap()(device.as_raw(), renderPass.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Returns the granularity for optimal render area
///
///   - `device` is the logical device that owns the render pass.
///
///   - `renderPass` is a handle to a render pass.
///
///   - `pGranularity` points to a `VkExtent2D` structure in which the granularity
///     is returned.
///
/// The conditions leading to an optimal `renderArea` are:
///
///   - the `offset`.x member in `renderArea` is a multiple of the `width` member of
///     the returned `VkExtent2D` (the horizontal granularity).
///
///   - the `offset`.y member in `renderArea` is a multiple of the `height` of the
///     returned `VkExtent2D` (the vertical granularity).
///
///   - either the `offset`.width member in `renderArea` is a multiple of the
///     horizontal granularity or `offset`.x+`offset`.width is equal to the `width`
///     of the `framebuffer` in the `VkRenderPassBeginInfo`.
///
///   - either the `offset`.height member in `renderArea` is a multiple of the
///     vertical granularity or `offset`.y+`offset`.height is equal to the `height`
///     of the `framebuffer` in the `VkRenderPassBeginInfo`.
///
/// Subpass dependencies are not affected by the render area, and apply to the
/// entire image subresources attached to the framebuffer as specified in the
/// description of [automatic layout transitions](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-layout-transitions).
/// Similarly, pipeline barriers are valid even if their effect extends outside the
/// render area.
///
pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass) -> VkExtent2D {
  unsafe {
    let mut pGranularity: VkExtent2D = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetRenderAreaGranularity.unwrap()(
        device.as_raw(),
        renderPass.as_raw(),
        (&mut pGranularity).as_raw(),
      );
      return pGranularity;
    })
  }
}

/// Create a new command pool object
///
///   - `device` is the logical device that creates the command pool.
///
///   - `pCreateInfo` contains information used to create the command pool.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pCommandPool` points to a `VkCommandPool` handle in which the created pool
///     is returned.
///
pub fn vkCreateCommandPool(
  device: VkDevice,
  pCreateInfo: &VkCommandPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkCommandPool, VkResult> {
  unsafe {
    let mut pCommandPool: VkCommandPool = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateCommandPool.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pCommandPool).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pCommandPool);
    })
  }
}

/// Destroy a command pool object
///
///   - `device` is the logical device that destroys the command pool.
///
///   - `commandPool` is the handle of the command pool to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
/// When a pool is destroyed, all command buffers allocated from the pool are
/// [freed](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeCommandBuffers).
///
/// Any primary command buffer allocated from another `VkCommandPool` that is in the
/// [recording or executable state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle) and has a secondary
/// command buffer allocated from `commandPool` recorded into it, becomes
/// [invalid](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
pub fn vkDestroyCommandPool(
  device: VkDevice,
  commandPool: Option<VkCommandPool>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyCommandPool.unwrap()(device.as_raw(), commandPool.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Reset a command pool
///
///   - `device` is the logical device that owns the command pool.
///
///   - `commandPool` is the command pool to reset.
///
///   - `flags` is a bitmask of `VkCommandPoolResetFlagBits` controlling the reset
///     operation.
///
/// Resetting a command pool recycles all of the resources from all of the command
/// buffers allocated from the command pool back to the command pool. All command
/// buffers that have been allocated from the command pool are put in the [initial
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
/// Any primary command buffer allocated from another `VkCommandPool` that is in the
/// [recording or executable state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle) and has a secondary
/// command buffer allocated from `commandPool` recorded into it, becomes
/// [invalid](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetCommandPool.unwrap()(device.as_raw(), commandPool.as_raw(), flags)
    })
  }
}

/// Allocate command buffers from an existing command pool
///
///   - `device` is the logical device that owns the command pool.
///
///   - `pAllocateInfo` is a pointer to an instance of the
///     `VkCommandBufferAllocateInfo` structure describing parameters of the
///     allocation.
///
///   - `pCommandBuffers` is a pointer to an array of `VkCommandBuffer` handles in
///     which the resulting command buffer objects are returned. The array must: be
///     at least the length specified by the `commandBufferCount` member of
///     `pAllocateInfo`. Each allocated command buffer begins in the initial state.
///
/// `vkAllocateCommandBuffers` can: be used to create multiple command buffers. If
/// the creation of any of those command buffers fails, the implementation must:
/// destroy all successfully created command buffer objects from this command, set
/// all entries of the `pCommandBuffers` array to `NULL` and return the error.
///
/// When command buffers are first allocated, they are in the [initial
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
pub fn vkAllocateCommandBuffers(
  device: VkDevice,
  pAllocateInfo: &VkCommandBufferAllocateInfo,
  pCommandBuffers: &mut [VkCommandBuffer],
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkAllocateCommandBuffers.unwrap()(
        device.as_raw(),
        pAllocateInfo.as_raw(),
        pCommandBuffers.as_raw(),
      )
    })
  }
}

/// Free command buffers
///
///   - `device` is the logical device that owns the command pool.
///
///   - `commandPool` is the command pool from which the command buffers were
///     allocated.
///
///   - `commandBufferCount` is the length of the `pCommandBuffers` array.
///
///   - `pCommandBuffers` is an array of handles of command buffers to free.
///
/// Any primary command buffer that is in the [recording or executable
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle) and has any element of `pCommandBuffers`
/// recorded into it, becomes [invalid](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, pCommandBuffers: &[VkCommandBuffer]) {
  unsafe {
    let commandBufferCount = pCommandBuffers.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkFreeCommandBuffers.unwrap()(
        device.as_raw(),
        commandPool.as_raw(),
        commandBufferCount,
        pCommandBuffers.as_raw(),
      )
    })
  }
}

/// Start recording a command buffer
///
///   - `commandBuffer` is the handle of the command buffer which is to be put in
///     the recording state.
///
///   - `pBeginInfo` is an instance of the `VkCommandBufferBeginInfo` structure,
///     which defines additional information about how the command buffer begins
///     recording.
///
pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: &VkCommandBufferBeginInfo) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkBeginCommandBuffer.unwrap()(commandBuffer.as_raw(), pBeginInfo.as_raw())
    })
  }
}

/// Finish recording a command buffer
///
///   - `commandBuffer` is the command buffer to complete recording.
///
/// If there was an error during recording, the application will be notified by an
/// unsuccessful return code returned by `vkEndCommandBuffer`. If the application
/// wishes to further use the command buffer, the command buffer must: be reset. The
/// command buffer must: have been in the [recording
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle), and is moved to the [executable
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkEndCommandBuffer.unwrap()(commandBuffer.as_raw())
    })
  }
}

/// Reset a command buffer to the initial state
///
///   - `commandBuffer` is the command buffer to reset. The command buffer can: be
///     in any state other than [pending](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle), and is moved
///     into the [initial state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
///   - `flags` is a bitmask of `VkCommandBufferResetFlagBits` controlling the reset
///     operation.
///
/// Any primary command buffer that is in the [recording or executable
/// state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle) and has `commandBuffer` recorded into it,
/// becomes [invalid](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkResetCommandBuffer.unwrap()(commandBuffer.as_raw(), flags)
    })
  }
}

/// Bind a pipeline object to a command buffer
///
/// Once a pipeline has been created, it can: be bound to the command buffer using
/// the command.
///
///   - `commandBuffer` is the command buffer that the pipeline will be bound to.
///
///   - `pipelineBindPoint` is a `VkPipelineBindPoint` value specifying whether to
///     bind to the compute or graphics bind point. Binding one does not disturb the
///     other.
///
///   - `pipeline` is the pipeline to be bound.
///
/// Once bound, a pipeline binding affects subsequent graphics or compute commands
/// in the command buffer until a different pipeline is bound to the bind point. The
/// pipeline bound to `VK_PIPELINE_BIND_POINT_COMPUTE` controls the behavior of
/// `vkCmdDispatch` and `vkCmdDispatchIndirect`. The pipeline bound to
/// `VK_PIPELINE_BIND_POINT_GRAPHICS` controls the behavior of all [drawing
/// commands](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#drawing). No other commands are affected by the pipeline state.
///
pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBindPipeline.unwrap()(commandBuffer.as_raw(), pipelineBindPoint, pipeline.as_raw())
    })
  }
}

/// Set the viewport on a command buffer
///
/// If the bound pipeline state object was not created with the
/// `VK_DYNAMIC_STATE_VIEWPORT` dynamic state enabled, viewport transformation
/// parameters are specified using the `pViewports` member of
/// `VkPipelineViewportStateCreateInfo` in the pipeline state object. If the
/// pipeline state object was created with the `VK_DYNAMIC_STATE_VIEWPORT` dynamic
/// state enabled, the viewport transformation parameters are dynamically set and
/// changed with the command.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `firstViewport` is the index of the first viewport whose parameters are
///     updated by the command.
///
///   - `viewportCount` is the number of viewports whose parameters are updated by
///     the command.
///
///   - `pViewports` is a pointer to an array of `VkViewport` structures specifying
///     viewport parameters.
///
/// The viewport parameters taken from element i of `pViewports` replace the current
/// state for the viewport index `firstViewport` + i, for i in \[0,
/// `viewportCount`).
///
pub fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, pViewports: &[VkViewport]) {
  unsafe {
    let viewportCount = pViewports.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetViewport.unwrap()(
        commandBuffer.as_raw(),
        firstViewport,
        viewportCount,
        pViewports.as_raw(),
      )
    })
  }
}

/// Set the dynamic scissor rectangles on a command buffer
///
/// The scissor test determines if a fragment’s framebuffer coordinates
/// (x<sub>f</sub>,y<sub>f</sub>) lie within the scissor rectangle corresponding to
/// the viewport index (see [Controlling the Viewport](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vertexpostproc-viewport))
/// used by the primitive that generated the fragment. If the pipeline state object
/// is created without `VK_DYNAMIC_STATE_SCISSOR` enabled then the scissor
/// rectangles are set by the `VkPipelineViewportStateCreateInfo` state of the
/// pipeline state object. Otherwise, to dynamically set the scissor rectangles
/// call.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `firstScissor` is the index of the first scissor whose state is updated by
///     the command.
///
///   - `scissorCount` is the number of scissors whose rectangles are updated by the
///     command.
///
///   - `pScissors` is a pointer to an array of `VkRect2D` structures defining
///     scissor rectangles.
///
/// The scissor rectangles taken from element i of `pScissors` replace the current
/// state for the scissor index `firstScissor` + i, for i in \[0, `scissorCount`).
///
/// Each scissor rectangle is described by a `VkRect2D` structure, with the
/// `offset`.x and `offset`.y values determining the upper left corner of the
/// scissor rectangle, and the `extent`.width and `extent`.height values determining
/// the size in pixels.
///
pub fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, pScissors: &[VkRect2D]) {
  unsafe {
    let scissorCount = pScissors.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetScissor.unwrap()(
        commandBuffer.as_raw(),
        firstScissor,
        scissorCount,
        pScissors.as_raw(),
      )
    })
  }
}

/// Set the dynamic line width state
///
/// The line width is specified by the
/// `VkPipelineRasterizationStateCreateInfo::lineWidth` property of the currently
/// active pipeline, if the pipeline was not created with
/// `VK_DYNAMIC_STATE_LINE_WIDTH` enabled.
///
/// Otherwise, the line width is set by calling `vkCmdSetLineWidth`.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `lineWidth` is the width of rasterized line segments.
///
pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetLineWidth.unwrap()(commandBuffer.as_raw(), lineWidth)
    })
  }
}

/// Set the depth bias dynamic state
///
/// The depth values of all fragments generated by the rasterization of a polygon
/// can: be offset by a single value that is computed for that polygon. This
/// behavior is controlled by the `depthBiasEnable`, `depthBiasConstantFactor`,
/// `depthBiasClamp`, and `depthBiasSlopeFactor` members of
/// `VkPipelineRasterizationStateCreateInfo`, or by the corresponding parameters to
/// the `vkCmdSetDepthBias` command if depth bias state is dynamic.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `depthBiasConstantFactor` is a scalar factor controlling the constant depth
///     value added to each fragment.
///
///   - `depthBiasClamp` is the maximum (or minimum) depth bias of a fragment.
///
///   - `depthBiasSlopeFactor` is a scalar factor applied to a fragment’s slope in
///     depth bias calculations.
///
/// If `depthBiasEnable` is `VK_FALSE`, no depth bias is applied and the fragment’s
/// depth values are unchanged.
///
/// `depthBiasSlopeFactor` scales the maximum depth slope of the polygon, and
/// `depthBiasConstantFactor` scales an implementation-dependent constant that
/// relates to the usable resolution of the depth buffer. The resulting values are
/// summed to produce the depth bias value which is then clamped to a minimum or
/// maximum value specified by `depthBiasClamp`. `depthBiasSlopeFactor`,
/// `depthBiasConstantFactor`, and `depthBiasClamp` can: each be positive, negative,
/// or zero.
///
/// The maximum depth slope m of a triangle is
///
/// m = \\sqrt{ \\left({{\\partial z\_f} \\over {\\partial x\_f}}\\right)^2 +
/// \\left({{\\partial z\_f} \\over {\\partial y\_f}}\\right)^2}
///
/// m = \\sqrt{ \\left({{\\partial z\_f} \\over {\\partial x\_f}}\\right)^2 +
/// \\left({{\\partial z\_f} \\over {\\partial y\_f}}\\right)^2}
///
/// where (x<sub>f</sub>, y<sub>f</sub>, z<sub>f</sub>) is a point on the triangle.
/// m may: be approximated as
///
/// m = \\max\\left( \\left| { {\\partial z\_f} \\over {\\partial x\_f} } \\right|,
/// \\left| { {\\partial z\_f} \\over {\\partial y\_f} } \\right| \\right).
///
/// m = \\max\\left( \\left| { {\\partial z\_f} \\over {\\partial x\_f} } \\right|,
/// \\left| { {\\partial z\_f} \\over {\\partial y\_f} } \\right| \\right).
///
/// The minimum resolvable difference r is an implementation-dependent parameter
/// that depends on the depth buffer representation. It is the smallest difference
/// in framebuffer coordinate z values that is guaranteed to remain distinct
/// throughout polygon rasterization and in the depth buffer. All pairs of fragments
/// generated by the rasterization of two polygons with otherwise identical
/// vertices, but `z`<sub>f</sub> values that differ by $r$, will have distinct
/// depth values.
///
/// For fixed-point depth buffer representations, r is constant throughout the range
/// of the entire depth buffer. For floating-point depth buffers, there is no single
/// minimum resolvable difference. In this case, the minimum resolvable difference
/// for a given polygon is dependent on the maximum exponent, e, in the range of z
/// values spanned by the primitive. If n is the number of bits in the
/// floating-point mantissa, the minimum resolvable difference, r, for the given
/// primitive is defined as
///
///   -
///     r = 2<sup>e-n</sup>
///
/// If a triangle is rasterized using the `VK_POLYGON_MODE_FILL_RECTANGLE_NV`
/// polygon mode, then this minimum resolvable difference may: not be resolvable for
/// samples outside of the triangle, where the depth is extrapolated.
///
/// If no depth buffer is present, r is undefined.
///
/// The bias value o for a polygon is
///
/// o = \\begin{cases} m \\times depthBiasSlopeFactor + r \\times
/// depthBiasConstantFactor & depthBiasClamp = 0\\ or\\ NaN \\\\ \\min(m \\times
/// depthBiasSlopeFactor + r \\times depthBiasConstantFactor, depthBiasClamp) &
/// depthBiasClamp \> 0 \\\\ \\max(m \\times depthBiasSlopeFactor + r \\times
/// depthBiasConstantFactor, depthBiasClamp) & depthBiasClamp \< 0 \\\\ \\end{cases}
///
/// o = \\begin{cases} m \\times depthBiasSlopeFactor + r \\times
/// depthBiasConstantFactor & depthBiasClamp = 0\\ or\\ NaN \\\\ \\min(m \\times
/// depthBiasSlopeFactor + r \\times depthBiasConstantFactor, depthBiasClamp) &
/// depthBiasClamp \> 0 \\\\ \\max(m \\times depthBiasSlopeFactor + r \\times
/// depthBiasConstantFactor, depthBiasClamp) & depthBiasClamp \< 0 \\\\ \\end{cases}
///
/// m is computed as described above. If the depth buffer uses a fixed-point
/// representation, m is a function of depth values in the range \[0,1\], and o is
/// applied to depth values in the same range.
///
/// For fixed-point depth buffers, fragment depth values are always limited to the
/// range \[0,1\] by clamping after depth bias addition is performed. Unless the
/// extension is enabled, fragment depth values are clamped even when the depth
/// buffer uses a floating-point representation.
///
pub fn vkCmdSetDepthBias(
  commandBuffer: VkCommandBuffer,
  depthBiasConstantFactor: f32,
  depthBiasClamp: f32,
  depthBiasSlopeFactor: f32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetDepthBias.unwrap()(
        commandBuffer.as_raw(),
        depthBiasConstantFactor,
        depthBiasClamp,
        depthBiasSlopeFactor,
      )
    })
  }
}

/// Set the values of blend constants
///
/// Otherwise, to dynamically set and change the blend constant.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `blendConstants` is an array of four values specifying the R, G, B, and A
///     components of the blend constant color used in blending, depending on the
///     [blend factor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blendfactors).
///
pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: [f32; 4]) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetBlendConstants.unwrap()(commandBuffer.as_raw(), blendConstants)
    })
  }
}

/// Set the depth bounds test values for a command buffer
///
/// The depth bounds test conditionally disables coverage of a sample based on the
/// outcome of a comparison between the value z<sub>a</sub> in the depth attachment
/// at location (x<sub>f</sub>,y<sub>f</sub>) (for the appropriate sample) and a
/// range of values. The test is enabled or disabled by the `depthBoundsTestEnable`
/// member of `VkPipelineDepthStencilStateCreateInfo`: If the pipeline state object
/// is created without the `VK_DYNAMIC_STATE_DEPTH_BOUNDS` dynamic state enabled
/// then the range of values used in the depth bounds test are defined by the
/// `minDepthBounds` and `maxDepthBounds` members of the
/// `VkPipelineDepthStencilStateCreateInfo` structure. Otherwise, to dynamically set
/// the depth bounds range values call.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `minDepthBounds` is the lower bound of the range of depth values used in the
///     depth bounds test.
///
///   - `maxDepthBounds` is the upper bound of the range.
///
pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetDepthBounds.unwrap()(commandBuffer.as_raw(), minDepthBounds, maxDepthBounds)
    })
  }
}

/// Set the stencil compare mask dynamic state
///
/// If the pipeline state object is created with the
/// `VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK` dynamic state enabled, then to
/// dynamically set the stencil compare mask call.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `faceMask` is a bitmask of `VkStencilFaceFlagBits` specifying the set of
///     stencil state for which to update the compare mask.
///
///   - `compareMask` is the new value to use as the stencil compare mask.
///
pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilCompareMask.unwrap()(commandBuffer.as_raw(), faceMask, compareMask)
    })
  }
}

/// Set the stencil write mask dynamic state
///
/// If the pipeline state object is created with the
/// `VK_DYNAMIC_STATE_STENCIL_WRITE_MASK` dynamic state enabled, then to dynamically
/// set the stencil write mask call.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `faceMask` is a bitmask of `VkStencilFaceFlagBits` specifying the set of
///     stencil state for which to update the write mask, as described above for
///     `vkCmdSetStencilCompareMask`.
///
///   - `writeMask` is the new value to use as the stencil write mask.
///
pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilWriteMask.unwrap()(commandBuffer.as_raw(), faceMask, writeMask)
    })
  }
}

/// Set the stencil reference dynamic state
///
/// If the pipeline state object is created with the
/// `VK_DYNAMIC_STATE_STENCIL_REFERENCE` dynamic state enabled, then to dynamically
/// set the stencil reference value call.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `faceMask` is a bitmask of `VkStencilFaceFlagBits` specifying the set of
///     stencil state for which to update the reference value, as described above
///     for `vkCmdSetStencilCompareMask`.
///
///   - `reference` is the new value to use as the stencil reference value.
///
pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilReference.unwrap()(commandBuffer.as_raw(), faceMask, reference)
    })
  }
}

/// Binds descriptor sets to a command buffer
///
///   - `commandBuffer` is the command buffer that the descriptor sets will be bound
///     to.
///
///   - `pipelineBindPoint` is a `VkPipelineBindPoint` indicating whether the
///     descriptors will be used by graphics pipelines or compute pipelines. There
///     is a separate set of bind points for each of graphics and compute, so
///     binding one does not disturb the other.
///
///   - `layout` is a `VkPipelineLayout` object used to program the bindings.
///
///   - `firstSet` is the set number of the first descriptor set to be bound.
///
///   - `descriptorSetCount` is the number of elements in the `pDescriptorSets`
///     array.
///
///   - `pDescriptorSets` is an array of handles to `VkDescriptorSet` objects
///     describing the descriptor sets to write to.
///
///   - `dynamicOffsetCount` is the number of dynamic offsets in the
///     `pDynamicOffsets` array.
///
///   - `pDynamicOffsets` is a pointer to an array of `uint32_t` values specifying
///     dynamic offsets.
///
/// `vkCmdBindDescriptorSets` causes the sets numbered \[`firstSet`..
/// `firstSet`+`descriptorSetCount`-1\] to use the bindings stored in
/// `pDescriptorSets`\[0..`descriptorSetCount`-1\] for subsequent rendering commands
/// (either compute or graphics, according to the `pipelineBindPoint`). Any bindings
/// that were previously applied via these sets are no longer valid.
///
/// Once bound, a descriptor set affects rendering of subsequent graphics or compute
/// commands in the command buffer until a different set is bound to the same set
/// number, or else until the set is disturbed as described in [Pipeline Layout
/// Compatibility](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-compatibility).
///
/// A compatible descriptor set must: be bound for all set numbers that any shaders
/// in a pipeline access, at the time that a draw or dispatch command is recorded to
/// execute using that pipeline. However, if none of the shaders in a pipeline
/// statically use any bindings with a particular set number, then no descriptor set
/// need be bound for that set number, even if the pipeline layout includes a
/// non-trivial descriptor set layout for that set number.
///
/// If any of the sets being bound include dynamic uniform or storage buffers, then
/// `pDynamicOffsets` includes one element for each array element in each dynamic
/// descriptor type binding in each set. Values are taken from `pDynamicOffsets` in
/// an order such that all entries for set N come before set N+1; within a set,
/// entries are ordered by the binding numbers in the descriptor set layouts; and
/// within a binding array, elements are in order. `dynamicOffsetCount` must: equal
/// the total number of dynamic descriptors in the sets being bound.
///
/// The effective offset used for dynamic uniform and storage buffer bindings is the
/// sum of the relative offset taken from `pDynamicOffsets`, and the base address of
/// the buffer plus base offset in the descriptor set. The length of the dynamic
/// uniform and storage buffer bindings is the buffer range as specified in the
/// descriptor set.
///
/// Each of the `pDescriptorSets` must: be compatible with the pipeline layout
/// specified by `layout`. The layout used to program the bindings must: also be
/// compatible with the pipeline used in subsequent graphics or compute commands, as
/// defined in the [Pipeline Layout Compatibility](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-compatibility)
/// section.
///
/// The descriptor set contents bound by a call to `vkCmdBindDescriptorSets` may: be
/// consumed during host execution of the command, or during shader execution of the
/// resulting draws, or any time in between. Thus, the contents must: not be altered
/// (overwritten by an update command, or freed) between when the command is
/// recorded and when the command completes executing on the queue. The contents of
/// `pDynamicOffsets` are consumed immediately during execution of
/// `vkCmdBindDescriptorSets`. Once all pending uses have completed, it is legal to
/// update and reuse a descriptor set.
///
pub fn vkCmdBindDescriptorSets(
  commandBuffer: VkCommandBuffer,
  pipelineBindPoint: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  firstSet: u32,
  pDescriptorSets: &[VkDescriptorSet],
  pDynamicOffsets: &[u32],
) {
  unsafe {
    let descriptorSetCount = pDescriptorSets.len() as u32;
    let dynamicOffsetCount = pDynamicOffsets.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBindDescriptorSets.unwrap()(
        commandBuffer.as_raw(),
        pipelineBindPoint,
        layout.as_raw(),
        firstSet,
        descriptorSetCount,
        pDescriptorSets.as_raw(),
        dynamicOffsetCount,
        pDynamicOffsets.as_raw(),
      )
    })
  }
}

/// Bind an index buffer to a command buffer
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `buffer` is the buffer being bound.
///
///   - `offset` is the starting offset in bytes within `buffer` used in index
///     buffer address calculations.
///
///   - `indexType` is a `VkIndexType` value specifying whether indices are treated
///     as 16 bits or 32 bits.
///
pub fn vkCmdBindIndexBuffer(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  indexType: VkIndexType,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBindIndexBuffer.unwrap()(commandBuffer.as_raw(), buffer.as_raw(), offset, indexType)
    })
  }
}

/// Bind vertex buffers to a command buffer
///
/// To bind vertex buffers to a command buffer for use in subsequent draw commands.
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `firstBinding` is the index of the first vertex input binding whose state is
///     updated by the command.
///
///   - `bindingCount` is the number of vertex input bindings whose state is updated
///     by the command.
///
///   - `pBuffers` is a pointer to an array of buffer handles.
///
///   - `pOffsets` is a pointer to an array of buffer offsets.
///
/// The values taken from elements i of `pBuffers` and `pOffsets` replace the
/// current state for the vertex input binding `firstBinding` + i, for i in \[0,
/// `bindingCount`). The vertex input binding is updated to start at the offset
/// indicated by `pOffsets`\[i\] from the start of the buffer `pBuffers`\[i\]. All
/// vertex input attributes that use each of these bindings will use these updated
/// addresses in their address calculations for subsequent draw commands.
///
pub fn vkCmdBindVertexBuffers(
  commandBuffer: VkCommandBuffer,
  firstBinding: u32,
  pBuffers: &[VkBuffer],
  pOffsets: &[VkDeviceSize],
) {
  unsafe {
    let bindingCount = pBuffers.len() as u32;
    assert!(bindingCount as usize == pOffsets.len());
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBindVertexBuffers.unwrap()(
        commandBuffer.as_raw(),
        firstBinding,
        bindingCount,
        pBuffers.as_raw(),
        pOffsets.as_raw(),
      )
    })
  }
}

/// Draw primitives
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `vertexCount` is the number of vertices to draw.
///
///   - `instanceCount` is the number of instances to draw.
///
///   - `firstVertex` is the index of the first vertex to draw.
///
///   - `firstInstance` is the instance ID of the first instance to draw.
///
/// When the command is executed, primitives are assembled using the current
/// primitive topology and `vertexCount` consecutive vertex indices with the first
/// `vertexIndex` value equal to `firstVertex`. The primitives are drawn
/// `instanceCount` times with `instanceIndex` starting with `firstInstance` and
/// increasing sequentially for each instance. The assembled primitives execute the
/// currently bound graphics pipeline.
///
pub fn vkCmdDraw(
  commandBuffer: VkCommandBuffer,
  vertexCount: u32,
  instanceCount: u32,
  firstVertex: u32,
  firstInstance: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDraw.unwrap()(
        commandBuffer.as_raw(),
        vertexCount,
        instanceCount,
        firstVertex,
        firstInstance,
      )
    })
  }
}

/// Issue an indexed draw into a command buffer
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `indexCount` is the number of vertices to draw.
///
///   - `instanceCount` is the number of instances to draw.
///
///   - `firstIndex` is the base index within the index buffer.
///
///   - `vertexOffset` is the value added to the vertex index before indexing into
///     the vertex buffer.
///
///   - `firstInstance` is the instance ID of the first instance to draw.
///
/// When the command is executed, primitives are assembled using the current
/// primitive topology and `indexCount` vertices whose indices are retrieved from
/// the index buffer. The index buffer is treated as an array of tightly packed
/// unsigned integers of size defined by the `vkCmdBindIndexBuffer::indexType`
/// parameter with which the buffer was bound.
///
/// The first vertex index is at an offset of `firstIndex` \* `indexSize` + `offset`
/// within the currently bound index buffer, where `offset` is the offset specified
/// by `vkCmdBindIndexBuffer` and `indexSize` is the byte size of the type specified
/// by `indexType`. Subsequent index values are retrieved from consecutive locations
/// in the index buffer. Indices are first compared to the primitive restart value,
/// then zero extended to 32 bits (if the `indexType` is `VK_INDEX_TYPE_UINT16`) and
/// have `vertexOffset` added to them, before being supplied as the `vertexIndex`
/// value.
///
/// The primitives are drawn `instanceCount` times with `instanceIndex` starting
/// with `firstInstance` and increasing sequentially for each instance. The
/// assembled primitives execute the currently bound graphics pipeline.
///
pub fn vkCmdDrawIndexed(
  commandBuffer: VkCommandBuffer,
  indexCount: u32,
  instanceCount: u32,
  firstIndex: u32,
  vertexOffset: i32,
  firstInstance: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDrawIndexed.unwrap()(
        commandBuffer.as_raw(),
        indexCount,
        instanceCount,
        firstIndex,
        vertexOffset,
        firstInstance,
      )
    })
  }
}

/// Issue an indirect draw into a command buffer
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `buffer` is the buffer containing draw parameters.
///
///   - `offset` is the byte offset into `buffer` where parameters begin.
///
///   - `drawCount` is the number of draws to execute, and can: be zero.
///
///   - `stride` is the byte stride between successive sets of draw parameters.
///
/// `vkCmdDrawIndirect` behaves similarly to `vkCmdDraw` except that the parameters
/// are read by the device from a buffer during execution. `drawCount` draws are
/// executed by the command, with parameters taken from `buffer` starting at
/// `offset` and increasing by `stride` bytes for each successive draw. The
/// parameters of each draw are encoded in an array of `VkDrawIndirectCommand`
/// structures. If `drawCount` is less than or equal to one, `stride` is ignored.
///
pub fn vkCmdDrawIndirect(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  drawCount: u32,
  stride: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDrawIndirect.unwrap()(
        commandBuffer.as_raw(),
        buffer.as_raw(),
        offset,
        drawCount,
        stride,
      )
    })
  }
}

/// Perform an indexed indirect draw
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `buffer` is the buffer containing draw parameters.
///
///   - `offset` is the byte offset into `buffer` where parameters begin.
///
///   - `drawCount` is the number of draws to execute, and can: be zero.
///
///   - `stride` is the byte stride between successive sets of draw parameters.
///
/// `vkCmdDrawIndexedIndirect` behaves similarly to `vkCmdDrawIndexed` except that
/// the parameters are read by the device from a buffer during execution.
/// `drawCount` draws are executed by the command, with parameters taken from
/// `buffer` starting at `offset` and increasing by `stride` bytes for each
/// successive draw. The parameters of each draw are encoded in an array of
/// `VkDrawIndexedIndirectCommand` structures. If `drawCount` is less than or equal
/// to one, `stride` is ignored.
///
pub fn vkCmdDrawIndexedIndirect(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  drawCount: u32,
  stride: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDrawIndexedIndirect.unwrap()(
        commandBuffer.as_raw(),
        buffer.as_raw(),
        offset,
        drawCount,
        stride,
      )
    })
  }
}

/// Dispatch compute work items
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `groupCountX` is the number of local workgroups to dispatch in the X
///     dimension.
///
///   - `groupCountY` is the number of local workgroups to dispatch in the Y
///     dimension.
///
///   - `groupCountZ` is the number of local workgroups to dispatch in the Z
///     dimension.
///
/// When the command is executed, a global workgroup consisting of groupCountX
/// {times} groupCountY {times} groupCountZ local workgroups is assembled.
///
pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDispatch.unwrap()(
        commandBuffer.as_raw(),
        groupCountX,
        groupCountY,
        groupCountZ,
      )
    })
  }
}

/// Dispatch compute work items using indirect parameters
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `buffer` is the buffer containing dispatch parameters.
///
///   - `offset` is the byte offset into `buffer` where parameters begin.
///
/// `vkCmdDispatchIndirect` behaves similarly to `vkCmdDispatch` except that the
/// parameters are read by the device from a buffer during execution. The parameters
/// of the dispatch are encoded in a `VkDispatchIndirectCommand` structure taken
/// from `buffer` starting at `offset`.
///
pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDispatchIndirect.unwrap()(commandBuffer.as_raw(), buffer.as_raw(), offset)
    })
  }
}

/// Copy data between buffer regions
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `srcBuffer` is the source buffer.
///
///   - `dstBuffer` is the destination buffer.
///
///   - `regionCount` is the number of regions to copy.
///
///   - `pRegions` is a pointer to an array of `VkBufferCopy` structures specifying
///     the regions to copy.
///
/// Each region in `pRegions` is copied from the source buffer to the same region of
/// the destination buffer. `srcBuffer` and `dstBuffer` can: be the same buffer or
/// alias the same memory, but the result is undefined if the copy regions overlap
/// in memory.
///
pub fn vkCmdCopyBuffer(
  commandBuffer: VkCommandBuffer,
  srcBuffer: VkBuffer,
  dstBuffer: VkBuffer,
  pRegions: &[VkBufferCopy],
) {
  unsafe {
    let regionCount = pRegions.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdCopyBuffer.unwrap()(
        commandBuffer.as_raw(),
        srcBuffer.as_raw(),
        dstBuffer.as_raw(),
        regionCount,
        pRegions.as_raw(),
      )
    })
  }
}

/// Copy data between images
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `srcImage` is the source image.
///
///   - `srcImageLayout` is the current layout of the source image subresource.
///
///   - `dstImage` is the destination image.
///
///   - `dstImageLayout` is the current layout of the destination image subresource.
///
///   - `regionCount` is the number of regions to copy.
///
///   - `pRegions` is a pointer to an array of `VkImageCopy` structures specifying
///     the regions to copy.
///
/// Each region in `pRegions` is copied from the source image to the same region of
/// the destination image. `srcImage` and `dstImage` can: be the same image or alias
/// the same memory.
///
/// The formats of `srcImage` and `dstImage` must: be compatible. Formats are
/// considered compatible if their element size is the same between both formats.
/// For example, `VK_FORMAT_R8G8B8A8_UNORM` is compatible with `VK_FORMAT_R32_UINT`
/// because both texels are 4 bytes in size. Depth/stencil formats must: match
/// exactly.
///
/// If the format of `srcImage` or `dstImage` is a [*multi-planar* image
/// format](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion), regions of each
/// plane to be copied must: be specified separately using the `srcSubresource` and
/// `dstSubresource` members of the `VkImageCopy` structure. In this case, the
/// `aspectMask` of the `srcSubresource` or `dstSubresource` that refers to the
/// multi-planar image must: be `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`,
/// `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR`, or `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`. For the
/// purposes of `vkCmdCopyImage`, each plane of a multi-planar image is treated as
/// having the format listed in [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-compatible-planes) for the
/// plane identified by the `aspectMask` of the corresponding subresource. This
/// applies both to `VkFormat` and to coordinates used in the copy, which correspond
/// to texels in the *plane* rather than how these texels map to coordinates in the
/// image as a whole.
///
/// > **Note**
/// >
/// > For example, the `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` plane of a
/// > `VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR` image is compatible with an image of
/// > format `VK_FORMAT_R8G8_UNORM` and (less usefully) with the
/// > `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` plane of an image of format
/// > `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR`, as each texel is 2
/// > bytes in size.
///
/// `vkCmdCopyImage` allows copying between size-compatible compressed and
/// uncompressed internal formats. Formats are size-compatible if the element size
/// of the uncompressed format is equal to the element size (compressed texel block
/// size) of the compressed format. Such a copy does not perform on-the-fly
/// compression or decompression. When copying from an uncompressed format to a
/// compressed format, each texel of uncompressed data of the source image is copied
/// as a raw value to the corresponding compressed texel block of the destination
/// image. When copying from a compressed format to an uncompressed format, each
/// compressed texel block of the source image is copied as a raw value to the
/// corresponding texel of uncompressed data in the destination image. Thus, for
/// example, it is legal to copy between a 128-bit uncompressed format and a
/// compressed format which has a 128-bit sized compressed texel block representing
/// 4{times}4 texels (using 8 bits per texel), or between a 64-bit uncompressed
/// format and a compressed format which has a 64-bit sized compressed texel block
/// representing 4{times}4 texels (using 4 bits per texel).
///
/// When copying between compressed and uncompressed formats the `extent` members
/// represent the texel dimensions of the source image and not the destination. When
/// copying from a compressed image to an uncompressed image the image texel
/// dimensions written to the uncompressed image will be source extent divided by
/// the compressed texel block dimensions. When copying from an uncompressed image
/// to a compressed image the image texel dimensions written to the compressed image
/// will be the source extent multiplied by the compressed texel block dimensions.
/// In both cases the number of bytes read and the number of bytes written will be
/// identical.
///
/// Copying to or from block-compressed images is typically done in multiples of the
/// compressed texel block size. For this reason the `extent` must: be a multiple of
/// the compressed texel block dimension. There is one exception to this rule which
/// is required: to handle compressed images created with dimensions that are not a
/// multiple of the compressed texel block dimensions: if the `srcImage` is
/// compressed, then:
///
///   - If `extent`.width is not a multiple of the compressed texel block width,
///     then (`extent`.width + `srcOffset`.x) must: equal the image subresource
///     width.
///
///   - If `extent`.height is not a multiple of the compressed texel block height,
///     then (`extent`.height + `srcOffset`.y) must: equal the image subresource
///     height.
///
///   - If `extent`.depth is not a multiple of the compressed texel block depth,
///     then (`extent`.depth + `srcOffset`.z) must: equal the image subresource
///     depth.
///
/// Similarly, if the `dstImage` is compressed, then:
///
///   - If `extent`.width is not a multiple of the compressed texel block width,
///     then (`extent`.width + `dstOffset`.x) must: equal the image subresource
///     width.
///
///   - If `extent`.height is not a multiple of the compressed texel block height,
///     then (`extent`.height + `dstOffset`.y) must: equal the image subresource
///     height.
///
///   - If `extent`.depth is not a multiple of the compressed texel block depth,
///     then (`extent`.depth + `dstOffset`.z) must: equal the image subresource
///     depth.
///
/// This allows the last compressed texel block of the image in each non-multiple
/// dimension to be included as a source or destination of the copy.
///
/// “etext:\_422” image formats that are not
/// [*multi-planar*](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion) are
/// treated as having a 2{times}1 compressed texel block for the purposes of these
/// rules.
///
/// `vkCmdCopyImage` can: be used to copy image data between multisample images, but
/// both images must: have the same number of samples.
///
pub fn vkCmdCopyImage(
  commandBuffer: VkCommandBuffer,
  srcImage: VkImage,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage,
  dstImageLayout: VkImageLayout,
  pRegions: &[VkImageCopy],
) {
  unsafe {
    let regionCount = pRegions.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdCopyImage.unwrap()(
        commandBuffer.as_raw(),
        srcImage.as_raw(),
        srcImageLayout,
        dstImage.as_raw(),
        dstImageLayout,
        regionCount,
        pRegions.as_raw(),
      )
    })
  }
}

/// Copy regions of an image, potentially performing format conversion,
///
/// To copy regions of a source image into a destination image, potentially
/// performing format conversion, arbitrary scaling, and filtering.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `srcImage` is the source image.
///
///   - `srcImageLayout` is the layout of the source image subresources for the
///     blit.
///
///   - `dstImage` is the destination image.
///
///   - `dstImageLayout` is the layout of the destination image subresources for the
///     blit.
///
///   - `regionCount` is the number of regions to blit.
///
///   - `pRegions` is a pointer to an array of `VkImageBlit` structures specifying
///     the regions to blit.
///
///   - `filter` is a `VkFilter` specifying the filter to apply if the blits require
///     scaling.
///
/// `vkCmdBlitImage` must: not be used for multisampled source or destination
/// images. Use `vkCmdResolveImage` for this purpose.
///
/// As the sizes of the source and destination extents can: differ in any dimension,
/// texels in the source extent are scaled and filtered to the destination extent.
/// Scaling occurs via the following operations:
///
///   - For each destination texel, the integer coordinate of that texel is
///     converted to an unnormalized texture coordinate, using the effective inverse
///     of the equations described in [unnormalized to integer
///     conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-unnormalized-to-integer):
///
///       -
///         u<sub>base</sub> = i + {onehalf}
///
///       -
///         v<sub>base</sub> = j + {onehalf}
///
///       -
///         w<sub>base</sub> = k + {onehalf}
///
///   - These base coordinates are then offset by the first destination offset:
///
///       -
///         u<sub>offset</sub> = u<sub>base</sub> - x<sub>dst0</sub>
///
///       -
///         v<sub>offset</sub> = v<sub>base</sub> - y<sub>dst0</sub>
///
///       -
///         w<sub>offset</sub> = w<sub>base</sub> - z<sub>dst0</sub>
///
///       -
///         a<sub>offset</sub> = a - `baseArrayCount`<sub>dst</sub>
///
///   - The scale is determined from the source and destination regions, and applied
///     to the offset coordinates:
///
///       -
///         scale\_u = (x<sub>src1</sub> - x<sub>src0</sub>) / (x<sub>dst1</sub> -
///         x<sub>dst0</sub>)
///
///       -
///         scale\_v = (y<sub>src1</sub> - y<sub>src0</sub>) / (y<sub>dst1</sub> -
///         y<sub>dst0</sub>)
///
///       -
///         scale\_w = (z<sub>src1</sub> - z<sub>src0</sub>) / (z<sub>dst1</sub> -
///         z<sub>dst0</sub>)
///
///       -
///         u<sub>scaled</sub> = u<sub>offset</sub> \* scale<sub>u</sub>
///
///       -
///         v<sub>scaled</sub> = v<sub>offset</sub> \* scale<sub>v</sub>
///
///       -
///         w<sub>scaled</sub> = w<sub>offset</sub> \* scale<sub>w</sub>
///
///   - Finally the source offset is added to the scaled coordinates, to determine
///     the final unnormalized coordinates used to sample from `srcImage`:
///
///       -
///         u = u<sub>scaled</sub> + x<sub>src0</sub>
///
///       -
///         v = v<sub>scaled</sub> + y<sub>src0</sub>
///
///       -
///         w = w<sub>scaled</sub> + z<sub>src0</sub>
///
///       -
///         q = `mipLevel`
///
///       -
///         a = a<sub>offset</sub> + `baseArrayCount`<sub>src</sub>
///
/// These coordinates are used to sample from the source image, as described in
/// [Image Operations chapter](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures), with the filter mode equal to that of
/// `filter`, a mipmap mode of `VK_SAMPLER_MIPMAP_MODE_NEAREST` and an address mode
/// of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`. Implementations must: clamp at the
/// edge of the source image, and may: additionally clamp to the edge of the source
/// region.
///
/// > **Note**
/// >
/// > Due to allowable rounding errors in the generation of the source texture
/// > coordinates, it is not always possible to guarantee exactly which source
/// > texels will be sampled for a given blit. As rounding errors are implementation
/// > dependent, the exact results of a blitting operation are also implementation
/// > dependent.
///
/// Blits are done layer by layer starting with the `baseArrayLayer` member of
/// `srcSubresource` for the source and `dstSubresource` for the destination.
/// `layerCount` layers are blitted to the destination image.
///
/// 3D textures are blitted slice by slice. Slices in the source region bounded by
/// `srcOffsets`\[0\].`z` and `srcOffsets`\[1\].`z` are copied to slices in the
/// destination region bounded by `dstOffsets`\[0\].`z` and `dstOffsets`\[1\].`z`.
/// For each destination slice, a source **z** coordinate is linearly interpolated
/// between `srcOffsets`\[0\].`z` and `srcOffsets`\[1\].`z`. If the `filter`
/// parameter is `VK_FILTER_LINEAR` then the value sampled from the source image is
/// taken by doing linear filtering using the interpolated **z** coordinate. If
/// `filter` parameter is `VK_FILTER_NEAREST` then value sampled from the source
/// image is taken from the single nearest slice (with undefined rounding mode).
///
/// The following filtering and conversion rules apply:
///
///   - Integer formats can: only be converted to other integer formats with the
///     same signedness.
///
///   - No format conversion is supported between depth/stencil images. The formats
///     must: match.
///
///   - Format conversions on unorm, snorm, unscaled and packed float formats of the
///     copied aspect of the image are performed by first converting the pixels to
///     float values.
///
///   - For sRGB source formats, nonlinear RGB values are converted to linear
///     representation prior to filtering.
///
///   - After filtering, the float values are first clamped and then cast to the
///     destination image format. In case of sRGB destination format, linear RGB
///     values are converted to nonlinear representation before writing the pixel to
///     the image.
///
/// Signed and unsigned integers are converted by first clamping to the
/// representable range of the destination format, then casting the value.
///
pub fn vkCmdBlitImage(
  commandBuffer: VkCommandBuffer,
  srcImage: VkImage,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage,
  dstImageLayout: VkImageLayout,
  pRegions: &[VkImageBlit],
  filter: VkFilter,
) {
  unsafe {
    let regionCount = pRegions.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBlitImage.unwrap()(
        commandBuffer.as_raw(),
        srcImage.as_raw(),
        srcImageLayout,
        dstImage.as_raw(),
        dstImageLayout,
        regionCount,
        pRegions.as_raw(),
        filter,
      )
    })
  }
}

/// Copy data from a buffer into an image
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `srcBuffer` is the source buffer.
///
///   - `dstImage` is the destination image.
///
///   - `dstImageLayout` is the layout of the destination image subresources for the
///     copy.
///
///   - `regionCount` is the number of regions to copy.
///
///   - `pRegions` is a pointer to an array of `VkBufferImageCopy` structures
///     specifying the regions to copy.
///
/// Each region in `pRegions` is copied from the specified region of the source
/// buffer to the specified region of the destination image.
///
/// If the format of `dstImage` is a [multi-planar image
/// format](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion)), regions of each
/// plane to be a target of a copy must: be specified separately using the
/// `pRegions` member of the `VkBufferImageCopy` structure. In this case, the
/// `aspectMask` of `imageSubresource` must: be `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`,
/// `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR`, or `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`. For the
/// purposes of `vkCmdCopyBufferToImage`, each plane of a multi-planar image is
/// treated as having the format listed in
/// [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-compatible-planes) for the plane identified by the
/// `aspectMask` of the corresponding subresource. This applies both to `VkFormat`
/// and to coordinates used in the copy, which correspond to texels in the *plane*
/// rather than how these texels map to coordinates in the image as a whole.
///
pub fn vkCmdCopyBufferToImage(
  commandBuffer: VkCommandBuffer,
  srcBuffer: VkBuffer,
  dstImage: VkImage,
  dstImageLayout: VkImageLayout,
  pRegions: &[VkBufferImageCopy],
) {
  unsafe {
    let regionCount = pRegions.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdCopyBufferToImage.unwrap()(
        commandBuffer.as_raw(),
        srcBuffer.as_raw(),
        dstImage.as_raw(),
        dstImageLayout,
        regionCount,
        pRegions.as_raw(),
      )
    })
  }
}

/// Copy image data into a buffer
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `srcImage` is the source image.
///
///   - `srcImageLayout` is the layout of the source image subresources for the
///     copy.
///
///   - `dstBuffer` is the destination buffer.
///
///   - `regionCount` is the number of regions to copy.
///
///   - `pRegions` is a pointer to an array of `VkBufferImageCopy` structures
///     specifying the regions to copy.
///
/// Each region in `pRegions` is copied from the specified region of the source
/// image to the specified region of the destination buffer.
///
/// If the `VkFormat` of `srcImage` is a [multi-planar image
/// format](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion), regions of each
/// plane to be a source of a copy must: be specified separately using the
/// `pRegions` member of the `VkBufferImageCopy` structure. In this case, the
/// `aspectMask` of `imageSubresource` must: be `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`,
/// `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR`, or `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`. For the
/// purposes of `vkCmdCopyBufferToImage`, each plane of a multi-planar image is
/// treated as having the format listed in
/// [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-compatible-planes) for the plane identified by the
/// `aspectMask` of the corresponding subresource. This applies both to `VkFormat`
/// and to coordinates used in the copy, which correspond to texels in the *plane*
/// rather than how these texels map to coordinates in the image as a whole.
///
pub fn vkCmdCopyImageToBuffer(
  commandBuffer: VkCommandBuffer,
  srcImage: VkImage,
  srcImageLayout: VkImageLayout,
  dstBuffer: VkBuffer,
  pRegions: &[VkBufferImageCopy],
) {
  unsafe {
    let regionCount = pRegions.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdCopyImageToBuffer.unwrap()(
        commandBuffer.as_raw(),
        srcImage.as_raw(),
        srcImageLayout,
        dstBuffer.as_raw(),
        regionCount,
        pRegions.as_raw(),
      )
    })
  }
}

/// Update a buffer's contents from host memory
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `dstBuffer` is a handle to the buffer to be updated.
///
///   - `dstOffset` is the byte offset into the buffer to start updating, and must:
///     be a multiple of 4.
///
///   - `dataSize` is the number of bytes to update, and must: be a multiple of 4.
///
///   - `pData` is a pointer to the source data for the buffer update, and must: be
///     at least `dataSize` bytes in size.
///
/// `dataSize` must: be less than or equal to 65536 bytes. For larger updates,
/// applications can: use buffer to buffer [copies](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#copies-buffers).
///
/// > **Note**
/// >
/// > Buffer updates performed with `vkCmdUpdateBuffer` first copy the data into
/// > command buffer memory when the command is recorded (which requires additional
/// > storage and may incur an additional allocation), and then copy the data from
/// > the command buffer into `dstBuffer` when the command is executed on a device.
/// >
/// > The additional cost of this functionality compared to [buffer to buffer
/// > copies](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#copies-buffers) means it is only recommended for very small amounts
/// > of data, and is why it is limited to only 65536 bytes.
/// >
/// > Applications can: work around this by issuing multiple `vkCmdUpdateBuffer`
/// > commands to different ranges of the same buffer, but it is strongly
/// > recommended that they should: not.
///
/// The source data is copied from the user pointer to the command buffer when the
/// command is called.
///
/// `vkCmdUpdateBuffer` is only allowed outside of a render pass. This command is
/// treated as “transfer” operation, for the purposes of synchronization barriers.
/// The `VK_BUFFER_USAGE_TRANSFER_DST_BIT` must: be specified in `usage` of
/// `VkBufferCreateInfo` in order for the buffer to be compatible with
/// `vkCmdUpdateBuffer`.
///
pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, pData: &[u8]) {
  unsafe {
    let dataSize = pData.len() as VkDeviceSize;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdUpdateBuffer.unwrap()(
        commandBuffer.as_raw(),
        dstBuffer.as_raw(),
        dstOffset,
        dataSize,
        pData.as_raw() as *const c_void,
      )
    })
  }
}

/// Fill a region of a buffer with a fixed value
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `dstBuffer` is the buffer to be filled.
///
///   - `dstOffset` is the byte offset into the buffer at which to start filling,
///     and must: be a multiple of 4.
///
///   - `size` is the number of bytes to fill, and must: be either a multiple of 4,
///     or `VK_WHOLE_SIZE` to fill the range from `offset` to the end of the buffer.
///     If `VK_WHOLE_SIZE` is used and the remaining size of the buffer is not a
///     multiple of 4, then the nearest smaller multiple is used.
///
///   - `data` is the 4-byte word written repeatedly to the buffer to fill `size`
///     bytes of data. The data word is written to memory according to the host
///     endianness.
///
/// `vkCmdFillBuffer` is treated as “transfer” operation for the purposes of
/// synchronization barriers. The `VK_BUFFER_USAGE_TRANSFER_DST_BIT` must: be
/// specified in `usage` of `VkBufferCreateInfo` in order for the buffer to be
/// compatible with `vkCmdFillBuffer`.
///
pub fn vkCmdFillBuffer(
  commandBuffer: VkCommandBuffer,
  dstBuffer: VkBuffer,
  dstOffset: VkDeviceSize,
  size: VkDeviceSize,
  data: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdFillBuffer.unwrap()(
        commandBuffer.as_raw(),
        dstBuffer.as_raw(),
        dstOffset,
        size,
        data,
      )
    })
  }
}

/// Clear regions of a color image
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `image` is the image to be cleared.
///
///   - `imageLayout` specifies the current layout of the image subresource ranges
///     to be cleared, and must: be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`,
///     `VK_IMAGE_LAYOUT_GENERAL` or `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`.
///
///   - `pColor` is a pointer to a `VkClearColorValue` structure that contains the
///     values the image subresource ranges will be cleared to (see
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#clears-values) below).
///
///   - `rangeCount` is the number of image subresource range structures in
///     `pRanges`.
///
///   - `pRanges` points to an array of `VkImageSubresourceRange` structures that
///     describe a range of mipmap levels, array layers, and aspects to be cleared,
///     as described in [Image Views](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-image-views). The `aspectMask` of
///     all image subresource ranges must: only include `VK_IMAGE_ASPECT_COLOR_BIT`.
///
/// Each specified range in `pRanges` is cleared to the value specified by `pColor`.
///
pub fn vkCmdClearColorImage(
  commandBuffer: VkCommandBuffer,
  image: VkImage,
  imageLayout: VkImageLayout,
  pColor: &VkClearColorValue,
  pRanges: &[VkImageSubresourceRange],
) {
  unsafe {
    let rangeCount = pRanges.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdClearColorImage.unwrap()(
        commandBuffer.as_raw(),
        image.as_raw(),
        imageLayout,
        pColor.as_raw(),
        rangeCount,
        pRanges.as_raw(),
      )
    })
  }
}

/// Fill regions of a combined depth/stencil image
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `image` is the image to be cleared.
///
///   - `imageLayout` specifies the current layout of the image subresource ranges
///     to be cleared, and must: be `VK_IMAGE_LAYOUT_GENERAL` or
///     `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`.
///
///   - `pDepthStencil` is a pointer to a `VkClearDepthStencilValue` structure that
///     contains the values the depth and stencil image subresource ranges will be
///     cleared to (see [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#clears-values) below).
///
///   - `rangeCount` is the number of image subresource range structures in
///     `pRanges`.
///
///   - `pRanges` points to an array of `VkImageSubresourceRange` structures that
///     describe a range of mipmap levels, array layers, and aspects to be cleared,
///     as described in [Image Views](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-image-views). The `aspectMask` of
///     each image subresource range in `pRanges` can: include
///     `VK_IMAGE_ASPECT_DEPTH_BIT` if the image format has a depth component, and
///     `VK_IMAGE_ASPECT_STENCIL_BIT` if the image format has a stencil component.
///     `pDepthStencil` is a pointer to a `VkClearDepthStencilValue` structure that
///     contains the values the image subresource ranges will be cleared to (see
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#clears-values) below).
///
pub fn vkCmdClearDepthStencilImage(
  commandBuffer: VkCommandBuffer,
  image: VkImage,
  imageLayout: VkImageLayout,
  pDepthStencil: &VkClearDepthStencilValue,
  pRanges: &[VkImageSubresourceRange],
) {
  unsafe {
    let rangeCount = pRanges.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdClearDepthStencilImage.unwrap()(
        commandBuffer.as_raw(),
        image.as_raw(),
        imageLayout,
        pDepthStencil.as_raw(),
        rangeCount,
        pRanges.as_raw(),
      )
    })
  }
}

/// Clear regions within currently bound framebuffer attachments
///
/// To clear one or more regions of color and depth/stencil attachments inside a
/// render pass instance.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `attachmentCount` is the number of entries in the `pAttachments` array.
///
///   - `pAttachments` is a pointer to an array of `VkClearAttachment` structures
///     defining the attachments to clear and the clear values to use.
///
///   - `rectCount` is the number of entries in the `pRects` array.
///
///   - `pRects` points to an array of `VkClearRect` structures defining regions
///     within each selected attachment to clear.
///
/// `vkCmdClearAttachments` can: clear multiple regions of each attachment used in
/// the current subpass of a render pass instance. This command must: be called only
/// inside a render pass instance, and implicitly selects the images to clear based
/// on the current framebuffer attachments and the command parameters.
///
pub fn vkCmdClearAttachments(
  commandBuffer: VkCommandBuffer,
  pAttachments: &[VkClearAttachment],
  pRects: &[VkClearRect],
) {
  unsafe {
    let attachmentCount = pAttachments.len() as u32;
    let rectCount = pRects.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdClearAttachments.unwrap()(
        commandBuffer.as_raw(),
        attachmentCount,
        pAttachments.as_raw(),
        rectCount,
        pRects.as_raw(),
      )
    })
  }
}

/// Resolve regions of an image
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `srcImage` is the source image.
///
///   - `srcImageLayout` is the layout of the source image subresources for the
///     resolve.
///
///   - `dstImage` is the destination image.
///
///   - `dstImageLayout` is the layout of the destination image subresources for the
///     resolve.
///
///   - `regionCount` is the number of regions to resolve.
///
///   - `pRegions` is a pointer to an array of `VkImageResolve` structures
///     specifying the regions to resolve.
///
/// During the resolve the samples corresponding to each pixel location in the
/// source are converted to a single sample before being written to the destination.
/// If the source formats are floating-point or normalized types, the sample values
/// for each pixel are resolved in an implementation-dependent manner. If the source
/// formats are integer types, a single sample’s value is selected for each pixel.
///
/// `srcOffset` and `dstOffset` select the initial `x`, `y`, and `z` offsets in
/// texels of the sub-regions of the source and destination image data. `extent` is
/// the size in texels of the source image to resolve in `width`, `height` and
/// `depth`.
///
/// Resolves are done layer by layer starting with `baseArrayLayer` member of
/// `srcSubresource` for the source and `dstSubresource` for the destination.
/// `layerCount` layers are resolved to the destination image.
///
pub fn vkCmdResolveImage(
  commandBuffer: VkCommandBuffer,
  srcImage: VkImage,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage,
  dstImageLayout: VkImageLayout,
  pRegions: &[VkImageResolve],
) {
  unsafe {
    let regionCount = pRegions.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdResolveImage.unwrap()(
        commandBuffer.as_raw(),
        srcImage.as_raw(),
        srcImageLayout,
        dstImage.as_raw(),
        dstImageLayout,
        regionCount,
        pRegions.as_raw(),
      )
    })
  }
}

/// Set an event object to signaled state
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `event` is the event that will be signaled.
///
///   - `stageMask` specifies the [source stage
///     mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages) used to determine when the `event`
///     is signaled.
///
/// When `vkCmdSetEvent` is submitted to a queue, it defines an execution dependency
/// on commands that were submitted before it, and defines an event signal operation
/// which sets the event to the signaled state.
///
/// The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes) includes
/// every command previously submitted to the same queue, including those in the
/// same command buffer and batch. The synchronization scope is limited to
/// operations on the pipeline stages determined by the [source stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `stageMask`.
///
/// The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes)
/// includes only the event signal operation.
///
/// If `event` is already in the signaled state when `vkCmdSetEvent` is executed on
/// the device, then `vkCmdSetEvent` has no effect, no event signal operation
/// occurs, and no execution dependency is generated.
///
pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetEvent.unwrap()(commandBuffer.as_raw(), event.as_raw(), stageMask)
    })
  }
}

/// Reset an event object to non-signaled state
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `event` is the event that will be unsignaled.
///
///   - `stageMask` is a bitmask of `VkPipelineStageFlagBits` specifying the [source
///     stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages) used to determine when the
///     `event` is unsignaled.
///
/// When `vkCmdResetEvent` is submitted to a queue, it defines an execution
/// dependency on commands that were submitted before it, and defines an event
/// unsignal operation which resets the event to the unsignaled state.
///
/// The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes) includes
/// every command previously submitted to the same queue, including those in the
/// same command buffer and batch. The synchronization scope is limited to
/// operations on the pipeline stages determined by the [source stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `stageMask`.
///
/// The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes)
/// includes only the event unsignal operation.
///
/// If `event` is already in the unsignaled state when `vkCmdResetEvent` is executed
/// on the device, then `vkCmdResetEvent` has no effect, no event unsignal operation
/// occurs, and no execution dependency is generated.
///
pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdResetEvent.unwrap()(commandBuffer.as_raw(), event.as_raw(), stageMask)
    })
  }
}

/// Wait for one or more events and insert a set of memory
///
/// To wait for one or more events to enter the signaled state on a device, call:
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `eventCount` is the length of the `pEvents` array.
///
///   - `pEvents` is an array of event object handles to wait on.
///
///   - `srcStageMask` is a bitmask of `VkPipelineStageFlagBits` specifying the
///     [source stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages).
///
///   - `dstStageMask` is a bitmask of `VkPipelineStageFlagBits` specifying the
///     [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages).
///
///   - `memoryBarrierCount` is the length of the `pMemoryBarriers` array.
///
///   - `pMemoryBarriers` is a pointer to an array of `VkMemoryBarrier` structures.
///
///   - `bufferMemoryBarrierCount` is the length of the `pBufferMemoryBarriers`
///     array.
///
///   - `pBufferMemoryBarriers` is a pointer to an array of `VkBufferMemoryBarrier`
///     structures.
///
///   - `imageMemoryBarrierCount` is the length of the `pImageMemoryBarriers` array.
///
///   - `pImageMemoryBarriers` is a pointer to an array of `VkImageMemoryBarrier`
///     structures.
///
/// When `vkCmdWaitEvents` is submitted to a queue, it defines a memory dependency
/// between prior event signal operations on the same queue or the host, and
/// subsequent commands. `vkCmdWaitEvents` must: not be used to wait on event signal
/// operations occuring on other queues.
///
/// The first synchronization scope only includes event signal operations that
/// operate on members of `pEvents`, and the operations that happened-before the
/// event signal operations. Event signal operations performed by `vkCmdSetEvent`
/// that were previously submitted to the same queue are included in the first
/// synchronization scope, if the [logically
/// latest](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-order) pipeline stage in their
/// `stageMask` parameter is [logically
/// earlier](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-order) than or equal to the [logically
/// latest](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-order) pipeline stage in
/// `srcStageMask`. Event signal operations performed by `vkSetEvent` are only
/// included in the first synchronization scope if `VK_PIPELINE_STAGE_HOST_BIT` is
/// included in `srcStageMask`.
///
/// The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes)
/// includes commands subsequently submitted to the same queue, including those in
/// the same command buffer and batch. The second synchronization scope is limited
/// to operations on the pipeline stages determined by the [destination stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `dstStageMask`.
///
/// The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is limited
/// to access in the pipeline stages determined by the [source stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `srcStageMask`.
/// Within that, the first access scope only includes the first access scopes
/// defined by elements of the `pMemoryBarriers`, `pBufferMemoryBarriers` and
/// `pImageMemoryBarriers` arrays, which each define a set of [memory
/// barriers](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-memory-barriers). If no memory barriers are
/// specified, then the first access scope includes no accesses.
///
/// The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
/// limited to access in the pipeline stages determined by the [destination stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `dstStageMask`.
/// Within that, the second access scope only includes the second access scopes
/// defined by elements of the `pMemoryBarriers`, `pBufferMemoryBarriers` and
/// `pImageMemoryBarriers` arrays, which each define a set of [memory
/// barriers](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-memory-barriers). If no memory barriers are
/// specified, then the second access scope includes no accesses.
///
/// > **Note**
/// >
/// > `vkCmdWaitEvents` is used with `vkCmdSetEvent` to define a memory dependency
/// > between two sets of action commands, roughly in the same way as pipeline
/// > barriers, but split into two commands such that work between the two may:
/// > execute unhindered.
///
/// > **Note**
/// >
/// > Applications should: be careful to avoid race conditions when using events.
/// > There is no direct ordering guarantee between a `vkCmdResetEvent` command and
/// > a `vkCmdWaitEvents` command submitted after it, so some other execution
/// > dependency must: be included between these commands (e.g. a semaphore).
///
pub fn vkCmdWaitEvents(
  commandBuffer: VkCommandBuffer,
  pEvents: &[VkEvent],
  srcStageMask: VkPipelineStageFlags,
  dstStageMask: VkPipelineStageFlags,
  pMemoryBarriers: &[VkMemoryBarrier],
  pBufferMemoryBarriers: &[VkBufferMemoryBarrier],
  pImageMemoryBarriers: &[VkImageMemoryBarrier],
) {
  unsafe {
    let eventCount = pEvents.len() as u32;
    let memoryBarrierCount = pMemoryBarriers.len() as u32;
    let bufferMemoryBarrierCount = pBufferMemoryBarriers.len() as u32;
    let imageMemoryBarrierCount = pImageMemoryBarriers.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdWaitEvents.unwrap()(
        commandBuffer.as_raw(),
        eventCount,
        pEvents.as_raw(),
        srcStageMask,
        dstStageMask,
        memoryBarrierCount,
        pMemoryBarriers.as_raw(),
        bufferMemoryBarrierCount,
        pBufferMemoryBarriers.as_raw(),
        imageMemoryBarrierCount,
        pImageMemoryBarriers.as_raw(),
      )
    })
  }
}

/// Insert a memory dependency
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `srcStageMask` is a bitmask of `VkPipelineStageFlagBits` specifying the
///     [source stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
///
///   - `dstStageMask` is a bitmask of `VkPipelineStageFlagBits` specifying the
///     [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
///
///   - `dependencyFlags` is a bitmask of `VkDependencyFlagBits` specifying how
///     execution and memory dependencies are formed.
///
///   - `memoryBarrierCount` is the length of the `pMemoryBarriers` array.
///
///   - `pMemoryBarriers` is a pointer to an array of `VkMemoryBarrier` structures.
///
///   - `bufferMemoryBarrierCount` is the length of the `pBufferMemoryBarriers`
///     array.
///
///   - `pBufferMemoryBarriers` is a pointer to an array of `VkBufferMemoryBarrier`
///     structures.
///
///   - `imageMemoryBarrierCount` is the length of the `pImageMemoryBarriers` array.
///
///   - `pImageMemoryBarriers` is a pointer to an array of `VkImageMemoryBarrier`
///     structures.
///
/// When `vkCmdPipelineBarrier` is submitted to a queue, it defines a memory
/// dependency between commands that were submitted before it, and those submitted
/// after it.
///
/// If `vkCmdPipelineBarrier` was recorded outside a render pass instance, the first
/// [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes) includes every
/// command submitted to the same queue before it, including those in the same
/// command buffer and batch. If `vkCmdPipelineBarrier` was recorded inside a render
/// pass instance, the first synchronization scope includes only commands submitted
/// before it within the same subpass. In either case, the first synchronization
/// scope is limited to operations on the pipeline stages determined by the [source
/// stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `srcStageMask`.
///
/// If `vkCmdPipelineBarrier` was recorded outside a render pass instance, the
/// second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes) includes
/// every command submitted to the same queue after it, including those in the same
/// command buffer and batch. If `vkCmdPipelineBarrier` was recorded inside a render
/// pass instance, the second synchronization scope includes only commands submitted
/// after it within the same subpass. In either case, the second synchronization
/// scope is limited to operations on the pipeline stages determined by the
/// [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by
/// `dstStageMask`.
///
/// The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is limited
/// to access in the pipeline stages determined by the [source stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `srcStageMask`.
/// Within that, the first access scope only includes the first access scopes
/// defined by elements of the `pMemoryBarriers`, `pBufferMemoryBarriers` and
/// `pImageMemoryBarriers` arrays, which each define a set of [memory
/// barriers](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-memory-barriers). If no memory barriers are
/// specified, then the first access scope includes no accesses.
///
/// The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
/// limited to access in the pipeline stages determined by the [destination stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `dstStageMask`.
/// Within that, the second access scope only includes the second access scopes
/// defined by elements of the `pMemoryBarriers`, `pBufferMemoryBarriers` and
/// `pImageMemoryBarriers` arrays, which each define a set of [memory
/// barriers](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-memory-barriers). If no memory barriers are
/// specified, then the second access scope includes no accesses.
///
/// If `dependencyFlags` includes `VK_DEPENDENCY_BY_REGION_BIT`, then any dependency
/// between [framebuffer-space](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-framebuffer-regions) pipeline
/// stages is [framebuffer-local](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-framebuffer-regions) - otherwise
/// it is [framebuffer-global](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-framebuffer-regions).
///
pub fn vkCmdPipelineBarrier(
  commandBuffer: VkCommandBuffer,
  srcStageMask: VkPipelineStageFlags,
  dstStageMask: VkPipelineStageFlags,
  dependencyFlags: VkDependencyFlags,
  pMemoryBarriers: &[VkMemoryBarrier],
  pBufferMemoryBarriers: &[VkBufferMemoryBarrier],
  pImageMemoryBarriers: &[VkImageMemoryBarrier],
) {
  unsafe {
    let memoryBarrierCount = pMemoryBarriers.len() as u32;
    let bufferMemoryBarrierCount = pBufferMemoryBarriers.len() as u32;
    let imageMemoryBarrierCount = pImageMemoryBarriers.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdPipelineBarrier.unwrap()(
        commandBuffer.as_raw(),
        srcStageMask,
        dstStageMask,
        dependencyFlags,
        memoryBarrierCount,
        pMemoryBarriers.as_raw(),
        bufferMemoryBarrierCount,
        pBufferMemoryBarriers.as_raw(),
        imageMemoryBarrierCount,
        pImageMemoryBarriers.as_raw(),
      )
    })
  }
}

/// Begin a query
///
///   - `commandBuffer` is the command buffer into which this command will be
///     recorded.
///
///   - `queryPool` is the query pool that will manage the results of the query.
///
///   - `query` is the query index within the query pool that will contain the
///     results.
///
///   - `flags` is a bitmask of `VkQueryControlFlagBits` specifying constraints on
///     the types of queries that can: be performed.
///
/// If the `queryType` of the pool is `VK_QUERY_TYPE_OCCLUSION` and `flags` contains
/// `VK_QUERY_CONTROL_PRECISE_BIT`, an implementation must: return a result that
/// matches the actual number of samples passed. This is described in more detail in
/// [Occlusion Queries](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-occlusion).
///
/// After beginning a query, that query is considered *active* within the command
/// buffer it was called in until that same query is ended. Queries active in a
/// primary command buffer when secondary command buffers are executed are
/// considered active for those secondary command buffers.
///
pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBeginQuery.unwrap()(commandBuffer.as_raw(), queryPool.as_raw(), query, flags)
    })
  }
}

/// Ends a query
///
/// To end a query after the set of desired draw or dispatch commands is executed.
///
///   - `commandBuffer` is the command buffer into which this command will be
///     recorded.
///
///   - `queryPool` is the query pool that is managing the results of the query.
///
///   - `query` is the query index within the query pool where the result is stored.
///
/// As queries operate asynchronously, ending a query does not immediately set the
/// query’s status to available. A query is considered *finished* when the final
/// results of the query are ready to be retrieved by `vkGetQueryPoolResults` and
/// `vkCmdCopyQueryPoolResults`, and this is when the query’s status is set to
/// available.
///
/// Once a query is ended the query must: finish in finite time, unless the state of
/// the query is changed using other commands, e.g. by issuing a reset of the query.
///
pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdEndQuery.unwrap()(commandBuffer.as_raw(), queryPool.as_raw(), query)
    })
  }
}

/// Reset queries in a query pool
///
///   - `commandBuffer` is the command buffer into which this command will be
///     recorded.
///
///   - `queryPool` is the handle of the query pool managing the queries being
///     reset.
///
///   - `firstQuery` is the initial query index to reset.
///
///   - `queryCount` is the number of queries to reset.
///
/// When executed on a queue, this command sets the status of query indices
/// \[`firstQuery`, `firstQuery` + `queryCount` - 1\] to unavailable.
///
pub fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdResetQueryPool.unwrap()(
        commandBuffer.as_raw(),
        queryPool.as_raw(),
        firstQuery,
        queryCount,
      )
    })
  }
}

/// Write a device timestamp into a query object
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `pipelineStage` is one of the `VkPipelineStageFlagBits`, specifying a stage
///     of the pipeline.
///
///   - `queryPool` is the query pool that will manage the timestamp.
///
///   - `query` is the query within the query pool that will contain the timestamp.
///
/// `vkCmdWriteTimestamp` latches the value of the timer when all previous commands
/// have completed executing as far as the specified pipeline stage, and writes the
/// timestamp value to memory. When the timestamp value is written, the availability
/// status of the query is set to available.
///
/// > **Note**
/// >
/// > If an implementation is unable to detect completion and latch the timer at any
/// > specific stage of the pipeline, it may: instead do so at any logically later
/// > stage.
///
/// `vkCmdCopyQueryPoolResults` can: then be called to copy the timestamp value from
/// the query pool into buffer memory, with ordering and synchronization behavior
/// equivalent to how other queries operate. Timestamp values can: also be retrieved
/// from the query pool using `vkGetQueryPoolResults`. As with other queries, the
/// query must: be reset using `vkCmdResetQueryPool` before requesting the timestamp
/// value be written to it.
///
/// While `vkCmdWriteTimestamp` can: be called inside or outside of a render pass
/// instance, `vkCmdCopyQueryPoolResults` must: only be called outside of a render
/// pass instance.
///
/// Timestamps may: only be meaningfully compared if they are written by commands
/// submitted to the same queue.
///
/// > **Note**
/// >
/// > An example of such a comparison is determining the execution time of a
/// > sequence of commands.
///
/// If `vkCmdWriteTimestamp` is called while executing a render pass instance that
/// has multiview enabled, the timestamp uses N consecutive query indices in the
/// query pool (starting at `query`) where N is the number of bits set in the view
/// mask of the subpass the command is executed in. The resulting query values are
/// determined by an implementation-dependent choice of one of the following
/// behaviors:
///
///   - The first query is a timestamp value and (if more than one bit is set in the
///     view mask) zero is written to the remaining queries. If two timestamps are
///     written in the same subpass, the sum of the execution time of all views
///     between those commands is the difference between the first query written by
///     each command.
///
///   - All N queries are timestamp values. If two timestamps are written in the
///     same subpass, the sum of the execution time of all views between those
///     commands is the sum of the difference between corresponding queries written
///     by each command. The difference between corresponding queries may: be the
///     execution time of a single view.
///
/// In either case, the application can: sum the differences between all N queries
/// to determine the total execution time.
///
pub fn vkCmdWriteTimestamp(
  commandBuffer: VkCommandBuffer,
  pipelineStage: VkPipelineStageFlagBits,
  queryPool: VkQueryPool,
  query: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdWriteTimestamp.unwrap()(
        commandBuffer.as_raw(),
        pipelineStage,
        queryPool.as_raw(),
        query,
      )
    })
  }
}

/// Copy the results of queries in a query pool to a buffer object
///
/// To copy query statuses and numerical results directly to buffer memory, call.
///
///   - `commandBuffer` is the command buffer into which this command will be
///     recorded.
///
///   - `queryPool` is the query pool managing the queries containing the desired
///     results.
///
///   - `firstQuery` is the initial query index.
///
///   - `queryCount` is the number of queries. `firstQuery` and `queryCount`
///     together define a range of queries.
///
///   - `dstBuffer` is a `VkBuffer` object that will receive the results of the copy
///     command.
///
///   - `dstOffset` is an offset into `dstBuffer`.
///
///   - `stride` is the stride in bytes between results for individual queries
///     within `dstBuffer`. The required size of the backing memory for `dstBuffer`
///     is determined as described above for `vkGetQueryPoolResults`.
///
///   - `flags` is a bitmask of `VkQueryResultFlagBits` specifying how and when
///     results are returned.
///
/// `vkCmdCopyQueryPoolResults` is guaranteed to see the effect of previous uses of
/// `vkCmdResetQueryPool` in the same queue, without any additional synchronization.
/// Thus, the results will always reflect the most recent use of the query.
///
/// `flags` has the same possible values described above for the `flags` parameter
/// of `vkGetQueryPoolResults`, but the different style of execution causes some
/// subtle behavioral differences. Because `vkCmdCopyQueryPoolResults` executes in
/// order with respect to other query commands, there is less ambiguity about which
/// use of a query is being requested.
///
/// If no bits are set in `flags`, results for all requested queries in the
/// available state are written as 32-bit unsigned integer values, and nothing is
/// written for queries in the unavailable state.
///
/// If `VK_QUERY_RESULT_64_BIT` is set, the results are written as an array of
/// 64-bit unsigned integer values as described for `vkGetQueryPoolResults`.
///
/// If `VK_QUERY_RESULT_WAIT_BIT` is set, the implementation will wait for each
/// query’s status to be in the available state before retrieving the numerical
/// results for that query. This is guaranteed to reflect the most recent use of the
/// query on the same queue, assuming that the query is not being simultaneously
/// used by other queues. If the query does not become available in a finite amount
/// of time (e.g. due to not issuing a query since the last reset), a
/// `VK_ERROR_DEVICE_LOST` error may: occur.
///
/// Similarly, if `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` is set and
/// `VK_QUERY_RESULT_WAIT_BIT` is not set, the availability is guaranteed to reflect
/// the most recent use of the query on the same queue, assuming that the query is
/// not being simultaneously used by other queues. As with `vkGetQueryPoolResults`,
/// implementations must: guarantee that if they return a non-zero availability
/// value, then the numerical results are valid.
///
/// If `VK_QUERY_RESULT_PARTIAL_BIT` is set, `VK_QUERY_RESULT_WAIT_BIT` is not set,
/// and the query’s status is unavailable, an intermediate result value between zero
/// and the final result value is written for that query.
///
/// `VK_QUERY_RESULT_PARTIAL_BIT` must: not be used if the pool’s `queryType` is
/// `VK_QUERY_TYPE_TIMESTAMP`.
///
/// `vkCmdCopyQueryPoolResults` is considered to be a transfer operation, and its
/// writes to buffer memory must: be synchronized using
/// `VK_PIPELINE_STAGE_TRANSFER_BIT` and `VK_ACCESS_TRANSFER_WRITE_BIT` before using
/// the results.
///
pub fn vkCmdCopyQueryPoolResults(
  commandBuffer: VkCommandBuffer,
  queryPool: VkQueryPool,
  firstQuery: u32,
  queryCount: u32,
  dstBuffer: VkBuffer,
  dstOffset: VkDeviceSize,
  stride: VkDeviceSize,
  flags: VkQueryResultFlags,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdCopyQueryPoolResults.unwrap()(
        commandBuffer.as_raw(),
        queryPool.as_raw(),
        firstQuery,
        queryCount,
        dstBuffer.as_raw(),
        dstOffset,
        stride,
        flags,
      )
    })
  }
}

/// Update the values of push constants
///
///   - `commandBuffer` is the command buffer in which the push constant update will
///     be recorded.
///
///   - `layout` is the pipeline layout used to program the push constant updates.
///
///   - `stageFlags` is a bitmask of `VkShaderStageFlagBits` specifying the shader
///     stages that will use the push constants in the updated range.
///
///   - `offset` is the start offset of the push constant range to update, in units
///     of bytes.
///
///   - `size` is the size of the push constant range to update, in units of bytes.
///
///   - `pValues` is an array of `size` bytes containing the new push constant
///     values.
///
pub fn vkCmdPushConstants(
  commandBuffer: VkCommandBuffer,
  layout: VkPipelineLayout,
  stageFlags: VkShaderStageFlags,
  offset: u32,
  pValues: &[u8],
) {
  unsafe {
    let size = pValues.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdPushConstants.unwrap()(
        commandBuffer.as_raw(),
        layout.as_raw(),
        stageFlags,
        offset,
        size,
        pValues.as_raw() as *const c_void,
      )
    })
  }
}

/// Begin a new render pass
///
///   - `commandBuffer` is the command buffer in which to record the command.
///
///   - `pRenderPassBegin` is a pointer to a `VkRenderPassBeginInfo` structure
///     (defined below) which indicates the render pass to begin an instance of, and
///     the framebuffer the instance uses.
///
///   - `contents` is a `VkSubpassContents` value specifying how the commands in the
///     first subpass will be provided.
///
/// After beginning a render pass instance, the command buffer is ready to record
/// the commands for the first subpass of that render pass.
///
pub fn vkCmdBeginRenderPass(
  commandBuffer: VkCommandBuffer,
  pRenderPassBegin: &VkRenderPassBeginInfo,
  contents: VkSubpassContents,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBeginRenderPass.unwrap()(commandBuffer.as_raw(), pRenderPassBegin.as_raw(), contents)
    })
  }
}

/// Transition to the next subpass of a render pass
///
/// To transition to the next subpass in the render pass instance after recording
/// the commands for a subpass.
///
///   - `commandBuffer` is the command buffer in which to record the command.
///
///   - `contents` specifies how the commands in the next subpass will be provided,
///     in the same fashion as the corresponding parameter of
///     `vkCmdBeginRenderPass`.
///
/// The subpass index for a render pass begins at zero when `vkCmdBeginRenderPass`
/// is recorded, and increments each time `vkCmdNextSubpass` is recorded.
///
/// Moving to the next subpass automatically performs any multisample resolve
/// operations in the subpass being ended. End-of-subpass multisample resolves are
/// treated as color attachment writes for the purposes of synchronization. That is,
/// they are considered to execute in the
/// `VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage and their writes
/// are synchronized with `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`. Synchronization
/// between rendering within a subpass and any resolve operations at the end of the
/// subpass occurs automatically, without need for explicit dependencies or pipeline
/// barriers. However, if the resolve attachment is also used in a different
/// subpass, an explicit dependency is needed.
///
/// After transitioning to the next subpass, the application can: record the
/// commands for that subpass.
///
pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdNextSubpass.unwrap()(commandBuffer.as_raw(), contents)
    })
  }
}

/// End the current render pass
///
/// To record a command to end a render pass instance after recording the commands
/// for the last subpass.
///
///   - `commandBuffer` is the command buffer in which to end the current render
///     pass instance.
///
/// Ending a render pass instance performs any multisample resolve operations on the
/// final subpass.
///
pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdEndRenderPass.unwrap()(commandBuffer.as_raw())
    })
  }
}

/// Execute a secondary command buffer from a primary command buffer
///
/// A secondary command buffer must: not be directly submitted to a queue. Instead,
/// secondary command buffers are recorded to execute as part of a primary command
/// buffer with the command.
///
///   - `commandBuffer` is a handle to a primary command buffer that the secondary
///     command buffers are executed in.
///
///   - `commandBufferCount` is the length of the `pCommandBuffers` array.
///
///   - `pCommandBuffers` is an array of secondary command buffer handles, which are
///     recorded to execute in the primary command buffer in the order they are
///     listed in the array.
///
/// If any element of `pCommandBuffers` was not recorded with the
/// `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT` flag, and it was recorded into
/// any other primary command buffer which is currently in the [executable or
/// recording state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle), that primary command buffer becomes
/// [invalid](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
///
pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, pCommandBuffers: &[VkCommandBuffer]) {
  unsafe {
    let commandBufferCount = pCommandBuffers.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdExecuteCommands.unwrap()(
        commandBuffer.as_raw(),
        commandBufferCount,
        pCommandBuffers.as_raw(),
      )
    })
  }
}

// feature: VK_KHR_surface

/// Destroy a VkSurfaceKHR object
///
/// To destroy a `VkSurfaceKHR` object, call:
///
///   - `instance` is the instance used to create the surface.
///
///   - `surface` is the surface to destroy.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
/// Destroying a `VkSurfaceKHR` merely severs the connection between Vulkan and the
/// native surface, and does not imply destroying the native surface, closing a
/// window, or similar behavior.
///
#[cfg(feature = "VK_KHR_surface")]
pub fn vkDestroySurfaceKHR(
  instance: VkInstance,
  surface: Option<VkSurfaceKHR>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkInstanceDispatchTable::with(instance, |_t| {
      _t.vkDestroySurfaceKHR.unwrap()(instance.as_raw(), surface.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Query if presentation is supported
///
/// To determine whether a queue family of a physical device supports presentation
/// to a given surface, call:
///
///   - `physicalDevice` is the physical device.
///
///   - `queueFamilyIndex` is the queue family.
///
///   - `surface` is the surface.
///
///   - `pSupported` is a pointer to a basetype:VkBool32, which is set to `VK_TRUE`
///     to indicate support, and `VK_FALSE` otherwise.
///
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfaceSupportKHR(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: u32,
  surface: VkSurfaceKHR,
) -> Result<VkBool32, VkResult> {
  unsafe {
    let mut pSupported: VkBool32 = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceSurfaceSupportKHR.unwrap()(
        physicalDevice.as_raw(),
        queueFamilyIndex,
        surface.as_raw(),
        &mut pSupported,
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSupported);
    })
  }
}

/// Query surface capabilities
///
/// To query the basic capabilities of a surface, needed in order to create a
/// swapchain, call:
///
///   - `physicalDevice` is the physical device that will be associated with the
///     swapchain to be created, as described for `vkCreateSwapchainKHR`.
///
///   - `surface` is the surface that will be associated with the swapchain.
///
///   - `pSurfaceCapabilities` is a pointer to an instance of the
///     `VkSurfaceCapabilitiesKHR` structure in which the capabilities are returned.
///
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
) -> Result<VkSurfaceCapabilitiesKHR, VkResult> {
  unsafe {
    let mut pSurfaceCapabilities: VkSurfaceCapabilitiesKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceSurfaceCapabilitiesKHR.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        (&mut pSurfaceCapabilities).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurfaceCapabilities);
    })
  }
}

/// Query color formats supported by surface
///
/// To query the supported swapchain format-color space pairs for a surface, call:
///
///   - `physicalDevice` is the physical device that will be associated with the
///     swapchain to be created, as described for `vkCreateSwapchainKHR`.
///
///   - `surface` is the surface that will be associated with the swapchain.
///
///   - `pSurfaceFormatCount` is a pointer to an integer related to the number of
///     format pairs available or queried, as described below.
///
///   - `pSurfaceFormats` is either `NULL` or a pointer to an array of
///     `VkSurfaceFormatKHR` structures.
///
/// If `pSurfaceFormats` is `NULL`, then the number of format pairs supported for
/// the given `surface` is returned in `pSurfaceFormatCount`. The number of format
/// pairs supported will be greater than or equal to 1. Otherwise,
/// `pSurfaceFormatCount` must: point to a variable set by the user to the number of
/// elements in the `pSurfaceFormats` array, and on return the variable is
/// overwritten with the number of structures actually written to `pSurfaceFormats`.
/// If the value of `pSurfaceFormatCount` is less than the number of format pairs
/// supported, at most `pSurfaceFormatCount` structures will be written. If
/// `pSurfaceFormatCount` is smaller than the number of format pairs supported for
/// the given `surface`, `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS` to
/// indicate that not all the available values were returned.
///
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
) -> Result<Vec<VkSurfaceFormatKHR>, VkResult> {
  unsafe {
    let mut pSurfaceFormatCount: u32 = 0;
    let mut pSurfaceFormats: Vec<VkSurfaceFormatKHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetPhysicalDeviceSurfaceFormatsKHR.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pSurfaceFormatCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pSurfaceFormatCount == 0 {
        return Ok(pSurfaceFormats);
      }
      pSurfaceFormats = Vec::with_capacity(pSurfaceFormatCount as usize);
      let _r = _t.vkGetPhysicalDeviceSurfaceFormatsKHR.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pSurfaceFormatCount,
        pSurfaceFormats.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pSurfaceFormats.set_len(pSurfaceFormatCount as usize);
      return Ok(pSurfaceFormats);
    })
  }
}

/// Query supported presentation modes
///
/// To query the supported presentation modes for a surface, call:
///
///   - `physicalDevice` is the physical device that will be associated with the
///     swapchain to be created, as described for `vkCreateSwapchainKHR`.
///
///   - `surface` is the surface that will be associated with the swapchain.
///
///   - `pPresentModeCount` is a pointer to an integer related to the number of
///     presentation modes available or queried, as described below.
///
///   - `pPresentModes` is either `NULL` or a pointer to an array of
///     `VkPresentModeKHR` values, indicating the supported presentation modes.
///
/// If `pPresentModes` is `NULL`, then the number of presentation modes supported
/// for the given `surface` is returned in `pPresentModeCount`. Otherwise,
/// `pPresentModeCount` must: point to a variable set by the user to the number of
/// elements in the `pPresentModes` array, and on return the variable is overwritten
/// with the number of values actually written to `pPresentModes`. If the value of
/// `pPresentModeCount` is less than the number of presentation modes supported, at
/// most `pPresentModeCount` values will be written. If `pPresentModeCount` is
/// smaller than the number of presentation modes supported for the given `surface`,
/// `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS` to indicate that not
/// all the available values were returned.
///
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
) -> Result<Vec<VkPresentModeKHR>, VkResult> {
  unsafe {
    let mut pPresentModeCount: u32 = 0;
    let mut pPresentModes: Vec<VkPresentModeKHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetPhysicalDeviceSurfacePresentModesKHR.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pPresentModeCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPresentModeCount == 0 {
        return Ok(pPresentModes);
      }
      pPresentModes = Vec::with_capacity(pPresentModeCount as usize);
      let _r = _t.vkGetPhysicalDeviceSurfacePresentModesKHR.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pPresentModeCount,
        pPresentModes.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pPresentModes.set_len(pPresentModeCount as usize);
      return Ok(pPresentModes);
    })
  }
}

// feature: VK_KHR_swapchain

/// Create a swapchain
///
/// To create a swapchain, call:
///
///   - `device` is the device to create the swapchain for.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkSwapchainCreateInfoKHR`
///     structure specifying the parameters of the created swapchain.
///
///   - `pAllocator` is the allocator used for host memory allocated for the
///     swapchain object when there is no more specific allocator available (see
///     [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSwapchain` is a pointer to a `VkSwapchainKHR` handle in which the created
///     swapchain object will be returned.
///
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkCreateSwapchainKHR(
  device: VkDevice,
  pCreateInfo: &VkSwapchainCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSwapchainKHR, VkResult> {
  unsafe {
    let mut pSwapchain: VkSwapchainKHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateSwapchainKHR.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSwapchain).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSwapchain);
    })
  }
}

/// Destroy a swapchain object
///
/// To destroy a swapchain object call:
///
///   - `device` is the `VkDevice` associated with `swapchain`.
///
///   - `swapchain` is the swapchain to destroy.
///
///   - `pAllocator` is the allocator used for host memory allocated for the
///     swapchain object when there is no more specific allocator available (see
///     [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
/// The application must: not destroy a swapchain until after completion of all
/// outstanding operations on images that were acquired from the swapchain.
/// `swapchain` and all associated `VkImage` handles are destroyed, and must: not be
/// acquired or used any more by the application. The memory of each `VkImage` will
/// only be freed after that image is no longer used by the presentation engine. For
/// example, if one image of the swapchain is being displayed in a window, the
/// memory for that image may: not be freed until the window is destroyed, or
/// another swapchain is created for the window. Destroying the swapchain does not
/// invalidate the parent `VkSurfaceKHR`, and a new swapchain can: be created with
/// it.
///
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkDestroySwapchainKHR(
  device: VkDevice,
  swapchain: Option<VkSwapchainKHR>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySwapchainKHR.unwrap()(device.as_raw(), swapchain.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Obtain the array of presentable images associated with a swapchain
///
/// To obtain the array of presentable images associated with a swapchain, call:
///
///   - `device` is the device associated with `swapchain`.
///
///   - `swapchain` is the swapchain to query.
///
///   - `pSwapchainImageCount` is a pointer to an integer related to the number of
///     presentable images available or queried, as described below.
///
///   - `pSwapchainImages` is either `NULL` or a pointer to an array of `VkImage`
///     handles.
///
/// If `pSwapchainImages` is `NULL`, then the number of presentable images for
/// `swapchain` is returned in `pSwapchainImageCount`. Otherwise,
/// `pSwapchainImageCount` must: point to a variable set by the user to the number
/// of elements in the `pSwapchainImages` array, and on return the variable is
/// overwritten with the number of structures actually written to
/// `pSwapchainImages`. If the value of `pSwapchainImageCount` is less than the
/// number of presentable images for `swapchain`, at most `pSwapchainImageCount`
/// structures will be written. If `pSwapchainImageCount` is smaller than the number
/// of presentable images for `swapchain`, `VK_INCOMPLETE` will be returned instead
/// of `VK_SUCCESS` to indicate that not all the available values were returned.
///
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkGetSwapchainImagesKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> Result<Vec<VkImage>, VkResult> {
  unsafe {
    let mut pSwapchainImageCount: u32 = 0;
    let mut pSwapchainImages: Vec<VkImage> = Vec::new();
    VkDeviceDispatchTable::with(device, |_t| loop {
      let _r = _t.vkGetSwapchainImagesKHR.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        &mut pSwapchainImageCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pSwapchainImageCount == 0 {
        return Ok(pSwapchainImages);
      }
      pSwapchainImages = Vec::with_capacity(pSwapchainImageCount as usize);
      let _r = _t.vkGetSwapchainImagesKHR.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        &mut pSwapchainImageCount,
        pSwapchainImages.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pSwapchainImages.set_len(pSwapchainImageCount as usize);
      return Ok(pSwapchainImages);
    })
  }
}

/// Retrieve the index of the next available presentable image
///
/// To acquire an available presentable image to use, and retrieve the index of that
/// image, call:
///
///   - `device` is the device associated with `swapchain`.
///
///   - `swapchain` is the non-retired swapchain from which an image is being
///     acquired.
///
///   - `timeout` indicates how long the function waits, in nanoseconds, if no image
///     is available.
///
///   - `semaphore` is `VK_NULL_HANDLE` or a semaphore to signal.
///
///   - `fence` is `VK_NULL_HANDLE` or a fence to signal.
///
///   - `pImageIndex` is a pointer to a `uint32_t` that is set to the index of the
///     next image to use (i.e. an index into the array of images returned by
///     `vkGetSwapchainImagesKHR`).
///
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkAcquireNextImageKHR(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  timeout: u64,
  semaphore: Option<VkSemaphore>,
  fence: Option<VkFence>,
) -> Result<u32, VkResult> {
  unsafe {
    let mut pImageIndex: u32 = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAcquireNextImageKHR.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        timeout,
        semaphore.as_raw(),
        fence.as_raw(),
        &mut pImageIndex,
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pImageIndex);
    })
  }
}

/// Queue an image for presentation
///
/// After queueing all rendering commands and transitioning the image to the correct
/// layout, to queue an image for presentation, call:
///
///   - `queue` is a queue that is capable of presentation to the target surface’s
///     platform on the same device as the image’s swapchain.
///
///   - `pPresentInfo` is a pointer to an instance of the `VkPresentInfoKHR`
///     structure specifying the parameters of the presentation.
///
/// > **Note**
/// >
/// > There is no requirement for an application to present images in the same order
/// > that they were acquired - applications can arbitrarily present any image that
/// > is currently acquired.
///
/// Any writes to memory backing the images referenced by the `pImageIndices` and
/// `pSwapchains` members of `pPresentInfo`, that are available before
/// `vkQueuePresentKHR` is executed, are automatically made visible to the read
/// access performed by the presentation engine. This automatic visibility operation
/// for an image happens-after the semaphore signal operation, and happens-before
/// the presentation engine accesses the image.
///
/// Queueing an image for presentation defines a set of *queue operations*,
/// including waiting on the semaphores and submitting a presentation request to the
/// presentation engine. However, the scope of this set of queue operations does not
/// include the actual processing of the image by the presentation engine.
///
/// If `vkQueuePresentKHR` fails to enqueue the corresponding set of queue
/// operations, it may: return `VK_ERROR_OUT_OF_HOST_MEMORY` or
/// `VK_ERROR_OUT_OF_DEVICE_MEMORY`. If it does, the implementation must: ensure
/// that the state and contents of any resources or synchronization primitives
/// referenced is unaffected by the call or its failure.
///
/// If `vkQueuePresentKHR` fails in such a way that the implementation can: not make
/// that guarantee, the implementation must: return `VK_ERROR_DEVICE_LOST`.
///
/// However, if the presentation request is rejected by the presentation engine with
/// an error `VK_ERROR_OUT_OF_DATE_KHR` or `VK_ERROR_SURFACE_LOST_KHR`, the set of
/// queue operations are still considered to be enqueued and thus any semaphore to
/// be waited on gets unsignaled when the corresponding queue operation is complete.
///
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: &VkPresentInfoKHR) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(queue, |_t| {
      _t.vkQueuePresentKHR.unwrap()(queue.as_raw(), pPresentInfo.as_raw())
    })
  }
}

// feature: VK_KHR_display

/// Query information about the available displays
///
/// Various functions are provided for enumerating the available display devices
/// present on a Vulkan physical device. To query information about the available
/// displays, call:
///
///   - `physicalDevice` is a physical device.
///
///   - `pPropertyCount` is a pointer to an integer related to the number of display
///     devices available or queried, as described below.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkDisplayPropertiesKHR` structures.
///
/// If `pProperties` is `NULL`, then the number of display devices available for
/// `physicalDevice` is returned in `pPropertyCount`. Otherwise, `pPropertyCount`
/// must: point to a variable set by the user to the number of elements in the
/// `pProperties` array, and on return the variable is overwritten with the number
/// of structures actually written to `pProperties`. If the value of
/// `pPropertyCount` is less than the number of display devices for
/// `physicalDevice`, at most `pPropertyCount` structures will be written. If
/// `pPropertyCount` is smaller than the number of display devices available for
/// `physicalDevice`, `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS` to
/// indicate that not all the available values were returned.
///
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(
  physicalDevice: VkPhysicalDevice,
) -> Result<Vec<VkDisplayPropertiesKHR>, VkResult> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkDisplayPropertiesKHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetPhysicalDeviceDisplayPropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkGetPhysicalDeviceDisplayPropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Query the plane properties
///
/// Images are presented to individual planes on a display. Devices must: support at
/// least one plane on each display. Planes can: be stacked and blended to composite
/// multiple images on one display. Devices may: support only a fixed stacking order
/// and fixed mapping between planes and displays, or they may: allow arbitrary
/// application specified stacking orders and mappings between planes and displays.
/// To query the properties of device display planes, call:
///
///   - `physicalDevice` is a physical device.
///
///   - `pPropertyCount` is a pointer to an integer related to the number of display
///     planes available or queried, as described below.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkDisplayPlanePropertiesKHR` structures.
///
/// If `pProperties` is `NULL`, then the number of display planes available for
/// `physicalDevice` is returned in `pPropertyCount`. Otherwise, `pPropertyCount`
/// must: point to a variable set by the user to the number of elements in the
/// `pProperties` array, and on return the variable is overwritten with the number
/// of structures actually written to `pProperties`. If the value of
/// `pPropertyCount` is less than the number of display planes for `physicalDevice`,
/// at most `pPropertyCount` structures will be written.
///
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
  physicalDevice: VkPhysicalDevice,
) -> Result<Vec<VkDisplayPlanePropertiesKHR>, VkResult> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkDisplayPlanePropertiesKHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetPhysicalDeviceDisplayPlanePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkGetPhysicalDeviceDisplayPlanePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Query the list of displays a plane supports
///
/// To determine which displays a plane is usable with, call
///
///   - `physicalDevice` is a physical device.
///
///   - `planeIndex` is the plane which the application wishes to use, and must: be
///     in the range \[0, physical device plane count - 1\].
///
///   - `pDisplayCount` is a pointer to an integer related to the number of displays
///     available or queried, as described below.
///
///   - `pDisplays` is either `NULL` or a pointer to an array of `VkDisplayKHR`
///     handles.
///
/// If `pDisplays` is `NULL`, then the number of displays usable with the specified
/// `planeIndex` for `physicalDevice` is returned in `pDisplayCount`. Otherwise,
/// `pDisplayCount` must: point to a variable set by the user to the number of
/// elements in the `pDisplays` array, and on return the variable is overwritten
/// with the number of handles actually written to `pDisplays`. If the value of
/// `pDisplayCount` is less than the number of display planes for `physicalDevice`,
/// at most `pDisplayCount` handles will be written. If `pDisplayCount` is smaller
/// than the number of displays usable with the specified `planeIndex` for
/// `physicalDevice`, `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS` to
/// indicate that not all the available values were returned.
///
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetDisplayPlaneSupportedDisplaysKHR(
  physicalDevice: VkPhysicalDevice,
  planeIndex: u32,
) -> Result<Vec<VkDisplayKHR>, VkResult> {
  unsafe {
    let mut pDisplayCount: u32 = 0;
    let mut pDisplays: Vec<VkDisplayKHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetDisplayPlaneSupportedDisplaysKHR.unwrap()(
        physicalDevice.as_raw(),
        planeIndex,
        &mut pDisplayCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pDisplayCount == 0 {
        return Ok(pDisplays);
      }
      pDisplays = Vec::with_capacity(pDisplayCount as usize);
      let _r = _t.vkGetDisplayPlaneSupportedDisplaysKHR.unwrap()(
        physicalDevice.as_raw(),
        planeIndex,
        &mut pDisplayCount,
        pDisplays.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pDisplays.set_len(pDisplayCount as usize);
      return Ok(pDisplays);
    })
  }
}

/// Query the set of mode properties supported by the display
///
/// Each display has one or more supported modes associated with it by default.
/// These built-in modes are queried by calling:
///
///   - `physicalDevice` is the physical device associated with `display`.
///
///   - `display` is the display to query.
///
///   - `pPropertyCount` is a pointer to an integer related to the number of display
///     modes available or queried, as described below.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkDisplayModePropertiesKHR` structures.
///
/// If `pProperties` is `NULL`, then the number of display modes available on the
/// specified `display` for `physicalDevice` is returned in `pPropertyCount`.
/// Otherwise, `pPropertyCount` must: point to a variable set by the user to the
/// number of elements in the `pProperties` array, and on return the variable is
/// overwritten with the number of structures actually written to `pProperties`. If
/// the value of `pPropertyCount` is less than the number of display modes for
/// `physicalDevice`, at most `pPropertyCount` structures will be written. If
/// `pPropertyCount` is smaller than the number of display modes available on the
/// specified `display` for `physicalDevice`, `VK_INCOMPLETE` will be returned
/// instead of `VK_SUCCESS` to indicate that not all the available values were
/// returned.
///
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetDisplayModePropertiesKHR(
  physicalDevice: VkPhysicalDevice,
  display: VkDisplayKHR,
) -> Result<Vec<VkDisplayModePropertiesKHR>, VkResult> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkDisplayModePropertiesKHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetDisplayModePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        display.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkGetDisplayModePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        display.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Create a display mode
///
/// Additional modes may: also be created by calling:
///
///   - `physicalDevice` is the physical device associated with `display`.
///
///   - `display` is the display to create an additional mode for.
///
///   - `pCreateInfo` is a `VkDisplayModeCreateInfoKHR` structure describing the new
///     mode to create.
///
///   - `pAllocator` is the allocator used for host memory allocated for the display
///     mode object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pMode` returns the handle of the mode created.
///
#[cfg(feature = "VK_KHR_display")]
pub fn vkCreateDisplayModeKHR(
  physicalDevice: VkPhysicalDevice,
  display: VkDisplayKHR,
  pCreateInfo: &VkDisplayModeCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkDisplayModeKHR, VkResult> {
  unsafe {
    let mut pMode: VkDisplayModeKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkCreateDisplayModeKHR.unwrap()(
        physicalDevice.as_raw(),
        display.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pMode).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pMode);
    })
  }
}

/// Query capabilities of a mode and plane combination
///
/// Applications that wish to present directly to a display must: select which
/// layer, or “plane” of the display they wish to target, and a mode to use with the
/// display. Each display supports at least one plane. The capabilities of a given
/// mode and plane combination are determined by calling:
///
///   - `physicalDevice` is the physical device associated with `display`
///
///   - `mode` is the display mode the application intends to program when using the
///     specified plane. Note this parameter also implicitly specifies a display.
///
///   - `planeIndex` is the plane which the application intends to use with the
///     display, and is less than the number of display planes supported by the
///     device.
///
///   - `pCapabilities` is a pointer to a `VkDisplayPlaneCapabilitiesKHR` structure
///     in which the capabilities are returned.
///
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetDisplayPlaneCapabilitiesKHR(
  physicalDevice: VkPhysicalDevice,
  mode: VkDisplayModeKHR,
  planeIndex: u32,
) -> Result<VkDisplayPlaneCapabilitiesKHR, VkResult> {
  unsafe {
    let mut pCapabilities: VkDisplayPlaneCapabilitiesKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetDisplayPlaneCapabilitiesKHR.unwrap()(
        physicalDevice.as_raw(),
        mode.as_raw(),
        planeIndex,
        (&mut pCapabilities).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pCapabilities);
    })
  }
}

/// Create a slink:VkSurfaceKHR structure representing a display plane and mode
///
/// A complete display configuration includes a mode, one or more display planes and
/// any parameters describing their behavior, and parameters describing some aspects
/// of the images associated with those planes. Display surfaces describe the
/// configuration of a single plane within a complete display configuration. To
/// create a `VkSurfaceKHR` structure for a display surface, call:
///
///   - `instance` is the instance corresponding to the physical device the targeted
///     display is on.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkDisplaySurfaceCreateInfoKHR` structure specifying which mode, plane, and
///     other parameters to use, as described below.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface is
///     returned.
///
#[cfg(feature = "VK_KHR_display")]
pub fn vkCreateDisplayPlaneSurfaceKHR(
  instance: VkInstance,
  pCreateInfo: &VkDisplaySurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateDisplayPlaneSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_KHR_display_swapchain

/// Create multiple swapchains that share presentable images
///
/// When the `VK_KHR_display_swapchain` extension is enabled, multiple swapchains
/// that share presentable images are created by calling:
///
///   - `device` is the device to create the swapchains for.
///
///   - `swapchainCount` is the number of swapchains to create.
///
///   - `pCreateInfos` is a pointer to an array of `VkSwapchainCreateInfoKHR`
///     structures specifying the parameters of the created swapchains.
///
///   - `pAllocator` is the allocator used for host memory allocated for the
///     swapchain objects when there is no more specific allocator available (see
///     [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSwapchains` is a pointer to an array of `VkSwapchainKHR` handles in which
///     the created swapchain objects will be returned.
///
/// `vkCreateSharedSwapchains` is similar to `vkCreateSwapchainKHR`, except that it
/// takes an array of `VkSwapchainCreateInfoKHR` structures, and returns an array of
/// swapchain objects.
///
/// The swapchain creation parameters that affect the properties and number of
/// presentable images must: match between all the swapchains. If the displays used
/// by any of the swapchains do not use the same presentable image layout or are
/// incompatible in a way that prevents sharing images, swapchain creation will fail
/// with the result code `VK_ERROR_INCOMPATIBLE_DISPLAY_KHR`. If any error occurs,
/// no swapchains will be created. Images presented to multiple swapchains must: be
/// re-acquired from all of them before transitioning away from
/// `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`. After destroying one or more of the
/// swapchains, the remaining swapchains and the presentable images can: continue to
/// be used.
///
#[cfg(feature = "VK_KHR_display_swapchain")]
pub fn vkCreateSharedSwapchainsKHR(
  device: VkDevice,
  pCreateInfos: &[VkSwapchainCreateInfoKHR],
  pAllocator: Option<&VkAllocationCallbacks>,
  pSwapchains: &mut [VkSwapchainKHR],
) -> VkResult {
  unsafe {
    let swapchainCount = pCreateInfos.len() as u32;
    assert!(swapchainCount as usize == pSwapchains.len());
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkCreateSharedSwapchainsKHR.unwrap()(
        device.as_raw(),
        swapchainCount,
        pCreateInfos.as_raw(),
        pAllocator.as_raw(),
        pSwapchains.as_raw(),
      )
    })
  }
}

// feature: VK_KHR_xlib_surface

/// Create a slink:VkSurfaceKHR object for an X11 window, using the Xlib client-side
/// library
///
/// To create a `VkSurfaceKHR` object for an X11 window, using the Xlib client-side
/// library, call:
///
///   - `instance` is the instance to associate the surface with.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkXlibSurfaceCreateInfoKHR` structure containing the parameters affecting
///     the creation of the surface object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub fn vkCreateXlibSurfaceKHR(
  instance: VkInstance,
  pCreateInfo: &VkXlibSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateXlibSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

/// Query physical device for presentation to X11 server using Xlib
///
/// To determine whether a queue family of a physical device supports presentation
/// to an X11 server, using the Xlib client-side library, call:
///
///   - `physicalDevice` is the physical device.
///
///   - `queueFamilyIndex` is the queue family index.
///
///   - `dpy` is a pointer to an Xlib `Display` connection to the server.
///
///   - `visualId` is an X11 visual (`VisualID`).
///
/// This platform-specific function can: be called prior to creating a surface.
///
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: u32,
  dpy: *mut wsi::xlib::Display,
  visualID: wsi::xlib::VisualID,
) -> VkBool32 {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceXlibPresentationSupportKHR.unwrap()(
        physicalDevice.as_raw(),
        queueFamilyIndex,
        dpy,
        visualID,
      )
    })
  }
}

// feature: VK_KHR_xcb_surface

/// Create a slink:VkSurfaceKHR object for a X11 window, using the XCB client-side
/// library
///
/// To create a `VkSurfaceKHR` object for an X11 window, using the XCB client-side
/// library, call:
///
///   - `instance` is the instance to associate the surface with.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkXcbSurfaceCreateInfoKHR`
///     structure containing parameters affecting the creation of the surface
///     object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub fn vkCreateXcbSurfaceKHR(
  instance: VkInstance,
  pCreateInfo: &VkXcbSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateXcbSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

/// Query physical device for presentation to X11 server using XCB
///
/// To determine whether a queue family of a physical device supports presentation
/// to an X11 server, using the XCB client-side library, call:
///
///   - `physicalDevice` is the physical device.
///
///   - `queueFamilyIndex` is the queue family index.
///
///   - `connection` is a pointer to an `xcb_connection_t` to the X server.
///     `visual_id` is an X11 visual (`xcb_visualid_t`).
///
/// This platform-specific function can: be called prior to creating a surface.
///
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: u32,
  connection: *mut wsi::xcb::xcb_connection_t,
  visual_id: wsi::xcb::xcb_visualid_t,
) -> VkBool32 {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceXcbPresentationSupportKHR.unwrap()(
        physicalDevice.as_raw(),
        queueFamilyIndex,
        connection,
        visual_id,
      )
    })
  }
}

// feature: VK_KHR_wayland_surface

/// Create a slink:VkSurfaceKHR object for a Wayland window
///
/// To create a `VkSurfaceKHR` object for a Wayland surface, call:
///
///   - `instance` is the instance to associate the surface with.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkWaylandSurfaceCreateInfoKHR` structure containing parameters affecting
///     the creation of the surface object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub fn vkCreateWaylandSurfaceKHR(
  instance: VkInstance,
  pCreateInfo: &VkWaylandSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateWaylandSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

/// Query physical device for presentation to Wayland
///
/// To determine whether a queue family of a physical device supports presentation
/// to a Wayland compositor, call:
///
///   - `physicalDevice` is the physical device.
///
///   - `queueFamilyIndex` is the queue family index.
///
///   - `display` is a pointer to the `wl_display` associated with a Wayland
///     compositor.
///
/// This platform-specific function can: be called prior to creating a surface.
///
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: u32,
  display: *mut wsi::wayland::wl_display,
) -> VkBool32 {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceWaylandPresentationSupportKHR.unwrap()(physicalDevice.as_raw(), queueFamilyIndex, display)
    })
  }
}

// feature: VK_KHR_mir_surface

/// Create a slink:VkSurfaceKHR object for a Mir window
///
/// To create a `VkSurfaceKHR` object for a Mir window, call:
///
///   - `instance` is the instance to associate the surface with.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkMirSurfaceCreateInfoKHR`
///     structure containing parameters affecting the creation of the surface
///     object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub fn vkCreateMirSurfaceKHR(
  instance: VkInstance,
  pCreateInfo: &VkMirSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateMirSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

/// Query physical device for presentation to Mir
///
/// To determine whether a queue family of a physical device supports presentation
/// to the Mir compositor, call:
///
///   - `physicalDevice` is the physical device.
///
///   - `queueFamilyIndex` is the queue family index.
///
///   - `connection` is a pointer to the `MirConnection`, and identifies the desired
///     Mir compositor.
///
/// This platform-specific function can: be called prior to creating a surface.
///
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub fn vkGetPhysicalDeviceMirPresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: u32,
  connection: *mut wsi::mir::MirConnection,
) -> VkBool32 {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceMirPresentationSupportKHR.unwrap()(physicalDevice.as_raw(), queueFamilyIndex, connection)
    })
  }
}

// feature: VK_KHR_android_surface

/// Create a slink:VkSurfaceKHR object for an Android native window
///
/// To create a `VkSurfaceKHR` object for an Android native window, call:
///
///   - `instance` is the instance to associate the surface with.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkAndroidSurfaceCreateInfoKHR` structure containing parameters affecting
///     the creation of the surface object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
/// During the lifetime of a surface created using a particular `ANativeWindow`
/// handle any attempts to create another surface for the same `ANativeWindow` and
/// any attempts to connect to the same `ANativeWindow` through other platform
/// mechanisms will fail.
///
/// > **Note**
/// >
/// > In particular, only one `VkSurfaceKHR` can: exist at a time for a given
/// > window. Similarly, a native window cannot: be used by both a `VkSurfaceKHR`
/// > and `EGLSurface` simultaneously.
///
/// If successful, `vkCreateAndroidSurfaceKHR` increments the ``ANativeWindow’s
/// reference count, and `vkDestroySurfaceKHR`` will decrement it.
///
/// On Android, when a swapchain’s `imageExtent` does not match the surface’s
/// `currentExtent`, the presentable images will be scaled to the surface’s
/// dimensions during presentation. `minImageExtent` is (1,1), and `maxImageExtent`
/// is the maximum image size supported by the consumer. For the system compositor,
/// `currentExtent` is the window size (i.e. the consumer’s preferred size).
///
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub fn vkCreateAndroidSurfaceKHR(
  instance: VkInstance,
  pCreateInfo: &VkAndroidSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateAndroidSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_KHR_win32_surface

/// Create a slink:VkSurfaceKHR object for an Win32 native window
///
/// To create a `VkSurfaceKHR` object for a Win32 window, call:
///
///   - `instance` is the instance to associate the surface with.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkWin32SurfaceCreateInfoKHR` structure containing parameters affecting the
///     creation of the surface object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkCreateWin32SurfaceKHR(
  instance: VkInstance,
  pCreateInfo: &VkWin32SurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateWin32SurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

/// query queue family support for presentation on a Win32 display
///
/// To determine whether a queue family of a physical device supports presentation
/// to the Microsoft Windows desktop, call:
///
///   - `physicalDevice` is the physical device.
///
///   - `queueFamilyIndex` is the queue family index.
///
/// This platform-specific function can: be called prior to creating a surface.
///
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: u32,
) -> VkBool32 {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceWin32PresentationSupportKHR.unwrap()(physicalDevice.as_raw(), queueFamilyIndex)
    })
  }
}

// feature: VK_EXT_debug_report

/// Create a debug report callback object
///
/// Debug report callbacks give more detailed feedback on the application’s use of
/// Vulkan when events of interest occur.
///
/// To register a debug report callback, an application uses
/// `vkCreateDebugReportCallbackEXT`.
///
///   - `instance` the instance the callback will be logged on.
///
///   - `pCreateInfo` points to a `VkDebugReportCallbackCreateInfoEXT` structure
///     which defines the conditions under which this callback will be called.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pCallback` is a pointer to record the `VkDebugReportCallbackEXT` object
///     created.
///
#[cfg(feature = "VK_EXT_debug_report")]
pub fn vkCreateDebugReportCallbackEXT(
  instance: VkInstance,
  pCreateInfo: &VkDebugReportCallbackCreateInfoEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkDebugReportCallbackEXT, VkResult> {
  unsafe {
    let mut pCallback: VkDebugReportCallbackEXT = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateDebugReportCallbackEXT.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pCallback).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pCallback);
    })
  }
}

/// Destroy a debug report callback object
///
///   - `instance` the instance where the callback was created.
///
///   - `callback` the `VkDebugReportCallbackEXT` object to destroy. `callback` is
///     an externally synchronized object and must: not be used on more than one
///     thread at a time. This means that `vkDestroyDebugReportCallbackEXT` must:
///     not be called when a callback is active.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
#[cfg(feature = "VK_EXT_debug_report")]
pub fn vkDestroyDebugReportCallbackEXT(
  instance: VkInstance,
  callback: VkDebugReportCallbackEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkInstanceDispatchTable::with(instance, |_t| {
      _t.vkDestroyDebugReportCallbackEXT.unwrap()(instance.as_raw(), callback.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Inject a message into a debug stream
///
///   - `instance` is the debug stream’s `VkInstance`.
///
///   - `flags` indicates the `VkDebugReportFlagBitsEXT` classification of this
///     event/message.
///
///   - `objectType` is a `VkDebugReportObjectTypeEXT` specifying the type of object
///     being used or created at the time the event was triggered.
///
///   - `object` this is the object where the issue was detected. `object` can: be
///     `VK_NULL_HANDLE` if there is no object associated with the event.
///
///   - `location` is an application defined value.
///
///   - `messageCode` is an application defined value.
///
///   - `pLayerPrefix` is the abbreviation of the component making this
///     event/message.
///
///   - `pMessage` is a null-terminated string detailing the trigger conditions.
///
/// The call will propagate through the layers and generate callback(s) as indicated
/// by the message’s flags. The parameters are passed on to the callback in addition
/// to the `pUserData` value that was defined at the time the callback was
/// registered.
///
#[cfg(feature = "VK_EXT_debug_report")]
pub fn vkDebugReportMessageEXT(
  instance: VkInstance,
  flags: VkDebugReportFlagsEXT,
  objectType: VkDebugReportObjectTypeEXT,
  object: u64,
  location: usize,
  messageCode: i32,
  pLayerPrefix: &AsRef<CStr>,
  pMessage: &AsRef<CStr>,
) {
  unsafe {
    VkInstanceDispatchTable::with(instance, |_t| {
      _t.vkDebugReportMessageEXT.unwrap()(
        instance.as_raw(),
        flags,
        objectType,
        object,
        location,
        messageCode,
        pLayerPrefix.as_raw(),
        pMessage.as_raw(),
      )
    })
  }
}

// feature: VK_EXT_debug_marker

/// Attach arbitrary data to an object
///
/// In addition to setting a name for an object, debugging and validation layers may
/// have uses for additional binary data on a per-object basis that has no other
/// place in the Vulkan API. For example, a `VkShaderModule` could have additional
/// debugging data attached to it to aid in offline shader tracing.
///
///   - `device` is the device that created the object.
///
///   - `pTagInfo` is a pointer to an instance of the
///     `VkDebugMarkerObjectTagInfoEXT` structure specifying the parameters of the
///     tag to attach to the object.
///
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkDebugMarkerSetObjectTagEXT(device: VkDevice, pTagInfo: &VkDebugMarkerObjectTagInfoEXT) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDebugMarkerSetObjectTagEXT.unwrap()(device.as_raw(), pTagInfo.as_raw())
    })
  }
}

/// Give a user-friendly name to an object
///
/// An object can be given a user-friendly name by calling.
///
///   - `device` is the device that created the object.
///
///   - `pNameInfo` is a pointer to an instance of the
///     `VkDebugMarkerObjectNameInfoEXT` structure specifying the parameters of the
///     name to set on the object.
///
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkDebugMarkerSetObjectNameEXT(device: VkDevice, pNameInfo: &VkDebugMarkerObjectNameInfoEXT) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDebugMarkerSetObjectNameEXT.unwrap()(device.as_raw(), pNameInfo.as_raw())
    })
  }
}

/// Open a command buffer marker region
///
/// A marker region can be opened by calling.
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `pMarkerInfo` is a pointer to an instance of the
///     `VkDebugMarkerMarkerInfoEXT` structure specifying the parameters of the
///     marker region to open.
///
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: &VkDebugMarkerMarkerInfoEXT) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerBeginEXT.unwrap()(commandBuffer.as_raw(), pMarkerInfo.as_raw())
    })
  }
}

/// Close a command buffer marker region
///
/// A marker region can be closed by calling.
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
/// An application may: open a marker region in one command buffer and close it in
/// another, or otherwise split marker regions across multiple command buffers or
/// multiple queue submissions. When viewed from the linear series of submissions to
/// a single queue, the calls to `vkCmdDebugMarkerBeginEXT` and
/// `vkCmdDebugMarkerEndEXT` must: be matched and balanced.
///
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerEndEXT.unwrap()(commandBuffer.as_raw())
    })
  }
}

/// Insert a marker label into a command buffer
///
/// A single marker label can be inserted into a command buffer by calling.
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `pMarkerInfo` is a pointer to an instance of the
///     `VkDebugMarkerMarkerInfoEXT` structure specifying the parameters of the
///     marker to insert.
///
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: &VkDebugMarkerMarkerInfoEXT) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerInsertEXT.unwrap()(commandBuffer.as_raw(), pMarkerInfo.as_raw())
    })
  }
}

// feature: VK_AMD_draw_indirect_count

/// Perform an indirect draw with the draw count sourced from a buffer
///
/// To record a non-indexed draw call with a draw call count sourced from a buffer.
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `buffer` is the buffer containing draw parameters.
///
///   - `offset` is the byte offset into `buffer` where parameters begin.
///
///   - `countBuffer` is the buffer containing the draw count.
///
///   - `countBufferOffset` is the byte offset into `countBuffer` where the draw
///     count begins.
///
///   - `maxDrawCount` specifies the maximum number of draws that will be executed.
///     The actual number of executed draw calls is the minimum of the count
///     specified in `countBuffer` and `maxDrawCount`.
///
///   - `stride` is the byte stride between successive sets of draw parameters.
///
/// `vkCmdDrawIndirectCountAMD` behaves similarly to `vkCmdDrawIndirect` except that
/// the draw count is read by the device from a buffer during execution. The command
/// will read an unsigned 32-bit integer from `countBuffer` located at
/// `countBufferOffset` and use this as the draw count.
///
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub fn vkCmdDrawIndirectCountAMD(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: u32,
  stride: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDrawIndirectCountAMD.unwrap()(
        commandBuffer.as_raw(),
        buffer.as_raw(),
        offset,
        countBuffer.as_raw(),
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    })
  }
}

/// Perform an indexed indirect draw with the draw count sourced from a buffer
///
/// To record an indexed draw call with a draw call count sourced from a buffer,
/// call.
///
///   - `commandBuffer` is the command buffer into which the command is recorded.
///
///   - `buffer` is the buffer containing draw parameters.
///
///   - `offset` is the byte offset into `buffer` where parameters begin.
///
///   - `countBuffer` is the buffer containing the draw count.
///
///   - `countBufferOffset` is the byte offset into `countBuffer` where the draw
///     count begins.
///
///   - `maxDrawCount` specifies the maximum number of draws that will be executed.
///     The actual number of executed draw calls is the minimum of the count
///     specified in `countBuffer` and `maxDrawCount`.
///
///   - `stride` is the byte stride between successive sets of draw parameters.
///
/// `vkCmdDrawIndexedIndirectCountAMD` behaves similarly to
/// `vkCmdDrawIndexedIndirect` except that the draw count is read by the device from
/// a buffer during execution. The command will read an unsigned 32-bit integer from
/// `countBuffer` located at `countBufferOffset` and use this as the draw count.
///
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub fn vkCmdDrawIndexedIndirectCountAMD(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: u32,
  stride: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDrawIndexedIndirectCountAMD.unwrap()(
        commandBuffer.as_raw(),
        buffer.as_raw(),
        offset,
        countBuffer.as_raw(),
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    })
  }
}

// feature: VK_KHR_get_physical_device_properties2

/// Reports capabilities of a physical device
///
///   - `physicalDevice` is the physical device from which to query the supported
///     features.
///
///   - `pFeatures` is a pointer to a `VkPhysicalDeviceFeatures2KHR` structure in
///     which the physical device features are returned.
///
/// Each structure in `pFeatures` and its `pNext` chain contain members
/// corresponding to fine-grained features. `vkGetPhysicalDeviceFeatures2KHR` writes
/// each member to a boolean value indicating whether that feature is supported.
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceFeatures2KHR(physicalDevice: VkPhysicalDevice) -> VkPhysicalDeviceFeatures2KHR {
  unsafe {
    let mut pFeatures: VkPhysicalDeviceFeatures2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceFeatures2KHR.unwrap()(physicalDevice.as_raw(), (&mut pFeatures).as_raw());
      return pFeatures;
    })
  }
}

/// Returns properties of a physical device
///
///   - `physicalDevice` is the handle to the physical device whose properties will
///     be queried.
///
///   - `pProperties` points to an instance of the `VkPhysicalDeviceProperties2KHR`
///     structure, that will be filled with returned information.
///
/// Each structure in `pProperties` and its `pNext` chain contain members
/// corresponding to properties or implementation-dependent limits.
/// `vkGetPhysicalDeviceProperties2KHR` writes each member to a value indicating the
/// value of that property or limit.
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceProperties2KHR(physicalDevice: VkPhysicalDevice) -> VkPhysicalDeviceProperties2KHR {
  unsafe {
    let mut pProperties: VkPhysicalDeviceProperties2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceProperties2KHR.unwrap()(physicalDevice.as_raw(), (&mut pProperties).as_raw());
      return pProperties;
    })
  }
}

/// Lists physical device's format capabilities
///
/// To query supported format features which are properties of the physical device.
///
///   - `physicalDevice` is the physical device from which to query the format
///     properties.
///
///   - `format` is the format whose properties are queried.
///
///   - `pFormatProperties` is a pointer to a `VkFormatProperties2KHR` structure in
///     which physical device properties for `format` are returned.
///
/// `vkGetPhysicalDeviceFormatProperties2KHR` behaves similarly to
/// `vkGetPhysicalDeviceFormatProperties`, with the ability to return extended
/// information in a `pNext` chain of output structures.
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceFormatProperties2KHR(
  physicalDevice: VkPhysicalDevice,
  format: VkFormat,
) -> VkFormatProperties2KHR {
  unsafe {
    let mut pFormatProperties: VkFormatProperties2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceFormatProperties2KHR.unwrap()(
        physicalDevice.as_raw(),
        format,
        (&mut pFormatProperties).as_raw(),
      );
      return pFormatProperties;
    })
  }
}

/// Lists physical device's image format capabilities
///
///   - `physicalDevice` is the physical device from which to query the image
///     capabilities.
///
///   - `pImageFormatInfo` points to an instance of the
///     `VkPhysicalDeviceImageFormatInfo2KHR` structure, describing the parameters
///     that would be consumed by `vkCreateImage`.
///
///   - `pImageFormatProperties` points to an instance of the
///     `VkImageFormatProperties2KHR` structure in which capabilities are returned.
///
/// `vkGetPhysicalDeviceImageFormatProperties2KHR` behaves similarly to
/// `vkGetPhysicalDeviceImageFormatProperties`, with the ability to return extended
/// information in a `pNext` chain of output structures.
///
/// If the loader implementation emulates
/// `vkGetPhysicalDeviceImageFormatProperties2KHR` on a device that does not support
/// the extension, and the query involves a structure the loader does not support,
/// `vkGetPhysicalDeviceImageFormatProperties2KHR` returns
/// `VK_ERROR_FORMAT_NOT_SUPPORTED`.
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceImageFormatProperties2KHR(
  physicalDevice: VkPhysicalDevice,
  pImageFormatInfo: &VkPhysicalDeviceImageFormatInfo2KHR,
) -> Result<VkImageFormatProperties2KHR, VkResult> {
  unsafe {
    let mut pImageFormatProperties: VkImageFormatProperties2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceImageFormatProperties2KHR.unwrap()(
        physicalDevice.as_raw(),
        pImageFormatInfo.as_raw(),
        (&mut pImageFormatProperties).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pImageFormatProperties);
    })
  }
}

/// Reports properties of the queues of the specified physical device
///
///   - `physicalDevice` is the handle to the physical device whose properties will
///     be queried.
///
///   - `pQueueFamilyPropertyCount` is a pointer to an integer related to the number
///     of queue families available or queried, as described in
///     `vkGetPhysicalDeviceQueueFamilyProperties`.
///
///   - `pQueueFamilyProperties` is either `NULL` or a pointer to an array of
///     `VkQueueFamilyProperties2KHR` structures.
///
/// `vkGetPhysicalDeviceQueueFamilyProperties2KHR` behaves similarly to
/// `vkGetPhysicalDeviceQueueFamilyProperties`, with the ability to return extended
/// information in a `pNext` chain of output structures.
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(
  physicalDevice: VkPhysicalDevice,
) -> Vec<VkQueueFamilyProperties2KHR> {
  unsafe {
    let mut pQueueFamilyPropertyCount: u32 = 0;
    let mut pQueueFamilyProperties: Vec<VkQueueFamilyProperties2KHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceQueueFamilyProperties2KHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pQueueFamilyPropertyCount,
        ::std::ptr::null_mut(),
      );
      if pQueueFamilyPropertyCount == 0 {
        return pQueueFamilyProperties;
      }
      pQueueFamilyProperties = Vec::with_capacity(pQueueFamilyPropertyCount as usize);
      _t.vkGetPhysicalDeviceQueueFamilyProperties2KHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pQueueFamilyPropertyCount,
        pQueueFamilyProperties.as_mut_slice().as_raw(),
      );
      pQueueFamilyProperties.set_len(pQueueFamilyPropertyCount as usize);
      return pQueueFamilyProperties;
    })
  }
}

/// Reports memory information for the specified physical device
///
///   - `physicalDevice` is the handle to the device to query.
///
///   - `pMemoryProperties` points to an instance of
///     `VkPhysicalDeviceMemoryProperties2KHR` structure in which the properties are
///     returned.
///
/// `vkGetPhysicalDeviceMemoryProperties2KHR` behaves similarly to
/// `vkGetPhysicalDeviceMemoryProperties`, with the ability to return extended
/// information in a `pNext` chain of output structures.
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceMemoryProperties2KHR(
  physicalDevice: VkPhysicalDevice,
) -> VkPhysicalDeviceMemoryProperties2KHR {
  unsafe {
    let mut pMemoryProperties: VkPhysicalDeviceMemoryProperties2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceMemoryProperties2KHR.unwrap()(physicalDevice.as_raw(), (&mut pMemoryProperties).as_raw());
      return pMemoryProperties;
    })
  }
}

/// Retrieve properties of an image format applied to sparse images
///
/// `vkGetPhysicalDeviceSparseImageFormatProperties2KHR` returns an array of
/// `VkSparseImageFormatProperties2KHR`. Each element will describe properties for
/// one set of image aspects that are bound simultaneously in the image. This is
/// usually one element for each aspect in the image, but for interleaved
/// depth/stencil images there is only one element describing the combined aspects.
///
///   - `physicalDevice` is the physical device from which to query the sparse image
///     capabilities.
///
///   - `pFormatInfo` is a pointer to a structure of type
///     `VkPhysicalDeviceSparseImageFormatInfo2KHR` containing input parameters to
///     the command.
///
///   - `pPropertyCount` is a pointer to an integer related to the number of sparse
///     format properties available or queried, as described below.
///
///   - `pProperties` is either `NULL` or a pointer to an array of
///     `VkSparseImageFormatProperties2KHR` structures.
///
/// `vkGetPhysicalDeviceSparseImageFormatProperties2KHR` behaves identically to
/// `vkGetPhysicalDeviceSparseImageFormatProperties`, with the ability to return
/// extended information by adding extension structures to the `pNext` chain of its
/// `pProperties` parameter.
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
  physicalDevice: VkPhysicalDevice,
  pFormatInfo: &VkPhysicalDeviceSparseImageFormatInfo2KHR,
) -> Vec<VkSparseImageFormatProperties2KHR> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkSparseImageFormatProperties2KHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceSparseImageFormatProperties2KHR
        .unwrap()(
        physicalDevice.as_raw(),
        pFormatInfo.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if pPropertyCount == 0 {
        return pProperties;
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      _t.vkGetPhysicalDeviceSparseImageFormatProperties2KHR
        .unwrap()(
        physicalDevice.as_raw(),
        pFormatInfo.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_slice().as_raw(),
      );
      pProperties.set_len(pPropertyCount as usize);
      return pProperties;
    })
  }
}

// feature: VK_AMD_shader_info

/// Get information about a shader in a pipeline
///
/// Information about a particular shader that has been compiled as part of a
/// pipeline object can be extracted by calling.
///
///   - `device` is the device that created `pipeline`.
///
///   - `pipeline` is the target of the query.
///
///   - `shaderStage` identifies the particular shader within the pipeline about
///     which information is being queried.
///
///   - `infoType` describes what kind of information is being queried.
///
///   - `pInfoSize` is a pointer to a value related to the amount of data the query
///     returns, as described below.
///
///   - `pInfo` is either NULL or a pointer to a buffer.
///
/// If `pInfo` is `NULL`, then the maximum size of the information that can: be
/// retrieved about the shader, in bytes, is returned in `pInfoSize`. Otherwise,
/// `pInfoSize` must: point to a variable set by the user to the size of the buffer,
/// in bytes, pointed to by `pInfo`, and on return the variable is overwritten with
/// the amount of data actually written to `pInfo`.
///
/// If `pInfoSize` is less than the maximum size that can: be retrieved by the
/// pipeline cache, then at most `pInfoSize` bytes will be written to `pInfo`, and
/// `vkGetShaderInfoAMD` will return `VK_INCOMPLETE`.
///
/// Not all information is available for every shader and implementations may not
/// support all kinds of information for any shader. When a certain type of
/// information is unavailable, the function returns `VK_ERROR_FEATURE_NOT_PRESENT`.
///
/// If information is successfully and fully queried, the function will return
/// `VK_SUCCESS`.
///
/// For `VK_SHADER_INFO_TYPE_STATISTICS_AMD`, an instance of
/// `VkShaderStatisticsInfoAMD` will be written to the buffer pointed to by `pInfo`.
/// This structure will be populated with statistics regarding the physical device
/// resources used by that shader along with other miscellaneous information and is
/// described in further detail below.
///
/// For `VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD`, `pInfo` points to a UTF-8
/// null-terminated string containing human-readable disassembly. The exact
/// formatting and contents of the disassembly string are vendor-specific.
///
/// The formatting and contents of all other types of information, including
/// `VK_SHADER_INFO_TYPE_BINARY_AMD`, are left to the vendor and are not further
/// specified by this extension.
///
#[cfg(feature = "VK_AMD_shader_info")]
pub fn vkGetShaderInfoAMD(
  device: VkDevice,
  pipeline: VkPipeline,
  shaderStage: VkShaderStageFlagBits,
  infoType: VkShaderInfoTypeAMD,
) -> Result<Vec<u8>, VkResult> {
  unsafe {
    let mut pInfoSize: usize = 0;
    let mut pInfo: Vec<u8> = Vec::new();
    VkDeviceDispatchTable::with(device, |_t| loop {
      let _r = _t.vkGetShaderInfoAMD.unwrap()(
        device.as_raw(),
        pipeline.as_raw(),
        shaderStage,
        infoType,
        &mut pInfoSize,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pInfoSize == 0 {
        return Ok(pInfo);
      }
      pInfo = Vec::with_capacity(pInfoSize as usize);
      let _r = _t.vkGetShaderInfoAMD.unwrap()(
        device.as_raw(),
        pipeline.as_raw(),
        shaderStage,
        infoType,
        &mut pInfoSize,
        pInfo.as_mut_slice().as_raw() as *mut c_void,
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pInfo.set_len(pInfoSize as usize);
      return Ok(pInfo);
    })
  }
}

// feature: VK_NV_external_memory_capabilities

/// determine image capabilities compatible with external memory handle types
///
/// To determine the image capabilities compatible with an external memory handle
/// type, call:
///
///   - `physicalDevice` is the physical device from which to query the image
///     capabilities
///
///   - `format` is the image format, corresponding to `VkImageCreateInfo::format`.
///
///   - `type` is the image type, corresponding to `VkImageCreateInfo::imageType`.
///
///   - `tiling` is the image tiling, corresponding to `VkImageCreateInfo::tiling`.
///
///   - `usage` is the intended usage of the image, corresponding to
///     `VkImageCreateInfo::usage`.
///
///   - `flags` is a bitmask describing additional parameters of the image,
///     corresponding to `VkImageCreateInfo::flags`.
///
///   - `externalHandleType` is either one of the bits from
///     `VkExternalMemoryHandleTypeFlagBitsNV`, or 0.
///
///   - `pExternalImageFormatProperties` points to an instance of the
///     `VkExternalImageFormatPropertiesNV` structure in which capabilities are
///     returned.
///
/// If `externalHandleType` is 0,
/// `pExternalImageFormatProperties`::imageFormatProperties will return the same
/// values as a call to `vkGetPhysicalDeviceImageFormatProperties`, and the other
/// members of `pExternalImageFormatProperties` will all be 0. Otherwise, they are
/// filled in as described for `VkExternalImageFormatPropertiesNV`.
///
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
  physicalDevice: VkPhysicalDevice,
  format: VkFormat,
  eType: VkImageType,
  tiling: VkImageTiling,
  usage: VkImageUsageFlags,
  flags: VkImageCreateFlags,
  externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
) -> Result<VkExternalImageFormatPropertiesNV, VkResult> {
  unsafe {
    let mut pExternalImageFormatProperties: VkExternalImageFormatPropertiesNV = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceExternalImageFormatPropertiesNV
        .unwrap()(
        physicalDevice.as_raw(),
        format,
        eType,
        tiling,
        usage,
        flags,
        externalHandleType,
        (&mut pExternalImageFormatProperties).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pExternalImageFormatProperties);
    })
  }
}

// feature: VK_NV_external_memory_win32

/// retrieve Win32 handle to a device memory object
///
/// To retrieve the handle corresponding to a device memory object created with
/// `VkExportMemoryAllocateInfoNV::handleTypes` set to include
/// `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV` or
/// `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV`, call:
///
///   - `device` is the logical device that owns the memory.
///
///   - `memory` is the `VkDeviceMemory` object.
///
///   - `handleType` is a bitmask of `VkExternalMemoryHandleTypeFlagBitsNV`
///     containing a single bit specifying the type of handle requested.
///
///   - `handle` points to a Windows `HANDLE` in which the handle is returned.
///
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetMemoryWin32HandleNV(
  device: VkDevice,
  memory: VkDeviceMemory,
  handleType: VkExternalMemoryHandleTypeFlagsNV,
) -> Result<wsi::win32::HANDLE, VkResult> {
  unsafe {
    let mut pHandle: wsi::win32::HANDLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryWin32HandleNV.unwrap()(device.as_raw(), memory.as_raw(), handleType, &mut pHandle);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pHandle);
    })
  }
}

// feature: VK_KHX_device_group_creation

/// Enumerates groups of physical devices that can be used to create a single
/// logical device
///
///   - `instance` is a handle to a Vulkan instance previously created with
///     `vkCreateInstance`.
///
///   - `pPhysicalDeviceGroupCount` is a pointer to an integer related to the number
///     of device groups available or queried, as described below.
///
///   - `pPhysicalDeviceGroupProperties` is either `NULL` or a pointer to an array
///     of `VkPhysicalDeviceGroupPropertiesKHX` structures.
///
/// If `pPhysicalDeviceGroupProperties` is `NULL`, then the number of device groups
/// available is returned in `pPhysicalDeviceGroupCount`. Otherwise,
/// `pPhysicalDeviceGroupCount` must: point to a variable set by the user to the
/// number of elements in the `pPhysicalDeviceGroupProperties` array, and on return
/// the variable is overwritten with the number of structures actually written to
/// `pPhysicalDeviceGroupProperties`. If `pPhysicalDeviceGroupCount` is less than
/// the number of device groups available, at most `pPhysicalDeviceGroupCount`
/// structures will be written. If `pPhysicalDeviceGroupCount` is smaller than the
/// number of device groups available, `VK_INCOMPLETE` will be returned instead of
/// `VK_SUCCESS`, to indicate that not all the available device groups were
/// returned.
///
/// Every physical device must: be in exactly one device group.
///
#[cfg(feature = "VK_KHX_device_group_creation")]
pub fn vkEnumeratePhysicalDeviceGroupsKHX(
  instance: VkInstance,
) -> Result<Vec<VkPhysicalDeviceGroupPropertiesKHX>, VkResult> {
  unsafe {
    let mut pPhysicalDeviceGroupCount: u32 = 0;
    let mut pPhysicalDeviceGroupProperties: Vec<VkPhysicalDeviceGroupPropertiesKHX> = Vec::new();
    VkInstanceDispatchTable::with(instance, |_t| loop {
      let _r = _t.vkEnumeratePhysicalDeviceGroupsKHX.unwrap()(
        instance.as_raw(),
        &mut pPhysicalDeviceGroupCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPhysicalDeviceGroupCount == 0 {
        return Ok(pPhysicalDeviceGroupProperties);
      }
      pPhysicalDeviceGroupProperties = Vec::with_capacity(pPhysicalDeviceGroupCount as usize);
      let _r = _t.vkEnumeratePhysicalDeviceGroupsKHX.unwrap()(
        instance.as_raw(),
        &mut pPhysicalDeviceGroupCount,
        pPhysicalDeviceGroupProperties.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pPhysicalDeviceGroupProperties.set_len(pPhysicalDeviceGroupCount as usize);
      return Ok(pPhysicalDeviceGroupProperties);
    })
  }
}

// feature: VK_KHX_device_group
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkGetDeviceGroupPeerMemoryFeaturesKHX(
  device: VkDevice,
  heapIndex: u32,
  localDeviceIndex: u32,
  remoteDeviceIndex: u32,
) -> VkPeerMemoryFeatureFlagsKHX {
  unsafe {
    let mut pPeerMemoryFeatures: VkPeerMemoryFeatureFlagsKHX = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetDeviceGroupPeerMemoryFeaturesKHX.unwrap()(
        device.as_raw(),
        heapIndex,
        localDeviceIndex,
        remoteDeviceIndex,
        &mut pPeerMemoryFeatures,
      );
      return pPeerMemoryFeatures;
    })
  }
}

/// Modify device mask of a command buffer
///
///   - `commandBuffer` is command buffer whose current device mask is modified.
///
///   - `deviceMask` is the new value of the current device mask.
///
/// `deviceMask` is used to filter out subsequent commands from executing on all
/// physical devices whose bit indices are not set in the mask.
///
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkCmdSetDeviceMaskKHX(commandBuffer: VkCommandBuffer, deviceMask: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetDeviceMaskKHX.unwrap()(commandBuffer.as_raw(), deviceMask)
    })
  }
}

/// Dispatch compute work items
///
/// To record a dispatch using non-zero base values for the components of
/// `WorkgroupId`.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `baseGroupX` is the start value for the X component of `WorkgroupId`.
///
///   - `baseGroupY` is the start value for the Y component of `WorkgroupId`.
///
///   - `baseGroupZ` is the start value for the Z component of `WorkgroupId`.
///
///   - `groupCountX` is the number of local workgroups to dispatch in the X
///     dimension.
///
///   - `groupCountY` is the number of local workgroups to dispatch in the Y
///     dimension.
///
///   - `groupCountZ` is the number of local workgroups to dispatch in the Z
///     dimension.
///
/// When the command is executed, a global workgroup consisting of groupCountX
/// {times} groupCountY {times} groupCountZ local workgroups is assembled, with
/// `WorkgroupId` values ranging from \[baseGroup, baseGroup + groupCount) in each
/// component. `vkCmdDispatch` is equivalent to
/// vkCmdDispatchBaseKHX(0,0,0,groupCountX,groupCountY,groupCountZ).
///
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkCmdDispatchBaseKHX(
  commandBuffer: VkCommandBuffer,
  baseGroupX: u32,
  baseGroupY: u32,
  baseGroupZ: u32,
  groupCountX: u32,
  groupCountY: u32,
  groupCountZ: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDispatchBaseKHX.unwrap()(
        commandBuffer.as_raw(),
        baseGroupX,
        baseGroupY,
        baseGroupZ,
        groupCountX,
        groupCountY,
        groupCountZ,
      )
    })
  }
}

/// Query present capabilities from other physical devices
///
/// A logical device that represents multiple physical devices may: support
/// presenting from images on more than one physical device, or combining images
/// from multiple physical devices.
///
/// To query these capabilities, call:
///
///   - `device` is the logical device.
///
///   - `pDeviceGroupPresentCapabilities` is a pointer to a structure of type
///     `VkDeviceGroupPresentCapabilitiesKHX` that is filled with the logical
///     device’s capabilities.
///
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkGetDeviceGroupPresentCapabilitiesKHX(
  device: VkDevice,
) -> Result<VkDeviceGroupPresentCapabilitiesKHX, VkResult> {
  unsafe {
    let mut pDeviceGroupPresentCapabilities: VkDeviceGroupPresentCapabilitiesKHX = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetDeviceGroupPresentCapabilitiesKHX.unwrap()(
        device.as_raw(),
        (&mut pDeviceGroupPresentCapabilities).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pDeviceGroupPresentCapabilities);
    })
  }
}

/// Query present capabilities for a surface
///
/// Some surfaces may: not be capable of using all the device group present modes.
///
/// To query the supported device group present modes for a particular surface,
/// call:
///
///   - `device` is the logical device.
///
///   - `surface` is the surface.
///
///   - `pModes` is a pointer to a value of type `VkDeviceGroupPresentModeFlagsKHX`
///     that is filled with the supported device group present modes for the
///     surface.
///
/// The modes returned by this command are not invariant, and may: change in
/// response to the surface being moved, resized, or occluded. These modes must: be
/// a subset of the modes returned by `vkGetDeviceGroupPresentCapabilitiesKHX`.
///
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkGetDeviceGroupSurfacePresentModesKHX(
  device: VkDevice,
  surface: VkSurfaceKHR,
) -> Result<VkDeviceGroupPresentModeFlagsKHX, VkResult> {
  unsafe {
    let mut pModes: VkDeviceGroupPresentModeFlagsKHX = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetDeviceGroupSurfacePresentModesKHX.unwrap()(device.as_raw(), surface.as_raw(), &mut pModes);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pModes);
    })
  }
}

/// Query present rectangles for a surface on a physical device
///
/// When using `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHX`, the
/// application may: need to know which regions of the surface are used when
/// presenting locally on each physical device. Presentation of swapchain images to
/// this surface need only have valid contents in the regions returned by this
/// command.
///
/// To query a set of rectangles used in presentation on the physical device, call:
///
///   - `physicalDevice` is the physical device.
///
///   - `surface` is the surface.
///
///   - `pRectCount` is a pointer to an integer related to the number of rectangles
///     available or queried, as described below.
///
///   - `pRects` is either `NULL` or a pointer to an array of `VkRect2D` structures.
///
/// If `pRects` is `NULL`, then the number of rectangles used when presenting the
/// given `surface` is returned in `pRectCount`. Otherwise, `pRectCount` must: point
/// to a variable set by the user to the number of elements in the `pRects` array,
/// and on return the variable is overwritten with the number of structures actually
/// written to `pRects`. If the value of `pRectCount` is less than the number of
/// rectangles, at most `pRectCount` structures will be written. If `pRectCount` is
/// smaller than the number of rectangles used for the given `surface`,
/// `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS` to indicate that not
/// all the available values were returned.
///
/// The values returned by this command are not invariant, and may: change in
/// response to the surface being moved, resized, or occluded.
///
/// The rectangles returned by this command must: not overlap.
///
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkGetPhysicalDevicePresentRectanglesKHX(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
) -> Result<Vec<VkRect2D>, VkResult> {
  unsafe {
    let mut pRectCount: u32 = 0;
    let mut pRects: Vec<VkRect2D> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetPhysicalDevicePresentRectanglesKHX.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pRectCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pRectCount == 0 {
        return Ok(pRects);
      }
      pRects = Vec::with_capacity(pRectCount as usize);
      let _r = _t.vkGetPhysicalDevicePresentRectanglesKHX.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pRectCount,
        pRects.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pRects.set_len(pRectCount as usize);
      return Ok(pRects);
    })
  }
}

/// Retrieve the index of the next available presentable image
///
/// To acquire an available presentable image to use, and retrieve the index of that
/// image, call:
///
///   - `device` is the device associated with `swapchain`.
///
///   - `pAcquireInfo` is a pointer to a structure of type
///     `VkAcquireNextImageInfoKHX` containing parameters of the acquire.
///
///   - `pImageIndex` is a pointer to a `uint32_t` that is set to the index of the
///     next image to use.
///
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkAcquireNextImage2KHX(device: VkDevice, pAcquireInfo: &VkAcquireNextImageInfoKHX) -> Result<u32, VkResult> {
  unsafe {
    let mut pImageIndex: u32 = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAcquireNextImage2KHX.unwrap()(device.as_raw(), pAcquireInfo.as_raw(), &mut pImageIndex);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pImageIndex);
    })
  }
}

// feature: VK_NN_vi_surface

/// Create a slink:VkSurfaceKHR object for a VI layer
///
/// To create a `VkSurfaceKHR` object for an `nn::vi`::\`Layer\`, query the layer’s
/// native handle using `nn::vi`::\`GetNativeWindow\`, and then call:
///
///   - `instance` is the instance with which to associate the surface.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkViSurfaceCreateInfoNN`
///     structure containing parameters affecting the creation of the surface
///     object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
/// During the lifetime of a surface created using a particular
/// `nn::vi`::\`NativeWindowHandle\` any attempts to create another surface for the
/// same `nn::vi`::\`Layer\` and any attempts to connect to the same
/// `nn::vi`::\`Layer\` through other platform mechanisms will have undefined
/// results.
///
/// The `currentExtent` of a VI surface is always undefined. Applications are
/// expected to choose an appropriate size for the swapchain’s `imageExtent` (e.g.,
/// by matching the the result of a call to `nn::vi`::\`GetDisplayResolution\`).
///
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub fn vkCreateViSurfaceNN(
  instance: VkInstance,
  pCreateInfo: &VkViSurfaceCreateInfoNN,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateViSurfaceNN.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_KHR_maintenance1

/// Trim a command pool
///
///   - `device` is the logical device that owns the command pool.
///
///   - `commandPool` is the command pool to trim.
///
///   - `flags` is reserved for future use.
///
/// Trimming a command pool recycles unused memory from the command pool back to the
/// system. Command buffers allocated from the pool are not affected by the command.
///
/// > **Note**
/// >
/// > This command provides applications with some control over the internal memory
/// > allocations used by command pools.
/// >
/// > Unused memory normally arises from command buffers that have been recorded and
/// > later reset, such that they are no longer using the memory. On reset, a
/// > command buffer can return memory to its command pool, but the only way to
/// > release memory from a command pool to the system requires calling
/// > `vkResetCommandPool`, which cannot be executed while any command buffers from
/// > that pool are still in use. Subsequent recording operations into command
/// > buffers will re-use this memory but since total memory requirements fluctuate
/// > over time, unused memory can accumulate.
/// >
/// > In this situation, trimming a command pool may: be useful to return unused
/// > memory back to the system, returning the total outstanding memory allocated by
/// > the pool back to a more “average” value.
/// >
/// > Implementations utilize many internal allocation strategies that make it
/// > impossible to guarantee that all unused memory is released back to the system.
/// > For instance, an implementation of a command pool may: involve allocating
/// > memory in bulk from the system and sub-allocating from that memory. In such an
/// > implementation any live command buffer that holds a reference to a bulk
/// > allocation would prevent that allocation from being freed, even if only a
/// > small proportion of the bulk allocation is in use.
/// >
/// > In most cases trimming will result in a reduction in allocated but unused
/// > memory, but it does not guarantee the “ideal” behaviour.
/// >
/// > Trimming may: be an expensive operation, and should: not be called frequently.
/// > Trimming should: be treated as a way to relieve memory pressure after
/// > application-known points when there exists enough unused memory that the cost
/// > of trimming is “worth” it.
///
#[cfg(feature = "VK_KHR_maintenance1")]
pub fn vkTrimCommandPoolKHR(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkTrimCommandPoolKHR.unwrap()(device.as_raw(), commandPool.as_raw(), flags)
    })
  }
}

// feature: VK_KHR_external_memory_capabilities

/// Query external handle types supported by buffers
///
///   - `physicalDevice` is the physical device from which to query the buffer
///     capabilities.
///
///   - `pExternalBufferInfo` points to an instance of the
///     `VkPhysicalDeviceExternalBufferInfoKHR` structure, describing the parameters
///     that would be consumed by `vkCreateBuffer`.
///
///   - `pExternalBufferProperties` points to an instance of the
///     `VkExternalBufferPropertiesKHR` structure in which capabilities are
///     returned.
///
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(
  physicalDevice: VkPhysicalDevice,
  pExternalBufferInfo: &VkPhysicalDeviceExternalBufferInfoKHR,
) -> VkExternalBufferPropertiesKHR {
  unsafe {
    let mut pExternalBufferProperties: VkExternalBufferPropertiesKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceExternalBufferPropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        pExternalBufferInfo.as_raw(),
        (&mut pExternalBufferProperties).as_raw(),
      );
      return pExternalBufferProperties;
    })
  }
}

// feature: VK_KHR_external_memory_win32

/// Get a Windows HANDLE for a memory object
///
/// To export a Windows handle representing the underlying resources of a Vulkan
/// device memory object.
///
///   - `device` is the logical device that created the device memory being
///     exported.
///
///   - `pGetWin32HandleInfo` is a pointer to an instance of the
///     `VkMemoryGetWin32HandleInfoKHR` structure containing parameters of the
///     export operation.
///
///   - `pHandle` will return the Windows handle representing the underlying
///     resources of the device memory object.
///
/// For handle types defined as NT handles, the handles returned by
/// `vkGetMemoryWin32HandleKHR` are owned by the application. To avoid leaking
/// resources, the application must: release ownership of them using the
/// `CloseHandle` system call when they are no longer needed.
///
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetMemoryWin32HandleKHR(
  device: VkDevice,
  pGetWin32HandleInfo: &VkMemoryGetWin32HandleInfoKHR,
) -> Result<wsi::win32::HANDLE, VkResult> {
  unsafe {
    let mut pHandle: wsi::win32::HANDLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryWin32HandleKHR.unwrap()(device.as_raw(), pGetWin32HandleInfo.as_raw(), &mut pHandle);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pHandle);
    })
  }
}

/// Get Properties of External Memory Win32 Handles
///
/// Windows memory handles compatible with Vulkan may: also be created by non-Vulkan
/// APIs using methods beyond the scope of this specification. To determine the
/// correct parameters to use when importing such handles, call.
///
///   - `device` is the logical device that will be importing `handle`.
///
///   - `handleType` is the type of the handle `handle`.
///
///   - `handle` is the handle which will be imported.
///
///   - `pMemoryWin32HandleProperties` will return properties of `handle`.
///
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetMemoryWin32HandlePropertiesKHR(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  handle: wsi::win32::HANDLE,
) -> Result<VkMemoryWin32HandlePropertiesKHR, VkResult> {
  unsafe {
    let mut pMemoryWin32HandleProperties: VkMemoryWin32HandlePropertiesKHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryWin32HandlePropertiesKHR.unwrap()(
        device.as_raw(),
        handleType,
        handle,
        (&mut pMemoryWin32HandleProperties).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pMemoryWin32HandleProperties);
    })
  }
}

// feature: VK_KHR_external_memory_fd

/// Get a POSIX file descriptor for a memory object
///
/// To export a POSIX file descriptor representing the underlying resources of a
/// Vulkan device memory object.
///
///   - `device` is the logical device that created the device memory being
///     exported.
///
///   - `pGetFdInfo` is a pointer to an instance of the `VkMemoryGetFdInfoKHR`
///     structure containing parameters of the export operation.
///
///   - `pFd` will return a file descriptor representing the underlying resources of
///     the device memory object.
///
/// Each call to `vkGetMemoryFdKHR` must: create a new file descriptor and transfer
/// ownership of it to the application. To avoid leaking resources, the application
/// must: release ownership of the file descriptor using the `close` system call
/// when it is no longer needed, or by importing a Vulkan memory object from it.
/// Where supported by the operating system, the implementation must: set the file
/// descriptor to be closed automatically when an `execve` system call is made.
///
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub fn vkGetMemoryFdKHR(device: VkDevice, pGetFdInfo: &VkMemoryGetFdInfoKHR) -> Result<c_int, VkResult> {
  unsafe {
    let mut pFd: c_int = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryFdKHR.unwrap()(device.as_raw(), pGetFdInfo.as_raw(), &mut pFd);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pFd);
    })
  }
}

/// Get Properties of External Memory File Descriptors
///
/// POSIX file descriptor memory handles compatible with Vulkan may: also be created
/// by non-Vulkan APIs using methods beyond the scope of this specification. To
/// determine the correct parameters to use when importing such handles, call.
///
///   - `device` is the logical device that will be importing `fd`.
///
///   - `handleType` is the type of the handle `fd`.
///
///   - `fd` is the handle which will be imported.
///
///   - `pMemoryFdProperties` will return properties of the handle `fd`.
///
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub fn vkGetMemoryFdPropertiesKHR(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  fd: c_int,
) -> Result<VkMemoryFdPropertiesKHR, VkResult> {
  unsafe {
    let mut pMemoryFdProperties: VkMemoryFdPropertiesKHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryFdPropertiesKHR.unwrap()(
        device.as_raw(),
        handleType,
        fd,
        (&mut pMemoryFdProperties).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pMemoryFdProperties);
    })
  }
}

// feature: VK_KHR_external_semaphore_capabilities

/// Function for querying external semaphore handle capabilities.
///
/// Semaphores may: support import and export of their
/// [payload](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-payloads) to external handles.
///
///   - `physicalDevice` is the physical device from which to query the semaphore
///     capabilities.
///
///   - `pExternalSemaphoreInfo` points to an instance of the
///     `VkPhysicalDeviceExternalSemaphoreInfoKHR` structure, describing the
///     parameters that would be consumed by `vkCreateSemaphore`.
///
///   - `pExternalSemaphoreProperties` points to an instance of the
///     `VkExternalSemaphorePropertiesKHR` structure in which capabilities are
///     returned.
///
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
  physicalDevice: VkPhysicalDevice,
  pExternalSemaphoreInfo: &VkPhysicalDeviceExternalSemaphoreInfoKHR,
) -> VkExternalSemaphorePropertiesKHR {
  unsafe {
    let mut pExternalSemaphoreProperties: VkExternalSemaphorePropertiesKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceExternalSemaphorePropertiesKHR
        .unwrap()(
        physicalDevice.as_raw(),
        pExternalSemaphoreInfo.as_raw(),
        (&mut pExternalSemaphoreProperties).as_raw(),
      );
      return pExternalSemaphoreProperties;
    })
  }
}

// feature: VK_KHR_external_semaphore_win32

/// Import a semaphore from a Windows HANDLE
///
///   - `device` is the logical device that created the semaphore.
///
///   - `pImportSemaphoreWin32HandleInfo` points to a
///     `VkImportSemaphoreWin32HandleInfoKHR` structure specifying the semaphore and
///     import parameters.
///
/// Importing a semaphore payload from Windows handles does not transfer ownership
/// of the handle to the Vulkan implementation. For handle types defined as NT
/// handles, the application must: release ownership using the `CloseHandle` system
/// call when the handle is no longer needed.
///
/// Applications can: import the same semaphore payload into multiple instances of
/// Vulkan, into the same instance from which it was exported, and multiple times
/// into a given Vulkan instance.
///
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkImportSemaphoreWin32HandleKHR(
  device: VkDevice,
  pImportSemaphoreWin32HandleInfo: &VkImportSemaphoreWin32HandleInfoKHR,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportSemaphoreWin32HandleKHR.unwrap()(device.as_raw(), pImportSemaphoreWin32HandleInfo.as_raw())
    })
  }
}

/// Get a Windows HANDLE for a semaphore
///
///   - `device` is the logical device that created the semaphore being exported.
///
///   - `pGetWin32HandleInfo` is a pointer to an instance of the
///     `VkSemaphoreGetWin32HandleInfoKHR` structure containing parameters of the
///     export operation.
///
///   - `pHandle` will return the Windows handle representing the semaphore state.
///
/// For handle types defined as NT handles, the handles returned by
/// `vkGetSemaphoreWin32HandleKHR` are owned by the application. To avoid leaking
/// resources, the application must: release ownership of them using the
/// `CloseHandle` system call when they are no longer needed.
///
/// Exporting a Windows handle from a semaphore may: have side effects depending on
/// the transference of the specified handle type, as described in [Importing
/// Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-importing).
///
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetSemaphoreWin32HandleKHR(
  device: VkDevice,
  pGetWin32HandleInfo: &VkSemaphoreGetWin32HandleInfoKHR,
) -> Result<wsi::win32::HANDLE, VkResult> {
  unsafe {
    let mut pHandle: wsi::win32::HANDLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetSemaphoreWin32HandleKHR.unwrap()(device.as_raw(), pGetWin32HandleInfo.as_raw(), &mut pHandle);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pHandle);
    })
  }
}

// feature: VK_KHR_external_semaphore_fd

/// Import a semaphore from a POSIX file descriptor
///
///   - `device` is the logical device that created the semaphore.
///
///   - `pImportSemaphoreFdInfo` points to a `VkImportSemaphoreFdInfoKHR` structure
///     specifying the semaphore and import parameters.
///
/// Importing a semaphore payload from a file descriptor transfers ownership of the
/// file descriptor from the application to the Vulkan implementation. The
/// application must: not perform any operations on the file descriptor after a
/// successful import.
///
/// Applications can: import the same semaphore payload into multiple instances of
/// Vulkan, into the same instance from which it was exported, and multiple times
/// into a given Vulkan instance.
///
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub fn vkImportSemaphoreFdKHR(device: VkDevice, pImportSemaphoreFdInfo: &VkImportSemaphoreFdInfoKHR) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportSemaphoreFdKHR.unwrap()(device.as_raw(), pImportSemaphoreFdInfo.as_raw())
    })
  }
}

/// Get a POSIX file descriptor handle for a semaphore
///
/// To export a POSIX file descriptor representing the payload of a semaphore, call.
///
///   - `device` is the logical device that created the semaphore being exported.
///
///   - `pGetFdInfo` is a pointer to an instance of the `VkSemaphoreGetFdInfoKHR`
///     structure containing parameters of the export operation.
///
///   - `pFd` will return the file descriptor representing the semaphore payload.
///
/// Each call to `vkGetSemaphoreFdKHR` must: create a new file descriptor and
/// transfer ownership of it to the application. To avoid leaking resources, the
/// application must: release ownership of the file descriptor when it is no longer
/// needed.
///
/// > **Note**
/// >
/// > Ownership can be released in many ways. For example, the application can call
/// > `close`() on the file descriptor, or transfer ownership back to Vulkan by
/// > using the file descriptor to import a semaphore payload.
///
/// Where supported by the operating system, the implementation must: set the file
/// descriptor to be closed automatically when an `execve` system call is made.
///
/// Exporting a file descriptor from a semaphore may: have side effects depending on
/// the transference of the specified handle type, as described in [Importing
/// Semaphore State](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-importing).
///
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub fn vkGetSemaphoreFdKHR(device: VkDevice, pGetFdInfo: &VkSemaphoreGetFdInfoKHR) -> Result<c_int, VkResult> {
  unsafe {
    let mut pFd: c_int = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetSemaphoreFdKHR.unwrap()(device.as_raw(), pGetFdInfo.as_raw(), &mut pFd);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pFd);
    })
  }
}

// feature: VK_KHR_push_descriptor

/// Pushes descriptor updates into a command buffer
///
/// In addition to allocating descriptor sets and binding them to a command buffer,
/// an application can: record descriptor updates into the command buffer.
///
///   - `commandBuffer` is the command buffer that the descriptors will be recorded
///     in.
///
///   - `pipelineBindPoint` is a `VkPipelineBindPoint` indicating whether the
///     descriptors will be used by graphics pipelines or compute pipelines. There
///     is a separate set of push descriptor bindings for each of graphics and
///     compute, so binding one does not disturb the other.
///
///   - `layout` is a `VkPipelineLayout` object used to program the bindings.
///
///   - `set` is the set number of the descriptor set in the pipeline layout that
///     will be updated.
///
///   - `descriptorWriteCount` is the number of elements in the `pDescriptorWrites`
///     array.
///
///   - `pDescriptorWrites` is a pointer to an array of `VkWriteDescriptorSet`
///     structures describing the descriptors to be updated.
///
/// *Push descriptors* are a small bank of descriptors whose storage is internally
/// managed by the command buffer rather than being written into a descriptor set
/// and later bound to a command buffer. Push descriptors allow for incremental
/// updates of descriptors without managing the lifetime of descriptor sets.
///
/// When a command buffer begins recording, all push descriptors have undefined
/// contents. Push descriptors can: be updated incrementally and cause shaders to
/// use the updated descriptors for subsequent rendering commands (either compute or
/// graphics, according to the `pipelineBindPoint`) until the descriptor is
/// overwritten, or else until the set is disturbed as described in [Pipeline Layout
/// Compatibility](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-compatibility). When the set is disturbed or push
/// descriptors with a different descriptor set layout are set, all push descriptors
/// become invalid.
///
/// Valid descriptors must: be pushed for all bindings that any shaders in a
/// pipeline access, at the time that a draw or dispatch command is recorded to
/// execute using that pipeline. This includes immutable sampler descriptors, which
/// must: be pushed before they are accessed by a pipeline. However, if none of the
/// shaders in a pipeline statically use certain bindings in the push descriptor
/// set, then those descriptors need not be valid.
///
/// Push descriptors do not use dynamic offsets. Instead, the corresponding
/// non-dynamic descriptor types can: be used and the `offset` member of
/// `VkDescriptorBufferInfo` can: be changed each time the descriptor is written.
///
/// Each element of `pDescriptorWrites` is interpreted as in `VkWriteDescriptorSet`,
/// except the `dstSet` member is ignored.
///
/// To push an immutable sampler, use a `VkWriteDescriptorSet` with `dstBinding` and
/// `dstArrayElement` selecting the immutable sampler’s binding. If the descriptor
/// type is `VK_DESCRIPTOR_TYPE_SAMPLER`, the `pImageInfo` parameter is ignored and
/// the immutable sampler is taken from the push descriptor set layout in the
/// pipeline layout. If the descriptor type is
/// `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, the `sampler` member of the
/// `pImageInfo` parameter is ignored and the immutable sampler is taken from the
/// push descriptor set layout in the pipeline layout.
///
#[cfg(feature = "VK_KHR_push_descriptor")]
pub fn vkCmdPushDescriptorSetKHR(
  commandBuffer: VkCommandBuffer,
  pipelineBindPoint: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  set: u32,
  pDescriptorWrites: &[VkWriteDescriptorSet],
) {
  unsafe {
    let descriptorWriteCount = pDescriptorWrites.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdPushDescriptorSetKHR.unwrap()(
        commandBuffer.as_raw(),
        pipelineBindPoint,
        layout.as_raw(),
        set,
        descriptorWriteCount,
        pDescriptorWrites.as_raw(),
      )
    })
  }
}

// feature: VK_KHR_descriptor_update_template

/// Create a new descriptor update template
///
/// Updating a large `VkDescriptorSet` array can: be an expensive operation since an
/// application must: specify one `VkWriteDescriptorSet` structure for each
/// descriptor or descriptor array to update, each of which re-specifies the same
/// state when updating the same descriptor in multiple descriptor sets. For cases
/// when an application wishes to update the same set of descriptors in multiple
/// descriptor sets allocated using the same `VkDescriptorSetLayout`,
/// `vkUpdateDescriptorSetWithTemplateKHR` can: be used as a replacement for
/// `vkUpdateDescriptorSets`.
///
/// `VkDescriptorUpdateTemplateKHR` allows implementations to convert a set of
/// descriptor update operations on a single descriptor set to an internal format
/// that, in conjunction with `vkUpdateDescriptorSetWithTemplateKHR` or
/// `vkCmdPushDescriptorSetWithTemplateKHR` , can: be more efficient compared to
/// calling `vkUpdateDescriptorSets` or `vkCmdPushDescriptorSetKHR` . The
/// descriptors themselves are not specified in the `VkDescriptorUpdateTemplateKHR`,
/// rather, offsets into an application provided pointer to host memory are
/// specified, which are combined with a pointer passed to
/// `vkUpdateDescriptorSetWithTemplateKHR` or
/// `vkCmdPushDescriptorSetWithTemplateKHR` . This allows large batches of updates
/// to be executed without having to convert application data structures into a
/// strictly-defined Vulkan data structure.
///
///   - `device` is the logical device that creates the descriptor update template.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkDescriptorUpdateTemplateCreateInfoKHR` structure specifying the set of
///     descriptors to update with a single call to
///     `vkCmdPushDescriptorSetWithTemplateKHR` or
///     `vkUpdateDescriptorSetWithTemplateKHR`.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pDescriptorUpdateTemplate` points to a `VkDescriptorUpdateTemplateKHR`
///     handle in which the resulting descriptor update template object is returned.
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub fn vkCreateDescriptorUpdateTemplateKHR(
  device: VkDevice,
  pCreateInfo: &VkDescriptorUpdateTemplateCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkDescriptorUpdateTemplateKHR, VkResult> {
  unsafe {
    let mut pDescriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateDescriptorUpdateTemplateKHR.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pDescriptorUpdateTemplate).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pDescriptorUpdateTemplate);
    })
  }
}

/// Destroy a descriptor update template object
///
///   - `device` is the logical device that has been used to create the descriptor
///     update template
///
///   - `descriptorUpdateTemplate` is the descriptor update template to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub fn vkDestroyDescriptorUpdateTemplateKHR(
  device: VkDevice,
  descriptorUpdateTemplate: Option<VkDescriptorUpdateTemplateKHR>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyDescriptorUpdateTemplateKHR.unwrap()(
        device.as_raw(),
        descriptorUpdateTemplate.as_raw(),
        pAllocator.as_raw(),
      )
    })
  }
}

/// Update the contents of a descriptor set object using an update template
///
/// Once a `VkDescriptorUpdateTemplateKHR` has been created, descriptor sets can: be
/// updated by calling.
///
///   - `device` is the logical device that updates the descriptor sets.
///
///   - `descriptorSet` is the descriptor set to update
///
///   - `descriptorUpdateTemplate` is the `VkDescriptorUpdateTemplateKHR` which
///     specifies the update mapping between `pData` and the descriptor set to
///     update.
///
///   - `pData` is a pointer to memory which contains one or more structures of
///     `VkDescriptorImageInfo`, `VkDescriptorBufferInfo`, or `VkBufferView` used to
///     write the descriptors.
///
/// **API example.**
///
/// ``` c++
/// struct AppBufferView {
///     VkBufferView bufferView;
///     uint32_t     applicationRelatedInformation;
/// };
///
/// struct AppDataStructure
/// {
///     VkDescriptorImageInfo  imageInfo;          // a single image info
///     VkDescriptorBufferInfo bufferInfoArray[3]; // 3 buffer infos in an array
///     AppBufferView          bufferView[2];      // An application defined structure containing a bufferView
///     // ... some more application related data
/// };
///
/// const VkDescriptorUpdateTemplateEntryKHR descriptorUpdateTemplateEntries[] =
/// {
///     // binding to a single image descriptor
///     {
///         0,                                           // binding
///         0,                                           // dstArrayElement
///         1,                                           // descriptorCount
///         VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,   // descriptorType
///         offsetof(AppDataStructure, imageInfo),       // offset
///         0                                            // stride is not required if descriptorCount is 1.
///     },
///
///     // binding to an array of buffer descriptors
///     {
///         0,                                           // binding
///         0,                                           // dstArrayElement
///         3,                                           // descriptorCount
///         VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,           // descriptorType
///         offsetof(AppDataStructure, bufferInfoArray), // offset
///         sizeof(VkDescriptorBufferInfo)               // stride, descriptor buffer infos are compact
///     },
///
///     // binding to an array of buffer views
///     {
///         0,                                           // binding
///         3,                                           // dstArrayElement
///         1,                                           // descriptorCount
///         VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER,     // descriptorType
///         offsetof(AppDataStructure, bufferView),      // offset
///         sizeof(AppBufferView)                        // stride, bufferViews do not have to be compact
///     },
/// };
///
/// // create an descriptor update template for descriptor set updates
/// const VkDescriptorUpdateTemplateCreateInfoKHR createInfo =
/// {
///     VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR,  // sType
///     NULL,                                                          // pNext
///     0,                                                             // flags
///     3,                                                             // descriptorUpdateEntryCount
///     descriptorUpdateTemplateEntries,                               // pDescriptorUpdateEntries
///     VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR,         // templateType
///     myLayout,                                                      // descriptorSetLayout
///     0,                                                             // pipelineBindPoint, ignored by given templateType
///     0,                                                             // pipelineLayout, ignored by given templateType
///     0,                                                             // set, ignored by given templateType
/// };
///
/// VkDescriptorUpdateTemplateKHR myDescriptorUpdateTemplate;
/// myResult = vkCreateDescriptorUpdateTemplateKHR(
///     myDevice,
///     &createInfo,
///     NULL,
///     &myDescriptorUpdateTemplate);
/// }
///
/// AppDataStructure appData;
///
/// // fill appData here or cache it in your engine
/// vkUpdateDescriptorSetWithTemplateKHR(myDevice, myDescriptorSet, myDescriptorUpdateTemplate, &appData);
/// ```
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub fn vkUpdateDescriptorSetWithTemplateKHR(
  device: VkDevice,
  descriptorSet: VkDescriptorSet,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
  pData: *const c_void,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkUpdateDescriptorSetWithTemplateKHR.unwrap()(
        device.as_raw(),
        descriptorSet.as_raw(),
        descriptorUpdateTemplate.as_raw(),
        pData,
      )
    })
  }
}

/// Pushes descriptor updates into a command buffer using a descriptor update
/// template
///
/// It is also possible to use a descriptor update template to specify the push
/// descriptors to update.
///
///   - `commandBuffer` is the command buffer that the descriptors will be recorded
///     in.
///
///   - `descriptorUpdateTemplate` A descriptor update template which defines how to
///     interpret the descriptor information in pData.
///
///   - `layout` is a `VkPipelineLayout` object used to program the bindings. It
///     must: be compatible with the layout used to create the
///     `descriptorUpdateTemplate` handle.
///
///   - `set` is the set number of the descriptor set in the pipeline layout that
///     will be updated. This must: be the same number used to create the
///     `descriptorUpdateTemplate` handle.
///
///   - `pData` Points to memory which contains the descriptors for the templated
///     update.
///
/// **API example.**
///
/// ``` c++
/// struct AppBufferView {
///     VkBufferView bufferView;
///     uint32_t     applicationRelatedInformation;
/// };
///
/// struct AppDataStructure
/// {
///     VkDescriptorImageInfo  imageInfo;          // a single image info
///     // ... some more application related data
/// };
///
/// const VkDescriptorUpdateTemplateEntryKHR descriptorUpdateTemplateEntries[] =
/// {
///     // binding to a single image descriptor
///     {
///         0,                                           // binding
///         0,                                           // dstArrayElement
///         1,                                           // descriptorCount
///         VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,   // descriptorType
///         offsetof(AppDataStructure, imageInfo),       // offset
///         0                                            // stride is not required if descriptorCount is 1.
///     }
///
/// };
///
/// // create an descriptor update template for descriptor set updates
/// const VkDescriptorUpdateTemplateCreateInfoKHR createInfo =
/// {
///     VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR,  // sType
///     NULL,                                                          // pNext
///     0,                                                             // flags
///     1,                                                             // descriptorUpdateEntryCount
///     descriptorUpdateTemplateEntries,                               // pDescriptorUpdateEntries
///     VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR,       // templateType
///     0,                                                             // descriptorSetLayout, ignored by given templateType
///     VK_PIPELINE_BIND_POINT_GRAPHICS,                               // pipelineBindPoint
///     myPipelineLayout,                                              // pipelineLayout
///     0,                                                             // set
/// };
///
/// VkDescriptorUpdateTemplateKHR myDescriptorUpdateTemplate;
/// myResult = vkCreateDescriptorUpdateTemplateKHR(
///     myDevice,
///     &createInfo,
///     NULL,
///     &myDescriptorUpdateTemplate);
/// }
///
/// AppDataStructure appData;
/// // fill appData here or cache it in your engine
/// vkCmdPushDescriptorSetWithTemplateKHR(myCmdBuffer, myDescriptorUpdateTemplate, myPipelineLayout, 0,&appData);
/// ```
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub fn vkCmdPushDescriptorSetWithTemplateKHR(
  commandBuffer: VkCommandBuffer,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
  layout: VkPipelineLayout,
  set: u32,
  pData: *const c_void,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdPushDescriptorSetWithTemplateKHR.unwrap()(
        commandBuffer.as_raw(),
        descriptorUpdateTemplate.as_raw(),
        layout.as_raw(),
        set,
        pData,
      )
    })
  }
}

// feature: VK_NVX_device_generated_commands

/// Performs the generation of commands on the device
///
/// The actual generation on the device is handled with:
///
///   - `commandBuffer` is the primary command buffer in which the generation
///     process takes space.
///
///   - `pProcessCommandsInfo` is a pointer to an instance of the
///     `VkCmdProcessCommandsInfoNVX` structure containing parameters affecting the
///     processing of commands.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCmdProcessCommandsNVX(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: &VkCmdProcessCommandsInfoNVX) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdProcessCommandsNVX.unwrap()(commandBuffer.as_raw(), pProcessCommandsInfo.as_raw())
    })
  }
}

/// Perform a reservation of command buffer space
///
/// Command space for generated commands recorded into a secondary command buffer
/// must: be reserved by calling:
///
///   - `commandBuffer` is the secondary command buffer in which the space for
///     device-generated commands is reserved.
///
///   - `pProcessCommandsInfo` is a pointer to an instance of the
///     `vkCmdReserveSpaceForCommandsNVX` structure containing parameters affecting
///     the reservation of command buffer space.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCmdReserveSpaceForCommandsNVX(
  commandBuffer: VkCommandBuffer,
  pReserveSpaceInfo: &VkCmdReserveSpaceForCommandsInfoNVX,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdReserveSpaceForCommandsNVX.unwrap()(commandBuffer.as_raw(), pReserveSpaceInfo.as_raw())
    })
  }
}

/// Create an indirect command layout object
///
/// Indirect command layouts are created by:
///
///   - `device` is the logical device that creates the object table.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkObjectTableCreateInfoNVX` structure containing parameters affecting
///     creation of the table.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pIndirectCommandsLayout` points to a `VkObjectTableNVX` handle in which the
///     resulting object table is returned.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCreateIndirectCommandsLayoutNVX(
  device: VkDevice,
  pCreateInfo: &VkIndirectCommandsLayoutCreateInfoNVX,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkIndirectCommandsLayoutNVX, VkResult> {
  unsafe {
    let mut pIndirectCommandsLayout: VkIndirectCommandsLayoutNVX = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateIndirectCommandsLayoutNVX.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pIndirectCommandsLayout).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pIndirectCommandsLayout);
    })
  }
}

/// Destroy a object table
///
/// Indirect command layouts are destroyed by:
///
///   - `device` is the logical device that destroys the layout.
///
///   - `indirectCommandsLayout` is the table to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkDestroyIndirectCommandsLayoutNVX(
  device: VkDevice,
  indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyIndirectCommandsLayoutNVX.unwrap()(
        device.as_raw(),
        indirectCommandsLayout.as_raw(),
        pAllocator.as_raw(),
      )
    })
  }
}

/// Create an object table
///
/// To create object tables, call:
///
///   - `device` is the logical device that creates the object table.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkObjectTableCreateInfoNVX` structure containing parameters affecting
///     creation of the table.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pObjectTable` points to a `VkObjectTableNVX` handle in which the resulting
///     object table is returned.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCreateObjectTableNVX(
  device: VkDevice,
  pCreateInfo: &VkObjectTableCreateInfoNVX,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkObjectTableNVX, VkResult> {
  unsafe {
    let mut pObjectTable: VkObjectTableNVX = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateObjectTableNVX.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pObjectTable).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pObjectTable);
    })
  }
}

/// Destroy a object table
///
/// To destroy an object table, call:
///
///   - `device` is the logical device that destroys the table.
///
///   - `objectTable` is the table to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkDestroyObjectTableNVX(
  device: VkDevice,
  objectTable: VkObjectTableNVX,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyObjectTableNVX.unwrap()(device.as_raw(), objectTable.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Register resource bindings in an object table
///
/// Resource bindings of Vulkan objects are registered at an arbitrary
/// ftext:uint32\_t index within an object table. As long as the object table
/// references such objects, they must: not be deleted.
///
///   - `device` is the logical device that creates the object table.
///
///   - `objectTable` is the table for which the resources are registered.
///
///   - `objectCount` is the number of resources to register.
///
///   - `ppObjectTableEntries` provides an array for detailed binding informations,
///     each array element is a pointer to a struct of type
///     `VkObjectTablePipelineEntryNVX`, `VkObjectTableDescriptorSetEntryNVX`,
///     `VkObjectTableVertexBufferEntryNVX`, `VkObjectTableIndexBufferEntryNVX` or
///     `VkObjectTablePushConstantEntryNVX` (see below for details).
///
///   - `pObjectIndices` are the indices at which each resource is registered.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkRegisterObjectsNVX(
  device: VkDevice,
  objectTable: VkObjectTableNVX,
  ppObjectTableEntries: &[&VkObjectTableEntryNVX],
  pObjectIndices: &[u32],
) -> VkResult {
  unsafe {
    let objectCount = ppObjectTableEntries.len() as u32;
    assert!(objectCount as usize == pObjectIndices.len());
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkRegisterObjectsNVX.unwrap()(
        device.as_raw(),
        objectTable.as_raw(),
        objectCount,
        ppObjectTableEntries.as_raw(),
        pObjectIndices.as_raw(),
      )
    })
  }
}

/// Unregister resource bindings in an object table
///
/// Use the following command to unregister resources from an object table:
///
///   - `device` is the logical device that creates the object table.
///
///   - `objectTable` is the table from which the resources are unregistered.
///
///   - `objectCount` is the number of resources being removed from the object
///     table.
///
///   - `pObjectEntryType` provides an array of `VkObjectEntryTypeNVX` for the
///     resources being removed.
///
///   - `pObjectIndices` provides the array of object indices to be removed.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkUnregisterObjectsNVX(
  device: VkDevice,
  objectTable: VkObjectTableNVX,
  pObjectEntryTypes: &[VkObjectEntryTypeNVX],
  pObjectIndices: &[u32],
) -> VkResult {
  unsafe {
    let objectCount = pObjectEntryTypes.len() as u32;
    assert!(objectCount as usize == pObjectIndices.len());
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkUnregisterObjectsNVX.unwrap()(
        device.as_raw(),
        objectTable.as_raw(),
        objectCount,
        pObjectEntryTypes.as_raw(),
        pObjectIndices.as_raw(),
      )
    })
  }
}

/// Returns device-generated commands related properties of a physical device
///
/// To query the support of related features and limitations, call:
///
///   - `physicalDevice` is the handle to the physical device whose properties will
///     be queried.
///
///   - `pFeatures` points to an instance of the
///     `VkDeviceGeneratedCommandsFeaturesNVX` structure, that will be filled with
///     returned information.
///
///   - `pLimits` points to an instance of the `VkDeviceGeneratedCommandsLimitsNVX`
///     structure, that will be filled with returned information.
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX(
  physicalDevice: VkPhysicalDevice,
  pFeatures: &mut VkDeviceGeneratedCommandsFeaturesNVX,
) -> VkDeviceGeneratedCommandsLimitsNVX {
  unsafe {
    let mut pLimits: VkDeviceGeneratedCommandsLimitsNVX = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX
        .unwrap()(
        physicalDevice.as_raw(),
        pFeatures.as_raw(),
        (&mut pLimits).as_raw(),
      );
      return pLimits;
    })
  }
}

// feature: VK_NV_clip_space_w_scaling

/// Set the viewport W scaling on a command buffer
///
/// If the bound pipeline state object was not created with the
/// `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, viewport **W**
/// scaling parameters are specified using the `pViewportWScalings` member of
/// `VkPipelineViewportWScalingStateCreateInfoNV` in the pipeline state object. If
/// the pipeline state object was created with the
/// `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, the viewport
/// transformation parameters are dynamically set and changed with the command:
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `firstViewport` is the index of the first viewport whose parameters are
///     updated by the command.
///
///   - `viewportCount` is the number of viewports whose parameters are updated by
///     the command.
///
///   - `pViewportWScalings` is a pointer to an array of `VkViewportWScalingNV`
///     structures specifying viewport parameters.
///
/// The viewport parameters taken from element i of `pViewportWScalings` replace the
/// current state for the viewport index `firstViewport` + i, for i in \[0,
/// `viewportCount`).
///
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub fn vkCmdSetViewportWScalingNV(
  commandBuffer: VkCommandBuffer,
  firstViewport: u32,
  pViewportWScalings: &[VkViewportWScalingNV],
) {
  unsafe {
    let viewportCount = pViewportWScalings.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetViewportWScalingNV.unwrap()(
        commandBuffer.as_raw(),
        firstViewport,
        viewportCount,
        pViewportWScalings.as_raw(),
      )
    })
  }
}

// feature: VK_EXT_direct_mode_display

/// Release access to an acquired VkDisplayKHR
///
/// To release a previously acquired display, call:
///
///   - `physicalDevice` The physical device the display is on.
///
///   - `display` The display to release control of.
///
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub fn vkReleaseDisplayEXT(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkReleaseDisplayEXT.unwrap()(physicalDevice.as_raw(), display.as_raw())
    })
  }
}

// feature: VK_EXT_acquire_xlib_display

/// Acquire access to a VkDisplayKHR using Xlib
///
/// To acquire permission to directly access a display in Vulkan from an X11 server,
/// call:
///
///   - `physicalDevice` The physical device the display is on.
///
///   - `dpy` A connection to the X11 server that currently owns `display`.
///
///   - `display` The display the caller wishes to control in Vulkan.
///
/// All permissions necessary to control the display are granted to the Vulkan
/// instance associated with `physicalDevice` until the display is released or the
/// X11 connection specified by `dpy` is terminated. Permission to access the
/// display may: be temporarily revoked during periods when the X11 server from
/// which control was acquired itself looses access to `display`. During such
/// periods, operations which require access to the display must: fail with an
/// approriate error code. If the X11 server associated with `dpy` does not own
/// `display`, or if permission to access it has already been acquired by another
/// entity, the call must: return the error code `VK_ERROR_INITIALIZATION_FAILED`.
///
/// > **Note**
/// >
/// > One example of when an X11 server loses access to a display is when it loses
/// > ownership of its virtual terminal.
///
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
pub fn vkAcquireXlibDisplayEXT(
  physicalDevice: VkPhysicalDevice,
  dpy: *mut wsi::xlib::Display,
  display: VkDisplayKHR,
) -> VkResult {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkAcquireXlibDisplayEXT.unwrap()(physicalDevice.as_raw(), dpy, display.as_raw())
    })
  }
}

/// Query the VkDisplayKHR corresponding to an X11 RandR Output
///
/// When acquiring displays from an X11 server, an application may also wish to
/// enumerate and identify them using a native handle rather than a `VkDisplayKHR`
/// handle. To determine the `VkDisplayKHR` handle corresponding to an X11 RandR
/// Output, call:
///
///   - `physicalDevice` The physical device to query the display handle on.
///
///   - `dpy` A connection to the X11 server from which `rrOutput` was queried.
///
///   - `rrOutput` An X11 RandR output ID.
///
///   - `pDisplay` The corresponding `VkDisplayKHR` handle will be returned here.
///
/// If there is no `VkDisplayKHR` corresponding to `rrOutput` on `physicalDevice`,
/// `VK_NULL_HANDLE` must: be returned in `pDisplay`.
///
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
pub fn vkGetRandROutputDisplayEXT(
  physicalDevice: VkPhysicalDevice,
  dpy: *mut wsi::xlib::Display,
  rrOutput: wsi::xlib::RROutput,
) -> Result<VkDisplayKHR, VkResult> {
  unsafe {
    let mut pDisplay: VkDisplayKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetRandROutputDisplayEXT.unwrap()(
        physicalDevice.as_raw(),
        dpy,
        rrOutput,
        (&mut pDisplay).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pDisplay);
    })
  }
}

// feature: VK_EXT_display_surface_counter

/// Query surface capabilities
///
/// To query the basic capabilities of a surface, needed in order to create a
/// swapchain, call:
///
///   - `physicalDevice` is the physical device that will be associated with the
///     swapchain to be created, as described for `vkCreateSwapchainKHR`.
///
///   - `surface` is the surface that will be associated with the swapchain.
///
///   - `pSurfaceCapabilities` is a pointer to an instance of the
///     `VkSurfaceCapabilities2EXT` structure in which the capabilities are
///     returned.
///
/// `vkGetPhysicalDeviceSurfaceCapabilities2EXT` behaves similarly to
/// `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`, with the ability to return extended
/// information by adding extension structures to the `pNext` chain of its
/// `pSurfaceCapabilities` parameter.
///
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
) -> Result<VkSurfaceCapabilities2EXT, VkResult> {
  unsafe {
    let mut pSurfaceCapabilities: VkSurfaceCapabilities2EXT = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceSurfaceCapabilities2EXT.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        (&mut pSurfaceCapabilities).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurfaceCapabilities);
    })
  }
}

// feature: VK_EXT_display_control

/// Set the power state of a display
///
/// To set the power state of a display, call:
///
///   - `device` is a logical device associated with `display`.
///
///   - `display` is the display whose power state is modified.
///
///   - `pDisplayPowerInfo` is an instance of `VkDisplayPowerInfoEXT` specifying the
///     new power state of `display`.
///
#[cfg(feature = "VK_EXT_display_control")]
pub fn vkDisplayPowerControlEXT(
  device: VkDevice,
  display: VkDisplayKHR,
  pDisplayPowerInfo: &VkDisplayPowerInfoEXT,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDisplayPowerControlEXT.unwrap()(
        device.as_raw(),
        display.as_raw(),
        pDisplayPowerInfo.as_raw(),
      )
    })
  }
}

/// Signal a fence when a device event occurs
///
/// To create a fence that will be signaled when an event occurs on a device, call:
///
///   - `device` is a logical device on which the event may: occur.
///
///   - `pDeviceEventInfo` is a pointer to an instance of the `VkDeviceEventInfoEXT`
///     structure describing the event of interest to the application.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pFence` points to a handle in which the resulting fence object is returned.
///
#[cfg(feature = "VK_EXT_display_control")]
pub fn vkRegisterDeviceEventEXT(
  device: VkDevice,
  pDeviceEventInfo: &VkDeviceEventInfoEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkFence, VkResult> {
  unsafe {
    let mut pFence: VkFence = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkRegisterDeviceEventEXT.unwrap()(
        device.as_raw(),
        pDeviceEventInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pFence).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pFence);
    })
  }
}

/// Signal a fence when a display event occurs
///
/// To create a fence that will be signaled when an event occurs on a `VkDisplayKHR`
/// object, call:
///
///   - `device` is a logical device associated with `display`
///
///   - `display` is the display on which the event may: occur.
///
///   - `pDisplayEventInfo` is a pointer to an instance of the
///     `VkDisplayEventInfoEXT` structure describing the event of interest to the
///     application.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pFence` points to a handle in which the resulting fence object is returned.
///
#[cfg(feature = "VK_EXT_display_control")]
pub fn vkRegisterDisplayEventEXT(
  device: VkDevice,
  display: VkDisplayKHR,
  pDisplayEventInfo: &VkDisplayEventInfoEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkFence, VkResult> {
  unsafe {
    let mut pFence: VkFence = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkRegisterDisplayEventEXT.unwrap()(
        device.as_raw(),
        display.as_raw(),
        pDisplayEventInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pFence).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pFence);
    })
  }
}

/// Query the current value of a surface counter
///
/// The requested counters become active when the first presentation command for the
/// associated swapchain is processed by the presentation engine. To query the value
/// of an active counter, use:
///
///   - `device` is the `VkDevice` associated with `swapchain`.
///
///   - `swapchain` is the swapchain from which to query the counter value.
///
///   - `counter` is the counter to query.
///
///   - `pCounterValue` will return the current value of the counter.
///
/// If a counter is not available because the swapchain is out of date, the
/// implementation may: return `VK_ERROR_OUT_OF_DATE_KHR`.
///
#[cfg(feature = "VK_EXT_display_control")]
pub fn vkGetSwapchainCounterEXT(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  counter: VkSurfaceCounterFlagBitsEXT,
) -> Result<u64, VkResult> {
  unsafe {
    let mut pCounterValue: u64 = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetSwapchainCounterEXT.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        counter,
        &mut pCounterValue,
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pCounterValue);
    })
  }
}

// feature: VK_GOOGLE_display_timing

/// Obtain the RC duration of the PE's display
///
/// To query the duration of a refresh cycle (RC) for the presentation engine’s
/// display, call:
///
///   - `device` is the device associated with `swapchain`.
///
///   - `swapchain` is the swapchain to obtain the refresh duration for.
///
///   - `pDisplayTimingProperties` is a pointer to an instance of the
///     `VkRefreshCycleDurationGOOGLE` structure.
///
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub fn vkGetRefreshCycleDurationGOOGLE(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
) -> Result<VkRefreshCycleDurationGOOGLE, VkResult> {
  unsafe {
    let mut pDisplayTimingProperties: VkRefreshCycleDurationGOOGLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetRefreshCycleDurationGOOGLE.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        (&mut pDisplayTimingProperties).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pDisplayTimingProperties);
    })
  }
}

/// Obtain timing of a previously-presented image
///
/// The implementation will maintain a limited amount of history of timing
/// information about previous presents. Because of the asynchronous nature of the
/// presentation engine, the timing information for a given `vkQueuePresentKHR`
/// command will become available some time later. These time values can be
/// asynchronously queried, and will be returned if available. All time values are
/// in nanoseconds, relative to a monotonically-increasing clock (e.g.
/// `CLOCK_MONOTONIC` (see clock\_gettime(2)) on Android and Linux).
///
/// To asynchronously query the presentation engine, for newly-available timing
/// information about one or more previous presents to a given swapchain, call:
///
///   - `device` is the device associated with `swapchain`.
///
///   - `swapchain` is the swapchain to obtain presentation timing information
///     duration for.
///
///   - `pPresentationTimingCount` is a pointer to an integer related to the number
///     of `VkPastPresentationTimingGOOGLE` structures to query, as described below.
///
///   - `pPresentationTimings` is either `NULL` or a pointer to an an array of
///     `VkPastPresentationTimingGOOGLE` structures.
///
/// If `pPresentationTimings` is `NULL`, then the number of newly-available timing
/// records for the given `swapchain` is returned in `pPresentationTimingCount`.
/// Otherwise, `pPresentationTimingCount` must: point to a variable set by the user
/// to the number of elements in the `pPresentationTimings` array, and on return the
/// variable is overwritten with the number of structures actually written to
/// `pPresentationTimings`. If the value of `pPresentationTimingCount` is less than
/// the number of newly-available timing records, at most `pPresentationTimingCount`
/// structures will be written. If `pPresentationTimingCount` is smaller than the
/// number of newly-available timing records for the given `swapchain`,
/// `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS` to indicate that not
/// all the available values were returned.
///
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub fn vkGetPastPresentationTimingGOOGLE(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
) -> Result<Vec<VkPastPresentationTimingGOOGLE>, VkResult> {
  unsafe {
    let mut pPresentationTimingCount: u32 = 0;
    let mut pPresentationTimings: Vec<VkPastPresentationTimingGOOGLE> = Vec::new();
    VkDeviceDispatchTable::with(device, |_t| loop {
      let _r = _t.vkGetPastPresentationTimingGOOGLE.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        &mut pPresentationTimingCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pPresentationTimingCount == 0 {
        return Ok(pPresentationTimings);
      }
      pPresentationTimings = Vec::with_capacity(pPresentationTimingCount as usize);
      let _r = _t.vkGetPastPresentationTimingGOOGLE.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        &mut pPresentationTimingCount,
        pPresentationTimings.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pPresentationTimings.set_len(pPresentationTimingCount as usize);
      return Ok(pPresentationTimings);
    })
  }
}

// feature: VK_EXT_discard_rectangles

/// Set discard rectangles dynamically
///
/// If the pipeline state object was created with the
/// `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` dynamic state enabled, the discard
/// rectangles are dynamically set and changed with the command.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `firstDiscardRectangle` is the index of the first discard rectangle whose
///     state is updated by the command.
///
///   - `discardRectangleCount` is the number of discard rectangles whose state are
///     updated by the command.
///
///   - `pDiscardRectangles` is a pointer to an array of `VkRect2D` structures
///     specifying discard rectangles.
///
/// The discard rectangle taken from element i of `pDiscardRectangles` replace the
/// current state for the discard rectangle index `firstDiscardRectangle` + i, for i
/// in \[0, `discardRectangleCount`).
///
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub fn vkCmdSetDiscardRectangleEXT(
  commandBuffer: VkCommandBuffer,
  firstDiscardRectangle: u32,
  pDiscardRectangles: &[VkRect2D],
) {
  unsafe {
    let discardRectangleCount = pDiscardRectangles.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetDiscardRectangleEXT.unwrap()(
        commandBuffer.as_raw(),
        firstDiscardRectangle,
        discardRectangleCount,
        pDiscardRectangles.as_raw(),
      )
    })
  }
}

// feature: VK_EXT_hdr_metadata

/// function to set Hdr metadata
///
///   - `device` is the logical device where the swapchain(s) were created.
///
///   - `swapchainCount` is the number of swapchains included in `pSwapchains`.
///
///   - `pSwapchains` is a pointer to the array of `swapchainCount` `VkSwapchainKHR`
///     handles.
///
///   - `pMetadata` is a pointer to the array of `swapchainCount` `VkHdrMetadataEXT`
///     structures.
///
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub fn vkSetHdrMetadataEXT(device: VkDevice, pSwapchains: &[VkSwapchainKHR], pMetadata: &[VkHdrMetadataEXT]) {
  unsafe {
    let swapchainCount = pSwapchains.len() as u32;
    assert!(swapchainCount as usize == pMetadata.len());
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkSetHdrMetadataEXT.unwrap()(
        device.as_raw(),
        swapchainCount,
        pSwapchains.as_raw(),
        pMetadata.as_raw(),
      )
    })
  }
}

// feature: VK_KHR_get_surface_capabilities2

/// Reports capabilities of a surface on a physical device
///
/// To query the basic capabilities of a surface defined by the core or extensions,
/// call:
///
///   - `physicalDevice` is the physical device that will be associated with the
///     swapchain to be created, as described for `vkCreateSwapchainKHR`.
///
///   - `pSurfaceInfo` points to an instance of the
///     `VkPhysicalDeviceSurfaceInfo2KHR` structure, describing the surface and
///     other fixed parameters that would be consumed by `vkCreateSwapchainKHR`.
///
///   - `pSurfaceCapabilities` points to an instance of the
///     `VkSurfaceCapabilities2KHR` structure in which the capabilities are
///     returned.
///
/// `vkGetPhysicalDeviceSurfaceCapabilities2KHR` behaves similarly to
/// `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`, with the ability to specify
/// extended inputs via chained input structures, and to return extended information
/// via chained output structures.
///
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(
  physicalDevice: VkPhysicalDevice,
  pSurfaceInfo: &VkPhysicalDeviceSurfaceInfo2KHR,
) -> Result<VkSurfaceCapabilities2KHR, VkResult> {
  unsafe {
    let mut pSurfaceCapabilities: VkSurfaceCapabilities2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceSurfaceCapabilities2KHR.unwrap()(
        physicalDevice.as_raw(),
        pSurfaceInfo.as_raw(),
        (&mut pSurfaceCapabilities).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurfaceCapabilities);
    })
  }
}

/// Query color formats supported by surface
///
/// To query the supported swapchain format tuples for a surface, call:
///
///   - `physicalDevice` is the physical device that will be associated with the
///     swapchain to be created, as described for `vkCreateSwapchainKHR`.
///
///   - `pSurfaceInfo` points to an instance of the
///     `VkPhysicalDeviceSurfaceInfo2KHR` structure, describing the surface and
///     other fixed parameters that would be consumed by `vkCreateSwapchainKHR`.
///
///   - `pSurfaceFormatCount` is a pointer to an integer related to the number of
///     format tuples available or queried, as described below.
///
///   - `pSurfaceFormats` is either `NULL` or a pointer to an array of
///     `VkSurfaceFormat2KHR` structures.
///
/// If `pSurfaceFormats` is `NULL`, then the number of format tuples supported for
/// the given `surface` is returned in `pSurfaceFormatCount`. The number of format
/// tuples supported will be greater than or equal to 1. Otherwise,
/// `pSurfaceFormatCount` must: point to a variable set by the user to the number of
/// elements in the `pSurfaceFormats` array, and on return the variable is
/// overwritten with the number of structures actually written to `pSurfaceFormats`.
/// If the value of `pSurfaceFormatCount` is less than the number of format tuples
/// supported, at most `pSurfaceFormatCount` structures will be written. If
/// `pSurfaceFormatCount` is smaller than the number of format tuples supported for
/// the surface parameters described in `pSurfaceInfo`, `VK_INCOMPLETE` will be
/// returned instead of `VK_SUCCESS` to indicate that not all the available values
/// were returned.
///
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub fn vkGetPhysicalDeviceSurfaceFormats2KHR(
  physicalDevice: VkPhysicalDevice,
  pSurfaceInfo: &VkPhysicalDeviceSurfaceInfo2KHR,
) -> Result<Vec<VkSurfaceFormat2KHR>, VkResult> {
  unsafe {
    let mut pSurfaceFormatCount: u32 = 0;
    let mut pSurfaceFormats: Vec<VkSurfaceFormat2KHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetPhysicalDeviceSurfaceFormats2KHR.unwrap()(
        physicalDevice.as_raw(),
        pSurfaceInfo.as_raw(),
        &mut pSurfaceFormatCount,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pSurfaceFormatCount == 0 {
        return Ok(pSurfaceFormats);
      }
      pSurfaceFormats = Vec::with_capacity(pSurfaceFormatCount as usize);
      let _r = _t.vkGetPhysicalDeviceSurfaceFormats2KHR.unwrap()(
        physicalDevice.as_raw(),
        pSurfaceInfo.as_raw(),
        &mut pSurfaceFormatCount,
        pSurfaceFormats.as_mut_slice().as_raw(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pSurfaceFormats.set_len(pSurfaceFormatCount as usize);
      return Ok(pSurfaceFormats);
    })
  }
}

// feature: VK_KHR_shared_presentable_image

/// Get a swapchain's status
///
/// In order to query a swapchain’s status when rendering to a shared presentable
/// image, call:
///
///   - `device` is the device associated with `swapchain`.
///
///   - `swapchain` is the swapchain to query.
///
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub fn vkGetSwapchainStatusKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetSwapchainStatusKHR.unwrap()(device.as_raw(), swapchain.as_raw())
    })
  }
}

// feature: VK_KHR_external_fence_capabilities

/// Function for querying external fence handle capabilities.
///
/// Fences may: support import and export of their
/// [payload](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-fences-payloads) to external handles.
///
///   - `physicalDevice` is the physical device from which to query the fence
///     capabilities.
///
///   - `pExternalFenceInfo` points to an instance of the
///     `VkPhysicalDeviceExternalFenceInfoKHR` structure, describing the parameters
///     that would be consumed by `vkCreateFence`.
///
///   - `pExternalFenceProperties` points to an instance of the
///     `VkExternalFencePropertiesKHR` structure in which capabilities are returned.
///
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub fn vkGetPhysicalDeviceExternalFencePropertiesKHR(
  physicalDevice: VkPhysicalDevice,
  pExternalFenceInfo: &VkPhysicalDeviceExternalFenceInfoKHR,
) -> VkExternalFencePropertiesKHR {
  unsafe {
    let mut pExternalFenceProperties: VkExternalFencePropertiesKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceExternalFencePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        pExternalFenceInfo.as_raw(),
        (&mut pExternalFenceProperties).as_raw(),
      );
      return pExternalFenceProperties;
    })
  }
}

// feature: VK_KHR_external_fence_win32

/// Import a fence from a Windows HANDLE
///
///   - `device` is the logical device that created the fence.
///
///   - `pImportFenceWin32HandleInfo` points to a `VkImportFenceWin32HandleInfoKHR`
///     structure specifying the fence and import parameters.
///
/// Importing a fence payload from Windows handles does not transfer ownership of
/// the handle to the Vulkan implementation. For handle types defined as NT handles,
/// the application must: release ownership using the `CloseHandle` system call when
/// the handle is no longer needed.
///
/// Applications can: import the same fence payload into multiple instances of
/// Vulkan, into the same instance from which it was exported, and multiple times
/// into a given Vulkan instance.
///
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkImportFenceWin32HandleKHR(
  device: VkDevice,
  pImportFenceWin32HandleInfo: &VkImportFenceWin32HandleInfoKHR,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportFenceWin32HandleKHR.unwrap()(device.as_raw(), pImportFenceWin32HandleInfo.as_raw())
    })
  }
}

/// Get a Windows HANDLE for a fence
///
///   - `device` is the logical device that created the fence being exported.
///
///   - `pGetWin32HandleInfo` is a pointer to an instance of the
///     `VkFenceGetWin32HandleInfoKHR` structure containing parameters of the export
///     operation.
///
///   - `pHandle` will return the Windows handle representing the fence state.
///
/// For handle types defined as NT handles, the handles returned by
/// `vkGetFenceWin32HandleKHR` are owned by the application. To avoid leaking
/// resources, the application must: release ownership of them using the
/// `CloseHandle` system call when they are no longer needed.
///
/// Exporting a Windows handle from a fence may: have side effects depending on the
/// transference of the specified handle type, as described in [Importing Fence
/// Payloads](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-fences-importing).
///
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetFenceWin32HandleKHR(
  device: VkDevice,
  pGetWin32HandleInfo: &VkFenceGetWin32HandleInfoKHR,
) -> Result<wsi::win32::HANDLE, VkResult> {
  unsafe {
    let mut pHandle: wsi::win32::HANDLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetFenceWin32HandleKHR.unwrap()(device.as_raw(), pGetWin32HandleInfo.as_raw(), &mut pHandle);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pHandle);
    })
  }
}

// feature: VK_KHR_external_fence_fd

/// Import a fence from a POSIX file descriptor
///
///   - `device` is the logical device that created the fence.
///
///   - `pImportFenceFdInfo` points to a `VkImportFenceFdInfoKHR` structure
///     specifying the fence and import parameters.
///
/// Importing a fence payload from a file descriptor transfers ownership of the file
/// descriptor from the application to the Vulkan implementation. The application
/// must: not perform any operations on the file descriptor after a successful
/// import.
///
/// Applications can: import the same fence payload into multiple instances of
/// Vulkan, into the same instance from which it was exported, and multiple times
/// into a given Vulkan instance.
///
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub fn vkImportFenceFdKHR(device: VkDevice, pImportFenceFdInfo: &VkImportFenceFdInfoKHR) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportFenceFdKHR.unwrap()(device.as_raw(), pImportFenceFdInfo.as_raw())
    })
  }
}

/// Get a POSIX file descriptor handle for a fence
///
///   - `device` is the logical device that created the fence being exported.
///
///   - `pGetFdInfo` is a pointer to an instance of the `VkFenceGetFdInfoKHR`
///     structure containing parameters of the export operation.
///
///   - `pFd` will return the file descriptor representing the fence payload.
///
/// Each call to `vkGetFenceFdKHR` must: create a new file descriptor and transfer
/// ownership of it to the application. To avoid leaking resources, the application
/// must: release ownership of the file descriptor when it is no longer needed.
///
/// > **Note**
/// >
/// > Ownership can be released in many ways. For example, the application can call
/// > `close`() on the file descriptor, or transfer ownership back to Vulkan by
/// > using the file descriptor to import a fence payload.
///
/// If `pGetFdInfo::handleType` is `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR`
/// and the fence is signaled at the time `vkGetFenceFdKHR` is called, `pFd` may:
/// return the value `-1` instead of a valid file descriptor.
///
/// Where supported by the operating system, the implementation must: set the file
/// descriptor to be closed automatically when an `execve` system call is made.
///
/// Exporting a file descriptor from a fence may: have side effects depending on the
/// transference of the specified handle type, as described in [Importing Fence
/// State](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-fences-importing).
///
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub fn vkGetFenceFdKHR(device: VkDevice, pGetFdInfo: &VkFenceGetFdInfoKHR) -> Result<c_int, VkResult> {
  unsafe {
    let mut pFd: c_int = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetFenceFdKHR.unwrap()(device.as_raw(), pGetFdInfo.as_raw(), &mut pFd);
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pFd);
    })
  }
}

// feature: VK_MVK_ios_surface

/// Create a VkSurfaceKHR object for an iOS UIView
///
/// To create a `VkSurfaceKHR` object for an iOS `UIView`, call:
///
///   - `instance` is the instance with which to associate the surface.
///
///   - `pCreateInfo` is a pointer to an instance of the `VkIOSSurfaceCreateInfoMVK`
///     structure containing parameters affecting the creation of the surface
///     object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub fn vkCreateIOSSurfaceMVK(
  instance: VkInstance,
  pCreateInfo: &VkIOSSurfaceCreateInfoMVK,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateIOSSurfaceMVK.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_MVK_macos_surface

/// Create a VkSurfaceKHR object for a macOS NSView
///
/// To create a `VkSurfaceKHR` object for a macOS `NSView`, call:
///
///   - `instance` is the instance with which to associate the surface.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkMacOSSurfaceCreateInfoMVK` structure containing parameters affecting the
///     creation of the surface object.
///
///   - `pAllocator` is the allocator used for host memory allocated for the surface
///     object when there is no more specific allocator available (see [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation)).
///
///   - `pSurface` points to a `VkSurfaceKHR` handle in which the created surface
///     object is returned.
///
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub fn vkCreateMacOSSurfaceMVK(
  instance: VkInstance,
  pCreateInfo: &VkMacOSSurfaceCreateInfoMVK,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSurfaceKHR, VkResult> {
  unsafe {
    let mut pSurface: VkSurfaceKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateMacOSSurfaceMVK.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_KHR_get_memory_requirements2

/// Returns the memory requirements for specified Vulkan object
///
///   - `device` is the logical device that owns the image.
///
///   - `pInfo` is a pointer to an instance of the
///     `VkImageMemoryRequirementsInfo2KHR` structure containing parameters required
///     for the memory requirements query.
///
///   - `pMemoryRequirements` points to an instance of the
///     `VkMemoryRequirements2KHR` structure in which the memory requirements of the
///     image object are returned.
///
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub fn vkGetImageMemoryRequirements2KHR(
  device: VkDevice,
  pInfo: &VkImageMemoryRequirementsInfo2KHR,
) -> VkMemoryRequirements2KHR {
  unsafe {
    let mut pMemoryRequirements: VkMemoryRequirements2KHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetImageMemoryRequirements2KHR.unwrap()(
        device.as_raw(),
        pInfo.as_raw(),
        (&mut pMemoryRequirements).as_raw(),
      );
      return pMemoryRequirements;
    })
  }
}

/// Returns the memory requirements for specified Vulkan object
///
///   - `device` is the logical device that owns the buffer.
///
///   - `pInfo` is a pointer to an instance of the
///     `VkBufferMemoryRequirementsInfo2KHR` structure containing parameters
///     required for the memory requirements query.
///
///   - `pMemoryRequirements` points to an instance of the
///     `VkMemoryRequirements2KHR` structure in which the memory requirements of the
///     buffer object are returned.
///
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub fn vkGetBufferMemoryRequirements2KHR(
  device: VkDevice,
  pInfo: &VkBufferMemoryRequirementsInfo2KHR,
) -> VkMemoryRequirements2KHR {
  unsafe {
    let mut pMemoryRequirements: VkMemoryRequirements2KHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetBufferMemoryRequirements2KHR.unwrap()(
        device.as_raw(),
        pInfo.as_raw(),
        (&mut pMemoryRequirements).as_raw(),
      );
      return pMemoryRequirements;
    })
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub fn vkGetImageSparseMemoryRequirements2KHR(
  device: VkDevice,
  pInfo: &VkImageSparseMemoryRequirementsInfo2KHR,
) -> Vec<VkSparseImageMemoryRequirements2KHR> {
  unsafe {
    let mut pSparseMemoryRequirementCount: u32 = 0;
    let mut pSparseMemoryRequirements: Vec<VkSparseImageMemoryRequirements2KHR> = Vec::new();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetImageSparseMemoryRequirements2KHR.unwrap()(
        device.as_raw(),
        pInfo.as_raw(),
        &mut pSparseMemoryRequirementCount,
        ::std::ptr::null_mut(),
      );
      if pSparseMemoryRequirementCount == 0 {
        return pSparseMemoryRequirements;
      }
      pSparseMemoryRequirements = Vec::with_capacity(pSparseMemoryRequirementCount as usize);
      _t.vkGetImageSparseMemoryRequirements2KHR.unwrap()(
        device.as_raw(),
        pInfo.as_raw(),
        &mut pSparseMemoryRequirementCount,
        pSparseMemoryRequirements.as_mut_slice().as_raw(),
      );
      pSparseMemoryRequirements.set_len(pSparseMemoryRequirementCount as usize);
      return pSparseMemoryRequirements;
    })
  }
}

// feature: VK_EXT_sample_locations

/// Set the dynamic sample locations state
///
/// The custom sample locations used for rasterization when
/// `VkPipelineSampleLocationsStateCreateInfoEXT::sampleLocationsEnable` is
/// `VK_TRUE` are specified by the
/// `VkPipelineSampleLocationsStateCreateInfoEXT::sampleLocationsInfo` property of
/// the currently bound graphics pipeline, if the pipeline was not created with
/// `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` enabled.
///
/// Otherwise, the sample locations used for rasterization are set by calling
/// `vkCmdSetSampleLocationsEXT`.
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `pSampleLocationsInfo` is the sample locations state to set.
///
#[cfg(feature = "VK_EXT_sample_locations")]
pub fn vkCmdSetSampleLocationsEXT(commandBuffer: VkCommandBuffer, pSampleLocationsInfo: &VkSampleLocationsInfoEXT) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetSampleLocationsEXT.unwrap()(commandBuffer.as_raw(), pSampleLocationsInfo.as_raw())
    })
  }
}

/// Report sample count specific multisampling capabilities of a physical device
///
/// In addition to the minimum capabilities described in the previous section
/// ([Limits](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits)), implementations may: support additional
/// multisampling capabilities specific to a particular sample count.
///
///   - `physicalDevice` is the physical device from which to query the additional
///     multisampling capabilities.
///
///   - `samples` is the sample count to query the capabilities for.
///
///   - `pMultisampleProperties` is a pointer to a structure of type
///     `VkMultisamplePropertiesEXT`, in which information about the additional
///     multisampling capabilities specific to the sample count is returned.
///
#[cfg(feature = "VK_EXT_sample_locations")]
pub fn vkGetPhysicalDeviceMultisamplePropertiesEXT(
  physicalDevice: VkPhysicalDevice,
  samples: VkSampleCountFlagBits,
) -> VkMultisamplePropertiesEXT {
  unsafe {
    let mut pMultisampleProperties: VkMultisamplePropertiesEXT = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceMultisamplePropertiesEXT.unwrap()(
        physicalDevice.as_raw(),
        samples,
        (&mut pMultisampleProperties).as_raw(),
      );
      return pMultisampleProperties;
    })
  }
}

// feature: VK_KHR_bind_memory2

/// Bind device memory to buffer objects
///
///   - `device` is the logical device that owns the buffers and memory.
///
///   - `bindInfoCount` is the number of elements in `pBindInfos`.
///
///   - `pBindInfos` is a pointer to an array of structures of type
///     `VkBindBufferMemoryInfoKHR`, describing buffers and memory to bind.
///
/// On some implementations, it may: be more efficient to batch memory bindings into
/// a single command.
///
#[cfg(feature = "VK_KHR_bind_memory2")]
pub fn vkBindBufferMemory2KHR(device: VkDevice, pBindInfos: &[VkBindBufferMemoryInfoKHR]) -> VkResult {
  unsafe {
    let bindInfoCount = pBindInfos.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindBufferMemory2KHR.unwrap()(device.as_raw(), bindInfoCount, pBindInfos.as_raw())
    })
  }
}

/// Bind device memory to image objects
///
///   - `device` is the logical device that owns the images and memory.
///
///   - `bindInfoCount` is the number of elements in `pBindInfos`.
///
///   - `pBindInfos` is a pointer to an array of structures of type
///     `VkBindImageMemoryInfoKHR`, describing images and memory to bind.
///
/// On some implementations, it may: be more efficient to batch memory bindings into
/// a single command.
///
#[cfg(feature = "VK_KHR_bind_memory2")]
pub fn vkBindImageMemory2KHR(device: VkDevice, pBindInfos: &[VkBindImageMemoryInfoKHR]) -> VkResult {
  unsafe {
    let bindInfoCount = pBindInfos.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindImageMemory2KHR.unwrap()(device.as_raw(), bindInfoCount, pBindInfos.as_raw())
    })
  }
}

// feature: VK_KHR_sampler_ycbcr_conversion

/// Create a new Ycbcr conversion
///
///   - `device` is the logical device that creates the sampler
///     Y’C<sub>B</sub>C<sub>R</sub> conversion.
///
///   - `pCreateInfo` is a pointer to an instance of the
///     `VkSamplerYcbcrConversionCreateInfoKHR` specifying the requested sampler
///     Y’C<sub>B</sub>C<sub>R</sub> conversion.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pYcbcrConversion` points to a `VkSamplerYcbcrConversionKHR` handle in which
///     the resulting sampler Y’C<sub>B</sub>C<sub>R</sub> conversion is returned.
///
/// The interpretation of the configured sampler Y’C<sub>B</sub>C<sub>R</sub>
/// conversion is described in more detail in [the description of sampler
/// Y’C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-sampler-YCbCr-conversion)
/// in the [Image Operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures) chapter.
///
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub fn vkCreateSamplerYcbcrConversionKHR(
  device: VkDevice,
  pCreateInfo: &VkSamplerYcbcrConversionCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkSamplerYcbcrConversionKHR, VkResult> {
  unsafe {
    let mut pYcbcrConversion: VkSamplerYcbcrConversionKHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateSamplerYcbcrConversionKHR.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pYcbcrConversion).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pYcbcrConversion);
    })
  }
}

/// Destroy a created Y'CbCr conversion
///
///   - `device` is the logical device that destroys the
///     Y’C<sub>B</sub>C<sub>R</sub> conversion.
///
///   - `ycbcrConversion` is the conversion to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub fn vkDestroySamplerYcbcrConversionKHR(
  device: VkDevice,
  ycbcrConversion: Option<VkSamplerYcbcrConversionKHR>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySamplerYcbcrConversionKHR.unwrap()(
        device.as_raw(),
        ycbcrConversion.as_raw(),
        pAllocator.as_raw(),
      )
    })
  }
}

// feature: VK_EXT_validation_cache

/// Creates a new validation cache
///
///   - `device` is the logical device that creates the validation cache object.
///
///   - `pCreateInfo` is a pointer to a `VkValidationCacheCreateInfoEXT` structure
///     that contains the initial parameters for the validation cache object.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
///   - `pValidationCache` is a pointer to a `VkValidationCacheEXT` handle in which
///     the resulting validation cache object is returned.
///
/// > **Note**
/// >
/// > Applications can: track and manage the total host memory size of a validation
/// > cache object using the `pAllocator`. Applications can: limit the amount of
/// > data retrieved from a validation cache object in
/// > `vkGetValidationCacheDataEXT`. Implementations should: not internally limit
/// > the total number of entries added to a validation cache object or the total
/// > host memory consumed.
///
/// Once created, a validation cache can: be passed to the `vkCreateShaderModule`
/// command as part of the `VkShaderModuleCreateInfo` `pNext` chain. If a
/// `VkShaderModuleValidationCacheCreateInfoEXT` object is part of the
/// `VkShaderModuleCreateInfo::pNext` chain, and its `validationCache` field is not
/// `VK_NULL_HANDLE`, the implementation will query it for possible reuse
/// opportunities and update it with new content. The use of the validation cache
/// object in these commands is internally synchronized, and the same validation
/// cache object can: be used in multiple threads simultaneously.
///
/// > **Note**
/// >
/// > Implementations should: make every effort to limit any critical sections to
/// > the actual accesses to the cache, which is expected to be significantly
/// > shorter than the duration of the `vkCreateShaderModule` command.
///
#[cfg(feature = "VK_EXT_validation_cache")]
pub fn vkCreateValidationCacheEXT(
  device: VkDevice,
  pCreateInfo: &VkValidationCacheCreateInfoEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> Result<VkValidationCacheEXT, VkResult> {
  unsafe {
    let mut pValidationCache: VkValidationCacheEXT = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateValidationCacheEXT.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pValidationCache).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pValidationCache);
    })
  }
}

/// Destroy a validation cache object
///
///   - `device` is the logical device that destroys the validation cache object.
///
///   - `validationCache` is the handle of the validation cache to destroy.
///
///   - `pAllocator` controls host memory allocation as described in the [Memory
///     Allocation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-allocation) chapter.
///
#[cfg(feature = "VK_EXT_validation_cache")]
pub fn vkDestroyValidationCacheEXT(
  device: VkDevice,
  validationCache: Option<VkValidationCacheEXT>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyValidationCacheEXT.unwrap()(
        device.as_raw(),
        validationCache.as_raw(),
        pAllocator.as_raw(),
      )
    })
  }
}

/// Combine the data stores of validation caches
///
/// Validation cache objects can: be merged using the command.
///
///   - `device` is the logical device that owns the validation cache objects.
///
///   - `dstCache` is the handle of the validation cache to merge results into.
///
///   - `srcCacheCount` is the length of the `pSrcCaches` array.
///
///   - `pSrcCaches` is an array of validation cache handles, which will be merged
///     into `dstCache`. The previous contents of `dstCache` are included after the
///     merge.
///
/// > **Note**
/// >
/// > The details of the merge operation are implementation dependent, but
/// > implementations should: merge the contents of the specified validation caches
/// > and prune duplicate entries.
///
#[cfg(feature = "VK_EXT_validation_cache")]
pub fn vkMergeValidationCachesEXT(
  device: VkDevice,
  dstCache: VkValidationCacheEXT,
  pSrcCaches: &[VkValidationCacheEXT],
) -> VkResult {
  unsafe {
    let srcCacheCount = pSrcCaches.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkMergeValidationCachesEXT.unwrap()(
        device.as_raw(),
        dstCache.as_raw(),
        srcCacheCount,
        pSrcCaches.as_raw(),
      )
    })
  }
}

/// Get the data store from a validation cache
///
/// Data can: be retrieved from a validation cache object using the command.
///
///   - `device` is the logical device that owns the validation cache.
///
///   - `validationCache` is the validation cache to retrieve data from.
///
///   - `pDataSize` is a pointer to a value related to the amount of data in the
///     validation cache, as described below.
///
///   - `pData` is either `NULL` or a pointer to a buffer.
///
/// If `pData` is `NULL`, then the maximum size of the data that can: be retrieved
/// from the validation cache, in bytes, is returned in `pDataSize`. Otherwise,
/// `pDataSize` must: point to a variable set by the user to the size of the buffer,
/// in bytes, pointed to by `pData`, and on return the variable is overwritten with
/// the amount of data actually written to `pData`.
///
/// If `pDataSize` is less than the maximum size that can: be retrieved by the
/// validation cache, at most `pDataSize` bytes will be written to `pData`, and
/// `vkGetValidationCacheDataEXT` will return `VK_INCOMPLETE`. Any data written to
/// `pData` is valid and can: be provided as the `pInitialData` member of the
/// `VkValidationCacheCreateInfoEXT` structure passed to
/// `vkCreateValidationCacheEXT`.
///
/// Two calls to `vkGetValidationCacheDataEXT` with the same parameters must:
/// retrieve the same data unless a command that modifies the contents of the cache
/// is called between them.
///
/// Applications can: store the data retrieved from the validation cache, and use
/// these data, possibly in a future run of the application, to populate new
/// validation cache objects. The results of validation, however, may: depend on the
/// vendor ID, device ID, driver version, and other details of the device. To enable
/// applications to detect when previously retrieved data is incompatible with the
/// device, the initial bytes written to `pData` must: be a header consisting of the
/// following members:
///
/// <table>
/// <caption>Layout for validation cache header version <code>VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT</code></caption>
/// <colgroup>
/// <col width="8%" />
/// <col width="21%" />
/// <col width="70%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Offset</th>
/// <th align="left">Size</th>
/// <th align="left">Meaning</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"><p>0</p></td>
/// <td align="left"><p>4</p></td>
/// <td align="left"><p>length in bytes of the entire validation cache header written as a stream of bytes, with the least significant byte first</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p>4</p></td>
/// <td align="left"><p>4</p></td>
/// <td align="left"><p>a <code>VkValidationCacheHeaderVersionEXT</code> value written as a stream of bytes, with the least significant byte first</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p>8</p></td>
/// <td align="left"><p><code>VK_UUID_SIZE</code></p></td>
/// <td align="left"><p>a layer commit ID expressed as a UUID, which uniquely identifies the version of the validation layers used to generate these validation results</p></td>
/// </tr>
/// </tbody>
/// </table>
///
/// The first four bytes encode the length of the entire validation cache header, in
/// bytes. This value includes all fields in the header including the validation
/// cache version field and the size of the length field.
///
/// The next four bytes encode the validation cache version, as described for
/// `VkValidationCacheHeaderVersionEXT`. A consumer of the validation cache should:
/// use the cache version to interpret the remainder of the cache header.
///
/// If `pDataSize` is less than what is necessary to store this header, nothing will
/// be written to `pData` and zero will be written to `pDataSize`.
///
#[cfg(feature = "VK_EXT_validation_cache")]
pub fn vkGetValidationCacheDataEXT(
  device: VkDevice,
  validationCache: VkValidationCacheEXT,
) -> Result<Vec<u8>, VkResult> {
  unsafe {
    let mut pDataSize: usize = 0;
    let mut pData: Vec<u8> = Vec::new();
    VkDeviceDispatchTable::with(device, |_t| loop {
      let _r = _t.vkGetValidationCacheDataEXT.unwrap()(
        device.as_raw(),
        validationCache.as_raw(),
        &mut pDataSize,
        ::std::ptr::null_mut(),
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      if pDataSize == 0 {
        return Ok(pData);
      }
      pData = Vec::with_capacity(pDataSize as usize);
      let _r = _t.vkGetValidationCacheDataEXT.unwrap()(
        device.as_raw(),
        validationCache.as_raw(),
        &mut pDataSize,
        pData.as_mut_slice().as_raw() as *mut c_void,
      );
      if _r == VkResult::E_INCOMPLETE {
        continue;
      }
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pData.set_len(pDataSize as usize);
      return Ok(pData);
    })
  }
}

// feature: VK_EXT_external_memory_host

/// Get properties of external memory host pointer
///
/// To determine the correct parameters to use when importing host pointers, call.
///
///   - `device` is the logical device that will be importing `pHostPointer`.
///
///   - `handleType` is the type of the handle `pHostPointer`.
///
///   - `pHostPointer` is the host pointer to import from.
///
#[cfg(feature = "VK_EXT_external_memory_host")]
pub fn vkGetMemoryHostPointerPropertiesEXT(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pHostPointer: *const c_void,
) -> Result<VkMemoryHostPointerPropertiesEXT, VkResult> {
  unsafe {
    let mut pMemoryHostPointerProperties: VkMemoryHostPointerPropertiesEXT = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryHostPointerPropertiesEXT.unwrap()(
        device.as_raw(),
        handleType,
        pHostPointer,
        (&mut pMemoryHostPointerProperties).as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      return Ok(pMemoryHostPointerProperties);
    })
  }
}

// feature: VK_AMD_buffer_marker

/// Execute a pipelined write of a marker value into a buffer
///
/// To write a 32-bit marker value into a buffer as a pipelined operation, call:
///
///   - `commandBuffer` is the command buffer into which the command will be
///     recorded.
///
///   - `pipelineStage` is one of the `VkPipelineStageFlagBits` values, specifying
///     the pipeline stage whose completion triggers the marker write.
///
///   - `dstBuffer` is the buffer where the marker will be written to.
///
///   - `dstOffset` is the byte offset into the buffer where the marker will be
///     written to.
///
///   - `marker` is the 32-bit value of the marker.
///
/// The command will write the 32-bit marker value into the buffer only after all
/// preceding commands have finished executing up to at least the specified pipeline
/// stage. This includes the completion of other preceding
/// `vkCmdWriteBufferMarkerAMD` commands so long as their specified pipeline stages
/// occur either at the same time or earlier than this command’s specified
/// `pipelineStage`.
///
/// While consecutive buffer marker writes with the same `pipelineStage` parameter
/// are implicitly complete in submission order, memory and execution dependencies
/// between buffer marker writes and other operations must still be explicitly
/// ordered using synchronization commands. The access scope for buffer marker
/// writes falls under the `VK_ACCESS_TRANSFER_WRITE_BIT`, and the pipeline stages
/// for identifying the synchronization scope must: include both `pipelineStage` and
/// `VK_PIPELINE_STAGE_TRANSFER_BIT`.
///
/// > **Note**
/// >
/// > Similar to `vkCmdWriteTimestamp`, if an implementation is unable to write a
/// > marker at any specific pipeline stage, it may: instead do so at any logically
/// > later stage.
///
/// > **Note**
/// >
/// > Implementations may: only support a limited number of pipelined marker write
/// > operations in flight at a given time, thus excessive number of marker write
/// > operations may: degrade command execution performance.
///
#[cfg(feature = "VK_AMD_buffer_marker")]
pub fn vkCmdWriteBufferMarkerAMD(
  commandBuffer: VkCommandBuffer,
  pipelineStage: VkPipelineStageFlagBits,
  dstBuffer: VkBuffer,
  dstOffset: VkDeviceSize,
  marker: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdWriteBufferMarkerAMD.unwrap()(
        commandBuffer.as_raw(),
        pipelineStage,
        dstBuffer.as_raw(),
        dstOffset,
        marker,
      )
    })
  }
}
