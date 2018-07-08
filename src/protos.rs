/* GENERATED FILE */

#![allow(non_camel_case_types)]

use platform::*;
use types::*;

// feature: VK_VERSION_1_0
pub type PFN_vkCreateInstance =
  extern "system" fn(*const VkInstanceCreateInfo, *const VkAllocationCallbacks, *mut usize) -> VkResult;
pub type PFN_vkDestroyInstance = extern "system" fn(usize, *const VkAllocationCallbacks);
pub type PFN_vkEnumeratePhysicalDevices = extern "system" fn(usize, *mut u32, *mut usize) -> VkResult;
pub type PFN_vkGetPhysicalDeviceFeatures = extern "system" fn(usize, *mut VkPhysicalDeviceFeatures);
pub type PFN_vkGetPhysicalDeviceFormatProperties = extern "system" fn(usize, VkFormat, *mut VkFormatProperties);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = extern "system" fn(
  usize,
  VkFormat,
  VkImageType,
  VkImageTiling,
  VkImageUsageFlags,
  VkImageCreateFlags,
  *mut VkImageFormatProperties,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceProperties = extern "system" fn(usize, *mut VkPhysicalDeviceProperties);
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties =
  extern "system" fn(usize, *mut u32, *mut VkQueueFamilyProperties);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = extern "system" fn(usize, *mut VkPhysicalDeviceMemoryProperties);
pub type PFN_vkGetInstanceProcAddr = extern "system" fn(usize, *const c_char) -> Option<PFN_vkVoidFunction>;
pub type PFN_vkGetDeviceProcAddr = extern "system" fn(usize, *const c_char) -> Option<PFN_vkVoidFunction>;
pub type PFN_vkCreateDevice =
  extern "system" fn(usize, *const VkDeviceCreateInfo, *const VkAllocationCallbacks, *mut usize) -> VkResult;
pub type PFN_vkDestroyDevice = extern "system" fn(usize, *const VkAllocationCallbacks);
pub type PFN_vkEnumerateInstanceExtensionProperties =
  extern "system" fn(*const c_char, *mut u32, *mut VkExtensionProperties) -> VkResult;
pub type PFN_vkEnumerateDeviceExtensionProperties =
  extern "system" fn(usize, *const c_char, *mut u32, *mut VkExtensionProperties) -> VkResult;
pub type PFN_vkEnumerateInstanceLayerProperties = extern "system" fn(*mut u32, *mut VkLayerProperties) -> VkResult;
pub type PFN_vkEnumerateDeviceLayerProperties = extern "system" fn(usize, *mut u32, *mut VkLayerProperties) -> VkResult;
pub type PFN_vkGetDeviceQueue = extern "system" fn(usize, u32, u32, *mut usize);
pub type PFN_vkQueueSubmit = extern "system" fn(usize, u32, *const VkSubmitInfo, u64) -> VkResult;
pub type PFN_vkQueueWaitIdle = extern "system" fn(usize) -> VkResult;
pub type PFN_vkDeviceWaitIdle = extern "system" fn(usize) -> VkResult;
pub type PFN_vkAllocateMemory =
  extern "system" fn(usize, *const VkMemoryAllocateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkFreeMemory = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkMapMemory =
  extern "system" fn(usize, u64, VkDeviceSize, VkDeviceSize, VkMemoryMapFlags, *mut *mut c_void) -> VkResult;
pub type PFN_vkUnmapMemory = extern "system" fn(usize, u64);
pub type PFN_vkFlushMappedMemoryRanges = extern "system" fn(usize, u32, *const VkMappedMemoryRange) -> VkResult;
pub type PFN_vkInvalidateMappedMemoryRanges = extern "system" fn(usize, u32, *const VkMappedMemoryRange) -> VkResult;
pub type PFN_vkGetDeviceMemoryCommitment = extern "system" fn(usize, u64, *mut VkDeviceSize);
pub type PFN_vkBindBufferMemory = extern "system" fn(usize, u64, u64, VkDeviceSize) -> VkResult;
pub type PFN_vkBindImageMemory = extern "system" fn(usize, u64, u64, VkDeviceSize) -> VkResult;
pub type PFN_vkGetBufferMemoryRequirements = extern "system" fn(usize, u64, *mut VkMemoryRequirements);
pub type PFN_vkGetImageMemoryRequirements = extern "system" fn(usize, u64, *mut VkMemoryRequirements);
pub type PFN_vkGetImageSparseMemoryRequirements =
  extern "system" fn(usize, u64, *mut u32, *mut VkSparseImageMemoryRequirements);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = extern "system" fn(
  usize,
  VkFormat,
  VkImageType,
  VkSampleCountFlagBits,
  VkImageUsageFlags,
  VkImageTiling,
  *mut u32,
  *mut VkSparseImageFormatProperties,
);
pub type PFN_vkQueueBindSparse = extern "system" fn(usize, u32, *const VkBindSparseInfo, u64) -> VkResult;
pub type PFN_vkCreateFence =
  extern "system" fn(usize, *const VkFenceCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyFence = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkResetFences = extern "system" fn(usize, u32, *const u64) -> VkResult;
pub type PFN_vkGetFenceStatus = extern "system" fn(usize, u64) -> VkResult;
pub type PFN_vkWaitForFences = extern "system" fn(usize, u32, *const u64, VkBool32, u64) -> VkResult;
pub type PFN_vkCreateSemaphore =
  extern "system" fn(usize, *const VkSemaphoreCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroySemaphore = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreateEvent =
  extern "system" fn(usize, *const VkEventCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyEvent = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkGetEventStatus = extern "system" fn(usize, u64) -> VkResult;
pub type PFN_vkSetEvent = extern "system" fn(usize, u64) -> VkResult;
pub type PFN_vkResetEvent = extern "system" fn(usize, u64) -> VkResult;
pub type PFN_vkCreateQueryPool =
  extern "system" fn(usize, *const VkQueryPoolCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyQueryPool = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkGetQueryPoolResults =
  extern "system" fn(usize, u64, u32, u32, usize, *mut c_void, VkDeviceSize, VkQueryResultFlags) -> VkResult;
pub type PFN_vkCreateBuffer =
  extern "system" fn(usize, *const VkBufferCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyBuffer = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreateBufferView =
  extern "system" fn(usize, *const VkBufferViewCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyBufferView = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreateImage =
  extern "system" fn(usize, *const VkImageCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyImage = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkGetImageSubresourceLayout =
  extern "system" fn(usize, u64, *const VkImageSubresource, *mut VkSubresourceLayout);
pub type PFN_vkCreateImageView =
  extern "system" fn(usize, *const VkImageViewCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyImageView = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreateShaderModule =
  extern "system" fn(usize, *const VkShaderModuleCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyShaderModule = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreatePipelineCache =
  extern "system" fn(usize, *const VkPipelineCacheCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyPipelineCache = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkGetPipelineCacheData = extern "system" fn(usize, u64, *mut usize, *mut c_void) -> VkResult;
pub type PFN_vkMergePipelineCaches = extern "system" fn(usize, u64, u32, *const u64) -> VkResult;
pub type PFN_vkCreateGraphicsPipelines =
  extern "system" fn(usize, u64, u32, *const VkGraphicsPipelineCreateInfo, *const VkAllocationCallbacks, *mut u64)
    -> VkResult;
pub type PFN_vkCreateComputePipelines =
  extern "system" fn(usize, u64, u32, *const VkComputePipelineCreateInfo, *const VkAllocationCallbacks, *mut u64)
    -> VkResult;
pub type PFN_vkDestroyPipeline = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreatePipelineLayout =
  extern "system" fn(usize, *const VkPipelineLayoutCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyPipelineLayout = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreateSampler =
  extern "system" fn(usize, *const VkSamplerCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroySampler = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreateDescriptorSetLayout =
  extern "system" fn(usize, *const VkDescriptorSetLayoutCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyDescriptorSetLayout = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreateDescriptorPool =
  extern "system" fn(usize, *const VkDescriptorPoolCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyDescriptorPool = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkResetDescriptorPool = extern "system" fn(usize, u64, VkDescriptorPoolResetFlags) -> VkResult;
pub type PFN_vkAllocateDescriptorSets =
  extern "system" fn(usize, *const VkDescriptorSetAllocateInfo, *mut u64) -> VkResult;
pub type PFN_vkFreeDescriptorSets = extern "system" fn(usize, u64, u32, *const u64) -> VkResult;
pub type PFN_vkUpdateDescriptorSets =
  extern "system" fn(usize, u32, *const VkWriteDescriptorSet, u32, *const VkCopyDescriptorSet);
pub type PFN_vkCreateFramebuffer =
  extern "system" fn(usize, *const VkFramebufferCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyFramebuffer = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkCreateRenderPass =
  extern "system" fn(usize, *const VkRenderPassCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyRenderPass = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkGetRenderAreaGranularity = extern "system" fn(usize, u64, *mut VkExtent2D);
pub type PFN_vkCreateCommandPool =
  extern "system" fn(usize, *const VkCommandPoolCreateInfo, *const VkAllocationCallbacks, *mut u64) -> VkResult;
pub type PFN_vkDestroyCommandPool = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
pub type PFN_vkResetCommandPool = extern "system" fn(usize, u64, VkCommandPoolResetFlags) -> VkResult;
pub type PFN_vkAllocateCommandBuffers =
  extern "system" fn(usize, *const VkCommandBufferAllocateInfo, *mut usize) -> VkResult;
pub type PFN_vkFreeCommandBuffers = extern "system" fn(usize, u64, u32, *const usize);
pub type PFN_vkBeginCommandBuffer = extern "system" fn(usize, *const VkCommandBufferBeginInfo) -> VkResult;
pub type PFN_vkEndCommandBuffer = extern "system" fn(usize) -> VkResult;
pub type PFN_vkResetCommandBuffer = extern "system" fn(usize, VkCommandBufferResetFlags) -> VkResult;
pub type PFN_vkCmdBindPipeline = extern "system" fn(usize, VkPipelineBindPoint, u64);
pub type PFN_vkCmdSetViewport = extern "system" fn(usize, u32, u32, *const VkViewport);
pub type PFN_vkCmdSetScissor = extern "system" fn(usize, u32, u32, *const VkRect2D);
pub type PFN_vkCmdSetLineWidth = extern "system" fn(usize, f32);
pub type PFN_vkCmdSetDepthBias = extern "system" fn(usize, f32, f32, f32);
pub type PFN_vkCmdSetBlendConstants = extern "system" fn(usize, [f32; 4]);
pub type PFN_vkCmdSetDepthBounds = extern "system" fn(usize, f32, f32);
pub type PFN_vkCmdSetStencilCompareMask = extern "system" fn(usize, VkStencilFaceFlags, u32);
pub type PFN_vkCmdSetStencilWriteMask = extern "system" fn(usize, VkStencilFaceFlags, u32);
pub type PFN_vkCmdSetStencilReference = extern "system" fn(usize, VkStencilFaceFlags, u32);
pub type PFN_vkCmdBindDescriptorSets =
  extern "system" fn(usize, VkPipelineBindPoint, u64, u32, u32, *const u64, u32, *const u32);
pub type PFN_vkCmdBindIndexBuffer = extern "system" fn(usize, u64, VkDeviceSize, VkIndexType);
pub type PFN_vkCmdBindVertexBuffers = extern "system" fn(usize, u32, u32, *const u64, *const VkDeviceSize);
pub type PFN_vkCmdDraw = extern "system" fn(usize, u32, u32, u32, u32);
pub type PFN_vkCmdDrawIndexed = extern "system" fn(usize, u32, u32, u32, i32, u32);
pub type PFN_vkCmdDrawIndirect = extern "system" fn(usize, u64, VkDeviceSize, u32, u32);
pub type PFN_vkCmdDrawIndexedIndirect = extern "system" fn(usize, u64, VkDeviceSize, u32, u32);
pub type PFN_vkCmdDispatch = extern "system" fn(usize, u32, u32, u32);
pub type PFN_vkCmdDispatchIndirect = extern "system" fn(usize, u64, VkDeviceSize);
pub type PFN_vkCmdCopyBuffer = extern "system" fn(usize, u64, u64, u32, *const VkBufferCopy);
pub type PFN_vkCmdCopyImage =
  extern "system" fn(usize, u64, VkImageLayout, u64, VkImageLayout, u32, *const VkImageCopy);
pub type PFN_vkCmdBlitImage =
  extern "system" fn(usize, u64, VkImageLayout, u64, VkImageLayout, u32, *const VkImageBlit, VkFilter);
pub type PFN_vkCmdCopyBufferToImage = extern "system" fn(usize, u64, u64, VkImageLayout, u32, *const VkBufferImageCopy);
pub type PFN_vkCmdCopyImageToBuffer = extern "system" fn(usize, u64, VkImageLayout, u64, u32, *const VkBufferImageCopy);
pub type PFN_vkCmdUpdateBuffer = extern "system" fn(usize, u64, VkDeviceSize, VkDeviceSize, *const c_void);
pub type PFN_vkCmdFillBuffer = extern "system" fn(usize, u64, VkDeviceSize, VkDeviceSize, u32);
pub type PFN_vkCmdClearColorImage =
  extern "system" fn(usize, u64, VkImageLayout, *const VkClearColorValue, u32, *const VkImageSubresourceRange);
pub type PFN_vkCmdClearDepthStencilImage =
  extern "system" fn(usize, u64, VkImageLayout, *const VkClearDepthStencilValue, u32, *const VkImageSubresourceRange);
pub type PFN_vkCmdClearAttachments = extern "system" fn(usize, u32, *const VkClearAttachment, u32, *const VkClearRect);
pub type PFN_vkCmdResolveImage =
  extern "system" fn(usize, u64, VkImageLayout, u64, VkImageLayout, u32, *const VkImageResolve);
pub type PFN_vkCmdSetEvent = extern "system" fn(usize, u64, VkPipelineStageFlags);
pub type PFN_vkCmdResetEvent = extern "system" fn(usize, u64, VkPipelineStageFlags);
pub type PFN_vkCmdWaitEvents = extern "system" fn(
  usize,
  u32,
  *const u64,
  VkPipelineStageFlags,
  VkPipelineStageFlags,
  u32,
  *const VkMemoryBarrier,
  u32,
  *const VkBufferMemoryBarrier,
  u32,
  *const VkImageMemoryBarrier,
);
pub type PFN_vkCmdPipelineBarrier = extern "system" fn(
  usize,
  VkPipelineStageFlags,
  VkPipelineStageFlags,
  VkDependencyFlags,
  u32,
  *const VkMemoryBarrier,
  u32,
  *const VkBufferMemoryBarrier,
  u32,
  *const VkImageMemoryBarrier,
);
pub type PFN_vkCmdBeginQuery = extern "system" fn(usize, u64, u32, VkQueryControlFlags);
pub type PFN_vkCmdEndQuery = extern "system" fn(usize, u64, u32);
pub type PFN_vkCmdResetQueryPool = extern "system" fn(usize, u64, u32, u32);
pub type PFN_vkCmdWriteTimestamp = extern "system" fn(usize, VkPipelineStageFlagBits, u64, u32);
pub type PFN_vkCmdCopyQueryPoolResults =
  extern "system" fn(usize, u64, u32, u32, u64, VkDeviceSize, VkDeviceSize, VkQueryResultFlags);
pub type PFN_vkCmdPushConstants = extern "system" fn(usize, u64, VkShaderStageFlags, u32, u32, *const c_void);
pub type PFN_vkCmdBeginRenderPass = extern "system" fn(usize, *const VkRenderPassBeginInfo, VkSubpassContents);
pub type PFN_vkCmdNextSubpass = extern "system" fn(usize, VkSubpassContents);
pub type PFN_vkCmdEndRenderPass = extern "system" fn(usize);
pub type PFN_vkCmdExecuteCommands = extern "system" fn(usize, u32, *const usize);

// feature: VK_KHR_surface
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkDestroySurfaceKHR = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(usize, u32, u64, *mut VkBool32) -> VkResult;
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR =
  extern "system" fn(usize, u64, *mut VkSurfaceCapabilitiesKHR) -> VkResult;
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR =
  extern "system" fn(usize, u64, *mut u32, *mut VkSurfaceFormatKHR) -> VkResult;
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR =
  extern "system" fn(usize, u64, *mut u32, *mut VkPresentModeKHR) -> VkResult;

// feature: VK_KHR_swapchain
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkCreateSwapchainKHR =
  extern "system" fn(usize, *const VkSwapchainCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkDestroySwapchainKHR = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkGetSwapchainImagesKHR = extern "system" fn(usize, u64, *mut u32, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkAcquireNextImageKHR = extern "system" fn(usize, u64, u64, u64, u64, *mut u32) -> VkResult;
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkQueuePresentKHR = extern "system" fn(usize, *const VkPresentInfoKHR) -> VkResult;

// feature: VK_KHR_display
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR =
  extern "system" fn(usize, *mut u32, *mut VkDisplayPropertiesKHR) -> VkResult;
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR =
  extern "system" fn(usize, *mut u32, *mut VkDisplayPlanePropertiesKHR) -> VkResult;
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = extern "system" fn(usize, u32, *mut u32, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetDisplayModePropertiesKHR =
  extern "system" fn(usize, u64, *mut u32, *mut VkDisplayModePropertiesKHR) -> VkResult;
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkCreateDisplayModeKHR =
  extern "system" fn(usize, u64, *const VkDisplayModeCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR =
  extern "system" fn(usize, u64, u32, *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkCreateDisplayPlaneSurfaceKHR =
  extern "system" fn(usize, *const VkDisplaySurfaceCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;

// feature: VK_KHR_display_swapchain
#[cfg(feature = "VK_KHR_display_swapchain")]
pub type PFN_vkCreateSharedSwapchainsKHR =
  extern "system" fn(usize, u32, *const VkSwapchainCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;

// feature: VK_KHR_xlib_surface
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub type PFN_vkCreateXlibSurfaceKHR =
  extern "system" fn(usize, *const VkXlibSurfaceCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR =
  extern "system" fn(usize, u32, *mut wsi::xlib::Display, wsi::xlib::VisualID) -> VkBool32;

// feature: VK_KHR_xcb_surface
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub type PFN_vkCreateXcbSurfaceKHR =
  extern "system" fn(usize, *const VkXcbSurfaceCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR =
  extern "system" fn(usize, u32, *mut wsi::xcb::xcb_connection_t, wsi::xcb::xcb_visualid_t) -> VkBool32;

// feature: VK_KHR_wayland_surface
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub type PFN_vkCreateWaylandSurfaceKHR =
  extern "system" fn(usize, *const VkWaylandSurfaceCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR =
  extern "system" fn(usize, u32, *mut wsi::wayland::wl_display) -> VkBool32;

// feature: VK_KHR_mir_surface
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub type PFN_vkCreateMirSurfaceKHR =
  extern "system" fn(usize, *const VkMirSurfaceCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub type PFN_vkGetPhysicalDeviceMirPresentationSupportKHR =
  extern "system" fn(usize, u32, *mut wsi::mir::MirConnection) -> VkBool32;

// feature: VK_KHR_android_surface
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub type PFN_vkCreateAndroidSurfaceKHR =
  extern "system" fn(usize, *const VkAndroidSurfaceCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;

// feature: VK_KHR_win32_surface
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkCreateWin32SurfaceKHR =
  extern "system" fn(usize, *const VkWin32SurfaceCreateInfoKHR, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = extern "system" fn(usize, u32) -> VkBool32;

// feature: VK_EXT_debug_report
#[cfg(feature = "VK_EXT_debug_report")]
pub type PFN_vkCreateDebugReportCallbackEXT =
  extern "system" fn(usize, *const VkDebugReportCallbackCreateInfoEXT, *const VkAllocationCallbacks, *mut u64)
    -> VkResult;
#[cfg(feature = "VK_EXT_debug_report")]
pub type PFN_vkDestroyDebugReportCallbackEXT = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
#[cfg(feature = "VK_EXT_debug_report")]
pub type PFN_vkDebugReportMessageEXT = extern "system" fn(
  usize,
  VkDebugReportFlagsEXT,
  VkDebugReportObjectTypeEXT,
  u64,
  usize,
  i32,
  *const c_char,
  *const c_char,
);

// feature: VK_EXT_debug_marker
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkDebugMarkerSetObjectTagEXT = extern "system" fn(usize, *const VkDebugMarkerObjectTagInfoEXT) -> VkResult;
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkDebugMarkerSetObjectNameEXT =
  extern "system" fn(usize, *const VkDebugMarkerObjectNameInfoEXT) -> VkResult;
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkCmdDebugMarkerBeginEXT = extern "system" fn(usize, *const VkDebugMarkerMarkerInfoEXT);
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkCmdDebugMarkerEndEXT = extern "system" fn(usize);
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkCmdDebugMarkerInsertEXT = extern "system" fn(usize, *const VkDebugMarkerMarkerInfoEXT);

// feature: VK_AMD_draw_indirect_count
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub type PFN_vkCmdDrawIndirectCountAMD = extern "system" fn(usize, u64, VkDeviceSize, u64, VkDeviceSize, u32, u32);
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub type PFN_vkCmdDrawIndexedIndirectCountAMD =
  extern "system" fn(usize, u64, VkDeviceSize, u64, VkDeviceSize, u32, u32);

// feature: VK_KHR_get_physical_device_properties2
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = extern "system" fn(usize, *mut VkPhysicalDeviceFeatures2KHR);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceProperties2KHR = extern "system" fn(usize, *mut VkPhysicalDeviceProperties2KHR);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = extern "system" fn(usize, VkFormat, *mut VkFormatProperties2KHR);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
  extern "system" fn(usize, *const VkPhysicalDeviceImageFormatInfo2KHR, *mut VkImageFormatProperties2KHR) -> VkResult;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
  extern "system" fn(usize, *mut u32, *mut VkQueueFamilyProperties2KHR);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR =
  extern "system" fn(usize, *mut VkPhysicalDeviceMemoryProperties2KHR);
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
  extern "system" fn(
    usize,
    *const VkPhysicalDeviceSparseImageFormatInfo2KHR,
    *mut u32,
    *mut VkSparseImageFormatProperties2KHR,
  );

// feature: VK_AMD_shader_info
#[cfg(feature = "VK_AMD_shader_info")]
pub type PFN_vkGetShaderInfoAMD =
  extern "system" fn(usize, u64, VkShaderStageFlagBits, VkShaderInfoTypeAMD, *mut usize, *mut c_void) -> VkResult;

// feature: VK_NV_external_memory_capabilities
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
  extern "system" fn(
    usize,
    VkFormat,
    VkImageType,
    VkImageTiling,
    VkImageUsageFlags,
    VkImageCreateFlags,
    VkExternalMemoryHandleTypeFlagsNV,
    *mut VkExternalImageFormatPropertiesNV,
  ) -> VkResult;

// feature: VK_NV_external_memory_win32
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkGetMemoryWin32HandleNV =
  extern "system" fn(usize, u64, VkExternalMemoryHandleTypeFlagsNV, *mut wsi::win32::HANDLE) -> VkResult;

// feature: VK_KHX_device_group_creation
#[cfg(feature = "VK_KHX_device_group_creation")]
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHX =
  extern "system" fn(usize, *mut u32, *mut VkPhysicalDeviceGroupPropertiesKHX) -> VkResult;

// feature: VK_KHX_device_group
#[cfg(feature = "VK_KHX_device_group")]
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX =
  extern "system" fn(usize, u32, u32, u32, *mut VkPeerMemoryFeatureFlagsKHX);
#[cfg(feature = "VK_KHX_device_group")]
pub type PFN_vkCmdSetDeviceMaskKHX = extern "system" fn(usize, u32);
#[cfg(feature = "VK_KHX_device_group")]
pub type PFN_vkCmdDispatchBaseKHX = extern "system" fn(usize, u32, u32, u32, u32, u32, u32);
#[cfg(feature = "VK_KHX_device_group")]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHX =
  extern "system" fn(usize, *mut VkDeviceGroupPresentCapabilitiesKHX) -> VkResult;
#[cfg(feature = "VK_KHX_device_group")]
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHX =
  extern "system" fn(usize, u64, *mut VkDeviceGroupPresentModeFlagsKHX) -> VkResult;
#[cfg(feature = "VK_KHX_device_group")]
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHX =
  extern "system" fn(usize, u64, *mut u32, *mut VkRect2D) -> VkResult;
#[cfg(feature = "VK_KHX_device_group")]
pub type PFN_vkAcquireNextImage2KHX = extern "system" fn(usize, *const VkAcquireNextImageInfoKHX, *mut u32) -> VkResult;

// feature: VK_NN_vi_surface
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub type PFN_vkCreateViSurfaceNN =
  extern "system" fn(usize, *const VkViSurfaceCreateInfoNN, *const VkAllocationCallbacks, *mut u64) -> VkResult;

// feature: VK_KHR_maintenance1
#[cfg(feature = "VK_KHR_maintenance1")]
pub type PFN_vkTrimCommandPoolKHR = extern "system" fn(usize, u64, VkCommandPoolTrimFlagsKHR);

// feature: VK_KHR_external_memory_capabilities
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
  extern "system" fn(usize, *const VkPhysicalDeviceExternalBufferInfoKHR, *mut VkExternalBufferPropertiesKHR);

// feature: VK_KHR_external_memory_win32
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkGetMemoryWin32HandleKHR =
  extern "system" fn(usize, *const VkMemoryGetWin32HandleInfoKHR, *mut wsi::win32::HANDLE) -> VkResult;
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = extern "system" fn(
  usize,
  VkExternalMemoryHandleTypeFlagBitsKHR,
  wsi::win32::HANDLE,
  *mut VkMemoryWin32HandlePropertiesKHR,
) -> VkResult;

// feature: VK_KHR_external_memory_fd
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub type PFN_vkGetMemoryFdKHR = extern "system" fn(usize, *const VkMemoryGetFdInfoKHR, *mut c_int) -> VkResult;
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub type PFN_vkGetMemoryFdPropertiesKHR =
  extern "system" fn(usize, VkExternalMemoryHandleTypeFlagBitsKHR, c_int, *mut VkMemoryFdPropertiesKHR) -> VkResult;

// feature: VK_KHR_external_semaphore_capabilities
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
  extern "system" fn(usize, *const VkPhysicalDeviceExternalSemaphoreInfoKHR, *mut VkExternalSemaphorePropertiesKHR);

// feature: VK_KHR_external_semaphore_win32
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkImportSemaphoreWin32HandleKHR =
  extern "system" fn(usize, *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkGetSemaphoreWin32HandleKHR =
  extern "system" fn(usize, *const VkSemaphoreGetWin32HandleInfoKHR, *mut wsi::win32::HANDLE) -> VkResult;

// feature: VK_KHR_external_semaphore_fd
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub type PFN_vkImportSemaphoreFdKHR = extern "system" fn(usize, *const VkImportSemaphoreFdInfoKHR) -> VkResult;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub type PFN_vkGetSemaphoreFdKHR = extern "system" fn(usize, *const VkSemaphoreGetFdInfoKHR, *mut c_int) -> VkResult;

// feature: VK_KHR_push_descriptor
#[cfg(feature = "VK_KHR_push_descriptor")]
pub type PFN_vkCmdPushDescriptorSetKHR =
  extern "system" fn(usize, VkPipelineBindPoint, u64, u32, u32, *const VkWriteDescriptorSet);

// feature: VK_KHR_descriptor_update_template
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkCreateDescriptorUpdateTemplateKHR =
  extern "system" fn(usize, *const VkDescriptorUpdateTemplateCreateInfoKHR, *const VkAllocationCallbacks, *mut u64)
    -> VkResult;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = extern "system" fn(usize, u64, u64, *const c_void);
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = extern "system" fn(usize, u64, u64, u32, *const c_void);

// feature: VK_NVX_device_generated_commands
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkCmdProcessCommandsNVX = extern "system" fn(usize, *const VkCmdProcessCommandsInfoNVX);
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkCmdReserveSpaceForCommandsNVX = extern "system" fn(usize, *const VkCmdReserveSpaceForCommandsInfoNVX);
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkCreateIndirectCommandsLayoutNVX =
  extern "system" fn(usize, *const VkIndirectCommandsLayoutCreateInfoNVX, *const VkAllocationCallbacks, *mut u64)
    -> VkResult;
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkDestroyIndirectCommandsLayoutNVX = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkCreateObjectTableNVX =
  extern "system" fn(usize, *const VkObjectTableCreateInfoNVX, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkDestroyObjectTableNVX = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkRegisterObjectsNVX =
  extern "system" fn(usize, u64, u32, *const *const VkObjectTableEntryNVX, *const u32) -> VkResult;
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkUnregisterObjectsNVX =
  extern "system" fn(usize, u64, u32, *const VkObjectEntryTypeNVX, *const u32) -> VkResult;
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX =
  extern "system" fn(usize, *mut VkDeviceGeneratedCommandsFeaturesNVX, *mut VkDeviceGeneratedCommandsLimitsNVX);

// feature: VK_NV_clip_space_w_scaling
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub type PFN_vkCmdSetViewportWScalingNV = extern "system" fn(usize, u32, u32, *const VkViewportWScalingNV);

// feature: VK_EXT_direct_mode_display
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub type PFN_vkReleaseDisplayEXT = extern "system" fn(usize, u64) -> VkResult;

// feature: VK_EXT_acquire_xlib_display
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
pub type PFN_vkAcquireXlibDisplayEXT = extern "system" fn(usize, *mut wsi::xlib::Display, u64) -> VkResult;
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
pub type PFN_vkGetRandROutputDisplayEXT =
  extern "system" fn(usize, *mut wsi::xlib::Display, wsi::xlib::RROutput, *mut u64) -> VkResult;

// feature: VK_EXT_display_surface_counter
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT =
  extern "system" fn(usize, u64, *mut VkSurfaceCapabilities2EXT) -> VkResult;

// feature: VK_EXT_display_control
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkDisplayPowerControlEXT = extern "system" fn(usize, u64, *const VkDisplayPowerInfoEXT) -> VkResult;
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkRegisterDeviceEventEXT =
  extern "system" fn(usize, *const VkDeviceEventInfoEXT, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkRegisterDisplayEventEXT =
  extern "system" fn(usize, u64, *const VkDisplayEventInfoEXT, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkGetSwapchainCounterEXT =
  extern "system" fn(usize, u64, VkSurfaceCounterFlagBitsEXT, *mut u64) -> VkResult;

// feature: VK_GOOGLE_display_timing
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub type PFN_vkGetRefreshCycleDurationGOOGLE =
  extern "system" fn(usize, u64, *mut VkRefreshCycleDurationGOOGLE) -> VkResult;
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub type PFN_vkGetPastPresentationTimingGOOGLE =
  extern "system" fn(usize, u64, *mut u32, *mut VkPastPresentationTimingGOOGLE) -> VkResult;

// feature: VK_EXT_discard_rectangles
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub type PFN_vkCmdSetDiscardRectangleEXT = extern "system" fn(usize, u32, u32, *const VkRect2D);

// feature: VK_EXT_hdr_metadata
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub type PFN_vkSetHdrMetadataEXT = extern "system" fn(usize, u32, *const u64, *const VkHdrMetadataEXT);

// feature: VK_KHR_get_surface_capabilities2
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR =
  extern "system" fn(usize, *const VkPhysicalDeviceSurfaceInfo2KHR, *mut VkSurfaceCapabilities2KHR) -> VkResult;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR =
  extern "system" fn(usize, *const VkPhysicalDeviceSurfaceInfo2KHR, *mut u32, *mut VkSurfaceFormat2KHR) -> VkResult;

// feature: VK_KHR_shared_presentable_image
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub type PFN_vkGetSwapchainStatusKHR = extern "system" fn(usize, u64) -> VkResult;

// feature: VK_KHR_external_fence_capabilities
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
  extern "system" fn(usize, *const VkPhysicalDeviceExternalFenceInfoKHR, *mut VkExternalFencePropertiesKHR);

// feature: VK_KHR_external_fence_win32
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkImportFenceWin32HandleKHR =
  extern "system" fn(usize, *const VkImportFenceWin32HandleInfoKHR) -> VkResult;
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type PFN_vkGetFenceWin32HandleKHR =
  extern "system" fn(usize, *const VkFenceGetWin32HandleInfoKHR, *mut wsi::win32::HANDLE) -> VkResult;

// feature: VK_KHR_external_fence_fd
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub type PFN_vkImportFenceFdKHR = extern "system" fn(usize, *const VkImportFenceFdInfoKHR) -> VkResult;
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub type PFN_vkGetFenceFdKHR = extern "system" fn(usize, *const VkFenceGetFdInfoKHR, *mut c_int) -> VkResult;

// feature: VK_MVK_ios_surface
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub type PFN_vkCreateIOSSurfaceMVK =
  extern "system" fn(usize, *const VkIOSSurfaceCreateInfoMVK, *const VkAllocationCallbacks, *mut u64) -> VkResult;

// feature: VK_MVK_macos_surface
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub type PFN_vkCreateMacOSSurfaceMVK =
  extern "system" fn(usize, *const VkMacOSSurfaceCreateInfoMVK, *const VkAllocationCallbacks, *mut u64) -> VkResult;

// feature: VK_KHR_get_memory_requirements2
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetImageMemoryRequirements2KHR =
  extern "system" fn(usize, *const VkImageMemoryRequirementsInfo2KHR, *mut VkMemoryRequirements2KHR);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetBufferMemoryRequirements2KHR =
  extern "system" fn(usize, *const VkBufferMemoryRequirementsInfo2KHR, *mut VkMemoryRequirements2KHR);
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetImageSparseMemoryRequirements2KHR =
  extern "system" fn(
    usize,
    *const VkImageSparseMemoryRequirementsInfo2KHR,
    *mut u32,
    *mut VkSparseImageMemoryRequirements2KHR,
  );

// feature: VK_EXT_sample_locations
#[cfg(feature = "VK_EXT_sample_locations")]
pub type PFN_vkCmdSetSampleLocationsEXT = extern "system" fn(usize, *const VkSampleLocationsInfoEXT);
#[cfg(feature = "VK_EXT_sample_locations")]
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT =
  extern "system" fn(usize, VkSampleCountFlagBits, *mut VkMultisamplePropertiesEXT);

// feature: VK_KHR_bind_memory2
#[cfg(feature = "VK_KHR_bind_memory2")]
pub type PFN_vkBindBufferMemory2KHR = extern "system" fn(usize, u32, *const VkBindBufferMemoryInfoKHR) -> VkResult;
#[cfg(feature = "VK_KHR_bind_memory2")]
pub type PFN_vkBindImageMemory2KHR = extern "system" fn(usize, u32, *const VkBindImageMemoryInfoKHR) -> VkResult;

// feature: VK_KHR_sampler_ycbcr_conversion
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type PFN_vkCreateSamplerYcbcrConversionKHR =
  extern "system" fn(usize, *const VkSamplerYcbcrConversionCreateInfoKHR, *const VkAllocationCallbacks, *mut u64)
    -> VkResult;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type PFN_vkDestroySamplerYcbcrConversionKHR = extern "system" fn(usize, u64, *const VkAllocationCallbacks);

// feature: VK_EXT_validation_cache
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkCreateValidationCacheEXT =
  extern "system" fn(usize, *const VkValidationCacheCreateInfoEXT, *const VkAllocationCallbacks, *mut u64) -> VkResult;
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkDestroyValidationCacheEXT = extern "system" fn(usize, u64, *const VkAllocationCallbacks);
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkMergeValidationCachesEXT = extern "system" fn(usize, u64, u32, *const u64) -> VkResult;
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkGetValidationCacheDataEXT = extern "system" fn(usize, u64, *mut usize, *mut c_void) -> VkResult;

// feature: VK_EXT_external_memory_host
#[cfg(feature = "VK_EXT_external_memory_host")]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = extern "system" fn(
  usize,
  VkExternalMemoryHandleTypeFlagBitsKHR,
  *const c_void,
  *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult;

// feature: VK_AMD_buffer_marker
#[cfg(feature = "VK_AMD_buffer_marker")]
pub type PFN_vkCmdWriteBufferMarkerAMD = extern "system" fn(usize, VkPipelineStageFlagBits, u64, VkDeviceSize, u32);
