/*
Copyright (c) 2016, Christoph Hommelsheim
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use util;
use std::ffi::CStr;
use ffi;
pub use types::*;
pub use util::VkResultObj;

const NULL_ALLOCATOR: *const VkAllocationCallbacks = 0 as *const VkAllocationCallbacks;


macro_rules! vk_tryobj {
    ( $($e:expr );+ ) => {
        $(
            let res = unsafe { $e };
            if res != VK_SUCCESS {
                return Err(res);
            }
        )+
    };
}

impl VkInstance {
    // see vkCreateInstance
    #[inline]
    pub fn create(create_info: &VkInstanceCreateInfo) -> VkResultObj<VkInstance> {
        let mut instance: VkInstance = util::vk_null_handle();
        vk_tryobj!(ffi::vkCreateInstance(create_info, NULL_ALLOCATOR, &mut instance));
        return Ok(instance);
    }

    // see vkDestroyInstance
    #[inline]
    pub fn destroy(self) {
        unsafe { ffi::vkDestroyInstance(self, NULL_ALLOCATOR) }
    }

    // see vkEnumeratePhysicalDevices
    pub fn get_physical_devices(self) -> VkResultObj<Vec<VkPhysicalDevice>> {
        let mut device_count: u32 = 0;
        vk_tryobj!(ffi::vkEnumeratePhysicalDevices(self, &mut device_count, util::vk_null()));
        let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(device_count as usize);
        vk_tryobj!(ffi::vkEnumeratePhysicalDevices(self, &mut device_count, devices.as_mut_ptr()));
        unsafe { devices.set_len(device_count as usize) };
        return Ok(devices);
    }


    // see vkGetInstanceProcAddr
    #[inline]
    pub unsafe fn get_proc_addr(self, name: &CStr) -> PFN_vkVoidFunction {
        return ffi::vkGetInstanceProcAddr(self, name.as_ptr());
    }

    /* TODO:
pub fn vkEnumerateInstanceExtensionProperties(
    pLayerName:     *const c_char,
    pPropertyCount: *mut u32,
    pProperties:    *mut VkExtensionProperties)
    -> VkResult;

pub fn vkEnumerateInstanceLayerProperties(
    pPropertyCount: *mut u32,
    pProperties:    *mut VkLayerProperties)
    -> VkResult;
*/
}

impl VkPhysicalDevice {
    // see vkGetPhysicalDeviceFeatures
    #[inline]
    pub fn get_features(self, features: &mut VkPhysicalDeviceFeatures) {
        unsafe { ffi::vkGetPhysicalDeviceFeatures(self, features) };
    }

    // see vkGetPhysicalDeviceFormatProperties
    #[inline]
    pub fn get_format_properties(self,
                                 format: VkFormat,
                                 format_properties: &mut VkFormatProperties) {
        unsafe { ffi::vkGetPhysicalDeviceFormatProperties(self, format, format_properties) };
    }

    // see vkGetPhysicalDeviceProperties
    #[inline]
    pub fn get_properties(self, properties: &mut VkPhysicalDeviceProperties) {
        unsafe { ffi::vkGetPhysicalDeviceProperties(self, properties) };
    }

    // TODO: vkGetPhysicalDeviceImageFormatProperties

    // see vkGetPhysicalDeviceQueueFamilyProperties
    pub fn get_query_family_properties(self) -> Vec<VkQueueFamilyProperties> {
        unsafe {
            let mut prop_count: u32 = 0;
            ffi::vkGetPhysicalDeviceQueueFamilyProperties(self, &mut prop_count, util::vk_null());
            let mut props: Vec<VkQueueFamilyProperties> = Vec::with_capacity(prop_count as usize);
            ffi::vkGetPhysicalDeviceQueueFamilyProperties(self,
                                                          &mut prop_count,
                                                          props.as_mut_ptr());
            props.set_len(prop_count as usize);
            return props;
        }
    }

    // see vkGetPhysicalDeviceMemoryProperties
    #[inline]
    pub fn get_memory_properties(self, properties: &mut VkPhysicalDeviceMemoryProperties) {
        unsafe { ffi::vkGetPhysicalDeviceMemoryProperties(self, properties) };
    }

    // see vkCreateDevice
    #[inline]
    pub fn create_device(self, create_info: &VkDeviceCreateInfo) -> VkResultObj<VkDevice> {
        let mut device: VkDevice = util::vk_null_handle();
        vk_tryobj!(ffi::vkCreateDevice(self, create_info, NULL_ALLOCATOR, &mut device));
        return Ok(device);
    }

    /* TODO:
pub fn vkEnumerateDeviceExtensionProperties(
    physicalDevice: VkPhysicalDevice,
    pLayerName:     *const c_char,
    pPropertyCount: *mut u32,
    pProperties:    *mut VkExtensionProperties)
    -> VkResult;

pub fn vkEnumerateDeviceLayerProperties(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties:    *mut VkLayerProperties)
    -> VkResult;


pub fn vkGetPhysicalDeviceSparseImageFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format:         VkFormat,
    ptype:          VkImageType,
    samples:        VkSampleCountFlagBits,
    usage:          VkImageUsageFlags,
    tiling:         VkImageTiling,
    pPropertyCount: *mut u32,
    pProperties:    *mut VkSparseImageFormatProperties);
*/
}

impl VkDevice {
    // see vkGetDeviceProcAddr
    #[inline]
    pub unsafe fn get_proc_addr(self, name: &CStr) -> PFN_vkVoidFunction {
        return ffi::vkGetDeviceProcAddr(self, name.as_ptr());
    }

    // see vkDestroyDevice
    #[inline]
    pub fn destroy(self) {
        unsafe { ffi::vkDestroyDevice(self, NULL_ALLOCATOR) }
    }

    // see vkGetDeviceQueue
    #[inline]
    pub fn get_queue(self, queue_family_index: u32, queue_index: u32) -> VkQueue {
        let mut queue: VkQueue = util::vk_null_handle();
        unsafe { ffi::vkGetDeviceQueue(self, queue_family_index, queue_index, &mut queue) };
        return queue;
    }


    // see vkDeviceWaitIdle
    #[inline]
    pub fn wait_idle(self) -> VkResultObj<()> {
        vk_tryobj!(ffi::vkDeviceWaitIdle(self));
        return Ok(());
    }

    // see vkCreateFence
    #[inline]
    pub fn create_fence(self, create_info: &VkFenceCreateInfo) -> VkResultObj<VkFence> {
        let mut fence: VkFence = util::vk_null_handle();
        vk_tryobj!(ffi::vkCreateFence(self, create_info, NULL_ALLOCATOR, &mut fence));
        return Ok(fence);
    }

    // see vkDestroyFence
    #[inline]
    pub fn destroy_fence(self, fence: VkFence) {
        unsafe { ffi::vkDestroyFence(self, fence, NULL_ALLOCATOR) };
    }

    // see vkResetFences
    #[inline]
    pub fn reset_fences(self, fences: &[VkFence]) -> VkResultObj<()> {
        vk_tryobj!(ffi::vkResetFences(self, fences.len() as u32, fences.as_ptr()));
        return Ok(());
    }

    // see vkGetFenceStatus
    #[inline]
    pub fn get_fence_status(self, fence: VkFence) -> VkResultObj<bool> {
        let res = unsafe { ffi::vkGetFenceStatus(self, fence) };
        if res == VK_SUCCESS {
            return Ok(true);
        }
        if res == VK_NOT_READY {
            return Ok(false);
        }
        return Err(res);
    }

    // see vkWaitForFences
    #[inline]
    pub fn wait_for_fences(self,
                           fences: &[VkFence],
                           wait_all: bool,
                           timeout: u64)
                           -> VkResultObj<()> {
        vk_tryobj!(ffi::vkWaitForFences(self,
                                        fences.len() as u32,
                                        fences.as_ptr(),
                                        wait_all as VkBool32,
                                        timeout));
        return Ok(());
    }

    // see vkCreateSemaphore
    #[inline]
    pub fn create_semaphore(self, create_info: &VkSemaphoreCreateInfo) -> VkResultObj<VkSemaphore> {
        let mut semaphore: VkSemaphore = util::vk_null_handle();
        vk_tryobj!(ffi::vkCreateSemaphore(self, create_info, NULL_ALLOCATOR, &mut semaphore));
        return Ok(semaphore);
    }

    // see vkDestroySemaphore
    #[inline]
    pub fn destroy_semaphore(self, semaphore: VkSemaphore) {
        unsafe { ffi::vkDestroySemaphore(self, semaphore, NULL_ALLOCATOR) };
    }

    // see vkCreateEvent
    #[inline]
    pub fn create_event(self, create_info: &VkEventCreateInfo) -> VkResultObj<VkEvent> {
        let mut event: VkEvent = util::vk_null_handle();
        vk_tryobj!(ffi::vkCreateEvent(self, create_info, NULL_ALLOCATOR, &mut event));
        return Ok(event);
    }

    // see vkDestroyEvent
    #[inline]
    pub fn destroy_event(self, event: VkEvent) {
        unsafe { ffi::vkDestroyEvent(self, event, NULL_ALLOCATOR) };
    }

    // see vkGetEventStatus
    #[inline]
    pub fn get_event_status(self, event: VkEvent) -> VkResultObj<bool> {
        let res = unsafe { ffi::vkGetEventStatus(self, event) };
        if res == VK_EVENT_SET {
            return Ok(true);
        }
        if res == VK_EVENT_RESET {
            return Ok(false);
        }
        return Err(res);
    }

    // see vkSetEvent
    #[inline]
    pub fn set_event(self, event: VkEvent) -> VkResultObj<()> {
        vk_tryobj!(ffi::vkSetEvent(self, event));
        return Ok(());
    }

    // see vkResetEvent
    #[inline]
    pub fn reset_event(self, event: VkEvent) -> VkResultObj<()> {
        vk_tryobj!(ffi::vkResetEvent(self, event));
        return Ok(());
    }

    // see vkCreateCommandPool
    #[inline]
    pub fn create_command_pool(self,
                               create_info: &VkCommandPoolCreateInfo)
                               -> VkResultObj<VkCommandPool> {
        let mut command_pool: VkCommandPool = util::vk_null_handle();
        vk_tryobj!(ffi::vkCreateCommandPool(self, create_info, NULL_ALLOCATOR, &mut command_pool));
        return Ok(command_pool);
    }

    // see vkDestroyCommandPool
    #[inline]
    pub fn destroy_command_pool(self, command_pool: VkCommandPool) {
        unsafe { ffi::vkDestroyCommandPool(self, command_pool, NULL_ALLOCATOR) };
    }

    // see vkResetCommandPool
    #[inline]
    pub fn reset_command_pool(self,
                              command_pool: VkCommandPool,
                              flags: VkCommandPoolResetFlags)
                              -> VkResultObj<()> {
        vk_tryobj!(ffi::vkResetCommandPool(self, command_pool, flags));
        return Ok(());
    }

    // see vkAllocateCommandBuffers
    pub fn allocate_command_buffer(self,
                                   command_pool: VkCommandPool,
                                   level: VkCommandBufferLevel)
                                   -> VkResultObj<VkCommandBuffer> {
        let single_alloc_info = VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: util::vk_null(),
            commandPool: command_pool,
            level: level,
            commandBufferCount: 1,
        };
        let mut command_buffer: VkCommandBuffer = util::vk_null_handle();
        vk_tryobj!(ffi::vkAllocateCommandBuffers(self, &single_alloc_info, &mut command_buffer));
        return Ok(command_buffer);
    }

    // see vkAllocateCommandBuffers
    pub fn allocate_command_buffers(self,
                                    alloc_info: &VkCommandBufferAllocateInfo)
                                    -> VkResultObj<Vec<VkCommandBuffer>> {
        let mut command_buffers: Vec<VkCommandBuffer> =
            Vec::with_capacity(alloc_info.commandBufferCount as usize);
        vk_tryobj!(ffi::vkAllocateCommandBuffers(self, alloc_info, command_buffers.as_mut_ptr()));
        return Ok(command_buffers);
    }

    // see vkFreeCommandBuffers
    #[inline]
    pub fn free_command_buffers(self,
                                command_pool: VkCommandPool,
                                command_buffers: &[VkCommandBuffer]) {
        unsafe {
            ffi::vkFreeCommandBuffers(self,
                                      command_pool,
                                      command_buffers.len() as u32,
                                      command_buffers.as_ptr())
        }
    }

    /* TODO:

pub fn vkAllocateMemory(
    device:         VkDevice,
    pAllocateInfo:  *const VkMemoryAllocateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pMemory:        *mut VkDeviceMemory)
    -> VkResult;

pub fn vkFreeMemory(
    device:     VkDevice,
    memory:     VkDeviceMemory,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkMapMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size:   VkDeviceSize,
    flags:  VkMemoryMapFlags,
    ppData: *mut *mut c_void)
    -> VkResult;

pub fn vkUnmapMemory(
    device: VkDevice,
    memory: VkDeviceMemory);

pub fn vkFlushMappedMemoryRanges(
    device:             VkDevice,
    memoryRangeCount:   u32,
    pMemoryRanges:      *const VkMappedMemoryRange)
    -> VkResult;

pub fn vkInvalidateMappedMemoryRanges(
    device:             VkDevice,
    memoryRangeCount:   u32,
    pMemoryRanges:      *const VkMappedMemoryRange)
    -> VkResult;

pub fn vkGetDeviceMemoryCommitment(
    device:                     VkDevice,
    memory:                     VkDeviceMemory,
    pCommittedMemoryInBytes:    *mut VkDeviceSize);

pub fn vkBindBufferMemory(
    device:         VkDevice,
    buffer:         VkBuffer,
    memory:         VkDeviceMemory,
    memoryOffset:   VkDeviceSize)
    -> VkResult;

pub fn vkBindImageMemory(
    device:         VkDevice,
    image:          VkImage,
    memory:         VkDeviceMemory,
    memoryOffset:   VkDeviceSize)
    -> VkResult;

pub fn vkGetBufferMemoryRequirements(
    device:                 VkDevice,
    buffer:                 VkBuffer,
    pMemoryRequirements:    *mut VkMemoryRequirements);

pub fn vkGetImageMemoryRequirements(
    device:                 VkDevice,
    image:                  VkImage,
    pMemoryRequirements:    *mut VkMemoryRequirements);

pub fn vkGetImageSparseMemoryRequirements(
    device:                         VkDevice,
    image:                          VkImage,
    pSparseMemoryRequirementCount:  *mut u32,
    pSparseMemoryRequirements:      *mut VkSparseImageMemoryRequirements);

*/
}

impl VkQueue {
    // see vkQueueSubmit
    #[inline]
    pub fn submit(self, submits: &[VkSubmitInfo], fence: VkFence) -> VkResultObj<()> {
        vk_tryobj!(ffi::vkQueueSubmit(self, submits.len() as u32, submits.as_ptr(), fence));
        return Ok(());
    }

    // see vkQueueWaitIdle
    #[inline]
    pub fn wait_idle(self) -> VkResultObj<()> {
        vk_tryobj!(ffi::vkQueueWaitIdle(self));
        return Ok(());
    }

    /* TODO:
pub fn vkQueueBindSparse(
    queue:          VkQueue,
    bindInfoCount:  u32,
    pBindInfo:      *const VkBindSparseInfo,
    fence:          VkFence)
    -> VkResult;
*/
}


/*



pub fn vkCreateQueryPool(
    device:         VkDevice,
    pCreateInfo:    *const VkQueryPoolCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pQueryPool:     *mut VkQueryPool)
    -> VkResult;

pub fn vkDestroyQueryPool(
    device:     VkDevice,
    queryPool:  VkQueryPool,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkGetQueryPoolResults(
    device:     VkDevice,
    queryPool:  VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dataSize:   usize,
    pData:      *mut c_void,
    stride:     VkDeviceSize,
    flags:      VkQueryResultFlags)
    -> VkResult;

pub fn vkCreateBuffer(
    device:         VkDevice,
    pCreateInfo:    *const VkBufferCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pBuffer:        *mut VkBuffer)
    -> VkResult;

pub fn vkDestroyBuffer(
    device:     VkDevice,
    buffer:     VkBuffer,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkCreateBufferView(
    device:         VkDevice,
    pCreateInfo:    *const VkBufferViewCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pView:          *mut VkBufferView)
    -> VkResult;

pub fn vkDestroyBufferView(
    device:     VkDevice,
    bufferView: VkBufferView,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkCreateImage(
    device:         VkDevice,
    pCreateInfo:    *const VkImageCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pImage:         *mut VkImage)
    -> VkResult;

pub fn vkDestroyImage(
    device:     VkDevice,
    image:      VkImage,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkGetImageSubresourceLayout(
    device:         VkDevice,
    image:          VkImage,
    pSubresource:   *const VkImageSubresource,
    pLayout:        *mut VkSubresourceLayout);

pub fn vkCreateImageView(
    device:         VkDevice,
    pCreateInfo:    *const VkImageViewCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pView:          *mut VkImageView)
    -> VkResult;

pub fn vkDestroyImageView(
    device:     VkDevice,
    imageView:  VkImageView,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkCreateShaderModule(
    device:         VkDevice,
    pCreateInfo:    *const VkShaderModuleCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pShaderModule:  *mut VkShaderModule)
    -> VkResult;

pub fn vkDestroyShaderModule(
    device:         VkDevice,
    shaderModule:   VkShaderModule,
    pAllocator:     *const VkAllocationCallbacks);

pub fn vkCreatePipelineCache(
    device:         VkDevice,
    pCreateInfo:    *const VkPipelineCacheCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pPipelineCache: *mut VkPipelineCache)
    -> VkResult;

pub fn vkDestroyPipelineCache(
    device:         VkDevice,
    pipelineCache:  VkPipelineCache,
    pAllocator:     *const VkAllocationCallbacks);

pub fn vkGetPipelineCacheData(
    device:         VkDevice,
    pipelineCache:  VkPipelineCache,
    pDataSize:      *mut usize,
    pData:          *mut c_void)
    -> VkResult;

pub fn vkMergePipelineCaches(
    device:         VkDevice,
    dstCache:       VkPipelineCache,
    srcCacheCount:  u32,
    pSrcCaches:     *const VkPipelineCache)
    -> VkResult;

pub fn vkCreateGraphicsPipelines(
    device:             VkDevice,
    pipelineCache:      VkPipelineCache,
    createInfoCount:    u32,
    pCreateInfos:       *const VkGraphicsPipelineCreateInfo,
    pAllocator:         *const VkAllocationCallbacks,
    pPipelines:         *mut VkPipeline)
    -> VkResult;

pub fn vkCreateComputePipelines(
    device:             VkDevice,
    pipelineCache:      VkPipelineCache,
    createInfoCount:    u32,
    pCreateInfos:       *const VkComputePipelineCreateInfo,
    pAllocator:         *const VkAllocationCallbacks,
    pPipelines:         *mut VkPipeline)
    -> VkResult;

pub fn vkDestroyPipeline(
    device:     VkDevice,
    pipeline:   VkPipeline,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkCreatePipelineLayout(
    device:             VkDevice,
    pCreateInfo:        *const VkPipelineLayoutCreateInfo,
    pAllocator:         *const VkAllocationCallbacks,
    pPipelineLayout:    *mut VkPipelineLayout)
    -> VkResult;

pub fn vkDestroyPipelineLayout(
    device:         VkDevice,
    pipelineLayout: VkPipelineLayout,
    pAllocator:     *const VkAllocationCallbacks);

pub fn vkCreateSampler(
    device:         VkDevice,
    pCreateInfo:    *const VkSamplerCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pSampler:       *mut VkSampler)
    -> VkResult;

pub fn vkDestroySampler(
    device:     VkDevice,
    sampler:    VkSampler,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkCreateDescriptorSetLayout(
    device:         VkDevice,
    pCreateInfo:    *const VkDescriptorSetLayoutCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pSetLayout:     *mut VkDescriptorSetLayout)
    -> VkResult;

pub fn vkDestroyDescriptorSetLayout(
    device:                 VkDevice,
    descriptorSetLayout:    VkDescriptorSetLayout,
    pAllocator:             *const VkAllocationCallbacks);

pub fn vkCreateDescriptorPool(
    device:             VkDevice,
    pCreateInfo:        *const VkDescriptorPoolCreateInfo,
    pAllocator:         *const VkAllocationCallbacks,
    pDescriptorPool:    *mut VkDescriptorPool)
    -> VkResult;

pub fn vkDestroyDescriptorPool(
    device:         VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator:     *const VkAllocationCallbacks);

pub fn vkResetDescriptorPool(
    device:         VkDevice,
    descriptorPool: VkDescriptorPool,
    flags:          VkDescriptorPoolResetFlags)
    -> VkResult;

pub fn vkAllocateDescriptorSets(
    device:             VkDevice,
    pAllocateInfo:      *const VkDescriptorSetAllocateInfo,
    pDescriptorSets:    *mut VkDescriptorSet)
    -> VkResult;

pub fn vkFreeDescriptorSets(
    device:             VkDevice,
    descriptorPool:     VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets:    *const VkDescriptorSet)
    -> VkResult;

pub fn vkUpdateDescriptorSets(
    device:                 VkDevice,
    descriptorWriteCount:   u32,
    pDescriptorWrites:      *const VkWriteDescriptorSet,
    descriptorCopyCount:    u32,
    pDescriptorCopies:      *const VkCopyDescriptorSet);

pub fn vkCreateFramebuffer(
    device:         VkDevice,
    pCreateInfo:    *const VkFramebufferCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pFramebuffer:   *mut VkFramebuffer)
    -> VkResult;

pub fn vkDestroyFramebuffer(
    device:         VkDevice,
    framebuffer:    VkFramebuffer,
    pAllocator:     *const VkAllocationCallbacks);

pub fn vkCreateRenderPass(
    device:         VkDevice,
    pCreateInfo:    *const VkRenderPassCreateInfo,
    pAllocator:     *const VkAllocationCallbacks,
    pRenderPass:    *mut VkRenderPass)
    -> VkResult;

pub fn vkDestroyRenderPass(
    device:     VkDevice,
    renderPass: VkRenderPass,
    pAllocator: *const VkAllocationCallbacks);

pub fn vkGetRenderAreaGranularity(
    device:         VkDevice,
    renderPass:     VkRenderPass,
    pGranularity:   *mut VkExtent2D);

pub fn vkBeginCommandBuffer(
    commandBuffer:  VkCommandBuffer,
    pBeginInfo:     *const VkCommandBufferBeginInfo)
    -> VkResult;

pub fn vkEndCommandBuffer(
    commandBuffer:  VkCommandBuffer)
    -> VkResult;

pub fn vkResetCommandBuffer(
    commandBuffer:  VkCommandBuffer,
    flags:          VkCommandBufferResetFlags)
    -> VkResult;

pub fn vkCmdBindPipeline(
    commandBuffer:      VkCommandBuffer,
    pipelineBindPoint:  VkPipelineBindPoint,
    pipeline:           VkPipeline);

pub fn vkCmdSetViewport(
    commandBuffer:  VkCommandBuffer,
    firstViewport:  u32,
    viewportCount:  u32,
    pViewports:     *const VkViewport);

pub fn vkCmdSetScissor(
    commandBuffer:  VkCommandBuffer,
    firstScissor:   u32,
    scissorCount:   u32,
    pScissors:      *const VkRect2D);

pub fn vkCmdSetLineWidth(
    commandBuffer:  VkCommandBuffer,
    lineWidth:      f32);

pub fn vkCmdSetDepthBias(
    commandBuffer:              VkCommandBuffer,
    depthBiasConstantFactor:    f32,
    depthBiasClamp:             f32,
    depthBiasSlopeFactor:       f32);

pub fn vkCmdSetBlendConstants(
    commandBuffer:  VkCommandBuffer,
    blendConstants: *const[f32;4]);

pub fn vkCmdSetDepthBounds(
    commandBuffer:  VkCommandBuffer,
    minDepthBounds: f32,
    maxDepthBounds: f32);

pub fn vkCmdSetStencilCompareMask(
    commandBuffer:  VkCommandBuffer,
    faceMask:       VkStencilFaceFlags,
    compareMask:    u32);

pub fn vkCmdSetStencilWriteMask(
    commandBuffer:  VkCommandBuffer,
    faceMask:       VkStencilFaceFlags,
    writeMask:      u32);

pub fn vkCmdSetStencilReference(
    commandBuffer:  VkCommandBuffer,
    faceMask:       VkStencilFaceFlags,
    reference:      u32);

pub fn vkCmdBindDescriptorSets(
    commandBuffer:      VkCommandBuffer,
    pipelineBindPoint:  VkPipelineBindPoint,
    layout:             VkPipelineLayout,
    firstSet:           u32,
    descriptorSetCount: u32,
    pDescriptorSets:    *const VkDescriptorSet,
    dynamicOffsetCount: u32,
    pDynamicOffsets:    *const u32);

pub fn vkCmdBindIndexBuffer(
    commandBuffer:  VkCommandBuffer,
    buffer:         VkBuffer,
    offset:         VkDeviceSize,
    indexType:      VkIndexType);

pub fn vkCmdBindVertexBuffers(
    commandBuffer:  VkCommandBuffer,
    firstBinding:   u32,
    bindingCount:   u32,
    pBuffers:       *const VkBuffer,
    pOffsets:       *const VkDeviceSize);

pub fn vkCmdDraw(
    commandBuffer:  VkCommandBuffer,
    vertexCount:    u32,
    instanceCount:  u32,
    firstVertex:    u32,
    firstInstance:  u32);

pub fn vkCmdDrawIndexed(
    commandBuffer:  VkCommandBuffer,
    indexCount:     u32,
    instanceCount:  u32,
    firstIndex:     u32,
    vertexOffset:   i32,
    firstInstance:  u32);

pub fn vkCmdDrawIndirect(
    commandBuffer:  VkCommandBuffer,
    buffer:         VkBuffer,
    offset:         VkDeviceSize,
    drawCount:      u32,
    stride:         u32);

pub fn vkCmdDrawIndexedIndirect(
    commandBuffer:  VkCommandBuffer,
    buffer:         VkBuffer,
    offset:         VkDeviceSize,
    drawCount:      u32,
    stride:         u32);

pub fn vkCmdDispatch(
    commandBuffer:  VkCommandBuffer,
    x:              u32,
    y:              u32,
    z:              u32);

pub fn vkCmdDispatchIndirect(
    commandBuffer:  VkCommandBuffer,
    buffer:         VkBuffer,
    offset:         VkDeviceSize);

pub fn vkCmdCopyBuffer(
    commandBuffer:  VkCommandBuffer,
    srcBuffer:      VkBuffer,
    dstBuffer:      VkBuffer,
    regionCount:    u32,
    pRegions:       *const VkBufferCopy);

pub fn vkCmdCopyImage(
    commandBuffer:  VkCommandBuffer,
    srcImage:       VkImage,
    srcImageLayout: VkImageLayout,
    dstImage:       VkImage,
    dstImageLayout: VkImageLayout,
    regionCount:    u32,
    pRegions:       *const VkImageCopy);

pub fn vkCmdBlitImage(
    commandBuffer:  VkCommandBuffer,
    srcImage:       VkImage,
    srcImageLayout: VkImageLayout,
    dstImage:       VkImage,
    dstImageLayout: VkImageLayout,
    regionCount:    u32,
    pRegions:       *const VkImageBlit,
    filter:         VkFilter);

pub fn vkCmdCopyBufferToImage(
    commandBuffer:  VkCommandBuffer,
    srcBuffer:      VkBuffer,
    dstImage:       VkImage,
    dstImageLayout: VkImageLayout,
    regionCount:    u32,
    pRegions:       *const VkBufferImageCopy);

pub fn vkCmdCopyImageToBuffer(
    commandBuffer:  VkCommandBuffer,
    srcImage:       VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer:      VkBuffer,
    regionCount:    u32,
    pRegions:       *const VkBufferImageCopy);

pub fn vkCmdUpdateBuffer(
    commandBuffer:  VkCommandBuffer,
    dstBuffer:      VkBuffer,
    dstOffset:      VkDeviceSize,
    dataSize:       VkDeviceSize,
    pData:          *const c_void);

pub fn vkCmdFillBuffer(
    commandBuffer:  VkCommandBuffer,
    dstBuffer:      VkBuffer,
    dstOffset:      VkDeviceSize,
    size:           VkDeviceSize,
    data:           u32);

pub fn vkCmdClearColorImage(
    commandBuffer:  VkCommandBuffer,
    image:          VkImage,
    imageLayout:    VkImageLayout,
    pColor:         *const VkClearColorValue,
    rangeCount:     u32,
    pRanges:        *const VkImageSubresourceRange);

pub fn vkCmdClearDepthStencilImage(
    commandBuffer:  VkCommandBuffer,
    image:          VkImage,
    imageLayout:    VkImageLayout,
    pDepthStencil:  *const VkClearDepthStencilValue,
    rangeCount:     u32,
    pRanges:        *const VkImageSubresourceRange);

pub fn vkCmdClearAttachments(
    commandBuffer:      VkCommandBuffer,
    attachmentCount:    u32,
    pAttachments:       *const VkClearAttachment,
    rectCount:          u32,
    pRects:             *const VkClearRect);

pub fn vkCmdResolveImage(
    commandBuffer:  VkCommandBuffer,
    srcImage:       VkImage,
    srcImageLayout: VkImageLayout,
    dstImage:       VkImage,
    dstImageLayout: VkImageLayout,
    regionCount:    u32,
    pRegions:       *const VkImageResolve);

pub fn vkCmdSetEvent(
    commandBuffer:  VkCommandBuffer,
    event:          VkEvent,
    stageMask:      VkPipelineStageFlags);

pub fn vkCmdResetEvent(
    commandBuffer:  VkCommandBuffer,
    event:          VkEvent,
    stageMask:      VkPipelineStageFlags);

pub fn vkCmdWaitEvents(
    commandBuffer:              VkCommandBuffer,
    eventCount:                 u32,
    pEvents:                    *const VkEvent,
    srcStageMask:               VkPipelineStageFlags,
    dstStageMask:               VkPipelineStageFlags,
    memoryBarrierCount:         u32,
    pMemoryBarriers:            *const VkMemoryBarrier,
    bufferMemoryBarrierCount:   u32,
    pBufferMemoryBarriers:      *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount:    u32,
    pImageMemoryBarriers:       *const VkImageMemoryBarrier);

pub fn vkCmdPipelineBarrier(
    commandBuffer:              VkCommandBuffer,
    srcStageMask:               VkPipelineStageFlags,
    dstStageMask:               VkPipelineStageFlags,
    dependencyFlags:            VkDependencyFlags,
    memoryBarrierCount:         u32,
    pMemoryBarriers:            *const VkMemoryBarrier,
    bufferMemoryBarrierCount:   u32,
    pBufferMemoryBarriers:      *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount:    u32,
    pImageMemoryBarriers:       *const VkImageMemoryBarrier);

pub fn vkCmdBeginQuery(
    commandBuffer:  VkCommandBuffer,
    queryPool:      VkQueryPool,
    query:          u32,
    flags:          VkQueryControlFlags);

pub fn vkCmdEndQuery(
    commandBuffer:  VkCommandBuffer,
    queryPool:      VkQueryPool,
    query:          u32);

pub fn vkCmdResetQueryPool(
    commandBuffer:  VkCommandBuffer,
    queryPool:      VkQueryPool,
    firstQuery:     u32,
    queryCount:     u32);

pub fn vkCmdWriteTimestamp(
    commandBuffer:  VkCommandBuffer,
    pipelineStage:  VkPipelineStageFlagBits,
    queryPool:      VkQueryPool,
    query:          u32);

pub fn vkCmdCopyQueryPoolResults(
    commandBuffer:  VkCommandBuffer,
    queryPool:      VkQueryPool,
    firstQuery:     u32,
    queryCount:     u32,
    dstBuffer:      VkBuffer,
    dstOffset:      VkDeviceSize,
    stride:         VkDeviceSize,
    flags:          VkQueryResultFlags);

pub fn vkCmdPushConstants(
    commandBuffer:  VkCommandBuffer,
    layout:         VkPipelineLayout,
    stageFlags:     VkShaderStageFlags,
    offset:         u32,
    size:           u32,
    pValues:        *const c_void);

pub fn vkCmdBeginRenderPass(
    commandBuffer:      VkCommandBuffer,
    pRenderPassBegin:   *const VkRenderPassBeginInfo,
    contents:           VkSubpassContents);

pub fn vkCmdNextSubpass(
    commandBuffer:  VkCommandBuffer,
    contents:       VkSubpassContents);

pub fn vkCmdEndRenderPass(
    commandBuffer:  VkCommandBuffer);

pub fn vkCmdExecuteCommands(
    commandBuffer:      VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers:    *const VkCommandBuffer);

}

*/
