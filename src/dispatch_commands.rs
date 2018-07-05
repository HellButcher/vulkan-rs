/* GENERATED FILE */

#![allow(non_snake_case)]

use dispatch_table::*;
use enums;
use enums::{VkError, VkResult};
use platform::*;
use types::*;
use AsRaw;

// feature: VK_VERSION_1_0

/// Create a new Vulkan instance
pub fn vkCreateInstance<'h>(
  pCreateInfo: &VkInstanceCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkInstance<'h>> {
  unsafe {
    let mut pInstance: VkInstance<'h> = ::std::mem::zeroed();
    VkLoaderDispatchTable::with(|_t| {
      let _r = _t.vkCreateInstance.unwrap()(pCreateInfo.as_raw(), pAllocator.as_raw(), (&mut pInstance).as_raw());
      if let Err(_e) = _r {
        return Err(_e);
      }
      VkInstanceDispatchTable::create(pCreateInfo, pAllocator, pInstance);
      return Ok(pInstance);
    })
  }
}

/// Destroy an instance of Vulkan
pub fn vkDestroyInstance<'h>(instance: VkInstance<'h>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkInstanceDispatchTable::with(instance, |_t| {
      _t.vkDestroyInstance.unwrap()(instance.as_raw(), pAllocator.as_raw())
    });
    VkInstanceDispatchTable::destroy(instance);
  }
}

/// Enumerates the physical devices accessible to a Vulkan instance
pub fn vkEnumeratePhysicalDevices<'h>(instance: VkInstance<'h>) -> VkResult<Vec<VkPhysicalDevice<'h>>> {
  unsafe {
    let mut pPhysicalDeviceCount: u32 = 0;
    let mut pPhysicalDevices: Vec<VkPhysicalDevice<'h>> = Vec::new();
    VkInstanceDispatchTable::with(instance, |_t| loop {
      let _r =
        _t.vkEnumeratePhysicalDevices.unwrap()(instance.as_raw(), &mut pPhysicalDeviceCount, ::std::ptr::null_mut());
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPhysicalDeviceCount == 0 {
        return Ok(pPhysicalDevices);
      }
      pPhysicalDevices = Vec::with_capacity(pPhysicalDeviceCount as usize);
      let _r = _t.vkEnumeratePhysicalDevices.unwrap()(
        instance.as_raw(),
        &mut pPhysicalDeviceCount,
        pPhysicalDevices.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pPhysicalDevices.set_len(pPhysicalDeviceCount as usize);
      return Ok(pPhysicalDevices);
    })
  }
}

/// Reports capabilities of a physical device
pub fn vkGetPhysicalDeviceFeatures<'h>(physicalDevice: VkPhysicalDevice<'h>) -> VkPhysicalDeviceFeatures {
  unsafe {
    let mut pFeatures: VkPhysicalDeviceFeatures = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceFeatures.unwrap()(physicalDevice.as_raw(), (&mut pFeatures).as_raw());
      return pFeatures;
    })
  }
}

/// Lists physical device\'s format capabilities
pub fn vkGetPhysicalDeviceFormatProperties<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  format: VkFormat,
) -> VkFormatProperties {
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

/// Lists physical device\'s image format capabilities
pub fn vkGetPhysicalDeviceImageFormatProperties<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  format: VkFormat,
  eType: VkImageType,
  tiling: VkImageTiling,
  usage: VkImageUsageFlags,
  flags: VkImageCreateFlags,
) -> VkResult<VkImageFormatProperties> {
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
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pImageFormatProperties);
    })
  }
}

/// Returns properties of a physical device
pub fn vkGetPhysicalDeviceProperties<'h>(physicalDevice: VkPhysicalDevice<'h>) -> VkPhysicalDeviceProperties {
  unsafe {
    let mut pProperties: VkPhysicalDeviceProperties = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceProperties.unwrap()(physicalDevice.as_raw(), (&mut pProperties).as_raw());
      return pProperties;
    })
  }
}

/// Reports properties of the queues of the specified physical device
pub fn vkGetPhysicalDeviceQueueFamilyProperties<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
) -> Vec<VkQueueFamilyProperties> {
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
        pQueueFamilyProperties.as_mut_ptr().as_raw(),
      );
      pQueueFamilyProperties.set_len(pQueueFamilyPropertyCount as usize);
      return pQueueFamilyProperties;
    })
  }
}

/// Reports memory information for the specified physical device
pub fn vkGetPhysicalDeviceMemoryProperties<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
) -> VkPhysicalDeviceMemoryProperties {
  unsafe {
    let mut pMemoryProperties: VkPhysicalDeviceMemoryProperties = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceMemoryProperties.unwrap()(physicalDevice.as_raw(), (&mut pMemoryProperties).as_raw());
      return pMemoryProperties;
    })
  }
}

/// Create a new device instance
pub fn vkCreateDevice<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  pCreateInfo: &VkDeviceCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkDevice<'h>> {
  unsafe {
    let mut pDevice: VkDevice<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkCreateDevice.unwrap()(
        physicalDevice.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pDevice).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      VkDeviceDispatchTable::create(physicalDevice, pCreateInfo, pAllocator, pDevice);
      return Ok(pDevice);
    })
  }
}

/// Destroy a logical device
pub fn vkDestroyDevice<'h>(device: VkDevice<'h>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyDevice.unwrap()(device.as_raw(), pAllocator.as_raw())
    });
    VkDeviceDispatchTable::destroy(device);
  }
}

/// Returns up to requested number of global extension properties
pub fn vkEnumerateInstanceExtensionProperties(
  pLayerName: Option<&AsRef<CStr>>,
) -> VkResult<Vec<VkExtensionProperties>> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkExtensionProperties> = Vec::new();
    VkLoaderDispatchTable::with(|_t| loop {
      let _r = _t.vkEnumerateInstanceExtensionProperties.unwrap()(
        pLayerName.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkEnumerateInstanceExtensionProperties.unwrap()(
        pLayerName.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Returns properties of available physical device extensions
pub fn vkEnumerateDeviceExtensionProperties<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  pLayerName: Option<&AsRef<CStr>>,
) -> VkResult<Vec<VkExtensionProperties>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkEnumerateDeviceExtensionProperties.unwrap()(
        physicalDevice.as_raw(),
        pLayerName.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Returns up to requested number of global layer properties
pub fn vkEnumerateInstanceLayerProperties() -> VkResult<Vec<VkLayerProperties>> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkLayerProperties> = Vec::new();
    VkLoaderDispatchTable::with(|_t| loop {
      let _r = _t.vkEnumerateInstanceLayerProperties.unwrap()(&mut pPropertyCount, ::std::ptr::null_mut());
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkEnumerateInstanceLayerProperties.unwrap()(&mut pPropertyCount, pProperties.as_mut_ptr().as_raw());
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Returns properties of available physical device layers
pub fn vkEnumerateDeviceLayerProperties<'h>(physicalDevice: VkPhysicalDevice<'h>) -> VkResult<Vec<VkLayerProperties>> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkLayerProperties> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkEnumerateDeviceLayerProperties.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkEnumerateDeviceLayerProperties.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Get a queue handle from a device
pub fn vkGetDeviceQueue<'h>(device: VkDevice<'h>, queueFamilyIndex: u32, queueIndex: u32) -> VkQueue<'h> {
  unsafe {
    let mut pQueue: VkQueue<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetDeviceQueue.unwrap()(device.as_raw(), queueFamilyIndex, queueIndex, (&mut pQueue).as_raw());
      return pQueue;
    })
  }
}

/// Submits a sequence of semaphores or command buffers to a queue
pub fn vkQueueSubmit<'h>(
  queue: VkQueue<'h>,
  pSubmits: &[VkSubmitInfo<'_, 'h>],
  fence: Option<VkFence<'h>>,
) -> VkResult {
  unsafe {
    let submitCount = pSubmits.len() as u32;
    VkDeviceDispatchTable::with(queue, |_t| {
      _t.vkQueueSubmit.unwrap()(queue.as_raw(), submitCount, pSubmits.as_raw(), fence.as_raw())
    })
  }
}

/// Wait for a queue to become idle
pub fn vkQueueWaitIdle<'h>(queue: VkQueue<'h>) -> VkResult {
  unsafe { VkDeviceDispatchTable::with(queue, |_t| _t.vkQueueWaitIdle.unwrap()(queue.as_raw())) }
}

/// Wait for a device to become idle
pub fn vkDeviceWaitIdle<'h>(device: VkDevice<'h>) -> VkResult {
  unsafe { VkDeviceDispatchTable::with(device, |_t| _t.vkDeviceWaitIdle.unwrap()(device.as_raw())) }
}

/// Allocate GPU memory
pub fn vkAllocateMemory<'h>(
  device: VkDevice<'h>,
  pAllocateInfo: &VkMemoryAllocateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkDeviceMemory<'h>> {
  unsafe {
    let mut pMemory: VkDeviceMemory<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAllocateMemory.unwrap()(
        device.as_raw(),
        pAllocateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pMemory).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pMemory);
    })
  }
}

/// Free GPU memory
pub fn vkFreeMemory<'h>(
  device: VkDevice<'h>,
  memory: Option<VkDeviceMemory<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkFreeMemory.unwrap()(device.as_raw(), memory.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Map a memory object into application address space
pub fn vkMapMemory<'h>(
  device: VkDevice<'h>,
  memory: VkDeviceMemory<'h>,
  offset: VkDeviceSize,
  size: VkDeviceSize,
  flags: VkMemoryMapFlags,
) -> VkResult<*mut c_void> {
  unsafe {
    let mut ppData: *mut c_void = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkMapMemory.unwrap()(device.as_raw(), memory.as_raw(), offset, size, flags, &mut ppData);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(ppData);
    })
  }
}

/// Unmap a previously mapped memory object
pub fn vkUnmapMemory<'h>(device: VkDevice<'h>, memory: VkDeviceMemory<'h>) {
  unsafe { VkDeviceDispatchTable::with(device, |_t| _t.vkUnmapMemory.unwrap()(device.as_raw(), memory.as_raw())) }
}

/// Flush mapped memory ranges
pub fn vkFlushMappedMemoryRanges<'h>(device: VkDevice<'h>, pMemoryRanges: &[VkMappedMemoryRange<'_, 'h>]) -> VkResult {
  unsafe {
    let memoryRangeCount = pMemoryRanges.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkFlushMappedMemoryRanges.unwrap()(device.as_raw(), memoryRangeCount, pMemoryRanges.as_raw())
    })
  }
}

/// Invalidate ranges of mapped memory objects
pub fn vkInvalidateMappedMemoryRanges<'h>(
  device: VkDevice<'h>,
  pMemoryRanges: &[VkMappedMemoryRange<'_, 'h>],
) -> VkResult {
  unsafe {
    let memoryRangeCount = pMemoryRanges.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkInvalidateMappedMemoryRanges.unwrap()(device.as_raw(), memoryRangeCount, pMemoryRanges.as_raw())
    })
  }
}

/// Query the current commitment for a VkDeviceMemory
pub fn vkGetDeviceMemoryCommitment<'h>(device: VkDevice<'h>, memory: VkDeviceMemory<'h>) -> VkDeviceSize {
  unsafe {
    let mut pCommittedMemoryInBytes: VkDeviceSize = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetDeviceMemoryCommitment.unwrap()(device.as_raw(), memory.as_raw(), &mut pCommittedMemoryInBytes);
      return pCommittedMemoryInBytes;
    })
  }
}

/// Bind device memory to a buffer object
pub fn vkBindBufferMemory<'h>(
  device: VkDevice<'h>,
  buffer: VkBuffer<'h>,
  memory: VkDeviceMemory<'h>,
  memoryOffset: VkDeviceSize,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindBufferMemory.unwrap()(device.as_raw(), buffer.as_raw(), memory.as_raw(), memoryOffset)
    })
  }
}

/// Bind device memory to an image object
pub fn vkBindImageMemory<'h>(
  device: VkDevice<'h>,
  image: VkImage<'h>,
  memory: VkDeviceMemory<'h>,
  memoryOffset: VkDeviceSize,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindImageMemory.unwrap()(device.as_raw(), image.as_raw(), memory.as_raw(), memoryOffset)
    })
  }
}

/// Returns the memory requirements for specified Vulkan object
pub fn vkGetBufferMemoryRequirements<'h>(device: VkDevice<'h>, buffer: VkBuffer<'h>) -> VkMemoryRequirements {
  unsafe {
    let mut pMemoryRequirements: VkMemoryRequirements = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetBufferMemoryRequirements.unwrap()(device.as_raw(), buffer.as_raw(), (&mut pMemoryRequirements).as_raw());
      return pMemoryRequirements;
    })
  }
}

/// Returns the memory requirements for specified Vulkan object
pub fn vkGetImageMemoryRequirements<'h>(device: VkDevice<'h>, image: VkImage<'h>) -> VkMemoryRequirements {
  unsafe {
    let mut pMemoryRequirements: VkMemoryRequirements = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetImageMemoryRequirements.unwrap()(device.as_raw(), image.as_raw(), (&mut pMemoryRequirements).as_raw());
      return pMemoryRequirements;
    })
  }
}

/// Query the memory requirements for a sparse image
pub fn vkGetImageSparseMemoryRequirements<'h>(
  device: VkDevice<'h>,
  image: VkImage<'h>,
) -> Vec<VkSparseImageMemoryRequirements> {
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
        pSparseMemoryRequirements.as_mut_ptr().as_raw(),
      );
      pSparseMemoryRequirements.set_len(pSparseMemoryRequirementCount as usize);
      return pSparseMemoryRequirements;
    })
  }
}

/// Retrieve properties of an image format applied to sparse images
pub fn vkGetPhysicalDeviceSparseImageFormatProperties<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
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
        pProperties.as_mut_ptr().as_raw(),
      );
      pProperties.set_len(pPropertyCount as usize);
      return pProperties;
    })
  }
}

/// Bind device memory to a sparse resource object
pub fn vkQueueBindSparse<'h>(
  queue: VkQueue<'h>,
  pBindInfo: &[VkBindSparseInfo<'_, 'h>],
  fence: Option<VkFence<'h>>,
) -> VkResult {
  unsafe {
    let bindInfoCount = pBindInfo.len() as u32;
    VkDeviceDispatchTable::with(queue, |_t| {
      _t.vkQueueBindSparse.unwrap()(queue.as_raw(), bindInfoCount, pBindInfo.as_raw(), fence.as_raw())
    })
  }
}

/// Create a new fence object
pub fn vkCreateFence<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkFenceCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkFence<'h>> {
  unsafe {
    let mut pFence: VkFence<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateFence.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pFence).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pFence);
    })
  }
}

/// Destroy a fence object
pub fn vkDestroyFence<'h>(
  device: VkDevice<'h>,
  fence: Option<VkFence<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyFence.unwrap()(device.as_raw(), fence.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Resets one or more fence objects
pub fn vkResetFences<'h>(device: VkDevice<'h>, pFences: &[VkFence<'h>]) -> VkResult {
  unsafe {
    let fenceCount = pFences.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetFences.unwrap()(device.as_raw(), fenceCount, pFences.as_raw())
    })
  }
}

/// Return the status of a fence
pub fn vkGetFenceStatus<'h>(device: VkDevice<'h>, fence: VkFence<'h>) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetFenceStatus.unwrap()(device.as_raw(), fence.as_raw())
    })
  }
}

/// Wait for one or more fences to become signaled
pub fn vkWaitForFences<'h>(device: VkDevice<'h>, pFences: &[VkFence<'h>], waitAll: VkBool32, timeout: u64) -> VkResult {
  unsafe {
    let fenceCount = pFences.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkWaitForFences.unwrap()(device.as_raw(), fenceCount, pFences.as_raw(), waitAll, timeout)
    })
  }
}

/// Create a new queue semaphore object
pub fn vkCreateSemaphore<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkSemaphoreCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSemaphore<'h>> {
  unsafe {
    let mut pSemaphore: VkSemaphore<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateSemaphore.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSemaphore).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSemaphore);
    })
  }
}

/// Destroy a semaphore object
pub fn vkDestroySemaphore<'h>(
  device: VkDevice<'h>,
  semaphore: Option<VkSemaphore<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySemaphore.unwrap()(device.as_raw(), semaphore.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new event object
pub fn vkCreateEvent<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkEventCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkEvent<'h>> {
  unsafe {
    let mut pEvent: VkEvent<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateEvent.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pEvent).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pEvent);
    })
  }
}

/// Destroy an event object
pub fn vkDestroyEvent<'h>(
  device: VkDevice<'h>,
  event: Option<VkEvent<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyEvent.unwrap()(device.as_raw(), event.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Retrieve the status of an event object
pub fn vkGetEventStatus<'h>(device: VkDevice<'h>, event: VkEvent<'h>) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetEventStatus.unwrap()(device.as_raw(), event.as_raw())
    })
  }
}

/// Set an event to signaled state
pub fn vkSetEvent<'h>(device: VkDevice<'h>, event: VkEvent<'h>) -> VkResult {
  unsafe { VkDeviceDispatchTable::with(device, |_t| _t.vkSetEvent.unwrap()(device.as_raw(), event.as_raw())) }
}

/// Reset an event to non-signaled state
pub fn vkResetEvent<'h>(device: VkDevice<'h>, event: VkEvent<'h>) -> VkResult {
  unsafe { VkDeviceDispatchTable::with(device, |_t| _t.vkResetEvent.unwrap()(device.as_raw(), event.as_raw())) }
}

/// Create a new query pool object
pub fn vkCreateQueryPool<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkQueryPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkQueryPool<'h>> {
  unsafe {
    let mut pQueryPool: VkQueryPool<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateQueryPool.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pQueryPool).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pQueryPool);
    })
  }
}

/// Destroy a query pool object
pub fn vkDestroyQueryPool<'h>(
  device: VkDevice<'h>,
  queryPool: Option<VkQueryPool<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyQueryPool.unwrap()(device.as_raw(), queryPool.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Copy results of queries in a query pool to a host memory region
pub fn vkGetQueryPoolResults<'h>(
  device: VkDevice<'h>,
  queryPool: VkQueryPool<'h>,
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
pub fn vkCreateBuffer<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkBufferCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkBuffer<'h>> {
  unsafe {
    let mut pBuffer: VkBuffer<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateBuffer.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pBuffer).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pBuffer);
    })
  }
}

/// Destroy a buffer object
pub fn vkDestroyBuffer<'h>(
  device: VkDevice<'h>,
  buffer: Option<VkBuffer<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyBuffer.unwrap()(device.as_raw(), buffer.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new buffer view object
pub fn vkCreateBufferView<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkBufferViewCreateInfo<'_, 'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkBufferView<'h>> {
  unsafe {
    let mut pView: VkBufferView<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateBufferView.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pView).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pView);
    })
  }
}

/// Destroy a buffer view object
pub fn vkDestroyBufferView<'h>(
  device: VkDevice<'h>,
  bufferView: Option<VkBufferView<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyBufferView.unwrap()(device.as_raw(), bufferView.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new image object
pub fn vkCreateImage<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkImageCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkImage<'h>> {
  unsafe {
    let mut pImage: VkImage<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateImage.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pImage).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pImage);
    })
  }
}

/// Destroy an image object
pub fn vkDestroyImage<'h>(
  device: VkDevice<'h>,
  image: Option<VkImage<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyImage.unwrap()(device.as_raw(), image.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Retrieve information about an image subresource
pub fn vkGetImageSubresourceLayout<'h>(
  device: VkDevice<'h>,
  image: VkImage<'h>,
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
pub fn vkCreateImageView<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkImageViewCreateInfo<'_, 'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkImageView<'h>> {
  unsafe {
    let mut pView: VkImageView<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateImageView.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pView).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pView);
    })
  }
}

/// Destroy an image view object
pub fn vkDestroyImageView<'h>(
  device: VkDevice<'h>,
  imageView: Option<VkImageView<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyImageView.unwrap()(device.as_raw(), imageView.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Creates a new shader module object
pub fn vkCreateShaderModule<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkShaderModuleCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkShaderModule<'h>> {
  unsafe {
    let mut pShaderModule: VkShaderModule<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateShaderModule.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pShaderModule).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pShaderModule);
    })
  }
}

/// Destroy a shader module module
pub fn vkDestroyShaderModule<'h>(
  device: VkDevice<'h>,
  shaderModule: Option<VkShaderModule<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyShaderModule.unwrap()(device.as_raw(), shaderModule.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Creates a new pipeline cache
pub fn vkCreatePipelineCache<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkPipelineCacheCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkPipelineCache<'h>> {
  unsafe {
    let mut pPipelineCache: VkPipelineCache<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreatePipelineCache.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pPipelineCache).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pPipelineCache);
    })
  }
}

/// Destroy a pipeline cache object
pub fn vkDestroyPipelineCache<'h>(
  device: VkDevice<'h>,
  pipelineCache: Option<VkPipelineCache<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyPipelineCache.unwrap()(device.as_raw(), pipelineCache.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Get the data store from a pipeline cache
pub fn vkGetPipelineCacheData<'h>(device: VkDevice<'h>, pipelineCache: VkPipelineCache<'h>) -> VkResult<Vec<u8>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pDataSize == 0 {
        return Ok(pData);
      }
      pData = Vec::with_capacity(pDataSize as usize);
      let _r = _t.vkGetPipelineCacheData.unwrap()(
        device.as_raw(),
        pipelineCache.as_raw(),
        &mut pDataSize,
        pData.as_mut_ptr().as_raw() as *mut c_void,
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pData.set_len(pDataSize as usize);
      return Ok(pData);
    })
  }
}

/// Combine the data stores of pipeline caches
pub fn vkMergePipelineCaches<'h>(
  device: VkDevice<'h>,
  dstCache: VkPipelineCache<'h>,
  pSrcCaches: &[VkPipelineCache<'h>],
) -> VkResult {
  unsafe {
    let srcCacheCount = pSrcCaches.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkMergePipelineCaches.unwrap()(device.as_raw(), dstCache.as_raw(), srcCacheCount, pSrcCaches.as_raw())
    })
  }
}

/// Create graphics pipelines
pub fn vkCreateGraphicsPipelines<'h>(
  device: VkDevice<'h>,
  pipelineCache: Option<VkPipelineCache<'h>>,
  pCreateInfos: &[VkGraphicsPipelineCreateInfo<'_, 'h>],
  pAllocator: Option<&VkAllocationCallbacks>,
  pPipelines: &mut [VkPipeline<'h>],
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
pub fn vkCreateComputePipelines<'h>(
  device: VkDevice<'h>,
  pipelineCache: Option<VkPipelineCache<'h>>,
  pCreateInfos: &[VkComputePipelineCreateInfo<'_, 'h>],
  pAllocator: Option<&VkAllocationCallbacks>,
  pPipelines: &mut [VkPipeline<'h>],
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
pub fn vkDestroyPipeline<'h>(
  device: VkDevice<'h>,
  pipeline: Option<VkPipeline<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyPipeline.unwrap()(device.as_raw(), pipeline.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Creates a new pipeline layout object
pub fn vkCreatePipelineLayout<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkPipelineLayoutCreateInfo<'_, 'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkPipelineLayout<'h>> {
  unsafe {
    let mut pPipelineLayout: VkPipelineLayout<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreatePipelineLayout.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pPipelineLayout).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pPipelineLayout);
    })
  }
}

/// Destroy a pipeline layout object
pub fn vkDestroyPipelineLayout<'h>(
  device: VkDevice<'h>,
  pipelineLayout: Option<VkPipelineLayout<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyPipelineLayout.unwrap()(device.as_raw(), pipelineLayout.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new sampler object
pub fn vkCreateSampler<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkSamplerCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSampler<'h>> {
  unsafe {
    let mut pSampler: VkSampler<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateSampler.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSampler).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSampler);
    })
  }
}

/// Destroy a sampler object
pub fn vkDestroySampler<'h>(
  device: VkDevice<'h>,
  sampler: Option<VkSampler<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySampler.unwrap()(device.as_raw(), sampler.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new descriptor set layout
pub fn vkCreateDescriptorSetLayout<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkDescriptorSetLayoutCreateInfo<'_, 'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkDescriptorSetLayout<'h>> {
  unsafe {
    let mut pSetLayout: VkDescriptorSetLayout<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateDescriptorSetLayout.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSetLayout).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSetLayout);
    })
  }
}

/// Destroy a descriptor set layout object
pub fn vkDestroyDescriptorSetLayout<'h>(
  device: VkDevice<'h>,
  descriptorSetLayout: Option<VkDescriptorSetLayout<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyDescriptorSetLayout.unwrap()(device.as_raw(), descriptorSetLayout.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Creates a descriptor pool object
pub fn vkCreateDescriptorPool<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkDescriptorPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkDescriptorPool<'h>> {
  unsafe {
    let mut pDescriptorPool: VkDescriptorPool<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateDescriptorPool.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pDescriptorPool).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pDescriptorPool);
    })
  }
}

/// Destroy a descriptor pool object
pub fn vkDestroyDescriptorPool<'h>(
  device: VkDevice<'h>,
  descriptorPool: Option<VkDescriptorPool<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyDescriptorPool.unwrap()(device.as_raw(), descriptorPool.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Resets a descriptor pool object
pub fn vkResetDescriptorPool<'h>(
  device: VkDevice<'h>,
  descriptorPool: VkDescriptorPool<'h>,
  flags: VkDescriptorPoolResetFlags,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetDescriptorPool.unwrap()(device.as_raw(), descriptorPool.as_raw(), flags)
    })
  }
}

/// Allocate one or more descriptor sets
pub fn vkAllocateDescriptorSets<'h>(
  device: VkDevice<'h>,
  pAllocateInfo: &VkDescriptorSetAllocateInfo<'_, 'h>,
) -> VkResult<Vec<VkDescriptorSet<'h>>> {
  unsafe {
    let mut pDescriptorSets: Vec<VkDescriptorSet<'h>> =
      Vec::with_capacity(pAllocateInfo.descriptor_set_count() as usize);
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAllocateDescriptorSets.unwrap()(
        device.as_raw(),
        pAllocateInfo.as_raw(),
        pDescriptorSets.as_mut_ptr().as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      pDescriptorSets.set_len(pAllocateInfo.descriptor_set_count() as usize);
      return Ok(pDescriptorSets);
    })
  }
}

/// Free one or more descriptor sets
pub fn vkFreeDescriptorSets<'h>(
  device: VkDevice<'h>,
  descriptorPool: VkDescriptorPool<'h>,
  pDescriptorSets: &[VkDescriptorSet<'h>],
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
pub fn vkUpdateDescriptorSets<'h>(
  device: VkDevice<'h>,
  pDescriptorWrites: &[VkWriteDescriptorSet<'_, 'h>],
  pDescriptorCopies: &[VkCopyDescriptorSet<'_, 'h>],
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
pub fn vkCreateFramebuffer<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkFramebufferCreateInfo<'_, 'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkFramebuffer<'h>> {
  unsafe {
    let mut pFramebuffer: VkFramebuffer<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateFramebuffer.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pFramebuffer).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pFramebuffer);
    })
  }
}

/// Destroy a framebuffer object
pub fn vkDestroyFramebuffer<'h>(
  device: VkDevice<'h>,
  framebuffer: Option<VkFramebuffer<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyFramebuffer.unwrap()(device.as_raw(), framebuffer.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Create a new render pass object
pub fn vkCreateRenderPass<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkRenderPassCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkRenderPass<'h>> {
  unsafe {
    let mut pRenderPass: VkRenderPass<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateRenderPass.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pRenderPass).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pRenderPass);
    })
  }
}

/// Destroy a render pass object
pub fn vkDestroyRenderPass<'h>(
  device: VkDevice<'h>,
  renderPass: Option<VkRenderPass<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyRenderPass.unwrap()(device.as_raw(), renderPass.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Returns the granularity for optimal render area
pub fn vkGetRenderAreaGranularity<'h>(device: VkDevice<'h>, renderPass: VkRenderPass<'h>) -> VkExtent2D {
  unsafe {
    let mut pGranularity: VkExtent2D = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetRenderAreaGranularity.unwrap()(device.as_raw(), renderPass.as_raw(), (&mut pGranularity).as_raw());
      return pGranularity;
    })
  }
}

/// Create a new command pool object
pub fn vkCreateCommandPool<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkCommandPoolCreateInfo,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkCommandPool<'h>> {
  unsafe {
    let mut pCommandPool: VkCommandPool<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateCommandPool.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pCommandPool).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pCommandPool);
    })
  }
}

/// Destroy a command pool object
pub fn vkDestroyCommandPool<'h>(
  device: VkDevice<'h>,
  commandPool: Option<VkCommandPool<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyCommandPool.unwrap()(device.as_raw(), commandPool.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Reset a command pool
pub fn vkResetCommandPool<'h>(
  device: VkDevice<'h>,
  commandPool: VkCommandPool<'h>,
  flags: VkCommandPoolResetFlags,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetCommandPool.unwrap()(device.as_raw(), commandPool.as_raw(), flags)
    })
  }
}

/// Allocate command buffers from an existing command pool
pub fn vkAllocateCommandBuffers<'h>(
  device: VkDevice<'h>,
  pAllocateInfo: &VkCommandBufferAllocateInfo<'_, 'h>,
) -> VkResult<Vec<VkCommandBuffer<'h>>> {
  unsafe {
    let mut pCommandBuffers: Vec<VkCommandBuffer<'h>> =
      Vec::with_capacity(pAllocateInfo.command_buffer_count() as usize);
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAllocateCommandBuffers.unwrap()(
        device.as_raw(),
        pAllocateInfo.as_raw(),
        pCommandBuffers.as_mut_ptr().as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      pCommandBuffers.set_len(pAllocateInfo.command_buffer_count() as usize);
      return Ok(pCommandBuffers);
    })
  }
}

/// Free command buffers
pub fn vkFreeCommandBuffers<'h>(
  device: VkDevice<'h>,
  commandPool: VkCommandPool<'h>,
  pCommandBuffers: &[VkCommandBuffer<'h>],
) {
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
pub fn vkBeginCommandBuffer<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pBeginInfo: &VkCommandBufferBeginInfo<'_, 'h>,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkBeginCommandBuffer.unwrap()(commandBuffer.as_raw(), pBeginInfo.as_raw())
    })
  }
}

/// Finish recording a command buffer
pub fn vkEndCommandBuffer<'h>(commandBuffer: VkCommandBuffer<'h>) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkEndCommandBuffer.unwrap()(commandBuffer.as_raw())
    })
  }
}

/// Reset a command buffer to the initial state
pub fn vkResetCommandBuffer<'h>(commandBuffer: VkCommandBuffer<'h>, flags: VkCommandBufferResetFlags) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkResetCommandBuffer.unwrap()(commandBuffer.as_raw(), flags)
    })
  }
}

/// Bind a pipeline object to a command buffer
pub fn vkCmdBindPipeline<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pipelineBindPoint: VkPipelineBindPoint,
  pipeline: VkPipeline<'h>,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBindPipeline.unwrap()(commandBuffer.as_raw(), pipelineBindPoint, pipeline.as_raw())
    })
  }
}

/// Set the viewport on a command buffer
pub fn vkCmdSetViewport<'h>(commandBuffer: VkCommandBuffer<'h>, firstViewport: u32, pViewports: &[VkViewport]) {
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
pub fn vkCmdSetScissor<'h>(commandBuffer: VkCommandBuffer<'h>, firstScissor: u32, pScissors: &[VkRect2D]) {
  unsafe {
    let scissorCount = pScissors.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetScissor.unwrap()(commandBuffer.as_raw(), firstScissor, scissorCount, pScissors.as_raw())
    })
  }
}

/// Set the dynamic line width state
pub fn vkCmdSetLineWidth<'h>(commandBuffer: VkCommandBuffer<'h>, lineWidth: f32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetLineWidth.unwrap()(commandBuffer.as_raw(), lineWidth)
    })
  }
}

/// Set the depth bias dynamic state
pub fn vkCmdSetDepthBias<'h>(
  commandBuffer: VkCommandBuffer<'h>,
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
pub fn vkCmdSetBlendConstants<'h>(commandBuffer: VkCommandBuffer<'h>, blendConstants: [f32; 4]) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetBlendConstants.unwrap()(commandBuffer.as_raw(), blendConstants)
    })
  }
}

/// Set the depth bounds test values for a command buffer
pub fn vkCmdSetDepthBounds<'h>(commandBuffer: VkCommandBuffer<'h>, minDepthBounds: f32, maxDepthBounds: f32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetDepthBounds.unwrap()(commandBuffer.as_raw(), minDepthBounds, maxDepthBounds)
    })
  }
}

/// Set the stencil compare mask dynamic state
pub fn vkCmdSetStencilCompareMask<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  faceMask: VkStencilFaceFlags,
  compareMask: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilCompareMask.unwrap()(commandBuffer.as_raw(), faceMask, compareMask)
    })
  }
}

/// Set the stencil write mask dynamic state
pub fn vkCmdSetStencilWriteMask<'h>(commandBuffer: VkCommandBuffer<'h>, faceMask: VkStencilFaceFlags, writeMask: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilWriteMask.unwrap()(commandBuffer.as_raw(), faceMask, writeMask)
    })
  }
}

/// Set the stencil reference dynamic state
pub fn vkCmdSetStencilReference<'h>(commandBuffer: VkCommandBuffer<'h>, faceMask: VkStencilFaceFlags, reference: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilReference.unwrap()(commandBuffer.as_raw(), faceMask, reference)
    })
  }
}

/// Binds descriptor sets to a command buffer
pub fn vkCmdBindDescriptorSets<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pipelineBindPoint: VkPipelineBindPoint,
  layout: VkPipelineLayout<'h>,
  firstSet: u32,
  pDescriptorSets: &[VkDescriptorSet<'h>],
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
pub fn vkCmdBindIndexBuffer<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  buffer: VkBuffer<'h>,
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
pub fn vkCmdBindVertexBuffers<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  firstBinding: u32,
  pBuffers: &[VkBuffer<'h>],
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
pub fn vkCmdDraw<'h>(
  commandBuffer: VkCommandBuffer<'h>,
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
pub fn vkCmdDrawIndexed<'h>(
  commandBuffer: VkCommandBuffer<'h>,
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
pub fn vkCmdDrawIndirect<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  buffer: VkBuffer<'h>,
  offset: VkDeviceSize,
  drawCount: u32,
  stride: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDrawIndirect.unwrap()(commandBuffer.as_raw(), buffer.as_raw(), offset, drawCount, stride)
    })
  }
}

/// Perform an indexed indirect draw
pub fn vkCmdDrawIndexedIndirect<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  buffer: VkBuffer<'h>,
  offset: VkDeviceSize,
  drawCount: u32,
  stride: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDrawIndexedIndirect.unwrap()(commandBuffer.as_raw(), buffer.as_raw(), offset, drawCount, stride)
    })
  }
}

/// Dispatch compute work items
pub fn vkCmdDispatch<'h>(commandBuffer: VkCommandBuffer<'h>, groupCountX: u32, groupCountY: u32, groupCountZ: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDispatch.unwrap()(commandBuffer.as_raw(), groupCountX, groupCountY, groupCountZ)
    })
  }
}

/// Dispatch compute work items using indirect parameters
pub fn vkCmdDispatchIndirect<'h>(commandBuffer: VkCommandBuffer<'h>, buffer: VkBuffer<'h>, offset: VkDeviceSize) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDispatchIndirect.unwrap()(commandBuffer.as_raw(), buffer.as_raw(), offset)
    })
  }
}

/// Copy data between buffer regions
pub fn vkCmdCopyBuffer<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  srcBuffer: VkBuffer<'h>,
  dstBuffer: VkBuffer<'h>,
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
pub fn vkCmdCopyImage<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  srcImage: VkImage<'h>,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage<'h>,
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
pub fn vkCmdBlitImage<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  srcImage: VkImage<'h>,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage<'h>,
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
pub fn vkCmdCopyBufferToImage<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  srcBuffer: VkBuffer<'h>,
  dstImage: VkImage<'h>,
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
pub fn vkCmdCopyImageToBuffer<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  srcImage: VkImage<'h>,
  srcImageLayout: VkImageLayout,
  dstBuffer: VkBuffer<'h>,
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

/// Update a buffer\'s contents from host memory
pub fn vkCmdUpdateBuffer<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  dstBuffer: VkBuffer<'h>,
  dstOffset: VkDeviceSize,
  pData: &[u8],
) {
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
pub fn vkCmdFillBuffer<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  dstBuffer: VkBuffer<'h>,
  dstOffset: VkDeviceSize,
  size: VkDeviceSize,
  data: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdFillBuffer.unwrap()(commandBuffer.as_raw(), dstBuffer.as_raw(), dstOffset, size, data)
    })
  }
}

/// Clear regions of a color image
pub fn vkCmdClearColorImage<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  image: VkImage<'h>,
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
pub fn vkCmdClearDepthStencilImage<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  image: VkImage<'h>,
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
pub fn vkCmdClearAttachments<'h>(
  commandBuffer: VkCommandBuffer<'h>,
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
pub fn vkCmdResolveImage<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  srcImage: VkImage<'h>,
  srcImageLayout: VkImageLayout,
  dstImage: VkImage<'h>,
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
pub fn vkCmdSetEvent<'h>(commandBuffer: VkCommandBuffer<'h>, event: VkEvent<'h>, stageMask: VkPipelineStageFlags) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetEvent.unwrap()(commandBuffer.as_raw(), event.as_raw(), stageMask)
    })
  }
}

/// Reset an event object to non-signaled state
pub fn vkCmdResetEvent<'h>(commandBuffer: VkCommandBuffer<'h>, event: VkEvent<'h>, stageMask: VkPipelineStageFlags) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdResetEvent.unwrap()(commandBuffer.as_raw(), event.as_raw(), stageMask)
    })
  }
}

/// Wait for one or more events and insert a set of memory
pub fn vkCmdWaitEvents<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pEvents: &[VkEvent<'h>],
  srcStageMask: VkPipelineStageFlags,
  dstStageMask: VkPipelineStageFlags,
  pMemoryBarriers: &[VkMemoryBarrier],
  pBufferMemoryBarriers: &[VkBufferMemoryBarrier<'_, 'h>],
  pImageMemoryBarriers: &[VkImageMemoryBarrier<'_, 'h>],
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
pub fn vkCmdPipelineBarrier<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  srcStageMask: VkPipelineStageFlags,
  dstStageMask: VkPipelineStageFlags,
  dependencyFlags: VkDependencyFlags,
  pMemoryBarriers: &[VkMemoryBarrier],
  pBufferMemoryBarriers: &[VkBufferMemoryBarrier<'_, 'h>],
  pImageMemoryBarriers: &[VkImageMemoryBarrier<'_, 'h>],
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
pub fn vkCmdBeginQuery<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  queryPool: VkQueryPool<'h>,
  query: u32,
  flags: VkQueryControlFlags,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBeginQuery.unwrap()(commandBuffer.as_raw(), queryPool.as_raw(), query, flags)
    })
  }
}

/// Ends a query
pub fn vkCmdEndQuery<'h>(commandBuffer: VkCommandBuffer<'h>, queryPool: VkQueryPool<'h>, query: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdEndQuery.unwrap()(commandBuffer.as_raw(), queryPool.as_raw(), query)
    })
  }
}

/// Reset queries in a query pool
pub fn vkCmdResetQueryPool<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  queryPool: VkQueryPool<'h>,
  firstQuery: u32,
  queryCount: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdResetQueryPool.unwrap()(commandBuffer.as_raw(), queryPool.as_raw(), firstQuery, queryCount)
    })
  }
}

/// Write a device timestamp into a query object
pub fn vkCmdWriteTimestamp<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pipelineStage: VkPipelineStageFlagBits,
  queryPool: VkQueryPool<'h>,
  query: u32,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdWriteTimestamp.unwrap()(commandBuffer.as_raw(), pipelineStage, queryPool.as_raw(), query)
    })
  }
}

/// Copy the results of queries in a query pool to a buffer object
pub fn vkCmdCopyQueryPoolResults<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  queryPool: VkQueryPool<'h>,
  firstQuery: u32,
  queryCount: u32,
  dstBuffer: VkBuffer<'h>,
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
pub fn vkCmdPushConstants<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  layout: VkPipelineLayout<'h>,
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
pub fn vkCmdBeginRenderPass<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pRenderPassBegin: &VkRenderPassBeginInfo<'_, 'h>,
  contents: VkSubpassContents,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBeginRenderPass.unwrap()(commandBuffer.as_raw(), pRenderPassBegin.as_raw(), contents)
    })
  }
}

/// Transition to the next subpass of a render pass
pub fn vkCmdNextSubpass<'h>(commandBuffer: VkCommandBuffer<'h>, contents: VkSubpassContents) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdNextSubpass.unwrap()(commandBuffer.as_raw(), contents)
    })
  }
}

/// End the current render pass
pub fn vkCmdEndRenderPass<'h>(commandBuffer: VkCommandBuffer<'h>) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdEndRenderPass.unwrap()(commandBuffer.as_raw())
    })
  }
}

/// Execute a secondary command buffer from a primary command buffer
pub fn vkCmdExecuteCommands<'h>(commandBuffer: VkCommandBuffer<'h>, pCommandBuffers: &[VkCommandBuffer<'h>]) {
  unsafe {
    let commandBufferCount = pCommandBuffers.len() as u32;
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdExecuteCommands.unwrap()(commandBuffer.as_raw(), commandBufferCount, pCommandBuffers.as_raw())
    })
  }
}

// feature: VK_KHR_surface

/// Destroy a VkSurfaceKHR object
#[cfg(feature = "VK_KHR_surface")]
pub fn vkDestroySurfaceKHR<'h>(
  instance: VkInstance<'h>,
  surface: Option<VkSurfaceKHR<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkInstanceDispatchTable::with(instance, |_t| {
      _t.vkDestroySurfaceKHR.unwrap()(instance.as_raw(), surface.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Query if presentation is supported
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfaceSupportKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  queueFamilyIndex: u32,
  surface: VkSurfaceKHR<'h>,
) -> VkResult<bool> {
  unsafe {
    let mut pSupported: VkBool32 = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceSurfaceSupportKHR.unwrap()(
        physicalDevice.as_raw(),
        queueFamilyIndex,
        surface.as_raw(),
        &mut pSupported,
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSupported != 0);
    })
  }
}

/// Query surface capabilities
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  surface: VkSurfaceKHR<'h>,
) -> VkResult<VkSurfaceCapabilitiesKHR> {
  unsafe {
    let mut pSurfaceCapabilities: VkSurfaceCapabilitiesKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceSurfaceCapabilitiesKHR.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        (&mut pSurfaceCapabilities).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurfaceCapabilities);
    })
  }
}

/// Query color formats supported by surface
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfaceFormatsKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  surface: VkSurfaceKHR<'h>,
) -> VkResult<Vec<VkSurfaceFormatKHR>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pSurfaceFormatCount == 0 {
        return Ok(pSurfaceFormats);
      }
      pSurfaceFormats = Vec::with_capacity(pSurfaceFormatCount as usize);
      let _r = _t.vkGetPhysicalDeviceSurfaceFormatsKHR.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pSurfaceFormatCount,
        pSurfaceFormats.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pSurfaceFormats.set_len(pSurfaceFormatCount as usize);
      return Ok(pSurfaceFormats);
    })
  }
}

/// Query supported presentation modes
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfacePresentModesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  surface: VkSurfaceKHR<'h>,
) -> VkResult<Vec<VkPresentModeKHR>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPresentModeCount == 0 {
        return Ok(pPresentModes);
      }
      pPresentModes = Vec::with_capacity(pPresentModeCount as usize);
      let _r = _t.vkGetPhysicalDeviceSurfacePresentModesKHR.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pPresentModeCount,
        pPresentModes.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pPresentModes.set_len(pPresentModeCount as usize);
      return Ok(pPresentModes);
    })
  }
}

// feature: VK_KHR_swapchain

/// Create a swapchain
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkCreateSwapchainKHR<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkSwapchainCreateInfoKHR<'_, 'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSwapchainKHR<'h>> {
  unsafe {
    let mut pSwapchain: VkSwapchainKHR<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateSwapchainKHR.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSwapchain).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSwapchain);
    })
  }
}

/// Destroy a swapchain object
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkDestroySwapchainKHR<'h>(
  device: VkDevice<'h>,
  swapchain: Option<VkSwapchainKHR<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySwapchainKHR.unwrap()(device.as_raw(), swapchain.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Obtain the array of presentable images associated with a swapchain
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkGetSwapchainImagesKHR<'h>(device: VkDevice<'h>, swapchain: VkSwapchainKHR<'h>) -> VkResult<Vec<VkImage<'h>>> {
  unsafe {
    let mut pSwapchainImageCount: u32 = 0;
    let mut pSwapchainImages: Vec<VkImage<'h>> = Vec::new();
    VkDeviceDispatchTable::with(device, |_t| loop {
      let _r = _t.vkGetSwapchainImagesKHR.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        &mut pSwapchainImageCount,
        ::std::ptr::null_mut(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pSwapchainImageCount == 0 {
        return Ok(pSwapchainImages);
      }
      pSwapchainImages = Vec::with_capacity(pSwapchainImageCount as usize);
      let _r = _t.vkGetSwapchainImagesKHR.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        &mut pSwapchainImageCount,
        pSwapchainImages.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pSwapchainImages.set_len(pSwapchainImageCount as usize);
      return Ok(pSwapchainImages);
    })
  }
}

/// Retrieve the index of the next available presentable image
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkAcquireNextImageKHR<'h>(
  device: VkDevice<'h>,
  swapchain: VkSwapchainKHR<'h>,
  timeout: u64,
  semaphore: Option<VkSemaphore<'h>>,
  fence: Option<VkFence<'h>>,
) -> VkResult<u32> {
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
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pImageIndex);
    })
  }
}

/// Queue an image for presentation
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkQueuePresentKHR<'h>(queue: VkQueue<'h>, pPresentInfo: &VkPresentInfoKHR<'_, 'h>) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(queue, |_t| {
      _t.vkQueuePresentKHR.unwrap()(queue.as_raw(), pPresentInfo.as_raw())
    })
  }
}

// feature: VK_KHR_display

/// Query information about the available displays
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetPhysicalDeviceDisplayPropertiesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
) -> VkResult<Vec<VkDisplayPropertiesKHR<'_, 'h>>> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkDisplayPropertiesKHR<'_, 'h>> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetPhysicalDeviceDisplayPropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkGetPhysicalDeviceDisplayPropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Query the plane properties
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
) -> VkResult<Vec<VkDisplayPlanePropertiesKHR<'h>>> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkDisplayPlanePropertiesKHR<'h>> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetPhysicalDeviceDisplayPlanePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkGetPhysicalDeviceDisplayPlanePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Query the list of displays a plane supports
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetDisplayPlaneSupportedDisplaysKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  planeIndex: u32,
) -> VkResult<Vec<VkDisplayKHR<'h>>> {
  unsafe {
    let mut pDisplayCount: u32 = 0;
    let mut pDisplays: Vec<VkDisplayKHR<'h>> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetDisplayPlaneSupportedDisplaysKHR.unwrap()(
        physicalDevice.as_raw(),
        planeIndex,
        &mut pDisplayCount,
        ::std::ptr::null_mut(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pDisplayCount == 0 {
        return Ok(pDisplays);
      }
      pDisplays = Vec::with_capacity(pDisplayCount as usize);
      let _r = _t.vkGetDisplayPlaneSupportedDisplaysKHR.unwrap()(
        physicalDevice.as_raw(),
        planeIndex,
        &mut pDisplayCount,
        pDisplays.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pDisplays.set_len(pDisplayCount as usize);
      return Ok(pDisplays);
    })
  }
}

/// Query the set of mode properties supported by the display
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetDisplayModePropertiesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  display: VkDisplayKHR<'h>,
) -> VkResult<Vec<VkDisplayModePropertiesKHR<'h>>> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkDisplayModePropertiesKHR<'h>> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| loop {
      let _r = _t.vkGetDisplayModePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        display.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPropertyCount == 0 {
        return Ok(pProperties);
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      let _r = _t.vkGetDisplayModePropertiesKHR.unwrap()(
        physicalDevice.as_raw(),
        display.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pProperties.set_len(pPropertyCount as usize);
      return Ok(pProperties);
    })
  }
}

/// Create a display mode
#[cfg(feature = "VK_KHR_display")]
pub fn vkCreateDisplayModeKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  display: VkDisplayKHR<'h>,
  pCreateInfo: &VkDisplayModeCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkDisplayModeKHR<'h>> {
  unsafe {
    let mut pMode: VkDisplayModeKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkCreateDisplayModeKHR.unwrap()(
        physicalDevice.as_raw(),
        display.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pMode).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pMode);
    })
  }
}

/// Query capabilities of a mode and plane combination
#[cfg(feature = "VK_KHR_display")]
pub fn vkGetDisplayPlaneCapabilitiesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  mode: VkDisplayModeKHR<'h>,
  planeIndex: u32,
) -> VkResult<VkDisplayPlaneCapabilitiesKHR> {
  unsafe {
    let mut pCapabilities: VkDisplayPlaneCapabilitiesKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetDisplayPlaneCapabilitiesKHR.unwrap()(
        physicalDevice.as_raw(),
        mode.as_raw(),
        planeIndex,
        (&mut pCapabilities).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pCapabilities);
    })
  }
}

/// Create a `VkSurfaceKHR` structure representing a display plane and mode
#[cfg(feature = "VK_KHR_display")]
pub fn vkCreateDisplayPlaneSurfaceKHR<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkDisplaySurfaceCreateInfoKHR<'_, 'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateDisplayPlaneSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_KHR_display_swapchain

/// Create multiple swapchains that share presentable images
#[cfg(feature = "VK_KHR_display_swapchain")]
pub fn vkCreateSharedSwapchainsKHR<'h>(
  device: VkDevice<'h>,
  pCreateInfos: &[VkSwapchainCreateInfoKHR<'_, 'h>],
  pAllocator: Option<&VkAllocationCallbacks>,
  pSwapchains: &mut [VkSwapchainKHR<'h>],
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

/// Create a `VkSurfaceKHR` object for an X11 window, using the Xlib client-side
/// library
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub fn vkCreateXlibSurfaceKHR<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkXlibSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateXlibSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

/// Query physical device for presentation to X11 server using Xlib
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  queueFamilyIndex: u32,
  dpy: *mut wsi::xlib::Display,
  visualID: wsi::xlib::VisualID,
) -> bool {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceXlibPresentationSupportKHR.unwrap()(
        physicalDevice.as_raw(),
        queueFamilyIndex,
        dpy,
        visualID,
      )
    }) != 0
  }
}

// feature: VK_KHR_xcb_surface

/// Create a `VkSurfaceKHR` object for a X11 window, using the XCB client-side
/// library
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub fn vkCreateXcbSurfaceKHR<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkXcbSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateXcbSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

/// Query physical device for presentation to X11 server using XCB
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  queueFamilyIndex: u32,
  connection: *mut wsi::xcb::xcb_connection_t,
  visual_id: wsi::xcb::xcb_visualid_t,
) -> bool {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceXcbPresentationSupportKHR.unwrap()(
        physicalDevice.as_raw(),
        queueFamilyIndex,
        connection,
        visual_id,
      )
    }) != 0
  }
}

// feature: VK_KHR_wayland_surface

/// Create a `VkSurfaceKHR` object for a Wayland window
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub fn vkCreateWaylandSurfaceKHR<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkWaylandSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateWaylandSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

/// Query physical device for presentation to Wayland
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  queueFamilyIndex: u32,
  display: *mut wsi::wayland::wl_display,
) -> bool {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceWaylandPresentationSupportKHR.unwrap()(physicalDevice.as_raw(), queueFamilyIndex, display)
    }) != 0
  }
}

// feature: VK_KHR_mir_surface

/// Create a `VkSurfaceKHR` object for a Mir window
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub fn vkCreateMirSurfaceKHR<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkMirSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateMirSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

/// Query physical device for presentation to Mir
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub fn vkGetPhysicalDeviceMirPresentationSupportKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  queueFamilyIndex: u32,
  connection: *mut wsi::mir::MirConnection,
) -> bool {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceMirPresentationSupportKHR.unwrap()(physicalDevice.as_raw(), queueFamilyIndex, connection)
    }) != 0
  }
}

// feature: VK_KHR_android_surface

/// Create a `VkSurfaceKHR` object for an Android native window
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub fn vkCreateAndroidSurfaceKHR<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkAndroidSurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateAndroidSurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_KHR_win32_surface

/// Create a `VkSurfaceKHR` object for an Win32 native window
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkCreateWin32SurfaceKHR<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkWin32SurfaceCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateWin32SurfaceKHR.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

/// query queue family support for presentation on a Win32 display
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  queueFamilyIndex: u32,
) -> bool {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceWin32PresentationSupportKHR.unwrap()(physicalDevice.as_raw(), queueFamilyIndex)
    }) != 0
  }
}

// feature: VK_EXT_debug_report

/// Create a debug report callback object
#[cfg(feature = "VK_EXT_debug_report")]
pub fn vkCreateDebugReportCallbackEXT<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkDebugReportCallbackCreateInfoEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkDebugReportCallbackEXT<'h>> {
  unsafe {
    let mut pCallback: VkDebugReportCallbackEXT<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateDebugReportCallbackEXT.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pCallback).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pCallback);
    })
  }
}

/// Destroy a debug report callback object
#[cfg(feature = "VK_EXT_debug_report")]
pub fn vkDestroyDebugReportCallbackEXT<'h>(
  instance: VkInstance<'h>,
  callback: VkDebugReportCallbackEXT<'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkInstanceDispatchTable::with(instance, |_t| {
      _t.vkDestroyDebugReportCallbackEXT.unwrap()(instance.as_raw(), callback.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Inject a message into a debug stream
#[cfg(feature = "VK_EXT_debug_report")]
pub fn vkDebugReportMessageEXT<'h>(
  instance: VkInstance<'h>,
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
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkDebugMarkerSetObjectTagEXT<'h>(device: VkDevice<'h>, pTagInfo: &VkDebugMarkerObjectTagInfoEXT) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDebugMarkerSetObjectTagEXT.unwrap()(device.as_raw(), pTagInfo.as_raw())
    })
  }
}

/// Give a user-friendly name to an object
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkDebugMarkerSetObjectNameEXT<'h>(device: VkDevice<'h>, pNameInfo: &VkDebugMarkerObjectNameInfoEXT) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDebugMarkerSetObjectNameEXT.unwrap()(device.as_raw(), pNameInfo.as_raw())
    })
  }
}

/// Open a command buffer marker region
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerBeginEXT<'h>(commandBuffer: VkCommandBuffer<'h>, pMarkerInfo: &VkDebugMarkerMarkerInfoEXT) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerBeginEXT.unwrap()(commandBuffer.as_raw(), pMarkerInfo.as_raw())
    })
  }
}

/// Close a command buffer marker region
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerEndEXT<'h>(commandBuffer: VkCommandBuffer<'h>) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerEndEXT.unwrap()(commandBuffer.as_raw())
    })
  }
}

/// Insert a marker label into a command buffer
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerInsertEXT<'h>(commandBuffer: VkCommandBuffer<'h>, pMarkerInfo: &VkDebugMarkerMarkerInfoEXT) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerInsertEXT.unwrap()(commandBuffer.as_raw(), pMarkerInfo.as_raw())
    })
  }
}

// feature: VK_AMD_draw_indirect_count

/// Perform an indirect draw with the draw count sourced from a buffer
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub fn vkCmdDrawIndirectCountAMD<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  buffer: VkBuffer<'h>,
  offset: VkDeviceSize,
  countBuffer: VkBuffer<'h>,
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
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub fn vkCmdDrawIndexedIndirectCountAMD<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  buffer: VkBuffer<'h>,
  offset: VkDeviceSize,
  countBuffer: VkBuffer<'h>,
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
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceFeatures2KHR<'h>(physicalDevice: VkPhysicalDevice<'h>) -> VkPhysicalDeviceFeatures2KHR {
  unsafe {
    let mut pFeatures: VkPhysicalDeviceFeatures2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceFeatures2KHR.unwrap()(physicalDevice.as_raw(), (&mut pFeatures).as_raw());
      return pFeatures;
    })
  }
}

/// Returns properties of a physical device
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceProperties2KHR<'h>(physicalDevice: VkPhysicalDevice<'h>) -> VkPhysicalDeviceProperties2KHR {
  unsafe {
    let mut pProperties: VkPhysicalDeviceProperties2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceProperties2KHR.unwrap()(physicalDevice.as_raw(), (&mut pProperties).as_raw());
      return pProperties;
    })
  }
}

/// Lists physical device\'s format capabilities
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceFormatProperties2KHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
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

/// Lists physical device\'s image format capabilities
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceImageFormatProperties2KHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  pImageFormatInfo: &VkPhysicalDeviceImageFormatInfo2KHR,
) -> VkResult<VkImageFormatProperties2KHR> {
  unsafe {
    let mut pImageFormatProperties: VkImageFormatProperties2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceImageFormatProperties2KHR.unwrap()(
        physicalDevice.as_raw(),
        pImageFormatInfo.as_raw(),
        (&mut pImageFormatProperties).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pImageFormatProperties);
    })
  }
}

/// Reports properties of the queues of the specified physical device
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
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
        pQueueFamilyProperties.as_mut_ptr().as_raw(),
      );
      pQueueFamilyProperties.set_len(pQueueFamilyPropertyCount as usize);
      return pQueueFamilyProperties;
    })
  }
}

/// Reports memory information for the specified physical device
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceMemoryProperties2KHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
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
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  pFormatInfo: &VkPhysicalDeviceSparseImageFormatInfo2KHR,
) -> Vec<VkSparseImageFormatProperties2KHR> {
  unsafe {
    let mut pPropertyCount: u32 = 0;
    let mut pProperties: Vec<VkSparseImageFormatProperties2KHR> = Vec::new();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceSparseImageFormatProperties2KHR.unwrap()(
        physicalDevice.as_raw(),
        pFormatInfo.as_raw(),
        &mut pPropertyCount,
        ::std::ptr::null_mut(),
      );
      if pPropertyCount == 0 {
        return pProperties;
      }
      pProperties = Vec::with_capacity(pPropertyCount as usize);
      _t.vkGetPhysicalDeviceSparseImageFormatProperties2KHR.unwrap()(
        physicalDevice.as_raw(),
        pFormatInfo.as_raw(),
        &mut pPropertyCount,
        pProperties.as_mut_ptr().as_raw(),
      );
      pProperties.set_len(pPropertyCount as usize);
      return pProperties;
    })
  }
}

// feature: VK_AMD_shader_info

/// Get information about a shader in a pipeline
#[cfg(feature = "VK_AMD_shader_info")]
pub fn vkGetShaderInfoAMD<'h>(
  device: VkDevice<'h>,
  pipeline: VkPipeline<'h>,
  shaderStage: VkShaderStageFlagBits,
  infoType: VkShaderInfoTypeAMD,
) -> VkResult<Vec<u8>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
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
        pInfo.as_mut_ptr().as_raw() as *mut c_void,
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pInfo.set_len(pInfoSize as usize);
      return Ok(pInfo);
    })
  }
}

// feature: VK_NV_external_memory_capabilities

/// determine image capabilities compatible with external memory handle types
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  format: VkFormat,
  eType: VkImageType,
  tiling: VkImageTiling,
  usage: VkImageUsageFlags,
  flags: VkImageCreateFlags,
  externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
) -> VkResult<VkExternalImageFormatPropertiesNV> {
  unsafe {
    let mut pExternalImageFormatProperties: VkExternalImageFormatPropertiesNV = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceExternalImageFormatPropertiesNV.unwrap()(
        physicalDevice.as_raw(),
        format,
        eType,
        tiling,
        usage,
        flags,
        externalHandleType,
        (&mut pExternalImageFormatProperties).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pExternalImageFormatProperties);
    })
  }
}

// feature: VK_NV_external_memory_win32

/// retrieve Win32 handle to a device memory object
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetMemoryWin32HandleNV<'h>(
  device: VkDevice<'h>,
  memory: VkDeviceMemory<'h>,
  handleType: VkExternalMemoryHandleTypeFlagsNV,
) -> VkResult<wsi::win32::HANDLE> {
  unsafe {
    let mut pHandle: wsi::win32::HANDLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryWin32HandleNV.unwrap()(device.as_raw(), memory.as_raw(), handleType, &mut pHandle);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pHandle);
    })
  }
}

// feature: VK_KHX_device_group_creation

/// Enumerates groups of physical devices that can be used to create a single
/// logical device
#[cfg(feature = "VK_KHX_device_group_creation")]
pub fn vkEnumeratePhysicalDeviceGroupsKHX<'h>(
  instance: VkInstance<'h>,
) -> VkResult<Vec<VkPhysicalDeviceGroupPropertiesKHX<'h>>> {
  unsafe {
    let mut pPhysicalDeviceGroupCount: u32 = 0;
    let mut pPhysicalDeviceGroupProperties: Vec<VkPhysicalDeviceGroupPropertiesKHX<'h>> = Vec::new();
    VkInstanceDispatchTable::with(instance, |_t| loop {
      let _r = _t.vkEnumeratePhysicalDeviceGroupsKHX.unwrap()(
        instance.as_raw(),
        &mut pPhysicalDeviceGroupCount,
        ::std::ptr::null_mut(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPhysicalDeviceGroupCount == 0 {
        return Ok(pPhysicalDeviceGroupProperties);
      }
      pPhysicalDeviceGroupProperties = Vec::with_capacity(pPhysicalDeviceGroupCount as usize);
      let _r = _t.vkEnumeratePhysicalDeviceGroupsKHX.unwrap()(
        instance.as_raw(),
        &mut pPhysicalDeviceGroupCount,
        pPhysicalDeviceGroupProperties.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pPhysicalDeviceGroupProperties.set_len(pPhysicalDeviceGroupCount as usize);
      return Ok(pPhysicalDeviceGroupProperties);
    })
  }
}

// feature: VK_KHX_device_group

/// Query supported peer memory features of a device
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkGetDeviceGroupPeerMemoryFeaturesKHX<'h>(
  device: VkDevice<'h>,
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
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkCmdSetDeviceMaskKHX<'h>(commandBuffer: VkCommandBuffer<'h>, deviceMask: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetDeviceMaskKHX.unwrap()(commandBuffer.as_raw(), deviceMask)
    })
  }
}

/// Dispatch compute work items
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkCmdDispatchBaseKHX<'h>(
  commandBuffer: VkCommandBuffer<'h>,
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
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkGetDeviceGroupPresentCapabilitiesKHX<'h>(
  device: VkDevice<'h>,
) -> VkResult<VkDeviceGroupPresentCapabilitiesKHX> {
  unsafe {
    let mut pDeviceGroupPresentCapabilities: VkDeviceGroupPresentCapabilitiesKHX = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetDeviceGroupPresentCapabilitiesKHX.unwrap()(
        device.as_raw(),
        (&mut pDeviceGroupPresentCapabilities).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pDeviceGroupPresentCapabilities);
    })
  }
}

/// Query present capabilities for a surface
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkGetDeviceGroupSurfacePresentModesKHX<'h>(
  device: VkDevice<'h>,
  surface: VkSurfaceKHR<'h>,
) -> VkResult<VkDeviceGroupPresentModeFlagsKHX> {
  unsafe {
    let mut pModes: VkDeviceGroupPresentModeFlagsKHX = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetDeviceGroupSurfacePresentModesKHX.unwrap()(device.as_raw(), surface.as_raw(), &mut pModes);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pModes);
    })
  }
}

/// Query present rectangles for a surface on a physical device
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkGetPhysicalDevicePresentRectanglesKHX<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  surface: VkSurfaceKHR<'h>,
) -> VkResult<Vec<VkRect2D>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pRectCount == 0 {
        return Ok(pRects);
      }
      pRects = Vec::with_capacity(pRectCount as usize);
      let _r = _t.vkGetPhysicalDevicePresentRectanglesKHX.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        &mut pRectCount,
        pRects.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pRects.set_len(pRectCount as usize);
      return Ok(pRects);
    })
  }
}

/// Retrieve the index of the next available presentable image
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkAcquireNextImage2KHX<'h>(
  device: VkDevice<'h>,
  pAcquireInfo: &VkAcquireNextImageInfoKHX<'_, 'h>,
) -> VkResult<u32> {
  unsafe {
    let mut pImageIndex: u32 = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAcquireNextImage2KHX.unwrap()(device.as_raw(), pAcquireInfo.as_raw(), &mut pImageIndex);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pImageIndex);
    })
  }
}

// feature: VK_NN_vi_surface

/// Create a `VkSurfaceKHR` object for a VI layer
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub fn vkCreateViSurfaceNN<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkViSurfaceCreateInfoNN,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateViSurfaceNN.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_KHR_maintenance1

/// Trim a command pool
#[cfg(feature = "VK_KHR_maintenance1")]
pub fn vkTrimCommandPoolKHR<'h>(
  device: VkDevice<'h>,
  commandPool: VkCommandPool<'h>,
  flags: VkCommandPoolTrimFlagsKHR,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkTrimCommandPoolKHR.unwrap()(device.as_raw(), commandPool.as_raw(), flags)
    })
  }
}

// feature: VK_KHR_external_memory_capabilities

/// Query external handle types supported by buffers
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
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
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetMemoryWin32HandleKHR<'h>(
  device: VkDevice<'h>,
  pGetWin32HandleInfo: &VkMemoryGetWin32HandleInfoKHR<'_, 'h>,
) -> VkResult<wsi::win32::HANDLE> {
  unsafe {
    let mut pHandle: wsi::win32::HANDLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryWin32HandleKHR.unwrap()(device.as_raw(), pGetWin32HandleInfo.as_raw(), &mut pHandle);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pHandle);
    })
  }
}

/// Get Properties of External Memory Win32 Handles
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetMemoryWin32HandlePropertiesKHR<'h>(
  device: VkDevice<'h>,
  handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  handle: wsi::win32::HANDLE,
) -> VkResult<VkMemoryWin32HandlePropertiesKHR> {
  unsafe {
    let mut pMemoryWin32HandleProperties: VkMemoryWin32HandlePropertiesKHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryWin32HandlePropertiesKHR.unwrap()(
        device.as_raw(),
        handleType,
        handle,
        (&mut pMemoryWin32HandleProperties).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pMemoryWin32HandleProperties);
    })
  }
}

// feature: VK_KHR_external_memory_fd

/// Get a POSIX file descriptor for a memory object
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub fn vkGetMemoryFdKHR<'h>(device: VkDevice<'h>, pGetFdInfo: &VkMemoryGetFdInfoKHR<'_, 'h>) -> VkResult<c_int> {
  unsafe {
    let mut pFd: c_int = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryFdKHR.unwrap()(device.as_raw(), pGetFdInfo.as_raw(), &mut pFd);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pFd);
    })
  }
}

/// Get Properties of External Memory File Descriptors
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub fn vkGetMemoryFdPropertiesKHR<'h>(
  device: VkDevice<'h>,
  handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  fd: c_int,
) -> VkResult<VkMemoryFdPropertiesKHR> {
  unsafe {
    let mut pMemoryFdProperties: VkMemoryFdPropertiesKHR = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r =
        _t.vkGetMemoryFdPropertiesKHR.unwrap()(device.as_raw(), handleType, fd, (&mut pMemoryFdProperties).as_raw());
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pMemoryFdProperties);
    })
  }
}

// feature: VK_KHR_external_semaphore_capabilities

/// Function for querying external semaphore handle capabilities.
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  pExternalSemaphoreInfo: &VkPhysicalDeviceExternalSemaphoreInfoKHR,
) -> VkExternalSemaphorePropertiesKHR {
  unsafe {
    let mut pExternalSemaphoreProperties: VkExternalSemaphorePropertiesKHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.unwrap()(
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
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkImportSemaphoreWin32HandleKHR<'h>(
  device: VkDevice<'h>,
  pImportSemaphoreWin32HandleInfo: &VkImportSemaphoreWin32HandleInfoKHR<'_, 'h>,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportSemaphoreWin32HandleKHR.unwrap()(device.as_raw(), pImportSemaphoreWin32HandleInfo.as_raw())
    })
  }
}

/// Get a Windows HANDLE for a semaphore
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetSemaphoreWin32HandleKHR<'h>(
  device: VkDevice<'h>,
  pGetWin32HandleInfo: &VkSemaphoreGetWin32HandleInfoKHR<'_, 'h>,
) -> VkResult<wsi::win32::HANDLE> {
  unsafe {
    let mut pHandle: wsi::win32::HANDLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetSemaphoreWin32HandleKHR.unwrap()(device.as_raw(), pGetWin32HandleInfo.as_raw(), &mut pHandle);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pHandle);
    })
  }
}

// feature: VK_KHR_external_semaphore_fd

/// Import a semaphore from a POSIX file descriptor
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub fn vkImportSemaphoreFdKHR<'h>(
  device: VkDevice<'h>,
  pImportSemaphoreFdInfo: &VkImportSemaphoreFdInfoKHR<'_, 'h>,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportSemaphoreFdKHR.unwrap()(device.as_raw(), pImportSemaphoreFdInfo.as_raw())
    })
  }
}

/// Get a POSIX file descriptor handle for a semaphore
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub fn vkGetSemaphoreFdKHR<'h>(device: VkDevice<'h>, pGetFdInfo: &VkSemaphoreGetFdInfoKHR<'_, 'h>) -> VkResult<c_int> {
  unsafe {
    let mut pFd: c_int = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetSemaphoreFdKHR.unwrap()(device.as_raw(), pGetFdInfo.as_raw(), &mut pFd);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pFd);
    })
  }
}

// feature: VK_KHR_push_descriptor

/// Pushes descriptor updates into a command buffer
#[cfg(feature = "VK_KHR_push_descriptor")]
pub fn vkCmdPushDescriptorSetKHR<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pipelineBindPoint: VkPipelineBindPoint,
  layout: VkPipelineLayout<'h>,
  set: u32,
  pDescriptorWrites: &[VkWriteDescriptorSet<'_, 'h>],
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
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub fn vkCreateDescriptorUpdateTemplateKHR<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkDescriptorUpdateTemplateCreateInfoKHR<'_, 'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkDescriptorUpdateTemplateKHR<'h>> {
  unsafe {
    let mut pDescriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateDescriptorUpdateTemplateKHR.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pDescriptorUpdateTemplate).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pDescriptorUpdateTemplate);
    })
  }
}

/// Destroy a descriptor update template object
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub fn vkDestroyDescriptorUpdateTemplateKHR<'h>(
  device: VkDevice<'h>,
  descriptorUpdateTemplate: Option<VkDescriptorUpdateTemplateKHR<'h>>,
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
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub fn vkUpdateDescriptorSetWithTemplateKHR<'h>(
  device: VkDevice<'h>,
  descriptorSet: VkDescriptorSet<'h>,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR<'h>,
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
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub fn vkCmdPushDescriptorSetWithTemplateKHR<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR<'h>,
  layout: VkPipelineLayout<'h>,
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
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCmdProcessCommandsNVX<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pProcessCommandsInfo: &VkCmdProcessCommandsInfoNVX<'_, 'h>,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdProcessCommandsNVX.unwrap()(commandBuffer.as_raw(), pProcessCommandsInfo.as_raw())
    })
  }
}

/// Perform a reservation of command buffer space
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCmdReserveSpaceForCommandsNVX<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pReserveSpaceInfo: &VkCmdReserveSpaceForCommandsInfoNVX<'_, 'h>,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdReserveSpaceForCommandsNVX.unwrap()(commandBuffer.as_raw(), pReserveSpaceInfo.as_raw())
    })
  }
}

/// Create an indirect command layout object
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCreateIndirectCommandsLayoutNVX<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkIndirectCommandsLayoutCreateInfoNVX,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkIndirectCommandsLayoutNVX<'h>> {
  unsafe {
    let mut pIndirectCommandsLayout: VkIndirectCommandsLayoutNVX<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateIndirectCommandsLayoutNVX.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pIndirectCommandsLayout).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pIndirectCommandsLayout);
    })
  }
}

/// Destroy a object table
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkDestroyIndirectCommandsLayoutNVX<'h>(
  device: VkDevice<'h>,
  indirectCommandsLayout: VkIndirectCommandsLayoutNVX<'h>,
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
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCreateObjectTableNVX<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkObjectTableCreateInfoNVX,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkObjectTableNVX<'h>> {
  unsafe {
    let mut pObjectTable: VkObjectTableNVX<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateObjectTableNVX.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pObjectTable).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pObjectTable);
    })
  }
}

/// Destroy a object table
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkDestroyObjectTableNVX<'h>(
  device: VkDevice<'h>,
  objectTable: VkObjectTableNVX<'h>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyObjectTableNVX.unwrap()(device.as_raw(), objectTable.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Register resource bindings in an object table
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkRegisterObjectsNVX<'h>(
  device: VkDevice<'h>,
  objectTable: VkObjectTableNVX<'h>,
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
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkUnregisterObjectsNVX<'h>(
  device: VkDevice<'h>,
  objectTable: VkObjectTableNVX<'h>,
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
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  pFeatures: &mut VkDeviceGeneratedCommandsFeaturesNVX,
) -> VkDeviceGeneratedCommandsLimitsNVX {
  unsafe {
    let mut pLimits: VkDeviceGeneratedCommandsLimitsNVX = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX.unwrap()(
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
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub fn vkCmdSetViewportWScalingNV<'h>(
  commandBuffer: VkCommandBuffer<'h>,
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
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub fn vkReleaseDisplayEXT<'h>(physicalDevice: VkPhysicalDevice<'h>, display: VkDisplayKHR<'h>) -> VkResult {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkReleaseDisplayEXT.unwrap()(physicalDevice.as_raw(), display.as_raw())
    })
  }
}

// feature: VK_EXT_acquire_xlib_display

/// Acquire access to a VkDisplayKHR using Xlib
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
pub fn vkAcquireXlibDisplayEXT<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  dpy: *mut wsi::xlib::Display,
  display: VkDisplayKHR<'h>,
) -> VkResult {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkAcquireXlibDisplayEXT.unwrap()(physicalDevice.as_raw(), dpy, display.as_raw())
    })
  }
}

/// Query the VkDisplayKHR corresponding to an X11 RandR Output
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
pub fn vkGetRandROutputDisplayEXT<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  dpy: *mut wsi::xlib::Display,
  rrOutput: wsi::xlib::RROutput,
) -> VkResult<VkDisplayKHR<'h>> {
  unsafe {
    let mut pDisplay: VkDisplayKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetRandROutputDisplayEXT.unwrap()(physicalDevice.as_raw(), dpy, rrOutput, (&mut pDisplay).as_raw());
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pDisplay);
    })
  }
}

// feature: VK_EXT_display_surface_counter

/// Query surface capabilities
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  surface: VkSurfaceKHR<'h>,
) -> VkResult<VkSurfaceCapabilities2EXT> {
  unsafe {
    let mut pSurfaceCapabilities: VkSurfaceCapabilities2EXT = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceSurfaceCapabilities2EXT.unwrap()(
        physicalDevice.as_raw(),
        surface.as_raw(),
        (&mut pSurfaceCapabilities).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurfaceCapabilities);
    })
  }
}

// feature: VK_EXT_display_control

/// Set the power state of a display
#[cfg(feature = "VK_EXT_display_control")]
pub fn vkDisplayPowerControlEXT<'h>(
  device: VkDevice<'h>,
  display: VkDisplayKHR<'h>,
  pDisplayPowerInfo: &VkDisplayPowerInfoEXT,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDisplayPowerControlEXT.unwrap()(device.as_raw(), display.as_raw(), pDisplayPowerInfo.as_raw())
    })
  }
}

/// Signal a fence when a device event occurs
#[cfg(feature = "VK_EXT_display_control")]
pub fn vkRegisterDeviceEventEXT<'h>(
  device: VkDevice<'h>,
  pDeviceEventInfo: &VkDeviceEventInfoEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkFence<'h>> {
  unsafe {
    let mut pFence: VkFence<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkRegisterDeviceEventEXT.unwrap()(
        device.as_raw(),
        pDeviceEventInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pFence).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pFence);
    })
  }
}

/// Signal a fence when a display event occurs
#[cfg(feature = "VK_EXT_display_control")]
pub fn vkRegisterDisplayEventEXT<'h>(
  device: VkDevice<'h>,
  display: VkDisplayKHR<'h>,
  pDisplayEventInfo: &VkDisplayEventInfoEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkFence<'h>> {
  unsafe {
    let mut pFence: VkFence<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkRegisterDisplayEventEXT.unwrap()(
        device.as_raw(),
        display.as_raw(),
        pDisplayEventInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pFence).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pFence);
    })
  }
}

/// Query the current value of a surface counter
#[cfg(feature = "VK_EXT_display_control")]
pub fn vkGetSwapchainCounterEXT<'h>(
  device: VkDevice<'h>,
  swapchain: VkSwapchainKHR<'h>,
  counter: VkSurfaceCounterFlagBitsEXT,
) -> VkResult<u64> {
  unsafe {
    let mut pCounterValue: u64 = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetSwapchainCounterEXT.unwrap()(device.as_raw(), swapchain.as_raw(), counter, &mut pCounterValue);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pCounterValue);
    })
  }
}

// feature: VK_GOOGLE_display_timing

/// Obtain the RC duration of the PE\'s display
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub fn vkGetRefreshCycleDurationGOOGLE<'h>(
  device: VkDevice<'h>,
  swapchain: VkSwapchainKHR<'h>,
) -> VkResult<VkRefreshCycleDurationGOOGLE> {
  unsafe {
    let mut pDisplayTimingProperties: VkRefreshCycleDurationGOOGLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetRefreshCycleDurationGOOGLE.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        (&mut pDisplayTimingProperties).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pDisplayTimingProperties);
    })
  }
}

/// Obtain timing of a previously-presented image
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub fn vkGetPastPresentationTimingGOOGLE<'h>(
  device: VkDevice<'h>,
  swapchain: VkSwapchainKHR<'h>,
) -> VkResult<Vec<VkPastPresentationTimingGOOGLE>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pPresentationTimingCount == 0 {
        return Ok(pPresentationTimings);
      }
      pPresentationTimings = Vec::with_capacity(pPresentationTimingCount as usize);
      let _r = _t.vkGetPastPresentationTimingGOOGLE.unwrap()(
        device.as_raw(),
        swapchain.as_raw(),
        &mut pPresentationTimingCount,
        pPresentationTimings.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pPresentationTimings.set_len(pPresentationTimingCount as usize);
      return Ok(pPresentationTimings);
    })
  }
}

// feature: VK_EXT_discard_rectangles

/// Set discard rectangles dynamically
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub fn vkCmdSetDiscardRectangleEXT<'h>(
  commandBuffer: VkCommandBuffer<'h>,
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
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub fn vkSetHdrMetadataEXT<'h>(
  device: VkDevice<'h>,
  pSwapchains: &[VkSwapchainKHR<'h>],
  pMetadata: &[VkHdrMetadataEXT],
) {
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
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub fn vkGetPhysicalDeviceSurfaceCapabilities2KHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  pSurfaceInfo: &VkPhysicalDeviceSurfaceInfo2KHR<'_, 'h>,
) -> VkResult<VkSurfaceCapabilities2KHR> {
  unsafe {
    let mut pSurfaceCapabilities: VkSurfaceCapabilities2KHR = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      let _r = _t.vkGetPhysicalDeviceSurfaceCapabilities2KHR.unwrap()(
        physicalDevice.as_raw(),
        pSurfaceInfo.as_raw(),
        (&mut pSurfaceCapabilities).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurfaceCapabilities);
    })
  }
}

/// Query color formats supported by surface
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub fn vkGetPhysicalDeviceSurfaceFormats2KHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
  pSurfaceInfo: &VkPhysicalDeviceSurfaceInfo2KHR<'_, 'h>,
) -> VkResult<Vec<VkSurfaceFormat2KHR>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pSurfaceFormatCount == 0 {
        return Ok(pSurfaceFormats);
      }
      pSurfaceFormats = Vec::with_capacity(pSurfaceFormatCount as usize);
      let _r = _t.vkGetPhysicalDeviceSurfaceFormats2KHR.unwrap()(
        physicalDevice.as_raw(),
        pSurfaceInfo.as_raw(),
        &mut pSurfaceFormatCount,
        pSurfaceFormats.as_mut_ptr().as_raw(),
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pSurfaceFormats.set_len(pSurfaceFormatCount as usize);
      return Ok(pSurfaceFormats);
    })
  }
}

// feature: VK_KHR_shared_presentable_image

/// Get a swapchain\'s status
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub fn vkGetSwapchainStatusKHR<'h>(device: VkDevice<'h>, swapchain: VkSwapchainKHR<'h>) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetSwapchainStatusKHR.unwrap()(device.as_raw(), swapchain.as_raw())
    })
  }
}

// feature: VK_KHR_external_fence_capabilities

/// Function for querying external fence handle capabilities.
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub fn vkGetPhysicalDeviceExternalFencePropertiesKHR<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
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
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkImportFenceWin32HandleKHR<'h>(
  device: VkDevice<'h>,
  pImportFenceWin32HandleInfo: &VkImportFenceWin32HandleInfoKHR<'_, 'h>,
) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportFenceWin32HandleKHR.unwrap()(device.as_raw(), pImportFenceWin32HandleInfo.as_raw())
    })
  }
}

/// Get a Windows HANDLE for a fence
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetFenceWin32HandleKHR<'h>(
  device: VkDevice<'h>,
  pGetWin32HandleInfo: &VkFenceGetWin32HandleInfoKHR<'_, 'h>,
) -> VkResult<wsi::win32::HANDLE> {
  unsafe {
    let mut pHandle: wsi::win32::HANDLE = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetFenceWin32HandleKHR.unwrap()(device.as_raw(), pGetWin32HandleInfo.as_raw(), &mut pHandle);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pHandle);
    })
  }
}

// feature: VK_KHR_external_fence_fd

/// Import a fence from a POSIX file descriptor
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub fn vkImportFenceFdKHR<'h>(device: VkDevice<'h>, pImportFenceFdInfo: &VkImportFenceFdInfoKHR<'_, 'h>) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportFenceFdKHR.unwrap()(device.as_raw(), pImportFenceFdInfo.as_raw())
    })
  }
}

/// Get a POSIX file descriptor handle for a fence
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub fn vkGetFenceFdKHR<'h>(device: VkDevice<'h>, pGetFdInfo: &VkFenceGetFdInfoKHR<'_, 'h>) -> VkResult<c_int> {
  unsafe {
    let mut pFd: c_int = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetFenceFdKHR.unwrap()(device.as_raw(), pGetFdInfo.as_raw(), &mut pFd);
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pFd);
    })
  }
}

// feature: VK_MVK_ios_surface

/// Create a VkSurfaceKHR object for an iOS UIView
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub fn vkCreateIOSSurfaceMVK<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkIOSSurfaceCreateInfoMVK,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateIOSSurfaceMVK.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_MVK_macos_surface

/// Create a VkSurfaceKHR object for a macOS NSView
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub fn vkCreateMacOSSurfaceMVK<'h>(
  instance: VkInstance<'h>,
  pCreateInfo: &VkMacOSSurfaceCreateInfoMVK,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSurfaceKHR<'h>> {
  unsafe {
    let mut pSurface: VkSurfaceKHR<'h> = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(instance, |_t| {
      let _r = _t.vkCreateMacOSSurfaceMVK.unwrap()(
        instance.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pSurface).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pSurface);
    })
  }
}

// feature: VK_KHR_get_memory_requirements2

/// Returns the memory requirements for specified Vulkan object
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub fn vkGetImageMemoryRequirements2KHR<'h>(
  device: VkDevice<'h>,
  pInfo: &VkImageMemoryRequirementsInfo2KHR<'_, 'h>,
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
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub fn vkGetBufferMemoryRequirements2KHR<'h>(
  device: VkDevice<'h>,
  pInfo: &VkBufferMemoryRequirementsInfo2KHR<'_, 'h>,
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
pub fn vkGetImageSparseMemoryRequirements2KHR<'h>(
  device: VkDevice<'h>,
  pInfo: &VkImageSparseMemoryRequirementsInfo2KHR<'_, 'h>,
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
        pSparseMemoryRequirements.as_mut_ptr().as_raw(),
      );
      pSparseMemoryRequirements.set_len(pSparseMemoryRequirementCount as usize);
      return pSparseMemoryRequirements;
    })
  }
}

// feature: VK_EXT_sample_locations

/// Set the dynamic sample locations state
#[cfg(feature = "VK_EXT_sample_locations")]
pub fn vkCmdSetSampleLocationsEXT<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pSampleLocationsInfo: &VkSampleLocationsInfoEXT,
) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetSampleLocationsEXT.unwrap()(commandBuffer.as_raw(), pSampleLocationsInfo.as_raw())
    })
  }
}

/// Report sample count specific multisampling capabilities of a physical device
#[cfg(feature = "VK_EXT_sample_locations")]
pub fn vkGetPhysicalDeviceMultisamplePropertiesEXT<'h>(
  physicalDevice: VkPhysicalDevice<'h>,
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
#[cfg(feature = "VK_KHR_bind_memory2")]
pub fn vkBindBufferMemory2KHR<'h>(device: VkDevice<'h>, pBindInfos: &[VkBindBufferMemoryInfoKHR<'_, 'h>]) -> VkResult {
  unsafe {
    let bindInfoCount = pBindInfos.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindBufferMemory2KHR.unwrap()(device.as_raw(), bindInfoCount, pBindInfos.as_raw())
    })
  }
}

/// Bind device memory to image objects
#[cfg(feature = "VK_KHR_bind_memory2")]
pub fn vkBindImageMemory2KHR<'h>(device: VkDevice<'h>, pBindInfos: &[VkBindImageMemoryInfoKHR<'_, 'h>]) -> VkResult {
  unsafe {
    let bindInfoCount = pBindInfos.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindImageMemory2KHR.unwrap()(device.as_raw(), bindInfoCount, pBindInfos.as_raw())
    })
  }
}

// feature: VK_KHR_sampler_ycbcr_conversion

/// Create a new Ycbcr conversion
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub fn vkCreateSamplerYcbcrConversionKHR<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkSamplerYcbcrConversionCreateInfoKHR,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkSamplerYcbcrConversionKHR<'h>> {
  unsafe {
    let mut pYcbcrConversion: VkSamplerYcbcrConversionKHR<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateSamplerYcbcrConversionKHR.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pYcbcrConversion).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pYcbcrConversion);
    })
  }
}

/// Destroy a created Y\'CbCr conversion
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub fn vkDestroySamplerYcbcrConversionKHR<'h>(
  device: VkDevice<'h>,
  ycbcrConversion: Option<VkSamplerYcbcrConversionKHR<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySamplerYcbcrConversionKHR.unwrap()(device.as_raw(), ycbcrConversion.as_raw(), pAllocator.as_raw())
    })
  }
}

// feature: VK_EXT_validation_cache

/// Creates a new validation cache
#[cfg(feature = "VK_EXT_validation_cache")]
pub fn vkCreateValidationCacheEXT<'h>(
  device: VkDevice<'h>,
  pCreateInfo: &VkValidationCacheCreateInfoEXT,
  pAllocator: Option<&VkAllocationCallbacks>,
) -> VkResult<VkValidationCacheEXT<'h>> {
  unsafe {
    let mut pValidationCache: VkValidationCacheEXT<'h> = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkCreateValidationCacheEXT.unwrap()(
        device.as_raw(),
        pCreateInfo.as_raw(),
        pAllocator.as_raw(),
        (&mut pValidationCache).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pValidationCache);
    })
  }
}

/// Destroy a validation cache object
#[cfg(feature = "VK_EXT_validation_cache")]
pub fn vkDestroyValidationCacheEXT<'h>(
  device: VkDevice<'h>,
  validationCache: Option<VkValidationCacheEXT<'h>>,
  pAllocator: Option<&VkAllocationCallbacks>,
) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyValidationCacheEXT.unwrap()(device.as_raw(), validationCache.as_raw(), pAllocator.as_raw())
    })
  }
}

/// Combine the data stores of validation caches
#[cfg(feature = "VK_EXT_validation_cache")]
pub fn vkMergeValidationCachesEXT<'h>(
  device: VkDevice<'h>,
  dstCache: VkValidationCacheEXT<'h>,
  pSrcCaches: &[VkValidationCacheEXT<'h>],
) -> VkResult {
  unsafe {
    let srcCacheCount = pSrcCaches.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkMergeValidationCachesEXT.unwrap()(device.as_raw(), dstCache.as_raw(), srcCacheCount, pSrcCaches.as_raw())
    })
  }
}

/// Get the data store from a validation cache
#[cfg(feature = "VK_EXT_validation_cache")]
pub fn vkGetValidationCacheDataEXT<'h>(
  device: VkDevice<'h>,
  validationCache: VkValidationCacheEXT<'h>,
) -> VkResult<Vec<u8>> {
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
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      if pDataSize == 0 {
        return Ok(pData);
      }
      pData = Vec::with_capacity(pDataSize as usize);
      let _r = _t.vkGetValidationCacheDataEXT.unwrap()(
        device.as_raw(),
        validationCache.as_raw(),
        &mut pDataSize,
        pData.as_mut_ptr().as_raw() as *mut c_void,
      );
      if _r == Err(VkError::INCOMPLETE) {
        continue;
      }
      if let Err(_e) = _r {
        return Err(_e);
      }
      pData.set_len(pDataSize as usize);
      return Ok(pData);
    })
  }
}

// feature: VK_EXT_external_memory_host

/// Get properties of external memory host pointer
#[cfg(feature = "VK_EXT_external_memory_host")]
pub fn vkGetMemoryHostPointerPropertiesEXT<'h>(
  device: VkDevice<'h>,
  handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pHostPointer: *const c_void,
) -> VkResult<VkMemoryHostPointerPropertiesEXT> {
  unsafe {
    let mut pMemoryHostPointerProperties: VkMemoryHostPointerPropertiesEXT = ::std::mem::zeroed();
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkGetMemoryHostPointerPropertiesEXT.unwrap()(
        device.as_raw(),
        handleType,
        pHostPointer,
        (&mut pMemoryHostPointerProperties).as_raw(),
      );
      if let Err(_e) = _r {
        return Err(_e);
      }
      return Ok(pMemoryHostPointerProperties);
    })
  }
}

// feature: VK_AMD_buffer_marker

/// Execute a pipelined write of a marker value into a buffer
#[cfg(feature = "VK_AMD_buffer_marker")]
pub fn vkCmdWriteBufferMarkerAMD<'h>(
  commandBuffer: VkCommandBuffer<'h>,
  pipelineStage: VkPipelineStageFlagBits,
  dstBuffer: VkBuffer<'h>,
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
