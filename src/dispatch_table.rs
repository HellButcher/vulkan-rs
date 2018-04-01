/* GENERATED FILE */

use protos::*;
use types_raw::PFN_vkVoidFunction;

#[allow(non_snake_case)]
pub struct VkLoaderDispatchTable {
  // feature: VK_VERSION_1_0
  pub vkCreateInstance: Option<PFN_vkCreateInstance>,
  pub vkEnumerateInstanceExtensionProperties: Option<PFN_vkEnumerateInstanceExtensionProperties>,
  pub vkEnumerateInstanceLayerProperties: Option<PFN_vkEnumerateInstanceLayerProperties>,
}

#[allow(non_snake_case)]
pub struct VkInstanceDispatchTable {
  // feature: VK_VERSION_1_0
  pub vkDestroyInstance: Option<PFN_vkDestroyInstance>,
  pub vkEnumeratePhysicalDevices: Option<PFN_vkEnumeratePhysicalDevices>,
  pub vkGetPhysicalDeviceFeatures: Option<PFN_vkGetPhysicalDeviceFeatures>,
  pub vkGetPhysicalDeviceFormatProperties: Option<PFN_vkGetPhysicalDeviceFormatProperties>,
  pub vkGetPhysicalDeviceImageFormatProperties: Option<PFN_vkGetPhysicalDeviceImageFormatProperties>,
  pub vkGetPhysicalDeviceProperties: Option<PFN_vkGetPhysicalDeviceProperties>,
  pub vkGetPhysicalDeviceQueueFamilyProperties: Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties>,
  pub vkGetPhysicalDeviceMemoryProperties: Option<PFN_vkGetPhysicalDeviceMemoryProperties>,
  pub vkCreateDevice: Option<PFN_vkCreateDevice>,
  pub vkEnumerateDeviceExtensionProperties: Option<PFN_vkEnumerateDeviceExtensionProperties>,
  pub vkEnumerateDeviceLayerProperties: Option<PFN_vkEnumerateDeviceLayerProperties>,
  pub vkGetPhysicalDeviceSparseImageFormatProperties: Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties>,

  // feature: VK_KHR_surface
  #[cfg(feature = "VK_KHR_surface")]
  pub vkDestroySurfaceKHR: Option<PFN_vkDestroySurfaceKHR>,
  #[cfg(feature = "VK_KHR_surface")]
  pub vkGetPhysicalDeviceSurfaceSupportKHR: Option<PFN_vkGetPhysicalDeviceSurfaceSupportKHR>,
  #[cfg(feature = "VK_KHR_surface")]
  pub vkGetPhysicalDeviceSurfaceCapabilitiesKHR: Option<PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
  #[cfg(feature = "VK_KHR_surface")]
  pub vkGetPhysicalDeviceSurfaceFormatsKHR: Option<PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
  #[cfg(feature = "VK_KHR_surface")]
  pub vkGetPhysicalDeviceSurfacePresentModesKHR: Option<PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,

  // feature: VK_KHR_display
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetPhysicalDeviceDisplayPropertiesKHR: Option<PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetPhysicalDeviceDisplayPlanePropertiesKHR: Option<PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetDisplayPlaneSupportedDisplaysKHR: Option<PFN_vkGetDisplayPlaneSupportedDisplaysKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetDisplayModePropertiesKHR: Option<PFN_vkGetDisplayModePropertiesKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkCreateDisplayModeKHR: Option<PFN_vkCreateDisplayModeKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetDisplayPlaneCapabilitiesKHR: Option<PFN_vkGetDisplayPlaneCapabilitiesKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkCreateDisplayPlaneSurfaceKHR: Option<PFN_vkCreateDisplayPlaneSurfaceKHR>,

  // feature: VK_KHR_xlib_surface
  #[cfg(feature = "VK_KHR_xlib_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
  pub vkCreateXlibSurfaceKHR: Option<PFN_vkCreateXlibSurfaceKHR>,
  #[cfg(feature = "VK_KHR_xlib_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
  pub vkGetPhysicalDeviceXlibPresentationSupportKHR: Option<PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR>,

  // feature: VK_KHR_xcb_surface
  #[cfg(feature = "VK_KHR_xcb_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
  pub vkCreateXcbSurfaceKHR: Option<PFN_vkCreateXcbSurfaceKHR>,
  #[cfg(feature = "VK_KHR_xcb_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
  pub vkGetPhysicalDeviceXcbPresentationSupportKHR: Option<PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>,

  // feature: VK_KHR_wayland_surface
  #[cfg(feature = "VK_KHR_wayland_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
  pub vkCreateWaylandSurfaceKHR: Option<PFN_vkCreateWaylandSurfaceKHR>,
  #[cfg(feature = "VK_KHR_wayland_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
  pub vkGetPhysicalDeviceWaylandPresentationSupportKHR: Option<PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>,

  // feature: VK_KHR_mir_surface
  #[cfg(feature = "VK_KHR_mir_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
  pub vkCreateMirSurfaceKHR: Option<PFN_vkCreateMirSurfaceKHR>,
  #[cfg(feature = "VK_KHR_mir_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
  pub vkGetPhysicalDeviceMirPresentationSupportKHR: Option<PFN_vkGetPhysicalDeviceMirPresentationSupportKHR>,

  // feature: VK_KHR_android_surface
  #[cfg(feature = "VK_KHR_android_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
  pub vkCreateAndroidSurfaceKHR: Option<PFN_vkCreateAndroidSurfaceKHR>,

  // feature: VK_KHR_win32_surface
  #[cfg(feature = "VK_KHR_win32_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkCreateWin32SurfaceKHR: Option<PFN_vkCreateWin32SurfaceKHR>,
  #[cfg(feature = "VK_KHR_win32_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkGetPhysicalDeviceWin32PresentationSupportKHR: Option<PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR>,

  // feature: VK_EXT_debug_report
  #[cfg(feature = "VK_EXT_debug_report")]
  pub vkCreateDebugReportCallbackEXT: Option<PFN_vkCreateDebugReportCallbackEXT>,
  #[cfg(feature = "VK_EXT_debug_report")]
  pub vkDestroyDebugReportCallbackEXT: Option<PFN_vkDestroyDebugReportCallbackEXT>,
  #[cfg(feature = "VK_EXT_debug_report")]
  pub vkDebugReportMessageEXT: Option<PFN_vkDebugReportMessageEXT>,

  // feature: VK_KHR_get_physical_device_properties2
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceFeatures2KHR: Option<PFN_vkGetPhysicalDeviceFeatures2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceProperties2KHR: Option<PFN_vkGetPhysicalDeviceProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceFormatProperties2KHR: Option<PFN_vkGetPhysicalDeviceFormatProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceImageFormatProperties2KHR: Option<PFN_vkGetPhysicalDeviceImageFormatProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceQueueFamilyProperties2KHR: Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceMemoryProperties2KHR: Option<PFN_vkGetPhysicalDeviceMemoryProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceSparseImageFormatProperties2KHR:
    Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR>,

  // feature: VK_NV_external_memory_capabilities
  #[cfg(feature = "VK_NV_external_memory_capabilities")]
  pub vkGetPhysicalDeviceExternalImageFormatPropertiesNV:
    Option<PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV>,

  // feature: VK_KHX_device_group_creation
  #[cfg(feature = "VK_KHX_device_group_creation")]
  pub vkEnumeratePhysicalDeviceGroupsKHX: Option<PFN_vkEnumeratePhysicalDeviceGroupsKHX>,

  // feature: VK_KHX_device_group
  #[cfg(feature = "VK_KHX_device_group")]
  pub vkGetPhysicalDevicePresentRectanglesKHX: Option<PFN_vkGetPhysicalDevicePresentRectanglesKHX>,

  // feature: VK_NN_vi_surface
  #[cfg(feature = "VK_NN_vi_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
  pub vkCreateViSurfaceNN: Option<PFN_vkCreateViSurfaceNN>,

  // feature: VK_KHR_external_memory_capabilities
  #[cfg(feature = "VK_KHR_external_memory_capabilities")]
  pub vkGetPhysicalDeviceExternalBufferPropertiesKHR: Option<PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR>,

  // feature: VK_KHR_external_semaphore_capabilities
  #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
  pub vkGetPhysicalDeviceExternalSemaphorePropertiesKHR: Option<PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR>,

  // feature: VK_NVX_device_generated_commands
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX: Option<PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX>,

  // feature: VK_EXT_direct_mode_display
  #[cfg(feature = "VK_EXT_direct_mode_display")]
  pub vkReleaseDisplayEXT: Option<PFN_vkReleaseDisplayEXT>,

  // feature: VK_EXT_acquire_xlib_display
  #[cfg(feature = "VK_EXT_acquire_xlib_display")]
  #[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
  pub vkAcquireXlibDisplayEXT: Option<PFN_vkAcquireXlibDisplayEXT>,
  #[cfg(feature = "VK_EXT_acquire_xlib_display")]
  #[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
  pub vkGetRandROutputDisplayEXT: Option<PFN_vkGetRandROutputDisplayEXT>,

  // feature: VK_EXT_display_surface_counter
  #[cfg(feature = "VK_EXT_display_surface_counter")]
  pub vkGetPhysicalDeviceSurfaceCapabilities2EXT: Option<PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT>,

  // feature: VK_KHR_get_surface_capabilities2
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  pub vkGetPhysicalDeviceSurfaceCapabilities2KHR: Option<PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  pub vkGetPhysicalDeviceSurfaceFormats2KHR: Option<PFN_vkGetPhysicalDeviceSurfaceFormats2KHR>,

  // feature: VK_KHR_external_fence_capabilities
  #[cfg(feature = "VK_KHR_external_fence_capabilities")]
  pub vkGetPhysicalDeviceExternalFencePropertiesKHR: Option<PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>,

  // feature: VK_MVK_ios_surface
  #[cfg(feature = "VK_MVK_ios_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
  pub vkCreateIOSSurfaceMVK: Option<PFN_vkCreateIOSSurfaceMVK>,

  // feature: VK_MVK_macos_surface
  #[cfg(feature = "VK_MVK_macos_surface")]
  #[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
  pub vkCreateMacOSSurfaceMVK: Option<PFN_vkCreateMacOSSurfaceMVK>,

  // feature: VK_EXT_sample_locations
  #[cfg(feature = "VK_EXT_sample_locations")]
  pub vkGetPhysicalDeviceMultisamplePropertiesEXT: Option<PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>,
}

#[allow(non_snake_case)]
pub struct VkDeviceDispatchTable {
  // feature: VK_VERSION_1_0
  pub vkDestroyDevice: Option<PFN_vkDestroyDevice>,
  pub vkGetDeviceQueue: Option<PFN_vkGetDeviceQueue>,
  pub vkQueueSubmit: Option<PFN_vkQueueSubmit>,
  pub vkQueueWaitIdle: Option<PFN_vkQueueWaitIdle>,
  pub vkDeviceWaitIdle: Option<PFN_vkDeviceWaitIdle>,
  pub vkAllocateMemory: Option<PFN_vkAllocateMemory>,
  pub vkFreeMemory: Option<PFN_vkFreeMemory>,
  pub vkMapMemory: Option<PFN_vkMapMemory>,
  pub vkUnmapMemory: Option<PFN_vkUnmapMemory>,
  pub vkFlushMappedMemoryRanges: Option<PFN_vkFlushMappedMemoryRanges>,
  pub vkInvalidateMappedMemoryRanges: Option<PFN_vkInvalidateMappedMemoryRanges>,
  pub vkGetDeviceMemoryCommitment: Option<PFN_vkGetDeviceMemoryCommitment>,
  pub vkBindBufferMemory: Option<PFN_vkBindBufferMemory>,
  pub vkBindImageMemory: Option<PFN_vkBindImageMemory>,
  pub vkGetBufferMemoryRequirements: Option<PFN_vkGetBufferMemoryRequirements>,
  pub vkGetImageMemoryRequirements: Option<PFN_vkGetImageMemoryRequirements>,
  pub vkGetImageSparseMemoryRequirements: Option<PFN_vkGetImageSparseMemoryRequirements>,
  pub vkQueueBindSparse: Option<PFN_vkQueueBindSparse>,
  pub vkCreateFence: Option<PFN_vkCreateFence>,
  pub vkDestroyFence: Option<PFN_vkDestroyFence>,
  pub vkResetFences: Option<PFN_vkResetFences>,
  pub vkGetFenceStatus: Option<PFN_vkGetFenceStatus>,
  pub vkWaitForFences: Option<PFN_vkWaitForFences>,
  pub vkCreateSemaphore: Option<PFN_vkCreateSemaphore>,
  pub vkDestroySemaphore: Option<PFN_vkDestroySemaphore>,
  pub vkCreateEvent: Option<PFN_vkCreateEvent>,
  pub vkDestroyEvent: Option<PFN_vkDestroyEvent>,
  pub vkGetEventStatus: Option<PFN_vkGetEventStatus>,
  pub vkSetEvent: Option<PFN_vkSetEvent>,
  pub vkResetEvent: Option<PFN_vkResetEvent>,
  pub vkCreateQueryPool: Option<PFN_vkCreateQueryPool>,
  pub vkDestroyQueryPool: Option<PFN_vkDestroyQueryPool>,
  pub vkGetQueryPoolResults: Option<PFN_vkGetQueryPoolResults>,
  pub vkCreateBuffer: Option<PFN_vkCreateBuffer>,
  pub vkDestroyBuffer: Option<PFN_vkDestroyBuffer>,
  pub vkCreateBufferView: Option<PFN_vkCreateBufferView>,
  pub vkDestroyBufferView: Option<PFN_vkDestroyBufferView>,
  pub vkCreateImage: Option<PFN_vkCreateImage>,
  pub vkDestroyImage: Option<PFN_vkDestroyImage>,
  pub vkGetImageSubresourceLayout: Option<PFN_vkGetImageSubresourceLayout>,
  pub vkCreateImageView: Option<PFN_vkCreateImageView>,
  pub vkDestroyImageView: Option<PFN_vkDestroyImageView>,
  pub vkCreateShaderModule: Option<PFN_vkCreateShaderModule>,
  pub vkDestroyShaderModule: Option<PFN_vkDestroyShaderModule>,
  pub vkCreatePipelineCache: Option<PFN_vkCreatePipelineCache>,
  pub vkDestroyPipelineCache: Option<PFN_vkDestroyPipelineCache>,
  pub vkGetPipelineCacheData: Option<PFN_vkGetPipelineCacheData>,
  pub vkMergePipelineCaches: Option<PFN_vkMergePipelineCaches>,
  pub vkCreateGraphicsPipelines: Option<PFN_vkCreateGraphicsPipelines>,
  pub vkCreateComputePipelines: Option<PFN_vkCreateComputePipelines>,
  pub vkDestroyPipeline: Option<PFN_vkDestroyPipeline>,
  pub vkCreatePipelineLayout: Option<PFN_vkCreatePipelineLayout>,
  pub vkDestroyPipelineLayout: Option<PFN_vkDestroyPipelineLayout>,
  pub vkCreateSampler: Option<PFN_vkCreateSampler>,
  pub vkDestroySampler: Option<PFN_vkDestroySampler>,
  pub vkCreateDescriptorSetLayout: Option<PFN_vkCreateDescriptorSetLayout>,
  pub vkDestroyDescriptorSetLayout: Option<PFN_vkDestroyDescriptorSetLayout>,
  pub vkCreateDescriptorPool: Option<PFN_vkCreateDescriptorPool>,
  pub vkDestroyDescriptorPool: Option<PFN_vkDestroyDescriptorPool>,
  pub vkResetDescriptorPool: Option<PFN_vkResetDescriptorPool>,
  pub vkAllocateDescriptorSets: Option<PFN_vkAllocateDescriptorSets>,
  pub vkFreeDescriptorSets: Option<PFN_vkFreeDescriptorSets>,
  pub vkUpdateDescriptorSets: Option<PFN_vkUpdateDescriptorSets>,
  pub vkCreateFramebuffer: Option<PFN_vkCreateFramebuffer>,
  pub vkDestroyFramebuffer: Option<PFN_vkDestroyFramebuffer>,
  pub vkCreateRenderPass: Option<PFN_vkCreateRenderPass>,
  pub vkDestroyRenderPass: Option<PFN_vkDestroyRenderPass>,
  pub vkGetRenderAreaGranularity: Option<PFN_vkGetRenderAreaGranularity>,
  pub vkCreateCommandPool: Option<PFN_vkCreateCommandPool>,
  pub vkDestroyCommandPool: Option<PFN_vkDestroyCommandPool>,
  pub vkResetCommandPool: Option<PFN_vkResetCommandPool>,
  pub vkAllocateCommandBuffers: Option<PFN_vkAllocateCommandBuffers>,
  pub vkFreeCommandBuffers: Option<PFN_vkFreeCommandBuffers>,
  pub vkBeginCommandBuffer: Option<PFN_vkBeginCommandBuffer>,
  pub vkEndCommandBuffer: Option<PFN_vkEndCommandBuffer>,
  pub vkResetCommandBuffer: Option<PFN_vkResetCommandBuffer>,
  pub vkCmdBindPipeline: Option<PFN_vkCmdBindPipeline>,
  pub vkCmdSetViewport: Option<PFN_vkCmdSetViewport>,
  pub vkCmdSetScissor: Option<PFN_vkCmdSetScissor>,
  pub vkCmdSetLineWidth: Option<PFN_vkCmdSetLineWidth>,
  pub vkCmdSetDepthBias: Option<PFN_vkCmdSetDepthBias>,
  pub vkCmdSetBlendConstants: Option<PFN_vkCmdSetBlendConstants>,
  pub vkCmdSetDepthBounds: Option<PFN_vkCmdSetDepthBounds>,
  pub vkCmdSetStencilCompareMask: Option<PFN_vkCmdSetStencilCompareMask>,
  pub vkCmdSetStencilWriteMask: Option<PFN_vkCmdSetStencilWriteMask>,
  pub vkCmdSetStencilReference: Option<PFN_vkCmdSetStencilReference>,
  pub vkCmdBindDescriptorSets: Option<PFN_vkCmdBindDescriptorSets>,
  pub vkCmdBindIndexBuffer: Option<PFN_vkCmdBindIndexBuffer>,
  pub vkCmdBindVertexBuffers: Option<PFN_vkCmdBindVertexBuffers>,
  pub vkCmdDraw: Option<PFN_vkCmdDraw>,
  pub vkCmdDrawIndexed: Option<PFN_vkCmdDrawIndexed>,
  pub vkCmdDrawIndirect: Option<PFN_vkCmdDrawIndirect>,
  pub vkCmdDrawIndexedIndirect: Option<PFN_vkCmdDrawIndexedIndirect>,
  pub vkCmdDispatch: Option<PFN_vkCmdDispatch>,
  pub vkCmdDispatchIndirect: Option<PFN_vkCmdDispatchIndirect>,
  pub vkCmdCopyBuffer: Option<PFN_vkCmdCopyBuffer>,
  pub vkCmdCopyImage: Option<PFN_vkCmdCopyImage>,
  pub vkCmdBlitImage: Option<PFN_vkCmdBlitImage>,
  pub vkCmdCopyBufferToImage: Option<PFN_vkCmdCopyBufferToImage>,
  pub vkCmdCopyImageToBuffer: Option<PFN_vkCmdCopyImageToBuffer>,
  pub vkCmdUpdateBuffer: Option<PFN_vkCmdUpdateBuffer>,
  pub vkCmdFillBuffer: Option<PFN_vkCmdFillBuffer>,
  pub vkCmdClearColorImage: Option<PFN_vkCmdClearColorImage>,
  pub vkCmdClearDepthStencilImage: Option<PFN_vkCmdClearDepthStencilImage>,
  pub vkCmdClearAttachments: Option<PFN_vkCmdClearAttachments>,
  pub vkCmdResolveImage: Option<PFN_vkCmdResolveImage>,
  pub vkCmdSetEvent: Option<PFN_vkCmdSetEvent>,
  pub vkCmdResetEvent: Option<PFN_vkCmdResetEvent>,
  pub vkCmdWaitEvents: Option<PFN_vkCmdWaitEvents>,
  pub vkCmdPipelineBarrier: Option<PFN_vkCmdPipelineBarrier>,
  pub vkCmdBeginQuery: Option<PFN_vkCmdBeginQuery>,
  pub vkCmdEndQuery: Option<PFN_vkCmdEndQuery>,
  pub vkCmdResetQueryPool: Option<PFN_vkCmdResetQueryPool>,
  pub vkCmdWriteTimestamp: Option<PFN_vkCmdWriteTimestamp>,
  pub vkCmdCopyQueryPoolResults: Option<PFN_vkCmdCopyQueryPoolResults>,
  pub vkCmdPushConstants: Option<PFN_vkCmdPushConstants>,
  pub vkCmdBeginRenderPass: Option<PFN_vkCmdBeginRenderPass>,
  pub vkCmdNextSubpass: Option<PFN_vkCmdNextSubpass>,
  pub vkCmdEndRenderPass: Option<PFN_vkCmdEndRenderPass>,
  pub vkCmdExecuteCommands: Option<PFN_vkCmdExecuteCommands>,

  // feature: VK_KHR_swapchain
  #[cfg(feature = "VK_KHR_swapchain")]
  pub vkCreateSwapchainKHR: Option<PFN_vkCreateSwapchainKHR>,
  #[cfg(feature = "VK_KHR_swapchain")]
  pub vkDestroySwapchainKHR: Option<PFN_vkDestroySwapchainKHR>,
  #[cfg(feature = "VK_KHR_swapchain")]
  pub vkGetSwapchainImagesKHR: Option<PFN_vkGetSwapchainImagesKHR>,
  #[cfg(feature = "VK_KHR_swapchain")]
  pub vkAcquireNextImageKHR: Option<PFN_vkAcquireNextImageKHR>,
  #[cfg(feature = "VK_KHR_swapchain")]
  pub vkQueuePresentKHR: Option<PFN_vkQueuePresentKHR>,

  // feature: VK_KHR_display_swapchain
  #[cfg(feature = "VK_KHR_display_swapchain")]
  pub vkCreateSharedSwapchainsKHR: Option<PFN_vkCreateSharedSwapchainsKHR>,

  // feature: VK_EXT_debug_marker
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkDebugMarkerSetObjectTagEXT: Option<PFN_vkDebugMarkerSetObjectTagEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkDebugMarkerSetObjectNameEXT: Option<PFN_vkDebugMarkerSetObjectNameEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkCmdDebugMarkerBeginEXT: Option<PFN_vkCmdDebugMarkerBeginEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkCmdDebugMarkerEndEXT: Option<PFN_vkCmdDebugMarkerEndEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkCmdDebugMarkerInsertEXT: Option<PFN_vkCmdDebugMarkerInsertEXT>,

  // feature: VK_AMD_draw_indirect_count
  #[cfg(feature = "VK_AMD_draw_indirect_count")]
  pub vkCmdDrawIndirectCountAMD: Option<PFN_vkCmdDrawIndirectCountAMD>,
  #[cfg(feature = "VK_AMD_draw_indirect_count")]
  pub vkCmdDrawIndexedIndirectCountAMD: Option<PFN_vkCmdDrawIndexedIndirectCountAMD>,

  // feature: VK_AMD_shader_info
  #[cfg(feature = "VK_AMD_shader_info")]
  pub vkGetShaderInfoAMD: Option<PFN_vkGetShaderInfoAMD>,

  // feature: VK_NV_external_memory_win32
  #[cfg(feature = "VK_NV_external_memory_win32")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkGetMemoryWin32HandleNV: Option<PFN_vkGetMemoryWin32HandleNV>,

  // feature: VK_KHX_device_group
  #[cfg(feature = "VK_KHX_device_group")]
  pub vkGetDeviceGroupPeerMemoryFeaturesKHX: Option<PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX>,
  #[cfg(feature = "VK_KHX_device_group")]
  pub vkCmdSetDeviceMaskKHX: Option<PFN_vkCmdSetDeviceMaskKHX>,
  #[cfg(feature = "VK_KHX_device_group")]
  pub vkCmdDispatchBaseKHX: Option<PFN_vkCmdDispatchBaseKHX>,
  #[cfg(feature = "VK_KHX_device_group")]
  pub vkGetDeviceGroupPresentCapabilitiesKHX: Option<PFN_vkGetDeviceGroupPresentCapabilitiesKHX>,
  #[cfg(feature = "VK_KHX_device_group")]
  pub vkGetDeviceGroupSurfacePresentModesKHX: Option<PFN_vkGetDeviceGroupSurfacePresentModesKHX>,
  #[cfg(feature = "VK_KHX_device_group")]
  pub vkAcquireNextImage2KHX: Option<PFN_vkAcquireNextImage2KHX>,

  // feature: VK_KHR_maintenance1
  #[cfg(feature = "VK_KHR_maintenance1")]
  pub vkTrimCommandPoolKHR: Option<PFN_vkTrimCommandPoolKHR>,

  // feature: VK_KHR_external_memory_win32
  #[cfg(feature = "VK_KHR_external_memory_win32")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkGetMemoryWin32HandleKHR: Option<PFN_vkGetMemoryWin32HandleKHR>,
  #[cfg(feature = "VK_KHR_external_memory_win32")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkGetMemoryWin32HandlePropertiesKHR: Option<PFN_vkGetMemoryWin32HandlePropertiesKHR>,

  // feature: VK_KHR_external_memory_fd
  #[cfg(feature = "VK_KHR_external_memory_fd")]
  pub vkGetMemoryFdKHR: Option<PFN_vkGetMemoryFdKHR>,
  #[cfg(feature = "VK_KHR_external_memory_fd")]
  pub vkGetMemoryFdPropertiesKHR: Option<PFN_vkGetMemoryFdPropertiesKHR>,

  // feature: VK_KHR_external_semaphore_win32
  #[cfg(feature = "VK_KHR_external_semaphore_win32")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkImportSemaphoreWin32HandleKHR: Option<PFN_vkImportSemaphoreWin32HandleKHR>,
  #[cfg(feature = "VK_KHR_external_semaphore_win32")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkGetSemaphoreWin32HandleKHR: Option<PFN_vkGetSemaphoreWin32HandleKHR>,

  // feature: VK_KHR_external_semaphore_fd
  #[cfg(feature = "VK_KHR_external_semaphore_fd")]
  pub vkImportSemaphoreFdKHR: Option<PFN_vkImportSemaphoreFdKHR>,
  #[cfg(feature = "VK_KHR_external_semaphore_fd")]
  pub vkGetSemaphoreFdKHR: Option<PFN_vkGetSemaphoreFdKHR>,

  // feature: VK_KHR_push_descriptor
  #[cfg(feature = "VK_KHR_push_descriptor")]
  pub vkCmdPushDescriptorSetKHR: Option<PFN_vkCmdPushDescriptorSetKHR>,

  // feature: VK_KHR_descriptor_update_template
  #[cfg(feature = "VK_KHR_descriptor_update_template")]
  pub vkCreateDescriptorUpdateTemplateKHR: Option<PFN_vkCreateDescriptorUpdateTemplateKHR>,
  #[cfg(feature = "VK_KHR_descriptor_update_template")]
  pub vkDestroyDescriptorUpdateTemplateKHR: Option<PFN_vkDestroyDescriptorUpdateTemplateKHR>,
  #[cfg(feature = "VK_KHR_descriptor_update_template")]
  pub vkUpdateDescriptorSetWithTemplateKHR: Option<PFN_vkUpdateDescriptorSetWithTemplateKHR>,
  #[cfg(feature = "VK_KHR_descriptor_update_template")]
  pub vkCmdPushDescriptorSetWithTemplateKHR: Option<PFN_vkCmdPushDescriptorSetWithTemplateKHR>,

  // feature: VK_NVX_device_generated_commands
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkCmdProcessCommandsNVX: Option<PFN_vkCmdProcessCommandsNVX>,
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkCmdReserveSpaceForCommandsNVX: Option<PFN_vkCmdReserveSpaceForCommandsNVX>,
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkCreateIndirectCommandsLayoutNVX: Option<PFN_vkCreateIndirectCommandsLayoutNVX>,
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkDestroyIndirectCommandsLayoutNVX: Option<PFN_vkDestroyIndirectCommandsLayoutNVX>,
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkCreateObjectTableNVX: Option<PFN_vkCreateObjectTableNVX>,
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkDestroyObjectTableNVX: Option<PFN_vkDestroyObjectTableNVX>,
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkRegisterObjectsNVX: Option<PFN_vkRegisterObjectsNVX>,
  #[cfg(feature = "VK_NVX_device_generated_commands")]
  pub vkUnregisterObjectsNVX: Option<PFN_vkUnregisterObjectsNVX>,

  // feature: VK_NV_clip_space_w_scaling
  #[cfg(feature = "VK_NV_clip_space_w_scaling")]
  pub vkCmdSetViewportWScalingNV: Option<PFN_vkCmdSetViewportWScalingNV>,

  // feature: VK_EXT_display_control
  #[cfg(feature = "VK_EXT_display_control")]
  pub vkDisplayPowerControlEXT: Option<PFN_vkDisplayPowerControlEXT>,
  #[cfg(feature = "VK_EXT_display_control")]
  pub vkRegisterDeviceEventEXT: Option<PFN_vkRegisterDeviceEventEXT>,
  #[cfg(feature = "VK_EXT_display_control")]
  pub vkRegisterDisplayEventEXT: Option<PFN_vkRegisterDisplayEventEXT>,
  #[cfg(feature = "VK_EXT_display_control")]
  pub vkGetSwapchainCounterEXT: Option<PFN_vkGetSwapchainCounterEXT>,

  // feature: VK_GOOGLE_display_timing
  #[cfg(feature = "VK_GOOGLE_display_timing")]
  pub vkGetRefreshCycleDurationGOOGLE: Option<PFN_vkGetRefreshCycleDurationGOOGLE>,
  #[cfg(feature = "VK_GOOGLE_display_timing")]
  pub vkGetPastPresentationTimingGOOGLE: Option<PFN_vkGetPastPresentationTimingGOOGLE>,

  // feature: VK_EXT_discard_rectangles
  #[cfg(feature = "VK_EXT_discard_rectangles")]
  pub vkCmdSetDiscardRectangleEXT: Option<PFN_vkCmdSetDiscardRectangleEXT>,

  // feature: VK_EXT_hdr_metadata
  #[cfg(feature = "VK_EXT_hdr_metadata")]
  pub vkSetHdrMetadataEXT: Option<PFN_vkSetHdrMetadataEXT>,

  // feature: VK_KHR_shared_presentable_image
  #[cfg(feature = "VK_KHR_shared_presentable_image")]
  pub vkGetSwapchainStatusKHR: Option<PFN_vkGetSwapchainStatusKHR>,

  // feature: VK_KHR_external_fence_win32
  #[cfg(feature = "VK_KHR_external_fence_win32")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkImportFenceWin32HandleKHR: Option<PFN_vkImportFenceWin32HandleKHR>,
  #[cfg(feature = "VK_KHR_external_fence_win32")]
  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub vkGetFenceWin32HandleKHR: Option<PFN_vkGetFenceWin32HandleKHR>,

  // feature: VK_KHR_external_fence_fd
  #[cfg(feature = "VK_KHR_external_fence_fd")]
  pub vkImportFenceFdKHR: Option<PFN_vkImportFenceFdKHR>,
  #[cfg(feature = "VK_KHR_external_fence_fd")]
  pub vkGetFenceFdKHR: Option<PFN_vkGetFenceFdKHR>,

  // feature: VK_KHR_get_memory_requirements2
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  pub vkGetImageMemoryRequirements2KHR: Option<PFN_vkGetImageMemoryRequirements2KHR>,
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  pub vkGetBufferMemoryRequirements2KHR: Option<PFN_vkGetBufferMemoryRequirements2KHR>,
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  pub vkGetImageSparseMemoryRequirements2KHR: Option<PFN_vkGetImageSparseMemoryRequirements2KHR>,

  // feature: VK_EXT_sample_locations
  #[cfg(feature = "VK_EXT_sample_locations")]
  pub vkCmdSetSampleLocationsEXT: Option<PFN_vkCmdSetSampleLocationsEXT>,

  // feature: VK_KHR_bind_memory2
  #[cfg(feature = "VK_KHR_bind_memory2")]
  pub vkBindBufferMemory2KHR: Option<PFN_vkBindBufferMemory2KHR>,
  #[cfg(feature = "VK_KHR_bind_memory2")]
  pub vkBindImageMemory2KHR: Option<PFN_vkBindImageMemory2KHR>,

  // feature: VK_KHR_sampler_ycbcr_conversion
  #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
  pub vkCreateSamplerYcbcrConversionKHR: Option<PFN_vkCreateSamplerYcbcrConversionKHR>,
  #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
  pub vkDestroySamplerYcbcrConversionKHR: Option<PFN_vkDestroySamplerYcbcrConversionKHR>,

  // feature: VK_EXT_validation_cache
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub vkCreateValidationCacheEXT: Option<PFN_vkCreateValidationCacheEXT>,
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub vkDestroyValidationCacheEXT: Option<PFN_vkDestroyValidationCacheEXT>,
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub vkMergeValidationCachesEXT: Option<PFN_vkMergeValidationCachesEXT>,
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub vkGetValidationCacheDataEXT: Option<PFN_vkGetValidationCacheDataEXT>,

  // feature: VK_EXT_external_memory_host
  #[cfg(feature = "VK_EXT_external_memory_host")]
  pub vkGetMemoryHostPointerPropertiesEXT: Option<PFN_vkGetMemoryHostPointerPropertiesEXT>,

  // feature: VK_AMD_buffer_marker
  #[cfg(feature = "VK_AMD_buffer_marker")]
  pub vkCmdWriteBufferMarkerAMD: Option<PFN_vkCmdWriteBufferMarkerAMD>,
}

impl VkLoaderDispatchTable {
  pub unsafe fn load<R, F1>(gpa: F1) -> Result<VkLoaderDispatchTable, R>
  where
    F1: Fn(&str) -> Result<PFN_vkVoidFunction, R>,
  {
    use std::mem::transmute as tm;
    let mut tab: VkLoaderDispatchTable = ::std::mem::zeroed();
    // feature: VK_VERSION_1_0
    tab.vkCreateInstance = tm(gpa("vkCreateInstance\0")?);
    tab.vkEnumerateInstanceExtensionProperties = tm(gpa("vkEnumerateInstanceExtensionProperties\0")?);
    tab.vkEnumerateInstanceLayerProperties = tm(gpa("vkEnumerateInstanceLayerProperties\0")?);
    Ok(tab)
  }
}

impl VkInstanceDispatchTable {
  pub unsafe fn load<R, F1, F2>(gpa: F1, has_ext: F2) -> Result<VkInstanceDispatchTable, R>
  where
    F1: Fn(&str) -> Result<PFN_vkVoidFunction, R>,
    F2: Fn(&str) -> bool,
  {
    use std::mem::transmute as tm;
    let mut tab: VkInstanceDispatchTable = ::std::mem::zeroed();
    // feature: VK_VERSION_1_0
    tab.vkDestroyInstance = tm(gpa("vkDestroyInstance\0")?);
    tab.vkEnumeratePhysicalDevices = tm(gpa("vkEnumeratePhysicalDevices\0")?);
    tab.vkGetPhysicalDeviceFeatures = tm(gpa("vkGetPhysicalDeviceFeatures\0")?);
    tab.vkGetPhysicalDeviceFormatProperties = tm(gpa("vkGetPhysicalDeviceFormatProperties\0")?);
    tab.vkGetPhysicalDeviceImageFormatProperties = tm(gpa("vkGetPhysicalDeviceImageFormatProperties\0")?);
    tab.vkGetPhysicalDeviceProperties = tm(gpa("vkGetPhysicalDeviceProperties\0")?);
    tab.vkGetPhysicalDeviceQueueFamilyProperties = tm(gpa("vkGetPhysicalDeviceQueueFamilyProperties\0")?);
    tab.vkGetPhysicalDeviceMemoryProperties = tm(gpa("vkGetPhysicalDeviceMemoryProperties\0")?);
    tab.vkCreateDevice = tm(gpa("vkCreateDevice\0")?);
    tab.vkEnumerateDeviceExtensionProperties = tm(gpa("vkEnumerateDeviceExtensionProperties\0")?);
    tab.vkEnumerateDeviceLayerProperties = tm(gpa("vkEnumerateDeviceLayerProperties\0")?);
    tab.vkGetPhysicalDeviceSparseImageFormatProperties = tm(gpa("vkGetPhysicalDeviceSparseImageFormatProperties\0")?);

    // feature: VK_KHR_surface
    #[cfg(feature = "VK_KHR_surface")]
    {
      if has_ext("VK_KHR_surface\0") {
        tab.vkDestroySurfaceKHR = tm(gpa("vkDestroySurfaceKHR\0")?);
        tab.vkGetPhysicalDeviceSurfaceSupportKHR = tm(gpa("vkGetPhysicalDeviceSurfaceSupportKHR\0")?);
        tab.vkGetPhysicalDeviceSurfaceCapabilitiesKHR = tm(gpa("vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0")?);
        tab.vkGetPhysicalDeviceSurfaceFormatsKHR = tm(gpa("vkGetPhysicalDeviceSurfaceFormatsKHR\0")?);
        tab.vkGetPhysicalDeviceSurfacePresentModesKHR = tm(gpa("vkGetPhysicalDeviceSurfacePresentModesKHR\0")?);
      }
    }

    // feature: VK_KHR_display
    #[cfg(feature = "VK_KHR_display")]
    {
      if has_ext("VK_KHR_display\0") {
        tab.vkGetPhysicalDeviceDisplayPropertiesKHR = tm(gpa("vkGetPhysicalDeviceDisplayPropertiesKHR\0")?);
        tab.vkGetPhysicalDeviceDisplayPlanePropertiesKHR = tm(gpa("vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0")?);
        tab.vkGetDisplayPlaneSupportedDisplaysKHR = tm(gpa("vkGetDisplayPlaneSupportedDisplaysKHR\0")?);
        tab.vkGetDisplayModePropertiesKHR = tm(gpa("vkGetDisplayModePropertiesKHR\0")?);
        tab.vkCreateDisplayModeKHR = tm(gpa("vkCreateDisplayModeKHR\0")?);
        tab.vkGetDisplayPlaneCapabilitiesKHR = tm(gpa("vkGetDisplayPlaneCapabilitiesKHR\0")?);
        tab.vkCreateDisplayPlaneSurfaceKHR = tm(gpa("vkCreateDisplayPlaneSurfaceKHR\0")?);
      }
    }

    // feature: VK_KHR_xlib_surface
    #[cfg(feature = "VK_KHR_xlib_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
    {
      if has_ext("VK_KHR_xlib_surface\0") {
        tab.vkCreateXlibSurfaceKHR = tm(gpa("vkCreateXlibSurfaceKHR\0")?);
        tab.vkGetPhysicalDeviceXlibPresentationSupportKHR = tm(gpa("vkGetPhysicalDeviceXlibPresentationSupportKHR\0")?);
      }
    }

    // feature: VK_KHR_xcb_surface
    #[cfg(feature = "VK_KHR_xcb_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
    {
      if has_ext("VK_KHR_xcb_surface\0") {
        tab.vkCreateXcbSurfaceKHR = tm(gpa("vkCreateXcbSurfaceKHR\0")?);
        tab.vkGetPhysicalDeviceXcbPresentationSupportKHR = tm(gpa("vkGetPhysicalDeviceXcbPresentationSupportKHR\0")?);
      }
    }

    // feature: VK_KHR_wayland_surface
    #[cfg(feature = "VK_KHR_wayland_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
    {
      if has_ext("VK_KHR_wayland_surface\0") {
        tab.vkCreateWaylandSurfaceKHR = tm(gpa("vkCreateWaylandSurfaceKHR\0")?);
        tab.vkGetPhysicalDeviceWaylandPresentationSupportKHR =
          tm(gpa("vkGetPhysicalDeviceWaylandPresentationSupportKHR\0")?);
      }
    }

    // feature: VK_KHR_mir_surface
    #[cfg(feature = "VK_KHR_mir_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
    {
      if has_ext("VK_KHR_mir_surface\0") {
        tab.vkCreateMirSurfaceKHR = tm(gpa("vkCreateMirSurfaceKHR\0")?);
        tab.vkGetPhysicalDeviceMirPresentationSupportKHR = tm(gpa("vkGetPhysicalDeviceMirPresentationSupportKHR\0")?);
      }
    }

    // feature: VK_KHR_android_surface
    #[cfg(feature = "VK_KHR_android_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
    {
      if has_ext("VK_KHR_android_surface\0") {
        tab.vkCreateAndroidSurfaceKHR = tm(gpa("vkCreateAndroidSurfaceKHR\0")?);
      }
    }

    // feature: VK_KHR_win32_surface
    #[cfg(feature = "VK_KHR_win32_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    {
      if has_ext("VK_KHR_win32_surface\0") {
        tab.vkCreateWin32SurfaceKHR = tm(gpa("vkCreateWin32SurfaceKHR\0")?);
        tab.vkGetPhysicalDeviceWin32PresentationSupportKHR =
          tm(gpa("vkGetPhysicalDeviceWin32PresentationSupportKHR\0")?);
      }
    }

    // feature: VK_EXT_debug_report
    #[cfg(feature = "VK_EXT_debug_report")]
    {
      if has_ext("VK_EXT_debug_report\0") {
        tab.vkCreateDebugReportCallbackEXT = tm(gpa("vkCreateDebugReportCallbackEXT\0")?);
        tab.vkDestroyDebugReportCallbackEXT = tm(gpa("vkDestroyDebugReportCallbackEXT\0")?);
        tab.vkDebugReportMessageEXT = tm(gpa("vkDebugReportMessageEXT\0")?);
      }
    }

    // feature: VK_KHR_get_physical_device_properties2
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    {
      if has_ext("VK_KHR_get_physical_device_properties2\0") {
        tab.vkGetPhysicalDeviceFeatures2KHR = tm(gpa("vkGetPhysicalDeviceFeatures2KHR\0")?);
        tab.vkGetPhysicalDeviceProperties2KHR = tm(gpa("vkGetPhysicalDeviceProperties2KHR\0")?);
        tab.vkGetPhysicalDeviceFormatProperties2KHR = tm(gpa("vkGetPhysicalDeviceFormatProperties2KHR\0")?);
        tab.vkGetPhysicalDeviceImageFormatProperties2KHR = tm(gpa("vkGetPhysicalDeviceImageFormatProperties2KHR\0")?);
        tab.vkGetPhysicalDeviceQueueFamilyProperties2KHR = tm(gpa("vkGetPhysicalDeviceQueueFamilyProperties2KHR\0")?);
        tab.vkGetPhysicalDeviceMemoryProperties2KHR = tm(gpa("vkGetPhysicalDeviceMemoryProperties2KHR\0")?);
        tab.vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
          tm(gpa("vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0")?);
      }
    }

    // feature: VK_NV_external_memory_capabilities
    #[cfg(feature = "VK_NV_external_memory_capabilities")]
    {
      if has_ext("VK_NV_external_memory_capabilities\0") {
        tab.vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
          tm(gpa("vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0")?);
      }
    }

    // feature: VK_KHX_device_group_creation
    #[cfg(feature = "VK_KHX_device_group_creation")]
    {
      if has_ext("VK_KHX_device_group_creation\0") {
        tab.vkEnumeratePhysicalDeviceGroupsKHX = tm(gpa("vkEnumeratePhysicalDeviceGroupsKHX\0")?);
      }
    }

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    {
      if has_ext("VK_KHX_device_group\0") {
        tab.vkGetPhysicalDevicePresentRectanglesKHX = tm(gpa("vkGetPhysicalDevicePresentRectanglesKHX\0")?);
      }
    }

    // feature: VK_NN_vi_surface
    #[cfg(feature = "VK_NN_vi_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
    {
      if has_ext("VK_NN_vi_surface\0") {
        tab.vkCreateViSurfaceNN = tm(gpa("vkCreateViSurfaceNN\0")?);
      }
    }

    // feature: VK_KHR_external_memory_capabilities
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    {
      if has_ext("VK_KHR_external_memory_capabilities\0") {
        tab.vkGetPhysicalDeviceExternalBufferPropertiesKHR =
          tm(gpa("vkGetPhysicalDeviceExternalBufferPropertiesKHR\0")?);
      }
    }

    // feature: VK_KHR_external_semaphore_capabilities
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    {
      if has_ext("VK_KHR_external_semaphore_capabilities\0") {
        tab.vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
          tm(gpa("vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0")?);
      }
    }

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    {
      if has_ext("VK_NVX_device_generated_commands\0") {
        tab.vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX =
          tm(gpa("vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX\0")?);
      }
    }

    // feature: VK_EXT_direct_mode_display
    #[cfg(feature = "VK_EXT_direct_mode_display")]
    {
      if has_ext("VK_EXT_direct_mode_display\0") {
        tab.vkReleaseDisplayEXT = tm(gpa("vkReleaseDisplayEXT\0")?);
      }
    }

    // feature: VK_EXT_acquire_xlib_display
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    #[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
    {
      if has_ext("VK_EXT_acquire_xlib_display\0") {
        tab.vkAcquireXlibDisplayEXT = tm(gpa("vkAcquireXlibDisplayEXT\0")?);
        tab.vkGetRandROutputDisplayEXT = tm(gpa("vkGetRandROutputDisplayEXT\0")?);
      }
    }

    // feature: VK_EXT_display_surface_counter
    #[cfg(feature = "VK_EXT_display_surface_counter")]
    {
      if has_ext("VK_EXT_display_surface_counter\0") {
        tab.vkGetPhysicalDeviceSurfaceCapabilities2EXT = tm(gpa("vkGetPhysicalDeviceSurfaceCapabilities2EXT\0")?);
      }
    }

    // feature: VK_KHR_get_surface_capabilities2
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    {
      if has_ext("VK_KHR_get_surface_capabilities2\0") {
        tab.vkGetPhysicalDeviceSurfaceCapabilities2KHR = tm(gpa("vkGetPhysicalDeviceSurfaceCapabilities2KHR\0")?);
        tab.vkGetPhysicalDeviceSurfaceFormats2KHR = tm(gpa("vkGetPhysicalDeviceSurfaceFormats2KHR\0")?);
      }
    }

    // feature: VK_KHR_external_fence_capabilities
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    {
      if has_ext("VK_KHR_external_fence_capabilities\0") {
        tab.vkGetPhysicalDeviceExternalFencePropertiesKHR = tm(gpa("vkGetPhysicalDeviceExternalFencePropertiesKHR\0")?);
      }
    }

    // feature: VK_MVK_ios_surface
    #[cfg(feature = "VK_MVK_ios_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
    {
      if has_ext("VK_MVK_ios_surface\0") {
        tab.vkCreateIOSSurfaceMVK = tm(gpa("vkCreateIOSSurfaceMVK\0")?);
      }
    }

    // feature: VK_MVK_macos_surface
    #[cfg(feature = "VK_MVK_macos_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
    {
      if has_ext("VK_MVK_macos_surface\0") {
        tab.vkCreateMacOSSurfaceMVK = tm(gpa("vkCreateMacOSSurfaceMVK\0")?);
      }
    }

    // feature: VK_EXT_sample_locations
    #[cfg(feature = "VK_EXT_sample_locations")]
    {
      if has_ext("VK_EXT_sample_locations\0") {
        tab.vkGetPhysicalDeviceMultisamplePropertiesEXT = tm(gpa("vkGetPhysicalDeviceMultisamplePropertiesEXT\0")?);
      }
    }
    Ok(tab)
  }
}

impl VkDeviceDispatchTable {
  pub unsafe fn load<R, F1, F2>(gpa: F1, has_ext: F2) -> Result<VkDeviceDispatchTable, R>
  where
    F1: Fn(&str) -> Result<PFN_vkVoidFunction, R>,
    F2: Fn(&str) -> bool,
  {
    use std::mem::transmute as tm;
    let mut tab: VkDeviceDispatchTable = ::std::mem::zeroed();
    // feature: VK_VERSION_1_0
    tab.vkDestroyDevice = tm(gpa("vkDestroyDevice\0")?);
    tab.vkGetDeviceQueue = tm(gpa("vkGetDeviceQueue\0")?);
    tab.vkQueueSubmit = tm(gpa("vkQueueSubmit\0")?);
    tab.vkQueueWaitIdle = tm(gpa("vkQueueWaitIdle\0")?);
    tab.vkDeviceWaitIdle = tm(gpa("vkDeviceWaitIdle\0")?);
    tab.vkAllocateMemory = tm(gpa("vkAllocateMemory\0")?);
    tab.vkFreeMemory = tm(gpa("vkFreeMemory\0")?);
    tab.vkMapMemory = tm(gpa("vkMapMemory\0")?);
    tab.vkUnmapMemory = tm(gpa("vkUnmapMemory\0")?);
    tab.vkFlushMappedMemoryRanges = tm(gpa("vkFlushMappedMemoryRanges\0")?);
    tab.vkInvalidateMappedMemoryRanges = tm(gpa("vkInvalidateMappedMemoryRanges\0")?);
    tab.vkGetDeviceMemoryCommitment = tm(gpa("vkGetDeviceMemoryCommitment\0")?);
    tab.vkBindBufferMemory = tm(gpa("vkBindBufferMemory\0")?);
    tab.vkBindImageMemory = tm(gpa("vkBindImageMemory\0")?);
    tab.vkGetBufferMemoryRequirements = tm(gpa("vkGetBufferMemoryRequirements\0")?);
    tab.vkGetImageMemoryRequirements = tm(gpa("vkGetImageMemoryRequirements\0")?);
    tab.vkGetImageSparseMemoryRequirements = tm(gpa("vkGetImageSparseMemoryRequirements\0")?);
    tab.vkQueueBindSparse = tm(gpa("vkQueueBindSparse\0")?);
    tab.vkCreateFence = tm(gpa("vkCreateFence\0")?);
    tab.vkDestroyFence = tm(gpa("vkDestroyFence\0")?);
    tab.vkResetFences = tm(gpa("vkResetFences\0")?);
    tab.vkGetFenceStatus = tm(gpa("vkGetFenceStatus\0")?);
    tab.vkWaitForFences = tm(gpa("vkWaitForFences\0")?);
    tab.vkCreateSemaphore = tm(gpa("vkCreateSemaphore\0")?);
    tab.vkDestroySemaphore = tm(gpa("vkDestroySemaphore\0")?);
    tab.vkCreateEvent = tm(gpa("vkCreateEvent\0")?);
    tab.vkDestroyEvent = tm(gpa("vkDestroyEvent\0")?);
    tab.vkGetEventStatus = tm(gpa("vkGetEventStatus\0")?);
    tab.vkSetEvent = tm(gpa("vkSetEvent\0")?);
    tab.vkResetEvent = tm(gpa("vkResetEvent\0")?);
    tab.vkCreateQueryPool = tm(gpa("vkCreateQueryPool\0")?);
    tab.vkDestroyQueryPool = tm(gpa("vkDestroyQueryPool\0")?);
    tab.vkGetQueryPoolResults = tm(gpa("vkGetQueryPoolResults\0")?);
    tab.vkCreateBuffer = tm(gpa("vkCreateBuffer\0")?);
    tab.vkDestroyBuffer = tm(gpa("vkDestroyBuffer\0")?);
    tab.vkCreateBufferView = tm(gpa("vkCreateBufferView\0")?);
    tab.vkDestroyBufferView = tm(gpa("vkDestroyBufferView\0")?);
    tab.vkCreateImage = tm(gpa("vkCreateImage\0")?);
    tab.vkDestroyImage = tm(gpa("vkDestroyImage\0")?);
    tab.vkGetImageSubresourceLayout = tm(gpa("vkGetImageSubresourceLayout\0")?);
    tab.vkCreateImageView = tm(gpa("vkCreateImageView\0")?);
    tab.vkDestroyImageView = tm(gpa("vkDestroyImageView\0")?);
    tab.vkCreateShaderModule = tm(gpa("vkCreateShaderModule\0")?);
    tab.vkDestroyShaderModule = tm(gpa("vkDestroyShaderModule\0")?);
    tab.vkCreatePipelineCache = tm(gpa("vkCreatePipelineCache\0")?);
    tab.vkDestroyPipelineCache = tm(gpa("vkDestroyPipelineCache\0")?);
    tab.vkGetPipelineCacheData = tm(gpa("vkGetPipelineCacheData\0")?);
    tab.vkMergePipelineCaches = tm(gpa("vkMergePipelineCaches\0")?);
    tab.vkCreateGraphicsPipelines = tm(gpa("vkCreateGraphicsPipelines\0")?);
    tab.vkCreateComputePipelines = tm(gpa("vkCreateComputePipelines\0")?);
    tab.vkDestroyPipeline = tm(gpa("vkDestroyPipeline\0")?);
    tab.vkCreatePipelineLayout = tm(gpa("vkCreatePipelineLayout\0")?);
    tab.vkDestroyPipelineLayout = tm(gpa("vkDestroyPipelineLayout\0")?);
    tab.vkCreateSampler = tm(gpa("vkCreateSampler\0")?);
    tab.vkDestroySampler = tm(gpa("vkDestroySampler\0")?);
    tab.vkCreateDescriptorSetLayout = tm(gpa("vkCreateDescriptorSetLayout\0")?);
    tab.vkDestroyDescriptorSetLayout = tm(gpa("vkDestroyDescriptorSetLayout\0")?);
    tab.vkCreateDescriptorPool = tm(gpa("vkCreateDescriptorPool\0")?);
    tab.vkDestroyDescriptorPool = tm(gpa("vkDestroyDescriptorPool\0")?);
    tab.vkResetDescriptorPool = tm(gpa("vkResetDescriptorPool\0")?);
    tab.vkAllocateDescriptorSets = tm(gpa("vkAllocateDescriptorSets\0")?);
    tab.vkFreeDescriptorSets = tm(gpa("vkFreeDescriptorSets\0")?);
    tab.vkUpdateDescriptorSets = tm(gpa("vkUpdateDescriptorSets\0")?);
    tab.vkCreateFramebuffer = tm(gpa("vkCreateFramebuffer\0")?);
    tab.vkDestroyFramebuffer = tm(gpa("vkDestroyFramebuffer\0")?);
    tab.vkCreateRenderPass = tm(gpa("vkCreateRenderPass\0")?);
    tab.vkDestroyRenderPass = tm(gpa("vkDestroyRenderPass\0")?);
    tab.vkGetRenderAreaGranularity = tm(gpa("vkGetRenderAreaGranularity\0")?);
    tab.vkCreateCommandPool = tm(gpa("vkCreateCommandPool\0")?);
    tab.vkDestroyCommandPool = tm(gpa("vkDestroyCommandPool\0")?);
    tab.vkResetCommandPool = tm(gpa("vkResetCommandPool\0")?);
    tab.vkAllocateCommandBuffers = tm(gpa("vkAllocateCommandBuffers\0")?);
    tab.vkFreeCommandBuffers = tm(gpa("vkFreeCommandBuffers\0")?);
    tab.vkBeginCommandBuffer = tm(gpa("vkBeginCommandBuffer\0")?);
    tab.vkEndCommandBuffer = tm(gpa("vkEndCommandBuffer\0")?);
    tab.vkResetCommandBuffer = tm(gpa("vkResetCommandBuffer\0")?);
    tab.vkCmdBindPipeline = tm(gpa("vkCmdBindPipeline\0")?);
    tab.vkCmdSetViewport = tm(gpa("vkCmdSetViewport\0")?);
    tab.vkCmdSetScissor = tm(gpa("vkCmdSetScissor\0")?);
    tab.vkCmdSetLineWidth = tm(gpa("vkCmdSetLineWidth\0")?);
    tab.vkCmdSetDepthBias = tm(gpa("vkCmdSetDepthBias\0")?);
    tab.vkCmdSetBlendConstants = tm(gpa("vkCmdSetBlendConstants\0")?);
    tab.vkCmdSetDepthBounds = tm(gpa("vkCmdSetDepthBounds\0")?);
    tab.vkCmdSetStencilCompareMask = tm(gpa("vkCmdSetStencilCompareMask\0")?);
    tab.vkCmdSetStencilWriteMask = tm(gpa("vkCmdSetStencilWriteMask\0")?);
    tab.vkCmdSetStencilReference = tm(gpa("vkCmdSetStencilReference\0")?);
    tab.vkCmdBindDescriptorSets = tm(gpa("vkCmdBindDescriptorSets\0")?);
    tab.vkCmdBindIndexBuffer = tm(gpa("vkCmdBindIndexBuffer\0")?);
    tab.vkCmdBindVertexBuffers = tm(gpa("vkCmdBindVertexBuffers\0")?);
    tab.vkCmdDraw = tm(gpa("vkCmdDraw\0")?);
    tab.vkCmdDrawIndexed = tm(gpa("vkCmdDrawIndexed\0")?);
    tab.vkCmdDrawIndirect = tm(gpa("vkCmdDrawIndirect\0")?);
    tab.vkCmdDrawIndexedIndirect = tm(gpa("vkCmdDrawIndexedIndirect\0")?);
    tab.vkCmdDispatch = tm(gpa("vkCmdDispatch\0")?);
    tab.vkCmdDispatchIndirect = tm(gpa("vkCmdDispatchIndirect\0")?);
    tab.vkCmdCopyBuffer = tm(gpa("vkCmdCopyBuffer\0")?);
    tab.vkCmdCopyImage = tm(gpa("vkCmdCopyImage\0")?);
    tab.vkCmdBlitImage = tm(gpa("vkCmdBlitImage\0")?);
    tab.vkCmdCopyBufferToImage = tm(gpa("vkCmdCopyBufferToImage\0")?);
    tab.vkCmdCopyImageToBuffer = tm(gpa("vkCmdCopyImageToBuffer\0")?);
    tab.vkCmdUpdateBuffer = tm(gpa("vkCmdUpdateBuffer\0")?);
    tab.vkCmdFillBuffer = tm(gpa("vkCmdFillBuffer\0")?);
    tab.vkCmdClearColorImage = tm(gpa("vkCmdClearColorImage\0")?);
    tab.vkCmdClearDepthStencilImage = tm(gpa("vkCmdClearDepthStencilImage\0")?);
    tab.vkCmdClearAttachments = tm(gpa("vkCmdClearAttachments\0")?);
    tab.vkCmdResolveImage = tm(gpa("vkCmdResolveImage\0")?);
    tab.vkCmdSetEvent = tm(gpa("vkCmdSetEvent\0")?);
    tab.vkCmdResetEvent = tm(gpa("vkCmdResetEvent\0")?);
    tab.vkCmdWaitEvents = tm(gpa("vkCmdWaitEvents\0")?);
    tab.vkCmdPipelineBarrier = tm(gpa("vkCmdPipelineBarrier\0")?);
    tab.vkCmdBeginQuery = tm(gpa("vkCmdBeginQuery\0")?);
    tab.vkCmdEndQuery = tm(gpa("vkCmdEndQuery\0")?);
    tab.vkCmdResetQueryPool = tm(gpa("vkCmdResetQueryPool\0")?);
    tab.vkCmdWriteTimestamp = tm(gpa("vkCmdWriteTimestamp\0")?);
    tab.vkCmdCopyQueryPoolResults = tm(gpa("vkCmdCopyQueryPoolResults\0")?);
    tab.vkCmdPushConstants = tm(gpa("vkCmdPushConstants\0")?);
    tab.vkCmdBeginRenderPass = tm(gpa("vkCmdBeginRenderPass\0")?);
    tab.vkCmdNextSubpass = tm(gpa("vkCmdNextSubpass\0")?);
    tab.vkCmdEndRenderPass = tm(gpa("vkCmdEndRenderPass\0")?);
    tab.vkCmdExecuteCommands = tm(gpa("vkCmdExecuteCommands\0")?);

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    {
      if has_ext("VK_KHR_swapchain\0") {
        tab.vkCreateSwapchainKHR = tm(gpa("vkCreateSwapchainKHR\0")?);
        tab.vkDestroySwapchainKHR = tm(gpa("vkDestroySwapchainKHR\0")?);
        tab.vkGetSwapchainImagesKHR = tm(gpa("vkGetSwapchainImagesKHR\0")?);
        tab.vkAcquireNextImageKHR = tm(gpa("vkAcquireNextImageKHR\0")?);
        tab.vkQueuePresentKHR = tm(gpa("vkQueuePresentKHR\0")?);
      }
    }

    // feature: VK_KHR_display_swapchain
    #[cfg(feature = "VK_KHR_display_swapchain")]
    {
      if has_ext("VK_KHR_display_swapchain\0") {
        tab.vkCreateSharedSwapchainsKHR = tm(gpa("vkCreateSharedSwapchainsKHR\0")?);
      }
    }

    // feature: VK_EXT_debug_marker
    #[cfg(feature = "VK_EXT_debug_marker")]
    {
      if has_ext("VK_EXT_debug_marker\0") {
        tab.vkDebugMarkerSetObjectTagEXT = tm(gpa("vkDebugMarkerSetObjectTagEXT\0")?);
        tab.vkDebugMarkerSetObjectNameEXT = tm(gpa("vkDebugMarkerSetObjectNameEXT\0")?);
        tab.vkCmdDebugMarkerBeginEXT = tm(gpa("vkCmdDebugMarkerBeginEXT\0")?);
        tab.vkCmdDebugMarkerEndEXT = tm(gpa("vkCmdDebugMarkerEndEXT\0")?);
        tab.vkCmdDebugMarkerInsertEXT = tm(gpa("vkCmdDebugMarkerInsertEXT\0")?);
      }
    }

    // feature: VK_AMD_draw_indirect_count
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    {
      if has_ext("VK_AMD_draw_indirect_count\0") {
        tab.vkCmdDrawIndirectCountAMD = tm(gpa("vkCmdDrawIndirectCountAMD\0")?);
        tab.vkCmdDrawIndexedIndirectCountAMD = tm(gpa("vkCmdDrawIndexedIndirectCountAMD\0")?);
      }
    }

    // feature: VK_AMD_shader_info
    #[cfg(feature = "VK_AMD_shader_info")]
    {
      if has_ext("VK_AMD_shader_info\0") {
        tab.vkGetShaderInfoAMD = tm(gpa("vkGetShaderInfoAMD\0")?);
      }
    }

    // feature: VK_NV_external_memory_win32
    #[cfg(feature = "VK_NV_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    {
      if has_ext("VK_NV_external_memory_win32\0") {
        tab.vkGetMemoryWin32HandleNV = tm(gpa("vkGetMemoryWin32HandleNV\0")?);
      }
    }

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    {
      if has_ext("VK_KHX_device_group\0") {
        tab.vkGetDeviceGroupPeerMemoryFeaturesKHX = tm(gpa("vkGetDeviceGroupPeerMemoryFeaturesKHX\0")?);
        tab.vkCmdSetDeviceMaskKHX = tm(gpa("vkCmdSetDeviceMaskKHX\0")?);
        tab.vkCmdDispatchBaseKHX = tm(gpa("vkCmdDispatchBaseKHX\0")?);
        tab.vkGetDeviceGroupPresentCapabilitiesKHX = tm(gpa("vkGetDeviceGroupPresentCapabilitiesKHX\0")?);
        tab.vkGetDeviceGroupSurfacePresentModesKHX = tm(gpa("vkGetDeviceGroupSurfacePresentModesKHX\0")?);
        tab.vkAcquireNextImage2KHX = tm(gpa("vkAcquireNextImage2KHX\0")?);
      }
    }

    // feature: VK_KHR_maintenance1
    #[cfg(feature = "VK_KHR_maintenance1")]
    {
      if has_ext("VK_KHR_maintenance1\0") {
        tab.vkTrimCommandPoolKHR = tm(gpa("vkTrimCommandPoolKHR\0")?);
      }
    }

    // feature: VK_KHR_external_memory_win32
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    {
      if has_ext("VK_KHR_external_memory_win32\0") {
        tab.vkGetMemoryWin32HandleKHR = tm(gpa("vkGetMemoryWin32HandleKHR\0")?);
        tab.vkGetMemoryWin32HandlePropertiesKHR = tm(gpa("vkGetMemoryWin32HandlePropertiesKHR\0")?);
      }
    }

    // feature: VK_KHR_external_memory_fd
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    {
      if has_ext("VK_KHR_external_memory_fd\0") {
        tab.vkGetMemoryFdKHR = tm(gpa("vkGetMemoryFdKHR\0")?);
        tab.vkGetMemoryFdPropertiesKHR = tm(gpa("vkGetMemoryFdPropertiesKHR\0")?);
      }
    }

    // feature: VK_KHR_external_semaphore_win32
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    {
      if has_ext("VK_KHR_external_semaphore_win32\0") {
        tab.vkImportSemaphoreWin32HandleKHR = tm(gpa("vkImportSemaphoreWin32HandleKHR\0")?);
        tab.vkGetSemaphoreWin32HandleKHR = tm(gpa("vkGetSemaphoreWin32HandleKHR\0")?);
      }
    }

    // feature: VK_KHR_external_semaphore_fd
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    {
      if has_ext("VK_KHR_external_semaphore_fd\0") {
        tab.vkImportSemaphoreFdKHR = tm(gpa("vkImportSemaphoreFdKHR\0")?);
        tab.vkGetSemaphoreFdKHR = tm(gpa("vkGetSemaphoreFdKHR\0")?);
      }
    }

    // feature: VK_KHR_push_descriptor
    #[cfg(feature = "VK_KHR_push_descriptor")]
    {
      if has_ext("VK_KHR_push_descriptor\0") {
        tab.vkCmdPushDescriptorSetKHR = tm(gpa("vkCmdPushDescriptorSetKHR\0")?);
      }
    }

    // feature: VK_KHR_descriptor_update_template
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    {
      if has_ext("VK_KHR_descriptor_update_template\0") {
        tab.vkCreateDescriptorUpdateTemplateKHR = tm(gpa("vkCreateDescriptorUpdateTemplateKHR\0")?);
        tab.vkDestroyDescriptorUpdateTemplateKHR = tm(gpa("vkDestroyDescriptorUpdateTemplateKHR\0")?);
        tab.vkUpdateDescriptorSetWithTemplateKHR = tm(gpa("vkUpdateDescriptorSetWithTemplateKHR\0")?);
        tab.vkCmdPushDescriptorSetWithTemplateKHR = tm(gpa("vkCmdPushDescriptorSetWithTemplateKHR\0")?);
      }
    }

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    {
      if has_ext("VK_NVX_device_generated_commands\0") {
        tab.vkCmdProcessCommandsNVX = tm(gpa("vkCmdProcessCommandsNVX\0")?);
        tab.vkCmdReserveSpaceForCommandsNVX = tm(gpa("vkCmdReserveSpaceForCommandsNVX\0")?);
        tab.vkCreateIndirectCommandsLayoutNVX = tm(gpa("vkCreateIndirectCommandsLayoutNVX\0")?);
        tab.vkDestroyIndirectCommandsLayoutNVX = tm(gpa("vkDestroyIndirectCommandsLayoutNVX\0")?);
        tab.vkCreateObjectTableNVX = tm(gpa("vkCreateObjectTableNVX\0")?);
        tab.vkDestroyObjectTableNVX = tm(gpa("vkDestroyObjectTableNVX\0")?);
        tab.vkRegisterObjectsNVX = tm(gpa("vkRegisterObjectsNVX\0")?);
        tab.vkUnregisterObjectsNVX = tm(gpa("vkUnregisterObjectsNVX\0")?);
      }
    }

    // feature: VK_NV_clip_space_w_scaling
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    {
      if has_ext("VK_NV_clip_space_w_scaling\0") {
        tab.vkCmdSetViewportWScalingNV = tm(gpa("vkCmdSetViewportWScalingNV\0")?);
      }
    }

    // feature: VK_EXT_display_control
    #[cfg(feature = "VK_EXT_display_control")]
    {
      if has_ext("VK_EXT_display_control\0") {
        tab.vkDisplayPowerControlEXT = tm(gpa("vkDisplayPowerControlEXT\0")?);
        tab.vkRegisterDeviceEventEXT = tm(gpa("vkRegisterDeviceEventEXT\0")?);
        tab.vkRegisterDisplayEventEXT = tm(gpa("vkRegisterDisplayEventEXT\0")?);
        tab.vkGetSwapchainCounterEXT = tm(gpa("vkGetSwapchainCounterEXT\0")?);
      }
    }

    // feature: VK_GOOGLE_display_timing
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    {
      if has_ext("VK_GOOGLE_display_timing\0") {
        tab.vkGetRefreshCycleDurationGOOGLE = tm(gpa("vkGetRefreshCycleDurationGOOGLE\0")?);
        tab.vkGetPastPresentationTimingGOOGLE = tm(gpa("vkGetPastPresentationTimingGOOGLE\0")?);
      }
    }

    // feature: VK_EXT_discard_rectangles
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    {
      if has_ext("VK_EXT_discard_rectangles\0") {
        tab.vkCmdSetDiscardRectangleEXT = tm(gpa("vkCmdSetDiscardRectangleEXT\0")?);
      }
    }

    // feature: VK_EXT_hdr_metadata
    #[cfg(feature = "VK_EXT_hdr_metadata")]
    {
      if has_ext("VK_EXT_hdr_metadata\0") {
        tab.vkSetHdrMetadataEXT = tm(gpa("vkSetHdrMetadataEXT\0")?);
      }
    }

    // feature: VK_KHR_shared_presentable_image
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    {
      if has_ext("VK_KHR_shared_presentable_image\0") {
        tab.vkGetSwapchainStatusKHR = tm(gpa("vkGetSwapchainStatusKHR\0")?);
      }
    }

    // feature: VK_KHR_external_fence_win32
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    {
      if has_ext("VK_KHR_external_fence_win32\0") {
        tab.vkImportFenceWin32HandleKHR = tm(gpa("vkImportFenceWin32HandleKHR\0")?);
        tab.vkGetFenceWin32HandleKHR = tm(gpa("vkGetFenceWin32HandleKHR\0")?);
      }
    }

    // feature: VK_KHR_external_fence_fd
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    {
      if has_ext("VK_KHR_external_fence_fd\0") {
        tab.vkImportFenceFdKHR = tm(gpa("vkImportFenceFdKHR\0")?);
        tab.vkGetFenceFdKHR = tm(gpa("vkGetFenceFdKHR\0")?);
      }
    }

    // feature: VK_KHR_get_memory_requirements2
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    {
      if has_ext("VK_KHR_get_memory_requirements2\0") {
        tab.vkGetImageMemoryRequirements2KHR = tm(gpa("vkGetImageMemoryRequirements2KHR\0")?);
        tab.vkGetBufferMemoryRequirements2KHR = tm(gpa("vkGetBufferMemoryRequirements2KHR\0")?);
        tab.vkGetImageSparseMemoryRequirements2KHR = tm(gpa("vkGetImageSparseMemoryRequirements2KHR\0")?);
      }
    }

    // feature: VK_EXT_sample_locations
    #[cfg(feature = "VK_EXT_sample_locations")]
    {
      if has_ext("VK_EXT_sample_locations\0") {
        tab.vkCmdSetSampleLocationsEXT = tm(gpa("vkCmdSetSampleLocationsEXT\0")?);
      }
    }

    // feature: VK_KHR_bind_memory2
    #[cfg(feature = "VK_KHR_bind_memory2")]
    {
      if has_ext("VK_KHR_bind_memory2\0") {
        tab.vkBindBufferMemory2KHR = tm(gpa("vkBindBufferMemory2KHR\0")?);
        tab.vkBindImageMemory2KHR = tm(gpa("vkBindImageMemory2KHR\0")?);
      }
    }

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    {
      if has_ext("VK_KHR_sampler_ycbcr_conversion\0") {
        tab.vkCreateSamplerYcbcrConversionKHR = tm(gpa("vkCreateSamplerYcbcrConversionKHR\0")?);
        tab.vkDestroySamplerYcbcrConversionKHR = tm(gpa("vkDestroySamplerYcbcrConversionKHR\0")?);
      }
    }

    // feature: VK_EXT_validation_cache
    #[cfg(feature = "VK_EXT_validation_cache")]
    {
      if has_ext("VK_EXT_validation_cache\0") {
        tab.vkCreateValidationCacheEXT = tm(gpa("vkCreateValidationCacheEXT\0")?);
        tab.vkDestroyValidationCacheEXT = tm(gpa("vkDestroyValidationCacheEXT\0")?);
        tab.vkMergeValidationCachesEXT = tm(gpa("vkMergeValidationCachesEXT\0")?);
        tab.vkGetValidationCacheDataEXT = tm(gpa("vkGetValidationCacheDataEXT\0")?);
      }
    }

    // feature: VK_EXT_external_memory_host
    #[cfg(feature = "VK_EXT_external_memory_host")]
    {
      if has_ext("VK_EXT_external_memory_host\0") {
        tab.vkGetMemoryHostPointerPropertiesEXT = tm(gpa("vkGetMemoryHostPointerPropertiesEXT\0")?);
      }
    }

    // feature: VK_AMD_buffer_marker
    #[cfg(feature = "VK_AMD_buffer_marker")]
    {
      if has_ext("VK_AMD_buffer_marker\0") {
        tab.vkCmdWriteBufferMarkerAMD = tm(gpa("vkCmdWriteBufferMarkerAMD\0")?);
      }
    }
    Ok(tab)
  }
}
