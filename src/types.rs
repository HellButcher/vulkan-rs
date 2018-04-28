/* GENERATED FILE */

#![allow(non_snake_case)]

pub use types_base::*;
use AsRaw;
use RawStruct;
use StructExtends;
use enums;
use platform::*;
use std::cell::Cell;
use types_raw;
use utils::VkDispatchableHandle;
use utils::VkNonDispatchableHandle;

// feature: VK_VERSION_1_0
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkBuffer__ {}

/// Opaque handle to a buffer object
pub type VkBuffer = VkNonDispatchableHandle<VkBuffer__>;

/// Structure specifying a buffer memory barrier
#[repr(C)]
pub struct VkBufferMemoryBarrier<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkBufferMemoryBarrier<'a> {
  #[inline]
  pub fn new() -> VkBufferMemoryBarrier<'a> {
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
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
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
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
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
impl<'a> Default for VkBufferMemoryBarrier<'a> {
  fn default() -> VkBufferMemoryBarrier<'a> {
    VkBufferMemoryBarrier::new()
  }
}
unsafe impl<'a> RawStruct for VkBufferMemoryBarrier<'a> {
  type Raw = types_raw::VkBufferMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_memory_barrier() {
  assert_size!(types_raw::VkBufferMemoryBarrier, VkBufferMemoryBarrier);
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
unsafe impl RawStruct for VkDispatchIndirectCommand {
  type Raw = types_raw::VkDispatchIndirectCommand;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_dispatch_indirect_command() {
  assert_size!(
    types_raw::VkDispatchIndirectCommand,
    VkDispatchIndirectCommand
  );
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
unsafe impl RawStruct for VkDrawIndexedIndirectCommand {
  type Raw = types_raw::VkDrawIndexedIndirectCommand;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_draw_indexed_indirect_command() {
  assert_size!(
    types_raw::VkDrawIndexedIndirectCommand,
    VkDrawIndexedIndirectCommand
  );
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
unsafe impl RawStruct for VkDrawIndirectCommand {
  type Raw = types_raw::VkDrawIndirectCommand;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_draw_indirect_command() {
  assert_size!(types_raw::VkDrawIndirectCommand, VkDrawIndirectCommand);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkImage__ {}

/// Opaque handle to a image object
pub type VkImage = VkNonDispatchableHandle<VkImage__>;

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
unsafe impl RawStruct for VkImageSubresourceRange {
  type Raw = types_raw::VkImageSubresourceRange;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_subresource_range() {
  assert_size!(types_raw::VkImageSubresourceRange, VkImageSubresourceRange);
}

/// Structure specifying the parameters of an image memory barrier
#[repr(C)]
pub struct VkImageMemoryBarrier<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub oldLayout: VkImageLayout,
  pub newLayout: VkImageLayout,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub image: VkImage,
  pub subresourceRange: VkImageSubresourceRange,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkImageMemoryBarrier<'a> {
  #[inline]
  pub fn new() -> VkImageMemoryBarrier<'a> {
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
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
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
  pub fn image(&self) -> VkImage {
    self.image
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
impl<'a> Default for VkImageMemoryBarrier<'a> {
  fn default() -> VkImageMemoryBarrier<'a> {
    VkImageMemoryBarrier::new()
  }
}
unsafe impl<'a> RawStruct for VkImageMemoryBarrier<'a> {
  type Raw = types_raw::VkImageMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_memory_barrier() {
  assert_size!(types_raw::VkImageMemoryBarrier, VkImageMemoryBarrier);
}

/// Structure specifying a global memory barrier
#[repr(C)]
pub struct VkMemoryBarrier<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkMemoryBarrier<'a> {
  #[inline]
  pub fn new() -> VkMemoryBarrier<'a> {
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
impl<'a> Default for VkMemoryBarrier<'a> {
  fn default() -> VkMemoryBarrier<'a> {
    VkMemoryBarrier::new()
  }
}
unsafe impl<'a> RawStruct for VkMemoryBarrier<'a> {
  type Raw = types_raw::VkMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_barrier() {
  assert_size!(types_raw::VkMemoryBarrier, VkMemoryBarrier);
}

/// Structure specifying application info
#[repr(C)]
pub struct VkApplicationInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pApplicationName: *const c_char,
  pub applicationVersion: u32,
  pEngineName: *const c_char,
  pub engineVersion: u32,
  pub apiVersion: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkApplicationInfo<'a> {
  #[inline]
  pub fn new() -> VkApplicationInfo<'a> {
    unsafe {
      VkApplicationInfo {
        sType: VkStructureType::APPLICATION_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_application_name(mut self, value: Option<&'a AsRef<CStr>>) -> Self {
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
  pub fn set_engine_name(mut self, value: Option<&'a AsRef<CStr>>) -> Self {
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
impl<'a> Default for VkApplicationInfo<'a> {
  fn default() -> VkApplicationInfo<'a> {
    VkApplicationInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkApplicationInfo<'a> {
  type Raw = types_raw::VkApplicationInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_application_info() {
  assert_size!(types_raw::VkApplicationInfo, VkApplicationInfo);
}

/// Structure specifying parameters of a newly created instance
#[repr(C)]
pub struct VkInstanceCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkInstanceCreateFlags,
  pub pApplicationInfo: Option<&'a VkApplicationInfo<'a>>,
  enabledLayerCount: u32,
  ppEnabledLayerNames: *const *const c_char,
  enabledExtensionCount: u32,
  ppEnabledExtensionNames: *const *const c_char,
}
impl<'a> VkInstanceCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkInstanceCreateInfo<'a> {
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
  pub fn set_application_info(mut self, value: Option<&'a VkApplicationInfo<'a>>) -> Self {
    self.pApplicationInfo = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkInstanceCreateFlags {
    self.flags
  }
  #[inline]
  pub fn application_info(&self) -> Option<&'a VkApplicationInfo<'a>> {
    self.pApplicationInfo
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
impl<'a> Default for VkInstanceCreateInfo<'a> {
  fn default() -> VkInstanceCreateInfo<'a> {
    VkInstanceCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkInstanceCreateInfo<'a> {
  type Raw = types_raw::VkInstanceCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_instance_create_info() {
  assert_size!(types_raw::VkInstanceCreateInfo, VkInstanceCreateInfo);
}

/// Application-defined memory allocation function
pub use types_raw::PFN_vkAllocationFunction;

/// Application-defined memory reallocation function
pub use types_raw::PFN_vkReallocationFunction;

/// Application-defined memory free function
pub use types_raw::PFN_vkFreeFunction;

/// Application-defined memory allocation notification function
pub use types_raw::PFN_vkInternalAllocationNotification;

/// Application-defined memory free notification function
pub use types_raw::PFN_vkInternalFreeNotification;

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
unsafe impl RawStruct for VkAllocationCallbacks {
  type Raw = types_raw::VkAllocationCallbacks;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_allocation_callbacks() {
  assert_size!(types_raw::VkAllocationCallbacks, VkAllocationCallbacks);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkInstance__ {}

/// Opaque handle to a instance object
pub type VkInstance = VkDispatchableHandle<VkInstance__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPhysicalDevice__ {}

/// Opaque handle to a physical device object
pub type VkPhysicalDevice = VkDispatchableHandle<VkPhysicalDevice__>;

/// Structure describing the fine-grained features that can be supported by an
/// implementation
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
impl VkPhysicalDeviceFeatures {
  #[inline]
  pub fn new() -> VkPhysicalDeviceFeatures {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_robust_buffer_access(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.robustBufferAccess = value;
    self
  }
  #[inline]
  pub fn set_full_draw_index_uint32(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.fullDrawIndexUint32 = value;
    self
  }
  #[inline]
  pub fn set_image_cube_array(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.imageCubeArray = value;
    self
  }
  #[inline]
  pub fn set_independent_blend(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.independentBlend = value;
    self
  }
  #[inline]
  pub fn set_geometry_shader(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.geometryShader = value;
    self
  }
  #[inline]
  pub fn set_tessellation_shader(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.tessellationShader = value;
    self
  }
  #[inline]
  pub fn set_sample_rate_shading(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sampleRateShading = value;
    self
  }
  #[inline]
  pub fn set_dual_src_blend(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.dualSrcBlend = value;
    self
  }
  #[inline]
  pub fn set_logic_op(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.logicOp = value;
    self
  }
  #[inline]
  pub fn set_multi_draw_indirect(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.multiDrawIndirect = value;
    self
  }
  #[inline]
  pub fn set_draw_indirect_first_instance(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.drawIndirectFirstInstance = value;
    self
  }
  #[inline]
  pub fn set_depth_clamp(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.depthClamp = value;
    self
  }
  #[inline]
  pub fn set_depth_bias_clamp(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.depthBiasClamp = value;
    self
  }
  #[inline]
  pub fn set_fill_mode_non_solid(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.fillModeNonSolid = value;
    self
  }
  #[inline]
  pub fn set_depth_bounds(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.depthBounds = value;
    self
  }
  #[inline]
  pub fn set_wide_lines(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.wideLines = value;
    self
  }
  #[inline]
  pub fn set_large_points(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.largePoints = value;
    self
  }
  #[inline]
  pub fn set_alpha_to_one(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.alphaToOne = value;
    self
  }
  #[inline]
  pub fn set_multi_viewport(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.multiViewport = value;
    self
  }
  #[inline]
  pub fn set_sampler_anisotropy(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.samplerAnisotropy = value;
    self
  }
  #[inline]
  pub fn set_texture_compression_etc2(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.textureCompressionETC2 = value;
    self
  }
  #[inline]
  pub fn set_texture_compression_astc_ldr(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.textureCompressionASTC_LDR = value;
    self
  }
  #[inline]
  pub fn set_texture_compression_bc(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.textureCompressionBC = value;
    self
  }
  #[inline]
  pub fn set_occlusion_query_precise(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.occlusionQueryPrecise = value;
    self
  }
  #[inline]
  pub fn set_pipeline_statistics_query(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.pipelineStatisticsQuery = value;
    self
  }
  #[inline]
  pub fn set_vertex_pipeline_stores_and_atomics(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.vertexPipelineStoresAndAtomics = value;
    self
  }
  #[inline]
  pub fn set_fragment_stores_and_atomics(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.fragmentStoresAndAtomics = value;
    self
  }
  #[inline]
  pub fn set_shader_tessellation_and_geometry_point_size(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderTessellationAndGeometryPointSize = value;
    self
  }
  #[inline]
  pub fn set_shader_image_gather_extended(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderImageGatherExtended = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_extended_formats(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderStorageImageExtendedFormats = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_multisample(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderStorageImageMultisample = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_read_without_format(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderStorageImageReadWithoutFormat = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_write_without_format(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderStorageImageWriteWithoutFormat = value;
    self
  }
  #[inline]
  pub fn set_shader_uniform_buffer_array_dynamic_indexing(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderUniformBufferArrayDynamicIndexing = value;
    self
  }
  #[inline]
  pub fn set_shader_sampled_image_array_dynamic_indexing(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderSampledImageArrayDynamicIndexing = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_buffer_array_dynamic_indexing(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderStorageBufferArrayDynamicIndexing = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_array_dynamic_indexing(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderStorageImageArrayDynamicIndexing = value;
    self
  }
  #[inline]
  pub fn set_shader_clip_distance(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderClipDistance = value;
    self
  }
  #[inline]
  pub fn set_shader_cull_distance(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderCullDistance = value;
    self
  }
  #[inline]
  pub fn set_shader_float64(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderFloat64 = value;
    self
  }
  #[inline]
  pub fn set_shader_int64(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderInt64 = value;
    self
  }
  #[inline]
  pub fn set_shader_int16(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderInt16 = value;
    self
  }
  #[inline]
  pub fn set_shader_resource_residency(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderResourceResidency = value;
    self
  }
  #[inline]
  pub fn set_shader_resource_min_lod(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.shaderResourceMinLod = value;
    self
  }
  #[inline]
  pub fn set_sparse_binding(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseBinding = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency_buffer(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseResidencyBuffer = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency_image2_d(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseResidencyImage2D = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency_image3_d(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseResidencyImage3D = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency2_samples(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseResidency2Samples = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency4_samples(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseResidency4Samples = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency8_samples(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseResidency8Samples = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency16_samples(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseResidency16Samples = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency_aliased(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sparseResidencyAliased = value;
    self
  }
  #[inline]
  pub fn set_variable_multisample_rate(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.variableMultisampleRate = value;
    self
  }
  #[inline]
  pub fn set_inherited_queries(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.inheritedQueries = value;
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
unsafe impl RawStruct for VkPhysicalDeviceFeatures {
  type Raw = types_raw::VkPhysicalDeviceFeatures;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_features() {
  assert_size!(
    types_raw::VkPhysicalDeviceFeatures,
    VkPhysicalDeviceFeatures
  );
}

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
unsafe impl RawStruct for VkFormatProperties {
  type Raw = types_raw::VkFormatProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_format_properties() {
  assert_size!(types_raw::VkFormatProperties, VkFormatProperties);
}

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
unsafe impl RawStruct for VkExtent3D {
  type Raw = types_raw::VkExtent3D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extent3_d() {
  assert_size!(types_raw::VkExtent3D, VkExtent3D);
}

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
unsafe impl RawStruct for VkImageFormatProperties {
  type Raw = types_raw::VkImageFormatProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_format_properties() {
  assert_size!(types_raw::VkImageFormatProperties, VkImageFormatProperties);
}

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
unsafe impl RawStruct for VkPhysicalDeviceLimits {
  type Raw = types_raw::VkPhysicalDeviceLimits;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_limits() {
  assert_size!(types_raw::VkPhysicalDeviceLimits, VkPhysicalDeviceLimits);
}

/// Structure specifying physical device sparse memory properties
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseProperties {
  pub residencyStandard2DBlockShape: VkBool32,
  pub residencyStandard2DMultisampleBlockShape: VkBool32,
  pub residencyStandard3DBlockShape: VkBool32,
  pub residencyAlignedMipSize: VkBool32,
  pub residencyNonResidentStrict: VkBool32,
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
unsafe impl RawStruct for VkPhysicalDeviceSparseProperties {
  type Raw = types_raw::VkPhysicalDeviceSparseProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_sparse_properties() {
  assert_size!(
    types_raw::VkPhysicalDeviceSparseProperties,
    VkPhysicalDeviceSparseProperties
  );
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
unsafe impl RawStruct for VkPhysicalDeviceProperties {
  type Raw = types_raw::VkPhysicalDeviceProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_properties() {
  assert_size!(
    types_raw::VkPhysicalDeviceProperties,
    VkPhysicalDeviceProperties
  );
}

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
unsafe impl RawStruct for VkQueueFamilyProperties {
  type Raw = types_raw::VkQueueFamilyProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_queue_family_properties() {
  assert_size!(types_raw::VkQueueFamilyProperties, VkQueueFamilyProperties);
}

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
unsafe impl RawStruct for VkMemoryType {
  type Raw = types_raw::VkMemoryType;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_type() {
  assert_size!(types_raw::VkMemoryType, VkMemoryType);
}

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
unsafe impl RawStruct for VkMemoryHeap {
  type Raw = types_raw::VkMemoryHeap;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_heap() {
  assert_size!(types_raw::VkMemoryHeap, VkMemoryHeap);
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
unsafe impl RawStruct for VkPhysicalDeviceMemoryProperties {
  type Raw = types_raw::VkPhysicalDeviceMemoryProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_memory_properties() {
  assert_size!(
    types_raw::VkPhysicalDeviceMemoryProperties,
    VkPhysicalDeviceMemoryProperties
  );
}

/// Dummy function pointer type returned by queries
pub use types_raw::PFN_vkVoidFunction;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDevice__ {}

/// Opaque handle to a device object
pub type VkDevice = VkDispatchableHandle<VkDevice__>;

/// Structure specifying parameters of a newly created device queue
#[repr(C)]
pub struct VkDeviceQueueCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDeviceQueueCreateFlags,
  pub queueFamilyIndex: u32,
  queueCount: u32,
  pQueuePriorities: *const f32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkDeviceQueueCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkDeviceQueueCreateInfo<'a> {
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
  pub fn set_queue_priorities(mut self, value: &'a [f32]) -> Self {
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
impl<'a> Default for VkDeviceQueueCreateInfo<'a> {
  fn default() -> VkDeviceQueueCreateInfo<'a> {
    VkDeviceQueueCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkDeviceQueueCreateInfo<'a> {
  type Raw = types_raw::VkDeviceQueueCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_queue_create_info() {
  assert_size!(types_raw::VkDeviceQueueCreateInfo, VkDeviceQueueCreateInfo);
}

/// Structure specifying parameters of a newly created device
#[repr(C)]
pub struct VkDeviceCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDeviceCreateFlags,
  queueCreateInfoCount: u32,
  pQueueCreateInfos: *const types_raw::VkDeviceQueueCreateInfo,
  enabledLayerCount: u32,
  ppEnabledLayerNames: *const *const c_char,
  enabledExtensionCount: u32,
  ppEnabledExtensionNames: *const *const c_char,
  pub pEnabledFeatures: Option<&'a VkPhysicalDeviceFeatures>,
}
impl<'a> VkDeviceCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkDeviceCreateInfo<'a> {
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
  pub fn set_queue_create_infos(mut self, value: &'a [VkDeviceQueueCreateInfo<'a>]) -> Self {
    self.queueCreateInfoCount = value.len() as u32;
    unsafe {
      self.pQueueCreateInfos = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_enabled_features(mut self, value: Option<&'a VkPhysicalDeviceFeatures>) -> Self {
    self.pEnabledFeatures = value;
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
  pub fn enabled_features(&self) -> Option<&'a VkPhysicalDeviceFeatures> {
    self.pEnabledFeatures
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
impl<'a> Default for VkDeviceCreateInfo<'a> {
  fn default() -> VkDeviceCreateInfo<'a> {
    VkDeviceCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkDeviceCreateInfo<'a> {
  type Raw = types_raw::VkDeviceCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_create_info() {
  assert_size!(types_raw::VkDeviceCreateInfo, VkDeviceCreateInfo);
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
unsafe impl RawStruct for VkExtensionProperties {
  type Raw = types_raw::VkExtensionProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extension_properties() {
  assert_size!(types_raw::VkExtensionProperties, VkExtensionProperties);
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
unsafe impl RawStruct for VkLayerProperties {
  type Raw = types_raw::VkLayerProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_layer_properties() {
  assert_size!(types_raw::VkLayerProperties, VkLayerProperties);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkQueue__ {}

/// Opaque handle to a queue object
pub type VkQueue = VkDispatchableHandle<VkQueue__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSemaphore__ {}

/// Opaque handle to a semaphore object
pub type VkSemaphore = VkNonDispatchableHandle<VkSemaphore__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkCommandBuffer__ {}

/// Opaque handle to a command buffer object
pub type VkCommandBuffer = VkDispatchableHandle<VkCommandBuffer__>;

/// Structure specifying a queue submit operation
#[repr(C)]
pub struct VkSubmitInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreCount: u32,
  pWaitSemaphores: *const types_raw::VkSemaphore,
  pWaitDstStageMask: *const VkPipelineStageFlags,
  commandBufferCount: u32,
  pCommandBuffers: *const types_raw::VkCommandBuffer,
  signalSemaphoreCount: u32,
  pSignalSemaphores: *const types_raw::VkSemaphore,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkSubmitInfo<'a> {
  #[inline]
  pub fn new() -> VkSubmitInfo<'a> {
    unsafe {
      VkSubmitInfo {
        sType: VkStructureType::SUBMIT_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_command_buffers(mut self, value: &'a [VkCommandBuffer]) -> Self {
    self.commandBufferCount = value.len() as u32;
    unsafe {
      self.pCommandBuffers = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
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
impl<'a> Default for VkSubmitInfo<'a> {
  fn default() -> VkSubmitInfo<'a> {
    VkSubmitInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkSubmitInfo<'a> {
  type Raw = types_raw::VkSubmitInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_submit_info() {
  assert_size!(types_raw::VkSubmitInfo, VkSubmitInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkFence__ {}

/// Opaque handle to a fence object
pub type VkFence = VkNonDispatchableHandle<VkFence__>;

/// Structure containing parameters of a memory allocation
#[repr(C)]
pub struct VkMemoryAllocateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeIndex: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkMemoryAllocateInfo<'a> {
  #[inline]
  pub fn new() -> VkMemoryAllocateInfo<'a> {
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
impl<'a> Default for VkMemoryAllocateInfo<'a> {
  fn default() -> VkMemoryAllocateInfo<'a> {
    VkMemoryAllocateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkMemoryAllocateInfo<'a> {
  type Raw = types_raw::VkMemoryAllocateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_allocate_info() {
  assert_size!(types_raw::VkMemoryAllocateInfo, VkMemoryAllocateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDeviceMemory__ {}

/// Opaque handle to a device memory object
pub type VkDeviceMemory = VkNonDispatchableHandle<VkDeviceMemory__>;

/// Structure specifying a mapped memory range
#[repr(C)]
pub struct VkMappedMemoryRange<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub memory: VkDeviceMemory,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkMappedMemoryRange<'a> {
  #[inline]
  pub fn new() -> VkMappedMemoryRange<'a> {
    unsafe {
      VkMappedMemoryRange {
        sType: VkStructureType::MAPPED_MEMORY_RANGE,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory) -> Self {
    self.memory = value;
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
  pub fn memory(&self) -> VkDeviceMemory {
    self.memory
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
impl<'a> Default for VkMappedMemoryRange<'a> {
  fn default() -> VkMappedMemoryRange<'a> {
    VkMappedMemoryRange::new()
  }
}
unsafe impl<'a> RawStruct for VkMappedMemoryRange<'a> {
  type Raw = types_raw::VkMappedMemoryRange;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_mapped_memory_range() {
  assert_size!(types_raw::VkMappedMemoryRange, VkMappedMemoryRange);
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
unsafe impl RawStruct for VkMemoryRequirements {
  type Raw = types_raw::VkMemoryRequirements;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_requirements() {
  assert_size!(types_raw::VkMemoryRequirements, VkMemoryRequirements);
}

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
unsafe impl RawStruct for VkSparseImageFormatProperties {
  type Raw = types_raw::VkSparseImageFormatProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_format_properties() {
  assert_size!(
    types_raw::VkSparseImageFormatProperties,
    VkSparseImageFormatProperties
  );
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
unsafe impl RawStruct for VkSparseImageMemoryRequirements {
  type Raw = types_raw::VkSparseImageMemoryRequirements;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_requirements() {
  assert_size!(
    types_raw::VkSparseImageMemoryRequirements,
    VkSparseImageMemoryRequirements
  );
}

/// Structure specifying a sparse memory bind operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseMemoryBind {
  pub resourceOffset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub memory: Option<VkDeviceMemory>,
  pub memoryOffset: VkDeviceSize,
  pub flags: VkSparseMemoryBindFlags,
}
impl VkSparseMemoryBind {
  #[inline]
  pub fn new() -> VkSparseMemoryBind {
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
  pub fn set_memory(mut self, value: Option<VkDeviceMemory>) -> Self {
    self.memory = value;
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
  pub fn memory(&self) -> Option<VkDeviceMemory> {
    self.memory
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
impl Default for VkSparseMemoryBind {
  fn default() -> VkSparseMemoryBind {
    VkSparseMemoryBind::new()
  }
}
unsafe impl RawStruct for VkSparseMemoryBind {
  type Raw = types_raw::VkSparseMemoryBind;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_memory_bind() {
  assert_size!(types_raw::VkSparseMemoryBind, VkSparseMemoryBind);
}

/// Structure specifying a sparse buffer memory bind operation
#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo<'a> {
  pub buffer: VkBuffer,
  bindCount: u32,
  pBinds: *const types_raw::VkSparseMemoryBind,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkSparseBufferMemoryBindInfo<'a> {
  #[inline]
  pub fn new() -> VkSparseBufferMemoryBindInfo<'a> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
    self
  }
  #[inline]
  pub fn set_binds(mut self, value: &'a [VkSparseMemoryBind]) -> Self {
    self.bindCount = value.len() as u32;
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn bind_count(&self) -> u32 {
    self.bindCount
  }
}
impl<'a> Default for VkSparseBufferMemoryBindInfo<'a> {
  fn default() -> VkSparseBufferMemoryBindInfo<'a> {
    VkSparseBufferMemoryBindInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkSparseBufferMemoryBindInfo<'a> {
  type Raw = types_raw::VkSparseBufferMemoryBindInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_buffer_memory_bind_info() {
  assert_size!(
    types_raw::VkSparseBufferMemoryBindInfo,
    VkSparseBufferMemoryBindInfo
  );
}

/// Structure specifying sparse image opaque memory bind info
#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo<'a> {
  pub image: VkImage,
  bindCount: u32,
  pBinds: *const types_raw::VkSparseMemoryBind,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkSparseImageOpaqueMemoryBindInfo<'a> {
  #[inline]
  pub fn new() -> VkSparseImageOpaqueMemoryBindInfo<'a> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
    self
  }
  #[inline]
  pub fn set_binds(mut self, value: &'a [VkSparseMemoryBind]) -> Self {
    self.bindCount = value.len() as u32;
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn image(&self) -> VkImage {
    self.image
  }
  #[inline]
  pub fn bind_count(&self) -> u32 {
    self.bindCount
  }
}
impl<'a> Default for VkSparseImageOpaqueMemoryBindInfo<'a> {
  fn default() -> VkSparseImageOpaqueMemoryBindInfo<'a> {
    VkSparseImageOpaqueMemoryBindInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkSparseImageOpaqueMemoryBindInfo<'a> {
  type Raw = types_raw::VkSparseImageOpaqueMemoryBindInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_opaque_memory_bind_info() {
  assert_size!(
    types_raw::VkSparseImageOpaqueMemoryBindInfo,
    VkSparseImageOpaqueMemoryBindInfo
  );
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
unsafe impl RawStruct for VkImageSubresource {
  type Raw = types_raw::VkImageSubresource;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_subresource() {
  assert_size!(types_raw::VkImageSubresource, VkImageSubresource);
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
unsafe impl RawStruct for VkOffset3D {
  type Raw = types_raw::VkOffset3D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_offset3_d() {
  assert_size!(types_raw::VkOffset3D, VkOffset3D);
}

/// Structure specifying sparse image memory bind
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBind {
  pub subresource: VkImageSubresource,
  pub offset: VkOffset3D,
  pub extent: VkExtent3D,
  pub memory: Option<VkDeviceMemory>,
  pub memoryOffset: VkDeviceSize,
  pub flags: VkSparseMemoryBindFlags,
}
impl VkSparseImageMemoryBind {
  #[inline]
  pub fn new() -> VkSparseImageMemoryBind {
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
  pub fn set_memory(mut self, value: Option<VkDeviceMemory>) -> Self {
    self.memory = value;
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
  pub fn memory(&self) -> Option<VkDeviceMemory> {
    self.memory
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
impl Default for VkSparseImageMemoryBind {
  fn default() -> VkSparseImageMemoryBind {
    VkSparseImageMemoryBind::new()
  }
}
unsafe impl RawStruct for VkSparseImageMemoryBind {
  type Raw = types_raw::VkSparseImageMemoryBind;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_bind() {
  assert_size!(types_raw::VkSparseImageMemoryBind, VkSparseImageMemoryBind);
}

/// Structure specifying sparse image memory bind info
#[repr(C)]
pub struct VkSparseImageMemoryBindInfo<'a> {
  pub image: VkImage,
  bindCount: u32,
  pBinds: *const types_raw::VkSparseImageMemoryBind,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkSparseImageMemoryBindInfo<'a> {
  #[inline]
  pub fn new() -> VkSparseImageMemoryBindInfo<'a> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
    self
  }
  #[inline]
  pub fn set_binds(mut self, value: &'a [VkSparseImageMemoryBind]) -> Self {
    self.bindCount = value.len() as u32;
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn image(&self) -> VkImage {
    self.image
  }
  #[inline]
  pub fn bind_count(&self) -> u32 {
    self.bindCount
  }
}
impl<'a> Default for VkSparseImageMemoryBindInfo<'a> {
  fn default() -> VkSparseImageMemoryBindInfo<'a> {
    VkSparseImageMemoryBindInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkSparseImageMemoryBindInfo<'a> {
  type Raw = types_raw::VkSparseImageMemoryBindInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_bind_info() {
  assert_size!(
    types_raw::VkSparseImageMemoryBindInfo,
    VkSparseImageMemoryBindInfo
  );
}

/// Structure specifying a sparse binding operation
#[repr(C)]
pub struct VkBindSparseInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreCount: u32,
  pWaitSemaphores: *const types_raw::VkSemaphore,
  bufferBindCount: u32,
  pBufferBinds: *const types_raw::VkSparseBufferMemoryBindInfo,
  imageOpaqueBindCount: u32,
  pImageOpaqueBinds: *const types_raw::VkSparseImageOpaqueMemoryBindInfo,
  imageBindCount: u32,
  pImageBinds: *const types_raw::VkSparseImageMemoryBindInfo,
  signalSemaphoreCount: u32,
  pSignalSemaphores: *const types_raw::VkSemaphore,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkBindSparseInfo<'a> {
  #[inline]
  pub fn new() -> VkBindSparseInfo<'a> {
    unsafe {
      VkBindSparseInfo {
        sType: VkStructureType::BIND_SPARSE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_wait_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
    self.waitSemaphoreCount = value.len() as u32;
    unsafe {
      self.pWaitSemaphores = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_buffer_binds(mut self, value: &'a [VkSparseBufferMemoryBindInfo<'a>]) -> Self {
    self.bufferBindCount = value.len() as u32;
    unsafe {
      self.pBufferBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_opaque_binds(mut self, value: &'a [VkSparseImageOpaqueMemoryBindInfo<'a>]) -> Self {
    self.imageOpaqueBindCount = value.len() as u32;
    unsafe {
      self.pImageOpaqueBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_binds(mut self, value: &'a [VkSparseImageMemoryBindInfo<'a>]) -> Self {
    self.imageBindCount = value.len() as u32;
    unsafe {
      self.pImageBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
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
impl<'a> Default for VkBindSparseInfo<'a> {
  fn default() -> VkBindSparseInfo<'a> {
    VkBindSparseInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkBindSparseInfo<'a> {
  type Raw = types_raw::VkBindSparseInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_sparse_info() {
  assert_size!(types_raw::VkBindSparseInfo, VkBindSparseInfo);
}

/// Structure specifying parameters of a newly created fence
#[repr(C)]
pub struct VkFenceCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkFenceCreateFlags,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkFenceCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkFenceCreateInfo<'a> {
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
impl<'a> Default for VkFenceCreateInfo<'a> {
  fn default() -> VkFenceCreateInfo<'a> {
    VkFenceCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkFenceCreateInfo<'a> {
  type Raw = types_raw::VkFenceCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_create_info() {
  assert_size!(types_raw::VkFenceCreateInfo, VkFenceCreateInfo);
}

/// Structure specifying parameters of a newly created semaphore
#[repr(C)]
pub struct VkSemaphoreCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkSemaphoreCreateFlags,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkSemaphoreCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkSemaphoreCreateInfo<'a> {
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
impl<'a> Default for VkSemaphoreCreateInfo<'a> {
  fn default() -> VkSemaphoreCreateInfo<'a> {
    VkSemaphoreCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkSemaphoreCreateInfo<'a> {
  type Raw = types_raw::VkSemaphoreCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_create_info() {
  assert_size!(types_raw::VkSemaphoreCreateInfo, VkSemaphoreCreateInfo);
}

/// Structure specifying parameters of a newly created event
#[repr(C)]
pub struct VkEventCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkEventCreateFlags,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkEventCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkEventCreateInfo<'a> {
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
impl<'a> Default for VkEventCreateInfo<'a> {
  fn default() -> VkEventCreateInfo<'a> {
    VkEventCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkEventCreateInfo<'a> {
  type Raw = types_raw::VkEventCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_event_create_info() {
  assert_size!(types_raw::VkEventCreateInfo, VkEventCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkEvent__ {}

/// Opaque handle to a event object
pub type VkEvent = VkNonDispatchableHandle<VkEvent__>;

/// Structure specifying parameters of a newly created query pool
#[repr(C)]
pub struct VkQueryPoolCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkQueryPoolCreateFlags,
  pub queryType: VkQueryType,
  pub queryCount: u32,
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkQueryPoolCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkQueryPoolCreateInfo<'a> {
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
impl<'a> Default for VkQueryPoolCreateInfo<'a> {
  fn default() -> VkQueryPoolCreateInfo<'a> {
    VkQueryPoolCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkQueryPoolCreateInfo<'a> {
  type Raw = types_raw::VkQueryPoolCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_query_pool_create_info() {
  assert_size!(types_raw::VkQueryPoolCreateInfo, VkQueryPoolCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkQueryPool__ {}

/// Opaque handle to a query pool object
pub type VkQueryPool = VkNonDispatchableHandle<VkQueryPool__>;

/// Structure specifying the parameters of a newly created buffer object
#[repr(C)]
pub struct VkBufferCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkBufferCreateFlags,
  pub size: VkDeviceSize,
  pub usage: VkBufferUsageFlags,
  pub sharingMode: VkSharingMode,
  queueFamilyIndexCount: u32,
  pQueueFamilyIndices: *const u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkBufferCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkBufferCreateInfo<'a> {
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
  pub fn set_queue_family_indices(mut self, value: &'a [u32]) -> Self {
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
impl<'a> Default for VkBufferCreateInfo<'a> {
  fn default() -> VkBufferCreateInfo<'a> {
    VkBufferCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkBufferCreateInfo<'a> {
  type Raw = types_raw::VkBufferCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_create_info() {
  assert_size!(types_raw::VkBufferCreateInfo, VkBufferCreateInfo);
}

/// Structure specifying parameters of a newly created buffer view
#[repr(C)]
pub struct VkBufferViewCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkBufferViewCreateFlags,
  pub buffer: VkBuffer,
  pub format: VkFormat,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkBufferViewCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkBufferViewCreateInfo<'a> {
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
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
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
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
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
impl<'a> Default for VkBufferViewCreateInfo<'a> {
  fn default() -> VkBufferViewCreateInfo<'a> {
    VkBufferViewCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkBufferViewCreateInfo<'a> {
  type Raw = types_raw::VkBufferViewCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_view_create_info() {
  assert_size!(types_raw::VkBufferViewCreateInfo, VkBufferViewCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkBufferView__ {}

/// Opaque handle to a buffer view object
pub type VkBufferView = VkNonDispatchableHandle<VkBufferView__>;

/// Structure specifying the parameters of a newly created image object
#[repr(C)]
pub struct VkImageCreateInfo<'a> {
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
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkImageCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkImageCreateInfo<'a> {
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
  pub fn set_queue_family_indices(mut self, value: &'a [u32]) -> Self {
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
impl<'a> Default for VkImageCreateInfo<'a> {
  fn default() -> VkImageCreateInfo<'a> {
    VkImageCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkImageCreateInfo<'a> {
  type Raw = types_raw::VkImageCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_create_info() {
  assert_size!(types_raw::VkImageCreateInfo, VkImageCreateInfo);
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
unsafe impl RawStruct for VkSubresourceLayout {
  type Raw = types_raw::VkSubresourceLayout;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subresource_layout() {
  assert_size!(types_raw::VkSubresourceLayout, VkSubresourceLayout);
}

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
unsafe impl RawStruct for VkComponentMapping {
  type Raw = types_raw::VkComponentMapping;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_component_mapping() {
  assert_size!(types_raw::VkComponentMapping, VkComponentMapping);
}

/// Structure specifying parameters of a newly created image view
#[repr(C)]
pub struct VkImageViewCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub viewType: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresourceRange: VkImageSubresourceRange,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkImageViewCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkImageViewCreateInfo<'a> {
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
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
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
  pub fn image(&self) -> VkImage {
    self.image
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
impl<'a> Default for VkImageViewCreateInfo<'a> {
  fn default() -> VkImageViewCreateInfo<'a> {
    VkImageViewCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkImageViewCreateInfo<'a> {
  type Raw = types_raw::VkImageViewCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_view_create_info() {
  assert_size!(types_raw::VkImageViewCreateInfo, VkImageViewCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkImageView__ {}

/// Opaque handle to a image view object
pub type VkImageView = VkNonDispatchableHandle<VkImageView__>;

/// Structure specifying parameters of a newly created shader module
#[repr(C)]
pub struct VkShaderModuleCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkShaderModuleCreateFlags,
  pub codeSize: usize,
  pCode: *const u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkShaderModuleCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkShaderModuleCreateInfo<'a> {
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
impl<'a> Default for VkShaderModuleCreateInfo<'a> {
  fn default() -> VkShaderModuleCreateInfo<'a> {
    VkShaderModuleCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkShaderModuleCreateInfo<'a> {
  type Raw = types_raw::VkShaderModuleCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_shader_module_create_info() {
  assert_size!(
    types_raw::VkShaderModuleCreateInfo,
    VkShaderModuleCreateInfo
  );
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkShaderModule__ {}

/// Opaque handle to a shader module object
pub type VkShaderModule = VkNonDispatchableHandle<VkShaderModule__>;

/// Structure specifying parameters of a newly created pipeline cache
#[repr(C)]
pub struct VkPipelineCacheCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCacheCreateFlags,
  initialDataSize: usize,
  pInitialData: *const c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineCacheCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineCacheCreateInfo<'a> {
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
  pub fn set_initial_data(mut self, value: &'a [u8]) -> Self {
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
impl<'a> Default for VkPipelineCacheCreateInfo<'a> {
  fn default() -> VkPipelineCacheCreateInfo<'a> {
    VkPipelineCacheCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineCacheCreateInfo<'a> {
  type Raw = types_raw::VkPipelineCacheCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_cache_create_info() {
  assert_size!(
    types_raw::VkPipelineCacheCreateInfo,
    VkPipelineCacheCreateInfo
  );
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPipelineCache__ {}

/// Opaque handle to a pipeline cache object
pub type VkPipelineCache = VkNonDispatchableHandle<VkPipelineCache__>;

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
unsafe impl RawStruct for VkSpecializationMapEntry {
  type Raw = types_raw::VkSpecializationMapEntry;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_specialization_map_entry() {
  assert_size!(
    types_raw::VkSpecializationMapEntry,
    VkSpecializationMapEntry
  );
}

/// Structure specifying specialization info
#[repr(C)]
pub struct VkSpecializationInfo<'a> {
  mapEntryCount: u32,
  pMapEntries: *const types_raw::VkSpecializationMapEntry,
  dataSize: usize,
  pData: *const c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkSpecializationInfo<'a> {
  #[inline]
  pub fn new() -> VkSpecializationInfo<'a> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_map_entries(mut self, value: &'a [VkSpecializationMapEntry]) -> Self {
    self.mapEntryCount = value.len() as u32;
    unsafe {
      self.pMapEntries = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_data(mut self, value: &'a [u8]) -> Self {
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
impl<'a> Default for VkSpecializationInfo<'a> {
  fn default() -> VkSpecializationInfo<'a> {
    VkSpecializationInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkSpecializationInfo<'a> {
  type Raw = types_raw::VkSpecializationInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_specialization_info() {
  assert_size!(types_raw::VkSpecializationInfo, VkSpecializationInfo);
}

/// Structure specifying parameters of a newly created pipeline shader stage
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineShaderStageCreateFlags,
  pub stage: VkShaderStageFlagBits,
  pub module: VkShaderModule,
  pName: *const c_char,
  pub pSpecializationInfo: Option<&'a VkSpecializationInfo<'a>>,
}
impl<'a> VkPipelineShaderStageCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineShaderStageCreateInfo<'a> {
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
  pub fn set_module(mut self, value: VkShaderModule) -> Self {
    self.module = value;
    self
  }
  #[inline]
  pub fn set_name(mut self, value: &'a AsRef<CStr>) -> Self {
    unsafe {
      self.pName = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_specialization_info(mut self, value: Option<&'a VkSpecializationInfo<'a>>) -> Self {
    self.pSpecializationInfo = value;
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
  pub fn module(&self) -> VkShaderModule {
    self.module
  }
  #[inline]
  pub fn name(&self) -> &CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pName) }
  }
  #[inline]
  pub fn specialization_info(&self) -> Option<&'a VkSpecializationInfo<'a>> {
    self.pSpecializationInfo
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
impl<'a> Default for VkPipelineShaderStageCreateInfo<'a> {
  fn default() -> VkPipelineShaderStageCreateInfo<'a> {
    VkPipelineShaderStageCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineShaderStageCreateInfo<'a> {
  type Raw = types_raw::VkPipelineShaderStageCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_shader_stage_create_info() {
  assert_size!(
    types_raw::VkPipelineShaderStageCreateInfo,
    VkPipelineShaderStageCreateInfo
  );
}

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
unsafe impl RawStruct for VkVertexInputBindingDescription {
  type Raw = types_raw::VkVertexInputBindingDescription;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_vertex_input_binding_description() {
  assert_size!(
    types_raw::VkVertexInputBindingDescription,
    VkVertexInputBindingDescription
  );
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
unsafe impl RawStruct for VkVertexInputAttributeDescription {
  type Raw = types_raw::VkVertexInputAttributeDescription;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_vertex_input_attribute_description() {
  assert_size!(
    types_raw::VkVertexInputAttributeDescription,
    VkVertexInputAttributeDescription
  );
}

/// Structure specifying parameters of a newly created pipeline vertex input state
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineVertexInputStateCreateFlags,
  vertexBindingDescriptionCount: u32,
  pVertexBindingDescriptions: *const types_raw::VkVertexInputBindingDescription,
  vertexAttributeDescriptionCount: u32,
  pVertexAttributeDescriptions: *const types_raw::VkVertexInputAttributeDescription,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineVertexInputStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineVertexInputStateCreateInfo<'a> {
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
  pub fn set_vertex_binding_descriptions(mut self, value: &'a [VkVertexInputBindingDescription]) -> Self {
    self.vertexBindingDescriptionCount = value.len() as u32;
    unsafe {
      self.pVertexBindingDescriptions = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_vertex_attribute_descriptions(mut self, value: &'a [VkVertexInputAttributeDescription]) -> Self {
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
impl<'a> Default for VkPipelineVertexInputStateCreateInfo<'a> {
  fn default() -> VkPipelineVertexInputStateCreateInfo<'a> {
    VkPipelineVertexInputStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineVertexInputStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineVertexInputStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_vertex_input_state_create_info() {
  assert_size!(
    types_raw::VkPipelineVertexInputStateCreateInfo,
    VkPipelineVertexInputStateCreateInfo
  );
}

/// Structure specifying parameters of a newly created pipeline input assembly state
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  pub primitiveRestartEnable: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineInputAssemblyStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineInputAssemblyStateCreateInfo<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.primitiveRestartEnable = value;
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
impl<'a> Default for VkPipelineInputAssemblyStateCreateInfo<'a> {
  fn default() -> VkPipelineInputAssemblyStateCreateInfo<'a> {
    VkPipelineInputAssemblyStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineInputAssemblyStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineInputAssemblyStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_input_assembly_state_create_info() {
  assert_size!(
    types_raw::VkPipelineInputAssemblyStateCreateInfo,
    VkPipelineInputAssemblyStateCreateInfo
  );
}

/// Structure specifying parameters of a newly created pipeline tessellation state
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patchControlPoints: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineTessellationStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineTessellationStateCreateInfo<'a> {
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
impl<'a> Default for VkPipelineTessellationStateCreateInfo<'a> {
  fn default() -> VkPipelineTessellationStateCreateInfo<'a> {
    VkPipelineTessellationStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineTessellationStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineTessellationStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_tessellation_state_create_info() {
  assert_size!(
    types_raw::VkPipelineTessellationStateCreateInfo,
    VkPipelineTessellationStateCreateInfo
  );
}

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
unsafe impl RawStruct for VkViewport {
  type Raw = types_raw::VkViewport;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_viewport() {
  assert_size!(types_raw::VkViewport, VkViewport);
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
unsafe impl RawStruct for VkOffset2D {
  type Raw = types_raw::VkOffset2D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_offset2_d() {
  assert_size!(types_raw::VkOffset2D, VkOffset2D);
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
unsafe impl RawStruct for VkExtent2D {
  type Raw = types_raw::VkExtent2D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extent2_d() {
  assert_size!(types_raw::VkExtent2D, VkExtent2D);
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
unsafe impl RawStruct for VkRect2D {
  type Raw = types_raw::VkRect2D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_rect2_d() {
  assert_size!(types_raw::VkRect2D, VkRect2D);
}

/// Structure specifying parameters of a newly created pipeline viewport state
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineViewportStateCreateFlags,
  viewportCount: u32,
  pViewports: *const types_raw::VkViewport,
  scissorCount: u32,
  pScissors: *const types_raw::VkRect2D,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineViewportStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineViewportStateCreateInfo<'a> {
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
  pub fn set_viewports(mut self, value: &'a [VkViewport]) -> Self {
    self.viewportCount = value.len() as u32;
    unsafe {
      self.pViewports = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_scissors(mut self, value: &'a [VkRect2D]) -> Self {
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
impl<'a> Default for VkPipelineViewportStateCreateInfo<'a> {
  fn default() -> VkPipelineViewportStateCreateInfo<'a> {
    VkPipelineViewportStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineViewportStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineViewportStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_viewport_state_create_info() {
  assert_size!(
    types_raw::VkPipelineViewportStateCreateInfo,
    VkPipelineViewportStateCreateInfo
  );
}

/// Structure specifying parameters of a newly created pipeline rasterization state
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
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
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineRasterizationStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineRasterizationStateCreateInfo<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.depthClampEnable = value;
    self
  }
  #[inline]
  pub fn set_rasterizer_discard_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.rasterizerDiscardEnable = value;
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.depthBiasEnable = value;
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
impl<'a> Default for VkPipelineRasterizationStateCreateInfo<'a> {
  fn default() -> VkPipelineRasterizationStateCreateInfo<'a> {
    VkPipelineRasterizationStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineRasterizationStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineRasterizationStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_rasterization_state_create_info() {
  assert_size!(
    types_raw::VkPipelineRasterizationStateCreateInfo,
    VkPipelineRasterizationStateCreateInfo
  );
}

/// Structure specifying parameters of a newly created pipeline multisample state
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineMultisampleStateCreateFlags,
  pub rasterizationSamples: VkSampleCountFlagBits,
  pub sampleShadingEnable: VkBool32,
  pub minSampleShading: f32,
  pSampleMask: *const VkSampleMask,
  pub alphaToCoverageEnable: VkBool32,
  pub alphaToOneEnable: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineMultisampleStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineMultisampleStateCreateInfo<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sampleShadingEnable = value;
    self
  }
  #[inline]
  pub fn set_min_sample_shading(mut self, value: f32) -> Self {
    self.minSampleShading = value;
    self
  }
  #[inline]
  pub fn set_alpha_to_coverage_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.alphaToCoverageEnable = value;
    self
  }
  #[inline]
  pub fn set_alpha_to_one_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.alphaToOneEnable = value;
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
impl<'a> Default for VkPipelineMultisampleStateCreateInfo<'a> {
  fn default() -> VkPipelineMultisampleStateCreateInfo<'a> {
    VkPipelineMultisampleStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineMultisampleStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineMultisampleStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_multisample_state_create_info() {
  assert_size!(
    types_raw::VkPipelineMultisampleStateCreateInfo,
    VkPipelineMultisampleStateCreateInfo
  );
}

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
unsafe impl RawStruct for VkStencilOpState {
  type Raw = types_raw::VkStencilOpState;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_stencil_op_state() {
  assert_size!(types_raw::VkStencilOpState, VkStencilOpState);
}

/// Structure specifying parameters of a newly created pipeline depth stencil state
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
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
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineDepthStencilStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineDepthStencilStateCreateInfo<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.depthTestEnable = value;
    self
  }
  #[inline]
  pub fn set_depth_write_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.depthWriteEnable = value;
    self
  }
  #[inline]
  pub fn set_depth_compare_op(mut self, value: VkCompareOp) -> Self {
    self.depthCompareOp = value;
    self
  }
  #[inline]
  pub fn set_depth_bounds_test_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.depthBoundsTestEnable = value;
    self
  }
  #[inline]
  pub fn set_stencil_test_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.stencilTestEnable = value;
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
impl<'a> Default for VkPipelineDepthStencilStateCreateInfo<'a> {
  fn default() -> VkPipelineDepthStencilStateCreateInfo<'a> {
    VkPipelineDepthStencilStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineDepthStencilStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineDepthStencilStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_depth_stencil_state_create_info() {
  assert_size!(
    types_raw::VkPipelineDepthStencilStateCreateInfo,
    VkPipelineDepthStencilStateCreateInfo
  );
}

/// Structure specifying a pipeline color blend attachment state
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
impl VkPipelineColorBlendAttachmentState {
  #[inline]
  pub fn new() -> VkPipelineColorBlendAttachmentState {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_blend_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.blendEnable = value;
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
unsafe impl RawStruct for VkPipelineColorBlendAttachmentState {
  type Raw = types_raw::VkPipelineColorBlendAttachmentState;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_color_blend_attachment_state() {
  assert_size!(
    types_raw::VkPipelineColorBlendAttachmentState,
    VkPipelineColorBlendAttachmentState
  );
}

/// Structure specifying parameters of a newly created pipeline color blend state
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineColorBlendStateCreateFlags,
  pub logicOpEnable: VkBool32,
  pub logicOp: VkLogicOp,
  attachmentCount: u32,
  pAttachments: *const types_raw::VkPipelineColorBlendAttachmentState,
  pub blendConstants: [f32; 4],
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineColorBlendStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineColorBlendStateCreateInfo<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.logicOpEnable = value;
    self
  }
  #[inline]
  pub fn set_logic_op(mut self, value: VkLogicOp) -> Self {
    self.logicOp = value;
    self
  }
  #[inline]
  pub fn set_attachments(mut self, value: &'a [VkPipelineColorBlendAttachmentState]) -> Self {
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
impl<'a> Default for VkPipelineColorBlendStateCreateInfo<'a> {
  fn default() -> VkPipelineColorBlendStateCreateInfo<'a> {
    VkPipelineColorBlendStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineColorBlendStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineColorBlendStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_color_blend_state_create_info() {
  assert_size!(
    types_raw::VkPipelineColorBlendStateCreateInfo,
    VkPipelineColorBlendStateCreateInfo
  );
}

/// Structure specifying parameters of a newly created pipeline dynamic state
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineDynamicStateCreateFlags,
  dynamicStateCount: u32,
  pDynamicStates: *const VkDynamicState,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineDynamicStateCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineDynamicStateCreateInfo<'a> {
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
  pub fn set_dynamic_states(mut self, value: &'a [VkDynamicState]) -> Self {
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
impl<'a> Default for VkPipelineDynamicStateCreateInfo<'a> {
  fn default() -> VkPipelineDynamicStateCreateInfo<'a> {
    VkPipelineDynamicStateCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineDynamicStateCreateInfo<'a> {
  type Raw = types_raw::VkPipelineDynamicStateCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_dynamic_state_create_info() {
  assert_size!(
    types_raw::VkPipelineDynamicStateCreateInfo,
    VkPipelineDynamicStateCreateInfo
  );
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPipelineLayout__ {}

/// Opaque handle to a pipeline layout object
pub type VkPipelineLayout = VkNonDispatchableHandle<VkPipelineLayout__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkRenderPass__ {}

/// Opaque handle to a render pass object
pub type VkRenderPass = VkNonDispatchableHandle<VkRenderPass__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPipeline__ {}

/// Opaque handle to a pipeline object
pub type VkPipeline = VkNonDispatchableHandle<VkPipeline__>;

/// Structure specifying parameters of a newly created graphics pipeline
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCreateFlags,
  stageCount: u32,
  pStages: *const types_raw::VkPipelineShaderStageCreateInfo,
  pub pVertexInputState: &'a VkPipelineVertexInputStateCreateInfo<'a>,
  pub pInputAssemblyState: &'a VkPipelineInputAssemblyStateCreateInfo<'a>,
  pub pTessellationState: Option<&'a VkPipelineTessellationStateCreateInfo<'a>>,
  pub pViewportState: Option<&'a VkPipelineViewportStateCreateInfo<'a>>,
  pub pRasterizationState: &'a VkPipelineRasterizationStateCreateInfo<'a>,
  pub pMultisampleState: Option<&'a VkPipelineMultisampleStateCreateInfo<'a>>,
  pub pDepthStencilState: Option<&'a VkPipelineDepthStencilStateCreateInfo<'a>>,
  pub pColorBlendState: Option<&'a VkPipelineColorBlendStateCreateInfo<'a>>,
  pub pDynamicState: Option<&'a VkPipelineDynamicStateCreateInfo<'a>>,
  pub layout: VkPipelineLayout,
  pub renderPass: VkRenderPass,
  pub subpass: u32,
  pub basePipelineHandle: Option<VkPipeline>,
  pub basePipelineIndex: i32,
}
impl<'a> VkGraphicsPipelineCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkGraphicsPipelineCreateInfo<'a> {
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
  pub fn set_stages(mut self, value: &'a [VkPipelineShaderStageCreateInfo<'a>]) -> Self {
    self.stageCount = value.len() as u32;
    unsafe {
      self.pStages = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_vertex_input_state(mut self, value: &'a VkPipelineVertexInputStateCreateInfo<'a>) -> Self {
    self.pVertexInputState = value;
    self
  }
  #[inline]
  pub fn set_input_assembly_state(mut self, value: &'a VkPipelineInputAssemblyStateCreateInfo<'a>) -> Self {
    self.pInputAssemblyState = value;
    self
  }
  #[inline]
  pub fn set_tessellation_state(mut self, value: Option<&'a VkPipelineTessellationStateCreateInfo<'a>>) -> Self {
    self.pTessellationState = value;
    self
  }
  #[inline]
  pub fn set_viewport_state(mut self, value: Option<&'a VkPipelineViewportStateCreateInfo<'a>>) -> Self {
    self.pViewportState = value;
    self
  }
  #[inline]
  pub fn set_rasterization_state(mut self, value: &'a VkPipelineRasterizationStateCreateInfo<'a>) -> Self {
    self.pRasterizationState = value;
    self
  }
  #[inline]
  pub fn set_multisample_state(mut self, value: Option<&'a VkPipelineMultisampleStateCreateInfo<'a>>) -> Self {
    self.pMultisampleState = value;
    self
  }
  #[inline]
  pub fn set_depth_stencil_state(mut self, value: Option<&'a VkPipelineDepthStencilStateCreateInfo<'a>>) -> Self {
    self.pDepthStencilState = value;
    self
  }
  #[inline]
  pub fn set_color_blend_state(mut self, value: Option<&'a VkPipelineColorBlendStateCreateInfo<'a>>) -> Self {
    self.pColorBlendState = value;
    self
  }
  #[inline]
  pub fn set_dynamic_state(mut self, value: Option<&'a VkPipelineDynamicStateCreateInfo<'a>>) -> Self {
    self.pDynamicState = value;
    self
  }
  #[inline]
  pub fn set_layout(mut self, value: VkPipelineLayout) -> Self {
    self.layout = value;
    self
  }
  #[inline]
  pub fn set_render_pass(mut self, value: VkRenderPass) -> Self {
    self.renderPass = value;
    self
  }
  #[inline]
  pub fn set_subpass(mut self, value: u32) -> Self {
    self.subpass = value;
    self
  }
  #[inline]
  pub fn set_base_pipeline_handle(mut self, value: Option<VkPipeline>) -> Self {
    self.basePipelineHandle = value;
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
  pub fn vertex_input_state(&self) -> &'a VkPipelineVertexInputStateCreateInfo<'a> {
    self.pVertexInputState
  }
  #[inline]
  pub fn input_assembly_state(&self) -> &'a VkPipelineInputAssemblyStateCreateInfo<'a> {
    self.pInputAssemblyState
  }
  #[inline]
  pub fn tessellation_state(&self) -> Option<&'a VkPipelineTessellationStateCreateInfo<'a>> {
    self.pTessellationState
  }
  #[inline]
  pub fn viewport_state(&self) -> Option<&'a VkPipelineViewportStateCreateInfo<'a>> {
    self.pViewportState
  }
  #[inline]
  pub fn rasterization_state(&self) -> &'a VkPipelineRasterizationStateCreateInfo<'a> {
    self.pRasterizationState
  }
  #[inline]
  pub fn multisample_state(&self) -> Option<&'a VkPipelineMultisampleStateCreateInfo<'a>> {
    self.pMultisampleState
  }
  #[inline]
  pub fn depth_stencil_state(&self) -> Option<&'a VkPipelineDepthStencilStateCreateInfo<'a>> {
    self.pDepthStencilState
  }
  #[inline]
  pub fn color_blend_state(&self) -> Option<&'a VkPipelineColorBlendStateCreateInfo<'a>> {
    self.pColorBlendState
  }
  #[inline]
  pub fn dynamic_state(&self) -> Option<&'a VkPipelineDynamicStateCreateInfo<'a>> {
    self.pDynamicState
  }
  #[inline]
  pub fn layout(&self) -> VkPipelineLayout {
    self.layout
  }
  #[inline]
  pub fn render_pass(&self) -> VkRenderPass {
    self.renderPass
  }
  #[inline]
  pub fn subpass(&self) -> u32 {
    self.subpass
  }
  #[inline]
  pub fn base_pipeline_handle(&self) -> Option<VkPipeline> {
    self.basePipelineHandle
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
impl<'a> Default for VkGraphicsPipelineCreateInfo<'a> {
  fn default() -> VkGraphicsPipelineCreateInfo<'a> {
    VkGraphicsPipelineCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkGraphicsPipelineCreateInfo<'a> {
  type Raw = types_raw::VkGraphicsPipelineCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_graphics_pipeline_create_info() {
  assert_size!(
    types_raw::VkGraphicsPipelineCreateInfo,
    VkGraphicsPipelineCreateInfo
  );
}

/// Structure specifying parameters of a newly created compute pipeline
#[repr(C)]
pub struct VkComputePipelineCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCreateFlags,
  pub stage: VkPipelineShaderStageCreateInfo<'a>,
  pub layout: VkPipelineLayout,
  pub basePipelineHandle: Option<VkPipeline>,
  pub basePipelineIndex: i32,
}
impl<'a> VkComputePipelineCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkComputePipelineCreateInfo<'a> {
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
  pub fn set_stage(mut self, value: VkPipelineShaderStageCreateInfo<'a>) -> Self {
    self.stage = value;
    self
  }
  #[inline]
  pub fn set_layout(mut self, value: VkPipelineLayout) -> Self {
    self.layout = value;
    self
  }
  #[inline]
  pub fn set_base_pipeline_handle(mut self, value: Option<VkPipeline>) -> Self {
    self.basePipelineHandle = value;
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
  pub fn stage(&self) -> &VkPipelineShaderStageCreateInfo<'a> {
    &self.stage
  }
  #[inline]
  pub fn layout(&self) -> VkPipelineLayout {
    self.layout
  }
  #[inline]
  pub fn base_pipeline_handle(&self) -> Option<VkPipeline> {
    self.basePipelineHandle
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
impl<'a> Default for VkComputePipelineCreateInfo<'a> {
  fn default() -> VkComputePipelineCreateInfo<'a> {
    VkComputePipelineCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkComputePipelineCreateInfo<'a> {
  type Raw = types_raw::VkComputePipelineCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_compute_pipeline_create_info() {
  assert_size!(
    types_raw::VkComputePipelineCreateInfo,
    VkComputePipelineCreateInfo
  );
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDescriptorSetLayout__ {}

/// Opaque handle to a descriptor set layout object
pub type VkDescriptorSetLayout = VkNonDispatchableHandle<VkDescriptorSetLayout__>;

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
unsafe impl RawStruct for VkPushConstantRange {
  type Raw = types_raw::VkPushConstantRange;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_push_constant_range() {
  assert_size!(types_raw::VkPushConstantRange, VkPushConstantRange);
}

/// Structure specifying the parameters of a newly created pipeline layout object
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineLayoutCreateFlags,
  setLayoutCount: u32,
  pSetLayouts: *const types_raw::VkDescriptorSetLayout,
  pushConstantRangeCount: u32,
  pPushConstantRanges: *const types_raw::VkPushConstantRange,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkPipelineLayoutCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkPipelineLayoutCreateInfo<'a> {
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
  pub fn set_set_layouts(mut self, value: &'a [VkDescriptorSetLayout]) -> Self {
    self.setLayoutCount = value.len() as u32;
    unsafe {
      self.pSetLayouts = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_push_constant_ranges(mut self, value: &'a [VkPushConstantRange]) -> Self {
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
impl<'a> Default for VkPipelineLayoutCreateInfo<'a> {
  fn default() -> VkPipelineLayoutCreateInfo<'a> {
    VkPipelineLayoutCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkPipelineLayoutCreateInfo<'a> {
  type Raw = types_raw::VkPipelineLayoutCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_layout_create_info() {
  assert_size!(
    types_raw::VkPipelineLayoutCreateInfo,
    VkPipelineLayoutCreateInfo
  );
}

/// Structure specifying parameters of a newly created sampler
#[repr(C)]
pub struct VkSamplerCreateInfo<'a> {
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
  pub anisotropyEnable: VkBool32,
  pub maxAnisotropy: f32,
  pub compareEnable: VkBool32,
  pub compareOp: VkCompareOp,
  pub minLod: f32,
  pub maxLod: f32,
  pub borderColor: VkBorderColor,
  pub unnormalizedCoordinates: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkSamplerCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkSamplerCreateInfo<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.anisotropyEnable = value;
    self
  }
  #[inline]
  pub fn set_max_anisotropy(mut self, value: f32) -> Self {
    self.maxAnisotropy = value;
    self
  }
  #[inline]
  pub fn set_compare_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.compareEnable = value;
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.unnormalizedCoordinates = value;
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
impl<'a> Default for VkSamplerCreateInfo<'a> {
  fn default() -> VkSamplerCreateInfo<'a> {
    VkSamplerCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkSamplerCreateInfo<'a> {
  type Raw = types_raw::VkSamplerCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sampler_create_info() {
  assert_size!(types_raw::VkSamplerCreateInfo, VkSamplerCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSampler__ {}

/// Opaque handle to a sampler object
pub type VkSampler = VkNonDispatchableHandle<VkSampler__>;

/// Structure specifying a descriptor set layout binding
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding<'a> {
  pub binding: u32,
  pub descriptorType: VkDescriptorType,
  descriptorCount: u32,
  pub stageFlags: VkShaderStageFlags,
  pImmutableSamplers: *const types_raw::VkSampler,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkDescriptorSetLayoutBinding<'a> {
  #[inline]
  pub fn new() -> VkDescriptorSetLayoutBinding<'a> {
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
  pub fn set_immutable_samplers(mut self, value: &'a [VkSampler]) -> Self {
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
impl<'a> Default for VkDescriptorSetLayoutBinding<'a> {
  fn default() -> VkDescriptorSetLayoutBinding<'a> {
    VkDescriptorSetLayoutBinding::new()
  }
}
unsafe impl<'a> RawStruct for VkDescriptorSetLayoutBinding<'a> {
  type Raw = types_raw::VkDescriptorSetLayoutBinding;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_set_layout_binding() {
  assert_size!(
    types_raw::VkDescriptorSetLayoutBinding,
    VkDescriptorSetLayoutBinding
  );
}

/// Structure specifying parameters of a newly created descriptor set layout
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDescriptorSetLayoutCreateFlags,
  bindingCount: u32,
  pBindings: *const types_raw::VkDescriptorSetLayoutBinding,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkDescriptorSetLayoutCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkDescriptorSetLayoutCreateInfo<'a> {
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
  pub fn set_bindings(mut self, value: &'a [VkDescriptorSetLayoutBinding<'a>]) -> Self {
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
impl<'a> Default for VkDescriptorSetLayoutCreateInfo<'a> {
  fn default() -> VkDescriptorSetLayoutCreateInfo<'a> {
    VkDescriptorSetLayoutCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkDescriptorSetLayoutCreateInfo<'a> {
  type Raw = types_raw::VkDescriptorSetLayoutCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_set_layout_create_info() {
  assert_size!(
    types_raw::VkDescriptorSetLayoutCreateInfo,
    VkDescriptorSetLayoutCreateInfo
  );
}

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
unsafe impl RawStruct for VkDescriptorPoolSize {
  type Raw = types_raw::VkDescriptorPoolSize;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_pool_size() {
  assert_size!(types_raw::VkDescriptorPoolSize, VkDescriptorPoolSize);
}

/// Structure specifying parameters of a newly created descriptor pool
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDescriptorPoolCreateFlags,
  pub maxSets: u32,
  poolSizeCount: u32,
  pPoolSizes: *const types_raw::VkDescriptorPoolSize,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkDescriptorPoolCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkDescriptorPoolCreateInfo<'a> {
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
  pub fn set_pool_sizes(mut self, value: &'a [VkDescriptorPoolSize]) -> Self {
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
impl<'a> Default for VkDescriptorPoolCreateInfo<'a> {
  fn default() -> VkDescriptorPoolCreateInfo<'a> {
    VkDescriptorPoolCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkDescriptorPoolCreateInfo<'a> {
  type Raw = types_raw::VkDescriptorPoolCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_pool_create_info() {
  assert_size!(
    types_raw::VkDescriptorPoolCreateInfo,
    VkDescriptorPoolCreateInfo
  );
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDescriptorPool__ {}

/// Opaque handle to a descriptor pool object
pub type VkDescriptorPool = VkNonDispatchableHandle<VkDescriptorPool__>;

/// Structure specifying the allocation parameters for descriptor sets
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub descriptorPool: VkDescriptorPool,
  descriptorSetCount: u32,
  pSetLayouts: *const types_raw::VkDescriptorSetLayout,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkDescriptorSetAllocateInfo<'a> {
  #[inline]
  pub fn new() -> VkDescriptorSetAllocateInfo<'a> {
    unsafe {
      VkDescriptorSetAllocateInfo {
        sType: VkStructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_descriptor_pool(mut self, value: VkDescriptorPool) -> Self {
    self.descriptorPool = value;
    self
  }
  #[inline]
  pub fn set_set_layouts(mut self, value: &'a [VkDescriptorSetLayout]) -> Self {
    self.descriptorSetCount = value.len() as u32;
    unsafe {
      self.pSetLayouts = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn descriptor_pool(&self) -> VkDescriptorPool {
    self.descriptorPool
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
impl<'a> Default for VkDescriptorSetAllocateInfo<'a> {
  fn default() -> VkDescriptorSetAllocateInfo<'a> {
    VkDescriptorSetAllocateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkDescriptorSetAllocateInfo<'a> {
  type Raw = types_raw::VkDescriptorSetAllocateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_set_allocate_info() {
  assert_size!(
    types_raw::VkDescriptorSetAllocateInfo,
    VkDescriptorSetAllocateInfo
  );
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDescriptorSet__ {}

/// Opaque handle to a descriptor set object
pub type VkDescriptorSet = VkNonDispatchableHandle<VkDescriptorSet__>;

/// Structure specifying descriptor image info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorImageInfo {
  pub sampler: VkSampler,
  pub imageView: VkImageView,
  pub imageLayout: VkImageLayout,
}
impl VkDescriptorImageInfo {
  #[inline]
  pub fn new() -> VkDescriptorImageInfo {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_sampler(mut self, value: VkSampler) -> Self {
    self.sampler = value;
    self
  }
  #[inline]
  pub fn set_image_view(mut self, value: VkImageView) -> Self {
    self.imageView = value;
    self
  }
  #[inline]
  pub fn set_image_layout(mut self, value: VkImageLayout) -> Self {
    self.imageLayout = value;
    self
  }
  #[inline]
  pub fn sampler(&self) -> VkSampler {
    self.sampler
  }
  #[inline]
  pub fn image_view(&self) -> VkImageView {
    self.imageView
  }
  #[inline]
  pub fn image_layout(&self) -> VkImageLayout {
    self.imageLayout
  }
}
impl Default for VkDescriptorImageInfo {
  fn default() -> VkDescriptorImageInfo {
    VkDescriptorImageInfo::new()
  }
}
unsafe impl RawStruct for VkDescriptorImageInfo {
  type Raw = types_raw::VkDescriptorImageInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_image_info() {
  assert_size!(types_raw::VkDescriptorImageInfo, VkDescriptorImageInfo);
}

/// Structure specifying descriptor buffer info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorBufferInfo {
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
}
impl VkDescriptorBufferInfo {
  #[inline]
  pub fn new() -> VkDescriptorBufferInfo {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
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
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
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
impl Default for VkDescriptorBufferInfo {
  fn default() -> VkDescriptorBufferInfo {
    VkDescriptorBufferInfo::new()
  }
}
unsafe impl RawStruct for VkDescriptorBufferInfo {
  type Raw = types_raw::VkDescriptorBufferInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_buffer_info() {
  assert_size!(types_raw::VkDescriptorBufferInfo, VkDescriptorBufferInfo);
}

/// Structure specifying the parameters of a descriptor set write operation
#[repr(C)]
pub struct VkWriteDescriptorSet<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub dstSet: VkDescriptorSet,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  descriptorCount: u32,
  pub descriptorType: VkDescriptorType,
  pImageInfo: *const types_raw::VkDescriptorImageInfo,
  pBufferInfo: *const types_raw::VkDescriptorBufferInfo,
  pTexelBufferView: *const types_raw::VkBufferView,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkWriteDescriptorSet<'a> {
  #[inline]
  pub fn new() -> VkWriteDescriptorSet<'a> {
    unsafe {
      VkWriteDescriptorSet {
        sType: VkStructureType::WRITE_DESCRIPTOR_SET,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dst_set(mut self, value: VkDescriptorSet) -> Self {
    self.dstSet = value;
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
  pub fn dst_set(&self) -> VkDescriptorSet {
    self.dstSet
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
impl<'a> Default for VkWriteDescriptorSet<'a> {
  fn default() -> VkWriteDescriptorSet<'a> {
    VkWriteDescriptorSet::new()
  }
}
unsafe impl<'a> RawStruct for VkWriteDescriptorSet<'a> {
  type Raw = types_raw::VkWriteDescriptorSet;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_write_descriptor_set() {
  assert_size!(types_raw::VkWriteDescriptorSet, VkWriteDescriptorSet);
}

/// Structure specifying a copy descriptor set operation
#[repr(C)]
pub struct VkCopyDescriptorSet<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcSet: VkDescriptorSet,
  pub srcBinding: u32,
  pub srcArrayElement: u32,
  pub dstSet: VkDescriptorSet,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkCopyDescriptorSet<'a> {
  #[inline]
  pub fn new() -> VkCopyDescriptorSet<'a> {
    unsafe {
      VkCopyDescriptorSet {
        sType: VkStructureType::COPY_DESCRIPTOR_SET,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_set(mut self, value: VkDescriptorSet) -> Self {
    self.srcSet = value;
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
  pub fn set_dst_set(mut self, value: VkDescriptorSet) -> Self {
    self.dstSet = value;
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
  pub fn src_set(&self) -> VkDescriptorSet {
    self.srcSet
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
  pub fn dst_set(&self) -> VkDescriptorSet {
    self.dstSet
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
impl<'a> Default for VkCopyDescriptorSet<'a> {
  fn default() -> VkCopyDescriptorSet<'a> {
    VkCopyDescriptorSet::new()
  }
}
unsafe impl<'a> RawStruct for VkCopyDescriptorSet<'a> {
  type Raw = types_raw::VkCopyDescriptorSet;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_copy_descriptor_set() {
  assert_size!(types_raw::VkCopyDescriptorSet, VkCopyDescriptorSet);
}

/// Structure specifying parameters of a newly created framebuffer
#[repr(C)]
pub struct VkFramebufferCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkFramebufferCreateFlags,
  pub renderPass: VkRenderPass,
  attachmentCount: u32,
  pAttachments: *const types_raw::VkImageView,
  pub width: u32,
  pub height: u32,
  pub layers: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkFramebufferCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkFramebufferCreateInfo<'a> {
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
  pub fn set_render_pass(mut self, value: VkRenderPass) -> Self {
    self.renderPass = value;
    self
  }
  #[inline]
  pub fn set_attachments(mut self, value: &'a [VkImageView]) -> Self {
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
  pub fn render_pass(&self) -> VkRenderPass {
    self.renderPass
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
impl<'a> Default for VkFramebufferCreateInfo<'a> {
  fn default() -> VkFramebufferCreateInfo<'a> {
    VkFramebufferCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkFramebufferCreateInfo<'a> {
  type Raw = types_raw::VkFramebufferCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_framebuffer_create_info() {
  assert_size!(types_raw::VkFramebufferCreateInfo, VkFramebufferCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkFramebuffer__ {}

/// Opaque handle to a framebuffer object
pub type VkFramebuffer = VkNonDispatchableHandle<VkFramebuffer__>;

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
unsafe impl RawStruct for VkAttachmentDescription {
  type Raw = types_raw::VkAttachmentDescription;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_attachment_description() {
  assert_size!(types_raw::VkAttachmentDescription, VkAttachmentDescription);
}

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
unsafe impl RawStruct for VkAttachmentReference {
  type Raw = types_raw::VkAttachmentReference;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_attachment_reference() {
  assert_size!(types_raw::VkAttachmentReference, VkAttachmentReference);
}

/// Structure specifying a subpass description
#[repr(C)]
pub struct VkSubpassDescription<'a> {
  pub flags: VkSubpassDescriptionFlags,
  pub pipelineBindPoint: VkPipelineBindPoint,
  inputAttachmentCount: u32,
  pInputAttachments: *const types_raw::VkAttachmentReference,
  colorAttachmentCount: u32,
  pColorAttachments: *const types_raw::VkAttachmentReference,
  pResolveAttachments: *const types_raw::VkAttachmentReference,
  pub pDepthStencilAttachment: Option<&'a VkAttachmentReference>,
  preserveAttachmentCount: u32,
  pPreserveAttachments: *const u32,
}
impl<'a> VkSubpassDescription<'a> {
  #[inline]
  pub fn new() -> VkSubpassDescription<'a> {
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
  pub fn set_input_attachments(mut self, value: &'a [VkAttachmentReference]) -> Self {
    self.inputAttachmentCount = value.len() as u32;
    unsafe {
      self.pInputAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_depth_stencil_attachment(mut self, value: Option<&'a VkAttachmentReference>) -> Self {
    self.pDepthStencilAttachment = value;
    self
  }
  #[inline]
  pub fn set_preserve_attachments(mut self, value: &'a [u32]) -> Self {
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
  pub fn depth_stencil_attachment(&self) -> Option<&'a VkAttachmentReference> {
    self.pDepthStencilAttachment
  }
  #[inline]
  pub fn preserve_attachment_count(&self) -> u32 {
    self.preserveAttachmentCount
  }
}
impl<'a> Default for VkSubpassDescription<'a> {
  fn default() -> VkSubpassDescription<'a> {
    VkSubpassDescription::new()
  }
}
unsafe impl<'a> RawStruct for VkSubpassDescription<'a> {
  type Raw = types_raw::VkSubpassDescription;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subpass_description() {
  assert_size!(types_raw::VkSubpassDescription, VkSubpassDescription);
}

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
unsafe impl RawStruct for VkSubpassDependency {
  type Raw = types_raw::VkSubpassDependency;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subpass_dependency() {
  assert_size!(types_raw::VkSubpassDependency, VkSubpassDependency);
}

/// Structure specifying parameters of a newly created render pass
#[repr(C)]
pub struct VkRenderPassCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkRenderPassCreateFlags,
  attachmentCount: u32,
  pAttachments: *const types_raw::VkAttachmentDescription,
  subpassCount: u32,
  pSubpasses: *const types_raw::VkSubpassDescription,
  dependencyCount: u32,
  pDependencies: *const types_raw::VkSubpassDependency,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkRenderPassCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkRenderPassCreateInfo<'a> {
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
  pub fn set_attachments(mut self, value: &'a [VkAttachmentDescription]) -> Self {
    self.attachmentCount = value.len() as u32;
    unsafe {
      self.pAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_subpasses(mut self, value: &'a [VkSubpassDescription<'a>]) -> Self {
    self.subpassCount = value.len() as u32;
    unsafe {
      self.pSubpasses = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_dependencies(mut self, value: &'a [VkSubpassDependency]) -> Self {
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
impl<'a> Default for VkRenderPassCreateInfo<'a> {
  fn default() -> VkRenderPassCreateInfo<'a> {
    VkRenderPassCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkRenderPassCreateInfo<'a> {
  type Raw = types_raw::VkRenderPassCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_create_info() {
  assert_size!(types_raw::VkRenderPassCreateInfo, VkRenderPassCreateInfo);
}

/// Structure specifying parameters of a newly created command pool
#[repr(C)]
pub struct VkCommandPoolCreateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkCommandPoolCreateFlags,
  pub queueFamilyIndex: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkCommandPoolCreateInfo<'a> {
  #[inline]
  pub fn new() -> VkCommandPoolCreateInfo<'a> {
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
impl<'a> Default for VkCommandPoolCreateInfo<'a> {
  fn default() -> VkCommandPoolCreateInfo<'a> {
    VkCommandPoolCreateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkCommandPoolCreateInfo<'a> {
  type Raw = types_raw::VkCommandPoolCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_command_pool_create_info() {
  assert_size!(types_raw::VkCommandPoolCreateInfo, VkCommandPoolCreateInfo);
}
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkCommandPool__ {}

/// Opaque handle to a command pool object
pub type VkCommandPool = VkNonDispatchableHandle<VkCommandPool__>;

/// Structure specifying the allocation parameters for command buffer object
#[repr(C)]
pub struct VkCommandBufferAllocateInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub commandPool: VkCommandPool,
  pub level: VkCommandBufferLevel,
  pub commandBufferCount: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkCommandBufferAllocateInfo<'a> {
  #[inline]
  pub fn new() -> VkCommandBufferAllocateInfo<'a> {
    unsafe {
      VkCommandBufferAllocateInfo {
        sType: VkStructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_command_pool(mut self, value: VkCommandPool) -> Self {
    self.commandPool = value;
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
  pub fn command_pool(&self) -> VkCommandPool {
    self.commandPool
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
impl<'a> Default for VkCommandBufferAllocateInfo<'a> {
  fn default() -> VkCommandBufferAllocateInfo<'a> {
    VkCommandBufferAllocateInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkCommandBufferAllocateInfo<'a> {
  type Raw = types_raw::VkCommandBufferAllocateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_command_buffer_allocate_info() {
  assert_size!(
    types_raw::VkCommandBufferAllocateInfo,
    VkCommandBufferAllocateInfo
  );
}

/// Structure specifying command buffer inheritance info
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub renderPass: Option<VkRenderPass>,
  pub subpass: u32,
  pub framebuffer: Option<VkFramebuffer>,
  pub occlusionQueryEnable: VkBool32,
  pub queryFlags: VkQueryControlFlags,
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkCommandBufferInheritanceInfo<'a> {
  #[inline]
  pub fn new() -> VkCommandBufferInheritanceInfo<'a> {
    unsafe {
      VkCommandBufferInheritanceInfo {
        sType: VkStructureType::COMMAND_BUFFER_INHERITANCE_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_render_pass(mut self, value: Option<VkRenderPass>) -> Self {
    self.renderPass = value;
    self
  }
  #[inline]
  pub fn set_subpass(mut self, value: u32) -> Self {
    self.subpass = value;
    self
  }
  #[inline]
  pub fn set_framebuffer(mut self, value: Option<VkFramebuffer>) -> Self {
    self.framebuffer = value;
    self
  }
  #[inline]
  pub fn set_occlusion_query_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.occlusionQueryEnable = value;
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
  pub fn render_pass(&self) -> Option<VkRenderPass> {
    self.renderPass
  }
  #[inline]
  pub fn subpass(&self) -> u32 {
    self.subpass
  }
  #[inline]
  pub fn framebuffer(&self) -> Option<VkFramebuffer> {
    self.framebuffer
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
impl<'a> Default for VkCommandBufferInheritanceInfo<'a> {
  fn default() -> VkCommandBufferInheritanceInfo<'a> {
    VkCommandBufferInheritanceInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkCommandBufferInheritanceInfo<'a> {
  type Raw = types_raw::VkCommandBufferInheritanceInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_command_buffer_inheritance_info() {
  assert_size!(
    types_raw::VkCommandBufferInheritanceInfo,
    VkCommandBufferInheritanceInfo
  );
}

/// Structure specifying a command buffer begin operation
#[repr(C)]
pub struct VkCommandBufferBeginInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkCommandBufferUsageFlags,
  pub pInheritanceInfo: Option<&'a VkCommandBufferInheritanceInfo<'a>>,
}
impl<'a> VkCommandBufferBeginInfo<'a> {
  #[inline]
  pub fn new() -> VkCommandBufferBeginInfo<'a> {
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
  pub fn set_inheritance_info(mut self, value: Option<&'a VkCommandBufferInheritanceInfo<'a>>) -> Self {
    self.pInheritanceInfo = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkCommandBufferUsageFlags {
    self.flags
  }
  #[inline]
  pub fn inheritance_info(&self) -> Option<&'a VkCommandBufferInheritanceInfo<'a>> {
    self.pInheritanceInfo
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
impl<'a> Default for VkCommandBufferBeginInfo<'a> {
  fn default() -> VkCommandBufferBeginInfo<'a> {
    VkCommandBufferBeginInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkCommandBufferBeginInfo<'a> {
  type Raw = types_raw::VkCommandBufferBeginInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_command_buffer_begin_info() {
  assert_size!(
    types_raw::VkCommandBufferBeginInfo,
    VkCommandBufferBeginInfo
  );
}

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
unsafe impl RawStruct for VkBufferCopy {
  type Raw = types_raw::VkBufferCopy;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_copy() {
  assert_size!(types_raw::VkBufferCopy, VkBufferCopy);
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
unsafe impl RawStruct for VkImageSubresourceLayers {
  type Raw = types_raw::VkImageSubresourceLayers;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_subresource_layers() {
  assert_size!(
    types_raw::VkImageSubresourceLayers,
    VkImageSubresourceLayers
  );
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
unsafe impl RawStruct for VkImageCopy {
  type Raw = types_raw::VkImageCopy;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_copy() {
  assert_size!(types_raw::VkImageCopy, VkImageCopy);
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
unsafe impl RawStruct for VkImageBlit {
  type Raw = types_raw::VkImageBlit;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_blit() {
  assert_size!(types_raw::VkImageBlit, VkImageBlit);
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
unsafe impl RawStruct for VkBufferImageCopy {
  type Raw = types_raw::VkBufferImageCopy;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_image_copy() {
  assert_size!(types_raw::VkBufferImageCopy, VkBufferImageCopy);
}

/// Structure specifying a clear color value
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
  pub float32: [f32; 4],
  pub int32: [i32; 4],
  pub uint32: [u32; 4],
}
unsafe impl RawStruct for VkClearColorValue {
  type Raw = types_raw::VkClearColorValue;
}
#[cfg(test)]
#[test]
fn test_union_size_vk_clear_color_value() {
  assert_size!(types_raw::VkClearColorValue, VkClearColorValue);
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
unsafe impl RawStruct for VkClearDepthStencilValue {
  type Raw = types_raw::VkClearDepthStencilValue;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_clear_depth_stencil_value() {
  assert_size!(
    types_raw::VkClearDepthStencilValue,
    VkClearDepthStencilValue
  );
}

/// Structure specifying a clear value
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
  pub color: VkClearColorValue,
  pub depthStencil: VkClearDepthStencilValue,
}
unsafe impl RawStruct for VkClearValue {
  type Raw = types_raw::VkClearValue;
}
#[cfg(test)]
#[test]
fn test_union_size_vk_clear_value() {
  assert_size!(types_raw::VkClearValue, VkClearValue);
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
unsafe impl RawStruct for VkClearAttachment {
  type Raw = types_raw::VkClearAttachment;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_clear_attachment() {
  assert_size!(types_raw::VkClearAttachment, VkClearAttachment);
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
unsafe impl RawStruct for VkClearRect {
  type Raw = types_raw::VkClearRect;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_clear_rect() {
  assert_size!(types_raw::VkClearRect, VkClearRect);
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
unsafe impl RawStruct for VkImageResolve {
  type Raw = types_raw::VkImageResolve;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_resolve() {
  assert_size!(types_raw::VkImageResolve, VkImageResolve);
}

/// Structure specifying render pass begin info
#[repr(C)]
pub struct VkRenderPassBeginInfo<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub renderPass: VkRenderPass,
  pub framebuffer: VkFramebuffer,
  pub renderArea: VkRect2D,
  clearValueCount: u32,
  pClearValues: *const types_raw::VkClearValue,
  _p: ::std::marker::PhantomData<&'a u8>,
}
impl<'a> VkRenderPassBeginInfo<'a> {
  #[inline]
  pub fn new() -> VkRenderPassBeginInfo<'a> {
    unsafe {
      VkRenderPassBeginInfo {
        sType: VkStructureType::RENDER_PASS_BEGIN_INFO,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_render_pass(mut self, value: VkRenderPass) -> Self {
    self.renderPass = value;
    self
  }
  #[inline]
  pub fn set_framebuffer(mut self, value: VkFramebuffer) -> Self {
    self.framebuffer = value;
    self
  }
  #[inline]
  pub fn set_render_area(mut self, value: VkRect2D) -> Self {
    self.renderArea = value;
    self
  }
  #[inline]
  pub fn set_clear_values(mut self, value: &'a [VkClearValue]) -> Self {
    self.clearValueCount = value.len() as u32;
    unsafe {
      self.pClearValues = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn render_pass(&self) -> VkRenderPass {
    self.renderPass
  }
  #[inline]
  pub fn framebuffer(&self) -> VkFramebuffer {
    self.framebuffer
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
impl<'a> Default for VkRenderPassBeginInfo<'a> {
  fn default() -> VkRenderPassBeginInfo<'a> {
    VkRenderPassBeginInfo::new()
  }
}
unsafe impl<'a> RawStruct for VkRenderPassBeginInfo<'a> {
  type Raw = types_raw::VkRenderPassBeginInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_begin_info() {
  assert_size!(types_raw::VkRenderPassBeginInfo, VkRenderPassBeginInfo);
}

// feature: VK_KHR_surface
#[cfg(feature = "VK_KHR_surface")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSurfaceKHR__ {}

/// Opaque handle to a surface object
#[cfg(feature = "VK_KHR_surface")]
pub type VkSurfaceKHR = VkNonDispatchableHandle<VkSurfaceKHR__>;

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
unsafe impl RawStruct for VkSurfaceCapabilitiesKHR {
  type Raw = types_raw::VkSurfaceCapabilitiesKHR;
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_capabilities_khr() {
  assert_size!(
    types_raw::VkSurfaceCapabilitiesKHR,
    VkSurfaceCapabilitiesKHR
  );
}

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
unsafe impl RawStruct for VkSurfaceFormatKHR {
  type Raw = types_raw::VkSurfaceFormatKHR;
}
#[cfg(feature = "VK_KHR_surface")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_format_khr() {
  assert_size!(types_raw::VkSurfaceFormatKHR, VkSurfaceFormatKHR);
}

// feature: VK_KHR_swapchain
#[cfg(feature = "VK_KHR_swapchain")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSwapchainKHR__ {}

/// Opaque handle to a swapchain object
#[cfg(feature = "VK_KHR_swapchain")]
pub type VkSwapchainKHR = VkNonDispatchableHandle<VkSwapchainKHR__>;

/// Structure specifying parameters of a newly created swapchain object
#[repr(C)]
#[cfg(feature = "VK_KHR_swapchain")]
pub struct VkSwapchainCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkSwapchainCreateFlagsKHR,
  pub surface: VkSurfaceKHR,
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
  pub clipped: VkBool32,
  pub oldSwapchain: Option<VkSwapchainKHR>,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'a> VkSwapchainCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkSwapchainCreateInfoKHR<'a> {
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
  pub fn set_surface(mut self, value: VkSurfaceKHR) -> Self {
    self.surface = value;
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
  pub fn set_queue_family_indices(mut self, value: &'a [u32]) -> Self {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.clipped = value;
    self
  }
  #[inline]
  pub fn set_old_swapchain(mut self, value: Option<VkSwapchainKHR>) -> Self {
    self.oldSwapchain = value;
    self
  }
  #[inline]
  pub fn flags(&self) -> VkSwapchainCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn surface(&self) -> VkSurfaceKHR {
    self.surface
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
  pub fn old_swapchain(&self) -> Option<VkSwapchainKHR> {
    self.oldSwapchain
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
impl<'a> Default for VkSwapchainCreateInfoKHR<'a> {
  fn default() -> VkSwapchainCreateInfoKHR<'a> {
    VkSwapchainCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<'a> RawStruct for VkSwapchainCreateInfoKHR<'a> {
  type Raw = types_raw::VkSwapchainCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_swapchain_create_info_khr() {
  assert_size!(
    types_raw::VkSwapchainCreateInfoKHR,
    VkSwapchainCreateInfoKHR
  );
}

/// Structure describing parameters of a queue presentation
#[repr(C)]
#[cfg(feature = "VK_KHR_swapchain")]
pub struct VkPresentInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreCount: u32,
  pWaitSemaphores: *const types_raw::VkSemaphore,
  swapchainCount: u32,
  pSwapchains: *const types_raw::VkSwapchainKHR,
  pImageIndices: *const u32,
  pResults: *mut VkResult,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'a> VkPresentInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkPresentInfoKHR<'a> {
    unsafe {
      VkPresentInfoKHR {
        sType: VkStructureType::PRESENT_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_wait_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
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
impl<'a> Default for VkPresentInfoKHR<'a> {
  fn default() -> VkPresentInfoKHR<'a> {
    VkPresentInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<'a> RawStruct for VkPresentInfoKHR<'a> {
  type Raw = types_raw::VkPresentInfoKHR;
}
#[cfg(feature = "VK_KHR_swapchain")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_info_khr() {
  assert_size!(types_raw::VkPresentInfoKHR, VkPresentInfoKHR);
}

// feature: VK_KHR_display
#[cfg(feature = "VK_KHR_display")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDisplayKHR__ {}

/// Opaque handle to a display object
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayKHR = VkNonDispatchableHandle<VkDisplayKHR__>;

/// Structure describing an available display device
#[repr(C)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayPropertiesKHR<'a> {
  pub display: VkDisplayKHR,
  displayName: *const c_char,
  pub physicalDimensions: VkExtent2D,
  pub physicalResolution: VkExtent2D,
  pub supportedTransforms: VkSurfaceTransformFlagsKHR,
  pub planeReorderPossible: VkBool32,
  pub persistentContent: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_display")]
impl<'a> VkDisplayPropertiesKHR<'a> {
  #[inline]
  pub fn display(&self) -> VkDisplayKHR {
    self.display
  }
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
unsafe impl<'a> RawStruct for VkDisplayPropertiesKHR<'a> {
  type Raw = types_raw::VkDisplayPropertiesKHR;
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_properties_khr() {
  assert_size!(types_raw::VkDisplayPropertiesKHR, VkDisplayPropertiesKHR);
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
unsafe impl RawStruct for VkDisplayModeParametersKHR {
  type Raw = types_raw::VkDisplayModeParametersKHR;
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_mode_parameters_khr() {
  assert_size!(
    types_raw::VkDisplayModeParametersKHR,
    VkDisplayModeParametersKHR
  );
}
#[cfg(feature = "VK_KHR_display")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDisplayModeKHR__ {}

/// Opaque handle to a display mode object
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayModeKHR = VkNonDispatchableHandle<VkDisplayModeKHR__>;

/// Structure describing display mode properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModePropertiesKHR {
  pub displayMode: VkDisplayModeKHR,
  pub parameters: VkDisplayModeParametersKHR,
}
#[cfg(feature = "VK_KHR_display")]
impl VkDisplayModePropertiesKHR {
  #[inline]
  pub fn display_mode(&self) -> VkDisplayModeKHR {
    self.displayMode
  }
  #[inline]
  pub fn parameters(&self) -> &VkDisplayModeParametersKHR {
    &self.parameters
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl RawStruct for VkDisplayModePropertiesKHR {
  type Raw = types_raw::VkDisplayModePropertiesKHR;
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_mode_properties_khr() {
  assert_size!(
    types_raw::VkDisplayModePropertiesKHR,
    VkDisplayModePropertiesKHR
  );
}

/// Structure specifying parameters of a newly created display mode object
#[repr(C)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModeCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDisplayModeCreateFlagsKHR,
  pub parameters: VkDisplayModeParametersKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_display")]
impl<'a> VkDisplayModeCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkDisplayModeCreateInfoKHR<'a> {
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
impl<'a> Default for VkDisplayModeCreateInfoKHR<'a> {
  fn default() -> VkDisplayModeCreateInfoKHR<'a> {
    VkDisplayModeCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl<'a> RawStruct for VkDisplayModeCreateInfoKHR<'a> {
  type Raw = types_raw::VkDisplayModeCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_mode_create_info_khr() {
  assert_size!(
    types_raw::VkDisplayModeCreateInfoKHR,
    VkDisplayModeCreateInfoKHR
  );
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
unsafe impl RawStruct for VkDisplayPlaneCapabilitiesKHR {
  type Raw = types_raw::VkDisplayPlaneCapabilitiesKHR;
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_plane_capabilities_khr() {
  assert_size!(
    types_raw::VkDisplayPlaneCapabilitiesKHR,
    VkDisplayPlaneCapabilitiesKHR
  );
}

/// Structure describing display plane properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayPlanePropertiesKHR {
  pub currentDisplay: VkDisplayKHR,
  pub currentStackIndex: u32,
}
#[cfg(feature = "VK_KHR_display")]
impl VkDisplayPlanePropertiesKHR {
  #[inline]
  pub fn current_display(&self) -> VkDisplayKHR {
    self.currentDisplay
  }
  #[inline]
  pub fn current_stack_index(&self) -> u32 {
    self.currentStackIndex
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl RawStruct for VkDisplayPlanePropertiesKHR {
  type Raw = types_raw::VkDisplayPlanePropertiesKHR;
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_plane_properties_khr() {
  assert_size!(
    types_raw::VkDisplayPlanePropertiesKHR,
    VkDisplayPlanePropertiesKHR
  );
}

/// Structure specifying parameters of a newly created display plane surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplaySurfaceCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDisplaySurfaceCreateFlagsKHR,
  pub displayMode: VkDisplayModeKHR,
  pub planeIndex: u32,
  pub planeStackIndex: u32,
  pub transform: VkSurfaceTransformFlagBitsKHR,
  pub globalAlpha: f32,
  pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
  pub imageExtent: VkExtent2D,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_display")]
impl<'a> VkDisplaySurfaceCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkDisplaySurfaceCreateInfoKHR<'a> {
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
  pub fn set_display_mode(mut self, value: VkDisplayModeKHR) -> Self {
    self.displayMode = value;
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
  pub fn display_mode(&self) -> VkDisplayModeKHR {
    self.displayMode
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
impl<'a> Default for VkDisplaySurfaceCreateInfoKHR<'a> {
  fn default() -> VkDisplaySurfaceCreateInfoKHR<'a> {
    VkDisplaySurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl<'a> RawStruct for VkDisplaySurfaceCreateInfoKHR<'a> {
  type Raw = types_raw::VkDisplaySurfaceCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_surface_create_info_khr() {
  assert_size!(
    types_raw::VkDisplaySurfaceCreateInfoKHR,
    VkDisplaySurfaceCreateInfoKHR
  );
}

// feature: VK_KHR_display_swapchain

/// Structure describing parameters of a queue presentation to a swapchain
#[repr(C)]
#[cfg(feature = "VK_KHR_display_swapchain")]
pub struct VkDisplayPresentInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcRect: VkRect2D,
  pub dstRect: VkRect2D,
  pub persistent: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl<'a> VkDisplayPresentInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkDisplayPresentInfoKHR<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.persistent = value;
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
impl<'a> Default for VkDisplayPresentInfoKHR<'a> {
  fn default() -> VkDisplayPresentInfoKHR<'a> {
    VkDisplayPresentInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
unsafe impl<'a> RawStruct for VkDisplayPresentInfoKHR<'a> {
  type Raw = types_raw::VkDisplayPresentInfoKHR;
}
#[cfg(feature = "VK_KHR_display_swapchain")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPresentInfoKHR<'b>> for VkDisplayPresentInfoKHR<'a> {
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
  assert_size!(types_raw::VkDisplayPresentInfoKHR, VkDisplayPresentInfoKHR);
}

// feature: VK_KHR_xlib_surface

/// Structure specifying parameters of a newly created Xlib surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub struct VkXlibSurfaceCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkXlibSurfaceCreateFlagsKHR,
  pub dpy: *mut wsi::xlib::Display,
  pub window: wsi::xlib::Window,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
impl<'a> VkXlibSurfaceCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkXlibSurfaceCreateInfoKHR<'a> {
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
impl<'a> Default for VkXlibSurfaceCreateInfoKHR<'a> {
  fn default() -> VkXlibSurfaceCreateInfoKHR<'a> {
    VkXlibSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
unsafe impl<'a> RawStruct for VkXlibSurfaceCreateInfoKHR<'a> {
  type Raw = types_raw::VkXlibSurfaceCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_xlib_surface_create_info_khr() {
  assert_size!(
    types_raw::VkXlibSurfaceCreateInfoKHR,
    VkXlibSurfaceCreateInfoKHR
  );
}

// feature: VK_KHR_xcb_surface

/// Structure specifying parameters of a newly created Xcb surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub struct VkXcbSurfaceCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkXcbSurfaceCreateFlagsKHR,
  pub connection: *mut wsi::xcb::xcb_connection_t,
  pub window: wsi::xcb::xcb_window_t,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
impl<'a> VkXcbSurfaceCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkXcbSurfaceCreateInfoKHR<'a> {
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
impl<'a> Default for VkXcbSurfaceCreateInfoKHR<'a> {
  fn default() -> VkXcbSurfaceCreateInfoKHR<'a> {
    VkXcbSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
unsafe impl<'a> RawStruct for VkXcbSurfaceCreateInfoKHR<'a> {
  type Raw = types_raw::VkXcbSurfaceCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_xcb_surface_create_info_khr() {
  assert_size!(
    types_raw::VkXcbSurfaceCreateInfoKHR,
    VkXcbSurfaceCreateInfoKHR
  );
}

// feature: VK_KHR_wayland_surface

/// Structure specifying parameters of a newly created Wayland surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub struct VkWaylandSurfaceCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkWaylandSurfaceCreateFlagsKHR,
  pub display: *mut wsi::wayland::wl_display,
  pub surface: *mut wsi::wayland::wl_surface,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
impl<'a> VkWaylandSurfaceCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkWaylandSurfaceCreateInfoKHR<'a> {
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
impl<'a> Default for VkWaylandSurfaceCreateInfoKHR<'a> {
  fn default() -> VkWaylandSurfaceCreateInfoKHR<'a> {
    VkWaylandSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
unsafe impl<'a> RawStruct for VkWaylandSurfaceCreateInfoKHR<'a> {
  type Raw = types_raw::VkWaylandSurfaceCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_wayland_surface_create_info_khr() {
  assert_size!(
    types_raw::VkWaylandSurfaceCreateInfoKHR,
    VkWaylandSurfaceCreateInfoKHR
  );
}

// feature: VK_KHR_mir_surface

/// Structure specifying parameters of a newly created Mir surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub struct VkMirSurfaceCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkMirSurfaceCreateFlagsKHR,
  pub connection: *mut wsi::mir::MirConnection,
  pub mirSurface: *mut wsi::mir::MirSurface,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
impl<'a> VkMirSurfaceCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkMirSurfaceCreateInfoKHR<'a> {
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
impl<'a> Default for VkMirSurfaceCreateInfoKHR<'a> {
  fn default() -> VkMirSurfaceCreateInfoKHR<'a> {
    VkMirSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
unsafe impl<'a> RawStruct for VkMirSurfaceCreateInfoKHR<'a> {
  type Raw = types_raw::VkMirSurfaceCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_mir_surface_create_info_khr() {
  assert_size!(
    types_raw::VkMirSurfaceCreateInfoKHR,
    VkMirSurfaceCreateInfoKHR
  );
}

// feature: VK_KHR_android_surface

/// Structure specifying parameters of a newly created Android surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub struct VkAndroidSurfaceCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkAndroidSurfaceCreateFlagsKHR,
  pub window: *mut wsi::android::ANativeWindow,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
impl<'a> VkAndroidSurfaceCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkAndroidSurfaceCreateInfoKHR<'a> {
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
impl<'a> Default for VkAndroidSurfaceCreateInfoKHR<'a> {
  fn default() -> VkAndroidSurfaceCreateInfoKHR<'a> {
    VkAndroidSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
unsafe impl<'a> RawStruct for VkAndroidSurfaceCreateInfoKHR<'a> {
  type Raw = types_raw::VkAndroidSurfaceCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_android_surface_create_info_khr() {
  assert_size!(
    types_raw::VkAndroidSurfaceCreateInfoKHR,
    VkAndroidSurfaceCreateInfoKHR
  );
}

// feature: VK_KHR_win32_surface

/// Structure specifying parameters of a newly created Win32 surface object
#[repr(C)]
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32SurfaceCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkWin32SurfaceCreateFlagsKHR,
  pub hinstance: wsi::win32::HINSTANCE,
  pub hwnd: wsi::win32::HWND,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkWin32SurfaceCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkWin32SurfaceCreateInfoKHR<'a> {
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
impl<'a> Default for VkWin32SurfaceCreateInfoKHR<'a> {
  fn default() -> VkWin32SurfaceCreateInfoKHR<'a> {
    VkWin32SurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkWin32SurfaceCreateInfoKHR<'a> {
  type Raw = types_raw::VkWin32SurfaceCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_win32_surface_create_info_khr() {
  assert_size!(
    types_raw::VkWin32SurfaceCreateInfoKHR,
    VkWin32SurfaceCreateInfoKHR
  );
}

// feature: VK_EXT_debug_report

/// Application-defined debug report callback function
#[cfg(feature = "VK_EXT_debug_report")]
pub use types_raw::PFN_vkDebugReportCallbackEXT;

/// Structure specifying parameters of a newly created debug report callback
#[repr(C)]
#[cfg(feature = "VK_EXT_debug_report")]
pub struct VkDebugReportCallbackCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkDebugReportFlagsEXT,
  pub pfnCallback: PFN_vkDebugReportCallbackEXT,
  pUserData: *mut c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<'a> VkDebugReportCallbackCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkDebugReportCallbackCreateInfoEXT<'a> {
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
impl<'a> Default for VkDebugReportCallbackCreateInfoEXT<'a> {
  fn default() -> VkDebugReportCallbackCreateInfoEXT<'a> {
    VkDebugReportCallbackCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl<'a> RawStruct for VkDebugReportCallbackCreateInfoEXT<'a> {
  type Raw = types_raw::VkDebugReportCallbackCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl<'b, 'a: 'b> StructExtends<VkInstanceCreateInfo<'b>> for VkDebugReportCallbackCreateInfoEXT<'a> {
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
  assert_size!(
    types_raw::VkDebugReportCallbackCreateInfoEXT,
    VkDebugReportCallbackCreateInfoEXT
  );
}
#[cfg(feature = "VK_EXT_debug_report")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDebugReportCallbackEXT__ {}

/// Opaque handle to a debug report callback object
#[cfg(feature = "VK_EXT_debug_report")]
pub type VkDebugReportCallbackEXT = VkNonDispatchableHandle<VkDebugReportCallbackEXT__>;

// feature: VK_AMD_rasterization_order

/// Structure defining rasterization order for a graphics pipeline
#[repr(C)]
#[cfg(feature = "VK_AMD_rasterization_order")]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub rasterizationOrder: VkRasterizationOrderAMD,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_AMD_rasterization_order")]
impl<'a> VkPipelineRasterizationStateRasterizationOrderAMD<'a> {
  #[inline]
  pub fn new() -> VkPipelineRasterizationStateRasterizationOrderAMD<'a> {
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
impl<'a> Default for VkPipelineRasterizationStateRasterizationOrderAMD<'a> {
  fn default() -> VkPipelineRasterizationStateRasterizationOrderAMD<'a> {
    VkPipelineRasterizationStateRasterizationOrderAMD::new()
  }
}
#[cfg(feature = "VK_AMD_rasterization_order")]
unsafe impl<'a> RawStruct for VkPipelineRasterizationStateRasterizationOrderAMD<'a> {
  type Raw = types_raw::VkPipelineRasterizationStateRasterizationOrderAMD;
}
#[cfg(feature = "VK_AMD_rasterization_order")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineRasterizationStateCreateInfo<'b>>
  for VkPipelineRasterizationStateRasterizationOrderAMD<'a>
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
  assert_size!(
    types_raw::VkPipelineRasterizationStateRasterizationOrderAMD,
    VkPipelineRasterizationStateRasterizationOrderAMD
  );
}

// feature: VK_EXT_debug_marker

/// Specify parameters of a name to give to an object
#[repr(C)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerObjectNameInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub objectType: VkDebugReportObjectTypeEXT,
  pub object: u64,
  pObjectName: *const c_char,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'a> VkDebugMarkerObjectNameInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkDebugMarkerObjectNameInfoEXT<'a> {
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
  pub fn set_object_name(mut self, value: &'a AsRef<CStr>) -> Self {
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
impl<'a> Default for VkDebugMarkerObjectNameInfoEXT<'a> {
  fn default() -> VkDebugMarkerObjectNameInfoEXT<'a> {
    VkDebugMarkerObjectNameInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
unsafe impl<'a> RawStruct for VkDebugMarkerObjectNameInfoEXT<'a> {
  type Raw = types_raw::VkDebugMarkerObjectNameInfoEXT;
}
#[cfg(feature = "VK_EXT_debug_marker")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_debug_marker_object_name_info_ext() {
  assert_size!(
    types_raw::VkDebugMarkerObjectNameInfoEXT,
    VkDebugMarkerObjectNameInfoEXT
  );
}

/// Specify parameters of a tag to attach to an object
#[repr(C)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerObjectTagInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub objectType: VkDebugReportObjectTypeEXT,
  pub object: u64,
  pub tagName: u64,
  tagSize: usize,
  pTag: *const c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'a> VkDebugMarkerObjectTagInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkDebugMarkerObjectTagInfoEXT<'a> {
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
  pub fn set_tag(mut self, value: &'a [u8]) -> Self {
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
impl<'a> Default for VkDebugMarkerObjectTagInfoEXT<'a> {
  fn default() -> VkDebugMarkerObjectTagInfoEXT<'a> {
    VkDebugMarkerObjectTagInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
unsafe impl<'a> RawStruct for VkDebugMarkerObjectTagInfoEXT<'a> {
  type Raw = types_raw::VkDebugMarkerObjectTagInfoEXT;
}
#[cfg(feature = "VK_EXT_debug_marker")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_debug_marker_object_tag_info_ext() {
  assert_size!(
    types_raw::VkDebugMarkerObjectTagInfoEXT,
    VkDebugMarkerObjectTagInfoEXT
  );
}

/// Specify parameters of a command buffer marker region
#[repr(C)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerMarkerInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pMarkerName: *const c_char,
  pub color: [f32; 4],
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'a> VkDebugMarkerMarkerInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkDebugMarkerMarkerInfoEXT<'a> {
    unsafe {
      VkDebugMarkerMarkerInfoEXT {
        sType: VkStructureType::DEBUG_MARKER_MARKER_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_marker_name(mut self, value: &'a AsRef<CStr>) -> Self {
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
impl<'a> Default for VkDebugMarkerMarkerInfoEXT<'a> {
  fn default() -> VkDebugMarkerMarkerInfoEXT<'a> {
    VkDebugMarkerMarkerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
unsafe impl<'a> RawStruct for VkDebugMarkerMarkerInfoEXT<'a> {
  type Raw = types_raw::VkDebugMarkerMarkerInfoEXT;
}
#[cfg(feature = "VK_EXT_debug_marker")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_debug_marker_marker_info_ext() {
  assert_size!(
    types_raw::VkDebugMarkerMarkerInfoEXT,
    VkDebugMarkerMarkerInfoEXT
  );
}

// feature: VK_NV_dedicated_allocation

/// Specify that an image is bound to a dedicated memory resource
#[repr(C)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationImageCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub dedicatedAllocation: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> VkDedicatedAllocationImageCreateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkDedicatedAllocationImageCreateInfoNV<'a> {
    unsafe {
      VkDedicatedAllocationImageCreateInfoNV {
        sType: VkStructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dedicated_allocation(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.dedicatedAllocation = value;
    self
  }
  #[inline]
  pub fn is_dedicated_allocation(&self) -> bool {
    self.dedicatedAllocation != 0
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> Default for VkDedicatedAllocationImageCreateInfoNV<'a> {
  fn default() -> VkDedicatedAllocationImageCreateInfoNV<'a> {
    VkDedicatedAllocationImageCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> RawStruct for VkDedicatedAllocationImageCreateInfoNV<'a> {
  type Raw = types_raw::VkDedicatedAllocationImageCreateInfoNV;
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageCreateInfo<'b>> for VkDedicatedAllocationImageCreateInfoNV<'a> {
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
  assert_size!(
    types_raw::VkDedicatedAllocationImageCreateInfoNV,
    VkDedicatedAllocationImageCreateInfoNV
  );
}

/// Specify that a buffer is bound to a dedicated memory resource
#[repr(C)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationBufferCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub dedicatedAllocation: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> VkDedicatedAllocationBufferCreateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkDedicatedAllocationBufferCreateInfoNV<'a> {
    unsafe {
      VkDedicatedAllocationBufferCreateInfoNV {
        sType: VkStructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dedicated_allocation(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.dedicatedAllocation = value;
    self
  }
  #[inline]
  pub fn is_dedicated_allocation(&self) -> bool {
    self.dedicatedAllocation != 0
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> Default for VkDedicatedAllocationBufferCreateInfoNV<'a> {
  fn default() -> VkDedicatedAllocationBufferCreateInfoNV<'a> {
    VkDedicatedAllocationBufferCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> RawStruct for VkDedicatedAllocationBufferCreateInfoNV<'a> {
  type Raw = types_raw::VkDedicatedAllocationBufferCreateInfoNV;
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'b, 'a: 'b> StructExtends<VkBufferCreateInfo<'b>> for VkDedicatedAllocationBufferCreateInfoNV<'a> {
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
  assert_size!(
    types_raw::VkDedicatedAllocationBufferCreateInfoNV,
    VkDedicatedAllocationBufferCreateInfoNV
  );
}

/// Specify a dedicated memory allocation resource
#[repr(C)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub image: Option<VkImage>,
  pub buffer: Option<VkBuffer>,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
    unsafe {
      VkDedicatedAllocationMemoryAllocateInfoNV {
        sType: VkStructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: Option<VkImage>) -> Self {
    self.image = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: Option<VkBuffer>) -> Self {
    self.buffer = value;
    self
  }
  #[inline]
  pub fn image(&self) -> Option<VkImage> {
    self.image
  }
  #[inline]
  pub fn buffer(&self) -> Option<VkBuffer> {
    self.buffer
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> Default for VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
  fn default() -> VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
    VkDedicatedAllocationMemoryAllocateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> RawStruct for VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
  type Raw = types_raw::VkDedicatedAllocationMemoryAllocateInfoNV;
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
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
  assert_size!(
    types_raw::VkDedicatedAllocationMemoryAllocateInfoNV,
    VkDedicatedAllocationMemoryAllocateInfoNV
  );
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
unsafe impl RawStruct for VkPhysicalDeviceFeatures2KHR {
  type Raw = types_raw::VkPhysicalDeviceFeatures2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl<'b> StructExtends<VkDeviceCreateInfo<'b>> for VkPhysicalDeviceFeatures2KHR {
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
  assert_size!(
    types_raw::VkPhysicalDeviceFeatures2KHR,
    VkPhysicalDeviceFeatures2KHR
  );
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
unsafe impl RawStruct for VkPhysicalDeviceProperties2KHR {
  type Raw = types_raw::VkPhysicalDeviceProperties2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_properties2_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceProperties2KHR,
    VkPhysicalDeviceProperties2KHR
  );
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
unsafe impl RawStruct for VkFormatProperties2KHR {
  type Raw = types_raw::VkFormatProperties2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_format_properties2_khr() {
  assert_size!(types_raw::VkFormatProperties2KHR, VkFormatProperties2KHR);
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
unsafe impl RawStruct for VkImageFormatProperties2KHR {
  type Raw = types_raw::VkImageFormatProperties2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_format_properties2_khr() {
  assert_size!(
    types_raw::VkImageFormatProperties2KHR,
    VkImageFormatProperties2KHR
  );
}

/// Structure specifying image creation parameters
#[repr(C)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceImageFormatInfo2KHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub format: VkFormat,
  pub eType: VkImageType,
  pub tiling: VkImageTiling,
  pub usage: VkImageUsageFlags,
  pub flags: VkImageCreateFlags,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'a> VkPhysicalDeviceImageFormatInfo2KHR<'a> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceImageFormatInfo2KHR<'a> {
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
impl<'a> Default for VkPhysicalDeviceImageFormatInfo2KHR<'a> {
  fn default() -> VkPhysicalDeviceImageFormatInfo2KHR<'a> {
    VkPhysicalDeviceImageFormatInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl<'a> RawStruct for VkPhysicalDeviceImageFormatInfo2KHR<'a> {
  type Raw = types_raw::VkPhysicalDeviceImageFormatInfo2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_image_format_info2_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceImageFormatInfo2KHR,
    VkPhysicalDeviceImageFormatInfo2KHR
  );
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
unsafe impl RawStruct for VkQueueFamilyProperties2KHR {
  type Raw = types_raw::VkQueueFamilyProperties2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_queue_family_properties2_khr() {
  assert_size!(
    types_raw::VkQueueFamilyProperties2KHR,
    VkQueueFamilyProperties2KHR
  );
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
unsafe impl RawStruct for VkPhysicalDeviceMemoryProperties2KHR {
  type Raw = types_raw::VkPhysicalDeviceMemoryProperties2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_memory_properties2_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceMemoryProperties2KHR,
    VkPhysicalDeviceMemoryProperties2KHR
  );
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
unsafe impl RawStruct for VkSparseImageFormatProperties2KHR {
  type Raw = types_raw::VkSparseImageFormatProperties2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_format_properties2_khr() {
  assert_size!(
    types_raw::VkSparseImageFormatProperties2KHR,
    VkSparseImageFormatProperties2KHR
  );
}

/// Structure specifying sparse image format inputs
#[repr(C)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub format: VkFormat,
  pub eType: VkImageType,
  pub samples: VkSampleCountFlagBits,
  pub usage: VkImageUsageFlags,
  pub tiling: VkImageTiling,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl<'a> VkPhysicalDeviceSparseImageFormatInfo2KHR<'a> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceSparseImageFormatInfo2KHR<'a> {
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
impl<'a> Default for VkPhysicalDeviceSparseImageFormatInfo2KHR<'a> {
  fn default() -> VkPhysicalDeviceSparseImageFormatInfo2KHR<'a> {
    VkPhysicalDeviceSparseImageFormatInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl<'a> RawStruct for VkPhysicalDeviceSparseImageFormatInfo2KHR<'a> {
  type Raw = types_raw::VkPhysicalDeviceSparseImageFormatInfo2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_sparse_image_format_info2_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceSparseImageFormatInfo2KHR,
    VkPhysicalDeviceSparseImageFormatInfo2KHR
  );
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
  pub supportsTextureGatherLODBiasAMD: VkBool32,
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
unsafe impl RawStruct for VkTextureLODGatherFormatPropertiesAMD {
  type Raw = types_raw::VkTextureLODGatherFormatPropertiesAMD;
}
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
  assert_size!(
    types_raw::VkTextureLODGatherFormatPropertiesAMD,
    VkTextureLODGatherFormatPropertiesAMD
  );
}

// feature: VK_AMD_shader_info

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
unsafe impl RawStruct for VkShaderResourceUsageAMD {
  type Raw = types_raw::VkShaderResourceUsageAMD;
}
#[cfg(feature = "VK_AMD_shader_info")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_shader_resource_usage_amd() {
  assert_size!(
    types_raw::VkShaderResourceUsageAMD,
    VkShaderResourceUsageAMD
  );
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
unsafe impl RawStruct for VkShaderStatisticsInfoAMD {
  type Raw = types_raw::VkShaderStatisticsInfoAMD;
}
#[cfg(feature = "VK_AMD_shader_info")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_shader_statistics_info_amd() {
  assert_size!(
    types_raw::VkShaderStatisticsInfoAMD,
    VkShaderStatisticsInfoAMD
  );
}

// feature: VK_KHX_multiview

/// Structure containing multiview info for all subpasses
#[repr(C)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkRenderPassMultiviewCreateInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  subpassCount: u32,
  pViewMasks: *const u32,
  dependencyCount: u32,
  pViewOffsets: *const i32,
  correlationMaskCount: u32,
  pCorrelationMasks: *const u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_multiview")]
impl<'a> VkRenderPassMultiviewCreateInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkRenderPassMultiviewCreateInfoKHX<'a> {
    unsafe {
      VkRenderPassMultiviewCreateInfoKHX {
        sType: VkStructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_view_masks(mut self, value: &'a [u32]) -> Self {
    self.subpassCount = value.len() as u32;
    unsafe {
      self.pViewMasks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_view_offsets(mut self, value: &'a [i32]) -> Self {
    self.dependencyCount = value.len() as u32;
    unsafe {
      self.pViewOffsets = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_correlation_masks(mut self, value: &'a [u32]) -> Self {
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
impl<'a> Default for VkRenderPassMultiviewCreateInfoKHX<'a> {
  fn default() -> VkRenderPassMultiviewCreateInfoKHX<'a> {
    VkRenderPassMultiviewCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl<'a> RawStruct for VkRenderPassMultiviewCreateInfoKHX<'a> {
  type Raw = types_raw::VkRenderPassMultiviewCreateInfoKHX;
}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl<'b, 'a: 'b> StructExtends<VkRenderPassCreateInfo<'b>> for VkRenderPassMultiviewCreateInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkRenderPassMultiviewCreateInfoKHX,
    VkRenderPassMultiviewCreateInfoKHX
  );
}

/// Structure describing multiview features that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkPhysicalDeviceMultiviewFeaturesKHX {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub multiview: VkBool32,
  pub multiviewGeometryShader: VkBool32,
  pub multiviewTessellationShader: VkBool32,
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.multiview = value;
    self
  }
  #[inline]
  pub fn set_multiview_geometry_shader(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.multiviewGeometryShader = value;
    self
  }
  #[inline]
  pub fn set_multiview_tessellation_shader(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.multiviewTessellationShader = value;
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
unsafe impl RawStruct for VkPhysicalDeviceMultiviewFeaturesKHX {
  type Raw = types_raw::VkPhysicalDeviceMultiviewFeaturesKHX;
}
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
unsafe impl<'b> StructExtends<VkDeviceCreateInfo<'b>> for VkPhysicalDeviceMultiviewFeaturesKHX {
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
  assert_size!(
    types_raw::VkPhysicalDeviceMultiviewFeaturesKHX,
    VkPhysicalDeviceMultiviewFeaturesKHX
  );
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
unsafe impl RawStruct for VkPhysicalDeviceMultiviewPropertiesKHX {
  type Raw = types_raw::VkPhysicalDeviceMultiviewPropertiesKHX;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceMultiviewPropertiesKHX,
    VkPhysicalDeviceMultiviewPropertiesKHX
  );
}

// feature: VK_NV_external_memory_capabilities

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
unsafe impl RawStruct for VkExternalImageFormatPropertiesNV {
  type Raw = types_raw::VkExternalImageFormatPropertiesNV;
}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_image_format_properties_nv() {
  assert_size!(
    types_raw::VkExternalImageFormatPropertiesNV,
    VkExternalImageFormatPropertiesNV
  );
}

// feature: VK_NV_external_memory

/// Specify that an image may be backed by external memory
#[repr(C)]
#[cfg(feature = "VK_NV_external_memory")]
pub struct VkExternalMemoryImageCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_external_memory")]
impl<'a> VkExternalMemoryImageCreateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkExternalMemoryImageCreateInfoNV<'a> {
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
impl<'a> Default for VkExternalMemoryImageCreateInfoNV<'a> {
  fn default() -> VkExternalMemoryImageCreateInfoNV<'a> {
    VkExternalMemoryImageCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'a> RawStruct for VkExternalMemoryImageCreateInfoNV<'a> {
  type Raw = types_raw::VkExternalMemoryImageCreateInfoNV;
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageCreateInfo<'b>> for VkExternalMemoryImageCreateInfoNV<'a> {
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
  assert_size!(
    types_raw::VkExternalMemoryImageCreateInfoNV,
    VkExternalMemoryImageCreateInfoNV
  );
}

/// Specify memory handle types that may be exported
#[repr(C)]
#[cfg(feature = "VK_NV_external_memory")]
pub struct VkExportMemoryAllocateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_external_memory")]
impl<'a> VkExportMemoryAllocateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkExportMemoryAllocateInfoNV<'a> {
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
impl<'a> Default for VkExportMemoryAllocateInfoNV<'a> {
  fn default() -> VkExportMemoryAllocateInfoNV<'a> {
    VkExportMemoryAllocateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'a> RawStruct for VkExportMemoryAllocateInfoNV<'a> {
  type Raw = types_raw::VkExportMemoryAllocateInfoNV;
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkExportMemoryAllocateInfoNV<'a> {
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
  assert_size!(
    types_raw::VkExportMemoryAllocateInfoNV,
    VkExportMemoryAllocateInfoNV
  );
}

// feature: VK_NV_external_memory_win32

/// import Win32 memory created on the same physical device
#[repr(C)]
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportMemoryWin32HandleInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagsNV,
  pub handle: wsi::win32::HANDLE,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkImportMemoryWin32HandleInfoNV<'a> {
  #[inline]
  pub fn new() -> VkImportMemoryWin32HandleInfoNV<'a> {
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
impl<'a> Default for VkImportMemoryWin32HandleInfoNV<'a> {
  fn default() -> VkImportMemoryWin32HandleInfoNV<'a> {
    VkImportMemoryWin32HandleInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkImportMemoryWin32HandleInfoNV<'a> {
  type Raw = types_raw::VkImportMemoryWin32HandleInfoNV;
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkImportMemoryWin32HandleInfoNV<'a> {
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
  assert_size!(
    types_raw::VkImportMemoryWin32HandleInfoNV,
    VkImportMemoryWin32HandleInfoNV
  );
}

/// specify security attributes and access rights for Win32 memory handles
#[repr(C)]
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportMemoryWin32HandleInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkExportMemoryWin32HandleInfoNV<'a> {
  #[inline]
  pub fn new() -> VkExportMemoryWin32HandleInfoNV<'a> {
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
impl<'a> Default for VkExportMemoryWin32HandleInfoNV<'a> {
  fn default() -> VkExportMemoryWin32HandleInfoNV<'a> {
    VkExportMemoryWin32HandleInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkExportMemoryWin32HandleInfoNV<'a> {
  type Raw = types_raw::VkExportMemoryWin32HandleInfoNV;
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkExportMemoryWin32HandleInfoNV<'a> {
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
  assert_size!(
    types_raw::VkExportMemoryWin32HandleInfoNV,
    VkExportMemoryWin32HandleInfoNV
  );
}

// feature: VK_NV_win32_keyed_mutex

/// use Windows keyex mutex mechanism to synchronize work
#[repr(C)]
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  acquireCount: u32,
  pAcquireSyncs: *const types_raw::VkDeviceMemory,
  pAcquireKeys: *const u64,
  pAcquireTimeoutMilliseconds: *const u32,
  releaseCount: u32,
  pReleaseSyncs: *const types_raw::VkDeviceMemory,
  pReleaseKeys: *const u64,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
  #[inline]
  pub fn new() -> VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
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
impl<'a> Default for VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
  fn default() -> VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
    VkWin32KeyedMutexAcquireReleaseInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
  type Raw = types_raw::VkWin32KeyedMutexAcquireReleaseInfoNV;
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSubmitInfo<'b>> for VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
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
  assert_size!(
    types_raw::VkWin32KeyedMutexAcquireReleaseInfoNV,
    VkWin32KeyedMutexAcquireReleaseInfoNV
  );
}

// feature: VK_KHX_device_group_creation

/// Structure specifying physical device group properties
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group_creation")]
pub struct VkPhysicalDeviceGroupPropertiesKHX {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub physicalDeviceCount: u32,
  pub physicalDevices: [VkPhysicalDevice; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize],
  pub subsetAllocation: VkBool32,
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl VkPhysicalDeviceGroupPropertiesKHX {
  #[inline]
  pub fn next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn physical_device_count(&self) -> u32 {
    self.physicalDeviceCount
  }
  #[inline]
  pub fn physical_devices(&self) -> [VkPhysicalDevice; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize] {
    self.physicalDevices
  }
  #[inline]
  pub fn is_subset_allocation(&self) -> bool {
    self.subsetAllocation != 0
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
unsafe impl RawStruct for VkPhysicalDeviceGroupPropertiesKHX {
  type Raw = types_raw::VkPhysicalDeviceGroupPropertiesKHX;
}
#[cfg(feature = "VK_KHX_device_group_creation")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_group_properties_khx() {
  assert_size!(
    types_raw::VkPhysicalDeviceGroupPropertiesKHX,
    VkPhysicalDeviceGroupPropertiesKHX
  );
}

/// Create a logical device from multiple physical devices
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group_creation")]
pub struct VkDeviceGroupDeviceCreateInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  physicalDeviceCount: u32,
  pPhysicalDevices: *const types_raw::VkPhysicalDevice,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl<'a> VkDeviceGroupDeviceCreateInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGroupDeviceCreateInfoKHX<'a> {
    unsafe {
      VkDeviceGroupDeviceCreateInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_DEVICE_CREATE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_physical_devices(mut self, value: &'a [VkPhysicalDevice]) -> Self {
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
impl<'a> Default for VkDeviceGroupDeviceCreateInfoKHX<'a> {
  fn default() -> VkDeviceGroupDeviceCreateInfoKHX<'a> {
    VkDeviceGroupDeviceCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
unsafe impl<'a> RawStruct for VkDeviceGroupDeviceCreateInfoKHX<'a> {
  type Raw = types_raw::VkDeviceGroupDeviceCreateInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group_creation")]
unsafe impl<'b, 'a: 'b> StructExtends<VkDeviceCreateInfo<'b>> for VkDeviceGroupDeviceCreateInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkDeviceGroupDeviceCreateInfoKHX,
    VkDeviceGroupDeviceCreateInfoKHX
  );
}

// feature: VK_KHX_device_group

/// Structure controlling how many instances of memory will be allocated
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkMemoryAllocateFlagsInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkMemoryAllocateFlagsKHX,
  pub deviceMask: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkMemoryAllocateFlagsInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkMemoryAllocateFlagsInfoKHX<'a> {
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
impl<'a> Default for VkMemoryAllocateFlagsInfoKHX<'a> {
  fn default() -> VkMemoryAllocateFlagsInfoKHX<'a> {
    VkMemoryAllocateFlagsInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkMemoryAllocateFlagsInfoKHX<'a> {
  type Raw = types_raw::VkMemoryAllocateFlagsInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkMemoryAllocateFlagsInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkMemoryAllocateFlagsInfoKHX,
    VkMemoryAllocateFlagsInfoKHX
  );
}

/// Set the initial device mask and render areas for a render pass instance
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupRenderPassBeginInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub deviceMask: u32,
  deviceRenderAreaCount: u32,
  pDeviceRenderAreas: *const types_raw::VkRect2D,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkDeviceGroupRenderPassBeginInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGroupRenderPassBeginInfoKHX<'a> {
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
  pub fn set_device_render_areas(mut self, value: &'a [VkRect2D]) -> Self {
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
impl<'a> Default for VkDeviceGroupRenderPassBeginInfoKHX<'a> {
  fn default() -> VkDeviceGroupRenderPassBeginInfoKHX<'a> {
    VkDeviceGroupRenderPassBeginInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkDeviceGroupRenderPassBeginInfoKHX<'a> {
  type Raw = types_raw::VkDeviceGroupRenderPassBeginInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkRenderPassBeginInfo<'b>> for VkDeviceGroupRenderPassBeginInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkDeviceGroupRenderPassBeginInfoKHX,
    VkDeviceGroupRenderPassBeginInfoKHX
  );
}

/// Set the initial device mask for a command buffer
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupCommandBufferBeginInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub deviceMask: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkDeviceGroupCommandBufferBeginInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGroupCommandBufferBeginInfoKHX<'a> {
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
impl<'a> Default for VkDeviceGroupCommandBufferBeginInfoKHX<'a> {
  fn default() -> VkDeviceGroupCommandBufferBeginInfoKHX<'a> {
    VkDeviceGroupCommandBufferBeginInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkDeviceGroupCommandBufferBeginInfoKHX<'a> {
  type Raw = types_raw::VkDeviceGroupCommandBufferBeginInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkCommandBufferBeginInfo<'b>> for VkDeviceGroupCommandBufferBeginInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkDeviceGroupCommandBufferBeginInfoKHX,
    VkDeviceGroupCommandBufferBeginInfoKHX
  );
}

/// Structure indicating which physical devices execute semaphore operations and
/// command buffers
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupSubmitInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreCount: u32,
  pWaitSemaphoreDeviceIndices: *const u32,
  commandBufferCount: u32,
  pCommandBufferDeviceMasks: *const u32,
  signalSemaphoreCount: u32,
  pSignalSemaphoreDeviceIndices: *const u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkDeviceGroupSubmitInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGroupSubmitInfoKHX<'a> {
    unsafe {
      VkDeviceGroupSubmitInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_SUBMIT_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_wait_semaphore_device_indices(mut self, value: &'a [u32]) -> Self {
    self.waitSemaphoreCount = value.len() as u32;
    unsafe {
      self.pWaitSemaphoreDeviceIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_command_buffer_device_masks(mut self, value: &'a [u32]) -> Self {
    self.commandBufferCount = value.len() as u32;
    unsafe {
      self.pCommandBufferDeviceMasks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphore_device_indices(mut self, value: &'a [u32]) -> Self {
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
impl<'a> Default for VkDeviceGroupSubmitInfoKHX<'a> {
  fn default() -> VkDeviceGroupSubmitInfoKHX<'a> {
    VkDeviceGroupSubmitInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkDeviceGroupSubmitInfoKHX<'a> {
  type Raw = types_raw::VkDeviceGroupSubmitInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSubmitInfo<'b>> for VkDeviceGroupSubmitInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkDeviceGroupSubmitInfoKHX,
    VkDeviceGroupSubmitInfoKHX
  );
}

/// Structure indicating which instances are bound
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupBindSparseInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub resourceDeviceIndex: u32,
  pub memoryDeviceIndex: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkDeviceGroupBindSparseInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGroupBindSparseInfoKHX<'a> {
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
impl<'a> Default for VkDeviceGroupBindSparseInfoKHX<'a> {
  fn default() -> VkDeviceGroupBindSparseInfoKHX<'a> {
    VkDeviceGroupBindSparseInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkDeviceGroupBindSparseInfoKHX<'a> {
  type Raw = types_raw::VkDeviceGroupBindSparseInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkBindSparseInfo<'b>> for VkDeviceGroupBindSparseInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkDeviceGroupBindSparseInfoKHX,
    VkDeviceGroupBindSparseInfoKHX
  );
}

/// Structure specifying device within a group to bind to
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  deviceIndexCount: u32,
  pDeviceIndices: *const u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
    unsafe {
      VkBindBufferMemoryDeviceGroupInfoKHX {
        sType: VkStructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_indices(mut self, value: &'a [u32]) -> Self {
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
impl<'a> Default for VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
  fn default() -> VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
    VkBindBufferMemoryDeviceGroupInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
  type Raw = types_raw::VkBindBufferMemoryDeviceGroupInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkBindBufferMemoryInfoKHR<'b>> for VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkBindBufferMemoryDeviceGroupInfoKHX,
    VkBindBufferMemoryDeviceGroupInfoKHX
  );
}

/// Structure specifying device within a group to bind to
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindImageMemoryDeviceGroupInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  deviceIndexCount: u32,
  pDeviceIndices: *const u32,
  SFRRectCount: u32,
  pSFRRects: *const types_raw::VkRect2D,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkBindImageMemoryDeviceGroupInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkBindImageMemoryDeviceGroupInfoKHX<'a> {
    unsafe {
      VkBindImageMemoryDeviceGroupInfoKHX {
        sType: VkStructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_indices(mut self, value: &'a [u32]) -> Self {
    self.deviceIndexCount = value.len() as u32;
    unsafe {
      self.pDeviceIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sfr_rects(mut self, value: &'a [VkRect2D]) -> Self {
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
impl<'a> Default for VkBindImageMemoryDeviceGroupInfoKHX<'a> {
  fn default() -> VkBindImageMemoryDeviceGroupInfoKHX<'a> {
    VkBindImageMemoryDeviceGroupInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkBindImageMemoryDeviceGroupInfoKHX<'a> {
  type Raw = types_raw::VkBindImageMemoryDeviceGroupInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkBindImageMemoryInfoKHR<'b>> for VkBindImageMemoryDeviceGroupInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkBindImageMemoryDeviceGroupInfoKHX,
    VkBindImageMemoryDeviceGroupInfoKHX
  );
}

/// Present capabilities from other physical devices
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupPresentCapabilitiesKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub presentMask: [u32; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize],
  pub modes: VkDeviceGroupPresentModeFlagsKHX,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkDeviceGroupPresentCapabilitiesKHX<'a> {
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
unsafe impl<'a> RawStruct for VkDeviceGroupPresentCapabilitiesKHX<'a> {
  type Raw = types_raw::VkDeviceGroupPresentCapabilitiesKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_present_capabilities_khx() {
  assert_size!(
    types_raw::VkDeviceGroupPresentCapabilitiesKHX,
    VkDeviceGroupPresentCapabilitiesKHX
  );
}

/// Specify that an image will be bound to swapchain memory
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkImageSwapchainCreateInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub swapchain: Option<VkSwapchainKHR>,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkImageSwapchainCreateInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkImageSwapchainCreateInfoKHX<'a> {
    unsafe {
      VkImageSwapchainCreateInfoKHX {
        sType: VkStructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_swapchain(mut self, value: Option<VkSwapchainKHR>) -> Self {
    self.swapchain = value;
    self
  }
  #[inline]
  pub fn swapchain(&self) -> Option<VkSwapchainKHR> {
    self.swapchain
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> Default for VkImageSwapchainCreateInfoKHX<'a> {
  fn default() -> VkImageSwapchainCreateInfoKHX<'a> {
    VkImageSwapchainCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkImageSwapchainCreateInfoKHX<'a> {
  type Raw = types_raw::VkImageSwapchainCreateInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageCreateInfo<'b>> for VkImageSwapchainCreateInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkImageSwapchainCreateInfoKHX,
    VkImageSwapchainCreateInfoKHX
  );
}

/// Structure specifying swapchain image memory to bind to
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindImageMemorySwapchainInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub swapchain: VkSwapchainKHR,
  pub imageIndex: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkBindImageMemorySwapchainInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkBindImageMemorySwapchainInfoKHX<'a> {
    unsafe {
      VkBindImageMemorySwapchainInfoKHX {
        sType: VkStructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_swapchain(mut self, value: VkSwapchainKHR) -> Self {
    self.swapchain = value;
    self
  }
  #[inline]
  pub fn set_image_index(mut self, value: u32) -> Self {
    self.imageIndex = value;
    self
  }
  #[inline]
  pub fn swapchain(&self) -> VkSwapchainKHR {
    self.swapchain
  }
  #[inline]
  pub fn image_index(&self) -> u32 {
    self.imageIndex
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> Default for VkBindImageMemorySwapchainInfoKHX<'a> {
  fn default() -> VkBindImageMemorySwapchainInfoKHX<'a> {
    VkBindImageMemorySwapchainInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkBindImageMemorySwapchainInfoKHX<'a> {
  type Raw = types_raw::VkBindImageMemorySwapchainInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkBindImageMemoryInfoKHR<'b>> for VkBindImageMemorySwapchainInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkBindImageMemorySwapchainInfoKHX,
    VkBindImageMemorySwapchainInfoKHX
  );
}

/// Structure specifying parameters of the acquire
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkAcquireNextImageInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub swapchain: VkSwapchainKHR,
  pub timeout: u64,
  pub semaphore: Option<VkSemaphore>,
  pub fence: Option<VkFence>,
  pub deviceMask: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkAcquireNextImageInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkAcquireNextImageInfoKHX<'a> {
    unsafe {
      VkAcquireNextImageInfoKHX {
        sType: VkStructureType::ACQUIRE_NEXT_IMAGE_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_swapchain(mut self, value: VkSwapchainKHR) -> Self {
    self.swapchain = value;
    self
  }
  #[inline]
  pub fn set_timeout(mut self, value: u64) -> Self {
    self.timeout = value;
    self
  }
  #[inline]
  pub fn set_semaphore(mut self, value: Option<VkSemaphore>) -> Self {
    self.semaphore = value;
    self
  }
  #[inline]
  pub fn set_fence(mut self, value: Option<VkFence>) -> Self {
    self.fence = value;
    self
  }
  #[inline]
  pub fn set_device_mask(mut self, value: u32) -> Self {
    self.deviceMask = value;
    self
  }
  #[inline]
  pub fn swapchain(&self) -> VkSwapchainKHR {
    self.swapchain
  }
  #[inline]
  pub fn timeout(&self) -> u64 {
    self.timeout
  }
  #[inline]
  pub fn semaphore(&self) -> Option<VkSemaphore> {
    self.semaphore
  }
  #[inline]
  pub fn fence(&self) -> Option<VkFence> {
    self.fence
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
impl<'a> Default for VkAcquireNextImageInfoKHX<'a> {
  fn default() -> VkAcquireNextImageInfoKHX<'a> {
    VkAcquireNextImageInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkAcquireNextImageInfoKHX<'a> {
  type Raw = types_raw::VkAcquireNextImageInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_acquire_next_image_info_khx() {
  assert_size!(
    types_raw::VkAcquireNextImageInfoKHX,
    VkAcquireNextImageInfoKHX
  );
}

/// Mode and mask controlling which physical devices\' images are presented
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupPresentInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchainCount: u32,
  pDeviceMasks: *const u32,
  pub mode: VkDeviceGroupPresentModeFlagBitsKHX,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkDeviceGroupPresentInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGroupPresentInfoKHX<'a> {
    unsafe {
      VkDeviceGroupPresentInfoKHX {
        sType: VkStructureType::DEVICE_GROUP_PRESENT_INFO_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_masks(mut self, value: &'a [u32]) -> Self {
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
impl<'a> Default for VkDeviceGroupPresentInfoKHX<'a> {
  fn default() -> VkDeviceGroupPresentInfoKHX<'a> {
    VkDeviceGroupPresentInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkDeviceGroupPresentInfoKHX<'a> {
  type Raw = types_raw::VkDeviceGroupPresentInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPresentInfoKHR<'b>> for VkDeviceGroupPresentInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkDeviceGroupPresentInfoKHX,
    VkDeviceGroupPresentInfoKHX
  );
}

/// Structure specifying parameters of a newly created swapchain object
#[repr(C)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupSwapchainCreateInfoKHX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub modes: VkDeviceGroupPresentModeFlagsKHX,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> VkDeviceGroupSwapchainCreateInfoKHX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGroupSwapchainCreateInfoKHX<'a> {
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
impl<'a> Default for VkDeviceGroupSwapchainCreateInfoKHX<'a> {
  fn default() -> VkDeviceGroupSwapchainCreateInfoKHX<'a> {
    VkDeviceGroupSwapchainCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'a> RawStruct for VkDeviceGroupSwapchainCreateInfoKHX<'a> {
  type Raw = types_raw::VkDeviceGroupSwapchainCreateInfoKHX;
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSwapchainCreateInfoKHR<'b>> for VkDeviceGroupSwapchainCreateInfoKHX<'a> {
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
  assert_size!(
    types_raw::VkDeviceGroupSwapchainCreateInfoKHX,
    VkDeviceGroupSwapchainCreateInfoKHX
  );
}

// feature: VK_EXT_validation_flags

/// Specify validation checks to disable for a Vulkan instance
#[repr(C)]
#[cfg(feature = "VK_EXT_validation_flags")]
pub struct VkValidationFlagsEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  disabledValidationCheckCount: u32,
  pDisabledValidationChecks: *mut VkValidationCheckEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_validation_flags")]
impl<'a> VkValidationFlagsEXT<'a> {
  #[inline]
  pub fn new() -> VkValidationFlagsEXT<'a> {
    unsafe {
      VkValidationFlagsEXT {
        sType: VkStructureType::VALIDATION_FLAGS_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_disabled_validation_checks(mut self, value: &'a mut [VkValidationCheckEXT]) -> Self {
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
impl<'a> Default for VkValidationFlagsEXT<'a> {
  fn default() -> VkValidationFlagsEXT<'a> {
    VkValidationFlagsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_flags")]
unsafe impl<'a> RawStruct for VkValidationFlagsEXT<'a> {
  type Raw = types_raw::VkValidationFlagsEXT;
}
#[cfg(feature = "VK_EXT_validation_flags")]
unsafe impl<'b, 'a: 'b> StructExtends<VkInstanceCreateInfo<'b>> for VkValidationFlagsEXT<'a> {
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
  assert_size!(types_raw::VkValidationFlagsEXT, VkValidationFlagsEXT);
}

// feature: VK_NN_vi_surface

/// Structure specifying parameters of a newly created VI surface object
#[repr(C)]
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub struct VkViSurfaceCreateInfoNN<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkViSurfaceCreateFlagsNN,
  window: *mut c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
impl<'a> VkViSurfaceCreateInfoNN<'a> {
  #[inline]
  pub fn new() -> VkViSurfaceCreateInfoNN<'a> {
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
impl<'a> Default for VkViSurfaceCreateInfoNN<'a> {
  fn default() -> VkViSurfaceCreateInfoNN<'a> {
    VkViSurfaceCreateInfoNN::new()
  }
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
unsafe impl<'a> RawStruct for VkViSurfaceCreateInfoNN<'a> {
  type Raw = types_raw::VkViSurfaceCreateInfoNN;
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_vi_surface_create_info_nn() {
  assert_size!(types_raw::VkViSurfaceCreateInfoNN, VkViSurfaceCreateInfoNN);
}

// feature: VK_KHR_external_memory_capabilities

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
unsafe impl RawStruct for VkExternalMemoryPropertiesKHR {
  type Raw = types_raw::VkExternalMemoryPropertiesKHR;
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_memory_properties_khr() {
  assert_size!(
    types_raw::VkExternalMemoryPropertiesKHR,
    VkExternalMemoryPropertiesKHR
  );
}

/// Structure specifying external image creation parameters
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl<'a> VkPhysicalDeviceExternalImageFormatInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalImageFormatInfoKHR<'a> {
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
impl<'a> Default for VkPhysicalDeviceExternalImageFormatInfoKHR<'a> {
  fn default() -> VkPhysicalDeviceExternalImageFormatInfoKHR<'a> {
    VkPhysicalDeviceExternalImageFormatInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl<'a> RawStruct for VkPhysicalDeviceExternalImageFormatInfoKHR<'a> {
  type Raw = types_raw::VkPhysicalDeviceExternalImageFormatInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPhysicalDeviceImageFormatInfo2KHR<'b>>
  for VkPhysicalDeviceExternalImageFormatInfoKHR<'a>
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
  assert_size!(
    types_raw::VkPhysicalDeviceExternalImageFormatInfoKHR,
    VkPhysicalDeviceExternalImageFormatInfoKHR
  );
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
unsafe impl RawStruct for VkExternalImageFormatPropertiesKHR {
  type Raw = types_raw::VkExternalImageFormatPropertiesKHR;
}
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
  assert_size!(
    types_raw::VkExternalImageFormatPropertiesKHR,
    VkExternalImageFormatPropertiesKHR
  );
}

/// Structure specifying buffer creation parameters
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceExternalBufferInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkBufferCreateFlags,
  pub usage: VkBufferUsageFlags,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl<'a> VkPhysicalDeviceExternalBufferInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalBufferInfoKHR<'a> {
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
impl<'a> Default for VkPhysicalDeviceExternalBufferInfoKHR<'a> {
  fn default() -> VkPhysicalDeviceExternalBufferInfoKHR<'a> {
    VkPhysicalDeviceExternalBufferInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl<'a> RawStruct for VkPhysicalDeviceExternalBufferInfoKHR<'a> {
  type Raw = types_raw::VkPhysicalDeviceExternalBufferInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_buffer_info_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceExternalBufferInfoKHR,
    VkPhysicalDeviceExternalBufferInfoKHR
  );
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
unsafe impl RawStruct for VkExternalBufferPropertiesKHR {
  type Raw = types_raw::VkExternalBufferPropertiesKHR;
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_buffer_properties_khr() {
  assert_size!(
    types_raw::VkExternalBufferPropertiesKHR,
    VkExternalBufferPropertiesKHR
  );
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
  pub deviceLUIDValid: VkBool32,
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
unsafe impl RawStruct for VkPhysicalDeviceIDPropertiesKHR {
  type Raw = types_raw::VkPhysicalDeviceIDPropertiesKHR;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceIDPropertiesKHR,
    VkPhysicalDeviceIDPropertiesKHR
  );
}

// feature: VK_KHR_external_memory

/// Specify that an image may be backed by external memory
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExternalMemoryImageCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'a> VkExternalMemoryImageCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkExternalMemoryImageCreateInfoKHR<'a> {
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
impl<'a> Default for VkExternalMemoryImageCreateInfoKHR<'a> {
  fn default() -> VkExternalMemoryImageCreateInfoKHR<'a> {
    VkExternalMemoryImageCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'a> RawStruct for VkExternalMemoryImageCreateInfoKHR<'a> {
  type Raw = types_raw::VkExternalMemoryImageCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageCreateInfo<'b>> for VkExternalMemoryImageCreateInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkExternalMemoryImageCreateInfoKHR,
    VkExternalMemoryImageCreateInfoKHR
  );
}

/// Specify that a buffer may be backed by external memory
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExternalMemoryBufferCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'a> VkExternalMemoryBufferCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkExternalMemoryBufferCreateInfoKHR<'a> {
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
impl<'a> Default for VkExternalMemoryBufferCreateInfoKHR<'a> {
  fn default() -> VkExternalMemoryBufferCreateInfoKHR<'a> {
    VkExternalMemoryBufferCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'a> RawStruct for VkExternalMemoryBufferCreateInfoKHR<'a> {
  type Raw = types_raw::VkExternalMemoryBufferCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'b, 'a: 'b> StructExtends<VkBufferCreateInfo<'b>> for VkExternalMemoryBufferCreateInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkExternalMemoryBufferCreateInfoKHR,
    VkExternalMemoryBufferCreateInfoKHR
  );
}

/// Specify exportable handle types for a device memory object
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExportMemoryAllocateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl<'a> VkExportMemoryAllocateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkExportMemoryAllocateInfoKHR<'a> {
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
impl<'a> Default for VkExportMemoryAllocateInfoKHR<'a> {
  fn default() -> VkExportMemoryAllocateInfoKHR<'a> {
    VkExportMemoryAllocateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'a> RawStruct for VkExportMemoryAllocateInfoKHR<'a> {
  type Raw = types_raw::VkExportMemoryAllocateInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkExportMemoryAllocateInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkExportMemoryAllocateInfoKHR,
    VkExportMemoryAllocateInfoKHR
  );
}

// feature: VK_KHR_external_memory_win32

/// import Win32 memory created on the same physical device
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportMemoryWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkImportMemoryWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImportMemoryWin32HandleInfoKHR<'a> {
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
impl<'a> Default for VkImportMemoryWin32HandleInfoKHR<'a> {
  fn default() -> VkImportMemoryWin32HandleInfoKHR<'a> {
    VkImportMemoryWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkImportMemoryWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkImportMemoryWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkImportMemoryWin32HandleInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkImportMemoryWin32HandleInfoKHR,
    VkImportMemoryWin32HandleInfoKHR
  );
}

/// Structure specifying additional attributes of Windows handles exported from a
/// memory
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportMemoryWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkExportMemoryWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkExportMemoryWin32HandleInfoKHR<'a> {
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
impl<'a> Default for VkExportMemoryWin32HandleInfoKHR<'a> {
  fn default() -> VkExportMemoryWin32HandleInfoKHR<'a> {
    VkExportMemoryWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkExportMemoryWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkExportMemoryWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkExportMemoryWin32HandleInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkExportMemoryWin32HandleInfoKHR,
    VkExportMemoryWin32HandleInfoKHR
  );
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
unsafe impl RawStruct for VkMemoryWin32HandlePropertiesKHR {
  type Raw = types_raw::VkMemoryWin32HandlePropertiesKHR;
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_win32_handle_properties_khr() {
  assert_size!(
    types_raw::VkMemoryWin32HandlePropertiesKHR,
    VkMemoryWin32HandlePropertiesKHR
  );
}

/// Structure describing a Win32 handle semaphore export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkMemoryGetWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkMemoryGetWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkMemoryGetWin32HandleInfoKHR<'a> {
    unsafe {
      VkMemoryGetWin32HandleInfoKHR {
        sType: VkStructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory) -> Self {
    self.memory = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn memory(&self) -> VkDeviceMemory {
    self.memory
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
impl<'a> Default for VkMemoryGetWin32HandleInfoKHR<'a> {
  fn default() -> VkMemoryGetWin32HandleInfoKHR<'a> {
    VkMemoryGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkMemoryGetWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkMemoryGetWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_get_win32_handle_info_khr() {
  assert_size!(
    types_raw::VkMemoryGetWin32HandleInfoKHR,
    VkMemoryGetWin32HandleInfoKHR
  );
}

// feature: VK_KHR_external_memory_fd

/// import memory created on the same physical device from a file descriptor
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkImportMemoryFdInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub fd: c_int,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl<'a> VkImportMemoryFdInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImportMemoryFdInfoKHR<'a> {
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
impl<'a> Default for VkImportMemoryFdInfoKHR<'a> {
  fn default() -> VkImportMemoryFdInfoKHR<'a> {
    VkImportMemoryFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl<'a> RawStruct for VkImportMemoryFdInfoKHR<'a> {
  type Raw = types_raw::VkImportMemoryFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkImportMemoryFdInfoKHR<'a> {
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
  assert_size!(types_raw::VkImportMemoryFdInfoKHR, VkImportMemoryFdInfoKHR);
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
unsafe impl RawStruct for VkMemoryFdPropertiesKHR {
  type Raw = types_raw::VkMemoryFdPropertiesKHR;
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_fd_properties_khr() {
  assert_size!(types_raw::VkMemoryFdPropertiesKHR, VkMemoryFdPropertiesKHR);
}

/// Structure describing a POSIX FD semaphore export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkMemoryGetFdInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl<'a> VkMemoryGetFdInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkMemoryGetFdInfoKHR<'a> {
    unsafe {
      VkMemoryGetFdInfoKHR {
        sType: VkStructureType::MEMORY_GET_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory) -> Self {
    self.memory = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn memory(&self) -> VkDeviceMemory {
    self.memory
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
impl<'a> Default for VkMemoryGetFdInfoKHR<'a> {
  fn default() -> VkMemoryGetFdInfoKHR<'a> {
    VkMemoryGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl<'a> RawStruct for VkMemoryGetFdInfoKHR<'a> {
  type Raw = types_raw::VkMemoryGetFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_get_fd_info_khr() {
  assert_size!(types_raw::VkMemoryGetFdInfoKHR, VkMemoryGetFdInfoKHR);
}

// feature: VK_KHR_win32_keyed_mutex

/// Use the Windows keyed mutex mechanism to synchronize work
#[repr(C)]
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  acquireCount: u32,
  pAcquireSyncs: *const types_raw::VkDeviceMemory,
  pAcquireKeys: *const u64,
  pAcquireTimeouts: *const u32,
  releaseCount: u32,
  pReleaseSyncs: *const types_raw::VkDeviceMemory,
  pReleaseKeys: *const u64,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
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
impl<'a> Default for VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
  fn default() -> VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
    VkWin32KeyedMutexAcquireReleaseInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
  type Raw = types_raw::VkWin32KeyedMutexAcquireReleaseInfoKHR;
}
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSubmitInfo<'b>> for VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkWin32KeyedMutexAcquireReleaseInfoKHR,
    VkWin32KeyedMutexAcquireReleaseInfoKHR
  );
}

// feature: VK_KHR_external_semaphore_capabilities

/// Structure specifying semaphore creation parameters.
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl<'a> VkPhysicalDeviceExternalSemaphoreInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalSemaphoreInfoKHR<'a> {
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
impl<'a> Default for VkPhysicalDeviceExternalSemaphoreInfoKHR<'a> {
  fn default() -> VkPhysicalDeviceExternalSemaphoreInfoKHR<'a> {
    VkPhysicalDeviceExternalSemaphoreInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
unsafe impl<'a> RawStruct for VkPhysicalDeviceExternalSemaphoreInfoKHR<'a> {
  type Raw = types_raw::VkPhysicalDeviceExternalSemaphoreInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_semaphore_info_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceExternalSemaphoreInfoKHR,
    VkPhysicalDeviceExternalSemaphoreInfoKHR
  );
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
unsafe impl RawStruct for VkExternalSemaphorePropertiesKHR {
  type Raw = types_raw::VkExternalSemaphorePropertiesKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_semaphore_properties_khr() {
  assert_size!(
    types_raw::VkExternalSemaphorePropertiesKHR,
    VkExternalSemaphorePropertiesKHR
  );
}

// feature: VK_KHR_external_semaphore

/// Structure specifying handle types that can be exported from a semaphore
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore")]
pub struct VkExportSemaphoreCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_semaphore")]
impl<'a> VkExportSemaphoreCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkExportSemaphoreCreateInfoKHR<'a> {
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
impl<'a> Default for VkExportSemaphoreCreateInfoKHR<'a> {
  fn default() -> VkExportSemaphoreCreateInfoKHR<'a> {
    VkExportSemaphoreCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
unsafe impl<'a> RawStruct for VkExportSemaphoreCreateInfoKHR<'a> {
  type Raw = types_raw::VkExportSemaphoreCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSemaphoreCreateInfo<'b>> for VkExportSemaphoreCreateInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkExportSemaphoreCreateInfoKHR,
    VkExportSemaphoreCreateInfoKHR
  );
}

// feature: VK_KHR_external_semaphore_win32

/// Structure specifying Windows handle to import to a semaphore
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportSemaphoreWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub semaphore: VkSemaphore,
  pub flags: VkSemaphoreImportFlagsKHR,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkImportSemaphoreWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImportSemaphoreWin32HandleInfoKHR<'a> {
    unsafe {
      VkImportSemaphoreWin32HandleInfoKHR {
        sType: VkStructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_semaphore(mut self, value: VkSemaphore) -> Self {
    self.semaphore = value;
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
  pub fn semaphore(&self) -> VkSemaphore {
    self.semaphore
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
impl<'a> Default for VkImportSemaphoreWin32HandleInfoKHR<'a> {
  fn default() -> VkImportSemaphoreWin32HandleInfoKHR<'a> {
    VkImportSemaphoreWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkImportSemaphoreWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkImportSemaphoreWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_semaphore_win32_handle_info_khr() {
  assert_size!(
    types_raw::VkImportSemaphoreWin32HandleInfoKHR,
    VkImportSemaphoreWin32HandleInfoKHR
  );
}

/// Structure specifying additional attributes of Windows handles exported from a
/// semaphore
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportSemaphoreWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkExportSemaphoreWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkExportSemaphoreWin32HandleInfoKHR<'a> {
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
impl<'a> Default for VkExportSemaphoreWin32HandleInfoKHR<'a> {
  fn default() -> VkExportSemaphoreWin32HandleInfoKHR<'a> {
    VkExportSemaphoreWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkExportSemaphoreWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkExportSemaphoreWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSemaphoreCreateInfo<'b>> for VkExportSemaphoreWin32HandleInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkExportSemaphoreWin32HandleInfoKHR,
    VkExportSemaphoreWin32HandleInfoKHR
  );
}

/// Structure specifying values for Direct3D 12 fence-backed semaphores
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkD3D12FenceSubmitInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  waitSemaphoreValuesCount: u32,
  pWaitSemaphoreValues: *const u64,
  signalSemaphoreValuesCount: u32,
  pSignalSemaphoreValues: *const u64,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkD3D12FenceSubmitInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkD3D12FenceSubmitInfoKHR<'a> {
    unsafe {
      VkD3D12FenceSubmitInfoKHR {
        sType: VkStructureType::D3D12_FENCE_SUBMIT_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_wait_semaphore_values(mut self, value: &'a [u64]) -> Self {
    self.waitSemaphoreValuesCount = value.len() as u32;
    unsafe {
      self.pWaitSemaphoreValues = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphore_values(mut self, value: &'a [u64]) -> Self {
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
impl<'a> Default for VkD3D12FenceSubmitInfoKHR<'a> {
  fn default() -> VkD3D12FenceSubmitInfoKHR<'a> {
    VkD3D12FenceSubmitInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkD3D12FenceSubmitInfoKHR<'a> {
  type Raw = types_raw::VkD3D12FenceSubmitInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSubmitInfo<'b>> for VkD3D12FenceSubmitInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkD3D12FenceSubmitInfoKHR,
    VkD3D12FenceSubmitInfoKHR
  );
}

/// Structure describing a Win32 handle semaphore export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkSemaphoreGetWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkSemaphoreGetWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkSemaphoreGetWin32HandleInfoKHR<'a> {
    unsafe {
      VkSemaphoreGetWin32HandleInfoKHR {
        sType: VkStructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_semaphore(mut self, value: VkSemaphore) -> Self {
    self.semaphore = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalSemaphoreHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn semaphore(&self) -> VkSemaphore {
    self.semaphore
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
impl<'a> Default for VkSemaphoreGetWin32HandleInfoKHR<'a> {
  fn default() -> VkSemaphoreGetWin32HandleInfoKHR<'a> {
    VkSemaphoreGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkSemaphoreGetWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkSemaphoreGetWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_get_win32_handle_info_khr() {
  assert_size!(
    types_raw::VkSemaphoreGetWin32HandleInfoKHR,
    VkSemaphoreGetWin32HandleInfoKHR
  );
}

// feature: VK_KHR_external_semaphore_fd

/// Structure specifying POSIX file descriptor to import to a semaphore
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub struct VkImportSemaphoreFdInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub semaphore: VkSemaphore,
  pub flags: VkSemaphoreImportFlagsKHR,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  pub fd: c_int,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl<'a> VkImportSemaphoreFdInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImportSemaphoreFdInfoKHR<'a> {
    unsafe {
      VkImportSemaphoreFdInfoKHR {
        sType: VkStructureType::IMPORT_SEMAPHORE_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_semaphore(mut self, value: VkSemaphore) -> Self {
    self.semaphore = value;
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
  pub fn semaphore(&self) -> VkSemaphore {
    self.semaphore
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
impl<'a> Default for VkImportSemaphoreFdInfoKHR<'a> {
  fn default() -> VkImportSemaphoreFdInfoKHR<'a> {
    VkImportSemaphoreFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
unsafe impl<'a> RawStruct for VkImportSemaphoreFdInfoKHR<'a> {
  type Raw = types_raw::VkImportSemaphoreFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_semaphore_fd_info_khr() {
  assert_size!(
    types_raw::VkImportSemaphoreFdInfoKHR,
    VkImportSemaphoreFdInfoKHR
  );
}

/// Structure describing a POSIX FD semaphore export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub struct VkSemaphoreGetFdInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl<'a> VkSemaphoreGetFdInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkSemaphoreGetFdInfoKHR<'a> {
    unsafe {
      VkSemaphoreGetFdInfoKHR {
        sType: VkStructureType::SEMAPHORE_GET_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_semaphore(mut self, value: VkSemaphore) -> Self {
    self.semaphore = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalSemaphoreHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn semaphore(&self) -> VkSemaphore {
    self.semaphore
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
impl<'a> Default for VkSemaphoreGetFdInfoKHR<'a> {
  fn default() -> VkSemaphoreGetFdInfoKHR<'a> {
    VkSemaphoreGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
unsafe impl<'a> RawStruct for VkSemaphoreGetFdInfoKHR<'a> {
  type Raw = types_raw::VkSemaphoreGetFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_get_fd_info_khr() {
  assert_size!(types_raw::VkSemaphoreGetFdInfoKHR, VkSemaphoreGetFdInfoKHR);
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
unsafe impl RawStruct for VkPhysicalDevicePushDescriptorPropertiesKHR {
  type Raw = types_raw::VkPhysicalDevicePushDescriptorPropertiesKHR;
}
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
  assert_size!(
    types_raw::VkPhysicalDevicePushDescriptorPropertiesKHR,
    VkPhysicalDevicePushDescriptorPropertiesKHR
  );
}

// feature: VK_KHR_16bit_storage
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_16bit_storage")]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub storageBuffer16BitAccess: VkBool32,
  pub uniformAndStorageBuffer16BitAccess: VkBool32,
  pub storagePushConstant16: VkBool32,
  pub storageInputOutput16: VkBool32,
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.storageBuffer16BitAccess = value;
    self
  }
  #[inline]
  pub fn set_uniform_and_storage_buffer16_bit_access(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.uniformAndStorageBuffer16BitAccess = value;
    self
  }
  #[inline]
  pub fn set_storage_push_constant16(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.storagePushConstant16 = value;
    self
  }
  #[inline]
  pub fn set_storage_input_output16(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.storageInputOutput16 = value;
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
unsafe impl RawStruct for VkPhysicalDevice16BitStorageFeaturesKHR {
  type Raw = types_raw::VkPhysicalDevice16BitStorageFeaturesKHR;
}
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
unsafe impl<'b> StructExtends<VkDeviceCreateInfo<'b>> for VkPhysicalDevice16BitStorageFeaturesKHR {
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
  assert_size!(
    types_raw::VkPhysicalDevice16BitStorageFeaturesKHR,
    VkPhysicalDevice16BitStorageFeaturesKHR
  );
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
unsafe impl RawStruct for VkRectLayerKHR {
  type Raw = types_raw::VkRectLayerKHR;
}
#[cfg(feature = "VK_KHR_incremental_present")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_rect_layer_khr() {
  assert_size!(types_raw::VkRectLayerKHR, VkRectLayerKHR);
}

/// Structure containing rectangular region changed by vkQueuePresentKHR for a given
/// VkImage
#[repr(C)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkPresentRegionKHR<'a> {
  rectangleCount: u32,
  pRectangles: *const types_raw::VkRectLayerKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'a> VkPresentRegionKHR<'a> {
  #[inline]
  pub fn new() -> VkPresentRegionKHR<'a> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_rectangles(mut self, value: &'a [VkRectLayerKHR]) -> Self {
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
impl<'a> Default for VkPresentRegionKHR<'a> {
  fn default() -> VkPresentRegionKHR<'a> {
    VkPresentRegionKHR::new()
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
unsafe impl<'a> RawStruct for VkPresentRegionKHR<'a> {
  type Raw = types_raw::VkPresentRegionKHR;
}
#[cfg(feature = "VK_KHR_incremental_present")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_region_khr() {
  assert_size!(types_raw::VkPresentRegionKHR, VkPresentRegionKHR);
}

/// Structure hint of rectangular regions changed by vkQueuePresentKHR
#[repr(C)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkPresentRegionsKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchainCount: u32,
  pRegions: *const types_raw::VkPresentRegionKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'a> VkPresentRegionsKHR<'a> {
  #[inline]
  pub fn new() -> VkPresentRegionsKHR<'a> {
    unsafe {
      VkPresentRegionsKHR {
        sType: VkStructureType::PRESENT_REGIONS_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_regions(mut self, value: &'a [VkPresentRegionKHR<'a>]) -> Self {
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
impl<'a> Default for VkPresentRegionsKHR<'a> {
  fn default() -> VkPresentRegionsKHR<'a> {
    VkPresentRegionsKHR::new()
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
unsafe impl<'a> RawStruct for VkPresentRegionsKHR<'a> {
  type Raw = types_raw::VkPresentRegionsKHR;
}
#[cfg(feature = "VK_KHR_incremental_present")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPresentInfoKHR<'b>> for VkPresentRegionsKHR<'a> {
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
  assert_size!(types_raw::VkPresentRegionsKHR, VkPresentRegionsKHR);
}

// feature: VK_KHR_descriptor_update_template
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDescriptorUpdateTemplateKHR__ {}

/// Opaque handle to a descriptor update template
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type VkDescriptorUpdateTemplateKHR = VkNonDispatchableHandle<VkDescriptorUpdateTemplateKHR__>;

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
unsafe impl RawStruct for VkDescriptorUpdateTemplateEntryKHR {
  type Raw = types_raw::VkDescriptorUpdateTemplateEntryKHR;
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_update_template_entry_khr() {
  assert_size!(
    types_raw::VkDescriptorUpdateTemplateEntryKHR,
    VkDescriptorUpdateTemplateEntryKHR
  );
}

/// Structure specifying parameters of a newly created descriptor update template
#[repr(C)]
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
  descriptorUpdateEntryCount: u32,
  pDescriptorUpdateEntries: *const types_raw::VkDescriptorUpdateTemplateEntryKHR,
  pub templateType: VkDescriptorUpdateTemplateTypeKHR,
  pub descriptorSetLayout: Option<VkDescriptorSetLayout>,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub pipelineLayout: Option<VkPipelineLayout>,
  pub set: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<'a> VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
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
  pub fn set_descriptor_update_entries(mut self, value: &'a [VkDescriptorUpdateTemplateEntryKHR]) -> Self {
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
  pub fn set_descriptor_set_layout(mut self, value: Option<VkDescriptorSetLayout>) -> Self {
    self.descriptorSetLayout = value;
    self
  }
  #[inline]
  pub fn set_pipeline_bind_point(mut self, value: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = value;
    self
  }
  #[inline]
  pub fn set_pipeline_layout(mut self, value: Option<VkPipelineLayout>) -> Self {
    self.pipelineLayout = value;
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
  pub fn descriptor_set_layout(&self) -> Option<VkDescriptorSetLayout> {
    self.descriptorSetLayout
  }
  #[inline]
  pub fn pipeline_bind_point(&self) -> VkPipelineBindPoint {
    self.pipelineBindPoint
  }
  #[inline]
  pub fn pipeline_layout(&self) -> Option<VkPipelineLayout> {
    self.pipelineLayout
  }
  #[inline]
  pub fn set(&self) -> u32 {
    self.set
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<'a> Default for VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
  fn default() -> VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
    VkDescriptorUpdateTemplateCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
unsafe impl<'a> RawStruct for VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
  type Raw = types_raw::VkDescriptorUpdateTemplateCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_update_template_create_info_khr() {
  assert_size!(
    types_raw::VkDescriptorUpdateTemplateCreateInfoKHR,
    VkDescriptorUpdateTemplateCreateInfoKHR
  );
}

// feature: VK_NVX_device_generated_commands
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkObjectTableNVX__ {}

/// Opaque handle to an object table
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkObjectTableNVX = VkNonDispatchableHandle<VkObjectTableNVX__>;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkIndirectCommandsLayoutNVX__ {}

/// Opaque handle to an indirect commands layout object
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkIndirectCommandsLayoutNVX = VkNonDispatchableHandle<VkIndirectCommandsLayoutNVX__>;

/// Structure specifying physical device support
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkDeviceGeneratedCommandsFeaturesNVX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub computeBindingPointSupport: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> VkDeviceGeneratedCommandsFeaturesNVX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGeneratedCommandsFeaturesNVX<'a> {
    unsafe {
      VkDeviceGeneratedCommandsFeaturesNVX {
        sType: VkStructureType::DEVICE_GENERATED_COMMANDS_FEATURES_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_compute_binding_point_support(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.computeBindingPointSupport = value;
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
impl<'a> Default for VkDeviceGeneratedCommandsFeaturesNVX<'a> {
  fn default() -> VkDeviceGeneratedCommandsFeaturesNVX<'a> {
    VkDeviceGeneratedCommandsFeaturesNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'a> RawStruct for VkDeviceGeneratedCommandsFeaturesNVX<'a> {
  type Raw = types_raw::VkDeviceGeneratedCommandsFeaturesNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_generated_commands_features_nvx() {
  assert_size!(
    types_raw::VkDeviceGeneratedCommandsFeaturesNVX,
    VkDeviceGeneratedCommandsFeaturesNVX
  );
}

/// Structure specifying physical device limits
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkDeviceGeneratedCommandsLimitsNVX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub maxIndirectCommandsLayoutTokenCount: u32,
  pub maxObjectEntryCounts: u32,
  pub minSequenceCountBufferOffsetAlignment: u32,
  pub minSequenceIndexBufferOffsetAlignment: u32,
  pub minCommandsTokenBufferOffsetAlignment: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> VkDeviceGeneratedCommandsLimitsNVX<'a> {
  #[inline]
  pub fn new() -> VkDeviceGeneratedCommandsLimitsNVX<'a> {
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
impl<'a> Default for VkDeviceGeneratedCommandsLimitsNVX<'a> {
  fn default() -> VkDeviceGeneratedCommandsLimitsNVX<'a> {
    VkDeviceGeneratedCommandsLimitsNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'a> RawStruct for VkDeviceGeneratedCommandsLimitsNVX<'a> {
  type Raw = types_raw::VkDeviceGeneratedCommandsLimitsNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_generated_commands_limits_nvx() {
  assert_size!(
    types_raw::VkDeviceGeneratedCommandsLimitsNVX,
    VkDeviceGeneratedCommandsLimitsNVX
  );
}

/// Structure specifying parameters for the reservation of command buffer space
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsTokenNVX {
  pub tokenType: VkIndirectCommandsTokenTypeNVX,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkIndirectCommandsTokenNVX {
  #[inline]
  pub fn new() -> VkIndirectCommandsTokenNVX {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_token_type(mut self, value: VkIndirectCommandsTokenTypeNVX) -> Self {
    self.tokenType = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
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
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn offset(&self) -> VkDeviceSize {
    self.offset
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkIndirectCommandsTokenNVX {
  fn default() -> VkIndirectCommandsTokenNVX {
    VkIndirectCommandsTokenNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkIndirectCommandsTokenNVX {
  type Raw = types_raw::VkIndirectCommandsTokenNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_indirect_commands_token_nvx() {
  assert_size!(
    types_raw::VkIndirectCommandsTokenNVX,
    VkIndirectCommandsTokenNVX
  );
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
unsafe impl RawStruct for VkIndirectCommandsLayoutTokenNVX {
  type Raw = types_raw::VkIndirectCommandsLayoutTokenNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_indirect_commands_layout_token_nvx() {
  assert_size!(
    types_raw::VkIndirectCommandsLayoutTokenNVX,
    VkIndirectCommandsLayoutTokenNVX
  );
}

/// Structure specifying the parameters of a newly created indirect commands layout
/// object
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsLayoutCreateInfoNVX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub flags: VkIndirectCommandsLayoutUsageFlagsNVX,
  tokenCount: u32,
  pTokens: *const types_raw::VkIndirectCommandsLayoutTokenNVX,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> VkIndirectCommandsLayoutCreateInfoNVX<'a> {
  #[inline]
  pub fn new() -> VkIndirectCommandsLayoutCreateInfoNVX<'a> {
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
  pub fn set_tokens(mut self, value: &'a [VkIndirectCommandsLayoutTokenNVX]) -> Self {
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
impl<'a> Default for VkIndirectCommandsLayoutCreateInfoNVX<'a> {
  fn default() -> VkIndirectCommandsLayoutCreateInfoNVX<'a> {
    VkIndirectCommandsLayoutCreateInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'a> RawStruct for VkIndirectCommandsLayoutCreateInfoNVX<'a> {
  type Raw = types_raw::VkIndirectCommandsLayoutCreateInfoNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_indirect_commands_layout_create_info_nvx() {
  assert_size!(
    types_raw::VkIndirectCommandsLayoutCreateInfoNVX,
    VkIndirectCommandsLayoutCreateInfoNVX
  );
}

/// Structure specifying parameters for the generation of commands
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkCmdProcessCommandsInfoNVX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub objectTable: VkObjectTableNVX,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
  indirectCommandsTokenCount: u32,
  pIndirectCommandsTokens: *const types_raw::VkIndirectCommandsTokenNVX,
  pub maxSequencesCount: u32,
  pub targetCommandBuffer: Option<VkCommandBuffer>,
  pub sequencesCountBuffer: Option<VkBuffer>,
  pub sequencesCountOffset: VkDeviceSize,
  pub sequencesIndexBuffer: Option<VkBuffer>,
  pub sequencesIndexOffset: VkDeviceSize,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> VkCmdProcessCommandsInfoNVX<'a> {
  #[inline]
  pub fn new() -> VkCmdProcessCommandsInfoNVX<'a> {
    unsafe {
      VkCmdProcessCommandsInfoNVX {
        sType: VkStructureType::CMD_PROCESS_COMMANDS_INFO_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_object_table(mut self, value: VkObjectTableNVX) -> Self {
    self.objectTable = value;
    self
  }
  #[inline]
  pub fn set_indirect_commands_layout(mut self, value: VkIndirectCommandsLayoutNVX) -> Self {
    self.indirectCommandsLayout = value;
    self
  }
  #[inline]
  pub fn set_indirect_commands_tokens(mut self, value: &'a [VkIndirectCommandsTokenNVX]) -> Self {
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
  pub fn set_target_command_buffer(mut self, value: Option<VkCommandBuffer>) -> Self {
    self.targetCommandBuffer = value;
    self
  }
  #[inline]
  pub fn set_sequences_count_buffer(mut self, value: Option<VkBuffer>) -> Self {
    self.sequencesCountBuffer = value;
    self
  }
  #[inline]
  pub fn set_sequences_count_offset(mut self, value: VkDeviceSize) -> Self {
    self.sequencesCountOffset = value;
    self
  }
  #[inline]
  pub fn set_sequences_index_buffer(mut self, value: Option<VkBuffer>) -> Self {
    self.sequencesIndexBuffer = value;
    self
  }
  #[inline]
  pub fn set_sequences_index_offset(mut self, value: VkDeviceSize) -> Self {
    self.sequencesIndexOffset = value;
    self
  }
  #[inline]
  pub fn object_table(&self) -> VkObjectTableNVX {
    self.objectTable
  }
  #[inline]
  pub fn indirect_commands_layout(&self) -> VkIndirectCommandsLayoutNVX {
    self.indirectCommandsLayout
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
  pub fn target_command_buffer(&self) -> Option<VkCommandBuffer> {
    self.targetCommandBuffer
  }
  #[inline]
  pub fn sequences_count_buffer(&self) -> Option<VkBuffer> {
    self.sequencesCountBuffer
  }
  #[inline]
  pub fn sequences_count_offset(&self) -> VkDeviceSize {
    self.sequencesCountOffset
  }
  #[inline]
  pub fn sequences_index_buffer(&self) -> Option<VkBuffer> {
    self.sequencesIndexBuffer
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
impl<'a> Default for VkCmdProcessCommandsInfoNVX<'a> {
  fn default() -> VkCmdProcessCommandsInfoNVX<'a> {
    VkCmdProcessCommandsInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'a> RawStruct for VkCmdProcessCommandsInfoNVX<'a> {
  type Raw = types_raw::VkCmdProcessCommandsInfoNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_cmd_process_commands_info_nvx() {
  assert_size!(
    types_raw::VkCmdProcessCommandsInfoNVX,
    VkCmdProcessCommandsInfoNVX
  );
}

/// Structure specifying parameters for the reservation of command buffer space
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkCmdReserveSpaceForCommandsInfoNVX<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub objectTable: VkObjectTableNVX,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
  pub maxSequencesCount: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> VkCmdReserveSpaceForCommandsInfoNVX<'a> {
  #[inline]
  pub fn new() -> VkCmdReserveSpaceForCommandsInfoNVX<'a> {
    unsafe {
      VkCmdReserveSpaceForCommandsInfoNVX {
        sType: VkStructureType::CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_object_table(mut self, value: VkObjectTableNVX) -> Self {
    self.objectTable = value;
    self
  }
  #[inline]
  pub fn set_indirect_commands_layout(mut self, value: VkIndirectCommandsLayoutNVX) -> Self {
    self.indirectCommandsLayout = value;
    self
  }
  #[inline]
  pub fn set_max_sequences_count(mut self, value: u32) -> Self {
    self.maxSequencesCount = value;
    self
  }
  #[inline]
  pub fn object_table(&self) -> VkObjectTableNVX {
    self.objectTable
  }
  #[inline]
  pub fn indirect_commands_layout(&self) -> VkIndirectCommandsLayoutNVX {
    self.indirectCommandsLayout
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
impl<'a> Default for VkCmdReserveSpaceForCommandsInfoNVX<'a> {
  fn default() -> VkCmdReserveSpaceForCommandsInfoNVX<'a> {
    VkCmdReserveSpaceForCommandsInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'a> RawStruct for VkCmdReserveSpaceForCommandsInfoNVX<'a> {
  type Raw = types_raw::VkCmdReserveSpaceForCommandsInfoNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_cmd_reserve_space_for_commands_info_nvx() {
  assert_size!(
    types_raw::VkCmdReserveSpaceForCommandsInfoNVX,
    VkCmdReserveSpaceForCommandsInfoNVX
  );
}

/// Structure specifying the parameters of a newly created object table
#[repr(C)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableCreateInfoNVX<'a> {
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
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> VkObjectTableCreateInfoNVX<'a> {
  #[inline]
  pub fn new() -> VkObjectTableCreateInfoNVX<'a> {
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
impl<'a> Default for VkObjectTableCreateInfoNVX<'a> {
  fn default() -> VkObjectTableCreateInfoNVX<'a> {
    VkObjectTableCreateInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl<'a> RawStruct for VkObjectTableCreateInfoNVX<'a> {
  type Raw = types_raw::VkObjectTableCreateInfoNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_create_info_nvx() {
  assert_size!(
    types_raw::VkObjectTableCreateInfoNVX,
    VkObjectTableCreateInfoNVX
  );
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
unsafe impl RawStruct for VkObjectTableEntryNVX {
  type Raw = types_raw::VkObjectTableEntryNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_entry_nvx() {
  assert_size!(types_raw::VkObjectTableEntryNVX, VkObjectTableEntryNVX);
}

/// Parameters of an object table pipeline entry
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTablePipelineEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub pipeline: VkPipeline,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkObjectTablePipelineEntryNVX {
  #[inline]
  pub fn new() -> VkObjectTablePipelineEntryNVX {
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
  pub fn set_pipeline(mut self, value: VkPipeline) -> Self {
    self.pipeline = value;
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
  pub fn pipeline(&self) -> VkPipeline {
    self.pipeline
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTablePipelineEntryNVX {
  fn default() -> VkObjectTablePipelineEntryNVX {
    VkObjectTablePipelineEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkObjectTablePipelineEntryNVX {
  type Raw = types_raw::VkObjectTablePipelineEntryNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_pipeline_entry_nvx() {
  assert_size!(
    types_raw::VkObjectTablePipelineEntryNVX,
    VkObjectTablePipelineEntryNVX
  );
}

/// Parameters of an object table descriptor set entry
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableDescriptorSetEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub pipelineLayout: VkPipelineLayout,
  pub descriptorSet: VkDescriptorSet,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkObjectTableDescriptorSetEntryNVX {
  #[inline]
  pub fn new() -> VkObjectTableDescriptorSetEntryNVX {
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
  pub fn set_pipeline_layout(mut self, value: VkPipelineLayout) -> Self {
    self.pipelineLayout = value;
    self
  }
  #[inline]
  pub fn set_descriptor_set(mut self, value: VkDescriptorSet) -> Self {
    self.descriptorSet = value;
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
  pub fn pipeline_layout(&self) -> VkPipelineLayout {
    self.pipelineLayout
  }
  #[inline]
  pub fn descriptor_set(&self) -> VkDescriptorSet {
    self.descriptorSet
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTableDescriptorSetEntryNVX {
  fn default() -> VkObjectTableDescriptorSetEntryNVX {
    VkObjectTableDescriptorSetEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkObjectTableDescriptorSetEntryNVX {
  type Raw = types_raw::VkObjectTableDescriptorSetEntryNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_descriptor_set_entry_nvx() {
  assert_size!(
    types_raw::VkObjectTableDescriptorSetEntryNVX,
    VkObjectTableDescriptorSetEntryNVX
  );
}

/// Parameters of an object table vertex buffer entry
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableVertexBufferEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub buffer: VkBuffer,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkObjectTableVertexBufferEntryNVX {
  #[inline]
  pub fn new() -> VkObjectTableVertexBufferEntryNVX {
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
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
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
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTableVertexBufferEntryNVX {
  fn default() -> VkObjectTableVertexBufferEntryNVX {
    VkObjectTableVertexBufferEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkObjectTableVertexBufferEntryNVX {
  type Raw = types_raw::VkObjectTableVertexBufferEntryNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_vertex_buffer_entry_nvx() {
  assert_size!(
    types_raw::VkObjectTableVertexBufferEntryNVX,
    VkObjectTableVertexBufferEntryNVX
  );
}

/// Parameters of an object table index buffer entry
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableIndexBufferEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub buffer: VkBuffer,
  pub indexType: VkIndexType,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkObjectTableIndexBufferEntryNVX {
  #[inline]
  pub fn new() -> VkObjectTableIndexBufferEntryNVX {
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
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
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
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn index_type(&self) -> VkIndexType {
    self.indexType
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTableIndexBufferEntryNVX {
  fn default() -> VkObjectTableIndexBufferEntryNVX {
    VkObjectTableIndexBufferEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkObjectTableIndexBufferEntryNVX {
  type Raw = types_raw::VkObjectTableIndexBufferEntryNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_index_buffer_entry_nvx() {
  assert_size!(
    types_raw::VkObjectTableIndexBufferEntryNVX,
    VkObjectTableIndexBufferEntryNVX
  );
}

/// Parameters of an object table push constant entry
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTablePushConstantEntryNVX {
  pub eType: VkObjectEntryTypeNVX,
  pub flags: VkObjectEntryUsageFlagsNVX,
  pub pipelineLayout: VkPipelineLayout,
  pub stageFlags: VkShaderStageFlags,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkObjectTablePushConstantEntryNVX {
  #[inline]
  pub fn new() -> VkObjectTablePushConstantEntryNVX {
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
  pub fn set_pipeline_layout(mut self, value: VkPipelineLayout) -> Self {
    self.pipelineLayout = value;
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
  pub fn pipeline_layout(&self) -> VkPipelineLayout {
    self.pipelineLayout
  }
  #[inline]
  pub fn stage_flags(&self) -> VkShaderStageFlags {
    self.stageFlags
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTablePushConstantEntryNVX {
  fn default() -> VkObjectTablePushConstantEntryNVX {
    VkObjectTablePushConstantEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkObjectTablePushConstantEntryNVX {
  type Raw = types_raw::VkObjectTablePushConstantEntryNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_push_constant_entry_nvx() {
  assert_size!(
    types_raw::VkObjectTablePushConstantEntryNVX,
    VkObjectTablePushConstantEntryNVX
  );
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
unsafe impl RawStruct for VkViewportWScalingNV {
  type Raw = types_raw::VkViewportWScalingNV;
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_viewport_w_scaling_nv() {
  assert_size!(types_raw::VkViewportWScalingNV, VkViewportWScalingNV);
}

/// Structure specifying parameters of a newly created pipeline viewport W scaling
/// state
#[repr(C)]
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub struct VkPipelineViewportWScalingStateCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub viewportWScalingEnable: VkBool32,
  viewportCount: u32,
  pViewportWScalings: *const types_raw::VkViewportWScalingNV,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl<'a> VkPipelineViewportWScalingStateCreateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkPipelineViewportWScalingStateCreateInfoNV<'a> {
    unsafe {
      VkPipelineViewportWScalingStateCreateInfoNV {
        sType: VkStructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_viewport_w_scaling_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.viewportWScalingEnable = value;
    self
  }
  #[inline]
  pub fn set_viewport_w_scalings(mut self, value: &'a [VkViewportWScalingNV]) -> Self {
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
impl<'a> Default for VkPipelineViewportWScalingStateCreateInfoNV<'a> {
  fn default() -> VkPipelineViewportWScalingStateCreateInfoNV<'a> {
    VkPipelineViewportWScalingStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl<'a> RawStruct for VkPipelineViewportWScalingStateCreateInfoNV<'a> {
  type Raw = types_raw::VkPipelineViewportWScalingStateCreateInfoNV;
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineViewportStateCreateInfo<'b>>
  for VkPipelineViewportWScalingStateCreateInfoNV<'a>
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
  assert_size!(
    types_raw::VkPipelineViewportWScalingStateCreateInfoNV,
    VkPipelineViewportWScalingStateCreateInfoNV
  );
}

// feature: VK_EXT_display_surface_counter

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
unsafe impl RawStruct for VkSurfaceCapabilities2EXT {
  type Raw = types_raw::VkSurfaceCapabilities2EXT;
}
#[cfg(feature = "VK_EXT_display_surface_counter")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_capabilities2_ext() {
  assert_size!(
    types_raw::VkSurfaceCapabilities2EXT,
    VkSurfaceCapabilities2EXT
  );
}

// feature: VK_EXT_display_control

/// Describe the power state of a display
#[repr(C)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDisplayPowerInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub powerState: VkDisplayPowerStateEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'a> VkDisplayPowerInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkDisplayPowerInfoEXT<'a> {
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
impl<'a> Default for VkDisplayPowerInfoEXT<'a> {
  fn default() -> VkDisplayPowerInfoEXT<'a> {
    VkDisplayPowerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'a> RawStruct for VkDisplayPowerInfoEXT<'a> {
  type Raw = types_raw::VkDisplayPowerInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_power_info_ext() {
  assert_size!(types_raw::VkDisplayPowerInfoEXT, VkDisplayPowerInfoEXT);
}

/// Describe a device event to create
#[repr(C)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDeviceEventInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub deviceEvent: VkDeviceEventTypeEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'a> VkDeviceEventInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkDeviceEventInfoEXT<'a> {
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
impl<'a> Default for VkDeviceEventInfoEXT<'a> {
  fn default() -> VkDeviceEventInfoEXT<'a> {
    VkDeviceEventInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'a> RawStruct for VkDeviceEventInfoEXT<'a> {
  type Raw = types_raw::VkDeviceEventInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_event_info_ext() {
  assert_size!(types_raw::VkDeviceEventInfoEXT, VkDeviceEventInfoEXT);
}

/// Describe a display event to create
#[repr(C)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDisplayEventInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub displayEvent: VkDisplayEventTypeEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'a> VkDisplayEventInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkDisplayEventInfoEXT<'a> {
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
impl<'a> Default for VkDisplayEventInfoEXT<'a> {
  fn default() -> VkDisplayEventInfoEXT<'a> {
    VkDisplayEventInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'a> RawStruct for VkDisplayEventInfoEXT<'a> {
  type Raw = types_raw::VkDisplayEventInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_event_info_ext() {
  assert_size!(types_raw::VkDisplayEventInfoEXT, VkDisplayEventInfoEXT);
}

/// Specify the surface counters desired
#[repr(C)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkSwapchainCounterCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub surfaceCounters: VkSurfaceCounterFlagsEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_display_control")]
impl<'a> VkSwapchainCounterCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkSwapchainCounterCreateInfoEXT<'a> {
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
impl<'a> Default for VkSwapchainCounterCreateInfoEXT<'a> {
  fn default() -> VkSwapchainCounterCreateInfoEXT<'a> {
    VkSwapchainCounterCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'a> RawStruct for VkSwapchainCounterCreateInfoEXT<'a> {
  type Raw = types_raw::VkSwapchainCounterCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSwapchainCreateInfoKHR<'b>> for VkSwapchainCounterCreateInfoEXT<'a> {
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
  assert_size!(
    types_raw::VkSwapchainCounterCreateInfoEXT,
    VkSwapchainCounterCreateInfoEXT
  );
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
unsafe impl RawStruct for VkRefreshCycleDurationGOOGLE {
  type Raw = types_raw::VkRefreshCycleDurationGOOGLE;
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_refresh_cycle_duration_google() {
  assert_size!(
    types_raw::VkRefreshCycleDurationGOOGLE,
    VkRefreshCycleDurationGOOGLE
  );
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
unsafe impl RawStruct for VkPastPresentationTimingGOOGLE {
  type Raw = types_raw::VkPastPresentationTimingGOOGLE;
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_past_presentation_timing_google() {
  assert_size!(
    types_raw::VkPastPresentationTimingGOOGLE,
    VkPastPresentationTimingGOOGLE
  );
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
unsafe impl RawStruct for VkPresentTimeGOOGLE {
  type Raw = types_raw::VkPresentTimeGOOGLE;
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_time_google() {
  assert_size!(types_raw::VkPresentTimeGOOGLE, VkPresentTimeGOOGLE);
}

/// The earliest time each image should be presented
#[repr(C)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkPresentTimesInfoGOOGLE<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  swapchainCount: u32,
  pTimes: *const types_raw::VkPresentTimeGOOGLE,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl<'a> VkPresentTimesInfoGOOGLE<'a> {
  #[inline]
  pub fn new() -> VkPresentTimesInfoGOOGLE<'a> {
    unsafe {
      VkPresentTimesInfoGOOGLE {
        sType: VkStructureType::PRESENT_TIMES_INFO_GOOGLE,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_times(mut self, value: &'a [VkPresentTimeGOOGLE]) -> Self {
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
impl<'a> Default for VkPresentTimesInfoGOOGLE<'a> {
  fn default() -> VkPresentTimesInfoGOOGLE<'a> {
    VkPresentTimesInfoGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl<'a> RawStruct for VkPresentTimesInfoGOOGLE<'a> {
  type Raw = types_raw::VkPresentTimesInfoGOOGLE;
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPresentInfoKHR<'b>> for VkPresentTimesInfoGOOGLE<'a> {
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
  assert_size!(
    types_raw::VkPresentTimesInfoGOOGLE,
    VkPresentTimesInfoGOOGLE
  );
}

// feature: VK_NVX_multiview_per_view_attributes

/// Structure describing multiview limits that can be supported by an implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub perViewPositionAllComponents: VkBool32,
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
unsafe impl RawStruct for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  type Raw = types_raw::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX,
    VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
  );
}

// feature: VK_NV_viewport_swizzle

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
unsafe impl RawStruct for VkViewportSwizzleNV {
  type Raw = types_raw::VkViewportSwizzleNV;
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_viewport_swizzle_nv() {
  assert_size!(types_raw::VkViewportSwizzleNV, VkViewportSwizzleNV);
}

/// Structure specifying swizzle applied to primitive clip coordinates
#[repr(C)]
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
  viewportCount: u32,
  pViewportSwizzles: *const types_raw::VkViewportSwizzleNV,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl<'a> VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
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
  pub fn set_viewport_swizzles(mut self, value: &'a [VkViewportSwizzleNV]) -> Self {
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
impl<'a> Default for VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
  fn default() -> VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
    VkPipelineViewportSwizzleStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl<'a> RawStruct for VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
  type Raw = types_raw::VkPipelineViewportSwizzleStateCreateInfoNV;
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineViewportStateCreateInfo<'b>>
  for VkPipelineViewportSwizzleStateCreateInfoNV<'a>
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
  assert_size!(
    types_raw::VkPipelineViewportSwizzleStateCreateInfoNV,
    VkPipelineViewportSwizzleStateCreateInfoNV
  );
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
unsafe impl RawStruct for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceDiscardRectanglePropertiesEXT;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceDiscardRectanglePropertiesEXT,
    VkPhysicalDeviceDiscardRectanglePropertiesEXT
  );
}

/// Structure specifying discard rectangle
#[repr(C)]
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
  pub discardRectangleMode: VkDiscardRectangleModeEXT,
  discardRectangleCount: u32,
  pDiscardRectangles: *const types_raw::VkRect2D,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl<'a> VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
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
  pub fn set_discard_rectangles(mut self, value: &'a [VkRect2D]) -> Self {
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
impl<'a> Default for VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
  fn default() -> VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
    VkPipelineDiscardRectangleStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
unsafe impl<'a> RawStruct for VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
  type Raw = types_raw::VkPipelineDiscardRectangleStateCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
unsafe impl<'b, 'a: 'b> StructExtends<VkGraphicsPipelineCreateInfo<'b>>
  for VkPipelineDiscardRectangleStateCreateInfoEXT<'a>
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
  assert_size!(
    types_raw::VkPipelineDiscardRectangleStateCreateInfoEXT,
    VkPipelineDiscardRectangleStateCreateInfoEXT
  );
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
  pub primitiveUnderestimation: VkBool32,
  pub conservativePointAndLineRasterization: VkBool32,
  pub degenerateTrianglesRasterized: VkBool32,
  pub degenerateLinesRasterized: VkBool32,
  pub fullyCoveredFragmentShaderInputVariable: VkBool32,
  pub conservativeRasterizationPostDepthCoverage: VkBool32,
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.primitiveUnderestimation = value;
    self
  }
  #[inline]
  pub fn set_conservative_point_and_line_rasterization(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.conservativePointAndLineRasterization = value;
    self
  }
  #[inline]
  pub fn set_degenerate_triangles_rasterized(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.degenerateTrianglesRasterized = value;
    self
  }
  #[inline]
  pub fn set_degenerate_lines_rasterized(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.degenerateLinesRasterized = value;
    self
  }
  #[inline]
  pub fn set_fully_covered_fragment_shader_input_variable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.fullyCoveredFragmentShaderInputVariable = value;
    self
  }
  #[inline]
  pub fn set_conservative_rasterization_post_depth_coverage(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.conservativeRasterizationPostDepthCoverage = value;
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
unsafe impl RawStruct for VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceConservativeRasterizationPropertiesEXT;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceConservativeRasterizationPropertiesEXT,
    VkPhysicalDeviceConservativeRasterizationPropertiesEXT
  );
}

/// Structure specifying conservative raster state
#[repr(C)]
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
  pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
  pub extraPrimitiveOverestimationSize: f32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl<'a> VkPipelineRasterizationConservativeStateCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkPipelineRasterizationConservativeStateCreateInfoEXT<'a> {
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
impl<'a> Default for VkPipelineRasterizationConservativeStateCreateInfoEXT<'a> {
  fn default() -> VkPipelineRasterizationConservativeStateCreateInfoEXT<'a> {
    VkPipelineRasterizationConservativeStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
unsafe impl<'a> RawStruct for VkPipelineRasterizationConservativeStateCreateInfoEXT<'a> {
  type Raw = types_raw::VkPipelineRasterizationConservativeStateCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineRasterizationStateCreateInfo<'b>>
  for VkPipelineRasterizationConservativeStateCreateInfoEXT<'a>
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
  assert_size!(
    types_raw::VkPipelineRasterizationConservativeStateCreateInfoEXT,
    VkPipelineRasterizationConservativeStateCreateInfoEXT
  );
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
unsafe impl RawStruct for VkXYColorEXT {
  type Raw = types_raw::VkXYColorEXT;
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_xy_color_ext() {
  assert_size!(types_raw::VkXYColorEXT, VkXYColorEXT);
}

/// structure to specify Hdr metadata
#[repr(C)]
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub struct VkHdrMetadataEXT<'a> {
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
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl<'a> VkHdrMetadataEXT<'a> {
  #[inline]
  pub fn new() -> VkHdrMetadataEXT<'a> {
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
impl<'a> Default for VkHdrMetadataEXT<'a> {
  fn default() -> VkHdrMetadataEXT<'a> {
    VkHdrMetadataEXT::new()
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
unsafe impl<'a> RawStruct for VkHdrMetadataEXT<'a> {
  type Raw = types_raw::VkHdrMetadataEXT;
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_hdr_metadata_ext() {
  assert_size!(types_raw::VkHdrMetadataEXT, VkHdrMetadataEXT);
}

// feature: VK_KHR_get_surface_capabilities2

/// Structure specifying a surface and related swapchain creation parameters
#[repr(C)]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub struct VkPhysicalDeviceSurfaceInfo2KHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub surface: VkSurfaceKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl<'a> VkPhysicalDeviceSurfaceInfo2KHR<'a> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceSurfaceInfo2KHR<'a> {
    unsafe {
      VkPhysicalDeviceSurfaceInfo2KHR {
        sType: VkStructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_surface(mut self, value: VkSurfaceKHR) -> Self {
    self.surface = value;
    self
  }
  #[inline]
  pub fn surface(&self) -> VkSurfaceKHR {
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
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl<'a> Default for VkPhysicalDeviceSurfaceInfo2KHR<'a> {
  fn default() -> VkPhysicalDeviceSurfaceInfo2KHR<'a> {
    VkPhysicalDeviceSurfaceInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
unsafe impl<'a> RawStruct for VkPhysicalDeviceSurfaceInfo2KHR<'a> {
  type Raw = types_raw::VkPhysicalDeviceSurfaceInfo2KHR;
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_surface_info2_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceSurfaceInfo2KHR,
    VkPhysicalDeviceSurfaceInfo2KHR
  );
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
unsafe impl RawStruct for VkSurfaceCapabilities2KHR {
  type Raw = types_raw::VkSurfaceCapabilities2KHR;
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_capabilities2_khr() {
  assert_size!(
    types_raw::VkSurfaceCapabilities2KHR,
    VkSurfaceCapabilities2KHR
  );
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
unsafe impl RawStruct for VkSurfaceFormat2KHR {
  type Raw = types_raw::VkSurfaceFormat2KHR;
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_surface_format2_khr() {
  assert_size!(types_raw::VkSurfaceFormat2KHR, VkSurfaceFormat2KHR);
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
unsafe impl RawStruct for VkSharedPresentSurfaceCapabilitiesKHR {
  type Raw = types_raw::VkSharedPresentSurfaceCapabilitiesKHR;
}
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
  assert_size!(
    types_raw::VkSharedPresentSurfaceCapabilitiesKHR,
    VkSharedPresentSurfaceCapabilitiesKHR
  );
}

// feature: VK_KHR_external_fence_capabilities

/// Structure specifying fence creation parameters.
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub struct VkPhysicalDeviceExternalFenceInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl<'a> VkPhysicalDeviceExternalFenceInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalFenceInfoKHR<'a> {
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
impl<'a> Default for VkPhysicalDeviceExternalFenceInfoKHR<'a> {
  fn default() -> VkPhysicalDeviceExternalFenceInfoKHR<'a> {
    VkPhysicalDeviceExternalFenceInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
unsafe impl<'a> RawStruct for VkPhysicalDeviceExternalFenceInfoKHR<'a> {
  type Raw = types_raw::VkPhysicalDeviceExternalFenceInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_fence_info_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceExternalFenceInfoKHR,
    VkPhysicalDeviceExternalFenceInfoKHR
  );
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
unsafe impl RawStruct for VkExternalFencePropertiesKHR {
  type Raw = types_raw::VkExternalFencePropertiesKHR;
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_external_fence_properties_khr() {
  assert_size!(
    types_raw::VkExternalFencePropertiesKHR,
    VkExternalFencePropertiesKHR
  );
}

// feature: VK_KHR_external_fence

/// Structure specifying handle types that can be exported from a fence
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence")]
pub struct VkExportFenceCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleTypes: VkExternalFenceHandleTypeFlagsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_fence")]
impl<'a> VkExportFenceCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkExportFenceCreateInfoKHR<'a> {
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
impl<'a> Default for VkExportFenceCreateInfoKHR<'a> {
  fn default() -> VkExportFenceCreateInfoKHR<'a> {
    VkExportFenceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence")]
unsafe impl<'a> RawStruct for VkExportFenceCreateInfoKHR<'a> {
  type Raw = types_raw::VkExportFenceCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence")]
unsafe impl<'b, 'a: 'b> StructExtends<VkFenceCreateInfo<'b>> for VkExportFenceCreateInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkExportFenceCreateInfoKHR,
    VkExportFenceCreateInfoKHR
  );
}

// feature: VK_KHR_external_fence_win32

/// (None)
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportFenceWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub fence: VkFence,
  pub flags: VkFenceImportFlagsKHR,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkImportFenceWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImportFenceWin32HandleInfoKHR<'a> {
    unsafe {
      VkImportFenceWin32HandleInfoKHR {
        sType: VkStructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_fence(mut self, value: VkFence) -> Self {
    self.fence = value;
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
  pub fn fence(&self) -> VkFence {
    self.fence
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
impl<'a> Default for VkImportFenceWin32HandleInfoKHR<'a> {
  fn default() -> VkImportFenceWin32HandleInfoKHR<'a> {
    VkImportFenceWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkImportFenceWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkImportFenceWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_fence_win32_handle_info_khr() {
  assert_size!(
    types_raw::VkImportFenceWin32HandleInfoKHR,
    VkImportFenceWin32HandleInfoKHR
  );
}

/// Structure specifying additional attributes of Windows handles exported from a
/// fence
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportFenceWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkExportFenceWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkExportFenceWin32HandleInfoKHR<'a> {
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
impl<'a> Default for VkExportFenceWin32HandleInfoKHR<'a> {
  fn default() -> VkExportFenceWin32HandleInfoKHR<'a> {
    VkExportFenceWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkExportFenceWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkExportFenceWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'b, 'a: 'b> StructExtends<VkFenceCreateInfo<'b>> for VkExportFenceWin32HandleInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkExportFenceWin32HandleInfoKHR,
    VkExportFenceWin32HandleInfoKHR
  );
}

/// Structure describing a Win32 handle fence export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkFenceGetWin32HandleInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl<'a> VkFenceGetWin32HandleInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkFenceGetWin32HandleInfoKHR<'a> {
    unsafe {
      VkFenceGetWin32HandleInfoKHR {
        sType: VkStructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_fence(mut self, value: VkFence) -> Self {
    self.fence = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalFenceHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn fence(&self) -> VkFence {
    self.fence
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
impl<'a> Default for VkFenceGetWin32HandleInfoKHR<'a> {
  fn default() -> VkFenceGetWin32HandleInfoKHR<'a> {
    VkFenceGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl<'a> RawStruct for VkFenceGetWin32HandleInfoKHR<'a> {
  type Raw = types_raw::VkFenceGetWin32HandleInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_get_win32_handle_info_khr() {
  assert_size!(
    types_raw::VkFenceGetWin32HandleInfoKHR,
    VkFenceGetWin32HandleInfoKHR
  );
}

// feature: VK_KHR_external_fence_fd

/// (None)
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub struct VkImportFenceFdInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub fence: VkFence,
  pub flags: VkFenceImportFlagsKHR,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  pub fd: c_int,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<'a> VkImportFenceFdInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImportFenceFdInfoKHR<'a> {
    unsafe {
      VkImportFenceFdInfoKHR {
        sType: VkStructureType::IMPORT_FENCE_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_fence(mut self, value: VkFence) -> Self {
    self.fence = value;
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
  pub fn fence(&self) -> VkFence {
    self.fence
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
impl<'a> Default for VkImportFenceFdInfoKHR<'a> {
  fn default() -> VkImportFenceFdInfoKHR<'a> {
    VkImportFenceFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
unsafe impl<'a> RawStruct for VkImportFenceFdInfoKHR<'a> {
  type Raw = types_raw::VkImportFenceFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_fence_fd_info_khr() {
  assert_size!(types_raw::VkImportFenceFdInfoKHR, VkImportFenceFdInfoKHR);
}

/// Structure describing a POSIX FD fence export operation
#[repr(C)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub struct VkFenceGetFdInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl<'a> VkFenceGetFdInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkFenceGetFdInfoKHR<'a> {
    unsafe {
      VkFenceGetFdInfoKHR {
        sType: VkStructureType::FENCE_GET_FD_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_fence(mut self, value: VkFence) -> Self {
    self.fence = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalFenceHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
  #[inline]
  pub fn fence(&self) -> VkFence {
    self.fence
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
impl<'a> Default for VkFenceGetFdInfoKHR<'a> {
  fn default() -> VkFenceGetFdInfoKHR<'a> {
    VkFenceGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
unsafe impl<'a> RawStruct for VkFenceGetFdInfoKHR<'a> {
  type Raw = types_raw::VkFenceGetFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_get_fd_info_khr() {
  assert_size!(types_raw::VkFenceGetFdInfoKHR, VkFenceGetFdInfoKHR);
}

// feature: VK_KHR_maintenance2

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
unsafe impl RawStruct for VkPhysicalDevicePointClippingPropertiesKHR {
  type Raw = types_raw::VkPhysicalDevicePointClippingPropertiesKHR;
}
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
  assert_size!(
    types_raw::VkPhysicalDevicePointClippingPropertiesKHR,
    VkPhysicalDevicePointClippingPropertiesKHR
  );
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
unsafe impl RawStruct for VkInputAttachmentAspectReferenceKHR {
  type Raw = types_raw::VkInputAttachmentAspectReferenceKHR;
}
#[cfg(feature = "VK_KHR_maintenance2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_input_attachment_aspect_reference_khr() {
  assert_size!(
    types_raw::VkInputAttachmentAspectReferenceKHR,
    VkInputAttachmentAspectReferenceKHR
  );
}

/// Structure specifying, for a given subpass/input attachment pair, which aspect
/// can: be read.
#[repr(C)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  aspectReferenceCount: u32,
  pAspectReferences: *const types_raw::VkInputAttachmentAspectReferenceKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'a> VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
    unsafe {
      VkRenderPassInputAttachmentAspectCreateInfoKHR {
        sType: VkStructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_aspect_references(mut self, value: &'a [VkInputAttachmentAspectReferenceKHR]) -> Self {
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
impl<'a> Default for VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
  fn default() -> VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
    VkRenderPassInputAttachmentAspectCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'a> RawStruct for VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
  type Raw = types_raw::VkRenderPassInputAttachmentAspectCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'b, 'a: 'b> StructExtends<VkRenderPassCreateInfo<'b>>
  for VkRenderPassInputAttachmentAspectCreateInfoKHR<'a>
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
  assert_size!(
    types_raw::VkRenderPassInputAttachmentAspectCreateInfoKHR,
    VkRenderPassInputAttachmentAspectCreateInfoKHR
  );
}

/// Specify the intended usage of an image view
#[repr(C)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkImageViewUsageCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub usage: VkImageUsageFlags,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'a> VkImageViewUsageCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImageViewUsageCreateInfoKHR<'a> {
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
impl<'a> Default for VkImageViewUsageCreateInfoKHR<'a> {
  fn default() -> VkImageViewUsageCreateInfoKHR<'a> {
    VkImageViewUsageCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'a> RawStruct for VkImageViewUsageCreateInfoKHR<'a> {
  type Raw = types_raw::VkImageViewUsageCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageViewCreateInfo<'b>> for VkImageViewUsageCreateInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkImageViewUsageCreateInfoKHR,
    VkImageViewUsageCreateInfoKHR
  );
}

/// Structure specifying the orientation of the tessellation domain
#[repr(C)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub domainOrigin: VkTessellationDomainOriginKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'a> VkPipelineTessellationDomainOriginStateCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkPipelineTessellationDomainOriginStateCreateInfoKHR<'a> {
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
impl<'a> Default for VkPipelineTessellationDomainOriginStateCreateInfoKHR<'a> {
  fn default() -> VkPipelineTessellationDomainOriginStateCreateInfoKHR<'a> {
    VkPipelineTessellationDomainOriginStateCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'a> RawStruct for VkPipelineTessellationDomainOriginStateCreateInfoKHR<'a> {
  type Raw = types_raw::VkPipelineTessellationDomainOriginStateCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineTessellationStateCreateInfo<'b>>
  for VkPipelineTessellationDomainOriginStateCreateInfoKHR<'a>
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
  assert_size!(
    types_raw::VkPipelineTessellationDomainOriginStateCreateInfoKHR,
    VkPipelineTessellationDomainOriginStateCreateInfoKHR
  );
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
  pub variablePointersStorageBuffer: VkBool32,
  pub variablePointers: VkBool32,
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.variablePointersStorageBuffer = value;
    self
  }
  #[inline]
  pub fn set_variable_pointers(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.variablePointers = value;
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
unsafe impl RawStruct for VkPhysicalDeviceVariablePointerFeaturesKHR {
  type Raw = types_raw::VkPhysicalDeviceVariablePointerFeaturesKHR;
}
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
unsafe impl<'b> StructExtends<VkDeviceCreateInfo<'b>> for VkPhysicalDeviceVariablePointerFeaturesKHR {
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
  assert_size!(
    types_raw::VkPhysicalDeviceVariablePointerFeaturesKHR,
    VkPhysicalDeviceVariablePointerFeaturesKHR
  );
}

// feature: VK_MVK_ios_surface

/// Structure specifying parameters of a newly created iOS surface object
#[repr(C)]
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub struct VkIOSSurfaceCreateInfoMVK<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkIOSSurfaceCreateFlagsMVK,
  pView: *const c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
impl<'a> VkIOSSurfaceCreateInfoMVK<'a> {
  #[inline]
  pub fn new() -> VkIOSSurfaceCreateInfoMVK<'a> {
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
impl<'a> Default for VkIOSSurfaceCreateInfoMVK<'a> {
  fn default() -> VkIOSSurfaceCreateInfoMVK<'a> {
    VkIOSSurfaceCreateInfoMVK::new()
  }
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
unsafe impl<'a> RawStruct for VkIOSSurfaceCreateInfoMVK<'a> {
  type Raw = types_raw::VkIOSSurfaceCreateInfoMVK;
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_ios_surface_create_info_mvk() {
  assert_size!(
    types_raw::VkIOSSurfaceCreateInfoMVK,
    VkIOSSurfaceCreateInfoMVK
  );
}

// feature: VK_MVK_macos_surface

/// Structure specifying parameters of a newly created macOS surface object
#[repr(C)]
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub struct VkMacOSSurfaceCreateInfoMVK<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkMacOSSurfaceCreateFlagsMVK,
  pView: *const c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
impl<'a> VkMacOSSurfaceCreateInfoMVK<'a> {
  #[inline]
  pub fn new() -> VkMacOSSurfaceCreateInfoMVK<'a> {
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
impl<'a> Default for VkMacOSSurfaceCreateInfoMVK<'a> {
  fn default() -> VkMacOSSurfaceCreateInfoMVK<'a> {
    VkMacOSSurfaceCreateInfoMVK::new()
  }
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
unsafe impl<'a> RawStruct for VkMacOSSurfaceCreateInfoMVK<'a> {
  type Raw = types_raw::VkMacOSSurfaceCreateInfoMVK;
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_mac_os_surface_create_info_mvk() {
  assert_size!(
    types_raw::VkMacOSSurfaceCreateInfoMVK,
    VkMacOSSurfaceCreateInfoMVK
  );
}

// feature: VK_KHR_get_memory_requirements2

/// (None)
#[repr(C)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkBufferMemoryRequirementsInfo2KHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub buffer: VkBuffer,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'a> VkBufferMemoryRequirementsInfo2KHR<'a> {
  #[inline]
  pub fn new() -> VkBufferMemoryRequirementsInfo2KHR<'a> {
    unsafe {
      VkBufferMemoryRequirementsInfo2KHR {
        sType: VkStructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
    self
  }
  #[inline]
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
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
impl<'a> Default for VkBufferMemoryRequirementsInfo2KHR<'a> {
  fn default() -> VkBufferMemoryRequirementsInfo2KHR<'a> {
    VkBufferMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl<'a> RawStruct for VkBufferMemoryRequirementsInfo2KHR<'a> {
  type Raw = types_raw::VkBufferMemoryRequirementsInfo2KHR;
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_memory_requirements_info2_khr() {
  assert_size!(
    types_raw::VkBufferMemoryRequirementsInfo2KHR,
    VkBufferMemoryRequirementsInfo2KHR
  );
}

/// (None)
#[repr(C)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkImageMemoryRequirementsInfo2KHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub image: VkImage,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'a> VkImageMemoryRequirementsInfo2KHR<'a> {
  #[inline]
  pub fn new() -> VkImageMemoryRequirementsInfo2KHR<'a> {
    unsafe {
      VkImageMemoryRequirementsInfo2KHR {
        sType: VkStructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
    self
  }
  #[inline]
  pub fn image(&self) -> VkImage {
    self.image
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
impl<'a> Default for VkImageMemoryRequirementsInfo2KHR<'a> {
  fn default() -> VkImageMemoryRequirementsInfo2KHR<'a> {
    VkImageMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl<'a> RawStruct for VkImageMemoryRequirementsInfo2KHR<'a> {
  type Raw = types_raw::VkImageMemoryRequirementsInfo2KHR;
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_memory_requirements_info2_khr() {
  assert_size!(
    types_raw::VkImageMemoryRequirementsInfo2KHR,
    VkImageMemoryRequirementsInfo2KHR
  );
}
#[repr(C)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkImageSparseMemoryRequirementsInfo2KHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub image: VkImage,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl<'a> VkImageSparseMemoryRequirementsInfo2KHR<'a> {
  #[inline]
  pub fn new() -> VkImageSparseMemoryRequirementsInfo2KHR<'a> {
    unsafe {
      VkImageSparseMemoryRequirementsInfo2KHR {
        sType: VkStructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
    self
  }
  #[inline]
  pub fn image(&self) -> VkImage {
    self.image
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
impl<'a> Default for VkImageSparseMemoryRequirementsInfo2KHR<'a> {
  fn default() -> VkImageSparseMemoryRequirementsInfo2KHR<'a> {
    VkImageSparseMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl<'a> RawStruct for VkImageSparseMemoryRequirementsInfo2KHR<'a> {
  type Raw = types_raw::VkImageSparseMemoryRequirementsInfo2KHR;
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_sparse_memory_requirements_info2_khr() {
  assert_size!(
    types_raw::VkImageSparseMemoryRequirementsInfo2KHR,
    VkImageSparseMemoryRequirementsInfo2KHR
  );
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
unsafe impl RawStruct for VkMemoryRequirements2KHR {
  type Raw = types_raw::VkMemoryRequirements2KHR;
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_requirements2_khr() {
  assert_size!(
    types_raw::VkMemoryRequirements2KHR,
    VkMemoryRequirements2KHR
  );
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
unsafe impl RawStruct for VkSparseImageMemoryRequirements2KHR {
  type Raw = types_raw::VkSparseImageMemoryRequirements2KHR;
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_requirements2_khr() {
  assert_size!(
    types_raw::VkSparseImageMemoryRequirements2KHR,
    VkSparseImageMemoryRequirements2KHR
  );
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
  pub prefersDedicatedAllocation: VkBool32,
  pub requiresDedicatedAllocation: VkBool32,
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
unsafe impl RawStruct for VkMemoryDedicatedRequirementsKHR {
  type Raw = types_raw::VkMemoryDedicatedRequirementsKHR;
}
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
  assert_size!(
    types_raw::VkMemoryDedicatedRequirementsKHR,
    VkMemoryDedicatedRequirementsKHR
  );
}

/// Specify a dedicated memory allocation resource
#[repr(C)]
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub struct VkMemoryDedicatedAllocateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub image: Option<VkImage>,
  pub buffer: Option<VkBuffer>,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl<'a> VkMemoryDedicatedAllocateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkMemoryDedicatedAllocateInfoKHR<'a> {
    unsafe {
      VkMemoryDedicatedAllocateInfoKHR {
        sType: VkStructureType::MEMORY_DEDICATED_ALLOCATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: Option<VkImage>) -> Self {
    self.image = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: Option<VkBuffer>) -> Self {
    self.buffer = value;
    self
  }
  #[inline]
  pub fn image(&self) -> Option<VkImage> {
    self.image
  }
  #[inline]
  pub fn buffer(&self) -> Option<VkBuffer> {
    self.buffer
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl<'a> Default for VkMemoryDedicatedAllocateInfoKHR<'a> {
  fn default() -> VkMemoryDedicatedAllocateInfoKHR<'a> {
    VkMemoryDedicatedAllocateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl<'a> RawStruct for VkMemoryDedicatedAllocateInfoKHR<'a> {
  type Raw = types_raw::VkMemoryDedicatedAllocateInfoKHR;
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkMemoryDedicatedAllocateInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkMemoryDedicatedAllocateInfoKHR,
    VkMemoryDedicatedAllocateInfoKHR
  );
}

// feature: VK_EXT_sampler_filter_minmax

/// Structure specifying sampler reduction mode
#[repr(C)]
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub struct VkSamplerReductionModeCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub reductionMode: VkSamplerReductionModeEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl<'a> VkSamplerReductionModeCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkSamplerReductionModeCreateInfoEXT<'a> {
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
impl<'a> Default for VkSamplerReductionModeCreateInfoEXT<'a> {
  fn default() -> VkSamplerReductionModeCreateInfoEXT<'a> {
    VkSamplerReductionModeCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
unsafe impl<'a> RawStruct for VkSamplerReductionModeCreateInfoEXT<'a> {
  type Raw = types_raw::VkSamplerReductionModeCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSamplerCreateInfo<'b>> for VkSamplerReductionModeCreateInfoEXT<'a> {
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
  assert_size!(
    types_raw::VkSamplerReductionModeCreateInfoEXT,
    VkSamplerReductionModeCreateInfoEXT
  );
}

/// Structure describing sampler filter minmax limits that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub filterMinmaxSingleComponentFormats: VkBool32,
  pub filterMinmaxImageComponentMapping: VkBool32,
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
unsafe impl RawStruct for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT,
    VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT
  );
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
unsafe impl RawStruct for VkSampleLocationEXT {
  type Raw = types_raw::VkSampleLocationEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sample_location_ext() {
  assert_size!(types_raw::VkSampleLocationEXT, VkSampleLocationEXT);
}

/// Structure specifying a set of sample locations
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSampleLocationsInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub sampleLocationsPerPixel: VkSampleCountFlagBits,
  pub sampleLocationGridSize: VkExtent2D,
  sampleLocationsCount: u32,
  pSampleLocations: *const types_raw::VkSampleLocationEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> VkSampleLocationsInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkSampleLocationsInfoEXT<'a> {
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
  pub fn set_sample_locations(mut self, value: &'a [VkSampleLocationEXT]) -> Self {
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
impl<'a> Default for VkSampleLocationsInfoEXT<'a> {
  fn default() -> VkSampleLocationsInfoEXT<'a> {
    VkSampleLocationsInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'a> RawStruct for VkSampleLocationsInfoEXT<'a> {
  type Raw = types_raw::VkSampleLocationsInfoEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageMemoryBarrier<'b>> for VkSampleLocationsInfoEXT<'a> {
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
  assert_size!(
    types_raw::VkSampleLocationsInfoEXT,
    VkSampleLocationsInfoEXT
  );
}

/// Structure specifying the sample locations state to use in the initial layout
/// transition of attachments
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkAttachmentSampleLocationsEXT<'a> {
  pub attachmentIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT<'a>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> VkAttachmentSampleLocationsEXT<'a> {
  #[inline]
  pub fn new() -> VkAttachmentSampleLocationsEXT<'a> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_attachment_index(mut self, value: u32) -> Self {
    self.attachmentIndex = value;
    self
  }
  #[inline]
  pub fn set_sample_locations_info(mut self, value: VkSampleLocationsInfoEXT<'a>) -> Self {
    self.sampleLocationsInfo = value;
    self
  }
  #[inline]
  pub fn attachment_index(&self) -> u32 {
    self.attachmentIndex
  }
  #[inline]
  pub fn sample_locations_info(&self) -> &VkSampleLocationsInfoEXT<'a> {
    &self.sampleLocationsInfo
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> Default for VkAttachmentSampleLocationsEXT<'a> {
  fn default() -> VkAttachmentSampleLocationsEXT<'a> {
    VkAttachmentSampleLocationsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'a> RawStruct for VkAttachmentSampleLocationsEXT<'a> {
  type Raw = types_raw::VkAttachmentSampleLocationsEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_attachment_sample_locations_ext() {
  assert_size!(
    types_raw::VkAttachmentSampleLocationsEXT,
    VkAttachmentSampleLocationsEXT
  );
}

/// Structure specifying the sample locations state to use for layout transitions of
/// attachments performed after a given subpass
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSubpassSampleLocationsEXT<'a> {
  pub subpassIndex: u32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT<'a>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> VkSubpassSampleLocationsEXT<'a> {
  #[inline]
  pub fn new() -> VkSubpassSampleLocationsEXT<'a> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_subpass_index(mut self, value: u32) -> Self {
    self.subpassIndex = value;
    self
  }
  #[inline]
  pub fn set_sample_locations_info(mut self, value: VkSampleLocationsInfoEXT<'a>) -> Self {
    self.sampleLocationsInfo = value;
    self
  }
  #[inline]
  pub fn subpass_index(&self) -> u32 {
    self.subpassIndex
  }
  #[inline]
  pub fn sample_locations_info(&self) -> &VkSampleLocationsInfoEXT<'a> {
    &self.sampleLocationsInfo
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> Default for VkSubpassSampleLocationsEXT<'a> {
  fn default() -> VkSubpassSampleLocationsEXT<'a> {
    VkSubpassSampleLocationsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'a> RawStruct for VkSubpassSampleLocationsEXT<'a> {
  type Raw = types_raw::VkSubpassSampleLocationsEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_subpass_sample_locations_ext() {
  assert_size!(
    types_raw::VkSubpassSampleLocationsEXT,
    VkSubpassSampleLocationsEXT
  );
}

/// Structure specifying sample locations to use for the layout transition of custom
/// sample locations compatible depth/stencil attachments
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkRenderPassSampleLocationsBeginInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  attachmentInitialSampleLocationsCount: u32,
  pAttachmentInitialSampleLocations: *const types_raw::VkAttachmentSampleLocationsEXT,
  postSubpassSampleLocationsCount: u32,
  pPostSubpassSampleLocations: *const types_raw::VkSubpassSampleLocationsEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> VkRenderPassSampleLocationsBeginInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkRenderPassSampleLocationsBeginInfoEXT<'a> {
    unsafe {
      VkRenderPassSampleLocationsBeginInfoEXT {
        sType: VkStructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_attachment_initial_sample_locations(mut self, value: &'a [VkAttachmentSampleLocationsEXT<'a>]) -> Self {
    self.attachmentInitialSampleLocationsCount = value.len() as u32;
    unsafe {
      self.pAttachmentInitialSampleLocations = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_post_subpass_sample_locations(mut self, value: &'a [VkSubpassSampleLocationsEXT<'a>]) -> Self {
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
impl<'a> Default for VkRenderPassSampleLocationsBeginInfoEXT<'a> {
  fn default() -> VkRenderPassSampleLocationsBeginInfoEXT<'a> {
    VkRenderPassSampleLocationsBeginInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'a> RawStruct for VkRenderPassSampleLocationsBeginInfoEXT<'a> {
  type Raw = types_raw::VkRenderPassSampleLocationsBeginInfoEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'b, 'a: 'b> StructExtends<VkRenderPassBeginInfo<'b>> for VkRenderPassSampleLocationsBeginInfoEXT<'a> {
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
  assert_size!(
    types_raw::VkRenderPassSampleLocationsBeginInfoEXT,
    VkRenderPassSampleLocationsBeginInfoEXT
  );
}

/// Structure specifying sample locations for a pipeline
#[repr(C)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub sampleLocationsEnable: VkBool32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT<'a>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
    unsafe {
      VkPipelineSampleLocationsStateCreateInfoEXT {
        sType: VkStructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_sample_locations_enable(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.sampleLocationsEnable = value;
    self
  }
  #[inline]
  pub fn set_sample_locations_info(mut self, value: VkSampleLocationsInfoEXT<'a>) -> Self {
    self.sampleLocationsInfo = value;
    self
  }
  #[inline]
  pub fn is_sample_locations_enable(&self) -> bool {
    self.sampleLocationsEnable != 0
  }
  #[inline]
  pub fn sample_locations_info(&self) -> &VkSampleLocationsInfoEXT<'a> {
    &self.sampleLocationsInfo
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> Default for VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
  fn default() -> VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
    VkPipelineSampleLocationsStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'a> RawStruct for VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
  type Raw = types_raw::VkPipelineSampleLocationsStateCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineMultisampleStateCreateInfo<'b>>
  for VkPipelineSampleLocationsStateCreateInfoEXT<'a>
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
  assert_size!(
    types_raw::VkPipelineSampleLocationsStateCreateInfoEXT,
    VkPipelineSampleLocationsStateCreateInfoEXT
  );
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
  pub variableSampleLocations: VkBool32,
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
unsafe impl RawStruct for VkPhysicalDeviceSampleLocationsPropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceSampleLocationsPropertiesEXT;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceSampleLocationsPropertiesEXT,
    VkPhysicalDeviceSampleLocationsPropertiesEXT
  );
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
unsafe impl RawStruct for VkMultisamplePropertiesEXT {
  type Raw = types_raw::VkMultisamplePropertiesEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_multisample_properties_ext() {
  assert_size!(
    types_raw::VkMultisamplePropertiesEXT,
    VkMultisamplePropertiesEXT
  );
}

// feature: VK_KHR_image_format_list

/// Specify that an image can: be used with a particular set of formats
#[repr(C)]
#[cfg(feature = "VK_KHR_image_format_list")]
pub struct VkImageFormatListCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  viewFormatCount: u32,
  pViewFormats: *const VkFormat,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_image_format_list")]
impl<'a> VkImageFormatListCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImageFormatListCreateInfoKHR<'a> {
    unsafe {
      VkImageFormatListCreateInfoKHR {
        sType: VkStructureType::IMAGE_FORMAT_LIST_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_view_formats(mut self, value: &'a [VkFormat]) -> Self {
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
impl<'a> Default for VkImageFormatListCreateInfoKHR<'a> {
  fn default() -> VkImageFormatListCreateInfoKHR<'a> {
    VkImageFormatListCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_image_format_list")]
unsafe impl<'a> RawStruct for VkImageFormatListCreateInfoKHR<'a> {
  type Raw = types_raw::VkImageFormatListCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_image_format_list")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageCreateInfo<'b>> for VkImageFormatListCreateInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkImageFormatListCreateInfoKHR,
    VkImageFormatListCreateInfoKHR
  );
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
  pub advancedBlendCoherentOperations: VkBool32,
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.advancedBlendCoherentOperations = value;
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
unsafe impl RawStruct for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  type Raw = types_raw::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT,
    VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
  );
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
  pub advancedBlendIndependentBlend: VkBool32,
  pub advancedBlendNonPremultipliedSrcColor: VkBool32,
  pub advancedBlendNonPremultipliedDstColor: VkBool32,
  pub advancedBlendCorrelatedOverlap: VkBool32,
  pub advancedBlendAllOperations: VkBool32,
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
unsafe impl RawStruct for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT,
    VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
  );
}

/// Structure specifying parameters that affect advanced blend operations
#[repr(C)]
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub srcPremultiplied: VkBool32,
  pub dstPremultiplied: VkBool32,
  pub blendOverlap: VkBlendOverlapEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl<'a> VkPipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkPipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
    unsafe {
      VkPipelineColorBlendAdvancedStateCreateInfoEXT {
        sType: VkStructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_premultiplied(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.srcPremultiplied = value;
    self
  }
  #[inline]
  pub fn set_dst_premultiplied(mut self, value: bool) -> Self {
    let value: VkBool32 = if value { 1 } else { 0 };
    self.dstPremultiplied = value;
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
impl<'a> Default for VkPipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
  fn default() -> VkPipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
    VkPipelineColorBlendAdvancedStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl<'a> RawStruct for VkPipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
  type Raw = types_raw::VkPipelineColorBlendAdvancedStateCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineColorBlendStateCreateInfo<'b>>
  for VkPipelineColorBlendAdvancedStateCreateInfoEXT<'a>
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
  assert_size!(
    types_raw::VkPipelineColorBlendAdvancedStateCreateInfoEXT,
    VkPipelineColorBlendAdvancedStateCreateInfoEXT
  );
}

// feature: VK_NV_fragment_coverage_to_color

/// Structure specifying whether fragment coverage replaces a color
#[repr(C)]
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub struct VkPipelineCoverageToColorStateCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
  pub coverageToColorEnable: VkBool32,
  pub coverageToColorLocation: u32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
impl<'a> VkPipelineCoverageToColorStateCreateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkPipelineCoverageToColorStateCreateInfoNV<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.coverageToColorEnable = value;
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
impl<'a> Default for VkPipelineCoverageToColorStateCreateInfoNV<'a> {
  fn default() -> VkPipelineCoverageToColorStateCreateInfoNV<'a> {
    VkPipelineCoverageToColorStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
unsafe impl<'a> RawStruct for VkPipelineCoverageToColorStateCreateInfoNV<'a> {
  type Raw = types_raw::VkPipelineCoverageToColorStateCreateInfoNV;
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineMultisampleStateCreateInfo<'b>>
  for VkPipelineCoverageToColorStateCreateInfoNV<'a>
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
  assert_size!(
    types_raw::VkPipelineCoverageToColorStateCreateInfoNV,
    VkPipelineCoverageToColorStateCreateInfoNV
  );
}

// feature: VK_NV_framebuffer_mixed_samples

/// Structure specifying parameters controlling coverage modulation
#[repr(C)]
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub struct VkPipelineCoverageModulationStateCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
  pub coverageModulationMode: VkCoverageModulationModeNV,
  pub coverageModulationTableEnable: VkBool32,
  coverageModulationTableCount: u32,
  pCoverageModulationTable: *const f32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
impl<'a> VkPipelineCoverageModulationStateCreateInfoNV<'a> {
  #[inline]
  pub fn new() -> VkPipelineCoverageModulationStateCreateInfoNV<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.coverageModulationTableEnable = value;
    self
  }
  #[inline]
  pub fn set_coverage_modulation_table(mut self, value: &'a [f32]) -> Self {
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
impl<'a> Default for VkPipelineCoverageModulationStateCreateInfoNV<'a> {
  fn default() -> VkPipelineCoverageModulationStateCreateInfoNV<'a> {
    VkPipelineCoverageModulationStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
unsafe impl<'a> RawStruct for VkPipelineCoverageModulationStateCreateInfoNV<'a> {
  type Raw = types_raw::VkPipelineCoverageModulationStateCreateInfoNV;
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
unsafe impl<'b, 'a: 'b> StructExtends<VkPipelineMultisampleStateCreateInfo<'b>>
  for VkPipelineCoverageModulationStateCreateInfoNV<'a>
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
  assert_size!(
    types_raw::VkPipelineCoverageModulationStateCreateInfoNV,
    VkPipelineCoverageModulationStateCreateInfoNV
  );
}

// feature: VK_KHR_bind_memory2

/// Structure specifying how to bind a buffer to memory
#[repr(C)]
#[cfg(feature = "VK_KHR_bind_memory2")]
pub struct VkBindBufferMemoryInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub buffer: VkBuffer,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<'a> VkBindBufferMemoryInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkBindBufferMemoryInfoKHR<'a> {
    unsafe {
      VkBindBufferMemoryInfoKHR {
        sType: VkStructureType::BIND_BUFFER_MEMORY_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
    self
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory) -> Self {
    self.memory = value;
    self
  }
  #[inline]
  pub fn set_memory_offset(mut self, value: VkDeviceSize) -> Self {
    self.memoryOffset = value;
    self
  }
  #[inline]
  pub fn buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn memory(&self) -> VkDeviceMemory {
    self.memory
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
impl<'a> Default for VkBindBufferMemoryInfoKHR<'a> {
  fn default() -> VkBindBufferMemoryInfoKHR<'a> {
    VkBindBufferMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
unsafe impl<'a> RawStruct for VkBindBufferMemoryInfoKHR<'a> {
  type Raw = types_raw::VkBindBufferMemoryInfoKHR;
}
#[cfg(feature = "VK_KHR_bind_memory2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_buffer_memory_info_khr() {
  assert_size!(
    types_raw::VkBindBufferMemoryInfoKHR,
    VkBindBufferMemoryInfoKHR
  );
}

/// Structure specifying how to bind an image to memory
#[repr(C)]
#[cfg(feature = "VK_KHR_bind_memory2")]
pub struct VkBindImageMemoryInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub image: VkImage,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl<'a> VkBindImageMemoryInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkBindImageMemoryInfoKHR<'a> {
    unsafe {
      VkBindImageMemoryInfoKHR {
        sType: VkStructureType::BIND_IMAGE_MEMORY_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
    self
  }
  #[inline]
  pub fn set_memory(mut self, value: VkDeviceMemory) -> Self {
    self.memory = value;
    self
  }
  #[inline]
  pub fn set_memory_offset(mut self, value: VkDeviceSize) -> Self {
    self.memoryOffset = value;
    self
  }
  #[inline]
  pub fn image(&self) -> VkImage {
    self.image
  }
  #[inline]
  pub fn memory(&self) -> VkDeviceMemory {
    self.memory
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
impl<'a> Default for VkBindImageMemoryInfoKHR<'a> {
  fn default() -> VkBindImageMemoryInfoKHR<'a> {
    VkBindImageMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
unsafe impl<'a> RawStruct for VkBindImageMemoryInfoKHR<'a> {
  type Raw = types_raw::VkBindImageMemoryInfoKHR;
}
#[cfg(feature = "VK_KHR_bind_memory2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_image_memory_info_khr() {
  assert_size!(
    types_raw::VkBindImageMemoryInfoKHR,
    VkBindImageMemoryInfoKHR
  );
}

// feature: VK_KHR_sampler_ycbcr_conversion

/// Structure specifying the parameters of the newly created conversion
#[repr(C)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub format: VkFormat,
  pub ycbcrModel: VkSamplerYcbcrModelConversionKHR,
  pub ycbcrRange: VkSamplerYcbcrRangeKHR,
  pub components: VkComponentMapping,
  pub xChromaOffset: VkChromaLocationKHR,
  pub yChromaOffset: VkChromaLocationKHR,
  pub chromaFilter: VkFilter,
  pub forceExplicitReconstruction: VkBool32,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'a> VkSamplerYcbcrConversionCreateInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkSamplerYcbcrConversionCreateInfoKHR<'a> {
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.forceExplicitReconstruction = value;
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
impl<'a> Default for VkSamplerYcbcrConversionCreateInfoKHR<'a> {
  fn default() -> VkSamplerYcbcrConversionCreateInfoKHR<'a> {
    VkSamplerYcbcrConversionCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'a> RawStruct for VkSamplerYcbcrConversionCreateInfoKHR<'a> {
  type Raw = types_raw::VkSamplerYcbcrConversionCreateInfoKHR;
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sampler_ycbcr_conversion_create_info_khr() {
  assert_size!(
    types_raw::VkSamplerYcbcrConversionCreateInfoKHR,
    VkSamplerYcbcrConversionCreateInfoKHR
  );
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSamplerYcbcrConversionKHR__ {}

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type VkSamplerYcbcrConversionKHR = VkNonDispatchableHandle<VkSamplerYcbcrConversionKHR__>;

/// Structure specifying Y\'CbCr conversion to a sampler or image view
#[repr(C)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub conversion: VkSamplerYcbcrConversionKHR,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'a> VkSamplerYcbcrConversionInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkSamplerYcbcrConversionInfoKHR<'a> {
    unsafe {
      VkSamplerYcbcrConversionInfoKHR {
        sType: VkStructureType::SAMPLER_YCBCR_CONVERSION_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_conversion(mut self, value: VkSamplerYcbcrConversionKHR) -> Self {
    self.conversion = value;
    self
  }
  #[inline]
  pub fn conversion(&self) -> VkSamplerYcbcrConversionKHR {
    self.conversion
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'a> Default for VkSamplerYcbcrConversionInfoKHR<'a> {
  fn default() -> VkSamplerYcbcrConversionInfoKHR<'a> {
    VkSamplerYcbcrConversionInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'a> RawStruct for VkSamplerYcbcrConversionInfoKHR<'a> {
  type Raw = types_raw::VkSamplerYcbcrConversionInfoKHR;
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'b, 'a: 'b> StructExtends<VkSamplerCreateInfo<'b>> for VkSamplerYcbcrConversionInfoKHR<'a> {
  #[inline]
  unsafe fn extend(&self, next: *const c_void) -> *const c_void {
    assert!(self.pNext.get().is_null());
    self.pNext.set(next);
    self as *const VkSamplerYcbcrConversionInfoKHR as *const c_void
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageViewCreateInfo<'b>> for VkSamplerYcbcrConversionInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkSamplerYcbcrConversionInfoKHR,
    VkSamplerYcbcrConversionInfoKHR
  );
}

/// Structure specifying how to bind an image plane to memory
#[repr(C)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkBindImagePlaneMemoryInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub planeAspect: VkImageAspectFlagBits,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'a> VkBindImagePlaneMemoryInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkBindImagePlaneMemoryInfoKHR<'a> {
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
impl<'a> Default for VkBindImagePlaneMemoryInfoKHR<'a> {
  fn default() -> VkBindImagePlaneMemoryInfoKHR<'a> {
    VkBindImagePlaneMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'a> RawStruct for VkBindImagePlaneMemoryInfoKHR<'a> {
  type Raw = types_raw::VkBindImagePlaneMemoryInfoKHR;
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'b, 'a: 'b> StructExtends<VkBindImageMemoryInfoKHR<'b>> for VkBindImagePlaneMemoryInfoKHR<'a> {
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
  assert_size!(
    types_raw::VkBindImagePlaneMemoryInfoKHR,
    VkBindImagePlaneMemoryInfoKHR
  );
}

/// Structure specifying image plane for memory requirements
#[repr(C)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkImagePlaneMemoryRequirementsInfoKHR<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub planeAspect: VkImageAspectFlagBits,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'a> VkImagePlaneMemoryRequirementsInfoKHR<'a> {
  #[inline]
  pub fn new() -> VkImagePlaneMemoryRequirementsInfoKHR<'a> {
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
impl<'a> Default for VkImagePlaneMemoryRequirementsInfoKHR<'a> {
  fn default() -> VkImagePlaneMemoryRequirementsInfoKHR<'a> {
    VkImagePlaneMemoryRequirementsInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'a> RawStruct for VkImagePlaneMemoryRequirementsInfoKHR<'a> {
  type Raw = types_raw::VkImagePlaneMemoryRequirementsInfoKHR;
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'b, 'a: 'b> StructExtends<VkImageMemoryRequirementsInfo2KHR<'b>>
  for VkImagePlaneMemoryRequirementsInfoKHR<'a>
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
  assert_size!(
    types_raw::VkImagePlaneMemoryRequirementsInfoKHR,
    VkImagePlaneMemoryRequirementsInfoKHR
  );
}

/// Structure describing Y\'CbCr conversion features that can be supported by an
/// implementation
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  sType: VkStructureType,
  pNext: *mut c_void,
  pub samplerYcbcrConversion: VkBool32,
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
    let value: VkBool32 = if value { 1 } else { 0 };
    self.samplerYcbcrConversion = value;
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
unsafe impl RawStruct for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  type Raw = types_raw::VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR;
}
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
unsafe impl<'b> StructExtends<VkDeviceCreateInfo<'b>> for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
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
  assert_size!(
    types_raw::VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR,
    VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
  );
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
unsafe impl RawStruct for VkSamplerYcbcrConversionImageFormatPropertiesKHR {
  type Raw = types_raw::VkSamplerYcbcrConversionImageFormatPropertiesKHR;
}
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
  assert_size!(
    types_raw::VkSamplerYcbcrConversionImageFormatPropertiesKHR,
    VkSamplerYcbcrConversionImageFormatPropertiesKHR
  );
}

// feature: VK_EXT_validation_cache
#[cfg(feature = "VK_EXT_validation_cache")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkValidationCacheEXT__ {}

/// Opaque handle to a validation cache object
#[cfg(feature = "VK_EXT_validation_cache")]
pub type VkValidationCacheEXT = VkNonDispatchableHandle<VkValidationCacheEXT__>;

/// Structure specifying parameters of a newly created validation cache
#[repr(C)]
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct VkValidationCacheCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub flags: VkValidationCacheCreateFlagsEXT,
  initialDataSize: usize,
  pInitialData: *const c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'a> VkValidationCacheCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkValidationCacheCreateInfoEXT<'a> {
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
  pub fn set_initial_data(mut self, value: &'a [u8]) -> Self {
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
impl<'a> Default for VkValidationCacheCreateInfoEXT<'a> {
  fn default() -> VkValidationCacheCreateInfoEXT<'a> {
    VkValidationCacheCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl<'a> RawStruct for VkValidationCacheCreateInfoEXT<'a> {
  type Raw = types_raw::VkValidationCacheCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_validation_cache")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_validation_cache_create_info_ext() {
  assert_size!(
    types_raw::VkValidationCacheCreateInfoEXT,
    VkValidationCacheCreateInfoEXT
  );
}

/// Specify validation cache to use during shader module creation
#[repr(C)]
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct VkShaderModuleValidationCacheCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub validationCache: VkValidationCacheEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'a> VkShaderModuleValidationCacheCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkShaderModuleValidationCacheCreateInfoEXT<'a> {
    unsafe {
      VkShaderModuleValidationCacheCreateInfoEXT {
        sType: VkStructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_validation_cache(mut self, value: VkValidationCacheEXT) -> Self {
    self.validationCache = value;
    self
  }
  #[inline]
  pub fn validation_cache(&self) -> VkValidationCacheEXT {
    self.validationCache
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'a> Default for VkShaderModuleValidationCacheCreateInfoEXT<'a> {
  fn default() -> VkShaderModuleValidationCacheCreateInfoEXT<'a> {
    VkShaderModuleValidationCacheCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl<'a> RawStruct for VkShaderModuleValidationCacheCreateInfoEXT<'a> {
  type Raw = types_raw::VkShaderModuleValidationCacheCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl<'b, 'a: 'b> StructExtends<VkShaderModuleCreateInfo<'b>> for VkShaderModuleValidationCacheCreateInfoEXT<'a> {
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
  assert_size!(
    types_raw::VkShaderModuleValidationCacheCreateInfoEXT,
    VkShaderModuleValidationCacheCreateInfoEXT
  );
}

// feature: VK_EXT_global_priority

/// Specify a system wide priority
#[repr(C)]
#[cfg(feature = "VK_EXT_global_priority")]
pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub globalPriority: VkQueueGlobalPriorityEXT,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_global_priority")]
impl<'a> VkDeviceQueueGlobalPriorityCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkDeviceQueueGlobalPriorityCreateInfoEXT<'a> {
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
impl<'a> Default for VkDeviceQueueGlobalPriorityCreateInfoEXT<'a> {
  fn default() -> VkDeviceQueueGlobalPriorityCreateInfoEXT<'a> {
    VkDeviceQueueGlobalPriorityCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_global_priority")]
unsafe impl<'a> RawStruct for VkDeviceQueueGlobalPriorityCreateInfoEXT<'a> {
  type Raw = types_raw::VkDeviceQueueGlobalPriorityCreateInfoEXT;
}
#[cfg(feature = "VK_EXT_global_priority")]
unsafe impl<'b, 'a: 'b> StructExtends<VkDeviceQueueCreateInfo<'b>> for VkDeviceQueueGlobalPriorityCreateInfoEXT<'a> {
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
  assert_size!(
    types_raw::VkDeviceQueueGlobalPriorityCreateInfoEXT,
    VkDeviceQueueGlobalPriorityCreateInfoEXT
  );
}

// feature: VK_EXT_external_memory_host

/// import memory from a host pointer
#[repr(C)]
#[cfg(feature = "VK_EXT_external_memory_host")]
pub struct VkImportMemoryHostPointerInfoEXT<'a> {
  sType: VkStructureType,
  pNext: Cell<*const c_void>,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pHostPointer: *mut c_void,
  _p: ::std::marker::PhantomData<&'a u8>,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl<'a> VkImportMemoryHostPointerInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkImportMemoryHostPointerInfoEXT<'a> {
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
impl<'a> Default for VkImportMemoryHostPointerInfoEXT<'a> {
  fn default() -> VkImportMemoryHostPointerInfoEXT<'a> {
    VkImportMemoryHostPointerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
unsafe impl<'a> RawStruct for VkImportMemoryHostPointerInfoEXT<'a> {
  type Raw = types_raw::VkImportMemoryHostPointerInfoEXT;
}
#[cfg(feature = "VK_EXT_external_memory_host")]
unsafe impl<'b, 'a: 'b> StructExtends<VkMemoryAllocateInfo<'b>> for VkImportMemoryHostPointerInfoEXT<'a> {
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
  assert_size!(
    types_raw::VkImportMemoryHostPointerInfoEXT,
    VkImportMemoryHostPointerInfoEXT
  );
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
unsafe impl RawStruct for VkMemoryHostPointerPropertiesEXT {
  type Raw = types_raw::VkMemoryHostPointerPropertiesEXT;
}
#[cfg(feature = "VK_EXT_external_memory_host")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_host_pointer_properties_ext() {
  assert_size!(
    types_raw::VkMemoryHostPointerPropertiesEXT,
    VkMemoryHostPointerPropertiesEXT
  );
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
unsafe impl RawStruct for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceExternalMemoryHostPropertiesEXT;
}
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
  assert_size!(
    types_raw::VkPhysicalDeviceExternalMemoryHostPropertiesEXT,
    VkPhysicalDeviceExternalMemoryHostPropertiesEXT
  );
}
