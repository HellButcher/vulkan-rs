/*
**  Copyright (c) 2016, Christoph Hommelsheim
**  All rights reserved.
**
**  Redistribution and use in source and binary forms, with or without
**  modification, are permitted provided that the following conditions are met:
**
**  * Redistributions of source code must retain the above copyright notice, this
**    list of conditions and the following disclaimer.
**
**  * Redistributions in binary form must reproduce the above copyright notice,
**    this list of conditions and the following disclaimer in the documentation
**    and/or other materials provided with the distribution.
**
**  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
**  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
**  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
**  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
**  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
**  DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
**  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
**  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
**  OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
**  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
**
*/
mod table {
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/cmds_table.rs"));
}

mod dispatch {
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/cmds_dispatch.rs"));
}

mod safe {
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/cmds_safe.rs"));
}

pub mod ffi;

// use types::*;
// use util::{VkResultObj,VkOwned};

pub use self::safe::*;

// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateInstance<'l> (create_info: &VkInstanceCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkInstance>> {
//     let instance = safe::vkCreateInstance(create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(instance, (), allocator))
//     }
// }

// impl VkDestroyableHandle for VkInstance {
//     type Owner = ();
//     #[inline]
//     fn destroy (self, _: Self::Owner, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroyInstance(self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateDevice<'l> (physical_device: VkPhysicalDevice, create_info: &VkDeviceCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkDevice>> {
//     let device = safe::vkCreateDevice(physical_device, create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(device, (), allocator))
//     }
// }

// impl VkDestroyableHandle for VkDevice {
//     type Owner = ();
//     #[inline]
//     fn destroy (self, _: Self::Owner, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroyDevice(self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkAllocateMemory<'l> (device: &'l VkDevice, allocate_info: &VkMemoryAllocateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkDeviceMemory>> {
//     let memory = safe::vkAllocateMemory(*device, allocate_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(memory, *device, allocator))
//     }
// }

// impl VkDestroyableHandle for VkDeviceMemory {
//     type Owner = VkDevice;
//     #[inline]
//     fn destroy (self, device: VkDevice, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkFreeMemory(device, self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateFence<'l> (device: &'l VkDevice, create_info: &VkFenceCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkFence>> {
//     let fence = safe::vkCreateFence(*device, create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(fence, *device, allocator))
//     }
// }

// impl VkDestroyableHandle for VkFence {
//     type Owner = VkDevice;
//     #[inline]
//     fn destroy (self, device: VkDevice, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroyFence(device, self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateSemaphore<'l> (device: &'l VkDevice, create_info: &VkSemaphoreCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkSemaphore>> {
//     let semaphore = safe::vkCreateSemaphore(*device, create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(semaphore, *device, allocator))
//     }
// }

// impl VkDestroyableHandle for VkSemaphore {
//     type Owner = VkDevice;
//     #[inline]
//     fn destroy (self, device: VkDevice, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroySemaphore(device, self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateEvent<'l> (device: &'l VkDevice, create_info: &VkEventCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkEvent>> {
//     let event = safe::vkCreateEvent(*device, create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(event, *device, allocator))
//     }
// }

// impl VkDestroyableHandle for VkEvent {
//     type Owner = VkDevice;
//     #[inline]
//     fn destroy (self, device: VkDevice, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroyEvent(device, self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateQueryPool<'l> (device: &'l VkDevice, create_info: &VkQueryPoolCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkQueryPool>> {
//     let query_pool = safe::vkCreateQueryPool(*device, create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(query_pool, *device, allocator))
//     }
// }

// impl VkDestroyableHandle for VkQueryPool {
//     type Owner = VkDevice;
//     #[inline]
//     fn destroy (self, device: VkDevice, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroyQueryPool(device, self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateBuffer<'l> (device: &'l VkDevice, create_info: &VkBufferCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkBuffer>> {
//     let query_pool = safe::vkCreateBuffer(*device, create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(query_pool, *device, allocator))
//     }
// }

// impl VkDestroyableHandle for VkBuffer {
//     type Owner = VkDevice;
//     #[inline]
//     fn destroy (self, device: VkDevice, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroyBuffer(device, self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateBufferView<'l> (device: &'l VkDevice, create_info: &VkBufferViewCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkBufferView>> {
//     let query_pool = safe::vkCreateBufferView(*device, create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(query_pool, *device, allocator))
//     }
// }

// impl VkDestroyableHandle for VkBufferView {
//     type Owner = VkDevice;
//     #[inline]
//     fn destroy (self, device: VkDevice, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroyBufferView(device, self, allocator);
//     }
// }


// #[inline]
// #[allow(non_snake_case)]
// pub fn vkCreateImage<'l> (device: &'l VkDevice, create_info: &VkImageCreateInfo, allocator: Option<&'l VkAllocationCallbacks>) -> VkResultObj<VkOwned<'l,VkImage>> {
//     let query_pool = safe::vkCreateImage(*device, create_info, allocator)?;
//     unsafe {
//         Ok(VkOwned::from_raw(query_pool, *device, allocator))
//     }
// }

// impl VkDestroyableHandle for VkImage {
//     type Owner = VkDevice;
//     #[inline]
//     fn destroy (self, device: VkDevice, allocator: Option<&VkAllocationCallbacks>) {
//         safe::vkDestroyImage(device, self, allocator);
//     }
// }
