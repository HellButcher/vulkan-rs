/* GENERATED FILE */

// feature: VK_VERSION_1_0

/// Encode pipeline cache version
pub use enums::VkPipelineCacheHeaderVersion;

/// Vulkan structure types (`stype`)
pub use enums::VkStructureType;

/// Bitmask specifying memory access types that will participate in a memory
/// dependency
pub use enums::VkAccessFlagBits;

/// Vulkan bitmasks
pub type VkFlags = u32;

/// Bitmask of VkAccessFlagBits
pub type VkAccessFlags = VkAccessFlagBits;

/// Vulkan device memory size and offsets
pub type VkDeviceSize = u64;

/// Layout of image and image subresources
pub use enums::VkImageLayout;

/// Bitmask specifying which aspects of an image are included in a view
pub use enums::VkImageAspectFlagBits;

/// Bitmask of VkImageAspectFlagBits
pub type VkImageAspectFlags = VkImageAspectFlagBits;

/// Specify an enumeration to track object handle types
pub use enums::VkObjectType;

/// Vulkan command return codes
pub use enums::VkResult;

/// Reserved for future use
pub type VkInstanceCreateFlags = VkFlags;

/// Allocation scope
pub use enums::VkSystemAllocationScope;

/// Allocation type
pub use enums::VkInternalAllocationType;

/// Vulkan boolean type
pub type VkBool32 = u32;

/// Available image formats
pub use enums::VkFormat;

/// Bitmask specifying features supported by a buffer
pub use enums::VkFormatFeatureFlagBits;

/// Bitmask of VkFormatFeatureFlagBits
pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;

/// Specifies the type of an image object
pub use enums::VkImageType;

/// Specifies the tiling arrangement of data in an image
pub use enums::VkImageTiling;

/// Bitmask specifying intended usage of an image
pub use enums::VkImageUsageFlagBits;

/// Bitmask of VkImageUsageFlagBits
pub type VkImageUsageFlags = VkImageUsageFlagBits;

/// Bitmask specifying additional parameters of an image
pub use enums::VkImageCreateFlagBits;

/// Bitmask of VkImageCreateFlagBits
pub type VkImageCreateFlags = VkImageCreateFlagBits;

/// Bitmask specifying sample counts supported for an image used for storage
/// operations
pub use enums::VkSampleCountFlagBits;

/// Bitmask of VkSampleCountFlagBits
pub type VkSampleCountFlags = VkSampleCountFlagBits;

/// Supported physical device types
pub use enums::VkPhysicalDeviceType;

/// Bitmask specifying capabilities of queues in a queue family
pub use enums::VkQueueFlagBits;

/// Bitmask of VkQueueFlagBits
pub type VkQueueFlags = VkQueueFlagBits;

/// Bitmask specifying properties for a memory type
pub use enums::VkMemoryPropertyFlagBits;

/// Bitmask of VkMemoryPropertyFlagBits
pub type VkMemoryPropertyFlags = VkMemoryPropertyFlagBits;

/// Bitmask specifying attribute flags for a heap
pub use enums::VkMemoryHeapFlagBits;

/// Bitmask of VkMemoryHeapFlagBits
pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;

/// Reserved for future use
pub type VkDeviceCreateFlags = VkFlags;

/// Reserved for future use
pub type VkDeviceQueueCreateFlags = VkFlags;

/// Bitmask specifying pipeline stages
pub use enums::VkPipelineStageFlagBits;

/// Bitmask of VkPipelineStageFlagBits
pub type VkPipelineStageFlags = VkPipelineStageFlagBits;

/// Reserved for future use
pub type VkMemoryMapFlags = VkFlags;

/// Bitmask specifying additional information about a sparse image resource
pub use enums::VkSparseImageFormatFlagBits;

/// Bitmask of VkSparseImageFormatFlagBits
pub type VkSparseImageFormatFlags = VkSparseImageFormatFlagBits;

/// Bitmask specifying usage of a sparse memory binding operation
pub use enums::VkSparseMemoryBindFlagBits;

/// Bitmask of VkSparseMemoryBindFlagBits
pub type VkSparseMemoryBindFlags = VkSparseMemoryBindFlagBits;

/// Bitmask specifying initial state and behavior of a fence
pub use enums::VkFenceCreateFlagBits;

/// Bitmask of VkFenceCreateFlagBits
pub type VkFenceCreateFlags = VkFenceCreateFlagBits;

/// Reserved for future use
pub type VkSemaphoreCreateFlags = VkFlags;

/// Reserved for future use
pub type VkEventCreateFlags = VkFlags;

/// Reserved for future use
pub type VkQueryPoolCreateFlags = VkFlags;

/// Specify the type of queries managed by a query pool
pub use enums::VkQueryType;

/// Bitmask specifying queried pipeline statistics
pub use enums::VkQueryPipelineStatisticFlagBits;

/// Bitmask of VkQueryPipelineStatisticFlagBits
pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;

/// Bitmask specifying how and when query results are returned
pub use enums::VkQueryResultFlagBits;

/// Bitmask of VkQueryResultFlagBits
pub type VkQueryResultFlags = VkQueryResultFlagBits;

/// Bitmask specifying additional parameters of a buffer
pub use enums::VkBufferCreateFlagBits;

/// Bitmask of VkBufferCreateFlagBits
pub type VkBufferCreateFlags = VkBufferCreateFlagBits;

/// Bitmask specifying allowed usage of a buffer
pub use enums::VkBufferUsageFlagBits;

/// Bitmask of VkBufferUsageFlagBits
pub type VkBufferUsageFlags = VkBufferUsageFlagBits;

/// Buffer and image sharing modes
pub use enums::VkSharingMode;

/// Reserved for future use
pub type VkBufferViewCreateFlags = VkFlags;

/// Reserved for future use
pub type VkImageViewCreateFlags = VkFlags;

/// Image view types
pub use enums::VkImageViewType;

/// Specify how a component is swizzled
pub use enums::VkComponentSwizzle;

/// Reserved for future use
pub type VkShaderModuleCreateFlags = VkFlags;

/// Reserved for future use
pub type VkPipelineCacheCreateFlags = VkFlags;

/// Bitmask controlling how a pipeline is created
pub use enums::VkPipelineCreateFlagBits;

/// Bitmask of VkPipelineCreateFlagBits
pub type VkPipelineCreateFlags = VkPipelineCreateFlagBits;

/// Reserved for future use
pub type VkPipelineShaderStageCreateFlags = VkFlags;

/// Bitmask specifying a pipeline stage
pub use enums::VkShaderStageFlagBits;

/// Reserved for future use
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;

/// Specify rate at which vertex attributes are pulled from buffers
pub use enums::VkVertexInputRate;

/// Reserved for future use
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;

/// Supported primitive topologies
pub use enums::VkPrimitiveTopology;

/// Reserved for future use
pub type VkPipelineTessellationStateCreateFlags = VkFlags;

/// Reserved for future use
pub type VkPipelineViewportStateCreateFlags = VkFlags;

/// Reserved for future use
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;

/// Control polygon rasterization mode
pub use enums::VkPolygonMode;

/// Bitmask controlling triangle culling
pub use enums::VkCullModeFlagBits;

/// Bitmask of VkCullModeFlagBits
pub type VkCullModeFlags = VkCullModeFlagBits;

/// Interpret polygon front-facing orientation
pub use enums::VkFrontFace;

/// Reserved for future use
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;

/// Mask of sample coverage information
pub type VkSampleMask = u32;

/// Reserved for future use
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;

/// Stencil comparison function
pub use enums::VkCompareOp;

/// Stencil comparison function
pub use enums::VkStencilOp;

/// Reserved for future use
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;

/// Framebuffer logical operations
pub use enums::VkLogicOp;

/// Framebuffer blending factors
pub use enums::VkBlendFactor;

/// Framebuffer blending operations
pub use enums::VkBlendOp;

/// Bitmask controlling which components are written to the framebuffer
pub use enums::VkColorComponentFlagBits;

/// Bitmask of VkColorComponentFlagBits
pub type VkColorComponentFlags = VkColorComponentFlagBits;

/// Reserved for future use
pub type VkPipelineDynamicStateCreateFlags = VkFlags;

/// Indicate which dynamic state is taken from dynamic state commands
pub use enums::VkDynamicState;

/// Reserved for future use
pub type VkPipelineLayoutCreateFlags = VkFlags;

/// Bitmask of VkShaderStageFlagBits
pub type VkShaderStageFlags = VkShaderStageFlagBits;

/// Reserved for future use
pub type VkSamplerCreateFlags = VkFlags;

/// Specify filters used for texture lookups
pub use enums::VkFilter;

/// Specify mipmap mode used for texture lookups
pub use enums::VkSamplerMipmapMode;

/// Specify behavior of sampling with texture coordinates outside an image
pub use enums::VkSamplerAddressMode;

/// Specify border color used for texture lookups
pub use enums::VkBorderColor;

/// Bitmask specifying descriptor set layout properties
pub use enums::VkDescriptorSetLayoutCreateFlagBits;

/// Bitmask of VkDescriptorSetLayoutCreateFlagBits
pub type VkDescriptorSetLayoutCreateFlags = VkDescriptorSetLayoutCreateFlagBits;

/// Specifies the type of a descriptor in a descriptor set
pub use enums::VkDescriptorType;

/// Bitmask specifying certain supported operations on a descriptor pool
pub use enums::VkDescriptorPoolCreateFlagBits;

/// Bitmask of VkDescriptorPoolCreateFlagBits
pub type VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlagBits;

/// Reserved for future use
pub type VkDescriptorPoolResetFlags = VkFlags;

/// Reserved for future use
pub type VkFramebufferCreateFlags = VkFlags;

/// Reserved for future use
pub type VkRenderPassCreateFlags = VkFlags;

/// Bitmask specifying additional properties of an attachment
pub use enums::VkAttachmentDescriptionFlagBits;

/// Bitmask of VkAttachmentDescriptionFlagBits
pub type VkAttachmentDescriptionFlags = VkAttachmentDescriptionFlagBits;

/// Specify how contents of an attachment are treated at the beginning of a subpass
pub use enums::VkAttachmentLoadOp;

/// Specify how contents of an attachment are treated at the end of a subpass
pub use enums::VkAttachmentStoreOp;

/// Bitmask specifying usage of a subpass
pub use enums::VkSubpassDescriptionFlagBits;

/// Bitmask of VkSubpassDescriptionFlagBits
pub type VkSubpassDescriptionFlags = VkSubpassDescriptionFlagBits;

/// Specify the bind point of a pipeline object to a command buffer
pub use enums::VkPipelineBindPoint;

/// Bitmask specifying how execution and memory dependencies are formed
pub use enums::VkDependencyFlagBits;

/// Bitmask of VkDependencyFlagBits
pub type VkDependencyFlags = VkDependencyFlagBits;

/// Bitmask specifying usage behavior for a command pool
pub use enums::VkCommandPoolCreateFlagBits;

/// Bitmask of VkCommandPoolCreateFlagBits
pub type VkCommandPoolCreateFlags = VkCommandPoolCreateFlagBits;

/// Bitmask controlling behavior of a command pool reset
pub use enums::VkCommandPoolResetFlagBits;

/// Bitmask of VkCommandPoolResetFlagBits
pub type VkCommandPoolResetFlags = VkCommandPoolResetFlagBits;

/// Enumerant specifying a command buffer level
pub use enums::VkCommandBufferLevel;

/// Bitmask specifying usage behavior for command buffer
pub use enums::VkCommandBufferUsageFlagBits;

/// Bitmask of VkCommandBufferUsageFlagBits
pub type VkCommandBufferUsageFlags = VkCommandBufferUsageFlagBits;

/// Bitmask specifying constraints on a query
pub use enums::VkQueryControlFlagBits;

/// Bitmask of VkQueryControlFlagBits
pub type VkQueryControlFlags = VkQueryControlFlagBits;

/// Bitmask controlling behavior of a command buffer reset
pub use enums::VkCommandBufferResetFlagBits;

/// Bitmask of VkCommandBufferResetFlagBits
pub type VkCommandBufferResetFlags = VkCommandBufferResetFlagBits;

/// Bitmask specifying sets of stencil state for which to update the compare mask
pub use enums::VkStencilFaceFlagBits;

/// Bitmask of VkStencilFaceFlagBits
pub type VkStencilFaceFlags = VkStencilFaceFlagBits;

/// Type of index buffer indices
pub use enums::VkIndexType;

/// Specify how commands in the first subpass of a render pass are provided
pub use enums::VkSubpassContents;

// feature: VK_KHR_surface

/// presentation transforms supported on a device
#[cfg(feature = "VK_KHR_surface")]
pub use enums::VkSurfaceTransformFlagBitsKHR;

/// Bitmask of VkSurfaceTransformFlagBitsKHR
#[cfg(feature = "VK_KHR_surface")]
pub type VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagBitsKHR;

/// alpha compositing modes supported on a device
#[cfg(feature = "VK_KHR_surface")]
pub use enums::VkCompositeAlphaFlagBitsKHR;

/// Bitmask of VkCompositeAlphaFlagBitsKHR
#[cfg(feature = "VK_KHR_surface")]
pub type VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagBitsKHR;

/// supported color space of the presentation engine
#[cfg(feature = "VK_KHR_surface")]
pub use enums::VkColorSpaceKHR;

/// presentation mode supported for a surface
#[cfg(feature = "VK_KHR_surface")]
pub use enums::VkPresentModeKHR;

// feature: VK_KHR_swapchain

/// Bitmask controlling swapchain creation
#[cfg(feature = "VK_KHR_swapchain")]
pub use enums::VkSwapchainCreateFlagBitsKHR;

/// Bitmask of VkSwapchainCreateFlagBitsKHR
#[cfg(feature = "VK_KHR_swapchain")]
pub type VkSwapchainCreateFlagsKHR = VkSwapchainCreateFlagBitsKHR;

// feature: VK_KHR_display

/// Alpha blending type
#[cfg(feature = "VK_KHR_display")]
pub use enums::VkDisplayPlaneAlphaFlagBitsKHR;

/// Bitmask of VkDisplayPlaneAlphaFlagBitsKHR
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagBitsKHR;
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayModeCreateFlagsKHR = VkFlags;
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;

// feature: VK_KHR_xlib_surface
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;

// feature: VK_KHR_xcb_surface
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;

// feature: VK_KHR_wayland_surface
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;

// feature: VK_KHR_mir_surface
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub type VkMirSurfaceCreateFlagsKHR = VkFlags;

// feature: VK_KHR_android_surface
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub type VkAndroidSurfaceCreateFlagsKHR = VkFlags;

// feature: VK_KHR_win32_surface
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;

// feature: VK_EXT_debug_report

/// Specify the type of an object handle
#[cfg(feature = "VK_EXT_debug_report")]
pub use enums::VkDebugReportObjectTypeEXT;

/// Bitmask specifying events which cause a debug report callback
#[cfg(feature = "VK_EXT_debug_report")]
pub use enums::VkDebugReportFlagBitsEXT;

/// Bitmask of VkDebugReportFlagBitsEXT
#[cfg(feature = "VK_EXT_debug_report")]
pub type VkDebugReportFlagsEXT = VkDebugReportFlagBitsEXT;

// feature: VK_AMD_rasterization_order

/// Specify rasterization order for a graphics pipeline
#[cfg(feature = "VK_AMD_rasterization_order")]
pub use enums::VkRasterizationOrderAMD;

// feature: VK_AMD_shader_info
#[cfg(feature = "VK_AMD_shader_info")]
pub use enums::VkShaderInfoTypeAMD;

// feature: VK_NV_external_memory_capabilities

/// Bitmask specifying external memory handle types
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub use enums::VkExternalMemoryHandleTypeFlagBitsNV;

/// Bitmask of VkExternalMemoryHandleTypeFlagBitsNV
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub type VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagBitsNV;

/// Bitmask specifying external memory features
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub use enums::VkExternalMemoryFeatureFlagBitsNV;

/// Bitmask of VkExternalMemoryFeatureFlagBitsNV
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub type VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagBitsNV;

// feature: VK_KHX_device_group

/// Bitmask specifying supported peer memory features
#[cfg(feature = "VK_KHX_device_group")]
pub use enums::VkPeerMemoryFeatureFlagBitsKHX;

/// Bitmask of VkPeerMemoryFeatureFlagBitsKHX
#[cfg(feature = "VK_KHX_device_group")]
pub type VkPeerMemoryFeatureFlagsKHX = VkPeerMemoryFeatureFlagBitsKHX;

/// Bitmask specifying flags for a device memory allocation
#[cfg(feature = "VK_KHX_device_group")]
pub use enums::VkMemoryAllocateFlagBitsKHX;

/// Bitmask of VkMemoryAllocateFlagBitsKHX
#[cfg(feature = "VK_KHX_device_group")]
pub type VkMemoryAllocateFlagsKHX = VkMemoryAllocateFlagBitsKHX;

/// Bitmask specifying supported device group present modes
#[cfg(feature = "VK_KHX_device_group")]
pub use enums::VkDeviceGroupPresentModeFlagBitsKHX;

/// Bitmask of VkDeviceGroupPresentModeFlagBitsKHX
#[cfg(feature = "VK_KHX_device_group")]
pub type VkDeviceGroupPresentModeFlagsKHX = VkDeviceGroupPresentModeFlagBitsKHX;

// feature: VK_EXT_validation_flags

/// Specify validation checks to disable
#[cfg(feature = "VK_EXT_validation_flags")]
pub use enums::VkValidationCheckEXT;

// feature: VK_NN_vi_surface
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub type VkViSurfaceCreateFlagsNN = VkFlags;

// feature: VK_KHR_maintenance1
#[cfg(feature = "VK_KHR_maintenance1")]
pub type VkCommandPoolTrimFlagsKHR = VkFlags;

// feature: VK_KHR_external_memory_capabilities

/// Bit specifying external memory handle types
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub use enums::VkExternalMemoryHandleTypeFlagBitsKHR;

/// Bitmask of VkExternalMemoryHandleTypeFlagBitsKHR
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub type VkExternalMemoryHandleTypeFlagsKHR = VkExternalMemoryHandleTypeFlagBitsKHR;

/// Bitmask specifying features of an external memory handle type
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub use enums::VkExternalMemoryFeatureFlagBitsKHR;

/// Bitmask of VkExternalMemoryFeatureFlagBitsKHR
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub type VkExternalMemoryFeatureFlagsKHR = VkExternalMemoryFeatureFlagBitsKHR;

// feature: VK_KHR_external_semaphore_capabilities

/// Bitmask of valid external semaphore handle types
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub use enums::VkExternalSemaphoreHandleTypeFlagBitsKHR;

/// Bitmask of VkExternalSemaphoreHandleTypeFlagBitsKHR
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkExternalSemaphoreHandleTypeFlagBitsKHR;

/// Bitfield describing features of an external semaphore handle type
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub use enums::VkExternalSemaphoreFeatureFlagBitsKHR;

/// Bitmask of VkExternalSemaphoreFeatureFlagBitsKHR
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub type VkExternalSemaphoreFeatureFlagsKHR = VkExternalSemaphoreFeatureFlagBitsKHR;

// feature: VK_KHR_external_semaphore

/// Bitmask specifying additional parameters of semaphore payload import
#[cfg(feature = "VK_KHR_external_semaphore")]
pub use enums::VkSemaphoreImportFlagBitsKHR;

/// Bitmask of VkSemaphoreImportFlagBitsKHR
#[cfg(feature = "VK_KHR_external_semaphore")]
pub type VkSemaphoreImportFlagsKHR = VkSemaphoreImportFlagBitsKHR;

// feature: VK_KHR_descriptor_update_template

/// Reserved for future use
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkFlags;

/// Indicates the valid usage of the descriptor update template
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub use enums::VkDescriptorUpdateTemplateTypeKHR;

// feature: VK_NVX_device_generated_commands

/// Bitmask specifying allowed usage of a indirect commands layout
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub use enums::VkIndirectCommandsLayoutUsageFlagBitsNVX;

/// Bitmask of VkIndirectCommandsLayoutUsageFlagBitsNVX
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkIndirectCommandsLayoutUsageFlagsNVX = VkIndirectCommandsLayoutUsageFlagBitsNVX;

/// Bitmask specifying allowed usage of an object entry
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub use enums::VkObjectEntryUsageFlagBitsNVX;

/// Bitmask of VkObjectEntryUsageFlagBitsNVX
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkObjectEntryUsageFlagsNVX = VkObjectEntryUsageFlagBitsNVX;

/// Enum specifying
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub use enums::VkIndirectCommandsTokenTypeNVX;

/// Enum specifying object table entry type
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub use enums::VkObjectEntryTypeNVX;

// feature: VK_EXT_display_surface_counter

/// Surface-relative counter types
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub use enums::VkSurfaceCounterFlagBitsEXT;

/// Bitmask of VkSurfaceCounterFlagBitsEXT
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub type VkSurfaceCounterFlagsEXT = VkSurfaceCounterFlagBitsEXT;

// feature: VK_EXT_display_control

/// Possible power states for a display
#[cfg(feature = "VK_EXT_display_control")]
pub use enums::VkDisplayPowerStateEXT;

/// Events that can occur on a device object
#[cfg(feature = "VK_EXT_display_control")]
pub use enums::VkDeviceEventTypeEXT;

/// Events that can occur on a display object
#[cfg(feature = "VK_EXT_display_control")]
pub use enums::VkDisplayEventTypeEXT;

// feature: VK_NV_viewport_swizzle

/// Specify how a viewport coordinate is swizzled
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub use enums::VkViewportCoordinateSwizzleNV;

/// Reserved for future use
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;

// feature: VK_EXT_discard_rectangles

/// Reserved for future use
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkFlags;

/// Specify the discard rectangle mode
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub use enums::VkDiscardRectangleModeEXT;

// feature: VK_EXT_conservative_rasterization

/// Reserved for future use
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub type VkPipelineRasterizationConservativeStateCreateFlagsEXT = VkFlags;

/// Specify the conservative rasterization mode
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub use enums::VkConservativeRasterizationModeEXT;

// feature: VK_KHR_external_fence_capabilities

/// Bitmask of valid external fence handle types
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub use enums::VkExternalFenceHandleTypeFlagBitsKHR;

/// Bitmask of VkExternalFenceHandleTypeFlagBitsKHR
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub type VkExternalFenceHandleTypeFlagsKHR = VkExternalFenceHandleTypeFlagBitsKHR;

/// Bitfield describing features of an external fence handle type
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub use enums::VkExternalFenceFeatureFlagBitsKHR;

/// Bitmask of VkExternalFenceFeatureFlagBitsKHR
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub type VkExternalFenceFeatureFlagsKHR = VkExternalFenceFeatureFlagBitsKHR;

// feature: VK_KHR_external_fence

/// Bitmask specifying additional parameters of fence payload import
#[cfg(feature = "VK_KHR_external_fence")]
pub use enums::VkFenceImportFlagBitsKHR;

/// Bitmask of VkFenceImportFlagBitsKHR
#[cfg(feature = "VK_KHR_external_fence")]
pub type VkFenceImportFlagsKHR = VkFenceImportFlagBitsKHR;

// feature: VK_KHR_maintenance2

/// Enum specifying the point clipping behaviour
#[cfg(feature = "VK_KHR_maintenance2")]
pub use enums::VkPointClippingBehaviorKHR;

/// Enum describing tessellation domain origin
#[cfg(feature = "VK_KHR_maintenance2")]
pub use enums::VkTessellationDomainOriginKHR;

// feature: VK_MVK_ios_surface
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub type VkIOSSurfaceCreateFlagsMVK = VkFlags;

// feature: VK_MVK_macos_surface
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub type VkMacOSSurfaceCreateFlagsMVK = VkFlags;

// feature: VK_EXT_sampler_filter_minmax

/// Specify reduction mode for texture filtering
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub use enums::VkSamplerReductionModeEXT;

// feature: VK_EXT_blend_operation_advanced

/// Enumerant specifying the blend overlap parameter
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub use enums::VkBlendOverlapEXT;

// feature: VK_NV_fragment_coverage_to_color

/// Reserved for future use
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub type VkPipelineCoverageToColorStateCreateFlagsNV = VkFlags;

// feature: VK_NV_framebuffer_mixed_samples

/// Reserved for future use
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub type VkPipelineCoverageModulationStateCreateFlagsNV = VkFlags;

/// Specify the discard rectangle mode
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub use enums::VkCoverageModulationModeNV;

// feature: VK_KHR_sampler_ycbcr_conversion

/// Color model component of a color space
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub use enums::VkSamplerYcbcrModelConversionKHR;

/// Range of encoded values in a color space
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub use enums::VkSamplerYcbcrRangeKHR;

/// Position of downsampled chroma samples
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub use enums::VkChromaLocationKHR;

// feature: VK_EXT_validation_cache

/// Reserved for future use
#[cfg(feature = "VK_EXT_validation_cache")]
pub type VkValidationCacheCreateFlagsEXT = VkFlags;

/// Encode validation cache version
#[cfg(feature = "VK_EXT_validation_cache")]
pub use enums::VkValidationCacheHeaderVersionEXT;

// feature: VK_EXT_global_priority

/// Values specifying a system-wide queue priority
#[cfg(feature = "VK_EXT_global_priority")]
pub use enums::VkQueueGlobalPriorityEXT;
