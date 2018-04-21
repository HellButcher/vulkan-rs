/* GENERATED FILE */

#![allow(non_snake_case)]

use AsRaw;
use dispatch_table::*;
use enums::VkResult;
use platform::*;
use types::*;

// feature: VK_VERSION_1_0
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
pub fn vkDestroyInstance(instance: VkInstance, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkInstanceDispatchTable::with(instance, |_t| {
      _t.vkDestroyInstance.unwrap()(instance.as_raw(), pAllocator.as_raw())
    });
    VkInstanceDispatchTable::destroy(instance);
  }
}
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
        pPhysicalDevices.as_mut_ptr().as_raw(),
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
pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice) -> VkPhysicalDeviceFeatures {
  unsafe {
    let mut pFeatures: VkPhysicalDeviceFeatures = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceFeatures.unwrap()(physicalDevice.as_raw(), (&mut pFeatures).as_raw());
      return pFeatures;
    })
  }
}
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
pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice) -> VkPhysicalDeviceProperties {
  unsafe {
    let mut pProperties: VkPhysicalDeviceProperties = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceProperties.unwrap()(physicalDevice.as_raw(), (&mut pProperties).as_raw());
      return pProperties;
    })
  }
}
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
        pQueueFamilyProperties.as_mut_ptr().as_raw(),
      );
      pQueueFamilyProperties.set_len(pQueueFamilyPropertyCount as usize);
      return pQueueFamilyProperties;
    })
  }
}
pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice) -> VkPhysicalDeviceMemoryProperties {
  unsafe {
    let mut pMemoryProperties: VkPhysicalDeviceMemoryProperties = ::std::mem::zeroed();
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceMemoryProperties.unwrap()(physicalDevice.as_raw(), (&mut pMemoryProperties).as_raw());
      return pMemoryProperties;
    })
  }
}
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
pub fn vkDestroyDevice(device: VkDevice, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyDevice.unwrap()(device.as_raw(), pAllocator.as_raw())
    });
    VkDeviceDispatchTable::destroy(device);
  }
}
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
        pProperties.as_mut_ptr().as_raw(),
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
        pProperties.as_mut_ptr().as_raw(),
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
      let _r = _t.vkEnumerateInstanceLayerProperties.unwrap()(&mut pPropertyCount, pProperties.as_mut_ptr().as_raw());
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
        pProperties.as_mut_ptr().as_raw(),
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
pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult {
  unsafe { VkDeviceDispatchTable::with(queue, |_t| _t.vkQueueWaitIdle.unwrap()(queue.as_raw())) }
}
pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult {
  unsafe { VkDeviceDispatchTable::with(device, |_t| _t.vkDeviceWaitIdle.unwrap()(device.as_raw())) }
}
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
        pSparseMemoryRequirements.as_mut_ptr().as_raw(),
      );
      pSparseMemoryRequirements.set_len(pSparseMemoryRequirementCount as usize);
      return pSparseMemoryRequirements;
    })
  }
}
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
        pProperties.as_mut_ptr().as_raw(),
      );
      pProperties.set_len(pPropertyCount as usize);
      return pProperties;
    })
  }
}
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
pub fn vkDestroyFence(device: VkDevice, fence: Option<VkFence>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyFence.unwrap()(device.as_raw(), fence.as_raw(), pAllocator.as_raw())
    })
  }
}
pub fn vkResetFences(device: VkDevice, pFences: &[VkFence]) -> VkResult {
  unsafe {
    let fenceCount = pFences.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetFences.unwrap()(device.as_raw(), fenceCount, pFences.as_raw())
    })
  }
}
pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetFenceStatus.unwrap()(device.as_raw(), fence.as_raw())
    })
  }
}
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
pub fn vkDestroyEvent(device: VkDevice, event: Option<VkEvent>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyEvent.unwrap()(device.as_raw(), event.as_raw(), pAllocator.as_raw())
    })
  }
}
pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetEventStatus.unwrap()(device.as_raw(), event.as_raw())
    })
  }
}
pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkSetEvent.unwrap()(device.as_raw(), event.as_raw())
    })
  }
}
pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetEvent.unwrap()(device.as_raw(), event.as_raw())
    })
  }
}
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
pub fn vkDestroyBuffer(device: VkDevice, buffer: Option<VkBuffer>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyBuffer.unwrap()(device.as_raw(), buffer.as_raw(), pAllocator.as_raw())
    })
  }
}
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
pub fn vkDestroyImage(device: VkDevice, image: Option<VkImage>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyImage.unwrap()(device.as_raw(), image.as_raw(), pAllocator.as_raw())
    })
  }
}
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
        pData.as_mut_ptr().as_raw() as *mut c_void,
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
pub fn vkDestroyPipeline(device: VkDevice, pipeline: Option<VkPipeline>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroyPipeline.unwrap()(device.as_raw(), pipeline.as_raw(), pAllocator.as_raw())
    })
  }
}
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
pub fn vkDestroySampler(device: VkDevice, sampler: Option<VkSampler>, pAllocator: Option<&VkAllocationCallbacks>) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDestroySampler.unwrap()(device.as_raw(), sampler.as_raw(), pAllocator.as_raw())
    })
  }
}
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
pub fn vkAllocateDescriptorSets(
  device: VkDevice,
  pAllocateInfo: &VkDescriptorSetAllocateInfo,
) -> Result<Vec<VkDescriptorSet>, VkResult> {
  unsafe {
    let mut pDescriptorSets: Vec<VkDescriptorSet> =
      Vec::with_capacity(pAllocateInfo.get_descriptor_set_count() as usize);
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAllocateDescriptorSets.unwrap()(
        device.as_raw(),
        pAllocateInfo.as_raw(),
        pDescriptorSets.as_mut_ptr().as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pDescriptorSets.set_len(pAllocateInfo.get_descriptor_set_count() as usize);
      return Ok(pDescriptorSets);
    })
  }
}
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
pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkResetCommandPool.unwrap()(device.as_raw(), commandPool.as_raw(), flags)
    })
  }
}
pub fn vkAllocateCommandBuffers(
  device: VkDevice,
  pAllocateInfo: &VkCommandBufferAllocateInfo,
) -> Result<Vec<VkCommandBuffer>, VkResult> {
  unsafe {
    let mut pCommandBuffers: Vec<VkCommandBuffer> =
      Vec::with_capacity(pAllocateInfo.get_command_buffer_count() as usize);
    VkDeviceDispatchTable::with(device, |_t| {
      let _r = _t.vkAllocateCommandBuffers.unwrap()(
        device.as_raw(),
        pAllocateInfo.as_raw(),
        pCommandBuffers.as_mut_ptr().as_raw(),
      );
      if _r != VkResult::E_SUCCESS {
        return Err(_r);
      }
      pCommandBuffers.set_len(pAllocateInfo.get_command_buffer_count() as usize);
      return Ok(pCommandBuffers);
    })
  }
}
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
pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: &VkCommandBufferBeginInfo) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkBeginCommandBuffer.unwrap()(commandBuffer.as_raw(), pBeginInfo.as_raw())
    })
  }
}
pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkEndCommandBuffer.unwrap()(commandBuffer.as_raw())
    })
  }
}
pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkResetCommandBuffer.unwrap()(commandBuffer.as_raw(), flags)
    })
  }
}
pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBindPipeline.unwrap()(commandBuffer.as_raw(), pipelineBindPoint, pipeline.as_raw())
    })
  }
}
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
pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetLineWidth.unwrap()(commandBuffer.as_raw(), lineWidth)
    })
  }
}
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
pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: [f32; 4]) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetBlendConstants.unwrap()(commandBuffer.as_raw(), blendConstants)
    })
  }
}
pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetDepthBounds.unwrap()(commandBuffer.as_raw(), minDepthBounds, maxDepthBounds)
    })
  }
}
pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilCompareMask.unwrap()(commandBuffer.as_raw(), faceMask, compareMask)
    })
  }
}
pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilWriteMask.unwrap()(commandBuffer.as_raw(), faceMask, writeMask)
    })
  }
}
pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetStencilReference.unwrap()(commandBuffer.as_raw(), faceMask, reference)
    })
  }
}
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
pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDispatchIndirect.unwrap()(commandBuffer.as_raw(), buffer.as_raw(), offset)
    })
  }
}
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
pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetEvent.unwrap()(commandBuffer.as_raw(), event.as_raw(), stageMask)
    })
  }
}
pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdResetEvent.unwrap()(commandBuffer.as_raw(), event.as_raw(), stageMask)
    })
  }
}
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
pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdBeginQuery.unwrap()(commandBuffer.as_raw(), queryPool.as_raw(), query, flags)
    })
  }
}
pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdEndQuery.unwrap()(commandBuffer.as_raw(), queryPool.as_raw(), query)
    })
  }
}
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
pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdNextSubpass.unwrap()(commandBuffer.as_raw(), contents)
    })
  }
}
pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdEndRenderPass.unwrap()(commandBuffer.as_raw())
    })
  }
}
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
#[cfg(feature = "VK_KHR_surface")]
pub fn vkGetPhysicalDeviceSurfaceSupportKHR(
  physicalDevice: VkPhysicalDevice,
  queueFamilyIndex: u32,
  surface: VkSurfaceKHR,
) -> Result<bool, VkResult> {
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
      return Ok(pSupported != 0);
    })
  }
}
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
        pSurfaceFormats.as_mut_ptr().as_raw(),
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
        pPresentModes.as_mut_ptr().as_raw(),
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
        pSwapchainImages.as_mut_ptr().as_raw(),
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
#[cfg(feature = "VK_KHR_swapchain")]
pub fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: &VkPresentInfoKHR) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(queue, |_t| {
      _t.vkQueuePresentKHR.unwrap()(queue.as_raw(), pPresentInfo.as_raw())
    })
  }
}

// feature: VK_KHR_display
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
        pProperties.as_mut_ptr().as_raw(),
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
        pProperties.as_mut_ptr().as_raw(),
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
        pDisplays.as_mut_ptr().as_raw(),
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
        pProperties.as_mut_ptr().as_raw(),
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
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
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
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
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
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
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
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub fn vkGetPhysicalDeviceMirPresentationSupportKHR(
  physicalDevice: VkPhysicalDevice,
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
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> bool {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkGetPhysicalDeviceWin32PresentationSupportKHR.unwrap()(physicalDevice.as_raw(), queueFamilyIndex)
    }) != 0
  }
}

// feature: VK_EXT_debug_report
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
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkDebugMarkerSetObjectTagEXT(device: VkDevice, pTagInfo: &VkDebugMarkerObjectTagInfoEXT) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDebugMarkerSetObjectTagEXT.unwrap()(device.as_raw(), pTagInfo.as_raw())
    })
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkDebugMarkerSetObjectNameEXT(device: VkDevice, pNameInfo: &VkDebugMarkerObjectNameInfoEXT) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkDebugMarkerSetObjectNameEXT.unwrap()(device.as_raw(), pNameInfo.as_raw())
    })
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: &VkDebugMarkerMarkerInfoEXT) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerBeginEXT.unwrap()(commandBuffer.as_raw(), pMarkerInfo.as_raw())
    })
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerEndEXT.unwrap()(commandBuffer.as_raw())
    })
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: &VkDebugMarkerMarkerInfoEXT) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdDebugMarkerInsertEXT.unwrap()(commandBuffer.as_raw(), pMarkerInfo.as_raw())
    })
  }
}

// feature: VK_AMD_draw_indirect_count
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
        pQueueFamilyProperties.as_mut_ptr().as_raw(),
      );
      pQueueFamilyProperties.set_len(pQueueFamilyPropertyCount as usize);
      return pQueueFamilyProperties;
    })
  }
}
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
        pProperties.as_mut_ptr().as_raw(),
      );
      pProperties.set_len(pPropertyCount as usize);
      return pProperties;
    })
  }
}

// feature: VK_AMD_shader_info
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
        pInfo.as_mut_ptr().as_raw() as *mut c_void,
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
        pPhysicalDeviceGroupProperties.as_mut_ptr().as_raw(),
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
#[cfg(feature = "VK_KHX_device_group")]
pub fn vkCmdSetDeviceMaskKHX(commandBuffer: VkCommandBuffer, deviceMask: u32) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetDeviceMaskKHX.unwrap()(commandBuffer.as_raw(), deviceMask)
    })
  }
}
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
        pRects.as_mut_ptr().as_raw(),
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
#[cfg(feature = "VK_KHR_maintenance1")]
pub fn vkTrimCommandPoolKHR(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR) {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkTrimCommandPoolKHR.unwrap()(device.as_raw(), commandPool.as_raw(), flags)
    })
  }
}

// feature: VK_KHR_external_memory_capabilities
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
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub fn vkImportSemaphoreFdKHR(device: VkDevice, pImportSemaphoreFdInfo: &VkImportSemaphoreFdInfoKHR) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportSemaphoreFdKHR.unwrap()(device.as_raw(), pImportSemaphoreFdInfo.as_raw())
    })
  }
}
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
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub fn vkCmdProcessCommandsNVX(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: &VkCmdProcessCommandsInfoNVX) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdProcessCommandsNVX.unwrap()(commandBuffer.as_raw(), pProcessCommandsInfo.as_raw())
    })
  }
}
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
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub fn vkReleaseDisplayEXT(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult {
  unsafe {
    VkInstanceDispatchTable::with(physicalDevice, |_t| {
      _t.vkReleaseDisplayEXT.unwrap()(physicalDevice.as_raw(), display.as_raw())
    })
  }
}

// feature: VK_EXT_acquire_xlib_display
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
        pPresentationTimings.as_mut_ptr().as_raw(),
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
        pSurfaceFormats.as_mut_ptr().as_raw(),
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
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub fn vkGetSwapchainStatusKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkGetSwapchainStatusKHR.unwrap()(device.as_raw(), swapchain.as_raw())
    })
  }
}

// feature: VK_KHR_external_fence_capabilities
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
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub fn vkImportFenceFdKHR(device: VkDevice, pImportFenceFdInfo: &VkImportFenceFdInfoKHR) -> VkResult {
  unsafe {
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkImportFenceFdKHR.unwrap()(device.as_raw(), pImportFenceFdInfo.as_raw())
    })
  }
}
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
        pSparseMemoryRequirements.as_mut_ptr().as_raw(),
      );
      pSparseMemoryRequirements.set_len(pSparseMemoryRequirementCount as usize);
      return pSparseMemoryRequirements;
    })
  }
}

// feature: VK_EXT_sample_locations
#[cfg(feature = "VK_EXT_sample_locations")]
pub fn vkCmdSetSampleLocationsEXT(commandBuffer: VkCommandBuffer, pSampleLocationsInfo: &VkSampleLocationsInfoEXT) {
  unsafe {
    VkDeviceDispatchTable::with(commandBuffer, |_t| {
      _t.vkCmdSetSampleLocationsEXT.unwrap()(commandBuffer.as_raw(), pSampleLocationsInfo.as_raw())
    })
  }
}
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
#[cfg(feature = "VK_KHR_bind_memory2")]
pub fn vkBindBufferMemory2KHR(device: VkDevice, pBindInfos: &[VkBindBufferMemoryInfoKHR]) -> VkResult {
  unsafe {
    let bindInfoCount = pBindInfos.len() as u32;
    VkDeviceDispatchTable::with(device, |_t| {
      _t.vkBindBufferMemory2KHR.unwrap()(device.as_raw(), bindInfoCount, pBindInfos.as_raw())
    })
  }
}
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
        pData.as_mut_ptr().as_raw() as *mut c_void,
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
