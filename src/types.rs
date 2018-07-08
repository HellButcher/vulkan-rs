/* GENERATED FILE */

#![allow(non_snake_case)]

#[path = "types_impl.rs"]
pub mod types_impl;

use enums;
use platform::*;
use std::cell::Cell;
use utils::VkDispatchableHandle;
use utils::VkNonDispatchableHandle;
use AsRaw;
use Struct;
use StructExtends;

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
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkBuffer__ {}

/// Opaque handle to a buffer object
pub type VkBuffer<'l> = VkNonDispatchableHandle<'l, VkBuffer__>;

/// Vulkan device memory size and offsets
pub type VkDeviceSize = u64;

/// Structure specifying a buffer memory barrier
#[repr(C)]
pub struct VkBufferMemoryBarrier<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  buffer: u64,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkBufferMemoryBarrier<'l, 'h> {
  #[inline]
  pub fn new() -> VkBufferMemoryBarrier<'l, 'h> {
    unsafe {
      VkBufferMemoryBarrier {
        sType: VkStructureType::BUFFER_MEMORY_BARRIER,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_access_mask(mut self, value: VkAccessFlags) -> Self {
    self.srcAccessMask = value;
    self
  }
  #[inline]
  pub fn set_dst_access_mask(mut self, value: VkAccessFlags) -> Self {
    self.dstAccessMask = value;
    self
  }
  #[inline]
  pub fn set_src_queue_family_index(mut self, value: u32) -> Self {
    self.srcQueueFamilyIndex = value;
    self
  }
  #[inline]
  pub fn set_dst_queue_family_index(mut self, value: u32) -> Self {
    self.dstQueueFamilyIndex = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: VkDeviceSize) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_size(mut self, value: VkDeviceSize) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn src_access_mask(&self) -> VkAccessFlags {
    self.srcAccessMask
  }
  #[inline]
  pub fn dst_access_mask(&self) -> VkAccessFlags {
    self.dstAccessMask
  }
  #[inline]
  pub fn src_queue_family_index(&self) -> u32 {
    self.srcQueueFamilyIndex
  }
  #[inline]
  pub fn dst_queue_family_index(&self) -> u32 {
    self.dstQueueFamilyIndex
  }
  #[inline]
  pub fn offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkBufferMemoryBarrier<'l, 'h> {
  fn default() -> VkBufferMemoryBarrier<'l, 'h> {
    VkBufferMemoryBarrier::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkBufferMemoryBarrier<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_memory_barrier() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(40 + ptr_size * 2, VkBufferMemoryBarrier);
}

/// Structure specifying a dispatch indirect command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDispatchIndirectCommand {
  pub x: u32,
  pub y: u32,
  pub z: u32,
}
impl VkDispatchIndirectCommand {
  #[inline]
  pub fn new() -> VkDispatchIndirectCommand {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_x(mut self, value: u32) -> Self {
    self.x = value;
    self
  }
  #[inline]
  pub fn set_y(mut self, value: u32) -> Self {
    self.y = value;
    self
  }
  #[inline]
  pub fn set_z(mut self, value: u32) -> Self {
    self.z = value;
    self
  }
  #[inline]
  pub fn x(&self) -> u32 {
    self.x
  }
  #[inline]
  pub fn y(&self) -> u32 {
    self.y
  }
  #[inline]
  pub fn z(&self) -> u32 {
    self.z
  }
}
impl Default for VkDispatchIndirectCommand {
  fn default() -> VkDispatchIndirectCommand {
    VkDispatchIndirectCommand::new()
  }
}
unsafe impl Struct for VkDispatchIndirectCommand {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_dispatch_indirect_command() {
  assert_size!(12, VkDispatchIndirectCommand);
}

/// Structure specifying a draw indexed indirect command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndexedIndirectCommand {
  pub indexCount: u32,
  pub instanceCount: u32,
  pub firstIndex: u32,
  pub vertexOffset: i32,
  pub firstInstance: u32,
}
impl VkDrawIndexedIndirectCommand {
  #[inline]
  pub fn new() -> VkDrawIndexedIndirectCommand {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_index_count(mut self, value: u32) -> Self {
    self.indexCount = value;
    self
  }
  #[inline]
  pub fn set_instance_count(mut self, value: u32) -> Self {
    self.instanceCount = value;
    self
  }
  #[inline]
  pub fn set_first_index(mut self, value: u32) -> Self {
    self.firstIndex = value;
    self
  }
  #[inline]
  pub fn set_vertex_offset(mut self, value: i32) -> Self {
    self.vertexOffset = value;
    self
  }
  #[inline]
  pub fn set_first_instance(mut self, value: u32) -> Self {
    self.firstInstance = value;
    self
  }
  #[inline]
  pub fn index_count(&self) -> u32 {
    self.indexCount
  }
  #[inline]
  pub fn instance_count(&self) -> u32 {
    self.instanceCount
  }
  #[inline]
  pub fn first_index(&self) -> u32 {
    self.firstIndex
  }
  #[inline]
  pub fn vertex_offset(&self) -> i32 {
    self.vertexOffset
  }
  #[inline]
  pub fn first_instance(&self) -> u32 {
    self.firstInstance
  }
}
impl Default for VkDrawIndexedIndirectCommand {
  fn default() -> VkDrawIndexedIndirectCommand {
    VkDrawIndexedIndirectCommand::new()
  }
}
unsafe impl Struct for VkDrawIndexedIndirectCommand {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_draw_indexed_indirect_command() {
  assert_size!(20, VkDrawIndexedIndirectCommand);
}

/// Structure specifying a draw indirect command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndirectCommand {
  pub vertexCount: u32,
  pub instanceCount: u32,
  pub firstVertex: u32,
  pub firstInstance: u32,
}
impl VkDrawIndirectCommand {
  #[inline]
  pub fn new() -> VkDrawIndirectCommand {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_vertex_count(mut self, value: u32) -> Self {
    self.vertexCount = value;
    self
  }
  #[inline]
  pub fn set_instance_count(mut self, value: u32) -> Self {
    self.instanceCount = value;
    self
  }
  #[inline]
  pub fn set_first_vertex(mut self, value: u32) -> Self {
    self.firstVertex = value;
    self
  }
  #[inline]
  pub fn set_first_instance(mut self, value: u32) -> Self {
    self.firstInstance = value;
    self
  }
  #[inline]
  pub fn vertex_count(&self) -> u32 {
    self.vertexCount
  }
  #[inline]
  pub fn instance_count(&self) -> u32 {
    self.instanceCount
  }
  #[inline]
  pub fn first_vertex(&self) -> u32 {
    self.firstVertex
  }
  #[inline]
  pub fn first_instance(&self) -> u32 {
    self.firstInstance
  }
}
impl Default for VkDrawIndirectCommand {
  fn default() -> VkDrawIndirectCommand {
    VkDrawIndirectCommand::new()
  }
}
unsafe impl Struct for VkDrawIndirectCommand {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_draw_indirect_command() {
  assert_size!(16, VkDrawIndirectCommand);
}

/// Layout of image and image subresources
pub use enums::VkImageLayout;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkImage__ {}

/// Opaque handle to a image object
pub type VkImage<'l> = VkNonDispatchableHandle<'l, VkImage__>;

/// Bitmask specifying which aspects of an image are included in a view
pub use enums::VkImageAspectFlagBits;

/// Bitmask of VkImageAspectFlagBits
pub type VkImageAspectFlags = VkImageAspectFlagBits;

/// Structure specifying a image subresource range
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceRange {
  pub aspectMask: VkImageAspectFlags,
  pub baseMipLevel: u32,
  pub levelCount: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
impl VkImageSubresourceRange {
  #[inline]
  pub fn new() -> VkImageSubresourceRange {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_aspect_mask(mut self, value: VkImageAspectFlags) -> Self {
    self.aspectMask = value;
    self
  }
  #[inline]
  pub fn set_base_mip_level(mut self, value: u32) -> Self {
    self.baseMipLevel = value;
    self
  }
  #[inline]
  pub fn set_level_count(mut self, value: u32) -> Self {
    self.levelCount = value;
    self
  }
  #[inline]
  pub fn set_base_array_layer(mut self, value: u32) -> Self {
    self.baseArrayLayer = value;
    self
  }
  #[inline]
  pub fn set_layer_count(mut self, value: u32) -> Self {
    self.layerCount = value;
    self
  }
  #[inline]
  pub fn aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn base_mip_level(&self) -> u32 {
    self.baseMipLevel
  }
  #[inline]
  pub fn level_count(&self) -> u32 {
    self.levelCount
  }
  #[inline]
  pub fn base_array_layer(&self) -> u32 {
    self.baseArrayLayer
  }
  #[inline]
  pub fn layer_count(&self) -> u32 {
    self.layerCount
  }
}
impl Default for VkImageSubresourceRange {
  fn default() -> VkImageSubresourceRange {
    VkImageSubresourceRange::new()
  }
}
unsafe impl Struct for VkImageSubresourceRange {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_subresource_range() {
  assert_size!(20, VkImageSubresourceRange);
}

/// Structure specifying the parameters of an image memory barrier
#[repr(C)]
pub struct VkImageMemoryBarrier<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub oldLayout: VkImageLayout,
  pub newLayout: VkImageLayout,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  image: u64,
  pub subresourceRange: VkImageSubresourceRange,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkImageMemoryBarrier<'l, 'h> {
  #[inline]
  pub fn new() -> VkImageMemoryBarrier<'l, 'h> {
    unsafe {
      VkImageMemoryBarrier {
        sType: VkStructureType::IMAGE_MEMORY_BARRIER,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_access_mask(mut self, value: VkAccessFlags) -> Self {
    self.srcAccessMask = value;
    self
  }
  #[inline]
  pub fn set_dst_access_mask(mut self, value: VkAccessFlags) -> Self {
    self.dstAccessMask = value;
    self
  }
  #[inline]
  pub fn set_old_layout(mut self, value: VkImageLayout) -> Self {
    self.oldLayout = value;
    self
  }
  #[inline]
  pub fn set_new_layout(mut self, value: VkImageLayout) -> Self {
    self.newLayout = value;
    self
  }
  #[inline]
  pub fn set_src_queue_family_index(mut self, value: u32) -> Self {
    self.srcQueueFamilyIndex = value;
    self
  }
  #[inline]
  pub fn set_dst_queue_family_index(mut self, value: u32) -> Self {
    self.dstQueueFamilyIndex = value;
    self
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage<'h>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_subresource_range(mut self, value: VkImageSubresourceRange) -> Self {
    self.subresourceRange = value;
    self
  }
  #[inline]
  pub fn src_access_mask(&self) -> VkAccessFlags {
    self.srcAccessMask
  }
  #[inline]
  pub fn dst_access_mask(&self) -> VkAccessFlags {
    self.dstAccessMask
  }
  #[inline]
  pub fn old_layout(&self) -> VkImageLayout {
    self.oldLayout
  }
  #[inline]
  pub fn new_layout(&self) -> VkImageLayout {
    self.newLayout
  }
  #[inline]
  pub fn src_queue_family_index(&self) -> u32 {
    self.srcQueueFamilyIndex
  }
  #[inline]
  pub fn dst_queue_family_index(&self) -> u32 {
    self.dstQueueFamilyIndex
  }
  #[inline]
  pub fn subresource_range(&self) -> &VkImageSubresourceRange {
    &self.subresourceRange
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkImageMemoryBarrier<'l, 'h> {
  fn default() -> VkImageMemoryBarrier<'l, 'h> {
    VkImageMemoryBarrier::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkImageMemoryBarrier<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_memory_barrier() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 3, VkImageMemoryBarrier);
}

/// Structure specifying a global memory barrier
#[repr(C)]
pub struct VkMemoryBarrier<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkMemoryBarrier<'l> {
  #[inline]
  pub fn new() -> VkMemoryBarrier<'l> {
    unsafe {
      VkMemoryBarrier {
        sType: VkStructureType::MEMORY_BARRIER,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_access_mask(mut self, value: VkAccessFlags) -> Self {
    self.srcAccessMask = value;
    self
  }
  #[inline]
  pub fn set_dst_access_mask(mut self, value: VkAccessFlags) -> Self {
    self.dstAccessMask = value;
    self
  }
  #[inline]
  pub fn src_access_mask(&self) -> VkAccessFlags {
    self.srcAccessMask
  }
  #[inline]
  pub fn dst_access_mask(&self) -> VkAccessFlags {
    self.dstAccessMask
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkMemoryBarrier<'l> {
  fn default() -> VkMemoryBarrier<'l> {
    VkMemoryBarrier::new()
  }
}
unsafe impl<'l> Struct for VkMemoryBarrier<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_barrier() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkMemoryBarrier);
}

/// Specify an enumeration to track object handle types
pub use enums::VkObjectType;

/// Vulkan command return codes
pub use enums::VkResult;

/// Reserved for future use
pub type VkInstanceCreateFlags = VkFlags;

/// Structure specifying application info
#[repr(C)]
pub struct VkApplicationInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pApplicationName: *const c_char,
  pub applicationVersion: u32,
  pEngineName: *const c_char,
  pub engineVersion: u32,
  pub apiVersion: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkApplicationInfo<'l> {
  #[inline]
  pub fn new() -> VkApplicationInfo<'l> {
    unsafe {
      VkApplicationInfo {
        sType: VkStructureType::APPLICATION_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_application_name(mut self, value: Option<&'l AsRef<CStr>>) -> Self {
    unsafe {
      self.pApplicationName = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_application_version(mut self, value: u32) -> Self {
    self.applicationVersion = value;
    self
  }
  #[inline]
  pub fn set_engine_name(mut self, value: Option<&'l AsRef<CStr>>) -> Self {
    unsafe {
      self.pEngineName = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_engine_version(mut self, value: u32) -> Self {
    self.engineVersion = value;
    self
  }
  #[inline]
  pub fn set_api_version(mut self, value: u32) -> Self {
    self.apiVersion = value;
    self
  }
  #[inline]
  pub fn application_name(&self) -> &CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pApplicationName) }
  }
  #[inline]
  pub fn application_version(&self) -> u32 {
    self.applicationVersion
  }
  #[inline]
  pub fn engine_name(&self) -> &CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pEngineName) }
  }
  #[inline]
  pub fn engine_version(&self) -> u32 {
    self.engineVersion
  }
  #[inline]
  pub fn api_version(&self) -> u32 {
    self.apiVersion
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkApplicationInfo<'l> {
  fn default() -> VkApplicationInfo<'l> {
    VkApplicationInfo::new()
  }
}
unsafe impl<'l> Struct for VkApplicationInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_application_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 5, VkApplicationInfo);
}

/// Structure specifying parameters of a newly created instance
#[repr(C)]
pub struct VkInstanceCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkInstanceCreateFlags,
  pApplicationInfo: *const VkApplicationInfo<'l>,
  enabledLayerCount: u32,
  ppEnabledLayerNames: *const *const c_char,
  enabledExtensionCount: u32,
  ppEnabledExtensionNames: *const *const c_char,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkInstanceCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkInstanceCreateInfo<'l> {
    unsafe {
      VkInstanceCreateInfo {
        sType: VkStructureType::INSTANCE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkInstanceCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_application_info(mut self, value: Option<&'l VkApplicationInfo<'l>>) -> Self {
    unsafe {
      self.pApplicationInfo = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkInstanceCreateFlags {
    self.flags
  }
  #[inline]
  pub fn enabled_layer_count(&self) -> u32 {
    self.enabledLayerCount
  }
  #[inline]
  pub fn enabled_extension_count(&self) -> u32 {
    self.enabledExtensionCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkInstanceCreateInfo<'l> {
  fn default() -> VkInstanceCreateInfo<'l> {
    VkInstanceCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkInstanceCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_instance_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 8, VkInstanceCreateInfo);
}

/// Allocation scope
pub use enums::VkSystemAllocationScope;

/// Application-defined memory allocation function
#[allow(non_camel_case_types)]
pub type PFN_vkAllocationFunction =
  extern "system" fn(*mut c_void, usize, usize, VkSystemAllocationScope) -> *mut c_void;

/// Application-defined memory reallocation function
#[allow(non_camel_case_types)]
pub type PFN_vkReallocationFunction =
  extern "system" fn(*mut c_void, *mut c_void, usize, usize, VkSystemAllocationScope) -> *mut c_void;

/// Application-defined memory free function
#[allow(non_camel_case_types)]
pub type PFN_vkFreeFunction = extern "system" fn(*mut c_void, *mut c_void);

/// Allocation type
pub use enums::VkInternalAllocationType;

/// Application-defined memory allocation notification function
#[allow(non_camel_case_types)]
pub type PFN_vkInternalAllocationNotification =
  extern "system" fn(*mut c_void, usize, VkInternalAllocationType, VkSystemAllocationScope);

/// Application-defined memory free notification function
#[allow(non_camel_case_types)]
pub type PFN_vkInternalFreeNotification =
  extern "system" fn(*mut c_void, usize, VkInternalAllocationType, VkSystemAllocationScope);

/// Structure containing callback function pointers for memory allocation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAllocationCallbacks {
  pUserData: *mut c_void,
  pub pfnAllocation: PFN_vkAllocationFunction,
  pub pfnReallocation: PFN_vkReallocationFunction,
  pub pfnFree: PFN_vkFreeFunction,
  pub pfnInternalAllocation: Option<PFN_vkInternalAllocationNotification>,
  pub pfnInternalFree: Option<PFN_vkInternalFreeNotification>,
}
impl VkAllocationCallbacks {
  #[inline]
  pub fn new() -> VkAllocationCallbacks {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_user_data(mut self, value: *mut c_void) -> Self {
    self.pUserData = value;
    self
  }
  #[inline]
  pub fn set_pfn_allocation(mut self, value: PFN_vkAllocationFunction) -> Self {
    self.pfnAllocation = value;
    self
  }
  #[inline]
  pub fn set_pfn_reallocation(mut self, value: PFN_vkReallocationFunction) -> Self {
    self.pfnReallocation = value;
    self
  }
  #[inline]
  pub fn set_pfn_free(mut self, value: PFN_vkFreeFunction) -> Self {
    self.pfnFree = value;
    self
  }
  #[inline]
  pub fn set_pfn_internal_allocation(mut self, value: Option<PFN_vkInternalAllocationNotification>) -> Self {
    self.pfnInternalAllocation = value;
    self
  }
  #[inline]
  pub fn set_pfn_internal_free(mut self, value: Option<PFN_vkInternalFreeNotification>) -> Self {
    self.pfnInternalFree = value;
    self
  }
  #[inline]
  pub fn user_data(&self) -> *mut c_void {
    self.pUserData
  }
  #[inline]
  pub fn pfn_allocation(&self) -> PFN_vkAllocationFunction {
    self.pfnAllocation
  }
  #[inline]
  pub fn pfn_reallocation(&self) -> PFN_vkReallocationFunction {
    self.pfnReallocation
  }
  #[inline]
  pub fn pfn_free(&self) -> PFN_vkFreeFunction {
    self.pfnFree
  }
  #[inline]
  pub fn pfn_internal_allocation(&self) -> Option<PFN_vkInternalAllocationNotification> {
    self.pfnInternalAllocation
  }
  #[inline]
  pub fn pfn_internal_free(&self) -> Option<PFN_vkInternalFreeNotification> {
    self.pfnInternalFree
  }
}
impl Default for VkAllocationCallbacks {
  fn default() -> VkAllocationCallbacks {
    VkAllocationCallbacks::new()
  }
}
unsafe impl Struct for VkAllocationCallbacks {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_allocation_callbacks() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 6, VkAllocationCallbacks);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkInstance__ {}

/// Opaque handle to a instance object
pub type VkInstance<'l> = VkDispatchableHandle<'l, VkInstance__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPhysicalDevice__ {}

/// Opaque handle to a physical device object
pub type VkPhysicalDevice<'l> = VkDispatchableHandle<'l, VkPhysicalDevice__>;

/// Vulkan boolean type
pub type VkBool32 = u32;

/// Structure describing the fine-grained features that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFeatures {
  robustBufferAccess: VkBool32,
  fullDrawIndexUint32: VkBool32,
  imageCubeArray: VkBool32,
  independentBlend: VkBool32,
  geometryShader: VkBool32,
  tessellationShader: VkBool32,
  sampleRateShading: VkBool32,
  dualSrcBlend: VkBool32,
  logicOp: VkBool32,
  multiDrawIndirect: VkBool32,
  drawIndirectFirstInstance: VkBool32,
  depthClamp: VkBool32,
  depthBiasClamp: VkBool32,
  fillModeNonSolid: VkBool32,
  depthBounds: VkBool32,
  wideLines: VkBool32,
  largePoints: VkBool32,
  alphaToOne: VkBool32,
  multiViewport: VkBool32,
  samplerAnisotropy: VkBool32,
  textureCompressionETC2: VkBool32,
  textureCompressionASTC_LDR: VkBool32,
  textureCompressionBC: VkBool32,
  occlusionQueryPrecise: VkBool32,
  pipelineStatisticsQuery: VkBool32,
  vertexPipelineStoresAndAtomics: VkBool32,
  fragmentStoresAndAtomics: VkBool32,
  shaderTessellationAndGeometryPointSize: VkBool32,
  shaderImageGatherExtended: VkBool32,
  shaderStorageImageExtendedFormats: VkBool32,
  shaderStorageImageMultisample: VkBool32,
  shaderStorageImageReadWithoutFormat: VkBool32,
  shaderStorageImageWriteWithoutFormat: VkBool32,
  shaderUniformBufferArrayDynamicIndexing: VkBool32,
  shaderSampledImageArrayDynamicIndexing: VkBool32,
  shaderStorageBufferArrayDynamicIndexing: VkBool32,
  shaderStorageImageArrayDynamicIndexing: VkBool32,
  shaderClipDistance: VkBool32,
  shaderCullDistance: VkBool32,
  shaderFloat64: VkBool32,
  shaderInt64: VkBool32,
  shaderInt16: VkBool32,
  shaderResourceResidency: VkBool32,
  shaderResourceMinLod: VkBool32,
  sparseBinding: VkBool32,
  sparseResidencyBuffer: VkBool32,
  sparseResidencyImage2D: VkBool32,
  sparseResidencyImage3D: VkBool32,
  sparseResidency2Samples: VkBool32,
  sparseResidency4Samples: VkBool32,
  sparseResidency8Samples: VkBool32,
  sparseResidency16Samples: VkBool32,
  sparseResidencyAliased: VkBool32,
  variableMultisampleRate: VkBool32,
  inheritedQueries: VkBool32,
}
impl VkPhysicalDeviceFeatures {
  #[inline]
  pub fn new() -> VkPhysicalDeviceFeatures {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_robust_buffer_access(mut self, value: bool) -> Self {
    unsafe {
      self.robustBufferAccess = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_full_draw_index_uint32(mut self, value: bool) -> Self {
    unsafe {
      self.fullDrawIndexUint32 = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_cube_array(mut self, value: bool) -> Self {
    unsafe {
      self.imageCubeArray = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_independent_blend(mut self, value: bool) -> Self {
    unsafe {
      self.independentBlend = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_geometry_shader(mut self, value: bool) -> Self {
    unsafe {
      self.geometryShader = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_tessellation_shader(mut self, value: bool) -> Self {
    unsafe {
      self.tessellationShader = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sample_rate_shading(mut self, value: bool) -> Self {
    unsafe {
      self.sampleRateShading = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_dual_src_blend(mut self, value: bool) -> Self {
    unsafe {
      self.dualSrcBlend = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_logic_op(mut self, value: bool) -> Self {
    unsafe {
      self.logicOp = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_multi_draw_indirect(mut self, value: bool) -> Self {
    unsafe {
      self.multiDrawIndirect = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_draw_indirect_first_instance(mut self, value: bool) -> Self {
    unsafe {
      self.drawIndirectFirstInstance = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_clamp(mut self, value: bool) -> Self {
    unsafe {
      self.depthClamp = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_bias_clamp(mut self, value: bool) -> Self {
    unsafe {
      self.depthBiasClamp = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_fill_mode_non_solid(mut self, value: bool) -> Self {
    unsafe {
      self.fillModeNonSolid = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_bounds(mut self, value: bool) -> Self {
    unsafe {
      self.depthBounds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_wide_lines(mut self, value: bool) -> Self {
    unsafe {
      self.wideLines = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_large_points(mut self, value: bool) -> Self {
    unsafe {
      self.largePoints = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_alpha_to_one(mut self, value: bool) -> Self {
    unsafe {
      self.alphaToOne = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_multi_viewport(mut self, value: bool) -> Self {
    unsafe {
      self.multiViewport = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sampler_anisotropy(mut self, value: bool) -> Self {
    unsafe {
      self.samplerAnisotropy = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_texture_compression_etc2(mut self, value: bool) -> Self {
    unsafe {
      self.textureCompressionETC2 = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_texture_compression_astc_ldr(mut self, value: bool) -> Self {
    unsafe {
      self.textureCompressionASTC_LDR = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_texture_compression_bc(mut self, value: bool) -> Self {
    unsafe {
      self.textureCompressionBC = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_occlusion_query_precise(mut self, value: bool) -> Self {
    unsafe {
      self.occlusionQueryPrecise = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_pipeline_statistics_query(mut self, value: bool) -> Self {
    unsafe {
      self.pipelineStatisticsQuery = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_vertex_pipeline_stores_and_atomics(mut self, value: bool) -> Self {
    unsafe {
      self.vertexPipelineStoresAndAtomics = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_fragment_stores_and_atomics(mut self, value: bool) -> Self {
    unsafe {
      self.fragmentStoresAndAtomics = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_tessellation_and_geometry_point_size(mut self, value: bool) -> Self {
    unsafe {
      self.shaderTessellationAndGeometryPointSize = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_image_gather_extended(mut self, value: bool) -> Self {
    unsafe {
      self.shaderImageGatherExtended = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_storage_image_extended_formats(mut self, value: bool) -> Self {
    unsafe {
      self.shaderStorageImageExtendedFormats = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_storage_image_multisample(mut self, value: bool) -> Self {
    unsafe {
      self.shaderStorageImageMultisample = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_storage_image_read_without_format(mut self, value: bool) -> Self {
    unsafe {
      self.shaderStorageImageReadWithoutFormat = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_storage_image_write_without_format(mut self, value: bool) -> Self {
    unsafe {
      self.shaderStorageImageWriteWithoutFormat = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_uniform_buffer_array_dynamic_indexing(mut self, value: bool) -> Self {
    unsafe {
      self.shaderUniformBufferArrayDynamicIndexing = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_sampled_image_array_dynamic_indexing(mut self, value: bool) -> Self {
    unsafe {
      self.shaderSampledImageArrayDynamicIndexing = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_storage_buffer_array_dynamic_indexing(mut self, value: bool) -> Self {
    unsafe {
      self.shaderStorageBufferArrayDynamicIndexing = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_storage_image_array_dynamic_indexing(mut self, value: bool) -> Self {
    unsafe {
      self.shaderStorageImageArrayDynamicIndexing = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_clip_distance(mut self, value: bool) -> Self {
    unsafe {
      self.shaderClipDistance = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_cull_distance(mut self, value: bool) -> Self {
    unsafe {
      self.shaderCullDistance = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_float64(mut self, value: bool) -> Self {
    unsafe {
      self.shaderFloat64 = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_int64(mut self, value: bool) -> Self {
    unsafe {
      self.shaderInt64 = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_int16(mut self, value: bool) -> Self {
    unsafe {
      self.shaderInt16 = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_resource_residency(mut self, value: bool) -> Self {
    unsafe {
      self.shaderResourceResidency = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_shader_resource_min_lod(mut self, value: bool) -> Self {
    unsafe {
      self.shaderResourceMinLod = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_binding(mut self, value: bool) -> Self {
    unsafe {
      self.sparseBinding = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_residency_buffer(mut self, value: bool) -> Self {
    unsafe {
      self.sparseResidencyBuffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_residency_image2_d(mut self, value: bool) -> Self {
    unsafe {
      self.sparseResidencyImage2D = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_residency_image3_d(mut self, value: bool) -> Self {
    unsafe {
      self.sparseResidencyImage3D = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_residency2_samples(mut self, value: bool) -> Self {
    unsafe {
      self.sparseResidency2Samples = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_residency4_samples(mut self, value: bool) -> Self {
    unsafe {
      self.sparseResidency4Samples = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_residency8_samples(mut self, value: bool) -> Self {
    unsafe {
      self.sparseResidency8Samples = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_residency16_samples(mut self, value: bool) -> Self {
    unsafe {
      self.sparseResidency16Samples = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sparse_residency_aliased(mut self, value: bool) -> Self {
    unsafe {
      self.sparseResidencyAliased = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_variable_multisample_rate(mut self, value: bool) -> Self {
    unsafe {
      self.variableMultisampleRate = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_inherited_queries(mut self, value: bool) -> Self {
    unsafe {
      self.inheritedQueries = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn is_robust_buffer_access(&self) -> bool {
    self.robustBufferAccess != 0
  }
  #[inline]
  pub fn is_full_draw_index_uint32(&self) -> bool {
    self.fullDrawIndexUint32 != 0
  }
  #[inline]
  pub fn is_image_cube_array(&self) -> bool {
    self.imageCubeArray != 0
  }
  #[inline]
  pub fn is_independent_blend(&self) -> bool {
    self.independentBlend != 0
  }
  #[inline]
  pub fn is_geometry_shader(&self) -> bool {
    self.geometryShader != 0
  }
  #[inline]
  pub fn is_tessellation_shader(&self) -> bool {
    self.tessellationShader != 0
  }
  #[inline]
  pub fn is_sample_rate_shading(&self) -> bool {
    self.sampleRateShading != 0
  }
  #[inline]
  pub fn is_dual_src_blend(&self) -> bool {
    self.dualSrcBlend != 0
  }
  #[inline]
  pub fn is_logic_op(&self) -> bool {
    self.logicOp != 0
  }
  #[inline]
  pub fn is_multi_draw_indirect(&self) -> bool {
    self.multiDrawIndirect != 0
  }
  #[inline]
  pub fn is_draw_indirect_first_instance(&self) -> bool {
    self.drawIndirectFirstInstance != 0
  }
  #[inline]
  pub fn is_depth_clamp(&self) -> bool {
    self.depthClamp != 0
  }
  #[inline]
  pub fn is_depth_bias_clamp(&self) -> bool {
    self.depthBiasClamp != 0
  }
  #[inline]
  pub fn is_fill_mode_non_solid(&self) -> bool {
    self.fillModeNonSolid != 0
  }
  #[inline]
  pub fn is_depth_bounds(&self) -> bool {
    self.depthBounds != 0
  }
  #[inline]
  pub fn is_wide_lines(&self) -> bool {
    self.wideLines != 0
  }
  #[inline]
  pub fn is_large_points(&self) -> bool {
    self.largePoints != 0
  }
  #[inline]
  pub fn is_alpha_to_one(&self) -> bool {
    self.alphaToOne != 0
  }
  #[inline]
  pub fn is_multi_viewport(&self) -> bool {
    self.multiViewport != 0
  }
  #[inline]
  pub fn is_sampler_anisotropy(&self) -> bool {
    self.samplerAnisotropy != 0
  }
  #[inline]
  pub fn is_texture_compression_etc2(&self) -> bool {
    self.textureCompressionETC2 != 0
  }
  #[inline]
  pub fn is_texture_compression_astc_ldr(&self) -> bool {
    self.textureCompressionASTC_LDR != 0
  }
  #[inline]
  pub fn is_texture_compression_bc(&self) -> bool {
    self.textureCompressionBC != 0
  }
  #[inline]
  pub fn is_occlusion_query_precise(&self) -> bool {
    self.occlusionQueryPrecise != 0
  }
  #[inline]
  pub fn is_pipeline_statistics_query(&self) -> bool {
    self.pipelineStatisticsQuery != 0
  }
  #[inline]
  pub fn is_vertex_pipeline_stores_and_atomics(&self) -> bool {
    self.vertexPipelineStoresAndAtomics != 0
  }
  #[inline]
  pub fn is_fragment_stores_and_atomics(&self) -> bool {
    self.fragmentStoresAndAtomics != 0
  }
  #[inline]
  pub fn is_shader_tessellation_and_geometry_point_size(&self) -> bool {
    self.shaderTessellationAndGeometryPointSize != 0
  }
  #[inline]
  pub fn is_shader_image_gather_extended(&self) -> bool {
    self.shaderImageGatherExtended != 0
  }
  #[inline]
  pub fn is_shader_storage_image_extended_formats(&self) -> bool {
    self.shaderStorageImageExtendedFormats != 0
  }
  #[inline]
  pub fn is_shader_storage_image_multisample(&self) -> bool {
    self.shaderStorageImageMultisample != 0
  }
  #[inline]
  pub fn is_shader_storage_image_read_without_format(&self) -> bool {
    self.shaderStorageImageReadWithoutFormat != 0
  }
  #[inline]
  pub fn is_shader_storage_image_write_without_format(&self) -> bool {
    self.shaderStorageImageWriteWithoutFormat != 0
  }
  #[inline]
  pub fn is_shader_uniform_buffer_array_dynamic_indexing(&self) -> bool {
    self.shaderUniformBufferArrayDynamicIndexing != 0
  }
  #[inline]
  pub fn is_shader_sampled_image_array_dynamic_indexing(&self) -> bool {
    self.shaderSampledImageArrayDynamicIndexing != 0
  }
  #[inline]
  pub fn is_shader_storage_buffer_array_dynamic_indexing(&self) -> bool {
    self.shaderStorageBufferArrayDynamicIndexing != 0
  }
  #[inline]
  pub fn is_shader_storage_image_array_dynamic_indexing(&self) -> bool {
    self.shaderStorageImageArrayDynamicIndexing != 0
  }
  #[inline]
  pub fn is_shader_clip_distance(&self) -> bool {
    self.shaderClipDistance != 0
  }
  #[inline]
  pub fn is_shader_cull_distance(&self) -> bool {
    self.shaderCullDistance != 0
  }
  #[inline]
  pub fn is_shader_float64(&self) -> bool {
    self.shaderFloat64 != 0
  }
  #[inline]
  pub fn is_shader_int64(&self) -> bool {
    self.shaderInt64 != 0
  }
  #[inline]
  pub fn is_shader_int16(&self) -> bool {
    self.shaderInt16 != 0
  }
  #[inline]
  pub fn is_shader_resource_residency(&self) -> bool {
    self.shaderResourceResidency != 0
  }
  #[inline]
  pub fn is_shader_resource_min_lod(&self) -> bool {
    self.shaderResourceMinLod != 0
  }
  #[inline]
  pub fn is_sparse_binding(&self) -> bool {
    self.sparseBinding != 0
  }
  #[inline]
  pub fn is_sparse_residency_buffer(&self) -> bool {
    self.sparseResidencyBuffer != 0
  }
  #[inline]
  pub fn is_sparse_residency_image2_d(&self) -> bool {
    self.sparseResidencyImage2D != 0
  }
  #[inline]
  pub fn is_sparse_residency_image3_d(&self) -> bool {
    self.sparseResidencyImage3D != 0
  }
  #[inline]
  pub fn is_sparse_residency2_samples(&self) -> bool {
    self.sparseResidency2Samples != 0
  }
  #[inline]
  pub fn is_sparse_residency4_samples(&self) -> bool {
    self.sparseResidency4Samples != 0
  }
  #[inline]
  pub fn is_sparse_residency8_samples(&self) -> bool {
    self.sparseResidency8Samples != 0
  }
  #[inline]
  pub fn is_sparse_residency16_samples(&self) -> bool {
    self.sparseResidency16Samples != 0
  }
  #[inline]
  pub fn is_sparse_residency_aliased(&self) -> bool {
    self.sparseResidencyAliased != 0
  }
  #[inline]
  pub fn is_variable_multisample_rate(&self) -> bool {
    self.variableMultisampleRate != 0
  }
  #[inline]
  pub fn is_inherited_queries(&self) -> bool {
    self.inheritedQueries != 0
  }
}
impl Default for VkPhysicalDeviceFeatures {
  fn default() -> VkPhysicalDeviceFeatures {
    VkPhysicalDeviceFeatures::new()
  }
}
unsafe impl Struct for VkPhysicalDeviceFeatures {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_features() {
  assert_size!(220, VkPhysicalDeviceFeatures);
}

/// Available image formats
pub use enums::VkFormat;

/// Bitmask specifying features supported by a buffer
pub use enums::VkFormatFeatureFlagBits;

/// Bitmask of VkFormatFeatureFlagBits
pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;

/// Structure specifying image format properties
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFormatProperties {
  pub linearTilingFeatures: VkFormatFeatureFlags,
  pub optimalTilingFeatures: VkFormatFeatureFlags,
  pub bufferFeatures: VkFormatFeatureFlags,
}
impl VkFormatProperties {
  #[inline]
  pub fn linear_tiling_features(&self) -> VkFormatFeatureFlags {
    self.linearTilingFeatures
  }
  #[inline]
  pub fn optimal_tiling_features(&self) -> VkFormatFeatureFlags {
    self.optimalTilingFeatures
  }
  #[inline]
  pub fn buffer_features(&self) -> VkFormatFeatureFlags {
    self.bufferFeatures
  }
}
unsafe impl Struct for VkFormatProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_format_properties() {
  assert_size!(12, VkFormatProperties);
}

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

/// Structure specifying a three-dimensional extent
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent3D {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
}
impl VkExtent3D {
  #[inline]
  pub fn new() -> VkExtent3D {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_width(mut self, value: u32) -> Self {
    self.width = value;
    self
  }
  #[inline]
  pub fn set_height(mut self, value: u32) -> Self {
    self.height = value;
    self
  }
  #[inline]
  pub fn set_depth(mut self, value: u32) -> Self {
    self.depth = value;
    self
  }
  #[inline]
  pub fn width(&self) -> u32 {
    self.width
  }
  #[inline]
  pub fn height(&self) -> u32 {
    self.height
  }
  #[inline]
  pub fn depth(&self) -> u32 {
    self.depth
  }
}
impl Default for VkExtent3D {
  fn default() -> VkExtent3D {
    VkExtent3D::new()
  }
}
unsafe impl Struct for VkExtent3D {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extent3_d() {
  assert_size!(12, VkExtent3D);
}

/// Bitmask specifying sample counts supported for an image used for storage
/// operations
pub use enums::VkSampleCountFlagBits;

/// Bitmask of VkSampleCountFlagBits
pub type VkSampleCountFlags = VkSampleCountFlagBits;

/// Structure specifying a image format properties
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatProperties {
  pub maxExtent: VkExtent3D,
  pub maxMipLevels: u32,
  pub maxArrayLayers: u32,
  pub sampleCounts: VkSampleCountFlags,
  pub maxResourceSize: VkDeviceSize,
}
impl VkImageFormatProperties {
  #[inline]
  pub fn max_extent(&self) -> &VkExtent3D {
    &self.maxExtent
  }
  #[inline]
  pub fn max_mip_levels(&self) -> u32 {
    self.maxMipLevels
  }
  #[inline]
  pub fn max_array_layers(&self) -> u32 {
    self.maxArrayLayers
  }
  #[inline]
  pub fn sample_counts(&self) -> VkSampleCountFlags {
    self.sampleCounts
  }
  #[inline]
  pub fn max_resource_size(&self) -> VkDeviceSize {
    self.maxResourceSize
  }
}
unsafe impl Struct for VkImageFormatProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_format_properties() {
  assert_size!(32, VkImageFormatProperties);
}

/// Supported physical device types
pub use enums::VkPhysicalDeviceType;

/// Structure reporting implementation-dependent physical device limits
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
  timestampComputeAndGraphics: VkBool32,
  pub timestampPeriod: f32,
  pub maxClipDistances: u32,
  pub maxCullDistances: u32,
  pub maxCombinedClipAndCullDistances: u32,
  pub discreteQueuePriorities: u32,
  pub pointSizeRange: [f32; 2],
  pub lineWidthRange: [f32; 2],
  pub pointSizeGranularity: f32,
  pub lineWidthGranularity: f32,
  strictLines: VkBool32,
  standardSampleLocations: VkBool32,
  pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
  pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
  pub nonCoherentAtomSize: VkDeviceSize,
}
impl VkPhysicalDeviceLimits {
  #[inline]
  pub fn max_image_dimension1_d(&self) -> u32 {
    self.maxImageDimension1D
  }
  #[inline]
  pub fn max_image_dimension2_d(&self) -> u32 {
    self.maxImageDimension2D
  }
  #[inline]
  pub fn max_image_dimension3_d(&self) -> u32 {
    self.maxImageDimension3D
  }
  #[inline]
  pub fn max_image_dimension_cube(&self) -> u32 {
    self.maxImageDimensionCube
  }
  #[inline]
  pub fn max_image_array_layers(&self) -> u32 {
    self.maxImageArrayLayers
  }
  #[inline]
  pub fn max_texel_buffer_elements(&self) -> u32 {
    self.maxTexelBufferElements
  }
  #[inline]
  pub fn max_uniform_buffer_range(&self) -> u32 {
    self.maxUniformBufferRange
  }
  #[inline]
  pub fn max_storage_buffer_range(&self) -> u32 {
    self.maxStorageBufferRange
  }
  #[inline]
  pub fn max_push_constants_size(&self) -> u32 {
    self.maxPushConstantsSize
  }
  #[inline]
  pub fn max_memory_allocation_count(&self) -> u32 {
    self.maxMemoryAllocationCount
  }
  #[inline]
  pub fn max_sampler_allocation_count(&self) -> u32 {
    self.maxSamplerAllocationCount
  }
  #[inline]
  pub fn buffer_image_granularity(&self) -> VkDeviceSize {
    self.bufferImageGranularity
  }
  #[inline]
  pub fn sparse_address_space_size(&self) -> VkDeviceSize {
    self.sparseAddressSpaceSize
  }
  #[inline]
  pub fn max_bound_descriptor_sets(&self) -> u32 {
    self.maxBoundDescriptorSets
  }
  #[inline]
  pub fn max_per_stage_descriptor_samplers(&self) -> u32 {
    self.maxPerStageDescriptorSamplers
  }
  #[inline]
  pub fn max_per_stage_descriptor_uniform_buffers(&self) -> u32 {
    self.maxPerStageDescriptorUniformBuffers
  }
  #[inline]
  pub fn max_per_stage_descriptor_storage_buffers(&self) -> u32 {
    self.maxPerStageDescriptorStorageBuffers
  }
  #[inline]
  pub fn max_per_stage_descriptor_sampled_images(&self) -> u32 {
    self.maxPerStageDescriptorSampledImages
  }
  #[inline]
  pub fn max_per_stage_descriptor_storage_images(&self) -> u32 {
    self.maxPerStageDescriptorStorageImages
  }
  #[inline]
  pub fn max_per_stage_descriptor_input_attachments(&self) -> u32 {
    self.maxPerStageDescriptorInputAttachments
  }
  #[inline]
  pub fn max_per_stage_resources(&self) -> u32 {
    self.maxPerStageResources
  }
  #[inline]
  pub fn max_descriptor_set_samplers(&self) -> u32 {
    self.maxDescriptorSetSamplers
  }
  #[inline]
  pub fn max_descriptor_set_uniform_buffers(&self) -> u32 {
    self.maxDescriptorSetUniformBuffers
  }
  #[inline]
  pub fn max_descriptor_set_uniform_buffers_dynamic(&self) -> u32 {
    self.maxDescriptorSetUniformBuffersDynamic
  }
  #[inline]
  pub fn max_descriptor_set_storage_buffers(&self) -> u32 {
    self.maxDescriptorSetStorageBuffers
  }
  #[inline]
  pub fn max_descriptor_set_storage_buffers_dynamic(&self) -> u32 {
    self.maxDescriptorSetStorageBuffersDynamic
  }
  #[inline]
  pub fn max_descriptor_set_sampled_images(&self) -> u32 {
    self.maxDescriptorSetSampledImages
  }
  #[inline]
  pub fn max_descriptor_set_storage_images(&self) -> u32 {
    self.maxDescriptorSetStorageImages
  }
  #[inline]
  pub fn max_descriptor_set_input_attachments(&self) -> u32 {
    self.maxDescriptorSetInputAttachments
  }
  #[inline]
  pub fn max_vertex_input_attributes(&self) -> u32 {
    self.maxVertexInputAttributes
  }
  #[inline]
  pub fn max_vertex_input_bindings(&self) -> u32 {
    self.maxVertexInputBindings
  }
  #[inline]
  pub fn max_vertex_input_attribute_offset(&self) -> u32 {
    self.maxVertexInputAttributeOffset
  }
  #[inline]
  pub fn max_vertex_input_binding_stride(&self) -> u32 {
    self.maxVertexInputBindingStride
  }
  #[inline]
  pub fn max_vertex_output_components(&self) -> u32 {
    self.maxVertexOutputComponents
  }
  #[inline]
  pub fn max_tessellation_generation_level(&self) -> u32 {
    self.maxTessellationGenerationLevel
  }
  #[inline]
  pub fn max_tessellation_patch_size(&self) -> u32 {
    self.maxTessellationPatchSize
  }
  #[inline]
  pub fn max_tessellation_control_per_vertex_input_components(&self) -> u32 {
    self.maxTessellationControlPerVertexInputComponents
  }
  #[inline]
  pub fn max_tessellation_control_per_vertex_output_components(&self) -> u32 {
    self.maxTessellationControlPerVertexOutputComponents
  }
  #[inline]
  pub fn max_tessellation_control_per_patch_output_components(&self) -> u32 {
    self.maxTessellationControlPerPatchOutputComponents
  }
  #[inline]
  pub fn max_tessellation_control_total_output_components(&self) -> u32 {
    self.maxTessellationControlTotalOutputComponents
  }
  #[inline]
  pub fn max_tessellation_evaluation_input_components(&self) -> u32 {
    self.maxTessellationEvaluationInputComponents
  }
  #[inline]
  pub fn max_tessellation_evaluation_output_components(&self) -> u32 {
    self.maxTessellationEvaluationOutputComponents
  }
  #[inline]
  pub fn max_geometry_shader_invocations(&self) -> u32 {
    self.maxGeometryShaderInvocations
  }
  #[inline]
  pub fn max_geometry_input_components(&self) -> u32 {
    self.maxGeometryInputComponents
  }
  #[inline]
  pub fn max_geometry_output_components(&self) -> u32 {
    self.maxGeometryOutputComponents
  }
  #[inline]
  pub fn max_geometry_output_vertices(&self) -> u32 {
    self.maxGeometryOutputVertices
  }
  #[inline]
  pub fn max_geometry_total_output_components(&self) -> u32 {
    self.maxGeometryTotalOutputComponents
  }
  #[inline]
  pub fn max_fragment_input_components(&self) -> u32 {
    self.maxFragmentInputComponents
  }
  #[inline]
  pub fn max_fragment_output_attachments(&self) -> u32 {
    self.maxFragmentOutputAttachments
  }
  #[inline]
  pub fn max_fragment_dual_src_attachments(&self) -> u32 {
    self.maxFragmentDualSrcAttachments
  }
  #[inline]
  pub fn max_fragment_combined_output_resources(&self) -> u32 {
    self.maxFragmentCombinedOutputResources
  }
  #[inline]
  pub fn max_compute_shared_memory_size(&self) -> u32 {
    self.maxComputeSharedMemorySize
  }
  #[inline]
  pub fn max_compute_work_group_count(&self) -> [u32; 3] {
    self.maxComputeWorkGroupCount
  }
  #[inline]
  pub fn max_compute_work_group_invocations(&self) -> u32 {
    self.maxComputeWorkGroupInvocations
  }
  #[inline]
  pub fn max_compute_work_group_size(&self) -> [u32; 3] {
    self.maxComputeWorkGroupSize
  }
  #[inline]
  pub fn sub_pixel_precision_bits(&self) -> u32 {
    self.subPixelPrecisionBits
  }
  #[inline]
  pub fn sub_texel_precision_bits(&self) -> u32 {
    self.subTexelPrecisionBits
  }
  #[inline]
  pub fn mipmap_precision_bits(&self) -> u32 {
    self.mipmapPrecisionBits
  }
  #[inline]
  pub fn max_draw_indexed_index_value(&self) -> u32 {
    self.maxDrawIndexedIndexValue
  }
  #[inline]
  pub fn max_draw_indirect_count(&self) -> u32 {
    self.maxDrawIndirectCount
  }
  #[inline]
  pub fn max_sampler_lod_bias(&self) -> f32 {
    self.maxSamplerLodBias
  }
  #[inline]
  pub fn max_sampler_anisotropy(&self) -> f32 {
    self.maxSamplerAnisotropy
  }
  #[inline]
  pub fn max_viewports(&self) -> u32 {
    self.maxViewports
  }
  #[inline]
  pub fn max_viewport_dimensions(&self) -> [u32; 2] {
    self.maxViewportDimensions
  }
  #[inline]
  pub fn viewport_bounds_range(&self) -> [f32; 2] {
    self.viewportBoundsRange
  }
  #[inline]
  pub fn viewport_sub_pixel_bits(&self) -> u32 {
    self.viewportSubPixelBits
  }
  #[inline]
  pub fn min_memory_map_alignment(&self) -> usize {
    self.minMemoryMapAlignment
  }
  #[inline]
  pub fn min_texel_buffer_offset_alignment(&self) -> VkDeviceSize {
    self.minTexelBufferOffsetAlignment
  }
  #[inline]
  pub fn min_uniform_buffer_offset_alignment(&self) -> VkDeviceSize {
    self.minUniformBufferOffsetAlignment
  }
  #[inline]
  pub fn min_storage_buffer_offset_alignment(&self) -> VkDeviceSize {
    self.minStorageBufferOffsetAlignment
  }
  #[inline]
  pub fn min_texel_offset(&self) -> i32 {
    self.minTexelOffset
  }
  #[inline]
  pub fn max_texel_offset(&self) -> u32 {
    self.maxTexelOffset
  }
  #[inline]
  pub fn min_texel_gather_offset(&self) -> i32 {
    self.minTexelGatherOffset
  }
  #[inline]
  pub fn max_texel_gather_offset(&self) -> u32 {
    self.maxTexelGatherOffset
  }
  #[inline]
  pub fn min_interpolation_offset(&self) -> f32 {
    self.minInterpolationOffset
  }
  #[inline]
  pub fn max_interpolation_offset(&self) -> f32 {
    self.maxInterpolationOffset
  }
  #[inline]
  pub fn sub_pixel_interpolation_offset_bits(&self) -> u32 {
    self.subPixelInterpolationOffsetBits
  }
  #[inline]
  pub fn max_framebuffer_width(&self) -> u32 {
    self.maxFramebufferWidth
  }
  #[inline]
  pub fn max_framebuffer_height(&self) -> u32 {
    self.maxFramebufferHeight
  }
  #[inline]
  pub fn max_framebuffer_layers(&self) -> u32 {
    self.maxFramebufferLayers
  }
  #[inline]
  pub fn framebuffer_color_sample_counts(&self) -> VkSampleCountFlags {
    self.framebufferColorSampleCounts
  }
  #[inline]
  pub fn framebuffer_depth_sample_counts(&self) -> VkSampleCountFlags {
    self.framebufferDepthSampleCounts
  }
  #[inline]
  pub fn framebuffer_stencil_sample_counts(&self) -> VkSampleCountFlags {
    self.framebufferStencilSampleCounts
  }
  #[inline]
  pub fn framebuffer_no_attachments_sample_counts(&self) -> VkSampleCountFlags {
    self.framebufferNoAttachmentsSampleCounts
  }
  #[inline]
  pub fn max_color_attachments(&self) -> u32 {
    self.maxColorAttachments
  }
  #[inline]
  pub fn sampled_image_color_sample_counts(&self) -> VkSampleCountFlags {
    self.sampledImageColorSampleCounts
  }
  #[inline]
  pub fn sampled_image_integer_sample_counts(&self) -> VkSampleCountFlags {
    self.sampledImageIntegerSampleCounts
  }
  #[inline]
  pub fn sampled_image_depth_sample_counts(&self) -> VkSampleCountFlags {
    self.sampledImageDepthSampleCounts
  }
  #[inline]
  pub fn sampled_image_stencil_sample_counts(&self) -> VkSampleCountFlags {
    self.sampledImageStencilSampleCounts
  }
  #[inline]
  pub fn storage_image_sample_counts(&self) -> VkSampleCountFlags {
    self.storageImageSampleCounts
  }
  #[inline]
  pub fn max_sample_mask_words(&self) -> u32 {
    self.maxSampleMaskWords
  }
  #[inline]
  pub fn is_timestamp_compute_and_graphics(&self) -> bool {
    self.timestampComputeAndGraphics != 0
  }
  #[inline]
  pub fn timestamp_period(&self) -> f32 {
    self.timestampPeriod
  }
  #[inline]
  pub fn max_clip_distances(&self) -> u32 {
    self.maxClipDistances
  }
  #[inline]
  pub fn max_cull_distances(&self) -> u32 {
    self.maxCullDistances
  }
  #[inline]
  pub fn max_combined_clip_and_cull_distances(&self) -> u32 {
    self.maxCombinedClipAndCullDistances
  }
  #[inline]
  pub fn discrete_queue_priorities(&self) -> u32 {
    self.discreteQueuePriorities
  }
  #[inline]
  pub fn point_size_range(&self) -> [f32; 2] {
    self.pointSizeRange
  }
  #[inline]
  pub fn line_width_range(&self) -> [f32; 2] {
    self.lineWidthRange
  }
  #[inline]
  pub fn point_size_granularity(&self) -> f32 {
    self.pointSizeGranularity
  }
  #[inline]
  pub fn line_width_granularity(&self) -> f32 {
    self.lineWidthGranularity
  }
  #[inline]
  pub fn is_strict_lines(&self) -> bool {
    self.strictLines != 0
  }
  #[inline]
  pub fn is_standard_sample_locations(&self) -> bool {
    self.standardSampleLocations != 0
  }
  #[inline]
  pub fn optimal_buffer_copy_offset_alignment(&self) -> VkDeviceSize {
    self.optimalBufferCopyOffsetAlignment
  }
  #[inline]
  pub fn optimal_buffer_copy_row_pitch_alignment(&self) -> VkDeviceSize {
    self.optimalBufferCopyRowPitchAlignment
  }
  #[inline]
  pub fn non_coherent_atom_size(&self) -> VkDeviceSize {
    self.nonCoherentAtomSize
  }
}
unsafe impl Struct for VkPhysicalDeviceLimits {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_limits() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(472 + ptr_size * 4, VkPhysicalDeviceLimits);
}

/// Structure specifying physical device sparse memory properties
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseProperties {
  residencyStandard2DBlockShape: VkBool32,
  residencyStandard2DMultisampleBlockShape: VkBool32,
  residencyStandard3DBlockShape: VkBool32,
  residencyAlignedMipSize: VkBool32,
  residencyNonResidentStrict: VkBool32,
}
impl VkPhysicalDeviceSparseProperties {
  #[inline]
  pub fn is_residency_standard2_d_block_shape(&self) -> bool {
    self.residencyStandard2DBlockShape != 0
  }
  #[inline]
  pub fn is_residency_standard2_d_multisample_block_shape(&self) -> bool {
    self.residencyStandard2DMultisampleBlockShape != 0
  }
  #[inline]
  pub fn is_residency_standard3_d_block_shape(&self) -> bool {
    self.residencyStandard3DBlockShape != 0
  }
  #[inline]
  pub fn is_residency_aligned_mip_size(&self) -> bool {
    self.residencyAlignedMipSize != 0
  }
  #[inline]
  pub fn is_residency_non_resident_strict(&self) -> bool {
    self.residencyNonResidentStrict != 0
  }
}
unsafe impl Struct for VkPhysicalDeviceSparseProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_sparse_properties() {
  assert_size!(20, VkPhysicalDeviceSparseProperties);
}

/// Structure specifying physical device properties
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
impl VkPhysicalDeviceProperties {
  #[inline]
  pub fn api_version(&self) -> u32 {
    self.apiVersion
  }
  #[inline]
  pub fn driver_version(&self) -> u32 {
    self.driverVersion
  }
  #[inline]
  pub fn vendor_id(&self) -> u32 {
    self.vendorID
  }
  #[inline]
  pub fn device_id(&self) -> u32 {
    self.deviceID
  }
  #[inline]
  pub fn device_type(&self) -> VkPhysicalDeviceType {
    self.deviceType
  }
  #[inline]
  pub fn device_name(&self) -> [c_char; enums::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize] {
    self.deviceName
  }
  #[inline]
  pub fn pipeline_cache_uuid(&self) -> [u8; enums::VK_UUID_SIZE as usize] {
    self.pipelineCacheUUID
  }
  #[inline]
  pub fn limits(&self) -> &VkPhysicalDeviceLimits {
    &self.limits
  }
  #[inline]
  pub fn sparse_properties(&self) -> &VkPhysicalDeviceSparseProperties {
    &self.sparseProperties
  }
}
unsafe impl Struct for VkPhysicalDeviceProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_properties() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(776 + ptr_size * 6, VkPhysicalDeviceProperties);
}

/// Bitmask specifying capabilities of queues in a queue family
pub use enums::VkQueueFlagBits;

/// Bitmask of VkQueueFlagBits
pub type VkQueueFlags = VkQueueFlagBits;

/// Structure providing information about a queue family
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueueFamilyProperties {
  pub queueFlags: VkQueueFlags,
  pub queueCount: u32,
  pub timestampValidBits: u32,
  pub minImageTransferGranularity: VkExtent3D,
}
impl VkQueueFamilyProperties {
  #[inline]
  pub fn queue_flags(&self) -> VkQueueFlags {
    self.queueFlags
  }
  #[inline]
  pub fn queue_count(&self) -> u32 {
    self.queueCount
  }
  #[inline]
  pub fn timestamp_valid_bits(&self) -> u32 {
    self.timestampValidBits
  }
  #[inline]
  pub fn min_image_transfer_granularity(&self) -> &VkExtent3D {
    &self.minImageTransferGranularity
  }
}
unsafe impl Struct for VkQueueFamilyProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_queue_family_properties() {
  assert_size!(24, VkQueueFamilyProperties);
}

/// Bitmask specifying properties for a memory type
pub use enums::VkMemoryPropertyFlagBits;

/// Bitmask of VkMemoryPropertyFlagBits
pub type VkMemoryPropertyFlags = VkMemoryPropertyFlagBits;

/// Structure specifying memory type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryType {
  pub propertyFlags: VkMemoryPropertyFlags,
  pub heapIndex: u32,
}
impl VkMemoryType {
  #[inline]
  pub fn property_flags(&self) -> VkMemoryPropertyFlags {
    self.propertyFlags
  }
  #[inline]
  pub fn heap_index(&self) -> u32 {
    self.heapIndex
  }
}
unsafe impl Struct for VkMemoryType {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_type() {
  assert_size!(8, VkMemoryType);
}

/// Bitmask specifying attribute flags for a heap
pub use enums::VkMemoryHeapFlagBits;

/// Bitmask of VkMemoryHeapFlagBits
pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;

/// Structure specifying a memory heap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHeap {
  pub size: VkDeviceSize,
  pub flags: VkMemoryHeapFlags,
}
impl VkMemoryHeap {
  #[inline]
  pub fn size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn flags(&self) -> VkMemoryHeapFlags {
    self.flags
  }
}
unsafe impl Struct for VkMemoryHeap {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_heap() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 1, VkMemoryHeap);
}

/// Structure specifying physical device memory properties
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties {
  pub memoryTypeCount: u32,
  pub memoryTypes: [VkMemoryType; enums::VK_MAX_MEMORY_TYPES as usize],
  pub memoryHeapCount: u32,
  pub memoryHeaps: [VkMemoryHeap; enums::VK_MAX_MEMORY_HEAPS as usize],
}
impl VkPhysicalDeviceMemoryProperties {
  #[inline]
  pub fn memory_type_count(&self) -> u32 {
    self.memoryTypeCount
  }
  #[inline]
  pub fn memory_types(&self) -> [VkMemoryType; enums::VK_MAX_MEMORY_TYPES as usize] {
    self.memoryTypes
  }
  #[inline]
  pub fn memory_heap_count(&self) -> u32 {
    self.memoryHeapCount
  }
  #[inline]
  pub fn memory_heaps(&self) -> [VkMemoryHeap; enums::VK_MAX_MEMORY_HEAPS as usize] {
    self.memoryHeaps
  }
}
unsafe impl Struct for VkPhysicalDeviceMemoryProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_memory_properties() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(392 + ptr_size * 16, VkPhysicalDeviceMemoryProperties);
}

/// Dummy function pointer type returned by queries
#[allow(non_camel_case_types)]
pub type PFN_vkVoidFunction = extern "system" fn();
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDevice__ {}

/// Opaque handle to a device object
pub type VkDevice<'l> = VkDispatchableHandle<'l, VkDevice__>;

/// Reserved for future use
pub type VkDeviceCreateFlags = VkFlags;

/// Reserved for future use
pub type VkDeviceQueueCreateFlags = VkFlags;

/// Structure specifying parameters of a newly created device queue
#[repr(C)]
pub struct VkDeviceQueueCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDeviceQueueCreateFlags,
  pub queueFamilyIndex: u32,
  queueCount: u32,
  pQueuePriorities: *const f32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkDeviceQueueCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkDeviceQueueCreateInfo<'l> {
    unsafe {
      VkDeviceQueueCreateInfo {
        sType: VkStructureType::DEVICE_QUEUE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDeviceQueueCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_queue_family_index(mut self, value: u32) -> Self {
    self.queueFamilyIndex = value;
    self
  }
  #[inline]
  pub fn set_queue_priorities(mut self, value: &'l [f32]) -> Self {
    self.queueCount = value.len() as u32;
    unsafe {
      self.pQueuePriorities = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkDeviceQueueCreateFlags {
    self.flags
  }
  #[inline]
  pub fn queue_family_index(&self) -> u32 {
    self.queueFamilyIndex
  }
  #[inline]
  pub fn queue_count(&self) -> u32 {
    self.queueCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkDeviceQueueCreateInfo<'l> {
  fn default() -> VkDeviceQueueCreateInfo<'l> {
    VkDeviceQueueCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkDeviceQueueCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_queue_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 4, VkDeviceQueueCreateInfo);
}

/// Structure specifying parameters of a newly created device
#[repr(C)]
pub struct VkDeviceCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDeviceCreateFlags,
  queueCreateInfoCount: u32,
  pQueueCreateInfos: *const VkDeviceQueueCreateInfo<'l>,
  enabledLayerCount: u32,
  ppEnabledLayerNames: *const *const c_char,
  enabledExtensionCount: u32,
  ppEnabledExtensionNames: *const *const c_char,
  pEnabledFeatures: *const VkPhysicalDeviceFeatures,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkDeviceCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkDeviceCreateInfo<'l> {
    unsafe {
      VkDeviceCreateInfo {
        sType: VkStructureType::DEVICE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDeviceCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_queue_create_infos(mut self, value: &'l [VkDeviceQueueCreateInfo<'l>]) -> Self {
    self.queueCreateInfoCount = value.len() as u32;
    unsafe {
      self.pQueueCreateInfos = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_enabled_features(mut self, value: Option<&'l VkPhysicalDeviceFeatures>) -> Self {
    unsafe {
      self.pEnabledFeatures = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkDeviceCreateFlags {
    self.flags
  }
  #[inline]
  pub fn queue_create_info_count(&self) -> u32 {
    self.queueCreateInfoCount
  }
  #[inline]
  pub fn enabled_layer_count(&self) -> u32 {
    self.enabledLayerCount
  }
  #[inline]
  pub fn enabled_extension_count(&self) -> u32 {
    self.enabledExtensionCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkDeviceCreateInfo<'l> {
  fn default() -> VkDeviceCreateInfo<'l> {
    VkDeviceCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkDeviceCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 8, VkDeviceCreateInfo);
}

/// Structure specifying a extension properties
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtensionProperties {
  pub extensionName: [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize],
  pub specVersion: u32,
}
impl VkExtensionProperties {
  #[inline]
  pub fn extension_name(&self) -> [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize] {
    self.extensionName
  }
  #[inline]
  pub fn spec_version(&self) -> u32 {
    self.specVersion
  }
}
unsafe impl Struct for VkExtensionProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extension_properties() {
  assert_size!(260, VkExtensionProperties);
}

/// Structure specifying layer properties
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkLayerProperties {
  pub layerName: [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize],
  pub specVersion: u32,
  pub implementationVersion: u32,
  pub description: [c_char; enums::VK_MAX_DESCRIPTION_SIZE as usize],
}
impl VkLayerProperties {
  #[inline]
  pub fn layer_name(&self) -> [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize] {
    self.layerName
  }
  #[inline]
  pub fn spec_version(&self) -> u32 {
    self.specVersion
  }
  #[inline]
  pub fn implementation_version(&self) -> u32 {
    self.implementationVersion
  }
  #[inline]
  pub fn description(&self) -> [c_char; enums::VK_MAX_DESCRIPTION_SIZE as usize] {
    self.description
  }
}
unsafe impl Struct for VkLayerProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_layer_properties() {
  assert_size!(520, VkLayerProperties);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkQueue__ {}

/// Opaque handle to a queue object
pub type VkQueue<'l> = VkDispatchableHandle<'l, VkQueue__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSemaphore__ {}

/// Opaque handle to a semaphore object
pub type VkSemaphore<'l> = VkNonDispatchableHandle<'l, VkSemaphore__>;

/// Bitmask specifying pipeline stages
pub use enums::VkPipelineStageFlagBits;

/// Bitmask of VkPipelineStageFlagBits
pub type VkPipelineStageFlags = VkPipelineStageFlagBits;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkCommandBuffer__ {}

/// Opaque handle to a command buffer object
pub type VkCommandBuffer<'l> = VkDispatchableHandle<'l, VkCommandBuffer__>;

/// Structure specifying a queue submit operation
#[repr(C)]
pub struct VkSubmitInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreCount: u32,
  pWaitSemaphores: *const u64,
  pWaitDstStageMask: *const VkPipelineStageFlags,
  commandBufferCount: u32,
  pCommandBuffers: *const usize,
  signalSemaphoreCount: u32,
  pSignalSemaphores: *const u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkSubmitInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkSubmitInfo<'l, 'h> {
    unsafe {
      VkSubmitInfo {
        sType: VkStructureType::SUBMIT_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_command_buffers(mut self, value: &'l [VkCommandBuffer<'h>]) -> Self {
    self.commandBufferCount = value.len() as u32;
    unsafe {
      self.pCommandBuffers = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphores(mut self, value: &'l [VkSemaphore<'h>]) -> Self {
    self.signalSemaphoreCount = value.len() as u32;
    unsafe {
      self.pSignalSemaphores = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn wait_semaphore_count(&self) -> u32 {
    self.waitSemaphoreCount
  }
  #[inline]
  pub fn command_buffer_count(&self) -> u32 {
    self.commandBufferCount
  }
  #[inline]
  pub fn signal_semaphore_count(&self) -> u32 {
    self.signalSemaphoreCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkSubmitInfo<'l, 'h> {
  fn default() -> VkSubmitInfo<'l, 'h> {
    VkSubmitInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkSubmitInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_submit_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 9, VkSubmitInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkFence__ {}

/// Opaque handle to a fence object
pub type VkFence<'l> = VkNonDispatchableHandle<'l, VkFence__>;

/// Structure containing parameters of a memory allocation
#[repr(C)]
pub struct VkMemoryAllocateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeIndex: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkMemoryAllocateInfo<'l> {
  #[inline]
  pub fn new() -> VkMemoryAllocateInfo<'l> {
    unsafe {
      VkMemoryAllocateInfo {
        sType: VkStructureType::MEMORY_ALLOCATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_allocation_size(mut self, value: VkDeviceSize) -> Self {
    self.allocationSize = value;
    self
  }
  #[inline]
  pub fn set_memory_type_index(mut self, value: u32) -> Self {
    self.memoryTypeIndex = value;
    self
  }
  #[inline]
  pub fn allocation_size(&self) -> VkDeviceSize {
    self.allocationSize
  }
  #[inline]
  pub fn memory_type_index(&self) -> u32 {
    self.memoryTypeIndex
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkMemoryAllocateInfo<'l> {
  fn default() -> VkMemoryAllocateInfo<'l> {
    VkMemoryAllocateInfo::new()
  }
}
unsafe impl<'l> Struct for VkMemoryAllocateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_allocate_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkMemoryAllocateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDeviceMemory__ {}

/// Opaque handle to a device memory object
pub type VkDeviceMemory<'l> = VkNonDispatchableHandle<'l, VkDeviceMemory__>;

/// Reserved for future use
pub type VkMemoryMapFlags = VkFlags;

/// Structure specifying a mapped memory range
#[repr(C)]
pub struct VkMappedMemoryRange<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  memory: u64,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkMappedMemoryRange<'l, 'h> {
  #[inline]
  pub fn new() -> VkMappedMemoryRange<'l, 'h> {
    unsafe {
      VkMappedMemoryRange {
        sType: VkStructureType::MAPPED_MEMORY_RANGE,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory<'h>) -> Self {
    unsafe {
      self.memory = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: VkDeviceSize) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_size(mut self, value: VkDeviceSize) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkMappedMemoryRange<'l, 'h> {
  fn default() -> VkMappedMemoryRange<'l, 'h> {
    VkMappedMemoryRange::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkMappedMemoryRange<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_mapped_memory_range() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 2, VkMappedMemoryRange);
}

/// Structure specifying memory requirements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryRequirements {
  pub size: VkDeviceSize,
  pub alignment: VkDeviceSize,
  pub memoryTypeBits: u32,
}
impl VkMemoryRequirements {
  #[inline]
  pub fn size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn alignment(&self) -> VkDeviceSize {
    self.alignment
  }
  #[inline]
  pub fn memory_type_bits(&self) -> u32 {
    self.memoryTypeBits
  }
}
unsafe impl Struct for VkMemoryRequirements {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_requirements() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 1, VkMemoryRequirements);
}

/// Bitmask specifying additional information about a sparse image resource
pub use enums::VkSparseImageFormatFlagBits;

/// Bitmask of VkSparseImageFormatFlagBits
pub type VkSparseImageFormatFlags = VkSparseImageFormatFlagBits;

/// Structure specifying sparse image format properties
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties {
  pub aspectMask: VkImageAspectFlags,
  pub imageGranularity: VkExtent3D,
  pub flags: VkSparseImageFormatFlags,
}
impl VkSparseImageFormatProperties {
  #[inline]
  pub fn aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn image_granularity(&self) -> &VkExtent3D {
    &self.imageGranularity
  }
  #[inline]
  pub fn flags(&self) -> VkSparseImageFormatFlags {
    self.flags
  }
}
unsafe impl Struct for VkSparseImageFormatProperties {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_format_properties() {
  assert_size!(20, VkSparseImageFormatProperties);
}

/// Structure specifying sparse image memory requirements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryRequirements {
  pub formatProperties: VkSparseImageFormatProperties,
  pub imageMipTailFirstLod: u32,
  pub imageMipTailSize: VkDeviceSize,
  pub imageMipTailOffset: VkDeviceSize,
  pub imageMipTailStride: VkDeviceSize,
}
impl VkSparseImageMemoryRequirements {
  #[inline]
  pub fn format_properties(&self) -> &VkSparseImageFormatProperties {
    &self.formatProperties
  }
  #[inline]
  pub fn image_mip_tail_first_lod(&self) -> u32 {
    self.imageMipTailFirstLod
  }
  #[inline]
  pub fn image_mip_tail_size(&self) -> VkDeviceSize {
    self.imageMipTailSize
  }
  #[inline]
  pub fn image_mip_tail_offset(&self) -> VkDeviceSize {
    self.imageMipTailOffset
  }
  #[inline]
  pub fn image_mip_tail_stride(&self) -> VkDeviceSize {
    self.imageMipTailStride
  }
}
unsafe impl Struct for VkSparseImageMemoryRequirements {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_requirements() {
  assert_size!(48, VkSparseImageMemoryRequirements);
}

/// Bitmask specifying usage of a sparse memory binding operation
pub use enums::VkSparseMemoryBindFlagBits;

/// Bitmask of VkSparseMemoryBindFlagBits
pub type VkSparseMemoryBindFlags = VkSparseMemoryBindFlagBits;

/// Structure specifying a sparse memory bind operation
#[repr(C)]
pub struct VkSparseMemoryBind<'h> {
  pub resourceOffset: VkDeviceSize,
  pub size: VkDeviceSize,
  memory: u64,
  pub memoryOffset: VkDeviceSize,
  pub flags: VkSparseMemoryBindFlags,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
impl<'h> VkSparseMemoryBind<'h> {
  #[inline]
  pub fn new() -> VkSparseMemoryBind<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_resource_offset(mut self, value: VkDeviceSize) -> Self {
    self.resourceOffset = value;
    self
  }
  #[inline]
  pub fn set_size(mut self, value: VkDeviceSize) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn set_memory(mut self, value: Option<VkDeviceMemory<'h>>) -> Self {
    unsafe {
      self.memory = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_memory_offset(mut self, value: VkDeviceSize) -> Self {
    self.memoryOffset = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSparseMemoryBindFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn resource_offset(&self) -> VkDeviceSize {
    self.resourceOffset
  }
  #[inline]
  pub fn size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn memory_offset(&self) -> VkDeviceSize {
    self.memoryOffset
  }
  #[inline]
  pub fn flags(&self) -> VkSparseMemoryBindFlags {
    self.flags
  }
}
impl<'h> Default for VkSparseMemoryBind<'h> {
  fn default() -> VkSparseMemoryBind<'h> {
    VkSparseMemoryBind::new()
  }
}
unsafe impl<'h> Struct for VkSparseMemoryBind<'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_memory_bind() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 1, VkSparseMemoryBind);
}

/// Structure specifying a sparse buffer memory bind operation
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo<'l, 'h: 'l> {
  buffer: u64,
  bindCount: u32,
  pBinds: *const VkSparseMemoryBind<'h>,
  _p: ::std::marker::PhantomData<(&'h u8, &'l u8)>,
}
impl<'l, 'h: 'l> VkSparseBufferMemoryBindInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkSparseBufferMemoryBindInfo<'l, 'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_binds(mut self, value: &'l [VkSparseMemoryBind<'h>]) -> Self {
    self.bindCount = value.len() as u32;
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn bind_count(&self) -> u32 {
    self.bindCount
  }
}
impl<'l, 'h: 'l> Default for VkSparseBufferMemoryBindInfo<'l, 'h> {
  fn default() -> VkSparseBufferMemoryBindInfo<'l, 'h> {
    VkSparseBufferMemoryBindInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkSparseBufferMemoryBindInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_buffer_memory_bind_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkSparseBufferMemoryBindInfo);
}

/// Structure specifying sparse image opaque memory bind info
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo<'l, 'h: 'l> {
  image: u64,
  bindCount: u32,
  pBinds: *const VkSparseMemoryBind<'h>,
  _p: ::std::marker::PhantomData<(&'h u8, &'l u8)>,
}
impl<'l, 'h: 'l> VkSparseImageOpaqueMemoryBindInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkSparseImageOpaqueMemoryBindInfo<'l, 'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage<'h>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_binds(mut self, value: &'l [VkSparseMemoryBind<'h>]) -> Self {
    self.bindCount = value.len() as u32;
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn bind_count(&self) -> u32 {
    self.bindCount
  }
}
impl<'l, 'h: 'l> Default for VkSparseImageOpaqueMemoryBindInfo<'l, 'h> {
  fn default() -> VkSparseImageOpaqueMemoryBindInfo<'l, 'h> {
    VkSparseImageOpaqueMemoryBindInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkSparseImageOpaqueMemoryBindInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_opaque_memory_bind_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkSparseImageOpaqueMemoryBindInfo);
}

/// Structure specifying a image subresource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresource {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub arrayLayer: u32,
}
impl VkImageSubresource {
  #[inline]
  pub fn new() -> VkImageSubresource {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_aspect_mask(mut self, value: VkImageAspectFlags) -> Self {
    self.aspectMask = value;
    self
  }
  #[inline]
  pub fn set_mip_level(mut self, value: u32) -> Self {
    self.mipLevel = value;
    self
  }
  #[inline]
  pub fn set_array_layer(mut self, value: u32) -> Self {
    self.arrayLayer = value;
    self
  }
  #[inline]
  pub fn aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn mip_level(&self) -> u32 {
    self.mipLevel
  }
  #[inline]
  pub fn array_layer(&self) -> u32 {
    self.arrayLayer
  }
}
impl Default for VkImageSubresource {
  fn default() -> VkImageSubresource {
    VkImageSubresource::new()
  }
}
unsafe impl Struct for VkImageSubresource {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_subresource() {
  assert_size!(12, VkImageSubresource);
}

/// Structure specifying a three-dimensional offset
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset3D {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}
impl VkOffset3D {
  #[inline]
  pub fn new() -> VkOffset3D {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_x(mut self, value: i32) -> Self {
    self.x = value;
    self
  }
  #[inline]
  pub fn set_y(mut self, value: i32) -> Self {
    self.y = value;
    self
  }
  #[inline]
  pub fn set_z(mut self, value: i32) -> Self {
    self.z = value;
    self
  }
  #[inline]
  pub fn x(&self) -> i32 {
    self.x
  }
  #[inline]
  pub fn y(&self) -> i32 {
    self.y
  }
  #[inline]
  pub fn z(&self) -> i32 {
    self.z
  }
}
impl Default for VkOffset3D {
  fn default() -> VkOffset3D {
    VkOffset3D::new()
  }
}
unsafe impl Struct for VkOffset3D {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_offset3_d() {
  assert_size!(12, VkOffset3D);
}

/// Structure specifying sparse image memory bind
#[repr(C)]
pub struct VkSparseImageMemoryBind<'h> {
  pub subresource: VkImageSubresource,
  pub offset: VkOffset3D,
  pub extent: VkExtent3D,
  memory: u64,
  pub memoryOffset: VkDeviceSize,
  pub flags: VkSparseMemoryBindFlags,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
impl<'h> VkSparseImageMemoryBind<'h> {
  #[inline]
  pub fn new() -> VkSparseImageMemoryBind<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_subresource(mut self, value: VkImageSubresource) -> Self {
    self.subresource = value;
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: VkOffset3D) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_extent(mut self, value: VkExtent3D) -> Self {
    self.extent = value;
    self
  }
  #[inline]
  pub fn set_memory(mut self, value: Option<VkDeviceMemory<'h>>) -> Self {
    unsafe {
      self.memory = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_memory_offset(mut self, value: VkDeviceSize) -> Self {
    self.memoryOffset = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSparseMemoryBindFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn subresource(&self) -> &VkImageSubresource {
    &self.subresource
  }
  #[inline]
  pub fn offset(&self) -> &VkOffset3D {
    &self.offset
  }
  #[inline]
  pub fn extent(&self) -> &VkExtent3D {
    &self.extent
  }
  #[inline]
  pub fn memory_offset(&self) -> VkDeviceSize {
    self.memoryOffset
  }
  #[inline]
  pub fn flags(&self) -> VkSparseMemoryBindFlags {
    self.flags
  }
}
impl<'h> Default for VkSparseImageMemoryBind<'h> {
  fn default() -> VkSparseImageMemoryBind<'h> {
    VkSparseImageMemoryBind::new()
  }
}
unsafe impl<'h> Struct for VkSparseImageMemoryBind<'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_bind() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 2, VkSparseImageMemoryBind);
}

/// Structure specifying sparse image memory bind info
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo<'l, 'h: 'l> {
  image: u64,
  bindCount: u32,
  pBinds: *const VkSparseImageMemoryBind<'h>,
  _p: ::std::marker::PhantomData<(&'h u8, &'l u8)>,
}
impl<'l, 'h: 'l> VkSparseImageMemoryBindInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkSparseImageMemoryBindInfo<'l, 'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage<'h>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_binds(mut self, value: &'l [VkSparseImageMemoryBind<'h>]) -> Self {
    self.bindCount = value.len() as u32;
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn bind_count(&self) -> u32 {
    self.bindCount
  }
}
impl<'l, 'h: 'l> Default for VkSparseImageMemoryBindInfo<'l, 'h> {
  fn default() -> VkSparseImageMemoryBindInfo<'l, 'h> {
    VkSparseImageMemoryBindInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkSparseImageMemoryBindInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_bind_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkSparseImageMemoryBindInfo);
}

/// Structure specifying a sparse binding operation
#[repr(C)]
pub struct VkBindSparseInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreCount: u32,
  pWaitSemaphores: *const u64,
  bufferBindCount: u32,
  pBufferBinds: *const VkSparseBufferMemoryBindInfo<'l, 'h>,
  imageOpaqueBindCount: u32,
  pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo<'l, 'h>,
  imageBindCount: u32,
  pImageBinds: *const VkSparseImageMemoryBindInfo<'l, 'h>,
  signalSemaphoreCount: u32,
  pSignalSemaphores: *const u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkBindSparseInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkBindSparseInfo<'l, 'h> {
    unsafe {
      VkBindSparseInfo {
        sType: VkStructureType::BIND_SPARSE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_wait_semaphores(mut self, value: &'l [VkSemaphore<'h>]) -> Self {
    self.waitSemaphoreCount = value.len() as u32;
    unsafe {
      self.pWaitSemaphores = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_buffer_binds(mut self, value: &'l [VkSparseBufferMemoryBindInfo<'l, 'h>]) -> Self {
    self.bufferBindCount = value.len() as u32;
    unsafe {
      self.pBufferBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_opaque_binds(mut self, value: &'l [VkSparseImageOpaqueMemoryBindInfo<'l, 'h>]) -> Self {
    self.imageOpaqueBindCount = value.len() as u32;
    unsafe {
      self.pImageOpaqueBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_binds(mut self, value: &'l [VkSparseImageMemoryBindInfo<'l, 'h>]) -> Self {
    self.imageBindCount = value.len() as u32;
    unsafe {
      self.pImageBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphores(mut self, value: &'l [VkSemaphore<'h>]) -> Self {
    self.signalSemaphoreCount = value.len() as u32;
    unsafe {
      self.pSignalSemaphores = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn wait_semaphore_count(&self) -> u32 {
    self.waitSemaphoreCount
  }
  #[inline]
  pub fn buffer_bind_count(&self) -> u32 {
    self.bufferBindCount
  }
  #[inline]
  pub fn image_opaque_bind_count(&self) -> u32 {
    self.imageOpaqueBindCount
  }
  #[inline]
  pub fn image_bind_count(&self) -> u32 {
    self.imageBindCount
  }
  #[inline]
  pub fn signal_semaphore_count(&self) -> u32 {
    self.signalSemaphoreCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkBindSparseInfo<'l, 'h> {
  fn default() -> VkBindSparseInfo<'l, 'h> {
    VkBindSparseInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkBindSparseInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_sparse_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 12, VkBindSparseInfo);
}

/// Bitmask specifying initial state and behavior of a fence
pub use enums::VkFenceCreateFlagBits;

/// Bitmask of VkFenceCreateFlagBits
pub type VkFenceCreateFlags = VkFenceCreateFlagBits;

/// Structure specifying parameters of a newly created fence
#[repr(C)]
pub struct VkFenceCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkFenceCreateFlags,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkFenceCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkFenceCreateInfo<'l> {
    unsafe {
      VkFenceCreateInfo {
        sType: VkStructureType::FENCE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkFenceCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkFenceCreateFlags {
    self.flags
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkFenceCreateInfo<'l> {
  fn default() -> VkFenceCreateInfo<'l> {
    VkFenceCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkFenceCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkFenceCreateInfo);
}

/// Reserved for future use
pub type VkSemaphoreCreateFlags = VkFlags;

/// Structure specifying parameters of a newly created semaphore
#[repr(C)]
pub struct VkSemaphoreCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkSemaphoreCreateFlags,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkSemaphoreCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkSemaphoreCreateInfo<'l> {
    unsafe {
      VkSemaphoreCreateInfo {
        sType: VkStructureType::SEMAPHORE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSemaphoreCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkSemaphoreCreateFlags {
    self.flags
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkSemaphoreCreateInfo<'l> {
  fn default() -> VkSemaphoreCreateInfo<'l> {
    VkSemaphoreCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkSemaphoreCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkSemaphoreCreateInfo);
}

/// Reserved for future use
pub type VkEventCreateFlags = VkFlags;

/// Structure specifying parameters of a newly created event
#[repr(C)]
pub struct VkEventCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkEventCreateFlags,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkEventCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkEventCreateInfo<'l> {
    unsafe {
      VkEventCreateInfo {
        sType: VkStructureType::EVENT_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkEventCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkEventCreateFlags {
    self.flags
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkEventCreateInfo<'l> {
  fn default() -> VkEventCreateInfo<'l> {
    VkEventCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkEventCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_event_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkEventCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkEvent__ {}

/// Opaque handle to a event object
pub type VkEvent<'l> = VkNonDispatchableHandle<'l, VkEvent__>;

/// Reserved for future use
pub type VkQueryPoolCreateFlags = VkFlags;

/// Specify the type of queries managed by a query pool
pub use enums::VkQueryType;

/// Bitmask specifying queried pipeline statistics
pub use enums::VkQueryPipelineStatisticFlagBits;

/// Bitmask of VkQueryPipelineStatisticFlagBits
pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;

/// Structure specifying parameters of a newly created query pool
#[repr(C)]
pub struct VkQueryPoolCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkQueryPoolCreateFlags,
  pub queryType: VkQueryType,
  pub queryCount: u32,
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkQueryPoolCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkQueryPoolCreateInfo<'l> {
    unsafe {
      VkQueryPoolCreateInfo {
        sType: VkStructureType::QUERY_POOL_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkQueryPoolCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_query_type(mut self, value: VkQueryType) -> Self {
    self.queryType = value;
    self
  }
  #[inline]
  pub fn set_query_count(mut self, value: u32) -> Self {
    self.queryCount = value;
    self
  }
  #[inline]
  pub fn set_pipeline_statistics(mut self, value: VkQueryPipelineStatisticFlags) -> Self {
    self.pipelineStatistics = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkQueryPoolCreateFlags {
    self.flags
  }
  #[inline]
  pub fn query_type(&self) -> VkQueryType {
    self.queryType
  }
  #[inline]
  pub fn query_count(&self) -> u32 {
    self.queryCount
  }
  #[inline]
  pub fn pipeline_statistics(&self) -> VkQueryPipelineStatisticFlags {
    self.pipelineStatistics
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkQueryPoolCreateInfo<'l> {
  fn default() -> VkQueryPoolCreateInfo<'l> {
    VkQueryPoolCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkQueryPoolCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_query_pool_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 2, VkQueryPoolCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkQueryPool__ {}

/// Opaque handle to a query pool object
pub type VkQueryPool<'l> = VkNonDispatchableHandle<'l, VkQueryPool__>;

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

/// Structure specifying the parameters of a newly created buffer object
#[repr(C)]
pub struct VkBufferCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkBufferCreateFlags,
  pub size: VkDeviceSize,
  pub usage: VkBufferUsageFlags,
  pub sharingMode: VkSharingMode,
  queueFamilyIndexCount: u32,
  pQueueFamilyIndices: *const u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkBufferCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkBufferCreateInfo<'l> {
    unsafe {
      VkBufferCreateInfo {
        sType: VkStructureType::BUFFER_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkBufferCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_size(mut self, value: VkDeviceSize) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn set_usage(mut self, value: VkBufferUsageFlags) -> Self {
    self.usage = value;
    self
  }
  #[inline]
  pub fn set_sharing_mode(mut self, value: VkSharingMode) -> Self {
    self.sharingMode = value;
    self
  }
  #[inline]
  pub fn set_queue_family_indices(mut self, value: &'l [u32]) -> Self {
    self.queueFamilyIndexCount = value.len() as u32;
    unsafe {
      self.pQueueFamilyIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkBufferCreateFlags {
    self.flags
  }
  #[inline]
  pub fn size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn usage(&self) -> VkBufferUsageFlags {
    self.usage
  }
  #[inline]
  pub fn sharing_mode(&self) -> VkSharingMode {
    self.sharingMode
  }
  #[inline]
  pub fn queue_family_index_count(&self) -> u32 {
    self.queueFamilyIndexCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkBufferCreateInfo<'l> {
  fn default() -> VkBufferCreateInfo<'l> {
    VkBufferCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkBufferCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 5, VkBufferCreateInfo);
}

/// Reserved for future use
pub type VkBufferViewCreateFlags = VkFlags;

/// Structure specifying parameters of a newly created buffer view
#[repr(C)]
pub struct VkBufferViewCreateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkBufferViewCreateFlags,
  buffer: u64,
  pub format: VkFormat,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkBufferViewCreateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkBufferViewCreateInfo<'l, 'h> {
    unsafe {
      VkBufferViewCreateInfo {
        sType: VkStructureType::BUFFER_VIEW_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkBufferViewCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: VkDeviceSize) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_range(mut self, value: VkDeviceSize) -> Self {
    self.range = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkBufferViewCreateFlags {
    self.flags
  }
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn range(&self) -> VkDeviceSize {
    self.range
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkBufferViewCreateInfo<'l, 'h> {
  fn default() -> VkBufferViewCreateInfo<'l, 'h> {
    VkBufferViewCreateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkBufferViewCreateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_view_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 4, VkBufferViewCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkBufferView__ {}

/// Opaque handle to a buffer view object
pub type VkBufferView<'l> = VkNonDispatchableHandle<'l, VkBufferView__>;

/// Structure specifying the parameters of a newly created image object
#[repr(C)]
pub struct VkImageCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
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
  queueFamilyIndexCount: u32,
  pQueueFamilyIndices: *const u32,
  pub initialLayout: VkImageLayout,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkImageCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkImageCreateInfo<'l> {
    unsafe {
      VkImageCreateInfo {
        sType: VkStructureType::IMAGE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkImageCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_image_type(mut self, value: VkImageType) -> Self {
    self.imageType = value;
    self
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_extent(mut self, value: VkExtent3D) -> Self {
    self.extent = value;
    self
  }
  #[inline]
  pub fn set_mip_levels(mut self, value: u32) -> Self {
    self.mipLevels = value;
    self
  }
  #[inline]
  pub fn set_array_layers(mut self, value: u32) -> Self {
    self.arrayLayers = value;
    self
  }
  #[inline]
  pub fn set_samples(mut self, value: VkSampleCountFlagBits) -> Self {
    self.samples = value;
    self
  }
  #[inline]
  pub fn set_tiling(mut self, value: VkImageTiling) -> Self {
    self.tiling = value;
    self
  }
  #[inline]
  pub fn set_usage(mut self, value: VkImageUsageFlags) -> Self {
    self.usage = value;
    self
  }
  #[inline]
  pub fn set_sharing_mode(mut self, value: VkSharingMode) -> Self {
    self.sharingMode = value;
    self
  }
  #[inline]
  pub fn set_queue_family_indices(mut self, value: &'l [u32]) -> Self {
    self.queueFamilyIndexCount = value.len() as u32;
    unsafe {
      self.pQueueFamilyIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_initial_layout(mut self, value: VkImageLayout) -> Self {
    self.initialLayout = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkImageCreateFlags {
    self.flags
  }
  #[inline]
  pub fn image_type(&self) -> VkImageType {
    self.imageType
  }
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn extent(&self) -> &VkExtent3D {
    &self.extent
  }
  #[inline]
  pub fn mip_levels(&self) -> u32 {
    self.mipLevels
  }
  #[inline]
  pub fn array_layers(&self) -> u32 {
    self.arrayLayers
  }
  #[inline]
  pub fn samples(&self) -> VkSampleCountFlagBits {
    self.samples
  }
  #[inline]
  pub fn tiling(&self) -> VkImageTiling {
    self.tiling
  }
  #[inline]
  pub fn usage(&self) -> VkImageUsageFlags {
    self.usage
  }
  #[inline]
  pub fn sharing_mode(&self) -> VkSharingMode {
    self.sharingMode
  }
  #[inline]
  pub fn queue_family_index_count(&self) -> u32 {
    self.queueFamilyIndexCount
  }
  #[inline]
  pub fn initial_layout(&self) -> VkImageLayout {
    self.initialLayout
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkImageCreateInfo<'l> {
  fn default() -> VkImageCreateInfo<'l> {
    VkImageCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkImageCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 5, VkImageCreateInfo);
}

/// Structure specifying subresource layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubresourceLayout {
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub rowPitch: VkDeviceSize,
  pub arrayPitch: VkDeviceSize,
  pub depthPitch: VkDeviceSize,
}
impl VkSubresourceLayout {
  #[inline]
  pub fn offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn row_pitch(&self) -> VkDeviceSize {
    self.rowPitch
  }
  #[inline]
  pub fn array_pitch(&self) -> VkDeviceSize {
    self.arrayPitch
  }
  #[inline]
  pub fn depth_pitch(&self) -> VkDeviceSize {
    self.depthPitch
  }
}
unsafe impl Struct for VkSubresourceLayout {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subresource_layout() {
  assert_size!(40, VkSubresourceLayout);
}

/// Reserved for future use
pub type VkImageViewCreateFlags = VkFlags;

/// Image view types
pub use enums::VkImageViewType;

/// Specify how a component is swizzled
pub use enums::VkComponentSwizzle;

/// Structure specifying a color component mapping
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComponentMapping {
  pub r: VkComponentSwizzle,
  pub g: VkComponentSwizzle,
  pub b: VkComponentSwizzle,
  pub a: VkComponentSwizzle,
}
impl VkComponentMapping {
  #[inline]
  pub fn new() -> VkComponentMapping {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_r(mut self, value: VkComponentSwizzle) -> Self {
    self.r = value;
    self
  }
  #[inline]
  pub fn set_g(mut self, value: VkComponentSwizzle) -> Self {
    self.g = value;
    self
  }
  #[inline]
  pub fn set_b(mut self, value: VkComponentSwizzle) -> Self {
    self.b = value;
    self
  }
  #[inline]
  pub fn set_a(mut self, value: VkComponentSwizzle) -> Self {
    self.a = value;
    self
  }
  #[inline]
  pub fn r(&self) -> VkComponentSwizzle {
    self.r
  }
  #[inline]
  pub fn g(&self) -> VkComponentSwizzle {
    self.g
  }
  #[inline]
  pub fn b(&self) -> VkComponentSwizzle {
    self.b
  }
  #[inline]
  pub fn a(&self) -> VkComponentSwizzle {
    self.a
  }
}
impl Default for VkComponentMapping {
  fn default() -> VkComponentMapping {
    VkComponentMapping::new()
  }
}
unsafe impl Struct for VkComponentMapping {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_component_mapping() {
  assert_size!(16, VkComponentMapping);
}

/// Structure specifying parameters of a newly created image view
#[repr(C)]
pub struct VkImageViewCreateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkImageViewCreateFlags,
  image: u64,
  pub viewType: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresourceRange: VkImageSubresourceRange,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkImageViewCreateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkImageViewCreateInfo<'l, 'h> {
    unsafe {
      VkImageViewCreateInfo {
        sType: VkStructureType::IMAGE_VIEW_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkImageViewCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage<'h>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_view_type(mut self, value: VkImageViewType) -> Self {
    self.viewType = value;
    self
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_components(mut self, value: VkComponentMapping) -> Self {
    self.components = value;
    self
  }
  #[inline]
  pub fn set_subresource_range(mut self, value: VkImageSubresourceRange) -> Self {
    self.subresourceRange = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkImageViewCreateFlags {
    self.flags
  }
  #[inline]
  pub fn view_type(&self) -> VkImageViewType {
    self.viewType
  }
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn components(&self) -> &VkComponentMapping {
    &self.components
  }
  #[inline]
  pub fn subresource_range(&self) -> &VkImageSubresourceRange {
    &self.subresourceRange
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkImageViewCreateInfo<'l, 'h> {
  fn default() -> VkImageViewCreateInfo<'l, 'h> {
    VkImageViewCreateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkImageViewCreateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_view_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 4, VkImageViewCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkImageView__ {}

/// Opaque handle to a image view object
pub type VkImageView<'l> = VkNonDispatchableHandle<'l, VkImageView__>;

/// Reserved for future use
pub type VkShaderModuleCreateFlags = VkFlags;

/// Structure specifying parameters of a newly created shader module
#[repr(C)]
pub struct VkShaderModuleCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkShaderModuleCreateFlags,
  pub codeSize: usize,
  pCode: *const u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkShaderModuleCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkShaderModuleCreateInfo<'l> {
    unsafe {
      VkShaderModuleCreateInfo {
        sType: VkStructureType::SHADER_MODULE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkShaderModuleCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_code_size(mut self, value: usize) -> Self {
    self.codeSize = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkShaderModuleCreateFlags {
    self.flags
  }
  #[inline]
  pub fn code_size(&self) -> usize {
    self.codeSize
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkShaderModuleCreateInfo<'l> {
  fn default() -> VkShaderModuleCreateInfo<'l> {
    VkShaderModuleCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkShaderModuleCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_shader_module_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkShaderModuleCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkShaderModule__ {}

/// Opaque handle to a shader module object
pub type VkShaderModule<'l> = VkNonDispatchableHandle<'l, VkShaderModule__>;

/// Reserved for future use
pub type VkPipelineCacheCreateFlags = VkFlags;

/// Structure specifying parameters of a newly created pipeline cache
#[repr(C)]
pub struct VkPipelineCacheCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCacheCreateFlags,
  initialDataSize: usize,
  pInitialData: *const c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineCacheCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineCacheCreateInfo<'l> {
    unsafe {
      VkPipelineCacheCreateInfo {
        sType: VkStructureType::PIPELINE_CACHE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineCacheCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_initial_data(mut self, value: &'l [u8]) -> Self {
    self.initialDataSize = value.len() as usize;
    unsafe {
      self.pInitialData = value.as_raw() as *const c_void;
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineCacheCreateFlags {
    self.flags
  }
  #[inline]
  pub fn initial_data_size(&self) -> usize {
    self.initialDataSize
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineCacheCreateInfo<'l> {
  fn default() -> VkPipelineCacheCreateInfo<'l> {
    VkPipelineCacheCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineCacheCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_cache_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkPipelineCacheCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPipelineCache__ {}

/// Opaque handle to a pipeline cache object
pub type VkPipelineCache<'l> = VkNonDispatchableHandle<'l, VkPipelineCache__>;

/// Bitmask controlling how a pipeline is created
pub use enums::VkPipelineCreateFlagBits;

/// Bitmask of VkPipelineCreateFlagBits
pub type VkPipelineCreateFlags = VkPipelineCreateFlagBits;

/// Reserved for future use
pub type VkPipelineShaderStageCreateFlags = VkFlags;

/// Bitmask specifying a pipeline stage
pub use enums::VkShaderStageFlagBits;

/// Structure specifying a specialization map entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSpecializationMapEntry {
  pub constantID: u32,
  pub offset: u32,
  pub size: usize,
}
impl VkSpecializationMapEntry {
  #[inline]
  pub fn new() -> VkSpecializationMapEntry {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_constant_id(mut self, value: u32) -> Self {
    self.constantID = value;
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: u32) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_size(mut self, value: usize) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn constant_id(&self) -> u32 {
    self.constantID
  }
  #[inline]
  pub fn offset(&self) -> u32 {
    self.offset
  }
  #[inline]
  pub fn size(&self) -> usize {
    self.size
  }
}
impl Default for VkSpecializationMapEntry {
  fn default() -> VkSpecializationMapEntry {
    VkSpecializationMapEntry::new()
  }
}
unsafe impl Struct for VkSpecializationMapEntry {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_specialization_map_entry() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 1, VkSpecializationMapEntry);
}

/// Structure specifying specialization info
#[repr(C)]
pub struct VkSpecializationInfo<'l> {
  mapEntryCount: u32,
  pMapEntries: *const VkSpecializationMapEntry,
  dataSize: usize,
  pData: *const c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkSpecializationInfo<'l> {
  #[inline]
  pub fn new() -> VkSpecializationInfo<'l> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_map_entries(mut self, value: &'l [VkSpecializationMapEntry]) -> Self {
    self.mapEntryCount = value.len() as u32;
    unsafe {
      self.pMapEntries = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_data(mut self, value: &'l [u8]) -> Self {
    self.dataSize = value.len() as usize;
    unsafe {
      self.pData = value.as_raw() as *const c_void;
    }
    self
  }
  #[inline]
  pub fn map_entry_count(&self) -> u32 {
    self.mapEntryCount
  }
  #[inline]
  pub fn data_size(&self) -> usize {
    self.dataSize
  }
}
impl<'l> Default for VkSpecializationInfo<'l> {
  fn default() -> VkSpecializationInfo<'l> {
    VkSpecializationInfo::new()
  }
}
unsafe impl<'l> Struct for VkSpecializationInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_specialization_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkSpecializationInfo);
}

/// Structure specifying parameters of a newly created pipeline shader stage
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineShaderStageCreateFlags,
  pub stage: VkShaderStageFlagBits,
  module: u64,
  pName: *const c_char,
  pSpecializationInfo: *const VkSpecializationInfo<'l>,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkPipelineShaderStageCreateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkPipelineShaderStageCreateInfo<'l, 'h> {
    unsafe {
      VkPipelineShaderStageCreateInfo {
        sType: VkStructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineShaderStageCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_stage(mut self, value: VkShaderStageFlagBits) -> Self {
    self.stage = value;
    self
  }
  #[inline]
  pub fn set_module(mut self, value: VkShaderModule<'h>) -> Self {
    unsafe {
      self.module = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_name(mut self, value: &'l AsRef<CStr>) -> Self {
    unsafe {
      self.pName = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_specialization_info(mut self, value: Option<&'l VkSpecializationInfo<'l>>) -> Self {
    unsafe {
      self.pSpecializationInfo = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineShaderStageCreateFlags {
    self.flags
  }
  #[inline]
  pub fn stage(&self) -> VkShaderStageFlagBits {
    self.stage
  }
  #[inline]
  pub fn name(&self) -> &CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pName) }
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkPipelineShaderStageCreateInfo<'l, 'h> {
  fn default() -> VkPipelineShaderStageCreateInfo<'l, 'h> {
    VkPipelineShaderStageCreateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkPipelineShaderStageCreateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_shader_stage_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 4, VkPipelineShaderStageCreateInfo);
}

/// Reserved for future use
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;

/// Specify rate at which vertex attributes are pulled from buffers
pub use enums::VkVertexInputRate;

/// Structure specifying vertex input binding description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputBindingDescription {
  pub binding: u32,
  pub stride: u32,
  pub inputRate: VkVertexInputRate,
}
impl VkVertexInputBindingDescription {
  #[inline]
  pub fn new() -> VkVertexInputBindingDescription {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_binding(mut self, value: u32) -> Self {
    self.binding = value;
    self
  }
  #[inline]
  pub fn set_stride(mut self, value: u32) -> Self {
    self.stride = value;
    self
  }
  #[inline]
  pub fn set_input_rate(mut self, value: VkVertexInputRate) -> Self {
    self.inputRate = value;
    self
  }
  #[inline]
  pub fn binding(&self) -> u32 {
    self.binding
  }
  #[inline]
  pub fn stride(&self) -> u32 {
    self.stride
  }
  #[inline]
  pub fn input_rate(&self) -> VkVertexInputRate {
    self.inputRate
  }
}
impl Default for VkVertexInputBindingDescription {
  fn default() -> VkVertexInputBindingDescription {
    VkVertexInputBindingDescription::new()
  }
}
unsafe impl Struct for VkVertexInputBindingDescription {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_vertex_input_binding_description() {
  assert_size!(12, VkVertexInputBindingDescription);
}

/// Structure specifying vertex input attribute description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputAttributeDescription {
  pub location: u32,
  pub binding: u32,
  pub format: VkFormat,
  pub offset: u32,
}
impl VkVertexInputAttributeDescription {
  #[inline]
  pub fn new() -> VkVertexInputAttributeDescription {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_location(mut self, value: u32) -> Self {
    self.location = value;
    self
  }
  #[inline]
  pub fn set_binding(mut self, value: u32) -> Self {
    self.binding = value;
    self
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: u32) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn location(&self) -> u32 {
    self.location
  }
  #[inline]
  pub fn binding(&self) -> u32 {
    self.binding
  }
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn offset(&self) -> u32 {
    self.offset
  }
}
impl Default for VkVertexInputAttributeDescription {
  fn default() -> VkVertexInputAttributeDescription {
    VkVertexInputAttributeDescription::new()
  }
}
unsafe impl Struct for VkVertexInputAttributeDescription {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_vertex_input_attribute_description() {
  assert_size!(16, VkVertexInputAttributeDescription);
}

/// Structure specifying parameters of a newly created pipeline vertex input state
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineVertexInputStateCreateFlags,
  vertexBindingDescriptionCount: u32,
  pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
  vertexAttributeDescriptionCount: u32,
  pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineVertexInputStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineVertexInputStateCreateInfo<'l> {
    unsafe {
      VkPipelineVertexInputStateCreateInfo {
        sType: VkStructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineVertexInputStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_vertex_binding_descriptions(mut self, value: &'l [VkVertexInputBindingDescription]) -> Self {
    self.vertexBindingDescriptionCount = value.len() as u32;
    unsafe {
      self.pVertexBindingDescriptions = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_vertex_attribute_descriptions(mut self, value: &'l [VkVertexInputAttributeDescription]) -> Self {
    self.vertexAttributeDescriptionCount = value.len() as u32;
    unsafe {
      self.pVertexAttributeDescriptions = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineVertexInputStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn vertex_binding_description_count(&self) -> u32 {
    self.vertexBindingDescriptionCount
  }
  #[inline]
  pub fn vertex_attribute_description_count(&self) -> u32 {
    self.vertexAttributeDescriptionCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineVertexInputStateCreateInfo<'l> {
  fn default() -> VkPipelineVertexInputStateCreateInfo<'l> {
    VkPipelineVertexInputStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineVertexInputStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_vertex_input_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 5, VkPipelineVertexInputStateCreateInfo);
}

/// Reserved for future use
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;

/// Supported primitive topologies
pub use enums::VkPrimitiveTopology;

/// Structure specifying parameters of a newly created pipeline input assembly state
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  primitiveRestartEnable: VkBool32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineInputAssemblyStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineInputAssemblyStateCreateInfo<'l> {
    unsafe {
      VkPipelineInputAssemblyStateCreateInfo {
        sType: VkStructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineInputAssemblyStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_topology(mut self, value: VkPrimitiveTopology) -> Self {
    self.topology = value;
    self
  }
  #[inline]
  pub fn set_primitive_restart_enable(mut self, value: bool) -> Self {
    unsafe {
      self.primitiveRestartEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineInputAssemblyStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn topology(&self) -> VkPrimitiveTopology {
    self.topology
  }
  #[inline]
  pub fn is_primitive_restart_enable(&self) -> bool {
    self.primitiveRestartEnable != 0
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineInputAssemblyStateCreateInfo<'l> {
  fn default() -> VkPipelineInputAssemblyStateCreateInfo<'l> {
    VkPipelineInputAssemblyStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineInputAssemblyStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_input_assembly_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPipelineInputAssemblyStateCreateInfo);
}

/// Reserved for future use
pub type VkPipelineTessellationStateCreateFlags = VkFlags;

/// Structure specifying parameters of a newly created pipeline tessellation state
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patchControlPoints: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineTessellationStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineTessellationStateCreateInfo<'l> {
    unsafe {
      VkPipelineTessellationStateCreateInfo {
        sType: VkStructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineTessellationStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_patch_control_points(mut self, value: u32) -> Self {
    self.patchControlPoints = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineTessellationStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn patch_control_points(&self) -> u32 {
    self.patchControlPoints
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineTessellationStateCreateInfo<'l> {
  fn default() -> VkPipelineTessellationStateCreateInfo<'l> {
    VkPipelineTessellationStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineTessellationStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_tessellation_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkPipelineTessellationStateCreateInfo);
}

/// Reserved for future use
pub type VkPipelineViewportStateCreateFlags = VkFlags;

/// Structure specifying a viewport
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
impl VkViewport {
  #[inline]
  pub fn new() -> VkViewport {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_x(mut self, value: f32) -> Self {
    self.x = value;
    self
  }
  #[inline]
  pub fn set_y(mut self, value: f32) -> Self {
    self.y = value;
    self
  }
  #[inline]
  pub fn set_width(mut self, value: f32) -> Self {
    self.width = value;
    self
  }
  #[inline]
  pub fn set_height(mut self, value: f32) -> Self {
    self.height = value;
    self
  }
  #[inline]
  pub fn set_min_depth(mut self, value: f32) -> Self {
    self.minDepth = value;
    self
  }
  #[inline]
  pub fn set_max_depth(mut self, value: f32) -> Self {
    self.maxDepth = value;
    self
  }
  #[inline]
  pub fn x(&self) -> f32 {
    self.x
  }
  #[inline]
  pub fn y(&self) -> f32 {
    self.y
  }
  #[inline]
  pub fn width(&self) -> f32 {
    self.width
  }
  #[inline]
  pub fn height(&self) -> f32 {
    self.height
  }
  #[inline]
  pub fn min_depth(&self) -> f32 {
    self.minDepth
  }
  #[inline]
  pub fn max_depth(&self) -> f32 {
    self.maxDepth
  }
}
impl Default for VkViewport {
  fn default() -> VkViewport {
    VkViewport::new()
  }
}
unsafe impl Struct for VkViewport {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_viewport() {
  assert_size!(24, VkViewport);
}

/// Structure specifying a two-dimensional offset
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset2D {
  pub x: i32,
  pub y: i32,
}
impl VkOffset2D {
  #[inline]
  pub fn new() -> VkOffset2D {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_x(mut self, value: i32) -> Self {
    self.x = value;
    self
  }
  #[inline]
  pub fn set_y(mut self, value: i32) -> Self {
    self.y = value;
    self
  }
  #[inline]
  pub fn x(&self) -> i32 {
    self.x
  }
  #[inline]
  pub fn y(&self) -> i32 {
    self.y
  }
}
impl Default for VkOffset2D {
  fn default() -> VkOffset2D {
    VkOffset2D::new()
  }
}
unsafe impl Struct for VkOffset2D {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_offset2_d() {
  assert_size!(8, VkOffset2D);
}

/// Structure specifying a two-dimensional extent
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent2D {
  pub width: u32,
  pub height: u32,
}
impl VkExtent2D {
  #[inline]
  pub fn new() -> VkExtent2D {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_width(mut self, value: u32) -> Self {
    self.width = value;
    self
  }
  #[inline]
  pub fn set_height(mut self, value: u32) -> Self {
    self.height = value;
    self
  }
  #[inline]
  pub fn width(&self) -> u32 {
    self.width
  }
  #[inline]
  pub fn height(&self) -> u32 {
    self.height
  }
}
impl Default for VkExtent2D {
  fn default() -> VkExtent2D {
    VkExtent2D::new()
  }
}
unsafe impl Struct for VkExtent2D {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extent2_d() {
  assert_size!(8, VkExtent2D);
}

/// Structure specifying a two-dimensional subregion
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}
impl VkRect2D {
  #[inline]
  pub fn new() -> VkRect2D {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_offset(mut self, value: VkOffset2D) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_extent(mut self, value: VkExtent2D) -> Self {
    self.extent = value;
    self
  }
  #[inline]
  pub fn offset(&self) -> &VkOffset2D {
    &self.offset
  }
  #[inline]
  pub fn extent(&self) -> &VkExtent2D {
    &self.extent
  }
}
impl Default for VkRect2D {
  fn default() -> VkRect2D {
    VkRect2D::new()
  }
}
unsafe impl Struct for VkRect2D {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_rect2_d() {
  assert_size!(16, VkRect2D);
}

/// Structure specifying parameters of a newly created pipeline viewport state
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineViewportStateCreateFlags,
  viewportCount: u32,
  pViewports: *const VkViewport,
  scissorCount: u32,
  pScissors: *const VkRect2D,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineViewportStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineViewportStateCreateInfo<'l> {
    unsafe {
      VkPipelineViewportStateCreateInfo {
        sType: VkStructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineViewportStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_viewports(mut self, value: &'l [VkViewport]) -> Self {
    self.viewportCount = value.len() as u32;
    unsafe {
      self.pViewports = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_scissors(mut self, value: &'l [VkRect2D]) -> Self {
    self.scissorCount = value.len() as u32;
    unsafe {
      self.pScissors = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineViewportStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn viewport_count(&self) -> u32 {
    self.viewportCount
  }
  #[inline]
  pub fn scissor_count(&self) -> u32 {
    self.scissorCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineViewportStateCreateInfo<'l> {
  fn default() -> VkPipelineViewportStateCreateInfo<'l> {
    VkPipelineViewportStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineViewportStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_viewport_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 5, VkPipelineViewportStateCreateInfo);
}

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

/// Structure specifying parameters of a newly created pipeline rasterization state
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineRasterizationStateCreateFlags,
  depthClampEnable: VkBool32,
  rasterizerDiscardEnable: VkBool32,
  pub polygonMode: VkPolygonMode,
  pub cullMode: VkCullModeFlags,
  pub frontFace: VkFrontFace,
  depthBiasEnable: VkBool32,
  pub depthBiasConstantFactor: f32,
  pub depthBiasClamp: f32,
  pub depthBiasSlopeFactor: f32,
  pub lineWidth: f32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineRasterizationStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineRasterizationStateCreateInfo<'l> {
    unsafe {
      VkPipelineRasterizationStateCreateInfo {
        sType: VkStructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineRasterizationStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_depth_clamp_enable(mut self, value: bool) -> Self {
    unsafe {
      self.depthClampEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_rasterizer_discard_enable(mut self, value: bool) -> Self {
    unsafe {
      self.rasterizerDiscardEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_polygon_mode(mut self, value: VkPolygonMode) -> Self {
    self.polygonMode = value;
    self
  }
  #[inline]
  pub fn set_cull_mode(mut self, value: VkCullModeFlags) -> Self {
    self.cullMode = value;
    self
  }
  #[inline]
  pub fn set_front_face(mut self, value: VkFrontFace) -> Self {
    self.frontFace = value;
    self
  }
  #[inline]
  pub fn set_depth_bias_enable(mut self, value: bool) -> Self {
    unsafe {
      self.depthBiasEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_bias_constant_factor(mut self, value: f32) -> Self {
    self.depthBiasConstantFactor = value;
    self
  }
  #[inline]
  pub fn set_depth_bias_clamp(mut self, value: f32) -> Self {
    self.depthBiasClamp = value;
    self
  }
  #[inline]
  pub fn set_depth_bias_slope_factor(mut self, value: f32) -> Self {
    self.depthBiasSlopeFactor = value;
    self
  }
  #[inline]
  pub fn set_line_width(mut self, value: f32) -> Self {
    self.lineWidth = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineRasterizationStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn is_depth_clamp_enable(&self) -> bool {
    self.depthClampEnable != 0
  }
  #[inline]
  pub fn is_rasterizer_discard_enable(&self) -> bool {
    self.rasterizerDiscardEnable != 0
  }
  #[inline]
  pub fn polygon_mode(&self) -> VkPolygonMode {
    self.polygonMode
  }
  #[inline]
  pub fn cull_mode(&self) -> VkCullModeFlags {
    self.cullMode
  }
  #[inline]
  pub fn front_face(&self) -> VkFrontFace {
    self.frontFace
  }
  #[inline]
  pub fn is_depth_bias_enable(&self) -> bool {
    self.depthBiasEnable != 0
  }
  #[inline]
  pub fn depth_bias_constant_factor(&self) -> f32 {
    self.depthBiasConstantFactor
  }
  #[inline]
  pub fn depth_bias_clamp(&self) -> f32 {
    self.depthBiasClamp
  }
  #[inline]
  pub fn depth_bias_slope_factor(&self) -> f32 {
    self.depthBiasSlopeFactor
  }
  #[inline]
  pub fn line_width(&self) -> f32 {
    self.lineWidth
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineRasterizationStateCreateInfo<'l> {
  fn default() -> VkPipelineRasterizationStateCreateInfo<'l> {
    VkPipelineRasterizationStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineRasterizationStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_rasterization_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(40 + ptr_size * 3, VkPipelineRasterizationStateCreateInfo);
}

/// Reserved for future use
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;

/// Mask of sample coverage information
pub type VkSampleMask = u32;

/// Structure specifying parameters of a newly created pipeline multisample state
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineMultisampleStateCreateFlags,
  pub rasterizationSamples: VkSampleCountFlagBits,
  sampleShadingEnable: VkBool32,
  pub minSampleShading: f32,
  pSampleMask: *const VkSampleMask,
  alphaToCoverageEnable: VkBool32,
  alphaToOneEnable: VkBool32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineMultisampleStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineMultisampleStateCreateInfo<'l> {
    unsafe {
      VkPipelineMultisampleStateCreateInfo {
        sType: VkStructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineMultisampleStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_rasterization_samples(mut self, value: VkSampleCountFlagBits) -> Self {
    self.rasterizationSamples = value;
    self
  }
  #[inline]
  pub fn set_sample_shading_enable(mut self, value: bool) -> Self {
    unsafe {
      self.sampleShadingEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_min_sample_shading(mut self, value: f32) -> Self {
    self.minSampleShading = value;
    self
  }
  #[inline]
  pub fn set_alpha_to_coverage_enable(mut self, value: bool) -> Self {
    unsafe {
      self.alphaToCoverageEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_alpha_to_one_enable(mut self, value: bool) -> Self {
    unsafe {
      self.alphaToOneEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineMultisampleStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn rasterization_samples(&self) -> VkSampleCountFlagBits {
    self.rasterizationSamples
  }
  #[inline]
  pub fn is_sample_shading_enable(&self) -> bool {
    self.sampleShadingEnable != 0
  }
  #[inline]
  pub fn min_sample_shading(&self) -> f32 {
    self.minSampleShading
  }
  #[inline]
  pub fn is_alpha_to_coverage_enable(&self) -> bool {
    self.alphaToCoverageEnable != 0
  }
  #[inline]
  pub fn is_alpha_to_one_enable(&self) -> bool {
    self.alphaToOneEnable != 0
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineMultisampleStateCreateInfo<'l> {
  fn default() -> VkPipelineMultisampleStateCreateInfo<'l> {
    VkPipelineMultisampleStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineMultisampleStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_multisample_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 3, VkPipelineMultisampleStateCreateInfo);
}

/// Reserved for future use
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;

/// Stencil comparison function
pub use enums::VkCompareOp;

/// Stencil comparison function
pub use enums::VkStencilOp;

/// Structure specifying stencil operation state
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
impl VkStencilOpState {
  #[inline]
  pub fn new() -> VkStencilOpState {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_fail_op(mut self, value: VkStencilOp) -> Self {
    self.failOp = value;
    self
  }
  #[inline]
  pub fn set_pass_op(mut self, value: VkStencilOp) -> Self {
    self.passOp = value;
    self
  }
  #[inline]
  pub fn set_depth_fail_op(mut self, value: VkStencilOp) -> Self {
    self.depthFailOp = value;
    self
  }
  #[inline]
  pub fn set_compare_op(mut self, value: VkCompareOp) -> Self {
    self.compareOp = value;
    self
  }
  #[inline]
  pub fn set_compare_mask(mut self, value: u32) -> Self {
    self.compareMask = value;
    self
  }
  #[inline]
  pub fn set_write_mask(mut self, value: u32) -> Self {
    self.writeMask = value;
    self
  }
  #[inline]
  pub fn set_reference(mut self, value: u32) -> Self {
    self.reference = value;
    self
  }
  #[inline]
  pub fn fail_op(&self) -> VkStencilOp {
    self.failOp
  }
  #[inline]
  pub fn pass_op(&self) -> VkStencilOp {
    self.passOp
  }
  #[inline]
  pub fn depth_fail_op(&self) -> VkStencilOp {
    self.depthFailOp
  }
  #[inline]
  pub fn compare_op(&self) -> VkCompareOp {
    self.compareOp
  }
  #[inline]
  pub fn compare_mask(&self) -> u32 {
    self.compareMask
  }
  #[inline]
  pub fn write_mask(&self) -> u32 {
    self.writeMask
  }
  #[inline]
  pub fn reference(&self) -> u32 {
    self.reference
  }
}
impl Default for VkStencilOpState {
  fn default() -> VkStencilOpState {
    VkStencilOpState::new()
  }
}
unsafe impl Struct for VkStencilOpState {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_stencil_op_state() {
  assert_size!(28, VkStencilOpState);
}

/// Structure specifying parameters of a newly created pipeline depth stencil state
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineDepthStencilStateCreateFlags,
  depthTestEnable: VkBool32,
  depthWriteEnable: VkBool32,
  pub depthCompareOp: VkCompareOp,
  depthBoundsTestEnable: VkBool32,
  stencilTestEnable: VkBool32,
  pub front: VkStencilOpState,
  pub back: VkStencilOpState,
  pub minDepthBounds: f32,
  pub maxDepthBounds: f32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineDepthStencilStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineDepthStencilStateCreateInfo<'l> {
    unsafe {
      VkPipelineDepthStencilStateCreateInfo {
        sType: VkStructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineDepthStencilStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_depth_test_enable(mut self, value: bool) -> Self {
    unsafe {
      self.depthTestEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_write_enable(mut self, value: bool) -> Self {
    unsafe {
      self.depthWriteEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_compare_op(mut self, value: VkCompareOp) -> Self {
    self.depthCompareOp = value;
    self
  }
  #[inline]
  pub fn set_depth_bounds_test_enable(mut self, value: bool) -> Self {
    unsafe {
      self.depthBoundsTestEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_stencil_test_enable(mut self, value: bool) -> Self {
    unsafe {
      self.stencilTestEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_front(mut self, value: VkStencilOpState) -> Self {
    self.front = value;
    self
  }
  #[inline]
  pub fn set_back(mut self, value: VkStencilOpState) -> Self {
    self.back = value;
    self
  }
  #[inline]
  pub fn set_min_depth_bounds(mut self, value: f32) -> Self {
    self.minDepthBounds = value;
    self
  }
  #[inline]
  pub fn set_max_depth_bounds(mut self, value: f32) -> Self {
    self.maxDepthBounds = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineDepthStencilStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn is_depth_test_enable(&self) -> bool {
    self.depthTestEnable != 0
  }
  #[inline]
  pub fn is_depth_write_enable(&self) -> bool {
    self.depthWriteEnable != 0
  }
  #[inline]
  pub fn depth_compare_op(&self) -> VkCompareOp {
    self.depthCompareOp
  }
  #[inline]
  pub fn is_depth_bounds_test_enable(&self) -> bool {
    self.depthBoundsTestEnable != 0
  }
  #[inline]
  pub fn is_stencil_test_enable(&self) -> bool {
    self.stencilTestEnable != 0
  }
  #[inline]
  pub fn front(&self) -> &VkStencilOpState {
    &self.front
  }
  #[inline]
  pub fn back(&self) -> &VkStencilOpState {
    &self.back
  }
  #[inline]
  pub fn min_depth_bounds(&self) -> f32 {
    self.minDepthBounds
  }
  #[inline]
  pub fn max_depth_bounds(&self) -> f32 {
    self.maxDepthBounds
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineDepthStencilStateCreateInfo<'l> {
  fn default() -> VkPipelineDepthStencilStateCreateInfo<'l> {
    VkPipelineDepthStencilStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineDepthStencilStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_depth_stencil_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(88 + ptr_size * 2, VkPipelineDepthStencilStateCreateInfo);
}

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

/// Structure specifying a pipeline color blend attachment state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendAttachmentState {
  blendEnable: VkBool32,
  pub srcColorBlendFactor: VkBlendFactor,
  pub dstColorBlendFactor: VkBlendFactor,
  pub colorBlendOp: VkBlendOp,
  pub srcAlphaBlendFactor: VkBlendFactor,
  pub dstAlphaBlendFactor: VkBlendFactor,
  pub alphaBlendOp: VkBlendOp,
  pub colorWriteMask: VkColorComponentFlags,
}
impl VkPipelineColorBlendAttachmentState {
  #[inline]
  pub fn new() -> VkPipelineColorBlendAttachmentState {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_blend_enable(mut self, value: bool) -> Self {
    unsafe {
      self.blendEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_src_color_blend_factor(mut self, value: VkBlendFactor) -> Self {
    self.srcColorBlendFactor = value;
    self
  }
  #[inline]
  pub fn set_dst_color_blend_factor(mut self, value: VkBlendFactor) -> Self {
    self.dstColorBlendFactor = value;
    self
  }
  #[inline]
  pub fn set_color_blend_op(mut self, value: VkBlendOp) -> Self {
    self.colorBlendOp = value;
    self
  }
  #[inline]
  pub fn set_src_alpha_blend_factor(mut self, value: VkBlendFactor) -> Self {
    self.srcAlphaBlendFactor = value;
    self
  }
  #[inline]
  pub fn set_dst_alpha_blend_factor(mut self, value: VkBlendFactor) -> Self {
    self.dstAlphaBlendFactor = value;
    self
  }
  #[inline]
  pub fn set_alpha_blend_op(mut self, value: VkBlendOp) -> Self {
    self.alphaBlendOp = value;
    self
  }
  #[inline]
  pub fn set_color_write_mask(mut self, value: VkColorComponentFlags) -> Self {
    self.colorWriteMask = value;
    self
  }
  #[inline]
  pub fn is_blend_enable(&self) -> bool {
    self.blendEnable != 0
  }
  #[inline]
  pub fn src_color_blend_factor(&self) -> VkBlendFactor {
    self.srcColorBlendFactor
  }
  #[inline]
  pub fn dst_color_blend_factor(&self) -> VkBlendFactor {
    self.dstColorBlendFactor
  }
  #[inline]
  pub fn color_blend_op(&self) -> VkBlendOp {
    self.colorBlendOp
  }
  #[inline]
  pub fn src_alpha_blend_factor(&self) -> VkBlendFactor {
    self.srcAlphaBlendFactor
  }
  #[inline]
  pub fn dst_alpha_blend_factor(&self) -> VkBlendFactor {
    self.dstAlphaBlendFactor
  }
  #[inline]
  pub fn alpha_blend_op(&self) -> VkBlendOp {
    self.alphaBlendOp
  }
  #[inline]
  pub fn color_write_mask(&self) -> VkColorComponentFlags {
    self.colorWriteMask
  }
}
impl Default for VkPipelineColorBlendAttachmentState {
  fn default() -> VkPipelineColorBlendAttachmentState {
    VkPipelineColorBlendAttachmentState::new()
  }
}
unsafe impl Struct for VkPipelineColorBlendAttachmentState {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_color_blend_attachment_state() {
  assert_size!(32, VkPipelineColorBlendAttachmentState);
}

/// Structure specifying parameters of a newly created pipeline color blend state
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineColorBlendStateCreateFlags,
  logicOpEnable: VkBool32,
  pub logicOp: VkLogicOp,
  attachmentCount: u32,
  pAttachments: *const VkPipelineColorBlendAttachmentState,
  pub blendConstants: [f32; 4],
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineColorBlendStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineColorBlendStateCreateInfo<'l> {
    unsafe {
      VkPipelineColorBlendStateCreateInfo {
        sType: VkStructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineColorBlendStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_logic_op_enable(mut self, value: bool) -> Self {
    unsafe {
      self.logicOpEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_logic_op(mut self, value: VkLogicOp) -> Self {
    self.logicOp = value;
    self
  }
  #[inline]
  pub fn set_attachments(mut self, value: &'l [VkPipelineColorBlendAttachmentState]) -> Self {
    self.attachmentCount = value.len() as u32;
    unsafe {
      self.pAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_blend_constants(mut self, value: [f32; 4]) -> Self {
    self.blendConstants = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineColorBlendStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn is_logic_op_enable(&self) -> bool {
    self.logicOpEnable != 0
  }
  #[inline]
  pub fn logic_op(&self) -> VkLogicOp {
    self.logicOp
  }
  #[inline]
  pub fn attachment_count(&self) -> u32 {
    self.attachmentCount
  }
  #[inline]
  pub fn blend_constants(&self) -> [f32; 4] {
    self.blendConstants
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineColorBlendStateCreateInfo<'l> {
  fn default() -> VkPipelineColorBlendStateCreateInfo<'l> {
    VkPipelineColorBlendStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineColorBlendStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_color_blend_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 3, VkPipelineColorBlendStateCreateInfo);
}

/// Reserved for future use
pub type VkPipelineDynamicStateCreateFlags = VkFlags;

/// Indicate which dynamic state is taken from dynamic state commands
pub use enums::VkDynamicState;

/// Structure specifying parameters of a newly created pipeline dynamic state
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineDynamicStateCreateFlags,
  dynamicStateCount: u32,
  pDynamicStates: *const VkDynamicState,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkPipelineDynamicStateCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkPipelineDynamicStateCreateInfo<'l> {
    unsafe {
      VkPipelineDynamicStateCreateInfo {
        sType: VkStructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineDynamicStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_dynamic_states(mut self, value: &'l [VkDynamicState]) -> Self {
    self.dynamicStateCount = value.len() as u32;
    unsafe {
      self.pDynamicStates = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineDynamicStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn dynamic_state_count(&self) -> u32 {
    self.dynamicStateCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkPipelineDynamicStateCreateInfo<'l> {
  fn default() -> VkPipelineDynamicStateCreateInfo<'l> {
    VkPipelineDynamicStateCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkPipelineDynamicStateCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_dynamic_state_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPipelineDynamicStateCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPipelineLayout__ {}

/// Opaque handle to a pipeline layout object
pub type VkPipelineLayout<'l> = VkNonDispatchableHandle<'l, VkPipelineLayout__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkRenderPass__ {}

/// Opaque handle to a render pass object
pub type VkRenderPass<'l> = VkNonDispatchableHandle<'l, VkRenderPass__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPipeline__ {}

/// Opaque handle to a pipeline object
pub type VkPipeline<'l> = VkNonDispatchableHandle<'l, VkPipeline__>;

/// Structure specifying parameters of a newly created graphics pipeline
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCreateFlags,
  stageCount: u32,
  pStages: *const VkPipelineShaderStageCreateInfo<'l, 'h>,
  pub pVertexInputState: &'l VkPipelineVertexInputStateCreateInfo<'l>,
  pub pInputAssemblyState: &'l VkPipelineInputAssemblyStateCreateInfo<'l>,
  pTessellationState: *const VkPipelineTessellationStateCreateInfo<'l>,
  pViewportState: *const VkPipelineViewportStateCreateInfo<'l>,
  pub pRasterizationState: &'l VkPipelineRasterizationStateCreateInfo<'l>,
  pMultisampleState: *const VkPipelineMultisampleStateCreateInfo<'l>,
  pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo<'l>,
  pColorBlendState: *const VkPipelineColorBlendStateCreateInfo<'l>,
  pDynamicState: *const VkPipelineDynamicStateCreateInfo<'l>,
  layout: u64,
  renderPass: u64,
  pub subpass: u32,
  basePipelineHandle: u64,
  pub basePipelineIndex: i32,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
impl<'l, 'h: 'l> VkGraphicsPipelineCreateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkGraphicsPipelineCreateInfo<'l, 'h> {
    unsafe {
      VkGraphicsPipelineCreateInfo {
        sType: VkStructureType::GRAPHICS_PIPELINE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_stages(mut self, value: &'l [VkPipelineShaderStageCreateInfo<'l, 'h>]) -> Self {
    self.stageCount = value.len() as u32;
    unsafe {
      self.pStages = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_vertex_input_state(mut self, value: &'l VkPipelineVertexInputStateCreateInfo<'l>) -> Self {
    self.pVertexInputState = value;
    self
  }
  #[inline]
  pub fn set_input_assembly_state(mut self, value: &'l VkPipelineInputAssemblyStateCreateInfo<'l>) -> Self {
    self.pInputAssemblyState = value;
    self
  }
  #[inline]
  pub fn set_tessellation_state(mut self, value: Option<&'l VkPipelineTessellationStateCreateInfo<'l>>) -> Self {
    unsafe {
      self.pTessellationState = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_viewport_state(mut self, value: Option<&'l VkPipelineViewportStateCreateInfo<'l>>) -> Self {
    unsafe {
      self.pViewportState = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_rasterization_state(mut self, value: &'l VkPipelineRasterizationStateCreateInfo<'l>) -> Self {
    self.pRasterizationState = value;
    self
  }
  #[inline]
  pub fn set_multisample_state(mut self, value: Option<&'l VkPipelineMultisampleStateCreateInfo<'l>>) -> Self {
    unsafe {
      self.pMultisampleState = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_stencil_state(mut self, value: Option<&'l VkPipelineDepthStencilStateCreateInfo<'l>>) -> Self {
    unsafe {
      self.pDepthStencilState = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_color_blend_state(mut self, value: Option<&'l VkPipelineColorBlendStateCreateInfo<'l>>) -> Self {
    unsafe {
      self.pColorBlendState = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_dynamic_state(mut self, value: Option<&'l VkPipelineDynamicStateCreateInfo<'l>>) -> Self {
    unsafe {
      self.pDynamicState = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_layout(mut self, value: VkPipelineLayout<'h>) -> Self {
    unsafe {
      self.layout = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_render_pass(mut self, value: VkRenderPass<'h>) -> Self {
    unsafe {
      self.renderPass = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_subpass(mut self, value: u32) -> Self {
    self.subpass = value;
    self
  }
  #[inline]
  pub fn set_base_pipeline_handle(mut self, value: Option<VkPipeline<'h>>) -> Self {
    unsafe {
      self.basePipelineHandle = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_base_pipeline_index(mut self, value: i32) -> Self {
    self.basePipelineIndex = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineCreateFlags {
    self.flags
  }
  #[inline]
  pub fn stage_count(&self) -> u32 {
    self.stageCount
  }
  #[inline]
  pub fn vertex_input_state(&self) -> &'l VkPipelineVertexInputStateCreateInfo<'l> {
    self.pVertexInputState
  }
  #[inline]
  pub fn input_assembly_state(&self) -> &'l VkPipelineInputAssemblyStateCreateInfo<'l> {
    self.pInputAssemblyState
  }
  #[inline]
  pub fn rasterization_state(&self) -> &'l VkPipelineRasterizationStateCreateInfo<'l> {
    self.pRasterizationState
  }
  #[inline]
  pub fn subpass(&self) -> u32 {
    self.subpass
  }
  #[inline]
  pub fn base_pipeline_index(&self) -> i32 {
    self.basePipelineIndex
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkGraphicsPipelineCreateInfo<'l, 'h> {
  fn default() -> VkGraphicsPipelineCreateInfo<'l, 'h> {
    VkGraphicsPipelineCreateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkGraphicsPipelineCreateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_graphics_pipeline_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 14, VkGraphicsPipelineCreateInfo);
}

/// Structure specifying parameters of a newly created compute pipeline
#[repr(C)]
pub struct VkComputePipelineCreateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCreateFlags,
  pub stage: VkPipelineShaderStageCreateInfo<'l, 'h>,
  layout: u64,
  basePipelineHandle: u64,
  pub basePipelineIndex: i32,
}
impl<'l, 'h: 'l> VkComputePipelineCreateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkComputePipelineCreateInfo<'l, 'h> {
    unsafe {
      VkComputePipelineCreateInfo {
        sType: VkStructureType::COMPUTE_PIPELINE_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_stage(mut self, value: VkPipelineShaderStageCreateInfo<'l, 'h>) -> Self {
    self.stage = value;
    self
  }
  #[inline]
  pub fn set_layout(mut self, value: VkPipelineLayout<'h>) -> Self {
    unsafe {
      self.layout = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_base_pipeline_handle(mut self, value: Option<VkPipeline<'h>>) -> Self {
    unsafe {
      self.basePipelineHandle = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_base_pipeline_index(mut self, value: i32) -> Self {
    self.basePipelineIndex = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineCreateFlags {
    self.flags
  }
  #[inline]
  pub fn stage(&self) -> &VkPipelineShaderStageCreateInfo<'l, 'h> {
    &self.stage
  }
  #[inline]
  pub fn base_pipeline_index(&self) -> i32 {
    self.basePipelineIndex
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkComputePipelineCreateInfo<'l, 'h> {
  fn default() -> VkComputePipelineCreateInfo<'l, 'h> {
    VkComputePipelineCreateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkComputePipelineCreateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_compute_pipeline_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 8, VkComputePipelineCreateInfo);
}

/// Reserved for future use
pub type VkPipelineLayoutCreateFlags = VkFlags;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDescriptorSetLayout__ {}

/// Opaque handle to a descriptor set layout object
pub type VkDescriptorSetLayout<'l> = VkNonDispatchableHandle<'l, VkDescriptorSetLayout__>;

/// Bitmask of VkShaderStageFlagBits
pub type VkShaderStageFlags = VkShaderStageFlagBits;

/// Structure specifying a push constant range
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPushConstantRange {
  pub stageFlags: VkShaderStageFlags,
  pub offset: u32,
  pub size: u32,
}
impl VkPushConstantRange {
  #[inline]
  pub fn new() -> VkPushConstantRange {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_stage_flags(mut self, value: VkShaderStageFlags) -> Self {
    self.stageFlags = value;
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: u32) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_size(mut self, value: u32) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn stage_flags(&self) -> VkShaderStageFlags {
    self.stageFlags
  }
  #[inline]
  pub fn offset(&self) -> u32 {
    self.offset
  }
  #[inline]
  pub fn size(&self) -> u32 {
    self.size
  }
}
impl Default for VkPushConstantRange {
  fn default() -> VkPushConstantRange {
    VkPushConstantRange::new()
  }
}
unsafe impl Struct for VkPushConstantRange {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_push_constant_range() {
  assert_size!(12, VkPushConstantRange);
}

/// Structure specifying the parameters of a newly created pipeline layout object
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineLayoutCreateFlags,
  setLayoutCount: u32,
  pSetLayouts: *const u64,
  pushConstantRangeCount: u32,
  pPushConstantRanges: *const VkPushConstantRange,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkPipelineLayoutCreateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkPipelineLayoutCreateInfo<'l, 'h> {
    unsafe {
      VkPipelineLayoutCreateInfo {
        sType: VkStructureType::PIPELINE_LAYOUT_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineLayoutCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_set_layouts(mut self, value: &'l [VkDescriptorSetLayout<'h>]) -> Self {
    self.setLayoutCount = value.len() as u32;
    unsafe {
      self.pSetLayouts = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_push_constant_ranges(mut self, value: &'l [VkPushConstantRange]) -> Self {
    self.pushConstantRangeCount = value.len() as u32;
    unsafe {
      self.pPushConstantRanges = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineLayoutCreateFlags {
    self.flags
  }
  #[inline]
  pub fn set_layout_count(&self) -> u32 {
    self.setLayoutCount
  }
  #[inline]
  pub fn push_constant_range_count(&self) -> u32 {
    self.pushConstantRangeCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkPipelineLayoutCreateInfo<'l, 'h> {
  fn default() -> VkPipelineLayoutCreateInfo<'l, 'h> {
    VkPipelineLayoutCreateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkPipelineLayoutCreateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_layout_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 5, VkPipelineLayoutCreateInfo);
}

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

/// Structure specifying parameters of a newly created sampler
#[repr(C)]
pub struct VkSamplerCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkSamplerCreateFlags,
  pub magFilter: VkFilter,
  pub minFilter: VkFilter,
  pub mipmapMode: VkSamplerMipmapMode,
  pub addressModeU: VkSamplerAddressMode,
  pub addressModeV: VkSamplerAddressMode,
  pub addressModeW: VkSamplerAddressMode,
  pub mipLodBias: f32,
  anisotropyEnable: VkBool32,
  pub maxAnisotropy: f32,
  compareEnable: VkBool32,
  pub compareOp: VkCompareOp,
  pub minLod: f32,
  pub maxLod: f32,
  pub borderColor: VkBorderColor,
  unnormalizedCoordinates: VkBool32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkSamplerCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkSamplerCreateInfo<'l> {
    unsafe {
      VkSamplerCreateInfo {
        sType: VkStructureType::SAMPLER_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSamplerCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_mag_filter(mut self, value: VkFilter) -> Self {
    self.magFilter = value;
    self
  }
  #[inline]
  pub fn set_min_filter(mut self, value: VkFilter) -> Self {
    self.minFilter = value;
    self
  }
  #[inline]
  pub fn set_mipmap_mode(mut self, value: VkSamplerMipmapMode) -> Self {
    self.mipmapMode = value;
    self
  }
  #[inline]
  pub fn set_address_mode_u(mut self, value: VkSamplerAddressMode) -> Self {
    self.addressModeU = value;
    self
  }
  #[inline]
  pub fn set_address_mode_v(mut self, value: VkSamplerAddressMode) -> Self {
    self.addressModeV = value;
    self
  }
  #[inline]
  pub fn set_address_mode_w(mut self, value: VkSamplerAddressMode) -> Self {
    self.addressModeW = value;
    self
  }
  #[inline]
  pub fn set_mip_lod_bias(mut self, value: f32) -> Self {
    self.mipLodBias = value;
    self
  }
  #[inline]
  pub fn set_anisotropy_enable(mut self, value: bool) -> Self {
    unsafe {
      self.anisotropyEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_max_anisotropy(mut self, value: f32) -> Self {
    self.maxAnisotropy = value;
    self
  }
  #[inline]
  pub fn set_compare_enable(mut self, value: bool) -> Self {
    unsafe {
      self.compareEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_compare_op(mut self, value: VkCompareOp) -> Self {
    self.compareOp = value;
    self
  }
  #[inline]
  pub fn set_min_lod(mut self, value: f32) -> Self {
    self.minLod = value;
    self
  }
  #[inline]
  pub fn set_max_lod(mut self, value: f32) -> Self {
    self.maxLod = value;
    self
  }
  #[inline]
  pub fn set_border_color(mut self, value: VkBorderColor) -> Self {
    self.borderColor = value;
    self
  }
  #[inline]
  pub fn set_unnormalized_coordinates(mut self, value: bool) -> Self {
    unsafe {
      self.unnormalizedCoordinates = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkSamplerCreateFlags {
    self.flags
  }
  #[inline]
  pub fn mag_filter(&self) -> VkFilter {
    self.magFilter
  }
  #[inline]
  pub fn min_filter(&self) -> VkFilter {
    self.minFilter
  }
  #[inline]
  pub fn mipmap_mode(&self) -> VkSamplerMipmapMode {
    self.mipmapMode
  }
  #[inline]
  pub fn address_mode_u(&self) -> VkSamplerAddressMode {
    self.addressModeU
  }
  #[inline]
  pub fn address_mode_v(&self) -> VkSamplerAddressMode {
    self.addressModeV
  }
  #[inline]
  pub fn address_mode_w(&self) -> VkSamplerAddressMode {
    self.addressModeW
  }
  #[inline]
  pub fn mip_lod_bias(&self) -> f32 {
    self.mipLodBias
  }
  #[inline]
  pub fn is_anisotropy_enable(&self) -> bool {
    self.anisotropyEnable != 0
  }
  #[inline]
  pub fn max_anisotropy(&self) -> f32 {
    self.maxAnisotropy
  }
  #[inline]
  pub fn is_compare_enable(&self) -> bool {
    self.compareEnable != 0
  }
  #[inline]
  pub fn compare_op(&self) -> VkCompareOp {
    self.compareOp
  }
  #[inline]
  pub fn min_lod(&self) -> f32 {
    self.minLod
  }
  #[inline]
  pub fn max_lod(&self) -> f32 {
    self.maxLod
  }
  #[inline]
  pub fn border_color(&self) -> VkBorderColor {
    self.borderColor
  }
  #[inline]
  pub fn is_unnormalized_coordinates(&self) -> bool {
    self.unnormalizedCoordinates != 0
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkSamplerCreateInfo<'l> {
  fn default() -> VkSamplerCreateInfo<'l> {
    VkSamplerCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkSamplerCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sampler_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(64 + ptr_size * 2, VkSamplerCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSampler__ {}

/// Opaque handle to a sampler object
pub type VkSampler<'l> = VkNonDispatchableHandle<'l, VkSampler__>;

/// Bitmask specifying descriptor set layout properties
pub use enums::VkDescriptorSetLayoutCreateFlagBits;

/// Bitmask of VkDescriptorSetLayoutCreateFlagBits
pub type VkDescriptorSetLayoutCreateFlags = VkDescriptorSetLayoutCreateFlagBits;

/// Specifies the type of a descriptor in a descriptor set
pub use enums::VkDescriptorType;

/// Structure specifying a descriptor set layout binding
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding<'l, 'h: 'l> {
  pub binding: u32,
  pub descriptorType: VkDescriptorType,
  descriptorCount: u32,
  pub stageFlags: VkShaderStageFlags,
  pImmutableSamplers: *const u64,
  _p: ::std::marker::PhantomData<(&'h u8, &'l u8)>,
}
impl<'l, 'h: 'l> VkDescriptorSetLayoutBinding<'l, 'h> {
  #[inline]
  pub fn new() -> VkDescriptorSetLayoutBinding<'l, 'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_binding(mut self, value: u32) -> Self {
    self.binding = value;
    self
  }
  #[inline]
  pub fn set_descriptor_type(mut self, value: VkDescriptorType) -> Self {
    self.descriptorType = value;
    self
  }
  #[inline]
  pub fn set_stage_flags(mut self, value: VkShaderStageFlags) -> Self {
    self.stageFlags = value;
    self
  }
  #[inline]
  pub fn set_immutable_samplers(mut self, value: &'l [VkSampler<'h>]) -> Self {
    self.descriptorCount = value.len() as u32;
    unsafe {
      self.pImmutableSamplers = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn binding(&self) -> u32 {
    self.binding
  }
  #[inline]
  pub fn descriptor_type(&self) -> VkDescriptorType {
    self.descriptorType
  }
  #[inline]
  pub fn descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
  #[inline]
  pub fn stage_flags(&self) -> VkShaderStageFlags {
    self.stageFlags
  }
}
impl<'l, 'h: 'l> Default for VkDescriptorSetLayoutBinding<'l, 'h> {
  fn default() -> VkDescriptorSetLayoutBinding<'l, 'h> {
    VkDescriptorSetLayoutBinding::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkDescriptorSetLayoutBinding<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_set_layout_binding() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 1, VkDescriptorSetLayoutBinding);
}

/// Structure specifying parameters of a newly created descriptor set layout
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDescriptorSetLayoutCreateFlags,
  bindingCount: u32,
  pBindings: *const VkDescriptorSetLayoutBinding<'l, 'h>,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkDescriptorSetLayoutCreateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkDescriptorSetLayoutCreateInfo<'l, 'h> {
    unsafe {
      VkDescriptorSetLayoutCreateInfo {
        sType: VkStructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDescriptorSetLayoutCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_bindings(mut self, value: &'l [VkDescriptorSetLayoutBinding<'l, 'h>]) -> Self {
    self.bindingCount = value.len() as u32;
    unsafe {
      self.pBindings = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkDescriptorSetLayoutCreateFlags {
    self.flags
  }
  #[inline]
  pub fn binding_count(&self) -> u32 {
    self.bindingCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkDescriptorSetLayoutCreateInfo<'l, 'h> {
  fn default() -> VkDescriptorSetLayoutCreateInfo<'l, 'h> {
    VkDescriptorSetLayoutCreateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkDescriptorSetLayoutCreateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_set_layout_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkDescriptorSetLayoutCreateInfo);
}

/// Bitmask specifying certain supported operations on a descriptor pool
pub use enums::VkDescriptorPoolCreateFlagBits;

/// Bitmask of VkDescriptorPoolCreateFlagBits
pub type VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlagBits;

/// Structure specifying descriptor pool size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolSize {
  pub eType: VkDescriptorType,
  pub descriptorCount: u32,
}
impl VkDescriptorPoolSize {
  #[inline]
  pub fn new() -> VkDescriptorPoolSize {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_type(mut self, value: VkDescriptorType) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_descriptor_count(mut self, value: u32) -> Self {
    self.descriptorCount = value;
    self
  }
  #[inline]
  pub fn get_type(&self) -> VkDescriptorType {
    self.eType
  }
  #[inline]
  pub fn descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
}
impl Default for VkDescriptorPoolSize {
  fn default() -> VkDescriptorPoolSize {
    VkDescriptorPoolSize::new()
  }
}
unsafe impl Struct for VkDescriptorPoolSize {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_pool_size() {
  assert_size!(8, VkDescriptorPoolSize);
}

/// Structure specifying parameters of a newly created descriptor pool
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDescriptorPoolCreateFlags,
  pub maxSets: u32,
  poolSizeCount: u32,
  pPoolSizes: *const VkDescriptorPoolSize,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkDescriptorPoolCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkDescriptorPoolCreateInfo<'l> {
    unsafe {
      VkDescriptorPoolCreateInfo {
        sType: VkStructureType::DESCRIPTOR_POOL_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDescriptorPoolCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_max_sets(mut self, value: u32) -> Self {
    self.maxSets = value;
    self
  }
  #[inline]
  pub fn set_pool_sizes(mut self, value: &'l [VkDescriptorPoolSize]) -> Self {
    self.poolSizeCount = value.len() as u32;
    unsafe {
      self.pPoolSizes = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkDescriptorPoolCreateFlags {
    self.flags
  }
  #[inline]
  pub fn max_sets(&self) -> u32 {
    self.maxSets
  }
  #[inline]
  pub fn pool_size_count(&self) -> u32 {
    self.poolSizeCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkDescriptorPoolCreateInfo<'l> {
  fn default() -> VkDescriptorPoolCreateInfo<'l> {
    VkDescriptorPoolCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkDescriptorPoolCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_pool_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 4, VkDescriptorPoolCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDescriptorPool__ {}

/// Opaque handle to a descriptor pool object
pub type VkDescriptorPool<'l> = VkNonDispatchableHandle<'l, VkDescriptorPool__>;

/// Reserved for future use
pub type VkDescriptorPoolResetFlags = VkFlags;

/// Structure specifying the allocation parameters for descriptor sets
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  descriptorPool: u64,
  descriptorSetCount: u32,
  pSetLayouts: *const u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkDescriptorSetAllocateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkDescriptorSetAllocateInfo<'l, 'h> {
    unsafe {
      VkDescriptorSetAllocateInfo {
        sType: VkStructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_descriptor_pool(mut self, value: VkDescriptorPool<'h>) -> Self {
    unsafe {
      self.descriptorPool = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_set_layouts(mut self, value: &'l [VkDescriptorSetLayout<'h>]) -> Self {
    self.descriptorSetCount = value.len() as u32;
    unsafe {
      self.pSetLayouts = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn descriptor_set_count(&self) -> u32 {
    self.descriptorSetCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkDescriptorSetAllocateInfo<'l, 'h> {
  fn default() -> VkDescriptorSetAllocateInfo<'l, 'h> {
    VkDescriptorSetAllocateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkDescriptorSetAllocateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_set_allocate_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 4, VkDescriptorSetAllocateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDescriptorSet__ {}

/// Opaque handle to a descriptor set object
pub type VkDescriptorSet<'l> = VkNonDispatchableHandle<'l, VkDescriptorSet__>;

/// Structure specifying descriptor image info
#[repr(C)]
pub struct VkDescriptorImageInfo<'h> {
  sampler: u64,
  imageView: u64,
  pub imageLayout: VkImageLayout,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
impl<'h> VkDescriptorImageInfo<'h> {
  #[inline]
  pub fn new() -> VkDescriptorImageInfo<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_sampler(mut self, value: VkSampler<'h>) -> Self {
    unsafe {
      self.sampler = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_view(mut self, value: VkImageView<'h>) -> Self {
    unsafe {
      self.imageView = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_layout(mut self, value: VkImageLayout) -> Self {
    self.imageLayout = value;
    self
  }
  #[inline]
  pub fn image_layout(&self) -> VkImageLayout {
    self.imageLayout
  }
}
impl<'h> Default for VkDescriptorImageInfo<'h> {
  fn default() -> VkDescriptorImageInfo<'h> {
    VkDescriptorImageInfo::new()
  }
}
unsafe impl<'h> Struct for VkDescriptorImageInfo<'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_image_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 1, VkDescriptorImageInfo);
}

/// Structure specifying descriptor buffer info
#[repr(C)]
pub struct VkDescriptorBufferInfo<'h> {
  buffer: u64,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
impl<'h> VkDescriptorBufferInfo<'h> {
  #[inline]
  pub fn new() -> VkDescriptorBufferInfo<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: VkDeviceSize) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_range(mut self, value: VkDeviceSize) -> Self {
    self.range = value;
    self
  }
  #[inline]
  pub fn offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn range(&self) -> VkDeviceSize {
    self.range
  }
}
impl<'h> Default for VkDescriptorBufferInfo<'h> {
  fn default() -> VkDescriptorBufferInfo<'h> {
    VkDescriptorBufferInfo::new()
  }
}
unsafe impl<'h> Struct for VkDescriptorBufferInfo<'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_buffer_info() {
  assert_size!(24, VkDescriptorBufferInfo);
}

/// Structure specifying the parameters of a descriptor set write operation
#[repr(C)]
pub struct VkWriteDescriptorSet<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  dstSet: u64,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  descriptorCount: u32,
  pub descriptorType: VkDescriptorType,
  pImageInfo: *const VkDescriptorImageInfo<'h>,
  pBufferInfo: *const VkDescriptorBufferInfo<'h>,
  pTexelBufferView: *const u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkWriteDescriptorSet<'l, 'h> {
  #[inline]
  pub fn new() -> VkWriteDescriptorSet<'l, 'h> {
    unsafe {
      VkWriteDescriptorSet {
        sType: VkStructureType::WRITE_DESCRIPTOR_SET,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dst_set(mut self, value: VkDescriptorSet<'h>) -> Self {
    unsafe {
      self.dstSet = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_dst_binding(mut self, value: u32) -> Self {
    self.dstBinding = value;
    self
  }
  #[inline]
  pub fn set_dst_array_element(mut self, value: u32) -> Self {
    self.dstArrayElement = value;
    self
  }
  #[inline]
  pub fn set_descriptor_type(mut self, value: VkDescriptorType) -> Self {
    self.descriptorType = value;
    self
  }
  #[inline]
  pub fn dst_binding(&self) -> u32 {
    self.dstBinding
  }
  #[inline]
  pub fn dst_array_element(&self) -> u32 {
    self.dstArrayElement
  }
  #[inline]
  pub fn descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
  #[inline]
  pub fn descriptor_type(&self) -> VkDescriptorType {
    self.descriptorType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkWriteDescriptorSet<'l, 'h> {
  fn default() -> VkWriteDescriptorSet<'l, 'h> {
    VkWriteDescriptorSet::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkWriteDescriptorSet<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_write_descriptor_set() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 5, VkWriteDescriptorSet);
}

/// Structure specifying a copy descriptor set operation
#[repr(C)]
pub struct VkCopyDescriptorSet<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  srcSet: u64,
  pub srcBinding: u32,
  pub srcArrayElement: u32,
  dstSet: u64,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkCopyDescriptorSet<'l, 'h> {
  #[inline]
  pub fn new() -> VkCopyDescriptorSet<'l, 'h> {
    unsafe {
      VkCopyDescriptorSet {
        sType: VkStructureType::COPY_DESCRIPTOR_SET,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_set(mut self, value: VkDescriptorSet<'h>) -> Self {
    unsafe {
      self.srcSet = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_src_binding(mut self, value: u32) -> Self {
    self.srcBinding = value;
    self
  }
  #[inline]
  pub fn set_src_array_element(mut self, value: u32) -> Self {
    self.srcArrayElement = value;
    self
  }
  #[inline]
  pub fn set_dst_set(mut self, value: VkDescriptorSet<'h>) -> Self {
    unsafe {
      self.dstSet = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_dst_binding(mut self, value: u32) -> Self {
    self.dstBinding = value;
    self
  }
  #[inline]
  pub fn set_dst_array_element(mut self, value: u32) -> Self {
    self.dstArrayElement = value;
    self
  }
  #[inline]
  pub fn set_descriptor_count(mut self, value: u32) -> Self {
    self.descriptorCount = value;
    self
  }
  #[inline]
  pub fn src_binding(&self) -> u32 {
    self.srcBinding
  }
  #[inline]
  pub fn src_array_element(&self) -> u32 {
    self.srcArrayElement
  }
  #[inline]
  pub fn dst_binding(&self) -> u32 {
    self.dstBinding
  }
  #[inline]
  pub fn dst_array_element(&self) -> u32 {
    self.dstArrayElement
  }
  #[inline]
  pub fn descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkCopyDescriptorSet<'l, 'h> {
  fn default() -> VkCopyDescriptorSet<'l, 'h> {
    VkCopyDescriptorSet::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkCopyDescriptorSet<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_copy_descriptor_set() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 3, VkCopyDescriptorSet);
}

/// Reserved for future use
pub type VkFramebufferCreateFlags = VkFlags;

/// Structure specifying parameters of a newly created framebuffer
#[repr(C)]
pub struct VkFramebufferCreateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkFramebufferCreateFlags,
  renderPass: u64,
  attachmentCount: u32,
  pAttachments: *const u64,
  pub width: u32,
  pub height: u32,
  pub layers: u32,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkFramebufferCreateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkFramebufferCreateInfo<'l, 'h> {
    unsafe {
      VkFramebufferCreateInfo {
        sType: VkStructureType::FRAMEBUFFER_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkFramebufferCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_render_pass(mut self, value: VkRenderPass<'h>) -> Self {
    unsafe {
      self.renderPass = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_attachments(mut self, value: &'l [VkImageView<'h>]) -> Self {
    self.attachmentCount = value.len() as u32;
    unsafe {
      self.pAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_width(mut self, value: u32) -> Self {
    self.width = value;
    self
  }
  #[inline]
  pub fn set_height(mut self, value: u32) -> Self {
    self.height = value;
    self
  }
  #[inline]
  pub fn set_layers(mut self, value: u32) -> Self {
    self.layers = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkFramebufferCreateFlags {
    self.flags
  }
  #[inline]
  pub fn attachment_count(&self) -> u32 {
    self.attachmentCount
  }
  #[inline]
  pub fn width(&self) -> u32 {
    self.width
  }
  #[inline]
  pub fn height(&self) -> u32 {
    self.height
  }
  #[inline]
  pub fn layers(&self) -> u32 {
    self.layers
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkFramebufferCreateInfo<'l, 'h> {
  fn default() -> VkFramebufferCreateInfo<'l, 'h> {
    VkFramebufferCreateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkFramebufferCreateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_framebuffer_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 6, VkFramebufferCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkFramebuffer__ {}

/// Opaque handle to a framebuffer object
pub type VkFramebuffer<'l> = VkNonDispatchableHandle<'l, VkFramebuffer__>;

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

/// Structure specifying an attachment description
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
impl VkAttachmentDescription {
  #[inline]
  pub fn new() -> VkAttachmentDescription {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkAttachmentDescriptionFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_samples(mut self, value: VkSampleCountFlagBits) -> Self {
    self.samples = value;
    self
  }
  #[inline]
  pub fn set_load_op(mut self, value: VkAttachmentLoadOp) -> Self {
    self.loadOp = value;
    self
  }
  #[inline]
  pub fn set_store_op(mut self, value: VkAttachmentStoreOp) -> Self {
    self.storeOp = value;
    self
  }
  #[inline]
  pub fn set_stencil_load_op(mut self, value: VkAttachmentLoadOp) -> Self {
    self.stencilLoadOp = value;
    self
  }
  #[inline]
  pub fn set_stencil_store_op(mut self, value: VkAttachmentStoreOp) -> Self {
    self.stencilStoreOp = value;
    self
  }
  #[inline]
  pub fn set_initial_layout(mut self, value: VkImageLayout) -> Self {
    self.initialLayout = value;
    self
  }
  #[inline]
  pub fn set_final_layout(mut self, value: VkImageLayout) -> Self {
    self.finalLayout = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkAttachmentDescriptionFlags {
    self.flags
  }
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn samples(&self) -> VkSampleCountFlagBits {
    self.samples
  }
  #[inline]
  pub fn load_op(&self) -> VkAttachmentLoadOp {
    self.loadOp
  }
  #[inline]
  pub fn store_op(&self) -> VkAttachmentStoreOp {
    self.storeOp
  }
  #[inline]
  pub fn stencil_load_op(&self) -> VkAttachmentLoadOp {
    self.stencilLoadOp
  }
  #[inline]
  pub fn stencil_store_op(&self) -> VkAttachmentStoreOp {
    self.stencilStoreOp
  }
  #[inline]
  pub fn initial_layout(&self) -> VkImageLayout {
    self.initialLayout
  }
  #[inline]
  pub fn final_layout(&self) -> VkImageLayout {
    self.finalLayout
  }
}
impl Default for VkAttachmentDescription {
  fn default() -> VkAttachmentDescription {
    VkAttachmentDescription::new()
  }
}
unsafe impl Struct for VkAttachmentDescription {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_attachment_description() {
  assert_size!(36, VkAttachmentDescription);
}

/// Bitmask specifying usage of a subpass
pub use enums::VkSubpassDescriptionFlagBits;

/// Bitmask of VkSubpassDescriptionFlagBits
pub type VkSubpassDescriptionFlags = VkSubpassDescriptionFlagBits;

/// Specify the bind point of a pipeline object to a command buffer
pub use enums::VkPipelineBindPoint;

/// Structure specifying an attachment reference
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReference {
  pub attachment: u32,
  pub layout: VkImageLayout,
}
impl VkAttachmentReference {
  #[inline]
  pub fn new() -> VkAttachmentReference {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_attachment(mut self, value: u32) -> Self {
    self.attachment = value;
    self
  }
  #[inline]
  pub fn set_layout(mut self, value: VkImageLayout) -> Self {
    self.layout = value;
    self
  }
  #[inline]
  pub fn attachment(&self) -> u32 {
    self.attachment
  }
  #[inline]
  pub fn layout(&self) -> VkImageLayout {
    self.layout
  }
}
impl Default for VkAttachmentReference {
  fn default() -> VkAttachmentReference {
    VkAttachmentReference::new()
  }
}
unsafe impl Struct for VkAttachmentReference {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_attachment_reference() {
  assert_size!(8, VkAttachmentReference);
}

/// Structure specifying a subpass description
#[repr(C)]
pub struct VkSubpassDescription<'l> {
  pub flags: VkSubpassDescriptionFlags,
  pub pipelineBindPoint: VkPipelineBindPoint,
  inputAttachmentCount: u32,
  pInputAttachments: *const VkAttachmentReference,
  colorAttachmentCount: u32,
  pColorAttachments: *const VkAttachmentReference,
  pResolveAttachments: *const VkAttachmentReference,
  pDepthStencilAttachment: *const VkAttachmentReference,
  preserveAttachmentCount: u32,
  pPreserveAttachments: *const u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkSubpassDescription<'l> {
  #[inline]
  pub fn new() -> VkSubpassDescription<'l> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSubpassDescriptionFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_pipeline_bind_point(mut self, value: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = value;
    self
  }
  #[inline]
  pub fn set_input_attachments(mut self, value: &'l [VkAttachmentReference]) -> Self {
    self.inputAttachmentCount = value.len() as u32;
    unsafe {
      self.pInputAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_stencil_attachment(mut self, value: Option<&'l VkAttachmentReference>) -> Self {
    unsafe {
      self.pDepthStencilAttachment = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_preserve_attachments(mut self, value: &'l [u32]) -> Self {
    self.preserveAttachmentCount = value.len() as u32;
    unsafe {
      self.pPreserveAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkSubpassDescriptionFlags {
    self.flags
  }
  #[inline]
  pub fn pipeline_bind_point(&self) -> VkPipelineBindPoint {
    self.pipelineBindPoint
  }
  #[inline]
  pub fn input_attachment_count(&self) -> u32 {
    self.inputAttachmentCount
  }
  #[inline]
  pub fn color_attachment_count(&self) -> u32 {
    self.colorAttachmentCount
  }
  #[inline]
  pub fn preserve_attachment_count(&self) -> u32 {
    self.preserveAttachmentCount
  }
}
impl<'l> Default for VkSubpassDescription<'l> {
  fn default() -> VkSubpassDescription<'l> {
    VkSubpassDescription::new()
  }
}
unsafe impl<'l> Struct for VkSubpassDescription<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subpass_description() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 8, VkSubpassDescription);
}

/// Bitmask specifying how execution and memory dependencies are formed
pub use enums::VkDependencyFlagBits;

/// Bitmask of VkDependencyFlagBits
pub type VkDependencyFlags = VkDependencyFlagBits;

/// Structure specifying a subpass dependency
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
impl VkSubpassDependency {
  #[inline]
  pub fn new() -> VkSubpassDependency {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_src_subpass(mut self, value: u32) -> Self {
    self.srcSubpass = value;
    self
  }
  #[inline]
  pub fn set_dst_subpass(mut self, value: u32) -> Self {
    self.dstSubpass = value;
    self
  }
  #[inline]
  pub fn set_src_stage_mask(mut self, value: VkPipelineStageFlags) -> Self {
    self.srcStageMask = value;
    self
  }
  #[inline]
  pub fn set_dst_stage_mask(mut self, value: VkPipelineStageFlags) -> Self {
    self.dstStageMask = value;
    self
  }
  #[inline]
  pub fn set_src_access_mask(mut self, value: VkAccessFlags) -> Self {
    self.srcAccessMask = value;
    self
  }
  #[inline]
  pub fn set_dst_access_mask(mut self, value: VkAccessFlags) -> Self {
    self.dstAccessMask = value;
    self
  }
  #[inline]
  pub fn set_dependency_flags(mut self, value: VkDependencyFlags) -> Self {
    self.dependencyFlags = value;
    self
  }
  #[inline]
  pub fn src_subpass(&self) -> u32 {
    self.srcSubpass
  }
  #[inline]
  pub fn dst_subpass(&self) -> u32 {
    self.dstSubpass
  }
  #[inline]
  pub fn src_stage_mask(&self) -> VkPipelineStageFlags {
    self.srcStageMask
  }
  #[inline]
  pub fn dst_stage_mask(&self) -> VkPipelineStageFlags {
    self.dstStageMask
  }
  #[inline]
  pub fn src_access_mask(&self) -> VkAccessFlags {
    self.srcAccessMask
  }
  #[inline]
  pub fn dst_access_mask(&self) -> VkAccessFlags {
    self.dstAccessMask
  }
  #[inline]
  pub fn dependency_flags(&self) -> VkDependencyFlags {
    self.dependencyFlags
  }
}
impl Default for VkSubpassDependency {
  fn default() -> VkSubpassDependency {
    VkSubpassDependency::new()
  }
}
unsafe impl Struct for VkSubpassDependency {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subpass_dependency() {
  assert_size!(28, VkSubpassDependency);
}

/// Structure specifying parameters of a newly created render pass
#[repr(C)]
pub struct VkRenderPassCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkRenderPassCreateFlags,
  attachmentCount: u32,
  pAttachments: *const VkAttachmentDescription,
  subpassCount: u32,
  pSubpasses: *const VkSubpassDescription<'l>,
  dependencyCount: u32,
  pDependencies: *const VkSubpassDependency,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkRenderPassCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkRenderPassCreateInfo<'l> {
    unsafe {
      VkRenderPassCreateInfo {
        sType: VkStructureType::RENDER_PASS_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkRenderPassCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_attachments(mut self, value: &'l [VkAttachmentDescription]) -> Self {
    self.attachmentCount = value.len() as u32;
    unsafe {
      self.pAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_subpasses(mut self, value: &'l [VkSubpassDescription<'l>]) -> Self {
    self.subpassCount = value.len() as u32;
    unsafe {
      self.pSubpasses = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_dependencies(mut self, value: &'l [VkSubpassDependency]) -> Self {
    self.dependencyCount = value.len() as u32;
    unsafe {
      self.pDependencies = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkRenderPassCreateFlags {
    self.flags
  }
  #[inline]
  pub fn attachment_count(&self) -> u32 {
    self.attachmentCount
  }
  #[inline]
  pub fn subpass_count(&self) -> u32 {
    self.subpassCount
  }
  #[inline]
  pub fn dependency_count(&self) -> u32 {
    self.dependencyCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkRenderPassCreateInfo<'l> {
  fn default() -> VkRenderPassCreateInfo<'l> {
    VkRenderPassCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkRenderPassCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 7, VkRenderPassCreateInfo);
}

/// Bitmask specifying usage behavior for a command pool
pub use enums::VkCommandPoolCreateFlagBits;

/// Bitmask of VkCommandPoolCreateFlagBits
pub type VkCommandPoolCreateFlags = VkCommandPoolCreateFlagBits;

/// Structure specifying parameters of a newly created command pool
#[repr(C)]
pub struct VkCommandPoolCreateInfo<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkCommandPoolCreateFlags,
  pub queueFamilyIndex: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
impl<'l> VkCommandPoolCreateInfo<'l> {
  #[inline]
  pub fn new() -> VkCommandPoolCreateInfo<'l> {
    unsafe {
      VkCommandPoolCreateInfo {
        sType: VkStructureType::COMMAND_POOL_CREATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkCommandPoolCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_queue_family_index(mut self, value: u32) -> Self {
    self.queueFamilyIndex = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkCommandPoolCreateFlags {
    self.flags
  }
  #[inline]
  pub fn queue_family_index(&self) -> u32 {
    self.queueFamilyIndex
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l> Default for VkCommandPoolCreateInfo<'l> {
  fn default() -> VkCommandPoolCreateInfo<'l> {
    VkCommandPoolCreateInfo::new()
  }
}
unsafe impl<'l> Struct for VkCommandPoolCreateInfo<'l> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_command_pool_create_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkCommandPoolCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkCommandPool__ {}

/// Opaque handle to a command pool object
pub type VkCommandPool<'l> = VkNonDispatchableHandle<'l, VkCommandPool__>;

/// Bitmask controlling behavior of a command pool reset
pub use enums::VkCommandPoolResetFlagBits;

/// Bitmask of VkCommandPoolResetFlagBits
pub type VkCommandPoolResetFlags = VkCommandPoolResetFlagBits;

/// Enumerant specifying a command buffer level
pub use enums::VkCommandBufferLevel;

/// Structure specifying the allocation parameters for command buffer object
#[repr(C)]
pub struct VkCommandBufferAllocateInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  commandPool: u64,
  pub level: VkCommandBufferLevel,
  pub commandBufferCount: u32,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkCommandBufferAllocateInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkCommandBufferAllocateInfo<'l, 'h> {
    unsafe {
      VkCommandBufferAllocateInfo {
        sType: VkStructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_command_pool(mut self, value: VkCommandPool<'h>) -> Self {
    unsafe {
      self.commandPool = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_level(mut self, value: VkCommandBufferLevel) -> Self {
    self.level = value;
    self
  }
  #[inline]
  pub fn set_command_buffer_count(mut self, value: u32) -> Self {
    self.commandBufferCount = value;
    self
  }
  #[inline]
  pub fn level(&self) -> VkCommandBufferLevel {
    self.level
  }
  #[inline]
  pub fn command_buffer_count(&self) -> u32 {
    self.commandBufferCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkCommandBufferAllocateInfo<'l, 'h> {
  fn default() -> VkCommandBufferAllocateInfo<'l, 'h> {
    VkCommandBufferAllocateInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkCommandBufferAllocateInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_command_buffer_allocate_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 2, VkCommandBufferAllocateInfo);
}

/// Bitmask specifying usage behavior for command buffer
pub use enums::VkCommandBufferUsageFlagBits;

/// Bitmask of VkCommandBufferUsageFlagBits
pub type VkCommandBufferUsageFlags = VkCommandBufferUsageFlagBits;

/// Bitmask specifying constraints on a query
pub use enums::VkQueryControlFlagBits;

/// Bitmask of VkQueryControlFlagBits
pub type VkQueryControlFlags = VkQueryControlFlagBits;

/// Structure specifying command buffer inheritance info
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  renderPass: u64,
  pub subpass: u32,
  framebuffer: u64,
  occlusionQueryEnable: VkBool32,
  pub queryFlags: VkQueryControlFlags,
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkCommandBufferInheritanceInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkCommandBufferInheritanceInfo<'l, 'h> {
    unsafe {
      VkCommandBufferInheritanceInfo {
        sType: VkStructureType::COMMAND_BUFFER_INHERITANCE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_render_pass(mut self, value: Option<VkRenderPass<'h>>) -> Self {
    unsafe {
      self.renderPass = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_subpass(mut self, value: u32) -> Self {
    self.subpass = value;
    self
  }
  #[inline]
  pub fn set_framebuffer(mut self, value: Option<VkFramebuffer<'h>>) -> Self {
    unsafe {
      self.framebuffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_occlusion_query_enable(mut self, value: bool) -> Self {
    unsafe {
      self.occlusionQueryEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_query_flags(mut self, value: VkQueryControlFlags) -> Self {
    self.queryFlags = value;
    self
  }
  #[inline]
  pub fn set_pipeline_statistics(mut self, value: VkQueryPipelineStatisticFlags) -> Self {
    self.pipelineStatistics = value;
    self
  }
  #[inline]
  pub fn subpass(&self) -> u32 {
    self.subpass
  }
  #[inline]
  pub fn is_occlusion_query_enable(&self) -> bool {
    self.occlusionQueryEnable != 0
  }
  #[inline]
  pub fn query_flags(&self) -> VkQueryControlFlags {
    self.queryFlags
  }
  #[inline]
  pub fn pipeline_statistics(&self) -> VkQueryPipelineStatisticFlags {
    self.pipelineStatistics
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkCommandBufferInheritanceInfo<'l, 'h> {
  fn default() -> VkCommandBufferInheritanceInfo<'l, 'h> {
    VkCommandBufferInheritanceInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkCommandBufferInheritanceInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_command_buffer_inheritance_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 4, VkCommandBufferInheritanceInfo);
}

/// Structure specifying a command buffer begin operation
#[repr(C)]
pub struct VkCommandBufferBeginInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkCommandBufferUsageFlags,
  pInheritanceInfo: *const VkCommandBufferInheritanceInfo<'l, 'h>,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkCommandBufferBeginInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkCommandBufferBeginInfo<'l, 'h> {
    unsafe {
      VkCommandBufferBeginInfo {
        sType: VkStructureType::COMMAND_BUFFER_BEGIN_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkCommandBufferUsageFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_inheritance_info(mut self, value: Option<&'l VkCommandBufferInheritanceInfo<'l, 'h>>) -> Self {
    unsafe {
      self.pInheritanceInfo = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkCommandBufferUsageFlags {
    self.flags
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkCommandBufferBeginInfo<'l, 'h> {
  fn default() -> VkCommandBufferBeginInfo<'l, 'h> {
    VkCommandBufferBeginInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkCommandBufferBeginInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_command_buffer_begin_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkCommandBufferBeginInfo);
}

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

/// Structure specifying a buffer copy operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCopy {
  pub srcOffset: VkDeviceSize,
  pub dstOffset: VkDeviceSize,
  pub size: VkDeviceSize,
}
impl VkBufferCopy {
  #[inline]
  pub fn new() -> VkBufferCopy {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_src_offset(mut self, value: VkDeviceSize) -> Self {
    self.srcOffset = value;
    self
  }
  #[inline]
  pub fn set_dst_offset(mut self, value: VkDeviceSize) -> Self {
    self.dstOffset = value;
    self
  }
  #[inline]
  pub fn set_size(mut self, value: VkDeviceSize) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn src_offset(&self) -> VkDeviceSize {
    self.srcOffset
  }
  #[inline]
  pub fn dst_offset(&self) -> VkDeviceSize {
    self.dstOffset
  }
  #[inline]
  pub fn size(&self) -> VkDeviceSize {
    self.size
  }
}
impl Default for VkBufferCopy {
  fn default() -> VkBufferCopy {
    VkBufferCopy::new()
  }
}
unsafe impl Struct for VkBufferCopy {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_copy() {
  assert_size!(24, VkBufferCopy);
}

/// Structure specifying a image subresource layers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceLayers {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
impl VkImageSubresourceLayers {
  #[inline]
  pub fn new() -> VkImageSubresourceLayers {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_aspect_mask(mut self, value: VkImageAspectFlags) -> Self {
    self.aspectMask = value;
    self
  }
  #[inline]
  pub fn set_mip_level(mut self, value: u32) -> Self {
    self.mipLevel = value;
    self
  }
  #[inline]
  pub fn set_base_array_layer(mut self, value: u32) -> Self {
    self.baseArrayLayer = value;
    self
  }
  #[inline]
  pub fn set_layer_count(mut self, value: u32) -> Self {
    self.layerCount = value;
    self
  }
  #[inline]
  pub fn aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn mip_level(&self) -> u32 {
    self.mipLevel
  }
  #[inline]
  pub fn base_array_layer(&self) -> u32 {
    self.baseArrayLayer
  }
  #[inline]
  pub fn layer_count(&self) -> u32 {
    self.layerCount
  }
}
impl Default for VkImageSubresourceLayers {
  fn default() -> VkImageSubresourceLayers {
    VkImageSubresourceLayers::new()
  }
}
unsafe impl Struct for VkImageSubresourceLayers {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_subresource_layers() {
  assert_size!(16, VkImageSubresourceLayers);
}

/// Structure specifying an image copy operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCopy {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
}
impl VkImageCopy {
  #[inline]
  pub fn new() -> VkImageCopy {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_src_subresource(mut self, value: VkImageSubresourceLayers) -> Self {
    self.srcSubresource = value;
    self
  }
  #[inline]
  pub fn set_src_offset(mut self, value: VkOffset3D) -> Self {
    self.srcOffset = value;
    self
  }
  #[inline]
  pub fn set_dst_subresource(mut self, value: VkImageSubresourceLayers) -> Self {
    self.dstSubresource = value;
    self
  }
  #[inline]
  pub fn set_dst_offset(mut self, value: VkOffset3D) -> Self {
    self.dstOffset = value;
    self
  }
  #[inline]
  pub fn set_extent(mut self, value: VkExtent3D) -> Self {
    self.extent = value;
    self
  }
  #[inline]
  pub fn src_subresource(&self) -> &VkImageSubresourceLayers {
    &self.srcSubresource
  }
  #[inline]
  pub fn src_offset(&self) -> &VkOffset3D {
    &self.srcOffset
  }
  #[inline]
  pub fn dst_subresource(&self) -> &VkImageSubresourceLayers {
    &self.dstSubresource
  }
  #[inline]
  pub fn dst_offset(&self) -> &VkOffset3D {
    &self.dstOffset
  }
  #[inline]
  pub fn extent(&self) -> &VkExtent3D {
    &self.extent
  }
}
impl Default for VkImageCopy {
  fn default() -> VkImageCopy {
    VkImageCopy::new()
  }
}
unsafe impl Struct for VkImageCopy {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_copy() {
  assert_size!(68, VkImageCopy);
}

/// Structure specifying an image blit operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageBlit {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffsets: [VkOffset3D; 2],
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffsets: [VkOffset3D; 2],
}
impl VkImageBlit {
  #[inline]
  pub fn new() -> VkImageBlit {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_src_subresource(mut self, value: VkImageSubresourceLayers) -> Self {
    self.srcSubresource = value;
    self
  }
  #[inline]
  pub fn set_src_offsets(mut self, value: [VkOffset3D; 2]) -> Self {
    self.srcOffsets = value;
    self
  }
  #[inline]
  pub fn set_dst_subresource(mut self, value: VkImageSubresourceLayers) -> Self {
    self.dstSubresource = value;
    self
  }
  #[inline]
  pub fn set_dst_offsets(mut self, value: [VkOffset3D; 2]) -> Self {
    self.dstOffsets = value;
    self
  }
  #[inline]
  pub fn src_subresource(&self) -> &VkImageSubresourceLayers {
    &self.srcSubresource
  }
  #[inline]
  pub fn src_offsets(&self) -> [VkOffset3D; 2] {
    self.srcOffsets
  }
  #[inline]
  pub fn dst_subresource(&self) -> &VkImageSubresourceLayers {
    &self.dstSubresource
  }
  #[inline]
  pub fn dst_offsets(&self) -> [VkOffset3D; 2] {
    self.dstOffsets
  }
}
impl Default for VkImageBlit {
  fn default() -> VkImageBlit {
    VkImageBlit::new()
  }
}
unsafe impl Struct for VkImageBlit {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_blit() {
  assert_size!(80, VkImageBlit);
}

/// Structure specifying a buffer image copy operation
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
impl VkBufferImageCopy {
  #[inline]
  pub fn new() -> VkBufferImageCopy {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_buffer_offset(mut self, value: VkDeviceSize) -> Self {
    self.bufferOffset = value;
    self
  }
  #[inline]
  pub fn set_buffer_row_length(mut self, value: u32) -> Self {
    self.bufferRowLength = value;
    self
  }
  #[inline]
  pub fn set_buffer_image_height(mut self, value: u32) -> Self {
    self.bufferImageHeight = value;
    self
  }
  #[inline]
  pub fn set_image_subresource(mut self, value: VkImageSubresourceLayers) -> Self {
    self.imageSubresource = value;
    self
  }
  #[inline]
  pub fn set_image_offset(mut self, value: VkOffset3D) -> Self {
    self.imageOffset = value;
    self
  }
  #[inline]
  pub fn set_image_extent(mut self, value: VkExtent3D) -> Self {
    self.imageExtent = value;
    self
  }
  #[inline]
  pub fn buffer_offset(&self) -> VkDeviceSize {
    self.bufferOffset
  }
  #[inline]
  pub fn buffer_row_length(&self) -> u32 {
    self.bufferRowLength
  }
  #[inline]
  pub fn buffer_image_height(&self) -> u32 {
    self.bufferImageHeight
  }
  #[inline]
  pub fn image_subresource(&self) -> &VkImageSubresourceLayers {
    &self.imageSubresource
  }
  #[inline]
  pub fn image_offset(&self) -> &VkOffset3D {
    &self.imageOffset
  }
  #[inline]
  pub fn image_extent(&self) -> &VkExtent3D {
    &self.imageExtent
  }
}
impl Default for VkBufferImageCopy {
  fn default() -> VkBufferImageCopy {
    VkBufferImageCopy::new()
  }
}
unsafe impl Struct for VkBufferImageCopy {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_image_copy() {
  assert_size!(56, VkBufferImageCopy);
}

/// Structure specifying a clear color value
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
  pub float32: [f32; 4],
  pub int32: [i32; 4],
  pub uint32: [u32; 4],
}
unsafe impl Struct for VkClearColorValue {}
#[cfg(test)]
#[test]
fn test_union_size_vk_clear_color_value() {
  assert_size!(16, VkClearColorValue);
}

/// Structure specifying a clear depth stencil value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearDepthStencilValue {
  pub depth: f32,
  pub stencil: u32,
}
impl VkClearDepthStencilValue {
  #[inline]
  pub fn new() -> VkClearDepthStencilValue {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_depth(mut self, value: f32) -> Self {
    self.depth = value;
    self
  }
  #[inline]
  pub fn set_stencil(mut self, value: u32) -> Self {
    self.stencil = value;
    self
  }
  #[inline]
  pub fn depth(&self) -> f32 {
    self.depth
  }
  #[inline]
  pub fn stencil(&self) -> u32 {
    self.stencil
  }
}
impl Default for VkClearDepthStencilValue {
  fn default() -> VkClearDepthStencilValue {
    VkClearDepthStencilValue::new()
  }
}
unsafe impl Struct for VkClearDepthStencilValue {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_clear_depth_stencil_value() {
  assert_size!(8, VkClearDepthStencilValue);
}

/// Structure specifying a clear value
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
  pub color: VkClearColorValue,
  pub depthStencil: VkClearDepthStencilValue,
}
unsafe impl Struct for VkClearValue {}
#[cfg(test)]
#[test]
fn test_union_size_vk_clear_value() {
  assert_size!(16, VkClearValue);
}

/// Structure specifying a clear attachment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearAttachment {
  pub aspectMask: VkImageAspectFlags,
  pub colorAttachment: u32,
  pub clearValue: VkClearValue,
}
impl VkClearAttachment {
  #[inline]
  pub fn new() -> VkClearAttachment {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_aspect_mask(mut self, value: VkImageAspectFlags) -> Self {
    self.aspectMask = value;
    self
  }
  #[inline]
  pub fn set_color_attachment(mut self, value: u32) -> Self {
    self.colorAttachment = value;
    self
  }
  #[inline]
  pub fn set_clear_value(mut self, value: VkClearValue) -> Self {
    self.clearValue = value;
    self
  }
  #[inline]
  pub fn aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn color_attachment(&self) -> u32 {
    self.colorAttachment
  }
  #[inline]
  pub fn clear_value(&self) -> &VkClearValue {
    &self.clearValue
  }
}
impl Default for VkClearAttachment {
  fn default() -> VkClearAttachment {
    VkClearAttachment::new()
  }
}
unsafe impl Struct for VkClearAttachment {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_clear_attachment() {
  assert_size!(24, VkClearAttachment);
}

/// Structure specifying a clear rectangle
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearRect {
  pub rect: VkRect2D,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
impl VkClearRect {
  #[inline]
  pub fn new() -> VkClearRect {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_rect(mut self, value: VkRect2D) -> Self {
    self.rect = value;
    self
  }
  #[inline]
  pub fn set_base_array_layer(mut self, value: u32) -> Self {
    self.baseArrayLayer = value;
    self
  }
  #[inline]
  pub fn set_layer_count(mut self, value: u32) -> Self {
    self.layerCount = value;
    self
  }
  #[inline]
  pub fn rect(&self) -> &VkRect2D {
    &self.rect
  }
  #[inline]
  pub fn base_array_layer(&self) -> u32 {
    self.baseArrayLayer
  }
  #[inline]
  pub fn layer_count(&self) -> u32 {
    self.layerCount
  }
}
impl Default for VkClearRect {
  fn default() -> VkClearRect {
    VkClearRect::new()
  }
}
unsafe impl Struct for VkClearRect {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_clear_rect() {
  assert_size!(24, VkClearRect);
}

/// Structure specifying an image resolve operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageResolve {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
}
impl VkImageResolve {
  #[inline]
  pub fn new() -> VkImageResolve {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_src_subresource(mut self, value: VkImageSubresourceLayers) -> Self {
    self.srcSubresource = value;
    self
  }
  #[inline]
  pub fn set_src_offset(mut self, value: VkOffset3D) -> Self {
    self.srcOffset = value;
    self
  }
  #[inline]
  pub fn set_dst_subresource(mut self, value: VkImageSubresourceLayers) -> Self {
    self.dstSubresource = value;
    self
  }
  #[inline]
  pub fn set_dst_offset(mut self, value: VkOffset3D) -> Self {
    self.dstOffset = value;
    self
  }
  #[inline]
  pub fn set_extent(mut self, value: VkExtent3D) -> Self {
    self.extent = value;
    self
  }
  #[inline]
  pub fn src_subresource(&self) -> &VkImageSubresourceLayers {
    &self.srcSubresource
  }
  #[inline]
  pub fn src_offset(&self) -> &VkOffset3D {
    &self.srcOffset
  }
  #[inline]
  pub fn dst_subresource(&self) -> &VkImageSubresourceLayers {
    &self.dstSubresource
  }
  #[inline]
  pub fn dst_offset(&self) -> &VkOffset3D {
    &self.dstOffset
  }
  #[inline]
  pub fn extent(&self) -> &VkExtent3D {
    &self.extent
  }
}
impl Default for VkImageResolve {
  fn default() -> VkImageResolve {
    VkImageResolve::new()
  }
}
unsafe impl Struct for VkImageResolve {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_resolve() {
  assert_size!(68, VkImageResolve);
}

/// Structure specifying render pass begin info
#[repr(C)]
pub struct VkRenderPassBeginInfo<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  renderPass: u64,
  framebuffer: u64,
  pub renderArea: VkRect2D,
  clearValueCount: u32,
  pClearValues: *const VkClearValue,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
impl<'l, 'h: 'l> VkRenderPassBeginInfo<'l, 'h> {
  #[inline]
  pub fn new() -> VkRenderPassBeginInfo<'l, 'h> {
    unsafe {
      VkRenderPassBeginInfo {
        sType: VkStructureType::RENDER_PASS_BEGIN_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_render_pass(mut self, value: VkRenderPass<'h>) -> Self {
    unsafe {
      self.renderPass = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_framebuffer(mut self, value: VkFramebuffer<'h>) -> Self {
    unsafe {
      self.framebuffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_render_area(mut self, value: VkRect2D) -> Self {
    self.renderArea = value;
    self
  }
  #[inline]
  pub fn set_clear_values(mut self, value: &'l [VkClearValue]) -> Self {
    self.clearValueCount = value.len() as u32;
    unsafe {
      self.pClearValues = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn render_area(&self) -> &VkRect2D {
    &self.renderArea
  }
  #[inline]
  pub fn clear_value_count(&self) -> u32 {
    self.clearValueCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
impl<'l, 'h: 'l> Default for VkRenderPassBeginInfo<'l, 'h> {
  fn default() -> VkRenderPassBeginInfo<'l, 'h> {
    VkRenderPassBeginInfo::new()
  }
}
unsafe impl<'l, 'h: 'l> Struct for VkRenderPassBeginInfo<'l, 'h> {}
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_begin_info() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 4, VkRenderPassBeginInfo);
}

/// Specify how commands in the first subpass of a render pass are provided
pub use enums::VkSubpassContents;

// feature: VK_KHR_surface
#[cfg(feature = "VK_KHR_surface")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSurfaceKHR__ {}

/// Opaque handle to a surface object
#[cfg(feature = "VK_KHR_surface")]
pub type VkSurfaceKHR<'l> = VkNonDispatchableHandle<'l, VkSurfaceKHR__>;

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

/// Structure describing capabilities of a surface
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
#[cfg(feature = "VK_KHR_surface")]
impl VkSurfaceCapabilitiesKHR {
  #[inline]
  pub fn min_image_count(&self) -> u32 {
    self.minImageCount
  }
  #[inline]
  pub fn max_image_count(&self) -> u32 {
    self.maxImageCount
  }
  #[inline]
  pub fn current_extent(&self) -> &VkExtent2D {
    &self.currentExtent
  }
  #[inline]
  pub fn min_image_extent(&self) -> &VkExtent2D {
    &self.minImageExtent
  }
  #[inline]
  pub fn max_image_extent(&self) -> &VkExtent2D {
    &self.maxImageExtent
  }
  #[inline]
  pub fn max_image_array_layers(&self) -> u32 {
    self.maxImageArrayLayers
  }
  #[inline]
  pub fn supported_transforms(&self) -> VkSurfaceTransformFlagsKHR {
    self.supportedTransforms
  }
  #[inline]
  pub fn current_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
    self.currentTransform
  }
  #[inline]
  pub fn supported_composite_alpha(&self) -> VkCompositeAlphaFlagsKHR {
    self.supportedCompositeAlpha
  }
  #[inline]
  pub fn supported_usage_flags(&self) -> VkImageUsageFlags {
    self.supportedUsageFlags
  }
}
#[cfg(feature = "VK_KHR_surface")]
unsafe impl Struct for VkSurfaceCapabilitiesKHR {}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_capabilities_khr() {
  assert_size!(52, VkSurfaceCapabilitiesKHR);
}

/// supported color space of the presentation engine
#[cfg(feature = "VK_KHR_surface")]
pub use enums::VkColorSpaceKHR;

/// Structure describing a supported swapchain format-color space pair
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_surface")]
pub struct VkSurfaceFormatKHR {
  pub format: VkFormat,
  pub colorSpace: VkColorSpaceKHR,
}
#[cfg(feature = "VK_KHR_surface")]
impl VkSurfaceFormatKHR {
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn color_space(&self) -> VkColorSpaceKHR {
    self.colorSpace
  }
}
#[cfg(feature = "VK_KHR_surface")]
unsafe impl Struct for VkSurfaceFormatKHR {}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_format_khr() {
  assert_size!(8, VkSurfaceFormatKHR);
}

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
#[cfg(feature = "VK_KHR_swapchain")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSwapchainKHR__ {}

/// Opaque handle to a swapchain object
#[cfg(feature = "VK_KHR_swapchain")]
pub type VkSwapchainKHR<'l> = VkNonDispatchableHandle<'l, VkSwapchainKHR__>;

/// Structure specifying parameters of a newly created swapchain object
#[repr(C)]
#[cfg(feature = "VK_KHR_swapchain")]
pub struct VkSwapchainCreateInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkSwapchainCreateFlagsKHR,
  surface: u64,
  pub minImageCount: u32,
  pub imageFormat: VkFormat,
  pub imageColorSpace: VkColorSpaceKHR,
  pub imageExtent: VkExtent2D,
  pub imageArrayLayers: u32,
  pub imageUsage: VkImageUsageFlags,
  pub imageSharingMode: VkSharingMode,
  queueFamilyIndexCount: u32,
  pQueueFamilyIndices: *const u32,
  pub preTransform: VkSurfaceTransformFlagBitsKHR,
  pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
  pub presentMode: VkPresentModeKHR,
  clipped: VkBool32,
  oldSwapchain: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'l, 'h: 'l> VkSwapchainCreateInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkSwapchainCreateInfoKHR<'l, 'h> {
    unsafe {
      VkSwapchainCreateInfoKHR {
        sType: VkStructureType::SWAPCHAIN_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSwapchainCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_surface(mut self, value: VkSurfaceKHR<'h>) -> Self {
    unsafe {
      self.surface = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_min_image_count(mut self, value: u32) -> Self {
    self.minImageCount = value;
    self
  }
  #[inline]
  pub fn set_image_format(mut self, value: VkFormat) -> Self {
    self.imageFormat = value;
    self
  }
  #[inline]
  pub fn set_image_color_space(mut self, value: VkColorSpaceKHR) -> Self {
    self.imageColorSpace = value;
    self
  }
  #[inline]
  pub fn set_image_extent(mut self, value: VkExtent2D) -> Self {
    self.imageExtent = value;
    self
  }
  #[inline]
  pub fn set_image_array_layers(mut self, value: u32) -> Self {
    self.imageArrayLayers = value;
    self
  }
  #[inline]
  pub fn set_image_usage(mut self, value: VkImageUsageFlags) -> Self {
    self.imageUsage = value;
    self
  }
  #[inline]
  pub fn set_image_sharing_mode(mut self, value: VkSharingMode) -> Self {
    self.imageSharingMode = value;
    self
  }
  #[inline]
  pub fn set_queue_family_indices(mut self, value: &'l [u32]) -> Self {
    self.queueFamilyIndexCount = value.len() as u32;
    unsafe {
      self.pQueueFamilyIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_pre_transform(mut self, value: VkSurfaceTransformFlagBitsKHR) -> Self {
    self.preTransform = value;
    self
  }
  #[inline]
  pub fn set_composite_alpha(mut self, value: VkCompositeAlphaFlagBitsKHR) -> Self {
    self.compositeAlpha = value;
    self
  }
  #[inline]
  pub fn set_present_mode(mut self, value: VkPresentModeKHR) -> Self {
    self.presentMode = value;
    self
  }
  #[inline]
  pub fn set_clipped(mut self, value: bool) -> Self {
    unsafe {
      self.clipped = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_old_swapchain(mut self, value: Option<VkSwapchainKHR<'h>>) -> Self {
    unsafe {
      self.oldSwapchain = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkSwapchainCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn min_image_count(&self) -> u32 {
    self.minImageCount
  }
  #[inline]
  pub fn image_format(&self) -> VkFormat {
    self.imageFormat
  }
  #[inline]
  pub fn image_color_space(&self) -> VkColorSpaceKHR {
    self.imageColorSpace
  }
  #[inline]
  pub fn image_extent(&self) -> &VkExtent2D {
    &self.imageExtent
  }
  #[inline]
  pub fn image_array_layers(&self) -> u32 {
    self.imageArrayLayers
  }
  #[inline]
  pub fn image_usage(&self) -> VkImageUsageFlags {
    self.imageUsage
  }
  #[inline]
  pub fn image_sharing_mode(&self) -> VkSharingMode {
    self.imageSharingMode
  }
  #[inline]
  pub fn queue_family_index_count(&self) -> u32 {
    self.queueFamilyIndexCount
  }
  #[inline]
  pub fn pre_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
    self.preTransform
  }
  #[inline]
  pub fn composite_alpha(&self) -> VkCompositeAlphaFlagBitsKHR {
    self.compositeAlpha
  }
  #[inline]
  pub fn present_mode(&self) -> VkPresentModeKHR {
    self.presentMode
  }
  #[inline]
  pub fn is_clipped(&self) -> bool {
    self.clipped != 0
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'l, 'h: 'l> Default for VkSwapchainCreateInfoKHR<'l, 'h> {
  fn default() -> VkSwapchainCreateInfoKHR<'l, 'h> {
    VkSwapchainCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<'l, 'h: 'l> Struct for VkSwapchainCreateInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_swapchain_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(64 + ptr_size * 5, VkSwapchainCreateInfoKHR);
}

/// Structure describing parameters of a queue presentation
#[repr(C)]
#[cfg(feature = "VK_KHR_swapchain")]
pub struct VkPresentInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreCount: u32,
  pWaitSemaphores: *const u64,
  swapchainCount: u32,
  pSwapchains: *const u64,
  pImageIndices: *const u32,
  pResults: *mut VkResult,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'l, 'h: 'l> VkPresentInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkPresentInfoKHR<'l, 'h> {
    unsafe {
      VkPresentInfoKHR {
        sType: VkStructureType::PRESENT_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_wait_semaphores(mut self, value: &'l [VkSemaphore<'h>]) -> Self {
    self.waitSemaphoreCount = value.len() as u32;
    unsafe {
      self.pWaitSemaphores = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn wait_semaphore_count(&self) -> u32 {
    self.waitSemaphoreCount
  }
  #[inline]
  pub fn swapchain_count(&self) -> u32 {
    self.swapchainCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'l, 'h: 'l> Default for VkPresentInfoKHR<'l, 'h> {
  fn default() -> VkPresentInfoKHR<'l, 'h> {
    VkPresentInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<'l, 'h: 'l> Struct for VkPresentInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 8, VkPresentInfoKHR);
}

// feature: VK_KHR_display

/// Alpha blending type
#[cfg(feature = "VK_KHR_display")]
pub use enums::VkDisplayPlaneAlphaFlagBitsKHR;

/// Bitmask of VkDisplayPlaneAlphaFlagBitsKHR
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagBitsKHR;
#[cfg(feature = "VK_KHR_display")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDisplayKHR__ {}

/// Opaque handle to a display object
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayKHR<'l> = VkNonDispatchableHandle<'l, VkDisplayKHR__>;

/// Structure describing an available display device
#[repr(C)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayPropertiesKHR<'l, 'h: 'l> {
  display: u64,
  displayName: *const c_char,
  pub physicalDimensions: VkExtent2D,
  pub physicalResolution: VkExtent2D,
  pub supportedTransforms: VkSurfaceTransformFlagsKHR,
  planeReorderPossible: VkBool32,
  persistentContent: VkBool32,
  _p: ::std::marker::PhantomData<(&'h u8, &'l u8)>,
}
#[cfg(feature = "VK_KHR_display")]
impl<'l, 'h: 'l> VkDisplayPropertiesKHR<'l, 'h> {
  #[inline]
  pub fn display_name(&self) -> &CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.displayName) }
  }
  #[inline]
  pub fn physical_dimensions(&self) -> &VkExtent2D {
    &self.physicalDimensions
  }
  #[inline]
  pub fn physical_resolution(&self) -> &VkExtent2D {
    &self.physicalResolution
  }
  #[inline]
  pub fn supported_transforms(&self) -> VkSurfaceTransformFlagsKHR {
    self.supportedTransforms
  }
  #[inline]
  pub fn is_plane_reorder_possible(&self) -> bool {
    self.planeReorderPossible != 0
  }
  #[inline]
  pub fn is_persistent_content(&self) -> bool {
    self.persistentContent != 0
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl<'l, 'h: 'l> Struct for VkDisplayPropertiesKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 2, VkDisplayPropertiesKHR);
}

/// Structure describing display parameters associated with a display mode
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModeParametersKHR {
  pub visibleRegion: VkExtent2D,
  pub refreshRate: u32,
}
#[cfg(feature = "VK_KHR_display")]
impl VkDisplayModeParametersKHR {
  #[inline]
  pub fn new() -> VkDisplayModeParametersKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_visible_region(mut self, value: VkExtent2D) -> Self {
    self.visibleRegion = value;
    self
  }
  #[inline]
  pub fn set_refresh_rate(mut self, value: u32) -> Self {
    self.refreshRate = value;
    self
  }
  #[inline]
  pub fn visible_region(&self) -> &VkExtent2D {
    &self.visibleRegion
  }
  #[inline]
  pub fn refresh_rate(&self) -> u32 {
    self.refreshRate
  }
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayModeParametersKHR {
  fn default() -> VkDisplayModeParametersKHR {
    VkDisplayModeParametersKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl Struct for VkDisplayModeParametersKHR {}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_mode_parameters_khr() {
  assert_size!(12, VkDisplayModeParametersKHR);
}
#[cfg(feature = "VK_KHR_display")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDisplayModeKHR__ {}

/// Opaque handle to a display mode object
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayModeKHR<'l> = VkNonDispatchableHandle<'l, VkDisplayModeKHR__>;

/// Structure describing display mode properties
#[repr(C)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModePropertiesKHR<'h> {
  displayMode: u64,
  pub parameters: VkDisplayModeParametersKHR,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_KHR_display")]
impl<'h> VkDisplayModePropertiesKHR<'h> {
  #[inline]
  pub fn parameters(&self) -> &VkDisplayModeParametersKHR {
    &self.parameters
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl<'h> Struct for VkDisplayModePropertiesKHR<'h> {}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_mode_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 1, VkDisplayModePropertiesKHR);
}
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayModeCreateFlagsKHR = VkFlags;

/// Structure specifying parameters of a newly created display mode object
#[repr(C)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModeCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDisplayModeCreateFlagsKHR,
  pub parameters: VkDisplayModeParametersKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_display")]
impl<'l> VkDisplayModeCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkDisplayModeCreateInfoKHR<'l> {
    unsafe {
      VkDisplayModeCreateInfoKHR {
        sType: VkStructureType::DISPLAY_MODE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDisplayModeCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_parameters(mut self, value: VkDisplayModeParametersKHR) -> Self {
    self.parameters = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkDisplayModeCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn parameters(&self) -> &VkDisplayModeParametersKHR {
    &self.parameters
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_display")]
impl<'l> Default for VkDisplayModeCreateInfoKHR<'l> {
  fn default() -> VkDisplayModeCreateInfoKHR<'l> {
    VkDisplayModeCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl<'l> Struct for VkDisplayModeCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_mode_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 2, VkDisplayModeCreateInfoKHR);
}

/// Structure describing capabilities of a mode and plane combination
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
#[cfg(feature = "VK_KHR_display")]
impl VkDisplayPlaneCapabilitiesKHR {
  #[inline]
  pub fn supported_alpha(&self) -> VkDisplayPlaneAlphaFlagsKHR {
    self.supportedAlpha
  }
  #[inline]
  pub fn min_src_position(&self) -> &VkOffset2D {
    &self.minSrcPosition
  }
  #[inline]
  pub fn max_src_position(&self) -> &VkOffset2D {
    &self.maxSrcPosition
  }
  #[inline]
  pub fn min_src_extent(&self) -> &VkExtent2D {
    &self.minSrcExtent
  }
  #[inline]
  pub fn max_src_extent(&self) -> &VkExtent2D {
    &self.maxSrcExtent
  }
  #[inline]
  pub fn min_dst_position(&self) -> &VkOffset2D {
    &self.minDstPosition
  }
  #[inline]
  pub fn max_dst_position(&self) -> &VkOffset2D {
    &self.maxDstPosition
  }
  #[inline]
  pub fn min_dst_extent(&self) -> &VkExtent2D {
    &self.minDstExtent
  }
  #[inline]
  pub fn max_dst_extent(&self) -> &VkExtent2D {
    &self.maxDstExtent
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl Struct for VkDisplayPlaneCapabilitiesKHR {}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_plane_capabilities_khr() {
  assert_size!(68, VkDisplayPlaneCapabilitiesKHR);
}

/// Structure describing display plane properties
#[repr(C)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayPlanePropertiesKHR<'h> {
  currentDisplay: u64,
  pub currentStackIndex: u32,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_KHR_display")]
impl<'h> VkDisplayPlanePropertiesKHR<'h> {
  #[inline]
  pub fn current_stack_index(&self) -> u32 {
    self.currentStackIndex
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl<'h> Struct for VkDisplayPlanePropertiesKHR<'h> {}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_plane_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 1, VkDisplayPlanePropertiesKHR);
}
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;

/// Structure specifying parameters of a newly created display plane surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplaySurfaceCreateInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDisplaySurfaceCreateFlagsKHR,
  displayMode: u64,
  pub planeIndex: u32,
  pub planeStackIndex: u32,
  pub transform: VkSurfaceTransformFlagBitsKHR,
  pub globalAlpha: f32,
  pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
  pub imageExtent: VkExtent2D,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_display")]
impl<'l, 'h: 'l> VkDisplaySurfaceCreateInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkDisplaySurfaceCreateInfoKHR<'l, 'h> {
    unsafe {
      VkDisplaySurfaceCreateInfoKHR {
        sType: VkStructureType::DISPLAY_SURFACE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDisplaySurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_display_mode(mut self, value: VkDisplayModeKHR<'h>) -> Self {
    unsafe {
      self.displayMode = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_plane_index(mut self, value: u32) -> Self {
    self.planeIndex = value;
    self
  }
  #[inline]
  pub fn set_plane_stack_index(mut self, value: u32) -> Self {
    self.planeStackIndex = value;
    self
  }
  #[inline]
  pub fn set_transform(mut self, value: VkSurfaceTransformFlagBitsKHR) -> Self {
    self.transform = value;
    self
  }
  #[inline]
  pub fn set_global_alpha(mut self, value: f32) -> Self {
    self.globalAlpha = value;
    self
  }
  #[inline]
  pub fn set_alpha_mode(mut self, value: VkDisplayPlaneAlphaFlagBitsKHR) -> Self {
    self.alphaMode = value;
    self
  }
  #[inline]
  pub fn set_image_extent(mut self, value: VkExtent2D) -> Self {
    self.imageExtent = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkDisplaySurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn plane_index(&self) -> u32 {
    self.planeIndex
  }
  #[inline]
  pub fn plane_stack_index(&self) -> u32 {
    self.planeStackIndex
  }
  #[inline]
  pub fn transform(&self) -> VkSurfaceTransformFlagBitsKHR {
    self.transform
  }
  #[inline]
  pub fn global_alpha(&self) -> f32 {
    self.globalAlpha
  }
  #[inline]
  pub fn alpha_mode(&self) -> VkDisplayPlaneAlphaFlagBitsKHR {
    self.alphaMode
  }
  #[inline]
  pub fn image_extent(&self) -> &VkExtent2D {
    &self.imageExtent
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_display")]
impl<'l, 'h: 'l> Default for VkDisplaySurfaceCreateInfoKHR<'l, 'h> {
  fn default() -> VkDisplaySurfaceCreateInfoKHR<'l, 'h> {
    VkDisplaySurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl<'l, 'h: 'l> Struct for VkDisplaySurfaceCreateInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_surface_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 4, VkDisplaySurfaceCreateInfoKHR);
}

// feature: VK_KHR_display_swapchain

/// Structure describing parameters of a queue presentation to a swapchain
#[repr(C)]
#[cfg(feature = "VK_KHR_display_swapchain")]
pub struct VkDisplayPresentInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcRect: VkRect2D,
  pub dstRect: VkRect2D,
  persistent: VkBool32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl<'l> VkDisplayPresentInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkDisplayPresentInfoKHR<'l> {
    unsafe {
      VkDisplayPresentInfoKHR {
        sType: VkStructureType::DISPLAY_PRESENT_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_rect(mut self, value: VkRect2D) -> Self {
    self.srcRect = value;
    self
  }
  #[inline]
  pub fn set_dst_rect(mut self, value: VkRect2D) -> Self {
    self.dstRect = value;
    self
  }
  #[inline]
  pub fn set_persistent(mut self, value: bool) -> Self {
    unsafe {
      self.persistent = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn src_rect(&self) -> &VkRect2D {
    &self.srcRect
  }
  #[inline]
  pub fn dst_rect(&self) -> &VkRect2D {
    &self.dstRect
  }
  #[inline]
  pub fn is_persistent(&self) -> bool {
    self.persistent != 0
  }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl<'l> Default for VkDisplayPresentInfoKHR<'l> {
  fn default() -> VkDisplayPresentInfoKHR<'l> {
    VkDisplayPresentInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
unsafe impl<'l> Struct for VkDisplayPresentInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_display_swapchain")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkPresentInfoKHR<'m, 'h>> for VkDisplayPresentInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDisplayPresentInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_present_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 3, VkDisplayPresentInfoKHR);
}

// feature: VK_KHR_xlib_surface
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;

/// Structure specifying parameters of a newly created Xlib surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub struct VkXlibSurfaceCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkXlibSurfaceCreateFlagsKHR,
  pub dpy: *mut wsi::xlib::Display,
  pub window: wsi::xlib::Window,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
impl<'l> VkXlibSurfaceCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkXlibSurfaceCreateInfoKHR<'l> {
    unsafe {
      VkXlibSurfaceCreateInfoKHR {
        sType: VkStructureType::XLIB_SURFACE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkXlibSurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_dpy(mut self, value: *mut wsi::xlib::Display) -> Self {
    self.dpy = value;
    self
  }
  #[inline]
  pub fn set_window(mut self, value: wsi::xlib::Window) -> Self {
    self.window = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkXlibSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn dpy(&self) -> *mut wsi::xlib::Display {
    self.dpy
  }
  #[inline]
  pub fn window(&self) -> wsi::xlib::Window {
    self.window
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
impl<'l> Default for VkXlibSurfaceCreateInfoKHR<'l> {
  fn default() -> VkXlibSurfaceCreateInfoKHR<'l> {
    VkXlibSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
unsafe impl<'l> Struct for VkXlibSurfaceCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_xlib_surface_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkXlibSurfaceCreateInfoKHR);
}

// feature: VK_KHR_xcb_surface
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;

/// Structure specifying parameters of a newly created Xcb surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub struct VkXcbSurfaceCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkXcbSurfaceCreateFlagsKHR,
  pub connection: *mut wsi::xcb::xcb_connection_t,
  pub window: wsi::xcb::xcb_window_t,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
impl<'l> VkXcbSurfaceCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkXcbSurfaceCreateInfoKHR<'l> {
    unsafe {
      VkXcbSurfaceCreateInfoKHR {
        sType: VkStructureType::XCB_SURFACE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkXcbSurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_connection(mut self, value: *mut wsi::xcb::xcb_connection_t) -> Self {
    self.connection = value;
    self
  }
  #[inline]
  pub fn set_window(mut self, value: wsi::xcb::xcb_window_t) -> Self {
    self.window = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkXcbSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn connection(&self) -> *mut wsi::xcb::xcb_connection_t {
    self.connection
  }
  #[inline]
  pub fn window(&self) -> wsi::xcb::xcb_window_t {
    self.window
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
impl<'l> Default for VkXcbSurfaceCreateInfoKHR<'l> {
  fn default() -> VkXcbSurfaceCreateInfoKHR<'l> {
    VkXcbSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
unsafe impl<'l> Struct for VkXcbSurfaceCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_xcb_surface_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkXcbSurfaceCreateInfoKHR);
}

// feature: VK_KHR_wayland_surface
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;

/// Structure specifying parameters of a newly created Wayland surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub struct VkWaylandSurfaceCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkWaylandSurfaceCreateFlagsKHR,
  pub display: *mut wsi::wayland::wl_display,
  pub surface: *mut wsi::wayland::wl_surface,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
impl<'l> VkWaylandSurfaceCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkWaylandSurfaceCreateInfoKHR<'l> {
    unsafe {
      VkWaylandSurfaceCreateInfoKHR {
        sType: VkStructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkWaylandSurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_display(mut self, value: *mut wsi::wayland::wl_display) -> Self {
    self.display = value;
    self
  }
  #[inline]
  pub fn set_surface(mut self, value: *mut wsi::wayland::wl_surface) -> Self {
    self.surface = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkWaylandSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn display(&self) -> *mut wsi::wayland::wl_display {
    self.display
  }
  #[inline]
  pub fn surface(&self) -> *mut wsi::wayland::wl_surface {
    self.surface
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
impl<'l> Default for VkWaylandSurfaceCreateInfoKHR<'l> {
  fn default() -> VkWaylandSurfaceCreateInfoKHR<'l> {
    VkWaylandSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
unsafe impl<'l> Struct for VkWaylandSurfaceCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_wayland_surface_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkWaylandSurfaceCreateInfoKHR);
}

// feature: VK_KHR_mir_surface
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub type VkMirSurfaceCreateFlagsKHR = VkFlags;

/// Structure specifying parameters of a newly created Mir surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub struct VkMirSurfaceCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkMirSurfaceCreateFlagsKHR,
  pub connection: *mut wsi::mir::MirConnection,
  pub mirSurface: *mut wsi::mir::MirSurface,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
impl<'l> VkMirSurfaceCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkMirSurfaceCreateInfoKHR<'l> {
    unsafe {
      VkMirSurfaceCreateInfoKHR {
        sType: VkStructureType::MIR_SURFACE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkMirSurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_connection(mut self, value: *mut wsi::mir::MirConnection) -> Self {
    self.connection = value;
    self
  }
  #[inline]
  pub fn set_mir_surface(mut self, value: *mut wsi::mir::MirSurface) -> Self {
    self.mirSurface = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkMirSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn connection(&self) -> *mut wsi::mir::MirConnection {
    self.connection
  }
  #[inline]
  pub fn mir_surface(&self) -> *mut wsi::mir::MirSurface {
    self.mirSurface
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
impl<'l> Default for VkMirSurfaceCreateInfoKHR<'l> {
  fn default() -> VkMirSurfaceCreateInfoKHR<'l> {
    VkMirSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
unsafe impl<'l> Struct for VkMirSurfaceCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_mir_surface_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkMirSurfaceCreateInfoKHR);
}

// feature: VK_KHR_android_surface
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub type VkAndroidSurfaceCreateFlagsKHR = VkFlags;

/// Structure specifying parameters of a newly created Android surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub struct VkAndroidSurfaceCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkAndroidSurfaceCreateFlagsKHR,
  pub window: *mut wsi::android::ANativeWindow,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
impl<'l> VkAndroidSurfaceCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkAndroidSurfaceCreateInfoKHR<'l> {
    unsafe {
      VkAndroidSurfaceCreateInfoKHR {
        sType: VkStructureType::ANDROID_SURFACE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkAndroidSurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_window(mut self, value: *mut wsi::android::ANativeWindow) -> Self {
    self.window = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkAndroidSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn window(&self) -> *mut wsi::android::ANativeWindow {
    self.window
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
impl<'l> Default for VkAndroidSurfaceCreateInfoKHR<'l> {
  fn default() -> VkAndroidSurfaceCreateInfoKHR<'l> {
    VkAndroidSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
unsafe impl<'l> Struct for VkAndroidSurfaceCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_android_surface_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkAndroidSurfaceCreateInfoKHR);
}

// feature: VK_KHR_win32_surface
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;

/// Structure specifying parameters of a newly created Win32 surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32SurfaceCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkWin32SurfaceCreateFlagsKHR,
  pub hinstance: wsi::win32::HINSTANCE,
  pub hwnd: wsi::win32::HWND,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> VkWin32SurfaceCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkWin32SurfaceCreateInfoKHR<'l> {
    unsafe {
      VkWin32SurfaceCreateInfoKHR {
        sType: VkStructureType::WIN32_SURFACE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkWin32SurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_hinstance(mut self, value: wsi::win32::HINSTANCE) -> Self {
    self.hinstance = value;
    self
  }
  #[inline]
  pub fn set_hwnd(mut self, value: wsi::win32::HWND) -> Self {
    self.hwnd = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkWin32SurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn hinstance(&self) -> wsi::win32::HINSTANCE {
    self.hinstance
  }
  #[inline]
  pub fn hwnd(&self) -> wsi::win32::HWND {
    self.hwnd
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> Default for VkWin32SurfaceCreateInfoKHR<'l> {
  fn default() -> VkWin32SurfaceCreateInfoKHR<'l> {
    VkWin32SurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l> Struct for VkWin32SurfaceCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_win32_surface_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkWin32SurfaceCreateInfoKHR);
}

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

/// Application-defined debug report callback function
#[cfg(feature = "VK_EXT_debug_report")]
#[allow(non_camel_case_types)]
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

/// Structure specifying parameters of a newly created debug report callback
#[repr(C)]
#[cfg(feature = "VK_EXT_debug_report")]
pub struct VkDebugReportCallbackCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDebugReportFlagsEXT,
  pub pfnCallback: PFN_vkDebugReportCallbackEXT,
  pUserData: *mut c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<'l> VkDebugReportCallbackCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkDebugReportCallbackCreateInfoEXT<'l> {
    unsafe {
      VkDebugReportCallbackCreateInfoEXT {
        sType: VkStructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDebugReportFlagsEXT) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_pfn_callback(mut self, value: PFN_vkDebugReportCallbackEXT) -> Self {
    self.pfnCallback = value;
    self
  }
  #[inline]
  pub fn set_user_data(mut self, value: *mut c_void) -> Self {
    self.pUserData = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkDebugReportFlagsEXT {
    self.flags
  }
  #[inline]
  pub fn pfn_callback(&self) -> PFN_vkDebugReportCallbackEXT {
    self.pfnCallback
  }
  #[inline]
  pub fn user_data(&self) -> *mut c_void {
    self.pUserData
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<'l> Default for VkDebugReportCallbackCreateInfoEXT<'l> {
  fn default() -> VkDebugReportCallbackCreateInfoEXT<'l> {
    VkDebugReportCallbackCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl<'l> Struct for VkDebugReportCallbackCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl<'m, 'l: 'm> StructExtends<VkInstanceCreateInfo<'m>> for VkDebugReportCallbackCreateInfoEXT<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDebugReportCallbackCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_debug_report_callback_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkDebugReportCallbackCreateInfoEXT);
}
#[cfg(feature = "VK_EXT_debug_report")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDebugReportCallbackEXT__ {}

/// Opaque handle to a debug report callback object
#[cfg(feature = "VK_EXT_debug_report")]
pub type VkDebugReportCallbackEXT<'l> = VkNonDispatchableHandle<'l, VkDebugReportCallbackEXT__>;

// feature: VK_AMD_rasterization_order

/// Specify rasterization order for a graphics pipeline
#[cfg(feature = "VK_AMD_rasterization_order")]
pub use enums::VkRasterizationOrderAMD;

/// Structure defining rasterization order for a graphics pipeline
#[repr(C)]
#[cfg(feature = "VK_AMD_rasterization_order")]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub rasterizationOrder: VkRasterizationOrderAMD,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_AMD_rasterization_order")]
impl<'l> VkPipelineRasterizationStateRasterizationOrderAMD<'l> {
  #[inline]
  pub fn new() -> VkPipelineRasterizationStateRasterizationOrderAMD<'l> {
    unsafe {
      VkPipelineRasterizationStateRasterizationOrderAMD {
        sType: VkStructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_rasterization_order(mut self, value: VkRasterizationOrderAMD) -> Self {
    self.rasterizationOrder = value;
    self
  }
  #[inline]
  pub fn rasterization_order(&self) -> VkRasterizationOrderAMD {
    self.rasterizationOrder
  }
}
#[cfg(feature = "VK_AMD_rasterization_order")]
impl<'l> Default for VkPipelineRasterizationStateRasterizationOrderAMD<'l> {
  fn default() -> VkPipelineRasterizationStateRasterizationOrderAMD<'l> {
    VkPipelineRasterizationStateRasterizationOrderAMD::new()
  }
}
#[cfg(feature = "VK_AMD_rasterization_order")]
unsafe impl<'l> Struct for VkPipelineRasterizationStateRasterizationOrderAMD<'l> {}
#[cfg(feature = "VK_AMD_rasterization_order")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineRasterizationStateCreateInfo<'m>>
  for VkPipelineRasterizationStateRasterizationOrderAMD<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineRasterizationStateRasterizationOrderAMD as *const c_void
  }
}
#[cfg(feature = "VK_AMD_rasterization_order")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_rasterization_state_rasterization_order_amd() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPipelineRasterizationStateRasterizationOrderAMD);
}

// feature: VK_EXT_debug_marker

/// Specify parameters of a name to give to an object
#[repr(C)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerObjectNameInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub objectType: VkDebugReportObjectTypeEXT,
  pub object: u64,
  pObjectName: *const c_char,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'l> VkDebugMarkerObjectNameInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkDebugMarkerObjectNameInfoEXT<'l> {
    unsafe {
      VkDebugMarkerObjectNameInfoEXT {
        sType: VkStructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_object_type(mut self, value: VkDebugReportObjectTypeEXT) -> Self {
    self.objectType = value;
    self
  }
  #[inline]
  pub fn set_object(mut self, value: u64) -> Self {
    self.object = value;
    self
  }
  #[inline]
  pub fn set_object_name(mut self, value: &'l AsRef<CStr>) -> Self {
    unsafe {
      self.pObjectName = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn object_type(&self) -> VkDebugReportObjectTypeEXT {
    self.objectType
  }
  #[inline]
  pub fn object(&self) -> u64 {
    self.object
  }
  #[inline]
  pub fn object_name(&self) -> &CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pObjectName) }
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'l> Default for VkDebugMarkerObjectNameInfoEXT<'l> {
  fn default() -> VkDebugMarkerObjectNameInfoEXT<'l> {
    VkDebugMarkerObjectNameInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
unsafe impl<'l> Struct for VkDebugMarkerObjectNameInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_debug_marker")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_debug_marker_object_name_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 4, VkDebugMarkerObjectNameInfoEXT);
}

/// Specify parameters of a tag to attach to an object
#[repr(C)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerObjectTagInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub objectType: VkDebugReportObjectTypeEXT,
  pub object: u64,
  pub tagName: u64,
  tagSize: usize,
  pTag: *const c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'l> VkDebugMarkerObjectTagInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkDebugMarkerObjectTagInfoEXT<'l> {
    unsafe {
      VkDebugMarkerObjectTagInfoEXT {
        sType: VkStructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_object_type(mut self, value: VkDebugReportObjectTypeEXT) -> Self {
    self.objectType = value;
    self
  }
  #[inline]
  pub fn set_object(mut self, value: u64) -> Self {
    self.object = value;
    self
  }
  #[inline]
  pub fn set_tag_name(mut self, value: u64) -> Self {
    self.tagName = value;
    self
  }
  #[inline]
  pub fn set_tag(mut self, value: &'l [u8]) -> Self {
    self.tagSize = value.len() as usize;
    unsafe {
      self.pTag = value.as_raw() as *const c_void;
    }
    self
  }
  #[inline]
  pub fn object_type(&self) -> VkDebugReportObjectTypeEXT {
    self.objectType
  }
  #[inline]
  pub fn object(&self) -> u64 {
    self.object
  }
  #[inline]
  pub fn tag_name(&self) -> u64 {
    self.tagName
  }
  #[inline]
  pub fn tag_size(&self) -> usize {
    self.tagSize
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'l> Default for VkDebugMarkerObjectTagInfoEXT<'l> {
  fn default() -> VkDebugMarkerObjectTagInfoEXT<'l> {
    VkDebugMarkerObjectTagInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
unsafe impl<'l> Struct for VkDebugMarkerObjectTagInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_debug_marker")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_debug_marker_object_tag_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 5, VkDebugMarkerObjectTagInfoEXT);
}

/// Specify parameters of a command buffer marker region
#[repr(C)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerMarkerInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pMarkerName: *const c_char,
  pub color: [f32; 4],
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'l> VkDebugMarkerMarkerInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkDebugMarkerMarkerInfoEXT<'l> {
    unsafe {
      VkDebugMarkerMarkerInfoEXT {
        sType: VkStructureType::DEBUG_MARKER_MARKER_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_marker_name(mut self, value: &'l AsRef<CStr>) -> Self {
    unsafe {
      self.pMarkerName = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_color(mut self, value: [f32; 4]) -> Self {
    self.color = value;
    self
  }
  #[inline]
  pub fn marker_name(&self) -> &CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pMarkerName) }
  }
  #[inline]
  pub fn color(&self) -> [f32; 4] {
    self.color
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'l> Default for VkDebugMarkerMarkerInfoEXT<'l> {
  fn default() -> VkDebugMarkerMarkerInfoEXT<'l> {
    VkDebugMarkerMarkerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
unsafe impl<'l> Struct for VkDebugMarkerMarkerInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_debug_marker")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_debug_marker_marker_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkDebugMarkerMarkerInfoEXT);
}

// feature: VK_NV_dedicated_allocation

/// Specify that an image is bound to a dedicated memory resource
#[repr(C)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationImageCreateInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  dedicatedAllocation: VkBool32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'l> VkDedicatedAllocationImageCreateInfoNV<'l> {
  #[inline]
  pub fn new() -> VkDedicatedAllocationImageCreateInfoNV<'l> {
    unsafe {
      VkDedicatedAllocationImageCreateInfoNV {
        sType: VkStructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dedicated_allocation(mut self, value: bool) -> Self {
    unsafe {
      self.dedicatedAllocation = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn is_dedicated_allocation(&self) -> bool {
    self.dedicatedAllocation != 0
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'l> Default for VkDedicatedAllocationImageCreateInfoNV<'l> {
  fn default() -> VkDedicatedAllocationImageCreateInfoNV<'l> {
    VkDedicatedAllocationImageCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'l> Struct for VkDedicatedAllocationImageCreateInfoNV<'l> {}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'m, 'l: 'm> StructExtends<VkImageCreateInfo<'m>> for VkDedicatedAllocationImageCreateInfoNV<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDedicatedAllocationImageCreateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_dedicated_allocation_image_create_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDedicatedAllocationImageCreateInfoNV);
}

/// Specify that a buffer is bound to a dedicated memory resource
#[repr(C)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationBufferCreateInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  dedicatedAllocation: VkBool32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'l> VkDedicatedAllocationBufferCreateInfoNV<'l> {
  #[inline]
  pub fn new() -> VkDedicatedAllocationBufferCreateInfoNV<'l> {
    unsafe {
      VkDedicatedAllocationBufferCreateInfoNV {
        sType: VkStructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dedicated_allocation(mut self, value: bool) -> Self {
    unsafe {
      self.dedicatedAllocation = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn is_dedicated_allocation(&self) -> bool {
    self.dedicatedAllocation != 0
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'l> Default for VkDedicatedAllocationBufferCreateInfoNV<'l> {
  fn default() -> VkDedicatedAllocationBufferCreateInfoNV<'l> {
    VkDedicatedAllocationBufferCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'l> Struct for VkDedicatedAllocationBufferCreateInfoNV<'l> {}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'m, 'l: 'm> StructExtends<VkBufferCreateInfo<'m>> for VkDedicatedAllocationBufferCreateInfoNV<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDedicatedAllocationBufferCreateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_dedicated_allocation_buffer_create_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDedicatedAllocationBufferCreateInfoNV);
}

/// Specify a dedicated memory allocation resource
#[repr(C)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  image: u64,
  buffer: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'l, 'h: 'l> VkDedicatedAllocationMemoryAllocateInfoNV<'l, 'h> {
  #[inline]
  pub fn new() -> VkDedicatedAllocationMemoryAllocateInfoNV<'l, 'h> {
    unsafe {
      VkDedicatedAllocationMemoryAllocateInfoNV {
        sType: VkStructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: Option<VkImage<'h>>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: Option<VkBuffer<'h>>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'l, 'h: 'l> Default for VkDedicatedAllocationMemoryAllocateInfoNV<'l, 'h> {
  fn default() -> VkDedicatedAllocationMemoryAllocateInfoNV<'l, 'h> {
    VkDedicatedAllocationMemoryAllocateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'l, 'h: 'l> Struct for VkDedicatedAllocationMemoryAllocateInfoNV<'l, 'h> {}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkMemoryAllocateInfo<'m>>
  for VkDedicatedAllocationMemoryAllocateInfoNV<'l, 'h>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDedicatedAllocationMemoryAllocateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_dedicated_allocation_memory_allocate_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 2, VkDedicatedAllocationMemoryAllocateInfoNV);
}

// feature: VK_KHR_get_physical_device_properties2

/// Structure describing the fine-grained features that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceFeatures2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub features: VkPhysicalDeviceFeatures,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkPhysicalDeviceFeatures2KHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceFeatures2KHR {
    unsafe {
      VkPhysicalDeviceFeatures2KHR {
        sType: VkStructureType::PHYSICAL_DEVICE_FEATURES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_features(mut self, value: VkPhysicalDeviceFeatures) -> Self {
    self.features = value;
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn features(&self) -> &VkPhysicalDeviceFeatures {
    &self.features
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceFeatures2KHR {
  fn default() -> VkPhysicalDeviceFeatures2KHR {
    VkPhysicalDeviceFeatures2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl Struct for VkPhysicalDeviceFeatures2KHR {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl<'m> StructExtends<VkDeviceCreateInfo<'m>> for VkPhysicalDeviceFeatures2KHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceFeatures2KHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_features2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(216 + ptr_size * 3, VkPhysicalDeviceFeatures2KHR);
}

/// Structure specifying physical device properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceProperties2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub properties: VkPhysicalDeviceProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkPhysicalDeviceProperties2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn properties(&self) -> &VkPhysicalDeviceProperties {
    &self.properties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl Struct for VkPhysicalDeviceProperties2KHR {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_properties2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(776 + ptr_size * 8, VkPhysicalDeviceProperties2KHR);
}

/// Structure specifying image format properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkFormatProperties2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub formatProperties: VkFormatProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkFormatProperties2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn format_properties(&self) -> &VkFormatProperties {
    &self.formatProperties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl Struct for VkFormatProperties2KHR {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_format_properties2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkFormatProperties2KHR);
}

/// Structure specifying a image format properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkImageFormatProperties2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub imageFormatProperties: VkImageFormatProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkImageFormatProperties2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn image_format_properties(&self) -> &VkImageFormatProperties {
    &self.imageFormatProperties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl Struct for VkImageFormatProperties2KHR {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_format_properties2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 2, VkImageFormatProperties2KHR);
}

/// Structure specifying image creation parameters
#[repr(C)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceImageFormatInfo2KHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub format: VkFormat,
  pub eType: VkImageType,
  pub tiling: VkImageTiling,
  pub usage: VkImageUsageFlags,
  pub flags: VkImageCreateFlags,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'l> VkPhysicalDeviceImageFormatInfo2KHR<'l> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceImageFormatInfo2KHR<'l> {
    unsafe {
      VkPhysicalDeviceImageFormatInfo2KHR {
        sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_type(mut self, value: VkImageType) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_tiling(mut self, value: VkImageTiling) -> Self {
    self.tiling = value;
    self
  }
  #[inline]
  pub fn set_usage(mut self, value: VkImageUsageFlags) -> Self {
    self.usage = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkImageCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_type(&self) -> VkImageType {
    self.eType
  }
  #[inline]
  pub fn tiling(&self) -> VkImageTiling {
    self.tiling
  }
  #[inline]
  pub fn usage(&self) -> VkImageUsageFlags {
    self.usage
  }
  #[inline]
  pub fn flags(&self) -> VkImageCreateFlags {
    self.flags
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'l> Default for VkPhysicalDeviceImageFormatInfo2KHR<'l> {
  fn default() -> VkPhysicalDeviceImageFormatInfo2KHR<'l> {
    VkPhysicalDeviceImageFormatInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl<'l> Struct for VkPhysicalDeviceImageFormatInfo2KHR<'l> {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_image_format_info2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkPhysicalDeviceImageFormatInfo2KHR);
}

/// Structure providing information about a queue family
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkQueueFamilyProperties2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub queueFamilyProperties: VkQueueFamilyProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkQueueFamilyProperties2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn queue_family_properties(&self) -> &VkQueueFamilyProperties {
    &self.queueFamilyProperties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl Struct for VkQueueFamilyProperties2KHR {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_queue_family_properties2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 2, VkQueueFamilyProperties2KHR);
}

/// Structure specifying physical device memory properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkPhysicalDeviceMemoryProperties2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn memory_properties(&self) -> &VkPhysicalDeviceMemoryProperties {
    &self.memoryProperties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl Struct for VkPhysicalDeviceMemoryProperties2KHR {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_memory_properties2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(392 + ptr_size * 18, VkPhysicalDeviceMemoryProperties2KHR);
}

/// Structure specifying sparse image format properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkSparseImageFormatProperties2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub properties: VkSparseImageFormatProperties,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkSparseImageFormatProperties2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn properties(&self) -> &VkSparseImageFormatProperties {
    &self.properties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl Struct for VkSparseImageFormatProperties2KHR {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_format_properties2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkSparseImageFormatProperties2KHR);
}

/// Structure specifying sparse image format inputs
#[repr(C)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub format: VkFormat,
  pub eType: VkImageType,
  pub samples: VkSampleCountFlagBits,
  pub usage: VkImageUsageFlags,
  pub tiling: VkImageTiling,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'l> VkPhysicalDeviceSparseImageFormatInfo2KHR<'l> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceSparseImageFormatInfo2KHR<'l> {
    unsafe {
      VkPhysicalDeviceSparseImageFormatInfo2KHR {
        sType: VkStructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_type(mut self, value: VkImageType) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_samples(mut self, value: VkSampleCountFlagBits) -> Self {
    self.samples = value;
    self
  }
  #[inline]
  pub fn set_usage(mut self, value: VkImageUsageFlags) -> Self {
    self.usage = value;
    self
  }
  #[inline]
  pub fn set_tiling(mut self, value: VkImageTiling) -> Self {
    self.tiling = value;
    self
  }
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_type(&self) -> VkImageType {
    self.eType
  }
  #[inline]
  pub fn samples(&self) -> VkSampleCountFlagBits {
    self.samples
  }
  #[inline]
  pub fn usage(&self) -> VkImageUsageFlags {
    self.usage
  }
  #[inline]
  pub fn tiling(&self) -> VkImageTiling {
    self.tiling
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'l> Default for VkPhysicalDeviceSparseImageFormatInfo2KHR<'l> {
  fn default() -> VkPhysicalDeviceSparseImageFormatInfo2KHR<'l> {
    VkPhysicalDeviceSparseImageFormatInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl<'l> Struct for VkPhysicalDeviceSparseImageFormatInfo2KHR<'l> {}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_sparse_image_format_info2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkPhysicalDeviceSparseImageFormatInfo2KHR);
}

// feature: VK_AMD_texture_gather_bias_lod

/// Structure informing whether or not texture gather bias/LOD functionality is
/// supported for a given image format and a given physical device.
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
pub struct VkTextureLODGatherFormatPropertiesAMD {
  sType: VkStructureType,
  pNext: *mut c_void,
  supportsTextureGatherLODBiasAMD: VkBool32,
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
impl VkTextureLODGatherFormatPropertiesAMD {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_supports_texture_gather_lod_bias_amd(&self) -> bool {
    self.supportsTextureGatherLODBiasAMD != 0
  }
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
unsafe impl Struct for VkTextureLODGatherFormatPropertiesAMD {}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
unsafe impl StructExtends<VkImageFormatProperties2KHR> for VkTextureLODGatherFormatPropertiesAMD {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkTextureLODGatherFormatPropertiesAMD as *const c_void
  }
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_texture_lod_gather_format_properties_amd() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkTextureLODGatherFormatPropertiesAMD);
}

// feature: VK_AMD_shader_info
#[cfg(feature = "VK_AMD_shader_info")]
pub use enums::VkShaderInfoTypeAMD;

/// Resource usage information about a particular shader within a pipeline
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
#[cfg(feature = "VK_AMD_shader_info")]
impl VkShaderResourceUsageAMD {
  #[inline]
  pub fn num_used_vgprs(&self) -> u32 {
    self.numUsedVgprs
  }
  #[inline]
  pub fn num_used_sgprs(&self) -> u32 {
    self.numUsedSgprs
  }
  #[inline]
  pub fn lds_size_per_local_work_group(&self) -> u32 {
    self.ldsSizePerLocalWorkGroup
  }
  #[inline]
  pub fn lds_usage_size_in_bytes(&self) -> usize {
    self.ldsUsageSizeInBytes
  }
  #[inline]
  pub fn scratch_mem_usage_in_bytes(&self) -> usize {
    self.scratchMemUsageInBytes
  }
}
#[cfg(feature = "VK_AMD_shader_info")]
unsafe impl Struct for VkShaderResourceUsageAMD {}
#[cfg(feature = "VK_AMD_shader_info")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_shader_resource_usage_amd() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkShaderResourceUsageAMD);
}

/// Statistical information about a particular shader within a pipeline
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
#[cfg(feature = "VK_AMD_shader_info")]
impl VkShaderStatisticsInfoAMD {
  #[inline]
  pub fn shader_stage_mask(&self) -> VkShaderStageFlags {
    self.shaderStageMask
  }
  #[inline]
  pub fn resource_usage(&self) -> &VkShaderResourceUsageAMD {
    &self.resourceUsage
  }
  #[inline]
  pub fn num_physical_vgprs(&self) -> u32 {
    self.numPhysicalVgprs
  }
  #[inline]
  pub fn num_physical_sgprs(&self) -> u32 {
    self.numPhysicalSgprs
  }
  #[inline]
  pub fn num_available_vgprs(&self) -> u32 {
    self.numAvailableVgprs
  }
  #[inline]
  pub fn num_available_sgprs(&self) -> u32 {
    self.numAvailableSgprs
  }
  #[inline]
  pub fn compute_work_group_size(&self) -> [u32; 3] {
    self.computeWorkGroupSize
  }
}
#[cfg(feature = "VK_AMD_shader_info")]
unsafe impl Struct for VkShaderStatisticsInfoAMD {}
#[cfg(feature = "VK_AMD_shader_info")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_shader_statistics_info_amd() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 5, VkShaderStatisticsInfoAMD);
}

// feature: VK_KHX_multiview

/// Structure containing multiview info for all subpasses
#[repr(C)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkRenderPassMultiviewCreateInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  subpassCount: u32,
  pViewMasks: *const u32,
  dependencyCount: u32,
  pViewOffsets: *const i32,
  correlationMaskCount: u32,
  pCorrelationMasks: *const u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_multiview")]
impl<'l> VkRenderPassMultiviewCreateInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkRenderPassMultiviewCreateInfoKHX<'l> {
    unsafe {
      VkRenderPassMultiviewCreateInfoKHX {
        sType: VkStructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_view_masks(mut self, value: &'l [u32]) -> Self {
    self.subpassCount = value.len() as u32;
    unsafe {
      self.pViewMasks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_view_offsets(mut self, value: &'l [i32]) -> Self {
    self.dependencyCount = value.len() as u32;
    unsafe {
      self.pViewOffsets = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_correlation_masks(mut self, value: &'l [u32]) -> Self {
    self.correlationMaskCount = value.len() as u32;
    unsafe {
      self.pCorrelationMasks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn subpass_count(&self) -> u32 {
    self.subpassCount
  }
  #[inline]
  pub fn dependency_count(&self) -> u32 {
    self.dependencyCount
  }
  #[inline]
  pub fn correlation_mask_count(&self) -> u32 {
    self.correlationMaskCount
  }
}
#[cfg(feature = "VK_KHX_multiview")]
impl<'l> Default for VkRenderPassMultiviewCreateInfoKHX<'l> {
  fn default() -> VkRenderPassMultiviewCreateInfoKHX<'l> {
    VkRenderPassMultiviewCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl<'l> Struct for VkRenderPassMultiviewCreateInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl<'m, 'l: 'm> StructExtends<VkRenderPassCreateInfo<'m>> for VkRenderPassMultiviewCreateInfoKHX<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkRenderPassMultiviewCreateInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_multiview")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_multiview_create_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 8, VkRenderPassMultiviewCreateInfoKHX);
}

/// Structure describing multiview features that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkPhysicalDeviceMultiviewFeaturesKHX {
  sType: VkStructureType,
  pNext: *mut c_void,
  multiview: VkBool32,
  multiviewGeometryShader: VkBool32,
  multiviewTessellationShader: VkBool32,
}
#[cfg(feature = "VK_KHX_multiview")]
impl VkPhysicalDeviceMultiviewFeaturesKHX {
  #[inline]
  pub fn new() -> VkPhysicalDeviceMultiviewFeaturesKHX {
    unsafe {
      VkPhysicalDeviceMultiviewFeaturesKHX {
        sType: VkStructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_multiview(mut self, value: bool) -> Self {
    unsafe {
      self.multiview = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_multiview_geometry_shader(mut self, value: bool) -> Self {
    unsafe {
      self.multiviewGeometryShader = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_multiview_tessellation_shader(mut self, value: bool) -> Self {
    unsafe {
      self.multiviewTessellationShader = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_multiview(&self) -> bool {
    self.multiview != 0
  }
  #[inline]
  pub fn is_multiview_geometry_shader(&self) -> bool {
    self.multiviewGeometryShader != 0
  }
  #[inline]
  pub fn is_multiview_tessellation_shader(&self) -> bool {
    self.multiviewTessellationShader != 0
  }
}
#[cfg(feature = "VK_KHX_multiview")]
impl Default for VkPhysicalDeviceMultiviewFeaturesKHX {
  fn default() -> VkPhysicalDeviceMultiviewFeaturesKHX {
    VkPhysicalDeviceMultiviewFeaturesKHX::new()
  }
}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl Struct for VkPhysicalDeviceMultiviewFeaturesKHX {}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl StructExtends<VkPhysicalDeviceFeatures2KHR> for VkPhysicalDeviceMultiviewFeaturesKHX {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceMultiviewFeaturesKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl<'m> StructExtends<VkDeviceCreateInfo<'m>> for VkPhysicalDeviceMultiviewFeaturesKHX {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceMultiviewFeaturesKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_multiview")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_multiview_features_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPhysicalDeviceMultiviewFeaturesKHX);
}

/// Structure describing multiview limits that can be supported by an implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkPhysicalDeviceMultiviewPropertiesKHX {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub maxMultiviewViewCount: u32,
  pub maxMultiviewInstanceIndex: u32,
}
#[cfg(feature = "VK_KHX_multiview")]
impl VkPhysicalDeviceMultiviewPropertiesKHX {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn max_multiview_view_count(&self) -> u32 {
    self.maxMultiviewViewCount
  }
  #[inline]
  pub fn max_multiview_instance_index(&self) -> u32 {
    self.maxMultiviewInstanceIndex
  }
}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl Struct for VkPhysicalDeviceMultiviewPropertiesKHX {}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceMultiviewPropertiesKHX {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceMultiviewPropertiesKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_multiview")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_multiview_properties_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkPhysicalDeviceMultiviewPropertiesKHX);
}

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

/// Structure specifying external image format properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub struct VkExternalImageFormatPropertiesNV {
  pub imageFormatProperties: VkImageFormatProperties,
  pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
  pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
  pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
impl VkExternalImageFormatPropertiesNV {
  #[inline]
  pub fn image_format_properties(&self) -> &VkImageFormatProperties {
    &self.imageFormatProperties
  }
  #[inline]
  pub fn external_memory_features(&self) -> VkExternalMemoryFeatureFlagsNV {
    self.externalMemoryFeatures
  }
  #[inline]
  pub fn export_from_imported_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.exportFromImportedHandleTypes
  }
  #[inline]
  pub fn compatible_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.compatibleHandleTypes
  }
}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
unsafe impl Struct for VkExternalImageFormatPropertiesNV {}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_image_format_properties_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(40 + ptr_size * 1, VkExternalImageFormatPropertiesNV);
}

// feature: VK_NV_external_memory

/// Specify that an image may be backed by external memory
#[repr(C)]
#[cfg(feature = "VK_NV_external_memory")]
pub struct VkExternalMemoryImageCreateInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_external_memory")]
impl<'l> VkExternalMemoryImageCreateInfoNV<'l> {
  #[inline]
  pub fn new() -> VkExternalMemoryImageCreateInfoNV<'l> {
    unsafe {
      VkExternalMemoryImageCreateInfoNV {
        sType: VkStructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsNV) -> Self {
    self.handleTypes = value;
    self
  }
  #[inline]
  pub fn handle_types(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.handleTypes
  }
}
#[cfg(feature = "VK_NV_external_memory")]
impl<'l> Default for VkExternalMemoryImageCreateInfoNV<'l> {
  fn default() -> VkExternalMemoryImageCreateInfoNV<'l> {
    VkExternalMemoryImageCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'l> Struct for VkExternalMemoryImageCreateInfoNV<'l> {}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'m, 'l: 'm> StructExtends<VkImageCreateInfo<'m>> for VkExternalMemoryImageCreateInfoNV<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExternalMemoryImageCreateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_external_memory")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_memory_image_create_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkExternalMemoryImageCreateInfoNV);
}

/// Specify memory handle types that may be exported
#[repr(C)]
#[cfg(feature = "VK_NV_external_memory")]
pub struct VkExportMemoryAllocateInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_external_memory")]
impl<'l> VkExportMemoryAllocateInfoNV<'l> {
  #[inline]
  pub fn new() -> VkExportMemoryAllocateInfoNV<'l> {
    unsafe {
      VkExportMemoryAllocateInfoNV {
        sType: VkStructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsNV) -> Self {
    self.handleTypes = value;
    self
  }
  #[inline]
  pub fn handle_types(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.handleTypes
  }
}
#[cfg(feature = "VK_NV_external_memory")]
impl<'l> Default for VkExportMemoryAllocateInfoNV<'l> {
  fn default() -> VkExportMemoryAllocateInfoNV<'l> {
    VkExportMemoryAllocateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'l> Struct for VkExportMemoryAllocateInfoNV<'l> {}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkExportMemoryAllocateInfoNV<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExportMemoryAllocateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_external_memory")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_export_memory_allocate_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkExportMemoryAllocateInfoNV);
}

// feature: VK_NV_external_memory_win32

/// import Win32 memory created on the same physical device
#[repr(C)]
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportMemoryWin32HandleInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagsNV,
  pub handle: wsi::win32::HANDLE,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> VkImportMemoryWin32HandleInfoNV<'l> {
  #[inline]
  pub fn new() -> VkImportMemoryWin32HandleInfoNV<'l> {
    unsafe {
      VkImportMemoryWin32HandleInfoNV {
        sType: VkStructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagsNV) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn set_handle(mut self, value: wsi::win32::HANDLE) -> Self {
    self.handle = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.handleType
  }
  #[inline]
  pub fn handle(&self) -> wsi::win32::HANDLE {
    self.handle
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> Default for VkImportMemoryWin32HandleInfoNV<'l> {
  fn default() -> VkImportMemoryWin32HandleInfoNV<'l> {
    VkImportMemoryWin32HandleInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l> Struct for VkImportMemoryWin32HandleInfoNV<'l> {}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkImportMemoryWin32HandleInfoNV<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkImportMemoryWin32HandleInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_memory_win32_handle_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkImportMemoryWin32HandleInfoNV);
}

/// specify security attributes and access rights for Win32 memory handles
#[repr(C)]
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportMemoryWin32HandleInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> VkExportMemoryWin32HandleInfoNV<'l> {
  #[inline]
  pub fn new() -> VkExportMemoryWin32HandleInfoNV<'l> {
    unsafe {
      VkExportMemoryWin32HandleInfoNV {
        sType: VkStructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_attributes(mut self, value: *const wsi::win32::SECURITY_ATTRIBUTES) -> Self {
    self.pAttributes = value;
    self
  }
  #[inline]
  pub fn set_dw_access(mut self, value: wsi::win32::DWORD) -> Self {
    self.dwAccess = value;
    self
  }
  #[inline]
  pub fn attributes(&self) -> *const wsi::win32::SECURITY_ATTRIBUTES {
    self.pAttributes
  }
  #[inline]
  pub fn dw_access(&self) -> wsi::win32::DWORD {
    self.dwAccess
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> Default for VkExportMemoryWin32HandleInfoNV<'l> {
  fn default() -> VkExportMemoryWin32HandleInfoNV<'l> {
    VkExportMemoryWin32HandleInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l> Struct for VkExportMemoryWin32HandleInfoNV<'l> {}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkExportMemoryWin32HandleInfoNV<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExportMemoryWin32HandleInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_export_memory_win32_handle_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkExportMemoryWin32HandleInfoNV);
}

// feature: VK_NV_win32_keyed_mutex

/// use Windows keyex mutex mechanism to synchronize work
#[repr(C)]
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  acquireCount: u32,
  pAcquireSyncs: *const u64,
  pAcquireKeys: *const u64,
  pAcquireTimeoutMilliseconds: *const u32,
  releaseCount: u32,
  pReleaseSyncs: *const u64,
  pReleaseKeys: *const u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> VkWin32KeyedMutexAcquireReleaseInfoNV<'l, 'h> {
  #[inline]
  pub fn new() -> VkWin32KeyedMutexAcquireReleaseInfoNV<'l, 'h> {
    unsafe {
      VkWin32KeyedMutexAcquireReleaseInfoNV {
        sType: VkStructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn acquire_count(&self) -> u32 {
    self.acquireCount
  }
  #[inline]
  pub fn release_count(&self) -> u32 {
    self.releaseCount
  }
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> Default for VkWin32KeyedMutexAcquireReleaseInfoNV<'l, 'h> {
  fn default() -> VkWin32KeyedMutexAcquireReleaseInfoNV<'l, 'h> {
    VkWin32KeyedMutexAcquireReleaseInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l, 'h: 'l> Struct for VkWin32KeyedMutexAcquireReleaseInfoNV<'l, 'h> {}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkSubmitInfo<'m, 'h>> for VkWin32KeyedMutexAcquireReleaseInfoNV<'l, 'h> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkWin32KeyedMutexAcquireReleaseInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_win32_keyed_mutex_acquire_release_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 9, VkWin32KeyedMutexAcquireReleaseInfoNV);
}

// feature: VK_KHX_device_group_creation

/// Structure specifying physical device group properties
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group_creation")]
pub struct VkPhysicalDeviceGroupPropertiesKHX<'h> {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub physicalDeviceCount: u32,
  physicalDevices: [usize; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize],
  subsetAllocation: VkBool32,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl<'h> VkPhysicalDeviceGroupPropertiesKHX<'h> {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn physical_device_count(&self) -> u32 {
    self.physicalDeviceCount
  }
  #[inline]
  pub fn is_subset_allocation(&self) -> bool {
    self.subsetAllocation != 0
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
unsafe impl<'h> Struct for VkPhysicalDeviceGroupPropertiesKHX<'h> {}
#[cfg(feature = "VK_KHX_device_group_creation")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_group_properties_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 36, VkPhysicalDeviceGroupPropertiesKHX);
}

/// Create a logical device from multiple physical devices
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group_creation")]
pub struct VkDeviceGroupDeviceCreateInfoKHX<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  physicalDeviceCount: u32,
  pPhysicalDevices: *const usize,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl<'l, 'h: 'l> VkDeviceGroupDeviceCreateInfoKHX<'l, 'h> {
  #[inline]
  pub fn new() -> VkDeviceGroupDeviceCreateInfoKHX<'l, 'h> {
    unsafe {
      VkDeviceGroupDeviceCreateInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_DEVICE_CREATE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_physical_devices(mut self, value: &'l [VkPhysicalDevice<'h>]) -> Self {
    self.physicalDeviceCount = value.len() as u32;
    unsafe {
      self.pPhysicalDevices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn physical_device_count(&self) -> u32 {
    self.physicalDeviceCount
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl<'l, 'h: 'l> Default for VkDeviceGroupDeviceCreateInfoKHX<'l, 'h> {
  fn default() -> VkDeviceGroupDeviceCreateInfoKHX<'l, 'h> {
    VkDeviceGroupDeviceCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
unsafe impl<'l, 'h: 'l> Struct for VkDeviceGroupDeviceCreateInfoKHX<'l, 'h> {}
#[cfg(feature = "VK_KHX_device_group_creation")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkDeviceCreateInfo<'m>> for VkDeviceGroupDeviceCreateInfoKHX<'l, 'h> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDeviceGroupDeviceCreateInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_device_create_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkDeviceGroupDeviceCreateInfoKHX);
}

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

/// Structure controlling how many instances of memory will be allocated
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkMemoryAllocateFlagsInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkMemoryAllocateFlagsKHX,
  pub deviceMask: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkMemoryAllocateFlagsInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkMemoryAllocateFlagsInfoKHX<'l> {
    unsafe {
      VkMemoryAllocateFlagsInfoKHX {
        sType: VkStructureType::MEMORY_ALLOCATE_FLAGS_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkMemoryAllocateFlagsKHX) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_device_mask(mut self, value: u32) -> Self {
    self.deviceMask = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkMemoryAllocateFlagsKHX {
    self.flags
  }
  #[inline]
  pub fn device_mask(&self) -> u32 {
    self.deviceMask
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkMemoryAllocateFlagsInfoKHX<'l> {
  fn default() -> VkMemoryAllocateFlagsInfoKHX<'l> {
    VkMemoryAllocateFlagsInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkMemoryAllocateFlagsInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkMemoryAllocateFlagsInfoKHX<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkMemoryAllocateFlagsInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_allocate_flags_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkMemoryAllocateFlagsInfoKHX);
}

/// Set the initial device mask and render areas for a render pass instance
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupRenderPassBeginInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub deviceMask: u32,
  deviceRenderAreaCount: u32,
  pDeviceRenderAreas: *const VkRect2D,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkDeviceGroupRenderPassBeginInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkDeviceGroupRenderPassBeginInfoKHX<'l> {
    unsafe {
      VkDeviceGroupRenderPassBeginInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_mask(mut self, value: u32) -> Self {
    self.deviceMask = value;
    self
  }
  #[inline]
  pub fn set_device_render_areas(mut self, value: &'l [VkRect2D]) -> Self {
    self.deviceRenderAreaCount = value.len() as u32;
    unsafe {
      self.pDeviceRenderAreas = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn device_mask(&self) -> u32 {
    self.deviceMask
  }
  #[inline]
  pub fn device_render_area_count(&self) -> u32 {
    self.deviceRenderAreaCount
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkDeviceGroupRenderPassBeginInfoKHX<'l> {
  fn default() -> VkDeviceGroupRenderPassBeginInfoKHX<'l> {
    VkDeviceGroupRenderPassBeginInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkDeviceGroupRenderPassBeginInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkRenderPassBeginInfo<'m, 'h>>
  for VkDeviceGroupRenderPassBeginInfoKHX<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDeviceGroupRenderPassBeginInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_render_pass_begin_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkDeviceGroupRenderPassBeginInfoKHX);
}

/// Set the initial device mask for a command buffer
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupCommandBufferBeginInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub deviceMask: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkDeviceGroupCommandBufferBeginInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkDeviceGroupCommandBufferBeginInfoKHX<'l> {
    unsafe {
      VkDeviceGroupCommandBufferBeginInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_mask(mut self, value: u32) -> Self {
    self.deviceMask = value;
    self
  }
  #[inline]
  pub fn device_mask(&self) -> u32 {
    self.deviceMask
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkDeviceGroupCommandBufferBeginInfoKHX<'l> {
  fn default() -> VkDeviceGroupCommandBufferBeginInfoKHX<'l> {
    VkDeviceGroupCommandBufferBeginInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkDeviceGroupCommandBufferBeginInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkCommandBufferBeginInfo<'m, 'h>>
  for VkDeviceGroupCommandBufferBeginInfoKHX<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDeviceGroupCommandBufferBeginInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_command_buffer_begin_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDeviceGroupCommandBufferBeginInfoKHX);
}

/// Structure indicating which physical devices execute semaphore operations and
/// command buffers
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupSubmitInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreCount: u32,
  pWaitSemaphoreDeviceIndices: *const u32,
  commandBufferCount: u32,
  pCommandBufferDeviceMasks: *const u32,
  signalSemaphoreCount: u32,
  pSignalSemaphoreDeviceIndices: *const u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkDeviceGroupSubmitInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkDeviceGroupSubmitInfoKHX<'l> {
    unsafe {
      VkDeviceGroupSubmitInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_SUBMIT_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_wait_semaphore_device_indices(mut self, value: &'l [u32]) -> Self {
    self.waitSemaphoreCount = value.len() as u32;
    unsafe {
      self.pWaitSemaphoreDeviceIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_command_buffer_device_masks(mut self, value: &'l [u32]) -> Self {
    self.commandBufferCount = value.len() as u32;
    unsafe {
      self.pCommandBufferDeviceMasks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphore_device_indices(mut self, value: &'l [u32]) -> Self {
    self.signalSemaphoreCount = value.len() as u32;
    unsafe {
      self.pSignalSemaphoreDeviceIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn wait_semaphore_count(&self) -> u32 {
    self.waitSemaphoreCount
  }
  #[inline]
  pub fn command_buffer_count(&self) -> u32 {
    self.commandBufferCount
  }
  #[inline]
  pub fn signal_semaphore_count(&self) -> u32 {
    self.signalSemaphoreCount
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkDeviceGroupSubmitInfoKHX<'l> {
  fn default() -> VkDeviceGroupSubmitInfoKHX<'l> {
    VkDeviceGroupSubmitInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkDeviceGroupSubmitInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkSubmitInfo<'m, 'h>> for VkDeviceGroupSubmitInfoKHX<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDeviceGroupSubmitInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_submit_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 8, VkDeviceGroupSubmitInfoKHX);
}

/// Structure indicating which instances are bound
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupBindSparseInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub resourceDeviceIndex: u32,
  pub memoryDeviceIndex: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkDeviceGroupBindSparseInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkDeviceGroupBindSparseInfoKHX<'l> {
    unsafe {
      VkDeviceGroupBindSparseInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_BIND_SPARSE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_resource_device_index(mut self, value: u32) -> Self {
    self.resourceDeviceIndex = value;
    self
  }
  #[inline]
  pub fn set_memory_device_index(mut self, value: u32) -> Self {
    self.memoryDeviceIndex = value;
    self
  }
  #[inline]
  pub fn resource_device_index(&self) -> u32 {
    self.resourceDeviceIndex
  }
  #[inline]
  pub fn memory_device_index(&self) -> u32 {
    self.memoryDeviceIndex
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkDeviceGroupBindSparseInfoKHX<'l> {
  fn default() -> VkDeviceGroupBindSparseInfoKHX<'l> {
    VkDeviceGroupBindSparseInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkDeviceGroupBindSparseInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkBindSparseInfo<'m, 'h>> for VkDeviceGroupBindSparseInfoKHX<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDeviceGroupBindSparseInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_bind_sparse_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkDeviceGroupBindSparseInfoKHX);
}

/// Structure specifying device within a group to bind to
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindBufferMemoryDeviceGroupInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  deviceIndexCount: u32,
  pDeviceIndices: *const u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkBindBufferMemoryDeviceGroupInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkBindBufferMemoryDeviceGroupInfoKHX<'l> {
    unsafe {
      VkBindBufferMemoryDeviceGroupInfoKHX {
        sType: VkStructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_indices(mut self, value: &'l [u32]) -> Self {
    self.deviceIndexCount = value.len() as u32;
    unsafe {
      self.pDeviceIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn device_index_count(&self) -> u32 {
    self.deviceIndexCount
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkBindBufferMemoryDeviceGroupInfoKHX<'l> {
  fn default() -> VkBindBufferMemoryDeviceGroupInfoKHX<'l> {
    VkBindBufferMemoryDeviceGroupInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkBindBufferMemoryDeviceGroupInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkBindBufferMemoryInfoKHR<'m, 'h>>
  for VkBindBufferMemoryDeviceGroupInfoKHX<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkBindBufferMemoryDeviceGroupInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_buffer_memory_device_group_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkBindBufferMemoryDeviceGroupInfoKHX);
}

/// Structure specifying device within a group to bind to
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindImageMemoryDeviceGroupInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  deviceIndexCount: u32,
  pDeviceIndices: *const u32,
  SFRRectCount: u32,
  pSFRRects: *const VkRect2D,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkBindImageMemoryDeviceGroupInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkBindImageMemoryDeviceGroupInfoKHX<'l> {
    unsafe {
      VkBindImageMemoryDeviceGroupInfoKHX {
        sType: VkStructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_indices(mut self, value: &'l [u32]) -> Self {
    self.deviceIndexCount = value.len() as u32;
    unsafe {
      self.pDeviceIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sfr_rects(mut self, value: &'l [VkRect2D]) -> Self {
    self.SFRRectCount = value.len() as u32;
    unsafe {
      self.pSFRRects = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn device_index_count(&self) -> u32 {
    self.deviceIndexCount
  }
  #[inline]
  pub fn sfr_rect_count(&self) -> u32 {
    self.SFRRectCount
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkBindImageMemoryDeviceGroupInfoKHX<'l> {
  fn default() -> VkBindImageMemoryDeviceGroupInfoKHX<'l> {
    VkBindImageMemoryDeviceGroupInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkBindImageMemoryDeviceGroupInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkBindImageMemoryInfoKHR<'m, 'h>>
  for VkBindImageMemoryDeviceGroupInfoKHX<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkBindImageMemoryDeviceGroupInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_image_memory_device_group_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 6, VkBindImageMemoryDeviceGroupInfoKHX);
}

/// Bitmask specifying supported device group present modes
#[cfg(feature = "VK_KHX_device_group")]
pub use enums::VkDeviceGroupPresentModeFlagBitsKHX;

/// Bitmask of VkDeviceGroupPresentModeFlagBitsKHX
#[cfg(feature = "VK_KHX_device_group")]
pub type VkDeviceGroupPresentModeFlagsKHX = VkDeviceGroupPresentModeFlagBitsKHX;

/// Present capabilities from other physical devices
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupPresentCapabilitiesKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub presentMask: [u32; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize],
  pub modes: VkDeviceGroupPresentModeFlagsKHX,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkDeviceGroupPresentCapabilitiesKHX<'l> {
  #[inline]
  pub fn present_mask(&self) -> [u32; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize] {
    self.presentMask
  }
  #[inline]
  pub fn modes(&self) -> VkDeviceGroupPresentModeFlagsKHX {
    self.modes
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkDeviceGroupPresentCapabilitiesKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_present_capabilities_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(128 + ptr_size * 3, VkDeviceGroupPresentCapabilitiesKHX);
}

/// Specify that an image will be bound to swapchain memory
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkImageSwapchainCreateInfoKHX<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchain: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l, 'h: 'l> VkImageSwapchainCreateInfoKHX<'l, 'h> {
  #[inline]
  pub fn new() -> VkImageSwapchainCreateInfoKHX<'l, 'h> {
    unsafe {
      VkImageSwapchainCreateInfoKHX {
        sType: VkStructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_swapchain(mut self, value: Option<VkSwapchainKHR<'h>>) -> Self {
    unsafe {
      self.swapchain = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l, 'h: 'l> Default for VkImageSwapchainCreateInfoKHX<'l, 'h> {
  fn default() -> VkImageSwapchainCreateInfoKHX<'l, 'h> {
    VkImageSwapchainCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l, 'h: 'l> Struct for VkImageSwapchainCreateInfoKHX<'l, 'h> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkImageCreateInfo<'m>> for VkImageSwapchainCreateInfoKHX<'l, 'h> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkImageSwapchainCreateInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_swapchain_create_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkImageSwapchainCreateInfoKHX);
}

/// Structure specifying swapchain image memory to bind to
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindImageMemorySwapchainInfoKHX<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchain: u64,
  pub imageIndex: u32,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l, 'h: 'l> VkBindImageMemorySwapchainInfoKHX<'l, 'h> {
  #[inline]
  pub fn new() -> VkBindImageMemorySwapchainInfoKHX<'l, 'h> {
    unsafe {
      VkBindImageMemorySwapchainInfoKHX {
        sType: VkStructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_swapchain(mut self, value: VkSwapchainKHR<'h>) -> Self {
    unsafe {
      self.swapchain = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_index(mut self, value: u32) -> Self {
    self.imageIndex = value;
    self
  }
  #[inline]
  pub fn image_index(&self) -> u32 {
    self.imageIndex
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l, 'h: 'l> Default for VkBindImageMemorySwapchainInfoKHX<'l, 'h> {
  fn default() -> VkBindImageMemorySwapchainInfoKHX<'l, 'h> {
    VkBindImageMemorySwapchainInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l, 'h: 'l> Struct for VkBindImageMemorySwapchainInfoKHX<'l, 'h> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkBindImageMemoryInfoKHR<'m, 'h>>
  for VkBindImageMemorySwapchainInfoKHX<'l, 'h>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkBindImageMemorySwapchainInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_image_memory_swapchain_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkBindImageMemorySwapchainInfoKHX);
}

/// Structure specifying parameters of the acquire
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkAcquireNextImageInfoKHX<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchain: u64,
  pub timeout: u64,
  semaphore: u64,
  fence: u64,
  pub deviceMask: u32,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l, 'h: 'l> VkAcquireNextImageInfoKHX<'l, 'h> {
  #[inline]
  pub fn new() -> VkAcquireNextImageInfoKHX<'l, 'h> {
    unsafe {
      VkAcquireNextImageInfoKHX {
        sType: VkStructureType::ACQUIRE_NEXT_IMAGE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_swapchain(mut self, value: VkSwapchainKHR<'h>) -> Self {
    unsafe {
      self.swapchain = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_timeout(mut self, value: u64) -> Self {
    self.timeout = value;
    self
  }
  #[inline]
  pub fn set_semaphore(mut self, value: Option<VkSemaphore<'h>>) -> Self {
    unsafe {
      self.semaphore = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_fence(mut self, value: Option<VkFence<'h>>) -> Self {
    unsafe {
      self.fence = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_device_mask(mut self, value: u32) -> Self {
    self.deviceMask = value;
    self
  }
  #[inline]
  pub fn timeout(&self) -> u64 {
    self.timeout
  }
  #[inline]
  pub fn device_mask(&self) -> u32 {
    self.deviceMask
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l, 'h: 'l> Default for VkAcquireNextImageInfoKHX<'l, 'h> {
  fn default() -> VkAcquireNextImageInfoKHX<'l, 'h> {
    VkAcquireNextImageInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l, 'h: 'l> Struct for VkAcquireNextImageInfoKHX<'l, 'h> {}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_acquire_next_image_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 3, VkAcquireNextImageInfoKHX);
}

/// Mode and mask controlling which physical devices\' images are presented
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupPresentInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchainCount: u32,
  pDeviceMasks: *const u32,
  pub mode: VkDeviceGroupPresentModeFlagBitsKHX,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkDeviceGroupPresentInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkDeviceGroupPresentInfoKHX<'l> {
    unsafe {
      VkDeviceGroupPresentInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_PRESENT_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_masks(mut self, value: &'l [u32]) -> Self {
    self.swapchainCount = value.len() as u32;
    unsafe {
      self.pDeviceMasks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_mode(mut self, value: VkDeviceGroupPresentModeFlagBitsKHX) -> Self {
    self.mode = value;
    self
  }
  #[inline]
  pub fn swapchain_count(&self) -> u32 {
    self.swapchainCount
  }
  #[inline]
  pub fn mode(&self) -> VkDeviceGroupPresentModeFlagBitsKHX {
    self.mode
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkDeviceGroupPresentInfoKHX<'l> {
  fn default() -> VkDeviceGroupPresentInfoKHX<'l> {
    VkDeviceGroupPresentInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkDeviceGroupPresentInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkPresentInfoKHR<'m, 'h>> for VkDeviceGroupPresentInfoKHX<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDeviceGroupPresentInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_present_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkDeviceGroupPresentInfoKHX);
}

/// Structure specifying parameters of a newly created swapchain object
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupSwapchainCreateInfoKHX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub modes: VkDeviceGroupPresentModeFlagsKHX,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> VkDeviceGroupSwapchainCreateInfoKHX<'l> {
  #[inline]
  pub fn new() -> VkDeviceGroupSwapchainCreateInfoKHX<'l> {
    unsafe {
      VkDeviceGroupSwapchainCreateInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_modes(mut self, value: VkDeviceGroupPresentModeFlagsKHX) -> Self {
    self.modes = value;
    self
  }
  #[inline]
  pub fn modes(&self) -> VkDeviceGroupPresentModeFlagsKHX {
    self.modes
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'l> Default for VkDeviceGroupSwapchainCreateInfoKHX<'l> {
  fn default() -> VkDeviceGroupSwapchainCreateInfoKHX<'l> {
    VkDeviceGroupSwapchainCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'l> Struct for VkDeviceGroupSwapchainCreateInfoKHX<'l> {}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkSwapchainCreateInfoKHR<'m, 'h>>
  for VkDeviceGroupSwapchainCreateInfoKHX<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDeviceGroupSwapchainCreateInfoKHX as *const c_void
  }
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_swapchain_create_info_khx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDeviceGroupSwapchainCreateInfoKHX);
}

// feature: VK_EXT_validation_flags

/// Specify validation checks to disable
#[cfg(feature = "VK_EXT_validation_flags")]
pub use enums::VkValidationCheckEXT;

/// Specify validation checks to disable for a Vulkan instance
#[repr(C)]
#[cfg(feature = "VK_EXT_validation_flags")]
pub struct VkValidationFlagsEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  disabledValidationCheckCount: u32,
  pDisabledValidationChecks: *mut VkValidationCheckEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_validation_flags")]
impl<'l> VkValidationFlagsEXT<'l> {
  #[inline]
  pub fn new() -> VkValidationFlagsEXT<'l> {
    unsafe {
      VkValidationFlagsEXT {
        sType: VkStructureType::VALIDATION_FLAGS_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_disabled_validation_checks(mut self, value: &'l [VkValidationCheckEXT]) -> Self {
    self.disabledValidationCheckCount = value.len() as u32;
    unsafe {
      self.pDisabledValidationChecks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn disabled_validation_check_count(&self) -> u32 {
    self.disabledValidationCheckCount
  }
}
#[cfg(feature = "VK_EXT_validation_flags")]
impl<'l> Default for VkValidationFlagsEXT<'l> {
  fn default() -> VkValidationFlagsEXT<'l> {
    VkValidationFlagsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_flags")]
unsafe impl<'l> Struct for VkValidationFlagsEXT<'l> {}
#[cfg(feature = "VK_EXT_validation_flags")]
unsafe impl<'m, 'l: 'm> StructExtends<VkInstanceCreateInfo<'m>> for VkValidationFlagsEXT<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkValidationFlagsEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_validation_flags")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_validation_flags_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkValidationFlagsEXT);
}

// feature: VK_NN_vi_surface
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub type VkViSurfaceCreateFlagsNN = VkFlags;

/// Structure specifying parameters of a newly created VI surface object
#[repr(C)]
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub struct VkViSurfaceCreateInfoNN<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkViSurfaceCreateFlagsNN,
  window: *mut c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
impl<'l> VkViSurfaceCreateInfoNN<'l> {
  #[inline]
  pub fn new() -> VkViSurfaceCreateInfoNN<'l> {
    unsafe {
      VkViSurfaceCreateInfoNN {
        sType: VkStructureType::VI_SURFACE_CREATE_INFO_NN,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkViSurfaceCreateFlagsNN) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_window(mut self, value: *mut c_void) -> Self {
    self.window = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkViSurfaceCreateFlagsNN {
    self.flags
  }
  #[inline]
  pub fn window(&self) -> *mut c_void {
    self.window
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
impl<'l> Default for VkViSurfaceCreateInfoNN<'l> {
  fn default() -> VkViSurfaceCreateInfoNN<'l> {
    VkViSurfaceCreateInfoNN::new()
  }
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
unsafe impl<'l> Struct for VkViSurfaceCreateInfoNN<'l> {}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_vi_surface_create_info_nn() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkViSurfaceCreateInfoNN);
}

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

/// Structure specifying external memory handle type capabilities
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkExternalMemoryPropertiesKHR {
  pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsKHR,
  pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
  pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl VkExternalMemoryPropertiesKHR {
  #[inline]
  pub fn external_memory_features(&self) -> VkExternalMemoryFeatureFlagsKHR {
    self.externalMemoryFeatures
  }
  #[inline]
  pub fn export_from_imported_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.exportFromImportedHandleTypes
  }
  #[inline]
  pub fn compatible_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.compatibleHandleTypes
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl Struct for VkExternalMemoryPropertiesKHR {}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_memory_properties_khr() {
  assert_size!(12, VkExternalMemoryPropertiesKHR);
}

/// Structure specifying external image creation parameters
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl<'l> VkPhysicalDeviceExternalImageFormatInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalImageFormatInfoKHR<'l> {
    unsafe {
      VkPhysicalDeviceExternalImageFormatInfoKHR {
        sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl<'l> Default for VkPhysicalDeviceExternalImageFormatInfoKHR<'l> {
  fn default() -> VkPhysicalDeviceExternalImageFormatInfoKHR<'l> {
    VkPhysicalDeviceExternalImageFormatInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl<'l> Struct for VkPhysicalDeviceExternalImageFormatInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPhysicalDeviceImageFormatInfo2KHR<'m>>
  for VkPhysicalDeviceExternalImageFormatInfoKHR<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceExternalImageFormatInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_image_format_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPhysicalDeviceExternalImageFormatInfoKHR);
}

/// Structure specifying supported external handle properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkExternalImageFormatPropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl VkExternalImageFormatPropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn external_memory_properties(&self) -> &VkExternalMemoryPropertiesKHR {
    &self.externalMemoryProperties
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl Struct for VkExternalImageFormatPropertiesKHR {}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl StructExtends<VkImageFormatProperties2KHR> for VkExternalImageFormatPropertiesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExternalImageFormatPropertiesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_image_format_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkExternalImageFormatPropertiesKHR);
}

/// Structure specifying buffer creation parameters
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceExternalBufferInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkBufferCreateFlags,
  pub usage: VkBufferUsageFlags,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl<'l> VkPhysicalDeviceExternalBufferInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalBufferInfoKHR<'l> {
    unsafe {
      VkPhysicalDeviceExternalBufferInfoKHR {
        sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkBufferCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_usage(mut self, value: VkBufferUsageFlags) -> Self {
    self.usage = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkBufferCreateFlags {
    self.flags
  }
  #[inline]
  pub fn usage(&self) -> VkBufferUsageFlags {
    self.usage
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl<'l> Default for VkPhysicalDeviceExternalBufferInfoKHR<'l> {
  fn default() -> VkPhysicalDeviceExternalBufferInfoKHR<'l> {
    VkPhysicalDeviceExternalBufferInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl<'l> Struct for VkPhysicalDeviceExternalBufferInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_buffer_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPhysicalDeviceExternalBufferInfoKHR);
}

/// Structure specifying supported external handle capabilities
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkExternalBufferPropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub externalMemoryProperties: VkExternalMemoryPropertiesKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl VkExternalBufferPropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn external_memory_properties(&self) -> &VkExternalMemoryPropertiesKHR {
    &self.externalMemoryProperties
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl Struct for VkExternalBufferPropertiesKHR {}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_buffer_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkExternalBufferPropertiesKHR);
}

/// Structure specifying IDs related to the physical device
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceIDPropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub deviceUUID: [u8; enums::VK_UUID_SIZE as usize],
  pub driverUUID: [u8; enums::VK_UUID_SIZE as usize],
  pub deviceLUID: [u8; enums::VK_LUID_SIZE_KHR as usize],
  pub deviceNodeMask: u32,
  deviceLUIDValid: VkBool32,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl VkPhysicalDeviceIDPropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn device_uuid(&self) -> [u8; enums::VK_UUID_SIZE as usize] {
    self.deviceUUID
  }
  #[inline]
  pub fn driver_uuid(&self) -> [u8; enums::VK_UUID_SIZE as usize] {
    self.driverUUID
  }
  #[inline]
  pub fn device_luid(&self) -> [u8; enums::VK_LUID_SIZE_KHR as usize] {
    self.deviceLUID
  }
  #[inline]
  pub fn device_node_mask(&self) -> u32 {
    self.deviceNodeMask
  }
  #[inline]
  pub fn is_device_luid_valid(&self) -> bool {
    self.deviceLUIDValid != 0
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl Struct for VkPhysicalDeviceIDPropertiesKHR {}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceIDPropertiesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceIDPropertiesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_id_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 2, VkPhysicalDeviceIDPropertiesKHR);
}

// feature: VK_KHR_external_memory

/// Specify that an image may be backed by external memory
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExternalMemoryImageCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'l> VkExternalMemoryImageCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkExternalMemoryImageCreateInfoKHR<'l> {
    unsafe {
      VkExternalMemoryImageCreateInfoKHR {
        sType: VkStructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
  #[inline]
  pub fn handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'l> Default for VkExternalMemoryImageCreateInfoKHR<'l> {
  fn default() -> VkExternalMemoryImageCreateInfoKHR<'l> {
    VkExternalMemoryImageCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'l> Struct for VkExternalMemoryImageCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'m, 'l: 'm> StructExtends<VkImageCreateInfo<'m>> for VkExternalMemoryImageCreateInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExternalMemoryImageCreateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_memory_image_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkExternalMemoryImageCreateInfoKHR);
}

/// Specify that a buffer may be backed by external memory
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExternalMemoryBufferCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'l> VkExternalMemoryBufferCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkExternalMemoryBufferCreateInfoKHR<'l> {
    unsafe {
      VkExternalMemoryBufferCreateInfoKHR {
        sType: VkStructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
  #[inline]
  pub fn handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'l> Default for VkExternalMemoryBufferCreateInfoKHR<'l> {
  fn default() -> VkExternalMemoryBufferCreateInfoKHR<'l> {
    VkExternalMemoryBufferCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'l> Struct for VkExternalMemoryBufferCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'m, 'l: 'm> StructExtends<VkBufferCreateInfo<'m>> for VkExternalMemoryBufferCreateInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExternalMemoryBufferCreateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_memory_buffer_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkExternalMemoryBufferCreateInfoKHR);
}

/// Specify exportable handle types for a device memory object
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExportMemoryAllocateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'l> VkExportMemoryAllocateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkExportMemoryAllocateInfoKHR<'l> {
    unsafe {
      VkExportMemoryAllocateInfoKHR {
        sType: VkStructureType::EXPORT_MEMORY_ALLOCATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
  #[inline]
  pub fn handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'l> Default for VkExportMemoryAllocateInfoKHR<'l> {
  fn default() -> VkExportMemoryAllocateInfoKHR<'l> {
    VkExportMemoryAllocateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'l> Struct for VkExportMemoryAllocateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkExportMemoryAllocateInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExportMemoryAllocateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_export_memory_allocate_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkExportMemoryAllocateInfoKHR);
}

// feature: VK_KHR_external_memory_win32

/// import Win32 memory created on the same physical device
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportMemoryWin32HandleInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> VkImportMemoryWin32HandleInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkImportMemoryWin32HandleInfoKHR<'l> {
    unsafe {
      VkImportMemoryWin32HandleInfoKHR {
        sType: VkStructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn set_handle(mut self, value: wsi::win32::HANDLE) -> Self {
    self.handle = value;
    self
  }
  #[inline]
  pub fn set_name(mut self, value: wsi::win32::LPCWSTR) -> Self {
    self.name = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn handle(&self) -> wsi::win32::HANDLE {
    self.handle
  }
  #[inline]
  pub fn name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> Default for VkImportMemoryWin32HandleInfoKHR<'l> {
  fn default() -> VkImportMemoryWin32HandleInfoKHR<'l> {
    VkImportMemoryWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l> Struct for VkImportMemoryWin32HandleInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkImportMemoryWin32HandleInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkImportMemoryWin32HandleInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_memory_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkImportMemoryWin32HandleInfoKHR);
}

/// Structure specifying additional attributes of Windows handles exported from a
/// memory
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportMemoryWin32HandleInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> VkExportMemoryWin32HandleInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkExportMemoryWin32HandleInfoKHR<'l> {
    unsafe {
      VkExportMemoryWin32HandleInfoKHR {
        sType: VkStructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_attributes(mut self, value: *const wsi::win32::SECURITY_ATTRIBUTES) -> Self {
    self.pAttributes = value;
    self
  }
  #[inline]
  pub fn set_dw_access(mut self, value: wsi::win32::DWORD) -> Self {
    self.dwAccess = value;
    self
  }
  #[inline]
  pub fn set_name(mut self, value: wsi::win32::LPCWSTR) -> Self {
    self.name = value;
    self
  }
  #[inline]
  pub fn attributes(&self) -> *const wsi::win32::SECURITY_ATTRIBUTES {
    self.pAttributes
  }
  #[inline]
  pub fn dw_access(&self) -> wsi::win32::DWORD {
    self.dwAccess
  }
  #[inline]
  pub fn name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> Default for VkExportMemoryWin32HandleInfoKHR<'l> {
  fn default() -> VkExportMemoryWin32HandleInfoKHR<'l> {
    VkExportMemoryWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l> Struct for VkExportMemoryWin32HandleInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkExportMemoryWin32HandleInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExportMemoryWin32HandleInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_export_memory_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkExportMemoryWin32HandleInfoKHR);
}

/// Properties of External Memory Windows Handles
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkMemoryWin32HandlePropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkMemoryWin32HandlePropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn memory_type_bits(&self) -> u32 {
    self.memoryTypeBits
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl Struct for VkMemoryWin32HandlePropertiesKHR {}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_win32_handle_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkMemoryWin32HandlePropertiesKHR);
}

/// Structure describing a Win32 handle semaphore export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkMemoryGetWin32HandleInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  memory: u64,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> VkMemoryGetWin32HandleInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkMemoryGetWin32HandleInfoKHR<'l, 'h> {
    unsafe {
      VkMemoryGetWin32HandleInfoKHR {
        sType: VkStructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory<'h>) -> Self {
    unsafe {
      self.memory = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> Default for VkMemoryGetWin32HandleInfoKHR<'l, 'h> {
  fn default() -> VkMemoryGetWin32HandleInfoKHR<'l, 'h> {
    VkMemoryGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l, 'h: 'l> Struct for VkMemoryGetWin32HandleInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_get_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkMemoryGetWin32HandleInfoKHR);
}

// feature: VK_KHR_external_memory_fd

/// import memory created on the same physical device from a file descriptor
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkImportMemoryFdInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub fd: c_int,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl<'l> VkImportMemoryFdInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkImportMemoryFdInfoKHR<'l> {
    unsafe {
      VkImportMemoryFdInfoKHR {
        sType: VkStructureType::IMPORT_MEMORY_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn set_fd(mut self, value: c_int) -> Self {
    self.fd = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn fd(&self) -> c_int {
    self.fd
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl<'l> Default for VkImportMemoryFdInfoKHR<'l> {
  fn default() -> VkImportMemoryFdInfoKHR<'l> {
    VkImportMemoryFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl<'l> Struct for VkImportMemoryFdInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkImportMemoryFdInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkImportMemoryFdInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_memory_fd_info_khr() {
  let int_size = ::std::mem::size_of::<::std::os::raw::c_int>();
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + int_size * 1 + ptr_size * 3, VkImportMemoryFdInfoKHR);
}

/// Properties of External Memory File Descriptors
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkMemoryFdPropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl VkMemoryFdPropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn memory_type_bits(&self) -> u32 {
    self.memoryTypeBits
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl Struct for VkMemoryFdPropertiesKHR {}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_fd_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkMemoryFdPropertiesKHR);
}

/// Structure describing a POSIX FD semaphore export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkMemoryGetFdInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  memory: u64,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl<'l, 'h: 'l> VkMemoryGetFdInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkMemoryGetFdInfoKHR<'l, 'h> {
    unsafe {
      VkMemoryGetFdInfoKHR {
        sType: VkStructureType::MEMORY_GET_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory<'h>) -> Self {
    unsafe {
      self.memory = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl<'l, 'h: 'l> Default for VkMemoryGetFdInfoKHR<'l, 'h> {
  fn default() -> VkMemoryGetFdInfoKHR<'l, 'h> {
    VkMemoryGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl<'l, 'h: 'l> Struct for VkMemoryGetFdInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_get_fd_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkMemoryGetFdInfoKHR);
}

// feature: VK_KHR_win32_keyed_mutex

/// Use the Windows keyed mutex mechanism to synchronize work
#[repr(C)]
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  acquireCount: u32,
  pAcquireSyncs: *const u64,
  pAcquireKeys: *const u64,
  pAcquireTimeouts: *const u32,
  releaseCount: u32,
  pReleaseSyncs: *const u64,
  pReleaseKeys: *const u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> VkWin32KeyedMutexAcquireReleaseInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkWin32KeyedMutexAcquireReleaseInfoKHR<'l, 'h> {
    unsafe {
      VkWin32KeyedMutexAcquireReleaseInfoKHR {
        sType: VkStructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn acquire_count(&self) -> u32 {
    self.acquireCount
  }
  #[inline]
  pub fn release_count(&self) -> u32 {
    self.releaseCount
  }
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> Default for VkWin32KeyedMutexAcquireReleaseInfoKHR<'l, 'h> {
  fn default() -> VkWin32KeyedMutexAcquireReleaseInfoKHR<'l, 'h> {
    VkWin32KeyedMutexAcquireReleaseInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l, 'h: 'l> Struct for VkWin32KeyedMutexAcquireReleaseInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkSubmitInfo<'m, 'h>> for VkWin32KeyedMutexAcquireReleaseInfoKHR<'l, 'h> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkWin32KeyedMutexAcquireReleaseInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_win32_keyed_mutex_acquire_release_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 9, VkWin32KeyedMutexAcquireReleaseInfoKHR);
}

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

/// Structure specifying semaphore creation parameters.
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl<'l> VkPhysicalDeviceExternalSemaphoreInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalSemaphoreInfoKHR<'l> {
    unsafe {
      VkPhysicalDeviceExternalSemaphoreInfoKHR {
        sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalSemaphoreHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl<'l> Default for VkPhysicalDeviceExternalSemaphoreInfoKHR<'l> {
  fn default() -> VkPhysicalDeviceExternalSemaphoreInfoKHR<'l> {
    VkPhysicalDeviceExternalSemaphoreInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
unsafe impl<'l> Struct for VkPhysicalDeviceExternalSemaphoreInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_semaphore_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPhysicalDeviceExternalSemaphoreInfoKHR);
}

/// Structure describing supported external semaphore handle features
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub struct VkExternalSemaphorePropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
  pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
  pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl VkExternalSemaphorePropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn export_from_imported_handle_types(&self) -> VkExternalSemaphoreHandleTypeFlagsKHR {
    self.exportFromImportedHandleTypes
  }
  #[inline]
  pub fn compatible_handle_types(&self) -> VkExternalSemaphoreHandleTypeFlagsKHR {
    self.compatibleHandleTypes
  }
  #[inline]
  pub fn external_semaphore_features(&self) -> VkExternalSemaphoreFeatureFlagsKHR {
    self.externalSemaphoreFeatures
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
unsafe impl Struct for VkExternalSemaphorePropertiesKHR {}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_semaphore_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkExternalSemaphorePropertiesKHR);
}

// feature: VK_KHR_external_semaphore

/// Bitmask specifying additional parameters of semaphore payload import
#[cfg(feature = "VK_KHR_external_semaphore")]
pub use enums::VkSemaphoreImportFlagBitsKHR;

/// Bitmask of VkSemaphoreImportFlagBitsKHR
#[cfg(feature = "VK_KHR_external_semaphore")]
pub type VkSemaphoreImportFlagsKHR = VkSemaphoreImportFlagBitsKHR;

/// Structure specifying handle types that can be exported from a semaphore
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore")]
pub struct VkExportSemaphoreCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_semaphore")]
impl<'l> VkExportSemaphoreCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkExportSemaphoreCreateInfoKHR<'l> {
    unsafe {
      VkExportSemaphoreCreateInfoKHR {
        sType: VkStructureType::EXPORT_SEMAPHORE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalSemaphoreHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
  #[inline]
  pub fn handle_types(&self) -> VkExternalSemaphoreHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
impl<'l> Default for VkExportSemaphoreCreateInfoKHR<'l> {
  fn default() -> VkExportSemaphoreCreateInfoKHR<'l> {
    VkExportSemaphoreCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
unsafe impl<'l> Struct for VkExportSemaphoreCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_semaphore")]
unsafe impl<'m, 'l: 'm> StructExtends<VkSemaphoreCreateInfo<'m>> for VkExportSemaphoreCreateInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExportSemaphoreCreateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_export_semaphore_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkExportSemaphoreCreateInfoKHR);
}

// feature: VK_KHR_external_semaphore_win32

/// Structure specifying Windows handle to import to a semaphore
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportSemaphoreWin32HandleInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  semaphore: u64,
  pub flags: VkSemaphoreImportFlagsKHR,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> VkImportSemaphoreWin32HandleInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkImportSemaphoreWin32HandleInfoKHR<'l, 'h> {
    unsafe {
      VkImportSemaphoreWin32HandleInfoKHR {
        sType: VkStructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_semaphore(mut self, value: VkSemaphore<'h>) -> Self {
    unsafe {
      self.semaphore = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSemaphoreImportFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalSemaphoreHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn set_handle(mut self, value: wsi::win32::HANDLE) -> Self {
    self.handle = value;
    self
  }
  #[inline]
  pub fn set_name(mut self, value: wsi::win32::LPCWSTR) -> Self {
    self.name = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkSemaphoreImportFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn handle(&self) -> wsi::win32::HANDLE {
    self.handle
  }
  #[inline]
  pub fn name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> Default for VkImportSemaphoreWin32HandleInfoKHR<'l, 'h> {
  fn default() -> VkImportSemaphoreWin32HandleInfoKHR<'l, 'h> {
    VkImportSemaphoreWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l, 'h: 'l> Struct for VkImportSemaphoreWin32HandleInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_semaphore_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 4, VkImportSemaphoreWin32HandleInfoKHR);
}

/// Structure specifying additional attributes of Windows handles exported from a
/// semaphore
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportSemaphoreWin32HandleInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> VkExportSemaphoreWin32HandleInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkExportSemaphoreWin32HandleInfoKHR<'l> {
    unsafe {
      VkExportSemaphoreWin32HandleInfoKHR {
        sType: VkStructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_attributes(mut self, value: *const wsi::win32::SECURITY_ATTRIBUTES) -> Self {
    self.pAttributes = value;
    self
  }
  #[inline]
  pub fn set_dw_access(mut self, value: wsi::win32::DWORD) -> Self {
    self.dwAccess = value;
    self
  }
  #[inline]
  pub fn set_name(mut self, value: wsi::win32::LPCWSTR) -> Self {
    self.name = value;
    self
  }
  #[inline]
  pub fn attributes(&self) -> *const wsi::win32::SECURITY_ATTRIBUTES {
    self.pAttributes
  }
  #[inline]
  pub fn dw_access(&self) -> wsi::win32::DWORD {
    self.dwAccess
  }
  #[inline]
  pub fn name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> Default for VkExportSemaphoreWin32HandleInfoKHR<'l> {
  fn default() -> VkExportSemaphoreWin32HandleInfoKHR<'l> {
    VkExportSemaphoreWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l> Struct for VkExportSemaphoreWin32HandleInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm> StructExtends<VkSemaphoreCreateInfo<'m>> for VkExportSemaphoreWin32HandleInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExportSemaphoreWin32HandleInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_export_semaphore_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkExportSemaphoreWin32HandleInfoKHR);
}

/// Structure specifying values for Direct3D 12 fence-backed semaphores
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkD3D12FenceSubmitInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreValuesCount: u32,
  pWaitSemaphoreValues: *const u64,
  signalSemaphoreValuesCount: u32,
  pSignalSemaphoreValues: *const u64,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> VkD3D12FenceSubmitInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkD3D12FenceSubmitInfoKHR<'l> {
    unsafe {
      VkD3D12FenceSubmitInfoKHR {
        sType: VkStructureType::D3D12_FENCE_SUBMIT_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_wait_semaphore_values(mut self, value: &'l [u64]) -> Self {
    self.waitSemaphoreValuesCount = value.len() as u32;
    unsafe {
      self.pWaitSemaphoreValues = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphore_values(mut self, value: &'l [u64]) -> Self {
    self.signalSemaphoreValuesCount = value.len() as u32;
    unsafe {
      self.pSignalSemaphoreValues = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn wait_semaphore_values_count(&self) -> u32 {
    self.waitSemaphoreValuesCount
  }
  #[inline]
  pub fn signal_semaphore_values_count(&self) -> u32 {
    self.signalSemaphoreValuesCount
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> Default for VkD3D12FenceSubmitInfoKHR<'l> {
  fn default() -> VkD3D12FenceSubmitInfoKHR<'l> {
    VkD3D12FenceSubmitInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l> Struct for VkD3D12FenceSubmitInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkSubmitInfo<'m, 'h>> for VkD3D12FenceSubmitInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkD3D12FenceSubmitInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_d3_d12_fence_submit_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 6, VkD3D12FenceSubmitInfoKHR);
}

/// Structure describing a Win32 handle semaphore export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkSemaphoreGetWin32HandleInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  semaphore: u64,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> VkSemaphoreGetWin32HandleInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkSemaphoreGetWin32HandleInfoKHR<'l, 'h> {
    unsafe {
      VkSemaphoreGetWin32HandleInfoKHR {
        sType: VkStructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_semaphore(mut self, value: VkSemaphore<'h>) -> Self {
    unsafe {
      self.semaphore = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalSemaphoreHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> Default for VkSemaphoreGetWin32HandleInfoKHR<'l, 'h> {
  fn default() -> VkSemaphoreGetWin32HandleInfoKHR<'l, 'h> {
    VkSemaphoreGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l, 'h: 'l> Struct for VkSemaphoreGetWin32HandleInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_get_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkSemaphoreGetWin32HandleInfoKHR);
}

// feature: VK_KHR_external_semaphore_fd

/// Structure specifying POSIX file descriptor to import to a semaphore
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub struct VkImportSemaphoreFdInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  semaphore: u64,
  pub flags: VkSemaphoreImportFlagsKHR,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  pub fd: c_int,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl<'l, 'h: 'l> VkImportSemaphoreFdInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkImportSemaphoreFdInfoKHR<'l, 'h> {
    unsafe {
      VkImportSemaphoreFdInfoKHR {
        sType: VkStructureType::IMPORT_SEMAPHORE_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_semaphore(mut self, value: VkSemaphore<'h>) -> Self {
    unsafe {
      self.semaphore = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSemaphoreImportFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalSemaphoreHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn set_fd(mut self, value: c_int) -> Self {
    self.fd = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkSemaphoreImportFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn fd(&self) -> c_int {
    self.fd
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl<'l, 'h: 'l> Default for VkImportSemaphoreFdInfoKHR<'l, 'h> {
  fn default() -> VkImportSemaphoreFdInfoKHR<'l, 'h> {
    VkImportSemaphoreFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
unsafe impl<'l, 'h: 'l> Struct for VkImportSemaphoreFdInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_semaphore_fd_info_khr() {
  let int_size = ::std::mem::size_of::<::std::os::raw::c_int>();
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + int_size * 1 + ptr_size * 2, VkImportSemaphoreFdInfoKHR);
}

/// Structure describing a POSIX FD semaphore export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub struct VkSemaphoreGetFdInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  semaphore: u64,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl<'l, 'h: 'l> VkSemaphoreGetFdInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkSemaphoreGetFdInfoKHR<'l, 'h> {
    unsafe {
      VkSemaphoreGetFdInfoKHR {
        sType: VkStructureType::SEMAPHORE_GET_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_semaphore(mut self, value: VkSemaphore<'h>) -> Self {
    unsafe {
      self.semaphore = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalSemaphoreHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl<'l, 'h: 'l> Default for VkSemaphoreGetFdInfoKHR<'l, 'h> {
  fn default() -> VkSemaphoreGetFdInfoKHR<'l, 'h> {
    VkSemaphoreGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
unsafe impl<'l, 'h: 'l> Struct for VkSemaphoreGetFdInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_get_fd_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkSemaphoreGetFdInfoKHR);
}

// feature: VK_KHR_push_descriptor

/// Structure describing push descriptor limits that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_push_descriptor")]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub maxPushDescriptors: u32,
}
#[cfg(feature = "VK_KHR_push_descriptor")]
impl VkPhysicalDevicePushDescriptorPropertiesKHR {
  #[inline]
  pub fn new() -> VkPhysicalDevicePushDescriptorPropertiesKHR {
    unsafe {
      VkPhysicalDevicePushDescriptorPropertiesKHR {
        sType: VkStructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_max_push_descriptors(mut self, value: u32) -> Self {
    self.maxPushDescriptors = value;
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn max_push_descriptors(&self) -> u32 {
    self.maxPushDescriptors
  }
}
#[cfg(feature = "VK_KHR_push_descriptor")]
impl Default for VkPhysicalDevicePushDescriptorPropertiesKHR {
  fn default() -> VkPhysicalDevicePushDescriptorPropertiesKHR {
    VkPhysicalDevicePushDescriptorPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_push_descriptor")]
unsafe impl Struct for VkPhysicalDevicePushDescriptorPropertiesKHR {}
#[cfg(feature = "VK_KHR_push_descriptor")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDevicePushDescriptorPropertiesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDevicePushDescriptorPropertiesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_push_descriptor")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_push_descriptor_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPhysicalDevicePushDescriptorPropertiesKHR);
}

// feature: VK_KHR_16bit_storage
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_16bit_storage")]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  storageBuffer16BitAccess: VkBool32,
  uniformAndStorageBuffer16BitAccess: VkBool32,
  storagePushConstant16: VkBool32,
  storageInputOutput16: VkBool32,
}
#[cfg(feature = "VK_KHR_16bit_storage")]
impl VkPhysicalDevice16BitStorageFeaturesKHR {
  #[inline]
  pub fn new() -> VkPhysicalDevice16BitStorageFeaturesKHR {
    unsafe {
      VkPhysicalDevice16BitStorageFeaturesKHR {
        sType: VkStructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_storage_buffer16_bit_access(mut self, value: bool) -> Self {
    unsafe {
      self.storageBuffer16BitAccess = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_uniform_and_storage_buffer16_bit_access(mut self, value: bool) -> Self {
    unsafe {
      self.uniformAndStorageBuffer16BitAccess = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_storage_push_constant16(mut self, value: bool) -> Self {
    unsafe {
      self.storagePushConstant16 = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_storage_input_output16(mut self, value: bool) -> Self {
    unsafe {
      self.storageInputOutput16 = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_storage_buffer16_bit_access(&self) -> bool {
    self.storageBuffer16BitAccess != 0
  }
  #[inline]
  pub fn is_uniform_and_storage_buffer16_bit_access(&self) -> bool {
    self.uniformAndStorageBuffer16BitAccess != 0
  }
  #[inline]
  pub fn is_storage_push_constant16(&self) -> bool {
    self.storagePushConstant16 != 0
  }
  #[inline]
  pub fn is_storage_input_output16(&self) -> bool {
    self.storageInputOutput16 != 0
  }
}
#[cfg(feature = "VK_KHR_16bit_storage")]
impl Default for VkPhysicalDevice16BitStorageFeaturesKHR {
  fn default() -> VkPhysicalDevice16BitStorageFeaturesKHR {
    VkPhysicalDevice16BitStorageFeaturesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_16bit_storage")]
unsafe impl Struct for VkPhysicalDevice16BitStorageFeaturesKHR {}
#[cfg(feature = "VK_KHR_16bit_storage")]
unsafe impl StructExtends<VkPhysicalDeviceFeatures2KHR> for VkPhysicalDevice16BitStorageFeaturesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDevice16BitStorageFeaturesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_16bit_storage")]
unsafe impl<'m> StructExtends<VkDeviceCreateInfo<'m>> for VkPhysicalDevice16BitStorageFeaturesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDevice16BitStorageFeaturesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_16bit_storage")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device16_bit_storage_features_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 2, VkPhysicalDevice16BitStorageFeaturesKHR);
}

// feature: VK_KHR_incremental_present

/// Structure containing a rectangle, including layer, changed by vkQueuePresentKHR
/// for a given VkImage
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkRectLayerKHR {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
  pub layer: u32,
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl VkRectLayerKHR {
  #[inline]
  pub fn new() -> VkRectLayerKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_offset(mut self, value: VkOffset2D) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_extent(mut self, value: VkExtent2D) -> Self {
    self.extent = value;
    self
  }
  #[inline]
  pub fn set_layer(mut self, value: u32) -> Self {
    self.layer = value;
    self
  }
  #[inline]
  pub fn offset(&self) -> &VkOffset2D {
    &self.offset
  }
  #[inline]
  pub fn extent(&self) -> &VkExtent2D {
    &self.extent
  }
  #[inline]
  pub fn layer(&self) -> u32 {
    self.layer
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl Default for VkRectLayerKHR {
  fn default() -> VkRectLayerKHR {
    VkRectLayerKHR::new()
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
unsafe impl Struct for VkRectLayerKHR {}
#[cfg(feature = "VK_KHR_incremental_present")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_rect_layer_khr() {
  assert_size!(20, VkRectLayerKHR);
}

/// Structure containing rectangular region changed by vkQueuePresentKHR for a given
/// VkImage
#[repr(C)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkPresentRegionKHR<'l> {
  rectangleCount: u32,
  pRectangles: *const VkRectLayerKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'l> VkPresentRegionKHR<'l> {
  #[inline]
  pub fn new() -> VkPresentRegionKHR<'l> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_rectangles(mut self, value: &'l [VkRectLayerKHR]) -> Self {
    self.rectangleCount = value.len() as u32;
    unsafe {
      self.pRectangles = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn rectangle_count(&self) -> u32 {
    self.rectangleCount
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'l> Default for VkPresentRegionKHR<'l> {
  fn default() -> VkPresentRegionKHR<'l> {
    VkPresentRegionKHR::new()
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
unsafe impl<'l> Struct for VkPresentRegionKHR<'l> {}
#[cfg(feature = "VK_KHR_incremental_present")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_region_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 2, VkPresentRegionKHR);
}

/// Structure hint of rectangular regions changed by vkQueuePresentKHR
#[repr(C)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkPresentRegionsKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchainCount: u32,
  pRegions: *const VkPresentRegionKHR<'l>,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'l> VkPresentRegionsKHR<'l> {
  #[inline]
  pub fn new() -> VkPresentRegionsKHR<'l> {
    unsafe {
      VkPresentRegionsKHR {
        sType: VkStructureType::PRESENT_REGIONS_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_regions(mut self, value: &'l [VkPresentRegionKHR<'l>]) -> Self {
    self.swapchainCount = value.len() as u32;
    unsafe {
      self.pRegions = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn swapchain_count(&self) -> u32 {
    self.swapchainCount
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'l> Default for VkPresentRegionsKHR<'l> {
  fn default() -> VkPresentRegionsKHR<'l> {
    VkPresentRegionsKHR::new()
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
unsafe impl<'l> Struct for VkPresentRegionsKHR<'l> {}
#[cfg(feature = "VK_KHR_incremental_present")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkPresentInfoKHR<'m, 'h>> for VkPresentRegionsKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPresentRegionsKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_regions_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkPresentRegionsKHR);
}

// feature: VK_KHR_descriptor_update_template
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDescriptorUpdateTemplateKHR__ {}

/// Opaque handle to a descriptor update template
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type VkDescriptorUpdateTemplateKHR<'l> = VkNonDispatchableHandle<'l, VkDescriptorUpdateTemplateKHR__>;

/// Reserved for future use
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkFlags;

/// Indicates the valid usage of the descriptor update template
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub use enums::VkDescriptorUpdateTemplateTypeKHR;

/// Describes a single descriptor update of the descriptor update template
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
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl VkDescriptorUpdateTemplateEntryKHR {
  #[inline]
  pub fn new() -> VkDescriptorUpdateTemplateEntryKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_dst_binding(mut self, value: u32) -> Self {
    self.dstBinding = value;
    self
  }
  #[inline]
  pub fn set_dst_array_element(mut self, value: u32) -> Self {
    self.dstArrayElement = value;
    self
  }
  #[inline]
  pub fn set_descriptor_count(mut self, value: u32) -> Self {
    self.descriptorCount = value;
    self
  }
  #[inline]
  pub fn set_descriptor_type(mut self, value: VkDescriptorType) -> Self {
    self.descriptorType = value;
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: usize) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn set_stride(mut self, value: usize) -> Self {
    self.stride = value;
    self
  }
  #[inline]
  pub fn dst_binding(&self) -> u32 {
    self.dstBinding
  }
  #[inline]
  pub fn dst_array_element(&self) -> u32 {
    self.dstArrayElement
  }
  #[inline]
  pub fn descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
  #[inline]
  pub fn descriptor_type(&self) -> VkDescriptorType {
    self.descriptorType
  }
  #[inline]
  pub fn offset(&self) -> usize {
    self.offset
  }
  #[inline]
  pub fn stride(&self) -> usize {
    self.stride
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl Default for VkDescriptorUpdateTemplateEntryKHR {
  fn default() -> VkDescriptorUpdateTemplateEntryKHR {
    VkDescriptorUpdateTemplateEntryKHR::new()
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
unsafe impl Struct for VkDescriptorUpdateTemplateEntryKHR {}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_update_template_entry_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 2, VkDescriptorUpdateTemplateEntryKHR);
}

/// Structure specifying parameters of a newly created descriptor update template
#[repr(C)]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
  descriptorUpdateEntryCount: u32,
  pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntryKHR,
  pub templateType: VkDescriptorUpdateTemplateTypeKHR,
  descriptorSetLayout: u64,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pipelineLayout: u64,
  pub set: u32,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<'l, 'h: 'l> VkDescriptorUpdateTemplateCreateInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkDescriptorUpdateTemplateCreateInfoKHR<'l, 'h> {
    unsafe {
      VkDescriptorUpdateTemplateCreateInfoKHR {
        sType: VkStructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDescriptorUpdateTemplateCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_descriptor_update_entries(mut self, value: &'l [VkDescriptorUpdateTemplateEntryKHR]) -> Self {
    self.descriptorUpdateEntryCount = value.len() as u32;
    unsafe {
      self.pDescriptorUpdateEntries = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_template_type(mut self, value: VkDescriptorUpdateTemplateTypeKHR) -> Self {
    self.templateType = value;
    self
  }
  #[inline]
  pub fn set_descriptor_set_layout(mut self, value: Option<VkDescriptorSetLayout<'h>>) -> Self {
    unsafe {
      self.descriptorSetLayout = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_pipeline_bind_point(mut self, value: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = value;
    self
  }
  #[inline]
  pub fn set_pipeline_layout(mut self, value: Option<VkPipelineLayout<'h>>) -> Self {
    unsafe {
      self.pipelineLayout = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_set(mut self, value: u32) -> Self {
    self.set = value;
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn flags(&self) -> VkDescriptorUpdateTemplateCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn descriptor_update_entry_count(&self) -> u32 {
    self.descriptorUpdateEntryCount
  }
  #[inline]
  pub fn template_type(&self) -> VkDescriptorUpdateTemplateTypeKHR {
    self.templateType
  }
  #[inline]
  pub fn pipeline_bind_point(&self) -> VkPipelineBindPoint {
    self.pipelineBindPoint
  }
  #[inline]
  pub fn set(&self) -> u32 {
    self.set
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<'l, 'h: 'l> Default for VkDescriptorUpdateTemplateCreateInfoKHR<'l, 'h> {
  fn default() -> VkDescriptorUpdateTemplateCreateInfoKHR<'l, 'h> {
    VkDescriptorUpdateTemplateCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
unsafe impl<'l, 'h: 'l> Struct for VkDescriptorUpdateTemplateCreateInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_update_template_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 6, VkDescriptorUpdateTemplateCreateInfoKHR);
}

// feature: VK_NVX_device_generated_commands
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkObjectTableNVX__ {}

/// Opaque handle to an object table
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkObjectTableNVX<'l> = VkNonDispatchableHandle<'l, VkObjectTableNVX__>;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkIndirectCommandsLayoutNVX__ {}

/// Opaque handle to an indirect commands layout object
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkIndirectCommandsLayoutNVX<'l> = VkNonDispatchableHandle<'l, VkIndirectCommandsLayoutNVX__>;

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

/// Structure specifying physical device support
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkDeviceGeneratedCommandsFeaturesNVX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  computeBindingPointSupport: VkBool32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l> VkDeviceGeneratedCommandsFeaturesNVX<'l> {
  #[inline]
  pub fn new() -> VkDeviceGeneratedCommandsFeaturesNVX<'l> {
    unsafe {
      VkDeviceGeneratedCommandsFeaturesNVX {
        sType: VkStructureType::DEVICE_GENERATED_COMMANDS_FEATURES_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_compute_binding_point_support(mut self, value: bool) -> Self {
    unsafe {
      self.computeBindingPointSupport = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn is_compute_binding_point_support(&self) -> bool {
    self.computeBindingPointSupport != 0
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l> Default for VkDeviceGeneratedCommandsFeaturesNVX<'l> {
  fn default() -> VkDeviceGeneratedCommandsFeaturesNVX<'l> {
    VkDeviceGeneratedCommandsFeaturesNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'l> Struct for VkDeviceGeneratedCommandsFeaturesNVX<'l> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_generated_commands_features_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDeviceGeneratedCommandsFeaturesNVX);
}

/// Structure specifying physical device limits
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkDeviceGeneratedCommandsLimitsNVX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub maxIndirectCommandsLayoutTokenCount: u32,
  pub maxObjectEntryCounts: u32,
  pub minSequenceCountBufferOffsetAlignment: u32,
  pub minSequenceIndexBufferOffsetAlignment: u32,
  pub minCommandsTokenBufferOffsetAlignment: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l> VkDeviceGeneratedCommandsLimitsNVX<'l> {
  #[inline]
  pub fn new() -> VkDeviceGeneratedCommandsLimitsNVX<'l> {
    unsafe {
      VkDeviceGeneratedCommandsLimitsNVX {
        sType: VkStructureType::DEVICE_GENERATED_COMMANDS_LIMITS_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_max_indirect_commands_layout_token_count(mut self, value: u32) -> Self {
    self.maxIndirectCommandsLayoutTokenCount = value;
    self
  }
  #[inline]
  pub fn set_max_object_entry_counts(mut self, value: u32) -> Self {
    self.maxObjectEntryCounts = value;
    self
  }
  #[inline]
  pub fn set_min_sequence_count_buffer_offset_alignment(mut self, value: u32) -> Self {
    self.minSequenceCountBufferOffsetAlignment = value;
    self
  }
  #[inline]
  pub fn set_min_sequence_index_buffer_offset_alignment(mut self, value: u32) -> Self {
    self.minSequenceIndexBufferOffsetAlignment = value;
    self
  }
  #[inline]
  pub fn set_min_commands_token_buffer_offset_alignment(mut self, value: u32) -> Self {
    self.minCommandsTokenBufferOffsetAlignment = value;
    self
  }
  #[inline]
  pub fn max_indirect_commands_layout_token_count(&self) -> u32 {
    self.maxIndirectCommandsLayoutTokenCount
  }
  #[inline]
  pub fn max_object_entry_counts(&self) -> u32 {
    self.maxObjectEntryCounts
  }
  #[inline]
  pub fn min_sequence_count_buffer_offset_alignment(&self) -> u32 {
    self.minSequenceCountBufferOffsetAlignment
  }
  #[inline]
  pub fn min_sequence_index_buffer_offset_alignment(&self) -> u32 {
    self.minSequenceIndexBufferOffsetAlignment
  }
  #[inline]
  pub fn min_commands_token_buffer_offset_alignment(&self) -> u32 {
    self.minCommandsTokenBufferOffsetAlignment
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l> Default for VkDeviceGeneratedCommandsLimitsNVX<'l> {
  fn default() -> VkDeviceGeneratedCommandsLimitsNVX<'l> {
    VkDeviceGeneratedCommandsLimitsNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'l> Struct for VkDeviceGeneratedCommandsLimitsNVX<'l> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_generated_commands_limits_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkDeviceGeneratedCommandsLimitsNVX);
}

/// Structure specifying parameters for the reservation of command buffer space
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsTokenNVX<'h> {
  pub tokenType: VkIndirectCommandsTokenTypeNVX,
  buffer: u64,
  pub offset: VkDeviceSize,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> VkIndirectCommandsTokenNVX<'h> {
  #[inline]
  pub fn new() -> VkIndirectCommandsTokenNVX<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_token_type(mut self, value: VkIndirectCommandsTokenTypeNVX) -> Self {
    self.tokenType = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_offset(mut self, value: VkDeviceSize) -> Self {
    self.offset = value;
    self
  }
  #[inline]
  pub fn token_type(&self) -> VkIndirectCommandsTokenTypeNVX {
    self.tokenType
  }
  #[inline]
  pub fn offset(&self) -> VkDeviceSize {
    self.offset
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> Default for VkIndirectCommandsTokenNVX<'h> {
  fn default() -> VkIndirectCommandsTokenNVX<'h> {
    VkIndirectCommandsTokenNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'h> Struct for VkIndirectCommandsTokenNVX<'h> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_indirect_commands_token_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 1, VkIndirectCommandsTokenNVX);
}

/// Struct specifying the details of an indirect command layout token
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsLayoutTokenNVX {
  pub tokenType: VkIndirectCommandsTokenTypeNVX,
  pub bindingUnit: u32,
  pub dynamicCount: u32,
  pub divisor: u32,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkIndirectCommandsLayoutTokenNVX {
  #[inline]
  pub fn new() -> VkIndirectCommandsLayoutTokenNVX {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_token_type(mut self, value: VkIndirectCommandsTokenTypeNVX) -> Self {
    self.tokenType = value;
    self
  }
  #[inline]
  pub fn set_binding_unit(mut self, value: u32) -> Self {
    self.bindingUnit = value;
    self
  }
  #[inline]
  pub fn set_dynamic_count(mut self, value: u32) -> Self {
    self.dynamicCount = value;
    self
  }
  #[inline]
  pub fn set_divisor(mut self, value: u32) -> Self {
    self.divisor = value;
    self
  }
  #[inline]
  pub fn token_type(&self) -> VkIndirectCommandsTokenTypeNVX {
    self.tokenType
  }
  #[inline]
  pub fn binding_unit(&self) -> u32 {
    self.bindingUnit
  }
  #[inline]
  pub fn dynamic_count(&self) -> u32 {
    self.dynamicCount
  }
  #[inline]
  pub fn divisor(&self) -> u32 {
    self.divisor
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkIndirectCommandsLayoutTokenNVX {
  fn default() -> VkIndirectCommandsLayoutTokenNVX {
    VkIndirectCommandsLayoutTokenNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl Struct for VkIndirectCommandsLayoutTokenNVX {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_indirect_commands_layout_token_nvx() {
  assert_size!(16, VkIndirectCommandsLayoutTokenNVX);
}

/// Structure specifying the parameters of a newly created indirect commands layout
/// object
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsLayoutCreateInfoNVX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub flags: VkIndirectCommandsLayoutUsageFlagsNVX,
  tokenCount: u32,
  pTokens: *const VkIndirectCommandsLayoutTokenNVX,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l> VkIndirectCommandsLayoutCreateInfoNVX<'l> {
  #[inline]
  pub fn new() -> VkIndirectCommandsLayoutCreateInfoNVX<'l> {
    unsafe {
      VkIndirectCommandsLayoutCreateInfoNVX {
        sType: VkStructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_pipeline_bind_point(mut self, value: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkIndirectCommandsLayoutUsageFlagsNVX) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_tokens(mut self, value: &'l [VkIndirectCommandsLayoutTokenNVX]) -> Self {
    self.tokenCount = value.len() as u32;
    unsafe {
      self.pTokens = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn pipeline_bind_point(&self) -> VkPipelineBindPoint {
    self.pipelineBindPoint
  }
  #[inline]
  pub fn flags(&self) -> VkIndirectCommandsLayoutUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn token_count(&self) -> u32 {
    self.tokenCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l> Default for VkIndirectCommandsLayoutCreateInfoNVX<'l> {
  fn default() -> VkIndirectCommandsLayoutCreateInfoNVX<'l> {
    VkIndirectCommandsLayoutCreateInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'l> Struct for VkIndirectCommandsLayoutCreateInfoNVX<'l> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_indirect_commands_layout_create_info_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 4, VkIndirectCommandsLayoutCreateInfoNVX);
}

/// Structure specifying parameters for the generation of commands
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkCmdProcessCommandsInfoNVX<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  objectTable: u64,
  indirectCommandsLayout: u64,
  indirectCommandsTokenCount: u32,
  pIndirectCommandsTokens: *const VkIndirectCommandsTokenNVX<'h>,
  pub maxSequencesCount: u32,
  targetCommandBuffer: usize,
  sequencesCountBuffer: u64,
  pub sequencesCountOffset: VkDeviceSize,
  sequencesIndexBuffer: u64,
  pub sequencesIndexOffset: VkDeviceSize,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l, 'h: 'l> VkCmdProcessCommandsInfoNVX<'l, 'h> {
  #[inline]
  pub fn new() -> VkCmdProcessCommandsInfoNVX<'l, 'h> {
    unsafe {
      VkCmdProcessCommandsInfoNVX {
        sType: VkStructureType::CMD_PROCESS_COMMANDS_INFO_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_object_table(mut self, value: VkObjectTableNVX<'h>) -> Self {
    unsafe {
      self.objectTable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_indirect_commands_layout(mut self, value: VkIndirectCommandsLayoutNVX<'h>) -> Self {
    unsafe {
      self.indirectCommandsLayout = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_indirect_commands_tokens(mut self, value: &'l [VkIndirectCommandsTokenNVX<'h>]) -> Self {
    self.indirectCommandsTokenCount = value.len() as u32;
    unsafe {
      self.pIndirectCommandsTokens = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_max_sequences_count(mut self, value: u32) -> Self {
    self.maxSequencesCount = value;
    self
  }
  #[inline]
  pub fn set_target_command_buffer(mut self, value: Option<VkCommandBuffer<'h>>) -> Self {
    unsafe {
      self.targetCommandBuffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sequences_count_buffer(mut self, value: Option<VkBuffer<'h>>) -> Self {
    unsafe {
      self.sequencesCountBuffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sequences_count_offset(mut self, value: VkDeviceSize) -> Self {
    self.sequencesCountOffset = value;
    self
  }
  #[inline]
  pub fn set_sequences_index_buffer(mut self, value: Option<VkBuffer<'h>>) -> Self {
    unsafe {
      self.sequencesIndexBuffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sequences_index_offset(mut self, value: VkDeviceSize) -> Self {
    self.sequencesIndexOffset = value;
    self
  }
  #[inline]
  pub fn indirect_commands_token_count(&self) -> u32 {
    self.indirectCommandsTokenCount
  }
  #[inline]
  pub fn max_sequences_count(&self) -> u32 {
    self.maxSequencesCount
  }
  #[inline]
  pub fn sequences_count_offset(&self) -> VkDeviceSize {
    self.sequencesCountOffset
  }
  #[inline]
  pub fn sequences_index_offset(&self) -> VkDeviceSize {
    self.sequencesIndexOffset
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l, 'h: 'l> Default for VkCmdProcessCommandsInfoNVX<'l, 'h> {
  fn default() -> VkCmdProcessCommandsInfoNVX<'l, 'h> {
    VkCmdProcessCommandsInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'l, 'h: 'l> Struct for VkCmdProcessCommandsInfoNVX<'l, 'h> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_cmd_process_commands_info_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 6, VkCmdProcessCommandsInfoNVX);
}

/// Structure specifying parameters for the reservation of command buffer space
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkCmdReserveSpaceForCommandsInfoNVX<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  objectTable: u64,
  indirectCommandsLayout: u64,
  pub maxSequencesCount: u32,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l, 'h: 'l> VkCmdReserveSpaceForCommandsInfoNVX<'l, 'h> {
  #[inline]
  pub fn new() -> VkCmdReserveSpaceForCommandsInfoNVX<'l, 'h> {
    unsafe {
      VkCmdReserveSpaceForCommandsInfoNVX {
        sType: VkStructureType::CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_object_table(mut self, value: VkObjectTableNVX<'h>) -> Self {
    unsafe {
      self.objectTable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_indirect_commands_layout(mut self, value: VkIndirectCommandsLayoutNVX<'h>) -> Self {
    unsafe {
      self.indirectCommandsLayout = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_max_sequences_count(mut self, value: u32) -> Self {
    self.maxSequencesCount = value;
    self
  }
  #[inline]
  pub fn max_sequences_count(&self) -> u32 {
    self.maxSequencesCount
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l, 'h: 'l> Default for VkCmdReserveSpaceForCommandsInfoNVX<'l, 'h> {
  fn default() -> VkCmdReserveSpaceForCommandsInfoNVX<'l, 'h> {
    VkCmdReserveSpaceForCommandsInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'l, 'h: 'l> Struct for VkCmdReserveSpaceForCommandsInfoNVX<'l, 'h> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_cmd_reserve_space_for_commands_info_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkCmdReserveSpaceForCommandsInfoNVX);
}

/// Structure specifying the parameters of a newly created object table
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableCreateInfoNVX<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  objectCount: u32,
  pObjectEntryTypes: *const VkObjectEntryTypeNVX,
  pObjectEntryCounts: *const u32,
  pObjectEntryUsageFlags: *const VkObjectEntryUsageFlagsNVX,
  pub maxUniformBuffersPerDescriptor: u32,
  pub maxStorageBuffersPerDescriptor: u32,
  pub maxStorageImagesPerDescriptor: u32,
  pub maxSampledImagesPerDescriptor: u32,
  pub maxPipelineLayouts: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l> VkObjectTableCreateInfoNVX<'l> {
  #[inline]
  pub fn new() -> VkObjectTableCreateInfoNVX<'l> {
    unsafe {
      VkObjectTableCreateInfoNVX {
        sType: VkStructureType::OBJECT_TABLE_CREATE_INFO_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_max_uniform_buffers_per_descriptor(mut self, value: u32) -> Self {
    self.maxUniformBuffersPerDescriptor = value;
    self
  }
  #[inline]
  pub fn set_max_storage_buffers_per_descriptor(mut self, value: u32) -> Self {
    self.maxStorageBuffersPerDescriptor = value;
    self
  }
  #[inline]
  pub fn set_max_storage_images_per_descriptor(mut self, value: u32) -> Self {
    self.maxStorageImagesPerDescriptor = value;
    self
  }
  #[inline]
  pub fn set_max_sampled_images_per_descriptor(mut self, value: u32) -> Self {
    self.maxSampledImagesPerDescriptor = value;
    self
  }
  #[inline]
  pub fn set_max_pipeline_layouts(mut self, value: u32) -> Self {
    self.maxPipelineLayouts = value;
    self
  }
  #[inline]
  pub fn object_count(&self) -> u32 {
    self.objectCount
  }
  #[inline]
  pub fn max_uniform_buffers_per_descriptor(&self) -> u32 {
    self.maxUniformBuffersPerDescriptor
  }
  #[inline]
  pub fn max_storage_buffers_per_descriptor(&self) -> u32 {
    self.maxStorageBuffersPerDescriptor
  }
  #[inline]
  pub fn max_storage_images_per_descriptor(&self) -> u32 {
    self.maxStorageImagesPerDescriptor
  }
  #[inline]
  pub fn max_sampled_images_per_descriptor(&self) -> u32 {
    self.maxSampledImagesPerDescriptor
  }
  #[inline]
  pub fn max_pipeline_layouts(&self) -> u32 {
    self.maxPipelineLayouts
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'l> Default for VkObjectTableCreateInfoNVX<'l> {
  fn default() -> VkObjectTableCreateInfoNVX<'l> {
    VkObjectTableCreateInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'l> Struct for VkObjectTableCreateInfoNVX<'l> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_create_info_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 7, VkObjectTableCreateInfoNVX);
}

/// Common parameters of an object table resource entry
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkObjectTableEntryNVX {
  #[inline]
  pub fn new() -> VkObjectTableEntryNVX {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_type(mut self, value: VkObjectEntryTypeNVX) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkObjectEntryUsageFlagsNVX) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn get_type(&self) -> VkObjectEntryTypeNVX {
    self.eType
  }
  #[inline]
  pub fn flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTableEntryNVX {
  fn default() -> VkObjectTableEntryNVX {
    VkObjectTableEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl Struct for VkObjectTableEntryNVX {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_entry_nvx() {
  assert_size!(8, VkObjectTableEntryNVX);
}

/// Parameters of an object table pipeline entry
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTablePipelineEntryNVX<'h> {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pipeline: u64,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> VkObjectTablePipelineEntryNVX<'h> {
  #[inline]
  pub fn new() -> VkObjectTablePipelineEntryNVX<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_type(mut self, value: VkObjectEntryTypeNVX) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkObjectEntryUsageFlagsNVX) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_pipeline(mut self, value: VkPipeline<'h>) -> Self {
    unsafe {
      self.pipeline = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn get_type(&self) -> VkObjectEntryTypeNVX {
    self.eType
  }
  #[inline]
  pub fn flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> Default for VkObjectTablePipelineEntryNVX<'h> {
  fn default() -> VkObjectTablePipelineEntryNVX<'h> {
    VkObjectTablePipelineEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'h> Struct for VkObjectTablePipelineEntryNVX<'h> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_pipeline_entry_nvx() {
  assert_size!(16, VkObjectTablePipelineEntryNVX);
}

/// Parameters of an object table descriptor set entry
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableDescriptorSetEntryNVX<'h> {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pipelineLayout: u64,
  descriptorSet: u64,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> VkObjectTableDescriptorSetEntryNVX<'h> {
  #[inline]
  pub fn new() -> VkObjectTableDescriptorSetEntryNVX<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_type(mut self, value: VkObjectEntryTypeNVX) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkObjectEntryUsageFlagsNVX) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_pipeline_layout(mut self, value: VkPipelineLayout<'h>) -> Self {
    unsafe {
      self.pipelineLayout = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_descriptor_set(mut self, value: VkDescriptorSet<'h>) -> Self {
    unsafe {
      self.descriptorSet = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn get_type(&self) -> VkObjectEntryTypeNVX {
    self.eType
  }
  #[inline]
  pub fn flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> Default for VkObjectTableDescriptorSetEntryNVX<'h> {
  fn default() -> VkObjectTableDescriptorSetEntryNVX<'h> {
    VkObjectTableDescriptorSetEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'h> Struct for VkObjectTableDescriptorSetEntryNVX<'h> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_descriptor_set_entry_nvx() {
  assert_size!(24, VkObjectTableDescriptorSetEntryNVX);
}

/// Parameters of an object table vertex buffer entry
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableVertexBufferEntryNVX<'h> {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  buffer: u64,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> VkObjectTableVertexBufferEntryNVX<'h> {
  #[inline]
  pub fn new() -> VkObjectTableVertexBufferEntryNVX<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_type(mut self, value: VkObjectEntryTypeNVX) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkObjectEntryUsageFlagsNVX) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn get_type(&self) -> VkObjectEntryTypeNVX {
    self.eType
  }
  #[inline]
  pub fn flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> Default for VkObjectTableVertexBufferEntryNVX<'h> {
  fn default() -> VkObjectTableVertexBufferEntryNVX<'h> {
    VkObjectTableVertexBufferEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'h> Struct for VkObjectTableVertexBufferEntryNVX<'h> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_vertex_buffer_entry_nvx() {
  assert_size!(16, VkObjectTableVertexBufferEntryNVX);
}

/// Parameters of an object table index buffer entry
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableIndexBufferEntryNVX<'h> {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  buffer: u64,
  pub indexType: VkIndexType,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> VkObjectTableIndexBufferEntryNVX<'h> {
  #[inline]
  pub fn new() -> VkObjectTableIndexBufferEntryNVX<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_type(mut self, value: VkObjectEntryTypeNVX) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkObjectEntryUsageFlagsNVX) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_index_type(mut self, value: VkIndexType) -> Self {
    self.indexType = value;
    self
  }
  #[inline]
  pub fn get_type(&self) -> VkObjectEntryTypeNVX {
    self.eType
  }
  #[inline]
  pub fn flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn index_type(&self) -> VkIndexType {
    self.indexType
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> Default for VkObjectTableIndexBufferEntryNVX<'h> {
  fn default() -> VkObjectTableIndexBufferEntryNVX<'h> {
    VkObjectTableIndexBufferEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'h> Struct for VkObjectTableIndexBufferEntryNVX<'h> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_index_buffer_entry_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 1, VkObjectTableIndexBufferEntryNVX);
}

/// Parameters of an object table push constant entry
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTablePushConstantEntryNVX<'h> {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pipelineLayout: u64,
  pub stageFlags: VkShaderStageFlags,
  _p: ::std::marker::PhantomData<(&'h u8)>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> VkObjectTablePushConstantEntryNVX<'h> {
  #[inline]
  pub fn new() -> VkObjectTablePushConstantEntryNVX<'h> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_type(mut self, value: VkObjectEntryTypeNVX) -> Self {
    self.eType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkObjectEntryUsageFlagsNVX) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_pipeline_layout(mut self, value: VkPipelineLayout<'h>) -> Self {
    unsafe {
      self.pipelineLayout = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_stage_flags(mut self, value: VkShaderStageFlags) -> Self {
    self.stageFlags = value;
    self
  }
  #[inline]
  pub fn get_type(&self) -> VkObjectEntryTypeNVX {
    self.eType
  }
  #[inline]
  pub fn flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn stage_flags(&self) -> VkShaderStageFlags {
    self.stageFlags
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'h> Default for VkObjectTablePushConstantEntryNVX<'h> {
  fn default() -> VkObjectTablePushConstantEntryNVX<'h> {
    VkObjectTablePushConstantEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'h> Struct for VkObjectTablePushConstantEntryNVX<'h> {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_push_constant_entry_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 1, VkObjectTablePushConstantEntryNVX);
}

// feature: VK_NV_clip_space_w_scaling

/// Structure specifying a viewport
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub struct VkViewportWScalingNV {
  pub xcoeff: f32,
  pub ycoeff: f32,
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl VkViewportWScalingNV {
  #[inline]
  pub fn new() -> VkViewportWScalingNV {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_xcoeff(mut self, value: f32) -> Self {
    self.xcoeff = value;
    self
  }
  #[inline]
  pub fn set_ycoeff(mut self, value: f32) -> Self {
    self.ycoeff = value;
    self
  }
  #[inline]
  pub fn xcoeff(&self) -> f32 {
    self.xcoeff
  }
  #[inline]
  pub fn ycoeff(&self) -> f32 {
    self.ycoeff
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl Default for VkViewportWScalingNV {
  fn default() -> VkViewportWScalingNV {
    VkViewportWScalingNV::new()
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl Struct for VkViewportWScalingNV {}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_viewport_w_scaling_nv() {
  assert_size!(8, VkViewportWScalingNV);
}

/// Structure specifying parameters of a newly created pipeline viewport W scaling
/// state
#[repr(C)]
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub struct VkPipelineViewportWScalingStateCreateInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  viewportWScalingEnable: VkBool32,
  viewportCount: u32,
  pViewportWScalings: *const VkViewportWScalingNV,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl<'l> VkPipelineViewportWScalingStateCreateInfoNV<'l> {
  #[inline]
  pub fn new() -> VkPipelineViewportWScalingStateCreateInfoNV<'l> {
    unsafe {
      VkPipelineViewportWScalingStateCreateInfoNV {
        sType: VkStructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_viewport_w_scaling_enable(mut self, value: bool) -> Self {
    unsafe {
      self.viewportWScalingEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_viewport_w_scalings(mut self, value: &'l [VkViewportWScalingNV]) -> Self {
    self.viewportCount = value.len() as u32;
    unsafe {
      self.pViewportWScalings = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn is_viewport_w_scaling_enable(&self) -> bool {
    self.viewportWScalingEnable != 0
  }
  #[inline]
  pub fn viewport_count(&self) -> u32 {
    self.viewportCount
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl<'l> Default for VkPipelineViewportWScalingStateCreateInfoNV<'l> {
  fn default() -> VkPipelineViewportWScalingStateCreateInfoNV<'l> {
    VkPipelineViewportWScalingStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl<'l> Struct for VkPipelineViewportWScalingStateCreateInfoNV<'l> {}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineViewportStateCreateInfo<'m>>
  for VkPipelineViewportWScalingStateCreateInfoNV<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineViewportWScalingStateCreateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_viewport_w_scaling_state_create_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPipelineViewportWScalingStateCreateInfoNV);
}

// feature: VK_EXT_display_surface_counter

/// Surface-relative counter types
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub use enums::VkSurfaceCounterFlagBitsEXT;

/// Bitmask of VkSurfaceCounterFlagBitsEXT
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub type VkSurfaceCounterFlagsEXT = VkSurfaceCounterFlagBitsEXT;

/// Structure describing capabilities of a surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub struct VkSurfaceCapabilities2EXT {
  sType: VkStructureType,
  pNext: *mut c_void,
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
#[cfg(feature = "VK_EXT_display_surface_counter")]
impl VkSurfaceCapabilities2EXT {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn min_image_count(&self) -> u32 {
    self.minImageCount
  }
  #[inline]
  pub fn max_image_count(&self) -> u32 {
    self.maxImageCount
  }
  #[inline]
  pub fn current_extent(&self) -> &VkExtent2D {
    &self.currentExtent
  }
  #[inline]
  pub fn min_image_extent(&self) -> &VkExtent2D {
    &self.minImageExtent
  }
  #[inline]
  pub fn max_image_extent(&self) -> &VkExtent2D {
    &self.maxImageExtent
  }
  #[inline]
  pub fn max_image_array_layers(&self) -> u32 {
    self.maxImageArrayLayers
  }
  #[inline]
  pub fn supported_transforms(&self) -> VkSurfaceTransformFlagsKHR {
    self.supportedTransforms
  }
  #[inline]
  pub fn current_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
    self.currentTransform
  }
  #[inline]
  pub fn supported_composite_alpha(&self) -> VkCompositeAlphaFlagsKHR {
    self.supportedCompositeAlpha
  }
  #[inline]
  pub fn supported_usage_flags(&self) -> VkImageUsageFlags {
    self.supportedUsageFlags
  }
  #[inline]
  pub fn supported_surface_counters(&self) -> VkSurfaceCounterFlagsEXT {
    self.supportedSurfaceCounters
  }
}
#[cfg(feature = "VK_EXT_display_surface_counter")]
unsafe impl Struct for VkSurfaceCapabilities2EXT {}
#[cfg(feature = "VK_EXT_display_surface_counter")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_capabilities2_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(56 + ptr_size * 2, VkSurfaceCapabilities2EXT);
}

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

/// Describe the power state of a display
#[repr(C)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDisplayPowerInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub powerState: VkDisplayPowerStateEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'l> VkDisplayPowerInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkDisplayPowerInfoEXT<'l> {
    unsafe {
      VkDisplayPowerInfoEXT {
        sType: VkStructureType::DISPLAY_POWER_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_power_state(mut self, value: VkDisplayPowerStateEXT) -> Self {
    self.powerState = value;
    self
  }
  #[inline]
  pub fn power_state(&self) -> VkDisplayPowerStateEXT {
    self.powerState
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'l> Default for VkDisplayPowerInfoEXT<'l> {
  fn default() -> VkDisplayPowerInfoEXT<'l> {
    VkDisplayPowerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'l> Struct for VkDisplayPowerInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_power_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDisplayPowerInfoEXT);
}

/// Describe a device event to create
#[repr(C)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDeviceEventInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub deviceEvent: VkDeviceEventTypeEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'l> VkDeviceEventInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkDeviceEventInfoEXT<'l> {
    unsafe {
      VkDeviceEventInfoEXT {
        sType: VkStructureType::DEVICE_EVENT_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_event(mut self, value: VkDeviceEventTypeEXT) -> Self {
    self.deviceEvent = value;
    self
  }
  #[inline]
  pub fn device_event(&self) -> VkDeviceEventTypeEXT {
    self.deviceEvent
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'l> Default for VkDeviceEventInfoEXT<'l> {
  fn default() -> VkDeviceEventInfoEXT<'l> {
    VkDeviceEventInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'l> Struct for VkDeviceEventInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_event_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDeviceEventInfoEXT);
}

/// Describe a display event to create
#[repr(C)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDisplayEventInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub displayEvent: VkDisplayEventTypeEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'l> VkDisplayEventInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkDisplayEventInfoEXT<'l> {
    unsafe {
      VkDisplayEventInfoEXT {
        sType: VkStructureType::DISPLAY_EVENT_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_display_event(mut self, value: VkDisplayEventTypeEXT) -> Self {
    self.displayEvent = value;
    self
  }
  #[inline]
  pub fn display_event(&self) -> VkDisplayEventTypeEXT {
    self.displayEvent
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'l> Default for VkDisplayEventInfoEXT<'l> {
  fn default() -> VkDisplayEventInfoEXT<'l> {
    VkDisplayEventInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'l> Struct for VkDisplayEventInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_event_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDisplayEventInfoEXT);
}

/// Specify the surface counters desired
#[repr(C)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkSwapchainCounterCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub surfaceCounters: VkSurfaceCounterFlagsEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'l> VkSwapchainCounterCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkSwapchainCounterCreateInfoEXT<'l> {
    unsafe {
      VkSwapchainCounterCreateInfoEXT {
        sType: VkStructureType::SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_surface_counters(mut self, value: VkSurfaceCounterFlagsEXT) -> Self {
    self.surfaceCounters = value;
    self
  }
  #[inline]
  pub fn surface_counters(&self) -> VkSurfaceCounterFlagsEXT {
    self.surfaceCounters
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'l> Default for VkSwapchainCounterCreateInfoEXT<'l> {
  fn default() -> VkSwapchainCounterCreateInfoEXT<'l> {
    VkSwapchainCounterCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'l> Struct for VkSwapchainCounterCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkSwapchainCreateInfoKHR<'m, 'h>>
  for VkSwapchainCounterCreateInfoEXT<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkSwapchainCounterCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_swapchain_counter_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkSwapchainCounterCreateInfoEXT);
}

// feature: VK_GOOGLE_display_timing

/// Structure containing the RC duration of a display
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkRefreshCycleDurationGOOGLE {
  pub refreshDuration: u64,
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl VkRefreshCycleDurationGOOGLE {
  #[inline]
  pub fn new() -> VkRefreshCycleDurationGOOGLE {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_refresh_duration(mut self, value: u64) -> Self {
    self.refreshDuration = value;
    self
  }
  #[inline]
  pub fn refresh_duration(&self) -> u64 {
    self.refreshDuration
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl Default for VkRefreshCycleDurationGOOGLE {
  fn default() -> VkRefreshCycleDurationGOOGLE {
    VkRefreshCycleDurationGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Struct for VkRefreshCycleDurationGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_refresh_cycle_duration_google() {
  assert_size!(8, VkRefreshCycleDurationGOOGLE);
}

/// Structure containing timing information about a previously-presented image
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
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl VkPastPresentationTimingGOOGLE {
  #[inline]
  pub fn new() -> VkPastPresentationTimingGOOGLE {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_present_id(mut self, value: u32) -> Self {
    self.presentID = value;
    self
  }
  #[inline]
  pub fn set_desired_present_time(mut self, value: u64) -> Self {
    self.desiredPresentTime = value;
    self
  }
  #[inline]
  pub fn set_actual_present_time(mut self, value: u64) -> Self {
    self.actualPresentTime = value;
    self
  }
  #[inline]
  pub fn set_earliest_present_time(mut self, value: u64) -> Self {
    self.earliestPresentTime = value;
    self
  }
  #[inline]
  pub fn set_present_margin(mut self, value: u64) -> Self {
    self.presentMargin = value;
    self
  }
  #[inline]
  pub fn present_id(&self) -> u32 {
    self.presentID
  }
  #[inline]
  pub fn desired_present_time(&self) -> u64 {
    self.desiredPresentTime
  }
  #[inline]
  pub fn actual_present_time(&self) -> u64 {
    self.actualPresentTime
  }
  #[inline]
  pub fn earliest_present_time(&self) -> u64 {
    self.earliestPresentTime
  }
  #[inline]
  pub fn present_margin(&self) -> u64 {
    self.presentMargin
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl Default for VkPastPresentationTimingGOOGLE {
  fn default() -> VkPastPresentationTimingGOOGLE {
    VkPastPresentationTimingGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Struct for VkPastPresentationTimingGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_past_presentation_timing_google() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(32 + ptr_size * 1, VkPastPresentationTimingGOOGLE);
}

/// The earliest time image should be presented
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkPresentTimeGOOGLE {
  pub presentID: u32,
  pub desiredPresentTime: u64,
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl VkPresentTimeGOOGLE {
  #[inline]
  pub fn new() -> VkPresentTimeGOOGLE {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_present_id(mut self, value: u32) -> Self {
    self.presentID = value;
    self
  }
  #[inline]
  pub fn set_desired_present_time(mut self, value: u64) -> Self {
    self.desiredPresentTime = value;
    self
  }
  #[inline]
  pub fn present_id(&self) -> u32 {
    self.presentID
  }
  #[inline]
  pub fn desired_present_time(&self) -> u64 {
    self.desiredPresentTime
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl Default for VkPresentTimeGOOGLE {
  fn default() -> VkPresentTimeGOOGLE {
    VkPresentTimeGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Struct for VkPresentTimeGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_time_google() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 1, VkPresentTimeGOOGLE);
}

/// The earliest time each image should be presented
#[repr(C)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkPresentTimesInfoGOOGLE<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchainCount: u32,
  pTimes: *const VkPresentTimeGOOGLE,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl<'l> VkPresentTimesInfoGOOGLE<'l> {
  #[inline]
  pub fn new() -> VkPresentTimesInfoGOOGLE<'l> {
    unsafe {
      VkPresentTimesInfoGOOGLE {
        sType: VkStructureType::PRESENT_TIMES_INFO_GOOGLE,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_times(mut self, value: &'l [VkPresentTimeGOOGLE]) -> Self {
    self.swapchainCount = value.len() as u32;
    unsafe {
      self.pTimes = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn swapchain_count(&self) -> u32 {
    self.swapchainCount
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl<'l> Default for VkPresentTimesInfoGOOGLE<'l> {
  fn default() -> VkPresentTimesInfoGOOGLE<'l> {
    VkPresentTimesInfoGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl<'l> Struct for VkPresentTimesInfoGOOGLE<'l> {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkPresentInfoKHR<'m, 'h>> for VkPresentTimesInfoGOOGLE<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPresentTimesInfoGOOGLE as *const c_void
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_times_info_google() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkPresentTimesInfoGOOGLE);
}

// feature: VK_NVX_multiview_per_view_attributes

/// Structure describing multiview limits that can be supported by an implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  sType: VkStructureType,
  pNext: *mut c_void,
  perViewPositionAllComponents: VkBool32,
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
impl VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_per_view_position_all_components(&self) -> bool {
    self.perViewPositionAllComponents != 0
  }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
unsafe impl Struct for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX as *const c_void
  }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_multiview_per_view_attributes_properties_nvx() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(
    0 + ptr_size * 3,
    VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
  );
}

// feature: VK_NV_viewport_swizzle

/// Specify how a viewport coordinate is swizzled
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub use enums::VkViewportCoordinateSwizzleNV;

/// Structure specifying a viewport swizzle
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub struct VkViewportSwizzleNV {
  pub x: VkViewportCoordinateSwizzleNV,
  pub y: VkViewportCoordinateSwizzleNV,
  pub z: VkViewportCoordinateSwizzleNV,
  pub w: VkViewportCoordinateSwizzleNV,
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl VkViewportSwizzleNV {
  #[inline]
  pub fn new() -> VkViewportSwizzleNV {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_x(mut self, value: VkViewportCoordinateSwizzleNV) -> Self {
    self.x = value;
    self
  }
  #[inline]
  pub fn set_y(mut self, value: VkViewportCoordinateSwizzleNV) -> Self {
    self.y = value;
    self
  }
  #[inline]
  pub fn set_z(mut self, value: VkViewportCoordinateSwizzleNV) -> Self {
    self.z = value;
    self
  }
  #[inline]
  pub fn set_w(mut self, value: VkViewportCoordinateSwizzleNV) -> Self {
    self.w = value;
    self
  }
  #[inline]
  pub fn x(&self) -> VkViewportCoordinateSwizzleNV {
    self.x
  }
  #[inline]
  pub fn y(&self) -> VkViewportCoordinateSwizzleNV {
    self.y
  }
  #[inline]
  pub fn z(&self) -> VkViewportCoordinateSwizzleNV {
    self.z
  }
  #[inline]
  pub fn w(&self) -> VkViewportCoordinateSwizzleNV {
    self.w
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl Default for VkViewportSwizzleNV {
  fn default() -> VkViewportSwizzleNV {
    VkViewportSwizzleNV::new()
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl Struct for VkViewportSwizzleNV {}
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_viewport_swizzle_nv() {
  assert_size!(16, VkViewportSwizzleNV);
}

/// Reserved for future use
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;

/// Structure specifying swizzle applied to primitive clip coordinates
#[repr(C)]
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
  viewportCount: u32,
  pViewportSwizzles: *const VkViewportSwizzleNV,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl<'l> VkPipelineViewportSwizzleStateCreateInfoNV<'l> {
  #[inline]
  pub fn new() -> VkPipelineViewportSwizzleStateCreateInfoNV<'l> {
    unsafe {
      VkPipelineViewportSwizzleStateCreateInfoNV {
        sType: VkStructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineViewportSwizzleStateCreateFlagsNV) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_viewport_swizzles(mut self, value: &'l [VkViewportSwizzleNV]) -> Self {
    self.viewportCount = value.len() as u32;
    unsafe {
      self.pViewportSwizzles = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineViewportSwizzleStateCreateFlagsNV {
    self.flags
  }
  #[inline]
  pub fn viewport_count(&self) -> u32 {
    self.viewportCount
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl<'l> Default for VkPipelineViewportSwizzleStateCreateInfoNV<'l> {
  fn default() -> VkPipelineViewportSwizzleStateCreateInfoNV<'l> {
    VkPipelineViewportSwizzleStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl<'l> Struct for VkPipelineViewportSwizzleStateCreateInfoNV<'l> {}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineViewportStateCreateInfo<'m>>
  for VkPipelineViewportSwizzleStateCreateInfoNV<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineViewportSwizzleStateCreateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_viewport_swizzle_state_create_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPipelineViewportSwizzleStateCreateInfoNV);
}

// feature: VK_EXT_discard_rectangles

/// Structure describing discard rectangle limits that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub maxDiscardRectangles: u32,
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  #[inline]
  pub fn new() -> VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    unsafe {
      VkPhysicalDeviceDiscardRectanglePropertiesEXT {
        sType: VkStructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_max_discard_rectangles(mut self, value: u32) -> Self {
    self.maxDiscardRectangles = value;
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn max_discard_rectangles(&self) -> u32 {
    self.maxDiscardRectangles
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl Default for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  fn default() -> VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    VkPhysicalDeviceDiscardRectanglePropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
unsafe impl Struct for VkPhysicalDeviceDiscardRectanglePropertiesEXT {}
#[cfg(feature = "VK_EXT_discard_rectangles")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceDiscardRectanglePropertiesEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_discard_rectangle_properties_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPhysicalDeviceDiscardRectanglePropertiesEXT);
}

/// Reserved for future use
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkFlags;

/// Specify the discard rectangle mode
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub use enums::VkDiscardRectangleModeEXT;

/// Structure specifying discard rectangle
#[repr(C)]
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
  pub discardRectangleMode: VkDiscardRectangleModeEXT,
  discardRectangleCount: u32,
  pDiscardRectangles: *const VkRect2D,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl<'l> VkPipelineDiscardRectangleStateCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkPipelineDiscardRectangleStateCreateInfoEXT<'l> {
    unsafe {
      VkPipelineDiscardRectangleStateCreateInfoEXT {
        sType: VkStructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineDiscardRectangleStateCreateFlagsEXT) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_discard_rectangle_mode(mut self, value: VkDiscardRectangleModeEXT) -> Self {
    self.discardRectangleMode = value;
    self
  }
  #[inline]
  pub fn set_discard_rectangles(mut self, value: &'l [VkRect2D]) -> Self {
    self.discardRectangleCount = value.len() as u32;
    unsafe {
      self.pDiscardRectangles = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineDiscardRectangleStateCreateFlagsEXT {
    self.flags
  }
  #[inline]
  pub fn discard_rectangle_mode(&self) -> VkDiscardRectangleModeEXT {
    self.discardRectangleMode
  }
  #[inline]
  pub fn discard_rectangle_count(&self) -> u32 {
    self.discardRectangleCount
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl<'l> Default for VkPipelineDiscardRectangleStateCreateInfoEXT<'l> {
  fn default() -> VkPipelineDiscardRectangleStateCreateInfoEXT<'l> {
    VkPipelineDiscardRectangleStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
unsafe impl<'l> Struct for VkPipelineDiscardRectangleStateCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_discard_rectangles")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkGraphicsPipelineCreateInfo<'m, 'h>>
  for VkPipelineDiscardRectangleStateCreateInfoEXT<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineDiscardRectangleStateCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_discard_rectangle_state_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 4, VkPipelineDiscardRectangleStateCreateInfoEXT);
}

// feature: VK_EXT_conservative_rasterization

/// Structure describing conservative raster properties that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub primitiveOverestimationSize: f32,
  pub maxExtraPrimitiveOverestimationSize: f32,
  pub extraPrimitiveOverestimationSizeGranularity: f32,
  primitiveUnderestimation: VkBool32,
  conservativePointAndLineRasterization: VkBool32,
  degenerateTrianglesRasterized: VkBool32,
  degenerateLinesRasterized: VkBool32,
  fullyCoveredFragmentShaderInputVariable: VkBool32,
  conservativeRasterizationPostDepthCoverage: VkBool32,
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  #[inline]
  pub fn new() -> VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
    unsafe {
      VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
        sType: VkStructureType::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_primitive_overestimation_size(mut self, value: f32) -> Self {
    self.primitiveOverestimationSize = value;
    self
  }
  #[inline]
  pub fn set_max_extra_primitive_overestimation_size(mut self, value: f32) -> Self {
    self.maxExtraPrimitiveOverestimationSize = value;
    self
  }
  #[inline]
  pub fn set_extra_primitive_overestimation_size_granularity(mut self, value: f32) -> Self {
    self.extraPrimitiveOverestimationSizeGranularity = value;
    self
  }
  #[inline]
  pub fn set_primitive_underestimation(mut self, value: bool) -> Self {
    unsafe {
      self.primitiveUnderestimation = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_conservative_point_and_line_rasterization(mut self, value: bool) -> Self {
    unsafe {
      self.conservativePointAndLineRasterization = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_degenerate_triangles_rasterized(mut self, value: bool) -> Self {
    unsafe {
      self.degenerateTrianglesRasterized = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_degenerate_lines_rasterized(mut self, value: bool) -> Self {
    unsafe {
      self.degenerateLinesRasterized = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_fully_covered_fragment_shader_input_variable(mut self, value: bool) -> Self {
    unsafe {
      self.fullyCoveredFragmentShaderInputVariable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_conservative_rasterization_post_depth_coverage(mut self, value: bool) -> Self {
    unsafe {
      self.conservativeRasterizationPostDepthCoverage = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn primitive_overestimation_size(&self) -> f32 {
    self.primitiveOverestimationSize
  }
  #[inline]
  pub fn max_extra_primitive_overestimation_size(&self) -> f32 {
    self.maxExtraPrimitiveOverestimationSize
  }
  #[inline]
  pub fn extra_primitive_overestimation_size_granularity(&self) -> f32 {
    self.extraPrimitiveOverestimationSizeGranularity
  }
  #[inline]
  pub fn is_primitive_underestimation(&self) -> bool {
    self.primitiveUnderestimation != 0
  }
  #[inline]
  pub fn is_conservative_point_and_line_rasterization(&self) -> bool {
    self.conservativePointAndLineRasterization != 0
  }
  #[inline]
  pub fn is_degenerate_triangles_rasterized(&self) -> bool {
    self.degenerateTrianglesRasterized != 0
  }
  #[inline]
  pub fn is_degenerate_lines_rasterized(&self) -> bool {
    self.degenerateLinesRasterized != 0
  }
  #[inline]
  pub fn is_fully_covered_fragment_shader_input_variable(&self) -> bool {
    self.fullyCoveredFragmentShaderInputVariable != 0
  }
  #[inline]
  pub fn is_conservative_rasterization_post_depth_coverage(&self) -> bool {
    self.conservativeRasterizationPostDepthCoverage != 0
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl Default for VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  fn default() -> VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
    VkPhysicalDeviceConservativeRasterizationPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
unsafe impl Struct for VkPhysicalDeviceConservativeRasterizationPropertiesEXT {}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceConservativeRasterizationPropertiesEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_conservative_rasterization_properties_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(
    32 + ptr_size * 3,
    VkPhysicalDeviceConservativeRasterizationPropertiesEXT
  );
}

/// Reserved for future use
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub type VkPipelineRasterizationConservativeStateCreateFlagsEXT = VkFlags;

/// Specify the conservative rasterization mode
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub use enums::VkConservativeRasterizationModeEXT;

/// Structure specifying conservative raster state
#[repr(C)]
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
  pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
  pub extraPrimitiveOverestimationSize: f32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl<'l> VkPipelineRasterizationConservativeStateCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkPipelineRasterizationConservativeStateCreateInfoEXT<'l> {
    unsafe {
      VkPipelineRasterizationConservativeStateCreateInfoEXT {
        sType: VkStructureType::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineRasterizationConservativeStateCreateFlagsEXT) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_conservative_rasterization_mode(mut self, value: VkConservativeRasterizationModeEXT) -> Self {
    self.conservativeRasterizationMode = value;
    self
  }
  #[inline]
  pub fn set_extra_primitive_overestimation_size(mut self, value: f32) -> Self {
    self.extraPrimitiveOverestimationSize = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineRasterizationConservativeStateCreateFlagsEXT {
    self.flags
  }
  #[inline]
  pub fn conservative_rasterization_mode(&self) -> VkConservativeRasterizationModeEXT {
    self.conservativeRasterizationMode
  }
  #[inline]
  pub fn extra_primitive_overestimation_size(&self) -> f32 {
    self.extraPrimitiveOverestimationSize
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl<'l> Default for VkPipelineRasterizationConservativeStateCreateInfoEXT<'l> {
  fn default() -> VkPipelineRasterizationConservativeStateCreateInfoEXT<'l> {
    VkPipelineRasterizationConservativeStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
unsafe impl<'l> Struct for VkPipelineRasterizationConservativeStateCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineRasterizationStateCreateInfo<'m>>
  for VkPipelineRasterizationConservativeStateCreateInfoEXT<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineRasterizationConservativeStateCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_rasterization_conservative_state_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPipelineRasterizationConservativeStateCreateInfoEXT);
}

// feature: VK_EXT_hdr_metadata

/// structure to specify X,Y chromaticity coordinates
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub struct VkXYColorEXT {
  pub x: f32,
  pub y: f32,
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl VkXYColorEXT {
  #[inline]
  pub fn new() -> VkXYColorEXT {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_x(mut self, value: f32) -> Self {
    self.x = value;
    self
  }
  #[inline]
  pub fn set_y(mut self, value: f32) -> Self {
    self.y = value;
    self
  }
  #[inline]
  pub fn x(&self) -> f32 {
    self.x
  }
  #[inline]
  pub fn y(&self) -> f32 {
    self.y
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl Default for VkXYColorEXT {
  fn default() -> VkXYColorEXT {
    VkXYColorEXT::new()
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
unsafe impl Struct for VkXYColorEXT {}
#[cfg(feature = "VK_EXT_hdr_metadata")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_xy_color_ext() {
  assert_size!(8, VkXYColorEXT);
}

/// structure to specify Hdr metadata
#[repr(C)]
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub struct VkHdrMetadataEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub displayPrimaryRed: VkXYColorEXT,
  pub displayPrimaryGreen: VkXYColorEXT,
  pub displayPrimaryBlue: VkXYColorEXT,
  pub whitePoint: VkXYColorEXT,
  pub maxLuminance: f32,
  pub minLuminance: f32,
  pub maxContentLightLevel: f32,
  pub maxFrameAverageLightLevel: f32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl<'l> VkHdrMetadataEXT<'l> {
  #[inline]
  pub fn new() -> VkHdrMetadataEXT<'l> {
    unsafe {
      VkHdrMetadataEXT {
        sType: VkStructureType::HDR_METADATA_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_display_primary_red(mut self, value: VkXYColorEXT) -> Self {
    self.displayPrimaryRed = value;
    self
  }
  #[inline]
  pub fn set_display_primary_green(mut self, value: VkXYColorEXT) -> Self {
    self.displayPrimaryGreen = value;
    self
  }
  #[inline]
  pub fn set_display_primary_blue(mut self, value: VkXYColorEXT) -> Self {
    self.displayPrimaryBlue = value;
    self
  }
  #[inline]
  pub fn set_white_point(mut self, value: VkXYColorEXT) -> Self {
    self.whitePoint = value;
    self
  }
  #[inline]
  pub fn set_max_luminance(mut self, value: f32) -> Self {
    self.maxLuminance = value;
    self
  }
  #[inline]
  pub fn set_min_luminance(mut self, value: f32) -> Self {
    self.minLuminance = value;
    self
  }
  #[inline]
  pub fn set_max_content_light_level(mut self, value: f32) -> Self {
    self.maxContentLightLevel = value;
    self
  }
  #[inline]
  pub fn set_max_frame_average_light_level(mut self, value: f32) -> Self {
    self.maxFrameAverageLightLevel = value;
    self
  }
  #[inline]
  pub fn display_primary_red(&self) -> &VkXYColorEXT {
    &self.displayPrimaryRed
  }
  #[inline]
  pub fn display_primary_green(&self) -> &VkXYColorEXT {
    &self.displayPrimaryGreen
  }
  #[inline]
  pub fn display_primary_blue(&self) -> &VkXYColorEXT {
    &self.displayPrimaryBlue
  }
  #[inline]
  pub fn white_point(&self) -> &VkXYColorEXT {
    &self.whitePoint
  }
  #[inline]
  pub fn max_luminance(&self) -> f32 {
    self.maxLuminance
  }
  #[inline]
  pub fn min_luminance(&self) -> f32 {
    self.minLuminance
  }
  #[inline]
  pub fn max_content_light_level(&self) -> f32 {
    self.maxContentLightLevel
  }
  #[inline]
  pub fn max_frame_average_light_level(&self) -> f32 {
    self.maxFrameAverageLightLevel
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl<'l> Default for VkHdrMetadataEXT<'l> {
  fn default() -> VkHdrMetadataEXT<'l> {
    VkHdrMetadataEXT::new()
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
unsafe impl<'l> Struct for VkHdrMetadataEXT<'l> {}
#[cfg(feature = "VK_EXT_hdr_metadata")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_hdr_metadata_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 2, VkHdrMetadataEXT);
}

// feature: VK_KHR_get_surface_capabilities2

/// Structure specifying a surface and related swapchain creation parameters
#[repr(C)]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub struct VkPhysicalDeviceSurfaceInfo2KHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  surface: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl<'l, 'h: 'l> VkPhysicalDeviceSurfaceInfo2KHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceSurfaceInfo2KHR<'l, 'h> {
    unsafe {
      VkPhysicalDeviceSurfaceInfo2KHR {
        sType: VkStructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_surface(mut self, value: VkSurfaceKHR<'h>) -> Self {
    unsafe {
      self.surface = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl<'l, 'h: 'l> Default for VkPhysicalDeviceSurfaceInfo2KHR<'l, 'h> {
  fn default() -> VkPhysicalDeviceSurfaceInfo2KHR<'l, 'h> {
    VkPhysicalDeviceSurfaceInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
unsafe impl<'l, 'h: 'l> Struct for VkPhysicalDeviceSurfaceInfo2KHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_surface_info2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkPhysicalDeviceSurfaceInfo2KHR);
}

/// Structure describing capabilities of a surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub struct VkSurfaceCapabilities2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl VkSurfaceCapabilities2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn surface_capabilities(&self) -> &VkSurfaceCapabilitiesKHR {
    &self.surfaceCapabilities
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
unsafe impl Struct for VkSurfaceCapabilities2KHR {}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_capabilities2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 3, VkSurfaceCapabilities2KHR);
}

/// Structure describing a supported swapchain format tuple
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub struct VkSurfaceFormat2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub surfaceFormat: VkSurfaceFormatKHR,
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl VkSurfaceFormat2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn surface_format(&self) -> &VkSurfaceFormatKHR {
    &self.surfaceFormat
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
unsafe impl Struct for VkSurfaceFormat2KHR {}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_format2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkSurfaceFormat2KHR);
}

// feature: VK_KHR_shared_presentable_image

/// structure describing capabilities of a surface for shared presentation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub sharedPresentSupportedUsageFlags: VkImageUsageFlags,
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
impl VkSharedPresentSurfaceCapabilitiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn shared_present_supported_usage_flags(&self) -> VkImageUsageFlags {
    self.sharedPresentSupportedUsageFlags
  }
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
unsafe impl Struct for VkSharedPresentSurfaceCapabilitiesKHR {}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
unsafe impl StructExtends<VkSurfaceCapabilities2KHR> for VkSharedPresentSurfaceCapabilitiesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkSharedPresentSurfaceCapabilitiesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_shared_present_surface_capabilities_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkSharedPresentSurfaceCapabilitiesKHR);
}

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

/// Structure specifying fence creation parameters.
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub struct VkPhysicalDeviceExternalFenceInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl<'l> VkPhysicalDeviceExternalFenceInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalFenceInfoKHR<'l> {
    unsafe {
      VkPhysicalDeviceExternalFenceInfoKHR {
        sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalFenceHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl<'l> Default for VkPhysicalDeviceExternalFenceInfoKHR<'l> {
  fn default() -> VkPhysicalDeviceExternalFenceInfoKHR<'l> {
    VkPhysicalDeviceExternalFenceInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
unsafe impl<'l> Struct for VkPhysicalDeviceExternalFenceInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_fence_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPhysicalDeviceExternalFenceInfoKHR);
}

/// Structure describing supported external fence handle features
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub struct VkExternalFencePropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
  pub compatibleHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
  pub externalFenceFeatures: VkExternalFenceFeatureFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl VkExternalFencePropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn export_from_imported_handle_types(&self) -> VkExternalFenceHandleTypeFlagsKHR {
    self.exportFromImportedHandleTypes
  }
  #[inline]
  pub fn compatible_handle_types(&self) -> VkExternalFenceHandleTypeFlagsKHR {
    self.compatibleHandleTypes
  }
  #[inline]
  pub fn external_fence_features(&self) -> VkExternalFenceFeatureFlagsKHR {
    self.externalFenceFeatures
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
unsafe impl Struct for VkExternalFencePropertiesKHR {}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_fence_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkExternalFencePropertiesKHR);
}

// feature: VK_KHR_external_fence

/// Bitmask specifying additional parameters of fence payload import
#[cfg(feature = "VK_KHR_external_fence")]
pub use enums::VkFenceImportFlagBitsKHR;

/// Bitmask of VkFenceImportFlagBitsKHR
#[cfg(feature = "VK_KHR_external_fence")]
pub type VkFenceImportFlagsKHR = VkFenceImportFlagBitsKHR;

/// Structure specifying handle types that can be exported from a fence
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence")]
pub struct VkExportFenceCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalFenceHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_fence")]
impl<'l> VkExportFenceCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkExportFenceCreateInfoKHR<'l> {
    unsafe {
      VkExportFenceCreateInfoKHR {
        sType: VkStructureType::EXPORT_FENCE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalFenceHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
  #[inline]
  pub fn handle_types(&self) -> VkExternalFenceHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_fence")]
impl<'l> Default for VkExportFenceCreateInfoKHR<'l> {
  fn default() -> VkExportFenceCreateInfoKHR<'l> {
    VkExportFenceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence")]
unsafe impl<'l> Struct for VkExportFenceCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_fence")]
unsafe impl<'m, 'l: 'm> StructExtends<VkFenceCreateInfo<'m>> for VkExportFenceCreateInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExportFenceCreateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_fence")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_export_fence_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkExportFenceCreateInfoKHR);
}

// feature: VK_KHR_external_fence_win32

/// (None)
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportFenceWin32HandleInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  fence: u64,
  pub flags: VkFenceImportFlagsKHR,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> VkImportFenceWin32HandleInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkImportFenceWin32HandleInfoKHR<'l, 'h> {
    unsafe {
      VkImportFenceWin32HandleInfoKHR {
        sType: VkStructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_fence(mut self, value: VkFence<'h>) -> Self {
    unsafe {
      self.fence = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkFenceImportFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalFenceHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn set_handle(mut self, value: wsi::win32::HANDLE) -> Self {
    self.handle = value;
    self
  }
  #[inline]
  pub fn set_name(mut self, value: wsi::win32::LPCWSTR) -> Self {
    self.name = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkFenceImportFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn handle(&self) -> wsi::win32::HANDLE {
    self.handle
  }
  #[inline]
  pub fn name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> Default for VkImportFenceWin32HandleInfoKHR<'l, 'h> {
  fn default() -> VkImportFenceWin32HandleInfoKHR<'l, 'h> {
    VkImportFenceWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l, 'h: 'l> Struct for VkImportFenceWin32HandleInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_fence_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 4, VkImportFenceWin32HandleInfoKHR);
}

/// Structure specifying additional attributes of Windows handles exported from a
/// fence
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportFenceWin32HandleInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> VkExportFenceWin32HandleInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkExportFenceWin32HandleInfoKHR<'l> {
    unsafe {
      VkExportFenceWin32HandleInfoKHR {
        sType: VkStructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_attributes(mut self, value: *const wsi::win32::SECURITY_ATTRIBUTES) -> Self {
    self.pAttributes = value;
    self
  }
  #[inline]
  pub fn set_dw_access(mut self, value: wsi::win32::DWORD) -> Self {
    self.dwAccess = value;
    self
  }
  #[inline]
  pub fn set_name(mut self, value: wsi::win32::LPCWSTR) -> Self {
    self.name = value;
    self
  }
  #[inline]
  pub fn attributes(&self) -> *const wsi::win32::SECURITY_ATTRIBUTES {
    self.pAttributes
  }
  #[inline]
  pub fn dw_access(&self) -> wsi::win32::DWORD {
    self.dwAccess
  }
  #[inline]
  pub fn name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l> Default for VkExportFenceWin32HandleInfoKHR<'l> {
  fn default() -> VkExportFenceWin32HandleInfoKHR<'l> {
    VkExportFenceWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l> Struct for VkExportFenceWin32HandleInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'m, 'l: 'm> StructExtends<VkFenceCreateInfo<'m>> for VkExportFenceWin32HandleInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkExportFenceWin32HandleInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_export_fence_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkExportFenceWin32HandleInfoKHR);
}

/// Structure describing a Win32 handle fence export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkFenceGetWin32HandleInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  fence: u64,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> VkFenceGetWin32HandleInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkFenceGetWin32HandleInfoKHR<'l, 'h> {
    unsafe {
      VkFenceGetWin32HandleInfoKHR {
        sType: VkStructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_fence(mut self, value: VkFence<'h>) -> Self {
    unsafe {
      self.fence = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalFenceHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'l, 'h: 'l> Default for VkFenceGetWin32HandleInfoKHR<'l, 'h> {
  fn default() -> VkFenceGetWin32HandleInfoKHR<'l, 'h> {
    VkFenceGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'l, 'h: 'l> Struct for VkFenceGetWin32HandleInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_get_win32_handle_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkFenceGetWin32HandleInfoKHR);
}

// feature: VK_KHR_external_fence_fd

/// (None)
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub struct VkImportFenceFdInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  fence: u64,
  pub flags: VkFenceImportFlagsKHR,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  pub fd: c_int,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<'l, 'h: 'l> VkImportFenceFdInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkImportFenceFdInfoKHR<'l, 'h> {
    unsafe {
      VkImportFenceFdInfoKHR {
        sType: VkStructureType::IMPORT_FENCE_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_fence(mut self, value: VkFence<'h>) -> Self {
    unsafe {
      self.fence = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkFenceImportFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalFenceHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn set_fd(mut self, value: c_int) -> Self {
    self.fd = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkFenceImportFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn fd(&self) -> c_int {
    self.fd
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<'l, 'h: 'l> Default for VkImportFenceFdInfoKHR<'l, 'h> {
  fn default() -> VkImportFenceFdInfoKHR<'l, 'h> {
    VkImportFenceFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
unsafe impl<'l, 'h: 'l> Struct for VkImportFenceFdInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_fence_fd_info_khr() {
  let int_size = ::std::mem::size_of::<::std::os::raw::c_int>();
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + int_size * 1 + ptr_size * 2, VkImportFenceFdInfoKHR);
}

/// Structure describing a POSIX FD fence export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub struct VkFenceGetFdInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  fence: u64,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<'l, 'h: 'l> VkFenceGetFdInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkFenceGetFdInfoKHR<'l, 'h> {
    unsafe {
      VkFenceGetFdInfoKHR {
        sType: VkStructureType::FENCE_GET_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_fence(mut self, value: VkFence<'h>) -> Self {
    unsafe {
      self.fence = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalFenceHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<'l, 'h: 'l> Default for VkFenceGetFdInfoKHR<'l, 'h> {
  fn default() -> VkFenceGetFdInfoKHR<'l, 'h> {
    VkFenceGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
unsafe impl<'l, 'h: 'l> Struct for VkFenceGetFdInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_get_fd_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkFenceGetFdInfoKHR);
}

// feature: VK_KHR_maintenance2

/// Enum specifying the point clipping behaviour
#[cfg(feature = "VK_KHR_maintenance2")]
pub use enums::VkPointClippingBehaviorKHR;

/// Structure describing the point clipping behavior supported by an implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkPhysicalDevicePointClippingPropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub pointClippingBehavior: VkPointClippingBehaviorKHR,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl VkPhysicalDevicePointClippingPropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn point_clipping_behavior(&self) -> VkPointClippingBehaviorKHR {
    self.pointClippingBehavior
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl Struct for VkPhysicalDevicePointClippingPropertiesKHR {}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDevicePointClippingPropertiesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDevicePointClippingPropertiesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_point_clipping_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPhysicalDevicePointClippingPropertiesKHR);
}

/// Structure specifying a subpass/input attachment pair and an aspect mask that
/// can: be read.
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkInputAttachmentAspectReferenceKHR {
  pub subpass: u32,
  pub inputAttachmentIndex: u32,
  pub aspectMask: VkImageAspectFlags,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl VkInputAttachmentAspectReferenceKHR {
  #[inline]
  pub fn new() -> VkInputAttachmentAspectReferenceKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_subpass(mut self, value: u32) -> Self {
    self.subpass = value;
    self
  }
  #[inline]
  pub fn set_input_attachment_index(mut self, value: u32) -> Self {
    self.inputAttachmentIndex = value;
    self
  }
  #[inline]
  pub fn set_aspect_mask(mut self, value: VkImageAspectFlags) -> Self {
    self.aspectMask = value;
    self
  }
  #[inline]
  pub fn subpass(&self) -> u32 {
    self.subpass
  }
  #[inline]
  pub fn input_attachment_index(&self) -> u32 {
    self.inputAttachmentIndex
  }
  #[inline]
  pub fn aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl Default for VkInputAttachmentAspectReferenceKHR {
  fn default() -> VkInputAttachmentAspectReferenceKHR {
    VkInputAttachmentAspectReferenceKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl Struct for VkInputAttachmentAspectReferenceKHR {}
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_input_attachment_aspect_reference_khr() {
  assert_size!(12, VkInputAttachmentAspectReferenceKHR);
}

/// Structure specifying, for a given subpass/input attachment pair, which aspect
/// can: be read.
#[repr(C)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  aspectReferenceCount: u32,
  pAspectReferences: *const VkInputAttachmentAspectReferenceKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'l> VkRenderPassInputAttachmentAspectCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkRenderPassInputAttachmentAspectCreateInfoKHR<'l> {
    unsafe {
      VkRenderPassInputAttachmentAspectCreateInfoKHR {
        sType: VkStructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_aspect_references(mut self, value: &'l [VkInputAttachmentAspectReferenceKHR]) -> Self {
    self.aspectReferenceCount = value.len() as u32;
    unsafe {
      self.pAspectReferences = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn aspect_reference_count(&self) -> u32 {
    self.aspectReferenceCount
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'l> Default for VkRenderPassInputAttachmentAspectCreateInfoKHR<'l> {
  fn default() -> VkRenderPassInputAttachmentAspectCreateInfoKHR<'l> {
    VkRenderPassInputAttachmentAspectCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'l> Struct for VkRenderPassInputAttachmentAspectCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'m, 'l: 'm> StructExtends<VkRenderPassCreateInfo<'m>>
  for VkRenderPassInputAttachmentAspectCreateInfoKHR<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkRenderPassInputAttachmentAspectCreateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_input_attachment_aspect_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkRenderPassInputAttachmentAspectCreateInfoKHR);
}

/// Specify the intended usage of an image view
#[repr(C)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkImageViewUsageCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub usage: VkImageUsageFlags,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'l> VkImageViewUsageCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkImageViewUsageCreateInfoKHR<'l> {
    unsafe {
      VkImageViewUsageCreateInfoKHR {
        sType: VkStructureType::IMAGE_VIEW_USAGE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_usage(mut self, value: VkImageUsageFlags) -> Self {
    self.usage = value;
    self
  }
  #[inline]
  pub fn usage(&self) -> VkImageUsageFlags {
    self.usage
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'l> Default for VkImageViewUsageCreateInfoKHR<'l> {
  fn default() -> VkImageViewUsageCreateInfoKHR<'l> {
    VkImageViewUsageCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'l> Struct for VkImageViewUsageCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkImageViewCreateInfo<'m, 'h>> for VkImageViewUsageCreateInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkImageViewUsageCreateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_view_usage_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkImageViewUsageCreateInfoKHR);
}

/// Enum describing tessellation domain origin
#[cfg(feature = "VK_KHR_maintenance2")]
pub use enums::VkTessellationDomainOriginKHR;

/// Structure specifying the orientation of the tessellation domain
#[repr(C)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub domainOrigin: VkTessellationDomainOriginKHR,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'l> VkPipelineTessellationDomainOriginStateCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkPipelineTessellationDomainOriginStateCreateInfoKHR<'l> {
    unsafe {
      VkPipelineTessellationDomainOriginStateCreateInfoKHR {
        sType: VkStructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_domain_origin(mut self, value: VkTessellationDomainOriginKHR) -> Self {
    self.domainOrigin = value;
    self
  }
  #[inline]
  pub fn domain_origin(&self) -> VkTessellationDomainOriginKHR {
    self.domainOrigin
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'l> Default for VkPipelineTessellationDomainOriginStateCreateInfoKHR<'l> {
  fn default() -> VkPipelineTessellationDomainOriginStateCreateInfoKHR<'l> {
    VkPipelineTessellationDomainOriginStateCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'l> Struct for VkPipelineTessellationDomainOriginStateCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineTessellationStateCreateInfo<'m>>
  for VkPipelineTessellationDomainOriginStateCreateInfoKHR<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineTessellationDomainOriginStateCreateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_tessellation_domain_origin_state_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPipelineTessellationDomainOriginStateCreateInfoKHR);
}

// feature: VK_KHR_variable_pointers

/// Structure describing variable pointers features that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_variable_pointers")]
pub struct VkPhysicalDeviceVariablePointerFeaturesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  variablePointersStorageBuffer: VkBool32,
  variablePointers: VkBool32,
}
#[cfg(feature = "VK_KHR_variable_pointers")]
impl VkPhysicalDeviceVariablePointerFeaturesKHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceVariablePointerFeaturesKHR {
    unsafe {
      VkPhysicalDeviceVariablePointerFeaturesKHR {
        sType: VkStructureType::PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_variable_pointers_storage_buffer(mut self, value: bool) -> Self {
    unsafe {
      self.variablePointersStorageBuffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_variable_pointers(mut self, value: bool) -> Self {
    unsafe {
      self.variablePointers = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_variable_pointers_storage_buffer(&self) -> bool {
    self.variablePointersStorageBuffer != 0
  }
  #[inline]
  pub fn is_variable_pointers(&self) -> bool {
    self.variablePointers != 0
  }
}
#[cfg(feature = "VK_KHR_variable_pointers")]
impl Default for VkPhysicalDeviceVariablePointerFeaturesKHR {
  fn default() -> VkPhysicalDeviceVariablePointerFeaturesKHR {
    VkPhysicalDeviceVariablePointerFeaturesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_variable_pointers")]
unsafe impl Struct for VkPhysicalDeviceVariablePointerFeaturesKHR {}
#[cfg(feature = "VK_KHR_variable_pointers")]
unsafe impl StructExtends<VkPhysicalDeviceFeatures2KHR> for VkPhysicalDeviceVariablePointerFeaturesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceVariablePointerFeaturesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_variable_pointers")]
unsafe impl<'m> StructExtends<VkDeviceCreateInfo<'m>> for VkPhysicalDeviceVariablePointerFeaturesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceVariablePointerFeaturesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_variable_pointers")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_variable_pointer_features_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkPhysicalDeviceVariablePointerFeaturesKHR);
}

// feature: VK_MVK_ios_surface
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub type VkIOSSurfaceCreateFlagsMVK = VkFlags;

/// Structure specifying parameters of a newly created iOS surface object
#[repr(C)]
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub struct VkIOSSurfaceCreateInfoMVK<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkIOSSurfaceCreateFlagsMVK,
  pView: *const c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
impl<'l> VkIOSSurfaceCreateInfoMVK<'l> {
  #[inline]
  pub fn new() -> VkIOSSurfaceCreateInfoMVK<'l> {
    unsafe {
      VkIOSSurfaceCreateInfoMVK {
        sType: VkStructureType::IOS_SURFACE_CREATE_INFO_MVK,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkIOSSurfaceCreateFlagsMVK) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_view(mut self, value: *const c_void) -> Self {
    self.pView = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkIOSSurfaceCreateFlagsMVK {
    self.flags
  }
  #[inline]
  pub fn view(&self) -> *const c_void {
    self.pView
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
impl<'l> Default for VkIOSSurfaceCreateInfoMVK<'l> {
  fn default() -> VkIOSSurfaceCreateInfoMVK<'l> {
    VkIOSSurfaceCreateInfoMVK::new()
  }
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
unsafe impl<'l> Struct for VkIOSSurfaceCreateInfoMVK<'l> {}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_ios_surface_create_info_mvk() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkIOSSurfaceCreateInfoMVK);
}

// feature: VK_MVK_macos_surface
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub type VkMacOSSurfaceCreateFlagsMVK = VkFlags;

/// Structure specifying parameters of a newly created macOS surface object
#[repr(C)]
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub struct VkMacOSSurfaceCreateInfoMVK<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkMacOSSurfaceCreateFlagsMVK,
  pView: *const c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
impl<'l> VkMacOSSurfaceCreateInfoMVK<'l> {
  #[inline]
  pub fn new() -> VkMacOSSurfaceCreateInfoMVK<'l> {
    unsafe {
      VkMacOSSurfaceCreateInfoMVK {
        sType: VkStructureType::MACOS_SURFACE_CREATE_INFO_MVK,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkMacOSSurfaceCreateFlagsMVK) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_view(mut self, value: *const c_void) -> Self {
    self.pView = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkMacOSSurfaceCreateFlagsMVK {
    self.flags
  }
  #[inline]
  pub fn view(&self) -> *const c_void {
    self.pView
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
impl<'l> Default for VkMacOSSurfaceCreateInfoMVK<'l> {
  fn default() -> VkMacOSSurfaceCreateInfoMVK<'l> {
    VkMacOSSurfaceCreateInfoMVK::new()
  }
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
unsafe impl<'l> Struct for VkMacOSSurfaceCreateInfoMVK<'l> {}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_mac_os_surface_create_info_mvk() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkMacOSSurfaceCreateInfoMVK);
}

// feature: VK_KHR_get_memory_requirements2

/// (None)
#[repr(C)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkBufferMemoryRequirementsInfo2KHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  buffer: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'l, 'h: 'l> VkBufferMemoryRequirementsInfo2KHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkBufferMemoryRequirementsInfo2KHR<'l, 'h> {
    unsafe {
      VkBufferMemoryRequirementsInfo2KHR {
        sType: VkStructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'l, 'h: 'l> Default for VkBufferMemoryRequirementsInfo2KHR<'l, 'h> {
  fn default() -> VkBufferMemoryRequirementsInfo2KHR<'l, 'h> {
    VkBufferMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl<'l, 'h: 'l> Struct for VkBufferMemoryRequirementsInfo2KHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_memory_requirements_info2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkBufferMemoryRequirementsInfo2KHR);
}

/// (None)
#[repr(C)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkImageMemoryRequirementsInfo2KHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  image: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'l, 'h: 'l> VkImageMemoryRequirementsInfo2KHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkImageMemoryRequirementsInfo2KHR<'l, 'h> {
    unsafe {
      VkImageMemoryRequirementsInfo2KHR {
        sType: VkStructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage<'h>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'l, 'h: 'l> Default for VkImageMemoryRequirementsInfo2KHR<'l, 'h> {
  fn default() -> VkImageMemoryRequirementsInfo2KHR<'l, 'h> {
    VkImageMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl<'l, 'h: 'l> Struct for VkImageMemoryRequirementsInfo2KHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_memory_requirements_info2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkImageMemoryRequirementsInfo2KHR);
}
#[repr(C)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkImageSparseMemoryRequirementsInfo2KHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  image: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'l, 'h: 'l> VkImageSparseMemoryRequirementsInfo2KHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkImageSparseMemoryRequirementsInfo2KHR<'l, 'h> {
    unsafe {
      VkImageSparseMemoryRequirementsInfo2KHR {
        sType: VkStructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage<'h>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'l, 'h: 'l> Default for VkImageSparseMemoryRequirementsInfo2KHR<'l, 'h> {
  fn default() -> VkImageSparseMemoryRequirementsInfo2KHR<'l, 'h> {
    VkImageSparseMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl<'l, 'h: 'l> Struct for VkImageSparseMemoryRequirementsInfo2KHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_sparse_memory_requirements_info2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkImageSparseMemoryRequirementsInfo2KHR);
}

/// Structure specifying memory requirements
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkMemoryRequirements2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub memoryRequirements: VkMemoryRequirements,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl VkMemoryRequirements2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn memory_requirements(&self) -> &VkMemoryRequirements {
    &self.memoryRequirements
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl Struct for VkMemoryRequirements2KHR {}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_requirements2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkMemoryRequirements2KHR);
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkSparseImageMemoryRequirements2KHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub memoryRequirements: VkSparseImageMemoryRequirements,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl VkSparseImageMemoryRequirements2KHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn memory_requirements(&self) -> &VkSparseImageMemoryRequirements {
    &self.memoryRequirements
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl Struct for VkSparseImageMemoryRequirements2KHR {}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_requirements2_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(48 + ptr_size * 2, VkSparseImageMemoryRequirements2KHR);
}

// feature: VK_KHR_dedicated_allocation

/// Structure describing dedicated allocation requirements of buffer and image
/// resources
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub struct VkMemoryDedicatedRequirementsKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  prefersDedicatedAllocation: VkBool32,
  requiresDedicatedAllocation: VkBool32,
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl VkMemoryDedicatedRequirementsKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_prefers_dedicated_allocation(&self) -> bool {
    self.prefersDedicatedAllocation != 0
  }
  #[inline]
  pub fn is_requires_dedicated_allocation(&self) -> bool {
    self.requiresDedicatedAllocation != 0
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl Struct for VkMemoryDedicatedRequirementsKHR {}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl StructExtends<VkMemoryRequirements2KHR> for VkMemoryDedicatedRequirementsKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkMemoryDedicatedRequirementsKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_dedicated_requirements_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkMemoryDedicatedRequirementsKHR);
}

/// Specify a dedicated memory allocation resource
#[repr(C)]
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub struct VkMemoryDedicatedAllocateInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  image: u64,
  buffer: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl<'l, 'h: 'l> VkMemoryDedicatedAllocateInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkMemoryDedicatedAllocateInfoKHR<'l, 'h> {
    unsafe {
      VkMemoryDedicatedAllocateInfoKHR {
        sType: VkStructureType::MEMORY_DEDICATED_ALLOCATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: Option<VkImage<'h>>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: Option<VkBuffer<'h>>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl<'l, 'h: 'l> Default for VkMemoryDedicatedAllocateInfoKHR<'l, 'h> {
  fn default() -> VkMemoryDedicatedAllocateInfoKHR<'l, 'h> {
    VkMemoryDedicatedAllocateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl<'l, 'h: 'l> Struct for VkMemoryDedicatedAllocateInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkMemoryAllocateInfo<'m>> for VkMemoryDedicatedAllocateInfoKHR<'l, 'h> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkMemoryDedicatedAllocateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_dedicated_allocate_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 2, VkMemoryDedicatedAllocateInfoKHR);
}

// feature: VK_EXT_sampler_filter_minmax

/// Specify reduction mode for texture filtering
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub use enums::VkSamplerReductionModeEXT;

/// Structure specifying sampler reduction mode
#[repr(C)]
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub struct VkSamplerReductionModeCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub reductionMode: VkSamplerReductionModeEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl<'l> VkSamplerReductionModeCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkSamplerReductionModeCreateInfoEXT<'l> {
    unsafe {
      VkSamplerReductionModeCreateInfoEXT {
        sType: VkStructureType::SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_reduction_mode(mut self, value: VkSamplerReductionModeEXT) -> Self {
    self.reductionMode = value;
    self
  }
  #[inline]
  pub fn reduction_mode(&self) -> VkSamplerReductionModeEXT {
    self.reductionMode
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl<'l> Default for VkSamplerReductionModeCreateInfoEXT<'l> {
  fn default() -> VkSamplerReductionModeCreateInfoEXT<'l> {
    VkSamplerReductionModeCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
unsafe impl<'l> Struct for VkSamplerReductionModeCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
unsafe impl<'m, 'l: 'm> StructExtends<VkSamplerCreateInfo<'m>> for VkSamplerReductionModeCreateInfoEXT<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkSamplerReductionModeCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sampler_reduction_mode_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkSamplerReductionModeCreateInfoEXT);
}

/// Structure describing sampler filter minmax limits that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  filterMinmaxSingleComponentFormats: VkBool32,
  filterMinmaxImageComponentMapping: VkBool32,
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_filter_minmax_single_component_formats(&self) -> bool {
    self.filterMinmaxSingleComponentFormats != 0
  }
  #[inline]
  pub fn is_filter_minmax_image_component_mapping(&self) -> bool {
    self.filterMinmaxImageComponentMapping != 0
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
unsafe impl Struct for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_sampler_filter_minmax_properties_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT);
}

// feature: VK_EXT_sample_locations

/// Structure specifying the coordinates of a sample location
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSampleLocationEXT {
  pub x: f32,
  pub y: f32,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl VkSampleLocationEXT {
  #[inline]
  pub fn new() -> VkSampleLocationEXT {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_x(mut self, value: f32) -> Self {
    self.x = value;
    self
  }
  #[inline]
  pub fn set_y(mut self, value: f32) -> Self {
    self.y = value;
    self
  }
  #[inline]
  pub fn x(&self) -> f32 {
    self.x
  }
  #[inline]
  pub fn y(&self) -> f32 {
    self.y
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl Default for VkSampleLocationEXT {
  fn default() -> VkSampleLocationEXT {
    VkSampleLocationEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl Struct for VkSampleLocationEXT {}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sample_location_ext() {
  assert_size!(8, VkSampleLocationEXT);
}

/// Structure specifying a set of sample locations
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSampleLocationsInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub sampleLocationsPerPixel: VkSampleCountFlagBits,
  pub sampleLocationGridSize: VkExtent2D,
  sampleLocationsCount: u32,
  pSampleLocations: *const VkSampleLocationEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> VkSampleLocationsInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkSampleLocationsInfoEXT<'l> {
    unsafe {
      VkSampleLocationsInfoEXT {
        sType: VkStructureType::SAMPLE_LOCATIONS_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_sample_locations_per_pixel(mut self, value: VkSampleCountFlagBits) -> Self {
    self.sampleLocationsPerPixel = value;
    self
  }
  #[inline]
  pub fn set_sample_location_grid_size(mut self, value: VkExtent2D) -> Self {
    self.sampleLocationGridSize = value;
    self
  }
  #[inline]
  pub fn set_sample_locations(mut self, value: &'l [VkSampleLocationEXT]) -> Self {
    self.sampleLocationsCount = value.len() as u32;
    unsafe {
      self.pSampleLocations = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn sample_locations_per_pixel(&self) -> VkSampleCountFlagBits {
    self.sampleLocationsPerPixel
  }
  #[inline]
  pub fn sample_location_grid_size(&self) -> &VkExtent2D {
    &self.sampleLocationGridSize
  }
  #[inline]
  pub fn sample_locations_count(&self) -> u32 {
    self.sampleLocationsCount
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> Default for VkSampleLocationsInfoEXT<'l> {
  fn default() -> VkSampleLocationsInfoEXT<'l> {
    VkSampleLocationsInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'l> Struct for VkSampleLocationsInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkImageMemoryBarrier<'m, 'h>> for VkSampleLocationsInfoEXT<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkSampleLocationsInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sample_locations_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkSampleLocationsInfoEXT);
}

/// Structure specifying the sample locations state to use in the initial layout
/// transition of attachments
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkAttachmentSampleLocationsEXT<'l> {
  pub attachmentIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT<'l>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> VkAttachmentSampleLocationsEXT<'l> {
  #[inline]
  pub fn new() -> VkAttachmentSampleLocationsEXT<'l> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_attachment_index(mut self, value: u32) -> Self {
    self.attachmentIndex = value;
    self
  }
  #[inline]
  pub fn set_sample_locations_info(mut self, value: VkSampleLocationsInfoEXT<'l>) -> Self {
    self.sampleLocationsInfo = value;
    self
  }
  #[inline]
  pub fn attachment_index(&self) -> u32 {
    self.attachmentIndex
  }
  #[inline]
  pub fn sample_locations_info(&self) -> &VkSampleLocationsInfoEXT<'l> {
    &self.sampleLocationsInfo
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> Default for VkAttachmentSampleLocationsEXT<'l> {
  fn default() -> VkAttachmentSampleLocationsEXT<'l> {
    VkAttachmentSampleLocationsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'l> Struct for VkAttachmentSampleLocationsEXT<'l> {}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_attachment_sample_locations_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 4, VkAttachmentSampleLocationsEXT);
}

/// Structure specifying the sample locations state to use for layout transitions of
/// attachments performed after a given subpass
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSubpassSampleLocationsEXT<'l> {
  pub subpassIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT<'l>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> VkSubpassSampleLocationsEXT<'l> {
  #[inline]
  pub fn new() -> VkSubpassSampleLocationsEXT<'l> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_subpass_index(mut self, value: u32) -> Self {
    self.subpassIndex = value;
    self
  }
  #[inline]
  pub fn set_sample_locations_info(mut self, value: VkSampleLocationsInfoEXT<'l>) -> Self {
    self.sampleLocationsInfo = value;
    self
  }
  #[inline]
  pub fn subpass_index(&self) -> u32 {
    self.subpassIndex
  }
  #[inline]
  pub fn sample_locations_info(&self) -> &VkSampleLocationsInfoEXT<'l> {
    &self.sampleLocationsInfo
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> Default for VkSubpassSampleLocationsEXT<'l> {
  fn default() -> VkSubpassSampleLocationsEXT<'l> {
    VkSubpassSampleLocationsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'l> Struct for VkSubpassSampleLocationsEXT<'l> {}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_subpass_sample_locations_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 4, VkSubpassSampleLocationsEXT);
}

/// Structure specifying sample locations to use for the layout transition of custom
/// sample locations compatible depth/stencil attachments
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkRenderPassSampleLocationsBeginInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  attachmentInitialSampleLocationsCount: u32,
  pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT<'l>,
  postSubpassSampleLocationsCount: u32,
  pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT<'l>,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> VkRenderPassSampleLocationsBeginInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkRenderPassSampleLocationsBeginInfoEXT<'l> {
    unsafe {
      VkRenderPassSampleLocationsBeginInfoEXT {
        sType: VkStructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_attachment_initial_sample_locations(mut self, value: &'l [VkAttachmentSampleLocationsEXT<'l>]) -> Self {
    self.attachmentInitialSampleLocationsCount = value.len() as u32;
    unsafe {
      self.pAttachmentInitialSampleLocations = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_post_subpass_sample_locations(mut self, value: &'l [VkSubpassSampleLocationsEXT<'l>]) -> Self {
    self.postSubpassSampleLocationsCount = value.len() as u32;
    unsafe {
      self.pPostSubpassSampleLocations = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn attachment_initial_sample_locations_count(&self) -> u32 {
    self.attachmentInitialSampleLocationsCount
  }
  #[inline]
  pub fn post_subpass_sample_locations_count(&self) -> u32 {
    self.postSubpassSampleLocationsCount
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> Default for VkRenderPassSampleLocationsBeginInfoEXT<'l> {
  fn default() -> VkRenderPassSampleLocationsBeginInfoEXT<'l> {
    VkRenderPassSampleLocationsBeginInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'l> Struct for VkRenderPassSampleLocationsBeginInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkRenderPassBeginInfo<'m, 'h>>
  for VkRenderPassSampleLocationsBeginInfoEXT<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkRenderPassSampleLocationsBeginInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_sample_locations_begin_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 6, VkRenderPassSampleLocationsBeginInfoEXT);
}

/// Structure specifying sample locations for a pipeline
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  sampleLocationsEnable: VkBool32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT<'l>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> VkPipelineSampleLocationsStateCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkPipelineSampleLocationsStateCreateInfoEXT<'l> {
    unsafe {
      VkPipelineSampleLocationsStateCreateInfoEXT {
        sType: VkStructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_sample_locations_enable(mut self, value: bool) -> Self {
    unsafe {
      self.sampleLocationsEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sample_locations_info(mut self, value: VkSampleLocationsInfoEXT<'l>) -> Self {
    self.sampleLocationsInfo = value;
    self
  }
  #[inline]
  pub fn is_sample_locations_enable(&self) -> bool {
    self.sampleLocationsEnable != 0
  }
  #[inline]
  pub fn sample_locations_info(&self) -> &VkSampleLocationsInfoEXT<'l> {
    &self.sampleLocationsInfo
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'l> Default for VkPipelineSampleLocationsStateCreateInfoEXT<'l> {
  fn default() -> VkPipelineSampleLocationsStateCreateInfoEXT<'l> {
    VkPipelineSampleLocationsStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'l> Struct for VkPipelineSampleLocationsStateCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineMultisampleStateCreateInfo<'m>>
  for VkPipelineSampleLocationsStateCreateInfoEXT<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineSampleLocationsStateCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_sample_locations_state_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 6, VkPipelineSampleLocationsStateCreateInfoEXT);
}

/// Structure describing sample location limits that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub sampleLocationSampleCounts: VkSampleCountFlags,
  pub maxSampleLocationGridSize: VkExtent2D,
  pub sampleLocationCoordinateRange: [f32; 2],
  pub sampleLocationSubPixelBits: u32,
  variableSampleLocations: VkBool32,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl VkPhysicalDeviceSampleLocationsPropertiesEXT {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn sample_location_sample_counts(&self) -> VkSampleCountFlags {
    self.sampleLocationSampleCounts
  }
  #[inline]
  pub fn max_sample_location_grid_size(&self) -> &VkExtent2D {
    &self.maxSampleLocationGridSize
  }
  #[inline]
  pub fn sample_location_coordinate_range(&self) -> [f32; 2] {
    self.sampleLocationCoordinateRange
  }
  #[inline]
  pub fn sample_location_sub_pixel_bits(&self) -> u32 {
    self.sampleLocationSubPixelBits
  }
  #[inline]
  pub fn is_variable_sample_locations(&self) -> bool {
    self.variableSampleLocations != 0
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl Struct for VkPhysicalDeviceSampleLocationsPropertiesEXT {}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceSampleLocationsPropertiesEXT {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceSampleLocationsPropertiesEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_sample_locations_properties_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 3, VkPhysicalDeviceSampleLocationsPropertiesEXT);
}

/// Structure returning information about sample count specific additional
/// multisampling capabilities
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkMultisamplePropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub maxSampleLocationGridSize: VkExtent2D,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl VkMultisamplePropertiesEXT {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn max_sample_location_grid_size(&self) -> &VkExtent2D {
    &self.maxSampleLocationGridSize
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl Struct for VkMultisamplePropertiesEXT {}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_multisample_properties_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkMultisamplePropertiesEXT);
}

// feature: VK_KHR_image_format_list

/// Specify that an image can: be used with a particular set of formats
#[repr(C)]
#[cfg(feature = "VK_KHR_image_format_list")]
pub struct VkImageFormatListCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  viewFormatCount: u32,
  pViewFormats: *const VkFormat,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_image_format_list")]
impl<'l> VkImageFormatListCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkImageFormatListCreateInfoKHR<'l> {
    unsafe {
      VkImageFormatListCreateInfoKHR {
        sType: VkStructureType::IMAGE_FORMAT_LIST_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_view_formats(mut self, value: &'l [VkFormat]) -> Self {
    self.viewFormatCount = value.len() as u32;
    unsafe {
      self.pViewFormats = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn view_format_count(&self) -> u32 {
    self.viewFormatCount
  }
}
#[cfg(feature = "VK_KHR_image_format_list")]
impl<'l> Default for VkImageFormatListCreateInfoKHR<'l> {
  fn default() -> VkImageFormatListCreateInfoKHR<'l> {
    VkImageFormatListCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_image_format_list")]
unsafe impl<'l> Struct for VkImageFormatListCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_image_format_list")]
unsafe impl<'m, 'l: 'm> StructExtends<VkImageCreateInfo<'m>> for VkImageFormatListCreateInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkImageFormatListCreateInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_image_format_list")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_format_list_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkImageFormatListCreateInfoKHR);
}

// feature: VK_EXT_blend_operation_advanced

/// Structure describing advanced blending features that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  advancedBlendCoherentOperations: VkBool32,
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  #[inline]
  pub fn new() -> VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    unsafe {
      VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
        sType: VkStructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_advanced_blend_coherent_operations(mut self, value: bool) -> Self {
    unsafe {
      self.advancedBlendCoherentOperations = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_advanced_blend_coherent_operations(&self) -> bool {
    self.advancedBlendCoherentOperations != 0
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl Default for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  fn default() -> VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl Struct for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl StructExtends<VkPhysicalDeviceFeatures2KHR> for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_blend_operation_advanced_features_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT);
}

/// Structure describing advanced blending limits that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub advancedBlendMaxColorAttachments: u32,
  advancedBlendIndependentBlend: VkBool32,
  advancedBlendNonPremultipliedSrcColor: VkBool32,
  advancedBlendNonPremultipliedDstColor: VkBool32,
  advancedBlendCorrelatedOverlap: VkBool32,
  advancedBlendAllOperations: VkBool32,
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn advanced_blend_max_color_attachments(&self) -> u32 {
    self.advancedBlendMaxColorAttachments
  }
  #[inline]
  pub fn is_advanced_blend_independent_blend(&self) -> bool {
    self.advancedBlendIndependentBlend != 0
  }
  #[inline]
  pub fn is_advanced_blend_non_premultiplied_src_color(&self) -> bool {
    self.advancedBlendNonPremultipliedSrcColor != 0
  }
  #[inline]
  pub fn is_advanced_blend_non_premultiplied_dst_color(&self) -> bool {
    self.advancedBlendNonPremultipliedDstColor != 0
  }
  #[inline]
  pub fn is_advanced_blend_correlated_overlap(&self) -> bool {
    self.advancedBlendCorrelatedOverlap != 0
  }
  #[inline]
  pub fn is_advanced_blend_all_operations(&self) -> bool {
    self.advancedBlendAllOperations != 0
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl Struct for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_blend_operation_advanced_properties_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 2, VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT);
}

/// Enumerant specifying the blend overlap parameter
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub use enums::VkBlendOverlapEXT;

/// Structure specifying parameters that affect advanced blend operations
#[repr(C)]
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  srcPremultiplied: VkBool32,
  dstPremultiplied: VkBool32,
  pub blendOverlap: VkBlendOverlapEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl<'l> VkPipelineColorBlendAdvancedStateCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkPipelineColorBlendAdvancedStateCreateInfoEXT<'l> {
    unsafe {
      VkPipelineColorBlendAdvancedStateCreateInfoEXT {
        sType: VkStructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_premultiplied(mut self, value: bool) -> Self {
    unsafe {
      self.srcPremultiplied = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_dst_premultiplied(mut self, value: bool) -> Self {
    unsafe {
      self.dstPremultiplied = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_blend_overlap(mut self, value: VkBlendOverlapEXT) -> Self {
    self.blendOverlap = value;
    self
  }
  #[inline]
  pub fn is_src_premultiplied(&self) -> bool {
    self.srcPremultiplied != 0
  }
  #[inline]
  pub fn is_dst_premultiplied(&self) -> bool {
    self.dstPremultiplied != 0
  }
  #[inline]
  pub fn blend_overlap(&self) -> VkBlendOverlapEXT {
    self.blendOverlap
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl<'l> Default for VkPipelineColorBlendAdvancedStateCreateInfoEXT<'l> {
  fn default() -> VkPipelineColorBlendAdvancedStateCreateInfoEXT<'l> {
    VkPipelineColorBlendAdvancedStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl<'l> Struct for VkPipelineColorBlendAdvancedStateCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineColorBlendStateCreateInfo<'m>>
  for VkPipelineColorBlendAdvancedStateCreateInfoEXT<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineColorBlendAdvancedStateCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_color_blend_advanced_state_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPipelineColorBlendAdvancedStateCreateInfoEXT);
}

// feature: VK_NV_fragment_coverage_to_color

/// Reserved for future use
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub type VkPipelineCoverageToColorStateCreateFlagsNV = VkFlags;

/// Structure specifying whether fragment coverage replaces a color
#[repr(C)]
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub struct VkPipelineCoverageToColorStateCreateInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
  coverageToColorEnable: VkBool32,
  pub coverageToColorLocation: u32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
impl<'l> VkPipelineCoverageToColorStateCreateInfoNV<'l> {
  #[inline]
  pub fn new() -> VkPipelineCoverageToColorStateCreateInfoNV<'l> {
    unsafe {
      VkPipelineCoverageToColorStateCreateInfoNV {
        sType: VkStructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineCoverageToColorStateCreateFlagsNV) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_coverage_to_color_enable(mut self, value: bool) -> Self {
    unsafe {
      self.coverageToColorEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_coverage_to_color_location(mut self, value: u32) -> Self {
    self.coverageToColorLocation = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineCoverageToColorStateCreateFlagsNV {
    self.flags
  }
  #[inline]
  pub fn is_coverage_to_color_enable(&self) -> bool {
    self.coverageToColorEnable != 0
  }
  #[inline]
  pub fn coverage_to_color_location(&self) -> u32 {
    self.coverageToColorLocation
  }
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
impl<'l> Default for VkPipelineCoverageToColorStateCreateInfoNV<'l> {
  fn default() -> VkPipelineCoverageToColorStateCreateInfoNV<'l> {
    VkPipelineCoverageToColorStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
unsafe impl<'l> Struct for VkPipelineCoverageToColorStateCreateInfoNV<'l> {}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineMultisampleStateCreateInfo<'m>>
  for VkPipelineCoverageToColorStateCreateInfoNV<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineCoverageToColorStateCreateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_coverage_to_color_state_create_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 3, VkPipelineCoverageToColorStateCreateInfoNV);
}

// feature: VK_NV_framebuffer_mixed_samples

/// Reserved for future use
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub type VkPipelineCoverageModulationStateCreateFlagsNV = VkFlags;

/// Specify the discard rectangle mode
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub use enums::VkCoverageModulationModeNV;

/// Structure specifying parameters controlling coverage modulation
#[repr(C)]
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub struct VkPipelineCoverageModulationStateCreateInfoNV<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
  pub coverageModulationMode: VkCoverageModulationModeNV,
  coverageModulationTableEnable: VkBool32,
  coverageModulationTableCount: u32,
  pCoverageModulationTable: *const f32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
impl<'l> VkPipelineCoverageModulationStateCreateInfoNV<'l> {
  #[inline]
  pub fn new() -> VkPipelineCoverageModulationStateCreateInfoNV<'l> {
    unsafe {
      VkPipelineCoverageModulationStateCreateInfoNV {
        sType: VkStructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineCoverageModulationStateCreateFlagsNV) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_coverage_modulation_mode(mut self, value: VkCoverageModulationModeNV) -> Self {
    self.coverageModulationMode = value;
    self
  }
  #[inline]
  pub fn set_coverage_modulation_table_enable(mut self, value: bool) -> Self {
    unsafe {
      self.coverageModulationTableEnable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_coverage_modulation_table(mut self, value: &'l [f32]) -> Self {
    self.coverageModulationTableCount = value.len() as u32;
    unsafe {
      self.pCoverageModulationTable = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkPipelineCoverageModulationStateCreateFlagsNV {
    self.flags
  }
  #[inline]
  pub fn coverage_modulation_mode(&self) -> VkCoverageModulationModeNV {
    self.coverageModulationMode
  }
  #[inline]
  pub fn is_coverage_modulation_table_enable(&self) -> bool {
    self.coverageModulationTableEnable != 0
  }
  #[inline]
  pub fn coverage_modulation_table_count(&self) -> u32 {
    self.coverageModulationTableCount
  }
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
impl<'l> Default for VkPipelineCoverageModulationStateCreateInfoNV<'l> {
  fn default() -> VkPipelineCoverageModulationStateCreateInfoNV<'l> {
    VkPipelineCoverageModulationStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
unsafe impl<'l> Struct for VkPipelineCoverageModulationStateCreateInfoNV<'l> {}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
unsafe impl<'m, 'l: 'm> StructExtends<VkPipelineMultisampleStateCreateInfo<'m>>
  for VkPipelineCoverageModulationStateCreateInfoNV<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPipelineCoverageModulationStateCreateInfoNV as *const c_void
  }
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_coverage_modulation_state_create_info_nv() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(16 + ptr_size * 3, VkPipelineCoverageModulationStateCreateInfoNV);
}

// feature: VK_KHR_bind_memory2

/// Structure specifying how to bind a buffer to memory
#[repr(C)]
#[cfg(feature = "VK_KHR_bind_memory2")]
pub struct VkBindBufferMemoryInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  buffer: u64,
  memory: u64,
  pub memoryOffset: VkDeviceSize,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<'l, 'h: 'l> VkBindBufferMemoryInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkBindBufferMemoryInfoKHR<'l, 'h> {
    unsafe {
      VkBindBufferMemoryInfoKHR {
        sType: VkStructureType::BIND_BUFFER_MEMORY_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer<'h>) -> Self {
    unsafe {
      self.buffer = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory<'h>) -> Self {
    unsafe {
      self.memory = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_memory_offset(mut self, value: VkDeviceSize) -> Self {
    self.memoryOffset = value;
    self
  }
  #[inline]
  pub fn memory_offset(&self) -> VkDeviceSize {
    self.memoryOffset
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<'l, 'h: 'l> Default for VkBindBufferMemoryInfoKHR<'l, 'h> {
  fn default() -> VkBindBufferMemoryInfoKHR<'l, 'h> {
    VkBindBufferMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
unsafe impl<'l, 'h: 'l> Struct for VkBindBufferMemoryInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_bind_memory2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_buffer_memory_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 2, VkBindBufferMemoryInfoKHR);
}

/// Structure specifying how to bind an image to memory
#[repr(C)]
#[cfg(feature = "VK_KHR_bind_memory2")]
pub struct VkBindImageMemoryInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  image: u64,
  memory: u64,
  pub memoryOffset: VkDeviceSize,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<'l, 'h: 'l> VkBindImageMemoryInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkBindImageMemoryInfoKHR<'l, 'h> {
    unsafe {
      VkBindImageMemoryInfoKHR {
        sType: VkStructureType::BIND_IMAGE_MEMORY_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage<'h>) -> Self {
    unsafe {
      self.image = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory<'h>) -> Self {
    unsafe {
      self.memory = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_memory_offset(mut self, value: VkDeviceSize) -> Self {
    self.memoryOffset = value;
    self
  }
  #[inline]
  pub fn memory_offset(&self) -> VkDeviceSize {
    self.memoryOffset
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<'l, 'h: 'l> Default for VkBindImageMemoryInfoKHR<'l, 'h> {
  fn default() -> VkBindImageMemoryInfoKHR<'l, 'h> {
    VkBindImageMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
unsafe impl<'l, 'h: 'l> Struct for VkBindImageMemoryInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_bind_memory2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_image_memory_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(24 + ptr_size * 2, VkBindImageMemoryInfoKHR);
}

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

/// Structure specifying the parameters of the newly created conversion
#[repr(C)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionCreateInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub format: VkFormat,
  pub ycbcrModel: VkSamplerYcbcrModelConversionKHR,
  pub ycbcrRange: VkSamplerYcbcrRangeKHR,
  pub components: VkComponentMapping,
  pub xChromaOffset: VkChromaLocationKHR,
  pub yChromaOffset: VkChromaLocationKHR,
  pub chromaFilter: VkFilter,
  forceExplicitReconstruction: VkBool32,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'l> VkSamplerYcbcrConversionCreateInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkSamplerYcbcrConversionCreateInfoKHR<'l> {
    unsafe {
      VkSamplerYcbcrConversionCreateInfoKHR {
        sType: VkStructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_ycbcr_model(mut self, value: VkSamplerYcbcrModelConversionKHR) -> Self {
    self.ycbcrModel = value;
    self
  }
  #[inline]
  pub fn set_ycbcr_range(mut self, value: VkSamplerYcbcrRangeKHR) -> Self {
    self.ycbcrRange = value;
    self
  }
  #[inline]
  pub fn set_components(mut self, value: VkComponentMapping) -> Self {
    self.components = value;
    self
  }
  #[inline]
  pub fn set_x_chroma_offset(mut self, value: VkChromaLocationKHR) -> Self {
    self.xChromaOffset = value;
    self
  }
  #[inline]
  pub fn set_y_chroma_offset(mut self, value: VkChromaLocationKHR) -> Self {
    self.yChromaOffset = value;
    self
  }
  #[inline]
  pub fn set_chroma_filter(mut self, value: VkFilter) -> Self {
    self.chromaFilter = value;
    self
  }
  #[inline]
  pub fn set_force_explicit_reconstruction(mut self, value: bool) -> Self {
    unsafe {
      self.forceExplicitReconstruction = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn ycbcr_model(&self) -> VkSamplerYcbcrModelConversionKHR {
    self.ycbcrModel
  }
  #[inline]
  pub fn ycbcr_range(&self) -> VkSamplerYcbcrRangeKHR {
    self.ycbcrRange
  }
  #[inline]
  pub fn components(&self) -> &VkComponentMapping {
    &self.components
  }
  #[inline]
  pub fn x_chroma_offset(&self) -> VkChromaLocationKHR {
    self.xChromaOffset
  }
  #[inline]
  pub fn y_chroma_offset(&self) -> VkChromaLocationKHR {
    self.yChromaOffset
  }
  #[inline]
  pub fn chroma_filter(&self) -> VkFilter {
    self.chromaFilter
  }
  #[inline]
  pub fn is_force_explicit_reconstruction(&self) -> bool {
    self.forceExplicitReconstruction != 0
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'l> Default for VkSamplerYcbcrConversionCreateInfoKHR<'l> {
  fn default() -> VkSamplerYcbcrConversionCreateInfoKHR<'l> {
    VkSamplerYcbcrConversionCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'l> Struct for VkSamplerYcbcrConversionCreateInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sampler_ycbcr_conversion_create_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(40 + ptr_size * 3, VkSamplerYcbcrConversionCreateInfoKHR);
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSamplerYcbcrConversionKHR__ {}

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type VkSamplerYcbcrConversionKHR<'l> = VkNonDispatchableHandle<'l, VkSamplerYcbcrConversionKHR__>;

/// Structure specifying Y\'CbCr conversion to a sampler or image view
#[repr(C)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionInfoKHR<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  conversion: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'l, 'h: 'l> VkSamplerYcbcrConversionInfoKHR<'l, 'h> {
  #[inline]
  pub fn new() -> VkSamplerYcbcrConversionInfoKHR<'l, 'h> {
    unsafe {
      VkSamplerYcbcrConversionInfoKHR {
        sType: VkStructureType::SAMPLER_YCBCR_CONVERSION_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_conversion(mut self, value: VkSamplerYcbcrConversionKHR<'h>) -> Self {
    unsafe {
      self.conversion = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'l, 'h: 'l> Default for VkSamplerYcbcrConversionInfoKHR<'l, 'h> {
  fn default() -> VkSamplerYcbcrConversionInfoKHR<'l, 'h> {
    VkSamplerYcbcrConversionInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'l, 'h: 'l> Struct for VkSamplerYcbcrConversionInfoKHR<'l, 'h> {}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkSamplerCreateInfo<'m>> for VkSamplerYcbcrConversionInfoKHR<'l, 'h> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkSamplerYcbcrConversionInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkImageViewCreateInfo<'m, 'h>>
  for VkSamplerYcbcrConversionInfoKHR<'l, 'h>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkSamplerYcbcrConversionInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sampler_ycbcr_conversion_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkSamplerYcbcrConversionInfoKHR);
}

/// Structure specifying how to bind an image plane to memory
#[repr(C)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkBindImagePlaneMemoryInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub planeAspect: VkImageAspectFlagBits,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'l> VkBindImagePlaneMemoryInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkBindImagePlaneMemoryInfoKHR<'l> {
    unsafe {
      VkBindImagePlaneMemoryInfoKHR {
        sType: VkStructureType::BIND_IMAGE_PLANE_MEMORY_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_plane_aspect(mut self, value: VkImageAspectFlagBits) -> Self {
    self.planeAspect = value;
    self
  }
  #[inline]
  pub fn plane_aspect(&self) -> VkImageAspectFlagBits {
    self.planeAspect
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'l> Default for VkBindImagePlaneMemoryInfoKHR<'l> {
  fn default() -> VkBindImagePlaneMemoryInfoKHR<'l> {
    VkBindImagePlaneMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'l> Struct for VkBindImagePlaneMemoryInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkBindImageMemoryInfoKHR<'m, 'h>> for VkBindImagePlaneMemoryInfoKHR<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkBindImagePlaneMemoryInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_image_plane_memory_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkBindImagePlaneMemoryInfoKHR);
}

/// Structure specifying image plane for memory requirements
#[repr(C)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkImagePlaneMemoryRequirementsInfoKHR<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub planeAspect: VkImageAspectFlagBits,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'l> VkImagePlaneMemoryRequirementsInfoKHR<'l> {
  #[inline]
  pub fn new() -> VkImagePlaneMemoryRequirementsInfoKHR<'l> {
    unsafe {
      VkImagePlaneMemoryRequirementsInfoKHR {
        sType: VkStructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_plane_aspect(mut self, value: VkImageAspectFlagBits) -> Self {
    self.planeAspect = value;
    self
  }
  #[inline]
  pub fn plane_aspect(&self) -> VkImageAspectFlagBits {
    self.planeAspect
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'l> Default for VkImagePlaneMemoryRequirementsInfoKHR<'l> {
  fn default() -> VkImagePlaneMemoryRequirementsInfoKHR<'l> {
    VkImagePlaneMemoryRequirementsInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'l> Struct for VkImagePlaneMemoryRequirementsInfoKHR<'l> {}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkImageMemoryRequirementsInfo2KHR<'m, 'h>>
  for VkImagePlaneMemoryRequirementsInfoKHR<'l>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkImagePlaneMemoryRequirementsInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_plane_memory_requirements_info_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkImagePlaneMemoryRequirementsInfoKHR);
}

/// Structure describing Y\'CbCr conversion features that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  samplerYcbcrConversion: VkBool32,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
    unsafe {
      VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
        sType: VkStructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_sampler_ycbcr_conversion(mut self, value: bool) -> Self {
    unsafe {
      self.samplerYcbcrConversion = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn is_sampler_ycbcr_conversion(&self) -> bool {
    self.samplerYcbcrConversion != 0
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  fn default() -> VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
    VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl Struct for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl StructExtends<VkPhysicalDeviceFeatures2KHR> for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'m> StructExtends<VkDeviceCreateInfo<'m>> for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_sampler_ycbcr_conversion_features_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR);
}

/// Structure specifying combined image sampler descriptor count for multi-planar
/// images
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub combinedImageSamplerDescriptorCount: u32,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl VkSamplerYcbcrConversionImageFormatPropertiesKHR {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn combined_image_sampler_descriptor_count(&self) -> u32 {
    self.combinedImageSamplerDescriptorCount
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl Struct for VkSamplerYcbcrConversionImageFormatPropertiesKHR {}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl StructExtends<VkImageFormatProperties2KHR> for VkSamplerYcbcrConversionImageFormatPropertiesKHR {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkSamplerYcbcrConversionImageFormatPropertiesKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sampler_ycbcr_conversion_image_format_properties_khr() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkSamplerYcbcrConversionImageFormatPropertiesKHR);
}

// feature: VK_EXT_validation_cache
#[cfg(feature = "VK_EXT_validation_cache")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkValidationCacheEXT__ {}

/// Opaque handle to a validation cache object
#[cfg(feature = "VK_EXT_validation_cache")]
pub type VkValidationCacheEXT<'l> = VkNonDispatchableHandle<'l, VkValidationCacheEXT__>;

/// Reserved for future use
#[cfg(feature = "VK_EXT_validation_cache")]
pub type VkValidationCacheCreateFlagsEXT = VkFlags;

/// Structure specifying parameters of a newly created validation cache
#[repr(C)]
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct VkValidationCacheCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkValidationCacheCreateFlagsEXT,
  initialDataSize: usize,
  pInitialData: *const c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'l> VkValidationCacheCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkValidationCacheCreateInfoEXT<'l> {
    unsafe {
      VkValidationCacheCreateInfoEXT {
        sType: VkStructureType::VALIDATION_CACHE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_flags(mut self, value: VkValidationCacheCreateFlagsEXT) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_initial_data(mut self, value: &'l [u8]) -> Self {
    self.initialDataSize = value.len() as usize;
    unsafe {
      self.pInitialData = value.as_raw() as *const c_void;
    }
    self
  }
  #[inline]
  pub fn flags(&self) -> VkValidationCacheCreateFlagsEXT {
    self.flags
  }
  #[inline]
  pub fn initial_data_size(&self) -> usize {
    self.initialDataSize
  }
  #[inline]
  pub fn extend<E>(self, e: &E) -> Self
  where
    E: StructExtends<Self> + Sized,
  {
    unsafe { self.pNext.set(e.extend(self.pNext.get())) };
    self
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'l> Default for VkValidationCacheCreateInfoEXT<'l> {
  fn default() -> VkValidationCacheCreateInfoEXT<'l> {
    VkValidationCacheCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl<'l> Struct for VkValidationCacheCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_validation_cache")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_validation_cache_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 5, VkValidationCacheCreateInfoEXT);
}

/// Specify validation cache to use during shader module creation
#[repr(C)]
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct VkShaderModuleValidationCacheCreateInfoEXT<'l, 'h: 'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  validationCache: u64,
  _p: ::std::marker::PhantomData<(&'l u8, &'h u8)>,
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'l, 'h: 'l> VkShaderModuleValidationCacheCreateInfoEXT<'l, 'h> {
  #[inline]
  pub fn new() -> VkShaderModuleValidationCacheCreateInfoEXT<'l, 'h> {
    unsafe {
      VkShaderModuleValidationCacheCreateInfoEXT {
        sType: VkStructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_validation_cache(mut self, value: VkValidationCacheEXT<'h>) -> Self {
    unsafe {
      self.validationCache = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'l, 'h: 'l> Default for VkShaderModuleValidationCacheCreateInfoEXT<'l, 'h> {
  fn default() -> VkShaderModuleValidationCacheCreateInfoEXT<'l, 'h> {
    VkShaderModuleValidationCacheCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl<'l, 'h: 'l> Struct for VkShaderModuleValidationCacheCreateInfoEXT<'l, 'h> {}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl<'m, 'l: 'm, 'h: 'l> StructExtends<VkShaderModuleCreateInfo<'m>>
  for VkShaderModuleValidationCacheCreateInfoEXT<'l, 'h>
{
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkShaderModuleValidationCacheCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_shader_module_validation_cache_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkShaderModuleValidationCacheCreateInfoEXT);
}

/// Encode validation cache version
#[cfg(feature = "VK_EXT_validation_cache")]
pub use enums::VkValidationCacheHeaderVersionEXT;

// feature: VK_EXT_global_priority

/// Values specifying a system-wide queue priority
#[cfg(feature = "VK_EXT_global_priority")]
pub use enums::VkQueueGlobalPriorityEXT;

/// Specify a system wide priority
#[repr(C)]
#[cfg(feature = "VK_EXT_global_priority")]
pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub globalPriority: VkQueueGlobalPriorityEXT,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_global_priority")]
impl<'l> VkDeviceQueueGlobalPriorityCreateInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkDeviceQueueGlobalPriorityCreateInfoEXT<'l> {
    unsafe {
      VkDeviceQueueGlobalPriorityCreateInfoEXT {
        sType: VkStructureType::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_global_priority(mut self, value: VkQueueGlobalPriorityEXT) -> Self {
    self.globalPriority = value;
    self
  }
  #[inline]
  pub fn global_priority(&self) -> VkQueueGlobalPriorityEXT {
    self.globalPriority
  }
}
#[cfg(feature = "VK_EXT_global_priority")]
impl<'l> Default for VkDeviceQueueGlobalPriorityCreateInfoEXT<'l> {
  fn default() -> VkDeviceQueueGlobalPriorityCreateInfoEXT<'l> {
    VkDeviceQueueGlobalPriorityCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_global_priority")]
unsafe impl<'l> Struct for VkDeviceQueueGlobalPriorityCreateInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_global_priority")]
unsafe impl<'m, 'l: 'm> StructExtends<VkDeviceQueueCreateInfo<'m>> for VkDeviceQueueGlobalPriorityCreateInfoEXT<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkDeviceQueueGlobalPriorityCreateInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_global_priority")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_queue_global_priority_create_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkDeviceQueueGlobalPriorityCreateInfoEXT);
}

// feature: VK_EXT_external_memory_host

/// import memory from a host pointer
#[repr(C)]
#[cfg(feature = "VK_EXT_external_memory_host")]
pub struct VkImportMemoryHostPointerInfoEXT<'l> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pHostPointer: *mut c_void,
  _p: ::std::marker::PhantomData<(&'l u8)>,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl<'l> VkImportMemoryHostPointerInfoEXT<'l> {
  #[inline]
  pub fn new() -> VkImportMemoryHostPointerInfoEXT<'l> {
    unsafe {
      VkImportMemoryHostPointerInfoEXT {
        sType: VkStructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn set_host_pointer(mut self, value: *mut c_void) -> Self {
    self.pHostPointer = value;
    self
  }
  #[inline]
  pub fn handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn host_pointer(&self) -> *mut c_void {
    self.pHostPointer
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl<'l> Default for VkImportMemoryHostPointerInfoEXT<'l> {
  fn default() -> VkImportMemoryHostPointerInfoEXT<'l> {
    VkImportMemoryHostPointerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
unsafe impl<'l> Struct for VkImportMemoryHostPointerInfoEXT<'l> {}
#[cfg(feature = "VK_EXT_external_memory_host")]
unsafe impl<'m, 'l: 'm> StructExtends<VkMemoryAllocateInfo<'m>> for VkImportMemoryHostPointerInfoEXT<'l> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkImportMemoryHostPointerInfoEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_memory_host_pointer_info_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 4, VkImportMemoryHostPointerInfoEXT);
}

/// Properties of external memory host pointer
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_external_memory_host")]
pub struct VkMemoryHostPointerPropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub memoryTypeBits: u32,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl VkMemoryHostPointerPropertiesEXT {
  #[inline]
  pub fn new() -> VkMemoryHostPointerPropertiesEXT {
    unsafe {
      VkMemoryHostPointerPropertiesEXT {
        sType: VkStructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_memory_type_bits(mut self, value: u32) -> Self {
    self.memoryTypeBits = value;
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn memory_type_bits(&self) -> u32 {
    self.memoryTypeBits
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl Default for VkMemoryHostPointerPropertiesEXT {
  fn default() -> VkMemoryHostPointerPropertiesEXT {
    VkMemoryHostPointerPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
unsafe impl Struct for VkMemoryHostPointerPropertiesEXT {}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_host_pointer_properties_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(0 + ptr_size * 3, VkMemoryHostPointerPropertiesEXT);
}

/// Structure describing external memory host pointer limits that can be supported
/// by an implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_external_memory_host")]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub minImportedHostPointerAlignment: VkDeviceSize,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    unsafe {
      VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
        sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_next(mut self, value: *mut c_void) -> Self {
    self.pNext = value;
    self
  }
  #[inline]
  pub fn set_min_imported_host_pointer_alignment(mut self, value: VkDeviceSize) -> Self {
    self.minImportedHostPointerAlignment = value;
    self
  }
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn min_imported_host_pointer_alignment(&self) -> VkDeviceSize {
    self.minImportedHostPointerAlignment
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl Default for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  fn default() -> VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    VkPhysicalDeviceExternalMemoryHostPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
unsafe impl Struct for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {}
#[cfg(feature = "VK_EXT_external_memory_host")]
unsafe impl StructExtends<VkPhysicalDeviceProperties2KHR> for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkPhysicalDeviceExternalMemoryHostPropertiesEXT as *const c_void
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_memory_host_properties_ext() {
  let ptr_size = ::std::mem::size_of::<usize>();
  assert_size!(8 + ptr_size * 2, VkPhysicalDeviceExternalMemoryHostPropertiesEXT);
}
