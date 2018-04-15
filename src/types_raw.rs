/* GENERATED FILE */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use enums;
use platform::*;
use types_base::*;

// feature: VK_VERSION_1_0
pub type VkBuffer = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferMemoryBarrier {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDispatchIndirectCommand {
  pub x: u32,
  pub y: u32,
  pub z: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndexedIndirectCommand {
  pub indexCount: u32,
  pub instanceCount: u32,
  pub firstIndex: u32,
  pub vertexOffset: i32,
  pub firstInstance: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndirectCommand {
  pub vertexCount: u32,
  pub instanceCount: u32,
  pub firstVertex: u32,
  pub firstInstance: u32,
}
pub type VkImage = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceRange {
  pub aspectMask: VkImageAspectFlags,
  pub baseMipLevel: u32,
  pub levelCount: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageMemoryBarrier {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub oldLayout: VkImageLayout,
  pub newLayout: VkImageLayout,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub image: VkImage,
  pub subresourceRange: VkImageSubresourceRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryBarrier {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkApplicationInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub pApplicationName: *const c_char,
  pub applicationVersion: u32,
  pub pEngineName: *const c_char,
  pub engineVersion: u32,
  pub apiVersion: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInstanceCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkInstanceCreateFlags,
  pub pApplicationInfo: *const VkApplicationInfo,
  pub enabledLayerCount: u32,
  pub ppEnabledLayerNames: *const *const c_char,
  pub enabledExtensionCount: u32,
  pub ppEnabledExtensionNames: *const *const c_char,
}
pub type PFN_vkAllocationFunction =
  extern "system" fn(*mut c_void, usize, usize, VkSystemAllocationScope) -> *mut c_void;
pub type PFN_vkReallocationFunction =
  extern "system" fn(*mut c_void, *mut c_void, usize, usize, VkSystemAllocationScope) -> *mut c_void;
pub type PFN_vkFreeFunction = extern "system" fn(*mut c_void, *mut c_void);
pub type PFN_vkInternalAllocationNotification =
  extern "system" fn(*mut c_void, usize, VkInternalAllocationType, VkSystemAllocationScope);
pub type PFN_vkInternalFreeNotification =
  extern "system" fn(*mut c_void, usize, VkInternalAllocationType, VkSystemAllocationScope);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAllocationCallbacks {
  pub pUserData: *mut c_void,
  pub pfnAllocation: PFN_vkAllocationFunction,
  pub pfnReallocation: PFN_vkReallocationFunction,
  pub pfnFree: PFN_vkFreeFunction,
  pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
  pub pfnInternalFree: PFN_vkInternalFreeNotification,
}
pub type VkInstance = usize;
pub type VkPhysicalDevice = usize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFeatures {
  pub robustBufferAccess: VkBool32,
  pub fullDrawIndexUint32: VkBool32,
  pub imageCubeArray: VkBool32,
  pub independentBlend: VkBool32,
  pub geometryShader: VkBool32,
  pub tessellationShader: VkBool32,
  pub sampleRateShading: VkBool32,
  pub dualSrcBlend: VkBool32,
  pub logicOp: VkBool32,
  pub multiDrawIndirect: VkBool32,
  pub drawIndirectFirstInstance: VkBool32,
  pub depthClamp: VkBool32,
  pub depthBiasClamp: VkBool32,
  pub fillModeNonSolid: VkBool32,
  pub depthBounds: VkBool32,
  pub wideLines: VkBool32,
  pub largePoints: VkBool32,
  pub alphaToOne: VkBool32,
  pub multiViewport: VkBool32,
  pub samplerAnisotropy: VkBool32,
  pub textureCompressionETC2: VkBool32,
  pub textureCompressionASTC_LDR: VkBool32,
  pub textureCompressionBC: VkBool32,
  pub occlusionQueryPrecise: VkBool32,
  pub pipelineStatisticsQuery: VkBool32,
  pub vertexPipelineStoresAndAtomics: VkBool32,
  pub fragmentStoresAndAtomics: VkBool32,
  pub shaderTessellationAndGeometryPointSize: VkBool32,
  pub shaderImageGatherExtended: VkBool32,
  pub shaderStorageImageExtendedFormats: VkBool32,
  pub shaderStorageImageMultisample: VkBool32,
  pub shaderStorageImageReadWithoutFormat: VkBool32,
  pub shaderStorageImageWriteWithoutFormat: VkBool32,
  pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
  pub shaderSampledImageArrayDynamicIndexing: VkBool32,
  pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
  pub shaderStorageImageArrayDynamicIndexing: VkBool32,
  pub shaderClipDistance: VkBool32,
  pub shaderCullDistance: VkBool32,
  pub shaderFloat64: VkBool32,
  pub shaderInt64: VkBool32,
  pub shaderInt16: VkBool32,
  pub shaderResourceResidency: VkBool32,
  pub shaderResourceMinLod: VkBool32,
  pub sparseBinding: VkBool32,
  pub sparseResidencyBuffer: VkBool32,
  pub sparseResidencyImage2D: VkBool32,
  pub sparseResidencyImage3D: VkBool32,
  pub sparseResidency2Samples: VkBool32,
  pub sparseResidency4Samples: VkBool32,
  pub sparseResidency8Samples: VkBool32,
  pub sparseResidency16Samples: VkBool32,
  pub sparseResidencyAliased: VkBool32,
  pub variableMultisampleRate: VkBool32,
  pub inheritedQueries: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFormatProperties {
  pub linearTilingFeatures: VkFormatFeatureFlags,
  pub optimalTilingFeatures: VkFormatFeatureFlags,
  pub bufferFeatures: VkFormatFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent3D {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatProperties {
  pub maxExtent: VkExtent3D,
  pub maxMipLevels: u32,
  pub maxArrayLayers: u32,
  pub sampleCounts: VkSampleCountFlags,
  pub maxResourceSize: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceLimits {
  pub maxImageDimension1D: u32,
  pub maxImageDimension2D: u32,
  pub maxImageDimension3D: u32,
  pub maxImageDimensionCube: u32,
  pub maxImageArrayLayers: u32,
  pub maxTexelBufferElements: u32,
  pub maxUniformBufferRange: u32,
  pub maxStorageBufferRange: u32,
  pub maxPushConstantsSize: u32,
  pub maxMemoryAllocationCount: u32,
  pub maxSamplerAllocationCount: u32,
  pub bufferImageGranularity: VkDeviceSize,
  pub sparseAddressSpaceSize: VkDeviceSize,
  pub maxBoundDescriptorSets: u32,
  pub maxPerStageDescriptorSamplers: u32,
  pub maxPerStageDescriptorUniformBuffers: u32,
  pub maxPerStageDescriptorStorageBuffers: u32,
  pub maxPerStageDescriptorSampledImages: u32,
  pub maxPerStageDescriptorStorageImages: u32,
  pub maxPerStageDescriptorInputAttachments: u32,
  pub maxPerStageResources: u32,
  pub maxDescriptorSetSamplers: u32,
  pub maxDescriptorSetUniformBuffers: u32,
  pub maxDescriptorSetUniformBuffersDynamic: u32,
  pub maxDescriptorSetStorageBuffers: u32,
  pub maxDescriptorSetStorageBuffersDynamic: u32,
  pub maxDescriptorSetSampledImages: u32,
  pub maxDescriptorSetStorageImages: u32,
  pub maxDescriptorSetInputAttachments: u32,
  pub maxVertexInputAttributes: u32,
  pub maxVertexInputBindings: u32,
  pub maxVertexInputAttributeOffset: u32,
  pub maxVertexInputBindingStride: u32,
  pub maxVertexOutputComponents: u32,
  pub maxTessellationGenerationLevel: u32,
  pub maxTessellationPatchSize: u32,
  pub maxTessellationControlPerVertexInputComponents: u32,
  pub maxTessellationControlPerVertexOutputComponents: u32,
  pub maxTessellationControlPerPatchOutputComponents: u32,
  pub maxTessellationControlTotalOutputComponents: u32,
  pub maxTessellationEvaluationInputComponents: u32,
  pub maxTessellationEvaluationOutputComponents: u32,
  pub maxGeometryShaderInvocations: u32,
  pub maxGeometryInputComponents: u32,
  pub maxGeometryOutputComponents: u32,
  pub maxGeometryOutputVertices: u32,
  pub maxGeometryTotalOutputComponents: u32,
  pub maxFragmentInputComponents: u32,
  pub maxFragmentOutputAttachments: u32,
  pub maxFragmentDualSrcAttachments: u32,
  pub maxFragmentCombinedOutputResources: u32,
  pub maxComputeSharedMemorySize: u32,
  pub maxComputeWorkGroupCount: [u32; 3],
  pub maxComputeWorkGroupInvocations: u32,
  pub maxComputeWorkGroupSize: [u32; 3],
  pub subPixelPrecisionBits: u32,
  pub subTexelPrecisionBits: u32,
  pub mipmapPrecisionBits: u32,
  pub maxDrawIndexedIndexValue: u32,
  pub maxDrawIndirectCount: u32,
  pub maxSamplerLodBias: f32,
  pub maxSamplerAnisotropy: f32,
  pub maxViewports: u32,
  pub maxViewportDimensions: [u32; 2],
  pub viewportBoundsRange: [f32; 2],
  pub viewportSubPixelBits: u32,
  pub minMemoryMapAlignment: usize,
  pub minTexelBufferOffsetAlignment: VkDeviceSize,
  pub minUniformBufferOffsetAlignment: VkDeviceSize,
  pub minStorageBufferOffsetAlignment: VkDeviceSize,
  pub minTexelOffset: i32,
  pub maxTexelOffset: u32,
  pub minTexelGatherOffset: i32,
  pub maxTexelGatherOffset: u32,
  pub minInterpolationOffset: f32,
  pub maxInterpolationOffset: f32,
  pub subPixelInterpolationOffsetBits: u32,
  pub maxFramebufferWidth: u32,
  pub maxFramebufferHeight: u32,
  pub maxFramebufferLayers: u32,
  pub framebufferColorSampleCounts: VkSampleCountFlags,
  pub framebufferDepthSampleCounts: VkSampleCountFlags,
  pub framebufferStencilSampleCounts: VkSampleCountFlags,
  pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
  pub maxColorAttachments: u32,
  pub sampledImageColorSampleCounts: VkSampleCountFlags,
  pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
  pub sampledImageDepthSampleCounts: VkSampleCountFlags,
  pub sampledImageStencilSampleCounts: VkSampleCountFlags,
  pub storageImageSampleCounts: VkSampleCountFlags,
  pub maxSampleMaskWords: u32,
  pub timestampComputeAndGraphics: VkBool32,
  pub timestampPeriod: f32,
  pub maxClipDistances: u32,
  pub maxCullDistances: u32,
  pub maxCombinedClipAndCullDistances: u32,
  pub discreteQueuePriorities: u32,
  pub pointSizeRange: [f32; 2],
  pub lineWidthRange: [f32; 2],
  pub pointSizeGranularity: f32,
  pub lineWidthGranularity: f32,
  pub strictLines: VkBool32,
  pub standardSampleLocations: VkBool32,
  pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
  pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
  pub nonCoherentAtomSize: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseProperties {
  pub residencyStandard2DBlockShape: VkBool32,
  pub residencyStandard2DMultisampleBlockShape: VkBool32,
  pub residencyStandard3DBlockShape: VkBool32,
  pub residencyAlignedMipSize: VkBool32,
  pub residencyNonResidentStrict: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProperties {
  pub apiVersion: u32,
  pub driverVersion: u32,
  pub vendorID: u32,
  pub deviceID: u32,
  pub deviceType: VkPhysicalDeviceType,
  pub deviceName: [c_char; enums::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
  pub pipelineCacheUUID: [u8; enums::VK_UUID_SIZE as usize],
  pub limits: VkPhysicalDeviceLimits,
  pub sparseProperties: VkPhysicalDeviceSparseProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueueFamilyProperties {
  pub queueFlags: VkQueueFlags,
  pub queueCount: u32,
  pub timestampValidBits: u32,
  pub minImageTransferGranularity: VkExtent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryType {
  pub propertyFlags: VkMemoryPropertyFlags,
  pub heapIndex: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHeap {
  pub size: VkDeviceSize,
  pub flags: VkMemoryHeapFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties {
  pub memoryTypeCount: u32,
  pub memoryTypes: [VkMemoryType; enums::VK_MAX_MEMORY_TYPES as usize],
  pub memoryHeapCount: u32,
  pub memoryHeaps: [VkMemoryHeap; enums::VK_MAX_MEMORY_HEAPS as usize],
}
pub type PFN_vkVoidFunction = extern "system" fn();
pub type VkDevice = usize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceQueueCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkDeviceQueueCreateFlags,
  pub queueFamilyIndex: u32,
  pub queueCount: u32,
  pub pQueuePriorities: *const f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkDeviceCreateFlags,
  pub queueCreateInfoCount: u32,
  pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
  pub enabledLayerCount: u32,
  pub ppEnabledLayerNames: *const *const c_char,
  pub enabledExtensionCount: u32,
  pub ppEnabledExtensionNames: *const *const c_char,
  pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtensionProperties {
  pub extensionName: [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize],
  pub specVersion: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkLayerProperties {
  pub layerName: [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize],
  pub specVersion: u32,
  pub implementationVersion: u32,
  pub description: [c_char; enums::VK_MAX_DESCRIPTION_SIZE as usize],
}
pub type VkQueue = usize;
pub type VkSemaphore = u64;
pub type VkCommandBuffer = usize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubmitInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub waitSemaphoreCount: u32,
  pub pWaitSemaphores: *const VkSemaphore,
  pub pWaitDstStageMask: *const VkPipelineStageFlags,
  pub commandBufferCount: u32,
  pub pCommandBuffers: *const VkCommandBuffer,
  pub signalSemaphoreCount: u32,
  pub pSignalSemaphores: *const VkSemaphore,
}
pub type VkFence = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryAllocateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeIndex: u32,
}
pub type VkDeviceMemory = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMappedMemoryRange {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryRequirements {
  pub size: VkDeviceSize,
  pub alignment: VkDeviceSize,
  pub memoryTypeBits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties {
  pub aspectMask: VkImageAspectFlags,
  pub imageGranularity: VkExtent3D,
  pub flags: VkSparseImageFormatFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryRequirements {
  pub formatProperties: VkSparseImageFormatProperties,
  pub imageMipTailFirstLod: u32,
  pub imageMipTailSize: VkDeviceSize,
  pub imageMipTailOffset: VkDeviceSize,
  pub imageMipTailStride: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseMemoryBind {
  pub resourceOffset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  pub flags: VkSparseMemoryBindFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseBufferMemoryBindInfo {
  pub buffer: VkBuffer,
  pub bindCount: u32,
  pub pBinds: *const VkSparseMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
  pub image: VkImage,
  pub bindCount: u32,
  pub pBinds: *const VkSparseMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresource {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub arrayLayer: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset3D {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBind {
  pub subresource: VkImageSubresource,
  pub offset: VkOffset3D,
  pub extent: VkExtent3D,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  pub flags: VkSparseMemoryBindFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBindInfo {
  pub image: VkImage,
  pub bindCount: u32,
  pub pBinds: *const VkSparseImageMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindSparseInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub waitSemaphoreCount: u32,
  pub pWaitSemaphores: *const VkSemaphore,
  pub bufferBindCount: u32,
  pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
  pub imageOpaqueBindCount: u32,
  pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
  pub imageBindCount: u32,
  pub pImageBinds: *const VkSparseImageMemoryBindInfo,
  pub signalSemaphoreCount: u32,
  pub pSignalSemaphores: *const VkSemaphore,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFenceCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkFenceCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkSemaphoreCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkEventCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkEventCreateFlags,
}
pub type VkEvent = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueryPoolCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkQueryPoolCreateFlags,
  pub queryType: VkQueryType,
  pub queryCount: u32,
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}
pub type VkQueryPool = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkBufferCreateFlags,
  pub size: VkDeviceSize,
  pub usage: VkBufferUsageFlags,
  pub sharingMode: VkSharingMode,
  pub queueFamilyIndexCount: u32,
  pub pQueueFamilyIndices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferViewCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkBufferViewCreateFlags,
  pub buffer: VkBuffer,
  pub format: VkFormat,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
}
pub type VkBufferView = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkImageCreateFlags,
  pub imageType: VkImageType,
  pub format: VkFormat,
  pub extent: VkExtent3D,
  pub mipLevels: u32,
  pub arrayLayers: u32,
  pub samples: VkSampleCountFlagBits,
  pub tiling: VkImageTiling,
  pub usage: VkImageUsageFlags,
  pub sharingMode: VkSharingMode,
  pub queueFamilyIndexCount: u32,
  pub pQueueFamilyIndices: *const u32,
  pub initialLayout: VkImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubresourceLayout {
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub rowPitch: VkDeviceSize,
  pub arrayPitch: VkDeviceSize,
  pub depthPitch: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComponentMapping {
  pub r: VkComponentSwizzle,
  pub g: VkComponentSwizzle,
  pub b: VkComponentSwizzle,
  pub a: VkComponentSwizzle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub viewType: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresourceRange: VkImageSubresourceRange,
}
pub type VkImageView = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShaderModuleCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkShaderModuleCreateFlags,
  pub codeSize: usize,
  pub pCode: *const u32,
}
pub type VkShaderModule = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineCacheCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineCacheCreateFlags,
  pub initialDataSize: usize,
  pub pInitialData: *const c_void,
}
pub type VkPipelineCache = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSpecializationMapEntry {
  pub constantID: u32,
  pub offset: u32,
  pub size: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSpecializationInfo {
  pub mapEntryCount: u32,
  pub pMapEntries: *const VkSpecializationMapEntry,
  pub dataSize: usize,
  pub pData: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineShaderStageCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineShaderStageCreateFlags,
  pub stage: VkShaderStageFlagBits,
  pub module: VkShaderModule,
  pub pName: *const c_char,
  pub pSpecializationInfo: *const VkSpecializationInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputBindingDescription {
  pub binding: u32,
  pub stride: u32,
  pub inputRate: VkVertexInputRate,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputAttributeDescription {
  pub location: u32,
  pub binding: u32,
  pub format: VkFormat,
  pub offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineVertexInputStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineVertexInputStateCreateFlags,
  pub vertexBindingDescriptionCount: u32,
  pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
  pub vertexAttributeDescriptionCount: u32,
  pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  pub primitiveRestartEnable: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineTessellationStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patchControlPoints: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkViewport {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
  pub minDepth: f32,
  pub maxDepth: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset2D {
  pub x: i32,
  pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent2D {
  pub width: u32,
  pub height: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineViewportStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineViewportStateCreateFlags,
  pub viewportCount: u32,
  pub pViewports: *const VkViewport,
  pub scissorCount: u32,
  pub pScissors: *const VkRect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineRasterizationStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineRasterizationStateCreateFlags,
  pub depthClampEnable: VkBool32,
  pub rasterizerDiscardEnable: VkBool32,
  pub polygonMode: VkPolygonMode,
  pub cullMode: VkCullModeFlags,
  pub frontFace: VkFrontFace,
  pub depthBiasEnable: VkBool32,
  pub depthBiasConstantFactor: f32,
  pub depthBiasClamp: f32,
  pub depthBiasSlopeFactor: f32,
  pub lineWidth: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineMultisampleStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineMultisampleStateCreateFlags,
  pub rasterizationSamples: VkSampleCountFlagBits,
  pub sampleShadingEnable: VkBool32,
  pub minSampleShading: f32,
  pub pSampleMask: *const VkSampleMask,
  pub alphaToCoverageEnable: VkBool32,
  pub alphaToOneEnable: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkStencilOpState {
  pub failOp: VkStencilOp,
  pub passOp: VkStencilOp,
  pub depthFailOp: VkStencilOp,
  pub compareOp: VkCompareOp,
  pub compareMask: u32,
  pub writeMask: u32,
  pub reference: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDepthStencilStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineDepthStencilStateCreateFlags,
  pub depthTestEnable: VkBool32,
  pub depthWriteEnable: VkBool32,
  pub depthCompareOp: VkCompareOp,
  pub depthBoundsTestEnable: VkBool32,
  pub stencilTestEnable: VkBool32,
  pub front: VkStencilOpState,
  pub back: VkStencilOpState,
  pub minDepthBounds: f32,
  pub maxDepthBounds: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendAttachmentState {
  pub blendEnable: VkBool32,
  pub srcColorBlendFactor: VkBlendFactor,
  pub dstColorBlendFactor: VkBlendFactor,
  pub colorBlendOp: VkBlendOp,
  pub srcAlphaBlendFactor: VkBlendFactor,
  pub dstAlphaBlendFactor: VkBlendFactor,
  pub alphaBlendOp: VkBlendOp,
  pub colorWriteMask: VkColorComponentFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineColorBlendStateCreateFlags,
  pub logicOpEnable: VkBool32,
  pub logicOp: VkLogicOp,
  pub attachmentCount: u32,
  pub pAttachments: *const VkPipelineColorBlendAttachmentState,
  pub blendConstants: [f32; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDynamicStateCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineDynamicStateCreateFlags,
  pub dynamicStateCount: u32,
  pub pDynamicStates: *const VkDynamicState,
}
pub type VkPipelineLayout = u64;
pub type VkRenderPass = u64;
pub type VkPipeline = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGraphicsPipelineCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineCreateFlags,
  pub stageCount: u32,
  pub pStages: *const VkPipelineShaderStageCreateInfo,
  pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
  pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
  pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
  pub pViewportState: *const VkPipelineViewportStateCreateInfo,
  pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
  pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
  pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
  pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
  pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
  pub layout: VkPipelineLayout,
  pub renderPass: VkRenderPass,
  pub subpass: u32,
  pub basePipelineHandle: VkPipeline,
  pub basePipelineIndex: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComputePipelineCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineCreateFlags,
  pub stage: VkPipelineShaderStageCreateInfo,
  pub layout: VkPipelineLayout,
  pub basePipelineHandle: VkPipeline,
  pub basePipelineIndex: i32,
}
pub type VkDescriptorSetLayout = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPushConstantRange {
  pub stageFlags: VkShaderStageFlags,
  pub offset: u32,
  pub size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineLayoutCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineLayoutCreateFlags,
  pub setLayoutCount: u32,
  pub pSetLayouts: *const VkDescriptorSetLayout,
  pub pushConstantRangeCount: u32,
  pub pPushConstantRanges: *const VkPushConstantRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkSamplerCreateFlags,
  pub magFilter: VkFilter,
  pub minFilter: VkFilter,
  pub mipmapMode: VkSamplerMipmapMode,
  pub addressModeU: VkSamplerAddressMode,
  pub addressModeV: VkSamplerAddressMode,
  pub addressModeW: VkSamplerAddressMode,
  pub mipLodBias: f32,
  pub anisotropyEnable: VkBool32,
  pub maxAnisotropy: f32,
  pub compareEnable: VkBool32,
  pub compareOp: VkCompareOp,
  pub minLod: f32,
  pub maxLod: f32,
  pub borderColor: VkBorderColor,
  pub unnormalizedCoordinates: VkBool32,
}
pub type VkSampler = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutBinding {
  pub binding: u32,
  pub descriptorType: VkDescriptorType,
  pub descriptorCount: u32,
  pub stageFlags: VkShaderStageFlags,
  pub pImmutableSamplers: *const VkSampler,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkDescriptorSetLayoutCreateFlags,
  pub bindingCount: u32,
  pub pBindings: *const VkDescriptorSetLayoutBinding,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolSize {
  pub eType: VkDescriptorType,
  pub descriptorCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkDescriptorPoolCreateFlags,
  pub maxSets: u32,
  pub poolSizeCount: u32,
  pub pPoolSizes: *const VkDescriptorPoolSize,
}
pub type VkDescriptorPool = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetAllocateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub descriptorPool: VkDescriptorPool,
  pub descriptorSetCount: u32,
  pub pSetLayouts: *const VkDescriptorSetLayout,
}
pub type VkDescriptorSet = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorImageInfo {
  pub sampler: VkSampler,
  pub imageView: VkImageView,
  pub imageLayout: VkImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorBufferInfo {
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWriteDescriptorSet {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub dstSet: VkDescriptorSet,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
  pub descriptorType: VkDescriptorType,
  pub pImageInfo: *const VkDescriptorImageInfo,
  pub pBufferInfo: *const VkDescriptorBufferInfo,
  pub pTexelBufferView: *const VkBufferView,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyDescriptorSet {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub srcSet: VkDescriptorSet,
  pub srcBinding: u32,
  pub srcArrayElement: u32,
  pub dstSet: VkDescriptorSet,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkFramebufferCreateFlags,
  pub renderPass: VkRenderPass,
  pub attachmentCount: u32,
  pub pAttachments: *const VkImageView,
  pub width: u32,
  pub height: u32,
  pub layers: u32,
}
pub type VkFramebuffer = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescription {
  pub flags: VkAttachmentDescriptionFlags,
  pub format: VkFormat,
  pub samples: VkSampleCountFlagBits,
  pub loadOp: VkAttachmentLoadOp,
  pub storeOp: VkAttachmentStoreOp,
  pub stencilLoadOp: VkAttachmentLoadOp,
  pub stencilStoreOp: VkAttachmentStoreOp,
  pub initialLayout: VkImageLayout,
  pub finalLayout: VkImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReference {
  pub attachment: u32,
  pub layout: VkImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescription {
  pub flags: VkSubpassDescriptionFlags,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub inputAttachmentCount: u32,
  pub pInputAttachments: *const VkAttachmentReference,
  pub colorAttachmentCount: u32,
  pub pColorAttachments: *const VkAttachmentReference,
  pub pResolveAttachments: *const VkAttachmentReference,
  pub pDepthStencilAttachment: *const VkAttachmentReference,
  pub preserveAttachmentCount: u32,
  pub pPreserveAttachments: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDependency {
  pub srcSubpass: u32,
  pub dstSubpass: u32,
  pub srcStageMask: VkPipelineStageFlags,
  pub dstStageMask: VkPipelineStageFlags,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub dependencyFlags: VkDependencyFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkRenderPassCreateFlags,
  pub attachmentCount: u32,
  pub pAttachments: *const VkAttachmentDescription,
  pub subpassCount: u32,
  pub pSubpasses: *const VkSubpassDescription,
  pub dependencyCount: u32,
  pub pDependencies: *const VkSubpassDependency,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandPoolCreateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkCommandPoolCreateFlags,
  pub queueFamilyIndex: u32,
}
pub type VkCommandPool = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferAllocateInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub commandPool: VkCommandPool,
  pub level: VkCommandBufferLevel,
  pub commandBufferCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferInheritanceInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub renderPass: VkRenderPass,
  pub subpass: u32,
  pub framebuffer: VkFramebuffer,
  pub occlusionQueryEnable: VkBool32,
  pub queryFlags: VkQueryControlFlags,
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferBeginInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkCommandBufferUsageFlags,
  pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCopy {
  pub srcOffset: VkDeviceSize,
  pub dstOffset: VkDeviceSize,
  pub size: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceLayers {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCopy {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageBlit {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffsets: [VkOffset3D; 2],
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffsets: [VkOffset3D; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferImageCopy {
  pub bufferOffset: VkDeviceSize,
  pub bufferRowLength: u32,
  pub bufferImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  pub imageOffset: VkOffset3D,
  pub imageExtent: VkExtent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
  pub float32: [f32; 4],
  pub int32: [i32; 4],
  pub uint32: [u32; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearDepthStencilValue {
  pub depth: f32,
  pub stencil: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
  pub color: VkClearColorValue,
  pub depthStencil: VkClearDepthStencilValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearAttachment {
  pub aspectMask: VkImageAspectFlags,
  pub colorAttachment: u32,
  pub clearValue: VkClearValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearRect {
  pub rect: VkRect2D,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageResolve {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassBeginInfo {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub renderPass: VkRenderPass,
  pub framebuffer: VkFramebuffer,
  pub renderArea: VkRect2D,
  pub clearValueCount: u32,
  pub pClearValues: *const VkClearValue,
}

// feature: VK_KHR_surface
#[cfg(feature = "VK_KHR_surface")]
pub type VkSurfaceKHR = u64;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_surface")]
pub struct VkSurfaceCapabilitiesKHR {
  pub minImageCount: u32,
  pub maxImageCount: u32,
  pub currentExtent: VkExtent2D,
  pub minImageExtent: VkExtent2D,
  pub maxImageExtent: VkExtent2D,
  pub maxImageArrayLayers: u32,
  pub supportedTransforms: VkSurfaceTransformFlagsKHR,
  pub currentTransform: VkSurfaceTransformFlagBitsKHR,
  pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
  pub supportedUsageFlags: VkImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_surface")]
pub struct VkSurfaceFormatKHR {
  pub format: VkFormat,
  pub colorSpace: VkColorSpaceKHR,
}

// feature: VK_KHR_swapchain
#[cfg(feature = "VK_KHR_swapchain")]
pub type VkSwapchainKHR = u64;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_swapchain")]
pub struct VkSwapchainCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkSwapchainCreateFlagsKHR,
  pub surface: VkSurfaceKHR,
  pub minImageCount: u32,
  pub imageFormat: VkFormat,
  pub imageColorSpace: VkColorSpaceKHR,
  pub imageExtent: VkExtent2D,
  pub imageArrayLayers: u32,
  pub imageUsage: VkImageUsageFlags,
  pub imageSharingMode: VkSharingMode,
  pub queueFamilyIndexCount: u32,
  pub pQueueFamilyIndices: *const u32,
  pub preTransform: VkSurfaceTransformFlagBitsKHR,
  pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
  pub presentMode: VkPresentModeKHR,
  pub clipped: VkBool32,
  pub oldSwapchain: VkSwapchainKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_swapchain")]
pub struct VkPresentInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub waitSemaphoreCount: u32,
  pub pWaitSemaphores: *const VkSemaphore,
  pub swapchainCount: u32,
  pub pSwapchains: *const VkSwapchainKHR,
  pub pImageIndices: *const u32,
  pub pResults: *mut VkResult,
}

// feature: VK_KHR_display
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayKHR = u64;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayPropertiesKHR {
  pub display: VkDisplayKHR,
  pub displayName: *const c_char,
  pub physicalDimensions: VkExtent2D,
  pub physicalResolution: VkExtent2D,
  pub supportedTransforms: VkSurfaceTransformFlagsKHR,
  pub planeReorderPossible: VkBool32,
  pub persistentContent: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModeParametersKHR {
  pub visibleRegion: VkExtent2D,
  pub refreshRate: u32,
}
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayModeKHR = u64;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModePropertiesKHR {
  pub displayMode: VkDisplayModeKHR,
  pub parameters: VkDisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModeCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkDisplayModeCreateFlagsKHR,
  pub parameters: VkDisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayPlaneCapabilitiesKHR {
  pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
  pub minSrcPosition: VkOffset2D,
  pub maxSrcPosition: VkOffset2D,
  pub minSrcExtent: VkExtent2D,
  pub maxSrcExtent: VkExtent2D,
  pub minDstPosition: VkOffset2D,
  pub maxDstPosition: VkOffset2D,
  pub minDstExtent: VkExtent2D,
  pub maxDstExtent: VkExtent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayPlanePropertiesKHR {
  pub currentDisplay: VkDisplayKHR,
  pub currentStackIndex: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplaySurfaceCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkDisplaySurfaceCreateFlagsKHR,
  pub displayMode: VkDisplayModeKHR,
  pub planeIndex: u32,
  pub planeStackIndex: u32,
  pub transform: VkSurfaceTransformFlagBitsKHR,
  pub globalAlpha: f32,
  pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
  pub imageExtent: VkExtent2D,
}

// feature: VK_KHR_display_swapchain
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display_swapchain")]
pub struct VkDisplayPresentInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub srcRect: VkRect2D,
  pub dstRect: VkRect2D,
  pub persistent: VkBool32,
}

// feature: VK_KHR_xlib_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub struct VkXlibSurfaceCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkXlibSurfaceCreateFlagsKHR,
  pub dpy: *mut wsi::xlib::Display,
  pub window: wsi::xlib::Window,
}

// feature: VK_KHR_xcb_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub struct VkXcbSurfaceCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkXcbSurfaceCreateFlagsKHR,
  pub connection: *mut wsi::xcb::xcb_connection_t,
  pub window: wsi::xcb::xcb_window_t,
}

// feature: VK_KHR_wayland_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub struct VkWaylandSurfaceCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkWaylandSurfaceCreateFlagsKHR,
  pub display: *mut wsi::wayland::wl_display,
  pub surface: *mut wsi::wayland::wl_surface,
}

// feature: VK_KHR_mir_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub struct VkMirSurfaceCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkMirSurfaceCreateFlagsKHR,
  pub connection: *mut wsi::mir::MirConnection,
  pub mirSurface: *mut wsi::mir::MirSurface,
}

// feature: VK_KHR_android_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub struct VkAndroidSurfaceCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkAndroidSurfaceCreateFlagsKHR,
  pub window: *mut wsi::android::ANativeWindow,
}

// feature: VK_KHR_win32_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32SurfaceCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkWin32SurfaceCreateFlagsKHR,
  pub hinstance: wsi::win32::HINSTANCE,
  pub hwnd: wsi::win32::HWND,
}

// feature: VK_EXT_debug_report
#[cfg(feature = "VK_EXT_debug_report")]
pub type PFN_vkDebugReportCallbackEXT = extern "system" fn(
  VkDebugReportFlagsEXT,
  VkDebugReportObjectTypeEXT,
  u64,
  usize,
  i32,
  *const c_char,
  *const c_char,
  *mut c_void,
) -> VkBool32;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_debug_report")]
pub struct VkDebugReportCallbackCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkDebugReportFlagsEXT,
  pub pfnCallback: PFN_vkDebugReportCallbackEXT,
  pub pUserData: *mut c_void,
}
#[cfg(feature = "VK_EXT_debug_report")]
pub type VkDebugReportCallbackEXT = u64;

// feature: VK_AMD_rasterization_order
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_AMD_rasterization_order")]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub rasterizationOrder: VkRasterizationOrderAMD,
}

// feature: VK_EXT_debug_marker
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerObjectNameInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub objectType: VkDebugReportObjectTypeEXT,
  pub object: u64,
  pub pObjectName: *const c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerObjectTagInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub objectType: VkDebugReportObjectTypeEXT,
  pub object: u64,
  pub tagName: u64,
  pub tagSize: usize,
  pub pTag: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerMarkerInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub pMarkerName: *const c_char,
  pub color: [f32; 4],
}

// feature: VK_NV_dedicated_allocation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationImageCreateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub dedicatedAllocation: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub dedicatedAllocation: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub image: VkImage,
  pub buffer: VkBuffer,
}

// feature: VK_KHR_get_physical_device_properties2
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceFeatures2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub features: VkPhysicalDeviceFeatures,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceProperties2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub properties: VkPhysicalDeviceProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkFormatProperties2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub formatProperties: VkFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkImageFormatProperties2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub imageFormatProperties: VkImageFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceImageFormatInfo2KHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub format: VkFormat,
  pub eType: VkImageType,
  pub tiling: VkImageTiling,
  pub usage: VkImageUsageFlags,
  pub flags: VkImageCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkQueueFamilyProperties2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub queueFamilyProperties: VkQueueFamilyProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkSparseImageFormatProperties2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub properties: VkSparseImageFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub format: VkFormat,
  pub eType: VkImageType,
  pub samples: VkSampleCountFlagBits,
  pub usage: VkImageUsageFlags,
  pub tiling: VkImageTiling,
}

// feature: VK_AMD_texture_gather_bias_lod
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
pub struct VkTextureLODGatherFormatPropertiesAMD {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub supportsTextureGatherLODBiasAMD: VkBool32,
}

// feature: VK_AMD_shader_info
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_AMD_shader_info")]
pub struct VkShaderResourceUsageAMD {
  pub numUsedVgprs: u32,
  pub numUsedSgprs: u32,
  pub ldsSizePerLocalWorkGroup: u32,
  pub ldsUsageSizeInBytes: usize,
  pub scratchMemUsageInBytes: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_AMD_shader_info")]
pub struct VkShaderStatisticsInfoAMD {
  pub shaderStageMask: VkShaderStageFlags,
  pub resourceUsage: VkShaderResourceUsageAMD,
  pub numPhysicalVgprs: u32,
  pub numPhysicalSgprs: u32,
  pub numAvailableVgprs: u32,
  pub numAvailableSgprs: u32,
  pub computeWorkGroupSize: [u32; 3],
}

// feature: VK_KHX_multiview
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkRenderPassMultiviewCreateInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub subpassCount: u32,
  pub pViewMasks: *const u32,
  pub dependencyCount: u32,
  pub pViewOffsets: *const i32,
  pub correlationMaskCount: u32,
  pub pCorrelationMasks: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkPhysicalDeviceMultiviewFeaturesKHX {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub multiview: VkBool32,
  pub multiviewGeometryShader: VkBool32,
  pub multiviewTessellationShader: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkPhysicalDeviceMultiviewPropertiesKHX {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub maxMultiviewViewCount: u32,
  pub maxMultiviewInstanceIndex: u32,
}

// feature: VK_NV_external_memory_capabilities
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub struct VkExternalImageFormatPropertiesNV {
  pub imageFormatProperties: VkImageFormatProperties,
  pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
  pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
  pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

// feature: VK_NV_external_memory
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory")]
pub struct VkExternalMemoryImageCreateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory")]
pub struct VkExportMemoryAllocateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

// feature: VK_NV_external_memory_win32
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportMemoryWin32HandleInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagsNV,
  pub handle: wsi::win32::HANDLE,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportMemoryWin32HandleInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
}

// feature: VK_NV_win32_keyed_mutex
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub acquireCount: u32,
  pub pAcquireSyncs: *const VkDeviceMemory,
  pub pAcquireKeys: *const u64,
  pub pAcquireTimeoutMilliseconds: *const u32,
  pub releaseCount: u32,
  pub pReleaseSyncs: *const VkDeviceMemory,
  pub pReleaseKeys: *const u64,
}

// feature: VK_KHX_device_group_creation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group_creation")]
pub struct VkPhysicalDeviceGroupPropertiesKHX {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub physicalDeviceCount: u32,
  pub physicalDevices: [VkPhysicalDevice; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize],
  pub subsetAllocation: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group_creation")]
pub struct VkDeviceGroupDeviceCreateInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub physicalDeviceCount: u32,
  pub pPhysicalDevices: *const VkPhysicalDevice,
}

// feature: VK_KHX_device_group
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkMemoryAllocateFlagsInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkMemoryAllocateFlagsKHX,
  pub deviceMask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupRenderPassBeginInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub deviceMask: u32,
  pub deviceRenderAreaCount: u32,
  pub pDeviceRenderAreas: *const VkRect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupCommandBufferBeginInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub deviceMask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupSubmitInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub waitSemaphoreCount: u32,
  pub pWaitSemaphoreDeviceIndices: *const u32,
  pub commandBufferCount: u32,
  pub pCommandBufferDeviceMasks: *const u32,
  pub signalSemaphoreCount: u32,
  pub pSignalSemaphoreDeviceIndices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupBindSparseInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub resourceDeviceIndex: u32,
  pub memoryDeviceIndex: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindBufferMemoryDeviceGroupInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub deviceIndexCount: u32,
  pub pDeviceIndices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindImageMemoryDeviceGroupInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub deviceIndexCount: u32,
  pub pDeviceIndices: *const u32,
  pub SFRRectCount: u32,
  pub pSFRRects: *const VkRect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupPresentCapabilitiesKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub presentMask: [u32; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize],
  pub modes: VkDeviceGroupPresentModeFlagsKHX,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkImageSwapchainCreateInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub swapchain: VkSwapchainKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindImageMemorySwapchainInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub swapchain: VkSwapchainKHR,
  pub imageIndex: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkAcquireNextImageInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub swapchain: VkSwapchainKHR,
  pub timeout: u64,
  pub semaphore: VkSemaphore,
  pub fence: VkFence,
  pub deviceMask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupPresentInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub swapchainCount: u32,
  pub pDeviceMasks: *const u32,
  pub mode: VkDeviceGroupPresentModeFlagBitsKHX,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupSwapchainCreateInfoKHX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub modes: VkDeviceGroupPresentModeFlagsKHX,
}

// feature: VK_EXT_validation_flags
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_validation_flags")]
pub struct VkValidationFlagsEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub disabledValidationCheckCount: u32,
  pub pDisabledValidationChecks: *mut VkValidationCheckEXT,
}

// feature: VK_NN_vi_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub struct VkViSurfaceCreateInfoNN {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkViSurfaceCreateFlagsNN,
  pub window: *mut c_void,
}

// feature: VK_KHR_external_memory_capabilities
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkExternalMemoryPropertiesKHR {
  pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsKHR,
  pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
  pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkExternalImageFormatPropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceExternalBufferInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkBufferCreateFlags,
  pub usage: VkBufferUsageFlags,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkExternalBufferPropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceIDPropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub deviceUUID: [u8; enums::VK_UUID_SIZE as usize],
  pub driverUUID: [u8; enums::VK_UUID_SIZE as usize],
  pub deviceLUID: [u8; enums::VK_LUID_SIZE_KHR as usize],
  pub deviceNodeMask: u32,
  pub deviceLUIDValid: VkBool32,
}

// feature: VK_KHR_external_memory
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExternalMemoryImageCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExternalMemoryBufferCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExportMemoryAllocateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}

// feature: VK_KHR_external_memory_win32
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportMemoryWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportMemoryWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkMemoryWin32HandlePropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkMemoryGetWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}

// feature: VK_KHR_external_memory_fd
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkImportMemoryFdInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub fd: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkMemoryFdPropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkMemoryGetFdInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}

// feature: VK_KHR_win32_keyed_mutex
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub acquireCount: u32,
  pub pAcquireSyncs: *const VkDeviceMemory,
  pub pAcquireKeys: *const u64,
  pub pAcquireTimeouts: *const u32,
  pub releaseCount: u32,
  pub pReleaseSyncs: *const VkDeviceMemory,
  pub pReleaseKeys: *const u64,
}

// feature: VK_KHR_external_semaphore_capabilities
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub struct VkExternalSemaphorePropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
  pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
  pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHR,
}

// feature: VK_KHR_external_semaphore
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore")]
pub struct VkExportSemaphoreCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
}

// feature: VK_KHR_external_semaphore_win32
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub flags: VkSemaphoreImportFlagsKHR,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkD3D12FenceSubmitInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub waitSemaphoreValuesCount: u32,
  pub pWaitSemaphoreValues: *const u64,
  pub signalSemaphoreValuesCount: u32,
  pub pSignalSemaphoreValues: *const u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}

// feature: VK_KHR_external_semaphore_fd
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub struct VkImportSemaphoreFdInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub flags: VkSemaphoreImportFlagsKHR,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  pub fd: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub struct VkSemaphoreGetFdInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}

// feature: VK_KHR_push_descriptor
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_push_descriptor")]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub maxPushDescriptors: u32,
}

// feature: VK_KHR_16bit_storage
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_16bit_storage")]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub storageBuffer16BitAccess: VkBool32,
  pub uniformAndStorageBuffer16BitAccess: VkBool32,
  pub storagePushConstant16: VkBool32,
  pub storageInputOutput16: VkBool32,
}

// feature: VK_KHR_incremental_present
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkRectLayerKHR {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
  pub layer: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkPresentRegionKHR {
  pub rectangleCount: u32,
  pub pRectangles: *const VkRectLayerKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkPresentRegionsKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub swapchainCount: u32,
  pub pRegions: *const VkPresentRegionKHR,
}

// feature: VK_KHR_descriptor_update_template
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type VkDescriptorUpdateTemplateKHR = u64;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub struct VkDescriptorUpdateTemplateEntryKHR {
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
  pub descriptorType: VkDescriptorType,
  pub offset: usize,
  pub stride: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
  pub descriptorUpdateEntryCount: u32,
  pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntryKHR,
  pub templateType: VkDescriptorUpdateTemplateTypeKHR,
  pub descriptorSetLayout: VkDescriptorSetLayout,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub pipelineLayout: VkPipelineLayout,
  pub set: u32,
}

// feature: VK_NVX_device_generated_commands
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkObjectTableNVX = u64;
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkIndirectCommandsLayoutNVX = u64;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkDeviceGeneratedCommandsFeaturesNVX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub computeBindingPointSupport: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkDeviceGeneratedCommandsLimitsNVX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub maxIndirectCommandsLayoutTokenCount: u32,
  pub maxObjectEntryCounts: u32,
  pub minSequenceCountBufferOffsetAlignment: u32,
  pub minSequenceIndexBufferOffsetAlignment: u32,
  pub minCommandsTokenBufferOffsetAlignment: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsTokenNVX {
  pub tokenType: VkIndirectCommandsTokenTypeNVX,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsLayoutTokenNVX {
  pub tokenType: VkIndirectCommandsTokenTypeNVX,
  pub bindingUnit: u32,
  pub dynamicCount: u32,
  pub divisor: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsLayoutCreateInfoNVX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub flags: VkIndirectCommandsLayoutUsageFlagsNVX,
  pub tokenCount: u32,
  pub pTokens: *const VkIndirectCommandsLayoutTokenNVX,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkCmdProcessCommandsInfoNVX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub objectTable: VkObjectTableNVX,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
  pub indirectCommandsTokenCount: u32,
  pub pIndirectCommandsTokens: *const VkIndirectCommandsTokenNVX,
  pub maxSequencesCount: u32,
  pub targetCommandBuffer: VkCommandBuffer,
  pub sequencesCountBuffer: VkBuffer,
  pub sequencesCountOffset: VkDeviceSize,
  pub sequencesIndexBuffer: VkBuffer,
  pub sequencesIndexOffset: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkCmdReserveSpaceForCommandsInfoNVX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub objectTable: VkObjectTableNVX,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
  pub maxSequencesCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableCreateInfoNVX {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub objectCount: u32,
  pub pObjectEntryTypes: *const VkObjectEntryTypeNVX,
  pub pObjectEntryCounts: *const u32,
  pub pObjectEntryUsageFlags: *const VkObjectEntryUsageFlagsNVX,
  pub maxUniformBuffersPerDescriptor: u32,
  pub maxStorageBuffersPerDescriptor: u32,
  pub maxStorageImagesPerDescriptor: u32,
  pub maxSampledImagesPerDescriptor: u32,
  pub maxPipelineLayouts: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTablePipelineEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub pipeline: VkPipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableDescriptorSetEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub pipelineLayout: VkPipelineLayout,
  pub descriptorSet: VkDescriptorSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableVertexBufferEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub buffer: VkBuffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableIndexBufferEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub buffer: VkBuffer,
  pub indexType: VkIndexType,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTablePushConstantEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub pipelineLayout: VkPipelineLayout,
  pub stageFlags: VkShaderStageFlags,
}

// feature: VK_NV_clip_space_w_scaling
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub struct VkViewportWScalingNV {
  pub xcoeff: f32,
  pub ycoeff: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub viewportWScalingEnable: VkBool32,
  pub viewportCount: u32,
  pub pViewportWScalings: *const VkViewportWScalingNV,
}

// feature: VK_EXT_display_surface_counter
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub struct VkSurfaceCapabilities2EXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub minImageCount: u32,
  pub maxImageCount: u32,
  pub currentExtent: VkExtent2D,
  pub minImageExtent: VkExtent2D,
  pub maxImageExtent: VkExtent2D,
  pub maxImageArrayLayers: u32,
  pub supportedTransforms: VkSurfaceTransformFlagsKHR,
  pub currentTransform: VkSurfaceTransformFlagBitsKHR,
  pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
  pub supportedUsageFlags: VkImageUsageFlags,
  pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
}

// feature: VK_EXT_display_control
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDisplayPowerInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub powerState: VkDisplayPowerStateEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDeviceEventInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub deviceEvent: VkDeviceEventTypeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDisplayEventInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub displayEvent: VkDisplayEventTypeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkSwapchainCounterCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub surfaceCounters: VkSurfaceCounterFlagsEXT,
}

// feature: VK_GOOGLE_display_timing
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkRefreshCycleDurationGOOGLE {
  pub refreshDuration: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkPastPresentationTimingGOOGLE {
  pub presentID: u32,
  pub desiredPresentTime: u64,
  pub actualPresentTime: u64,
  pub earliestPresentTime: u64,
  pub presentMargin: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkPresentTimeGOOGLE {
  pub presentID: u32,
  pub desiredPresentTime: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkPresentTimesInfoGOOGLE {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub swapchainCount: u32,
  pub pTimes: *const VkPresentTimeGOOGLE,
}

// feature: VK_NVX_multiview_per_view_attributes
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub perViewPositionAllComponents: VkBool32,
}

// feature: VK_NV_viewport_swizzle
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub struct VkViewportSwizzleNV {
  pub x: VkViewportCoordinateSwizzleNV,
  pub y: VkViewportCoordinateSwizzleNV,
  pub z: VkViewportCoordinateSwizzleNV,
  pub w: VkViewportCoordinateSwizzleNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
  pub viewportCount: u32,
  pub pViewportSwizzles: *const VkViewportSwizzleNV,
}

// feature: VK_EXT_discard_rectangles
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub maxDiscardRectangles: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
  pub discardRectangleMode: VkDiscardRectangleModeEXT,
  pub discardRectangleCount: u32,
  pub pDiscardRectangles: *const VkRect2D,
}

// feature: VK_EXT_conservative_rasterization
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub primitiveOverestimationSize: f32,
  pub maxExtraPrimitiveOverestimationSize: f32,
  pub extraPrimitiveOverestimationSizeGranularity: f32,
  pub primitiveUnderestimation: VkBool32,
  pub conservativePointAndLineRasterization: VkBool32,
  pub degenerateTrianglesRasterized: VkBool32,
  pub degenerateLinesRasterized: VkBool32,
  pub fullyCoveredFragmentShaderInputVariable: VkBool32,
  pub conservativeRasterizationPostDepthCoverage: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
  pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
  pub extraPrimitiveOverestimationSize: f32,
}

// feature: VK_EXT_hdr_metadata
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub struct VkXYColorEXT {
  pub x: f32,
  pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub struct VkHdrMetadataEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub displayPrimaryRed: VkXYColorEXT,
  pub displayPrimaryGreen: VkXYColorEXT,
  pub displayPrimaryBlue: VkXYColorEXT,
  pub whitePoint: VkXYColorEXT,
  pub maxLuminance: f32,
  pub minLuminance: f32,
  pub maxContentLightLevel: f32,
  pub maxFrameAverageLightLevel: f32,
}

// feature: VK_KHR_get_surface_capabilities2
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub surface: VkSurfaceKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub struct VkSurfaceCapabilities2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub struct VkSurfaceFormat2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub surfaceFormat: VkSurfaceFormatKHR,
}

// feature: VK_KHR_shared_presentable_image
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub sharedPresentSupportedUsageFlags: VkImageUsageFlags,
}

// feature: VK_KHR_external_fence_capabilities
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub struct VkPhysicalDeviceExternalFenceInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub struct VkExternalFencePropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
  pub compatibleHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
  pub externalFenceFeatures: VkExternalFenceFeatureFlagsKHR,
}

// feature: VK_KHR_external_fence
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence")]
pub struct VkExportFenceCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleTypes: VkExternalFenceHandleTypeFlagsKHR,
}

// feature: VK_KHR_external_fence_win32
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportFenceWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub flags: VkFenceImportFlagsKHR,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportFenceWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkFenceGetWin32HandleInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
}

// feature: VK_KHR_external_fence_fd
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub struct VkImportFenceFdInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub flags: VkFenceImportFlagsKHR,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  pub fd: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub struct VkFenceGetFdInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
}

// feature: VK_KHR_maintenance2
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkPhysicalDevicePointClippingPropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub pointClippingBehavior: VkPointClippingBehaviorKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkInputAttachmentAspectReferenceKHR {
  pub subpass: u32,
  pub inputAttachmentIndex: u32,
  pub aspectMask: VkImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub aspectReferenceCount: u32,
  pub pAspectReferences: *const VkInputAttachmentAspectReferenceKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkImageViewUsageCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub usage: VkImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub domainOrigin: VkTessellationDomainOriginKHR,
}

// feature: VK_KHR_variable_pointers
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_variable_pointers")]
pub struct VkPhysicalDeviceVariablePointerFeaturesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub variablePointersStorageBuffer: VkBool32,
  pub variablePointers: VkBool32,
}

// feature: VK_MVK_ios_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub struct VkIOSSurfaceCreateInfoMVK {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkIOSSurfaceCreateFlagsMVK,
  pub pView: *const c_void,
}

// feature: VK_MVK_macos_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub struct VkMacOSSurfaceCreateInfoMVK {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkMacOSSurfaceCreateFlagsMVK,
  pub pView: *const c_void,
}

// feature: VK_KHR_get_memory_requirements2
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkBufferMemoryRequirementsInfo2KHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkImageMemoryRequirementsInfo2KHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub image: VkImage,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkImageSparseMemoryRequirementsInfo2KHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub image: VkImage,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkMemoryRequirements2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub memoryRequirements: VkMemoryRequirements,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkSparseImageMemoryRequirements2KHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub memoryRequirements: VkSparseImageMemoryRequirements,
}

// feature: VK_KHR_dedicated_allocation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub struct VkMemoryDedicatedRequirementsKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub prefersDedicatedAllocation: VkBool32,
  pub requiresDedicatedAllocation: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub struct VkMemoryDedicatedAllocateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub image: VkImage,
  pub buffer: VkBuffer,
}

// feature: VK_EXT_sampler_filter_minmax
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub struct VkSamplerReductionModeCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub reductionMode: VkSamplerReductionModeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub filterMinmaxSingleComponentFormats: VkBool32,
  pub filterMinmaxImageComponentMapping: VkBool32,
}

// feature: VK_EXT_sample_locations
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSampleLocationEXT {
  pub x: f32,
  pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSampleLocationsInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub sampleLocationsPerPixel: VkSampleCountFlagBits,
  pub sampleLocationGridSize: VkExtent2D,
  pub sampleLocationsCount: u32,
  pub pSampleLocations: *const VkSampleLocationEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkAttachmentSampleLocationsEXT {
  pub attachmentIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSubpassSampleLocationsEXT {
  pub subpassIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub attachmentInitialSampleLocationsCount: u32,
  pub pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
  pub postSubpassSampleLocationsCount: u32,
  pub pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub sampleLocationsEnable: VkBool32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub sampleLocationSampleCounts: VkSampleCountFlags,
  pub maxSampleLocationGridSize: VkExtent2D,
  pub sampleLocationCoordinateRange: [f32; 2],
  pub sampleLocationSubPixelBits: u32,
  pub variableSampleLocations: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkMultisamplePropertiesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub maxSampleLocationGridSize: VkExtent2D,
}

// feature: VK_KHR_image_format_list
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_image_format_list")]
pub struct VkImageFormatListCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub viewFormatCount: u32,
  pub pViewFormats: *const VkFormat,
}

// feature: VK_EXT_blend_operation_advanced
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub advancedBlendCoherentOperations: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub advancedBlendMaxColorAttachments: u32,
  pub advancedBlendIndependentBlend: VkBool32,
  pub advancedBlendNonPremultipliedSrcColor: VkBool32,
  pub advancedBlendNonPremultipliedDstColor: VkBool32,
  pub advancedBlendCorrelatedOverlap: VkBool32,
  pub advancedBlendAllOperations: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub srcPremultiplied: VkBool32,
  pub dstPremultiplied: VkBool32,
  pub blendOverlap: VkBlendOverlapEXT,
}

// feature: VK_NV_fragment_coverage_to_color
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
  pub coverageToColorEnable: VkBool32,
  pub coverageToColorLocation: u32,
}

// feature: VK_NV_framebuffer_mixed_samples
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
  pub coverageModulationMode: VkCoverageModulationModeNV,
  pub coverageModulationTableEnable: VkBool32,
  pub coverageModulationTableCount: u32,
  pub pCoverageModulationTable: *const f32,
}

// feature: VK_KHR_bind_memory2
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_bind_memory2")]
pub struct VkBindBufferMemoryInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_bind_memory2")]
pub struct VkBindImageMemoryInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub image: VkImage,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
}

// feature: VK_KHR_sampler_ycbcr_conversion
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionCreateInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub format: VkFormat,
  pub ycbcrModel: VkSamplerYcbcrModelConversionKHR,
  pub ycbcrRange: VkSamplerYcbcrRangeKHR,
  pub components: VkComponentMapping,
  pub xChromaOffset: VkChromaLocationKHR,
  pub yChromaOffset: VkChromaLocationKHR,
  pub chromaFilter: VkFilter,
  pub forceExplicitReconstruction: VkBool32,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type VkSamplerYcbcrConversionKHR = u64;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub conversion: VkSamplerYcbcrConversionKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkBindImagePlaneMemoryInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub planeAspect: VkImageAspectFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkImagePlaneMemoryRequirementsInfoKHR {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub planeAspect: VkImageAspectFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub samplerYcbcrConversion: VkBool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub combinedImageSamplerDescriptorCount: u32,
}

// feature: VK_EXT_validation_cache
#[cfg(feature = "VK_EXT_validation_cache")]
pub type VkValidationCacheEXT = u64;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct VkValidationCacheCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub flags: VkValidationCacheCreateFlagsEXT,
  pub initialDataSize: usize,
  pub pInitialData: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub validationCache: VkValidationCacheEXT,
}

// feature: VK_EXT_global_priority
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_global_priority")]
pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub globalPriority: VkQueueGlobalPriorityEXT,
}

// feature: VK_EXT_external_memory_host
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_external_memory_host")]
pub struct VkImportMemoryHostPointerInfoEXT {
  pub sType: VkStructureType,
  pub pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub pHostPointer: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_external_memory_host")]
pub struct VkMemoryHostPointerPropertiesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_external_memory_host")]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  pub sType: VkStructureType,
  pub pNext: *mut c_void,
  pub minImportedHostPointerAlignment: VkDeviceSize,
}
