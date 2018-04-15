/* GENERATED FILE */

#![allow(non_snake_case)]

pub use types_base::*;
use AsRaw;
use RawStruct;
use enums;
use platform::*;
use types_raw;
use utils::VkDispatchableHandle;
use utils::VkNonDispatchableHandle;

// feature: VK_VERSION_1_0
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkBuffer__ {}
pub type VkBuffer = VkNonDispatchableHandle<VkBuffer__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferMemoryBarrier {
  sType: VkStructureType,
  pNext: *const c_void,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
}
impl VkBufferMemoryBarrier {
  #[inline]
  pub fn new() -> VkBufferMemoryBarrier {
    unsafe {
      VkBufferMemoryBarrier {
        sType: VkStructureType::E_BUFFER_MEMORY_BARRIER,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_src_access_mask(&self) -> VkAccessFlags {
    self.srcAccessMask
  }
  #[inline]
  pub fn get_dst_access_mask(&self) -> VkAccessFlags {
    self.dstAccessMask
  }
  #[inline]
  pub fn get_src_queue_family_index(&self) -> u32 {
    self.srcQueueFamilyIndex
  }
  #[inline]
  pub fn get_dst_queue_family_index(&self) -> u32 {
    self.dstQueueFamilyIndex
  }
  #[inline]
  pub fn get_buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn get_offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn get_size(&self) -> VkDeviceSize {
    self.size
  }
}
impl Default for VkBufferMemoryBarrier {
  fn default() -> VkBufferMemoryBarrier {
    VkBufferMemoryBarrier::new()
  }
}
unsafe impl RawStruct for VkBufferMemoryBarrier {
  type Raw = types_raw::VkBufferMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_memory_barrier() {
  assert_size!(types_raw::VkBufferMemoryBarrier, VkBufferMemoryBarrier);
}
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
  pub fn get_x(&self) -> u32 {
    self.x
  }
  #[inline]
  pub fn get_y(&self) -> u32 {
    self.y
  }
  #[inline]
  pub fn get_z(&self) -> u32 {
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
  pub fn get_index_count(&self) -> u32 {
    self.indexCount
  }
  #[inline]
  pub fn get_instance_count(&self) -> u32 {
    self.instanceCount
  }
  #[inline]
  pub fn get_first_index(&self) -> u32 {
    self.firstIndex
  }
  #[inline]
  pub fn get_vertex_offset(&self) -> i32 {
    self.vertexOffset
  }
  #[inline]
  pub fn get_first_instance(&self) -> u32 {
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
  pub fn get_vertex_count(&self) -> u32 {
    self.vertexCount
  }
  #[inline]
  pub fn get_instance_count(&self) -> u32 {
    self.instanceCount
  }
  #[inline]
  pub fn get_first_vertex(&self) -> u32 {
    self.firstVertex
  }
  #[inline]
  pub fn get_first_instance(&self) -> u32 {
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
pub type VkImage = VkNonDispatchableHandle<VkImage__>;
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
  pub fn get_aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn get_base_mip_level(&self) -> u32 {
    self.baseMipLevel
  }
  #[inline]
  pub fn get_level_count(&self) -> u32 {
    self.levelCount
  }
  #[inline]
  pub fn get_base_array_layer(&self) -> u32 {
    self.baseArrayLayer
  }
  #[inline]
  pub fn get_layer_count(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageMemoryBarrier {
  sType: VkStructureType,
  pNext: *const c_void,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
  pub oldLayout: VkImageLayout,
  pub newLayout: VkImageLayout,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub image: VkImage,
  pub subresourceRange: VkImageSubresourceRange,
}
impl VkImageMemoryBarrier {
  #[inline]
  pub fn new() -> VkImageMemoryBarrier {
    unsafe {
      VkImageMemoryBarrier {
        sType: VkStructureType::E_IMAGE_MEMORY_BARRIER,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_src_access_mask(&self) -> VkAccessFlags {
    self.srcAccessMask
  }
  #[inline]
  pub fn get_dst_access_mask(&self) -> VkAccessFlags {
    self.dstAccessMask
  }
  #[inline]
  pub fn get_old_layout(&self) -> VkImageLayout {
    self.oldLayout
  }
  #[inline]
  pub fn get_new_layout(&self) -> VkImageLayout {
    self.newLayout
  }
  #[inline]
  pub fn get_src_queue_family_index(&self) -> u32 {
    self.srcQueueFamilyIndex
  }
  #[inline]
  pub fn get_dst_queue_family_index(&self) -> u32 {
    self.dstQueueFamilyIndex
  }
  #[inline]
  pub fn get_image(&self) -> VkImage {
    self.image
  }
  #[inline]
  pub fn get_subresource_range(&self) -> VkImageSubresourceRange {
    self.subresourceRange
  }
}
impl Default for VkImageMemoryBarrier {
  fn default() -> VkImageMemoryBarrier {
    VkImageMemoryBarrier::new()
  }
}
unsafe impl RawStruct for VkImageMemoryBarrier {
  type Raw = types_raw::VkImageMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_memory_barrier() {
  assert_size!(types_raw::VkImageMemoryBarrier, VkImageMemoryBarrier);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryBarrier {
  sType: VkStructureType,
  pNext: *const c_void,
  pub srcAccessMask: VkAccessFlags,
  pub dstAccessMask: VkAccessFlags,
}
impl VkMemoryBarrier {
  #[inline]
  pub fn new() -> VkMemoryBarrier {
    unsafe {
      VkMemoryBarrier {
        sType: VkStructureType::E_MEMORY_BARRIER,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_src_access_mask(&self) -> VkAccessFlags {
    self.srcAccessMask
  }
  #[inline]
  pub fn get_dst_access_mask(&self) -> VkAccessFlags {
    self.dstAccessMask
  }
}
impl Default for VkMemoryBarrier {
  fn default() -> VkMemoryBarrier {
    VkMemoryBarrier::new()
  }
}
unsafe impl RawStruct for VkMemoryBarrier {
  type Raw = types_raw::VkMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_barrier() {
  assert_size!(types_raw::VkMemoryBarrier, VkMemoryBarrier);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkApplicationInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_APPLICATION_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_application_name(&self) -> &'a CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pApplicationName) }
  }
  #[inline]
  pub fn get_application_version(&self) -> u32 {
    self.applicationVersion
  }
  #[inline]
  pub fn get_engine_name(&self) -> &'a CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pEngineName) }
  }
  #[inline]
  pub fn get_engine_version(&self) -> u32 {
    self.engineVersion
  }
  #[inline]
  pub fn get_api_version(&self) -> u32 {
    self.apiVersion
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInstanceCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_INSTANCE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkInstanceCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_application_info(&self) -> Option<&'a VkApplicationInfo<'a>> {
    self.pApplicationInfo
  }
  #[inline]
  pub fn get_enabled_layer_count(&self) -> u32 {
    self.enabledLayerCount
  }
  #[inline]
  pub fn get_enabled_extension_count(&self) -> u32 {
    self.enabledExtensionCount
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
pub use types_raw::PFN_vkAllocationFunction;
pub use types_raw::PFN_vkReallocationFunction;
pub use types_raw::PFN_vkFreeFunction;
pub use types_raw::PFN_vkInternalAllocationNotification;
pub use types_raw::PFN_vkInternalFreeNotification;
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
  pub fn get_user_data(&self) -> *mut c_void {
    self.pUserData
  }
  #[inline]
  pub fn get_pfn_allocation(&self) -> PFN_vkAllocationFunction {
    self.pfnAllocation
  }
  #[inline]
  pub fn get_pfn_reallocation(&self) -> PFN_vkReallocationFunction {
    self.pfnReallocation
  }
  #[inline]
  pub fn get_pfn_free(&self) -> PFN_vkFreeFunction {
    self.pfnFree
  }
  #[inline]
  pub fn get_pfn_internal_allocation(&self) -> Option<PFN_vkInternalAllocationNotification> {
    self.pfnInternalAllocation
  }
  #[inline]
  pub fn get_pfn_internal_free(&self) -> Option<PFN_vkInternalFreeNotification> {
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
pub type VkInstance = VkDispatchableHandle<VkInstance__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPhysicalDevice__ {}
pub type VkPhysicalDevice = VkDispatchableHandle<VkPhysicalDevice__>;
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
  pub fn set_robust_buffer_access(mut self, value: VkBool32) -> Self {
    self.robustBufferAccess = value;
    self
  }
  #[inline]
  pub fn set_full_draw_index_uint32(mut self, value: VkBool32) -> Self {
    self.fullDrawIndexUint32 = value;
    self
  }
  #[inline]
  pub fn set_image_cube_array(mut self, value: VkBool32) -> Self {
    self.imageCubeArray = value;
    self
  }
  #[inline]
  pub fn set_independent_blend(mut self, value: VkBool32) -> Self {
    self.independentBlend = value;
    self
  }
  #[inline]
  pub fn set_geometry_shader(mut self, value: VkBool32) -> Self {
    self.geometryShader = value;
    self
  }
  #[inline]
  pub fn set_tessellation_shader(mut self, value: VkBool32) -> Self {
    self.tessellationShader = value;
    self
  }
  #[inline]
  pub fn set_sample_rate_shading(mut self, value: VkBool32) -> Self {
    self.sampleRateShading = value;
    self
  }
  #[inline]
  pub fn set_dual_src_blend(mut self, value: VkBool32) -> Self {
    self.dualSrcBlend = value;
    self
  }
  #[inline]
  pub fn set_logic_op(mut self, value: VkBool32) -> Self {
    self.logicOp = value;
    self
  }
  #[inline]
  pub fn set_multi_draw_indirect(mut self, value: VkBool32) -> Self {
    self.multiDrawIndirect = value;
    self
  }
  #[inline]
  pub fn set_draw_indirect_first_instance(mut self, value: VkBool32) -> Self {
    self.drawIndirectFirstInstance = value;
    self
  }
  #[inline]
  pub fn set_depth_clamp(mut self, value: VkBool32) -> Self {
    self.depthClamp = value;
    self
  }
  #[inline]
  pub fn set_depth_bias_clamp(mut self, value: VkBool32) -> Self {
    self.depthBiasClamp = value;
    self
  }
  #[inline]
  pub fn set_fill_mode_non_solid(mut self, value: VkBool32) -> Self {
    self.fillModeNonSolid = value;
    self
  }
  #[inline]
  pub fn set_depth_bounds(mut self, value: VkBool32) -> Self {
    self.depthBounds = value;
    self
  }
  #[inline]
  pub fn set_wide_lines(mut self, value: VkBool32) -> Self {
    self.wideLines = value;
    self
  }
  #[inline]
  pub fn set_large_points(mut self, value: VkBool32) -> Self {
    self.largePoints = value;
    self
  }
  #[inline]
  pub fn set_alpha_to_one(mut self, value: VkBool32) -> Self {
    self.alphaToOne = value;
    self
  }
  #[inline]
  pub fn set_multi_viewport(mut self, value: VkBool32) -> Self {
    self.multiViewport = value;
    self
  }
  #[inline]
  pub fn set_sampler_anisotropy(mut self, value: VkBool32) -> Self {
    self.samplerAnisotropy = value;
    self
  }
  #[inline]
  pub fn set_texture_compression_etc2(mut self, value: VkBool32) -> Self {
    self.textureCompressionETC2 = value;
    self
  }
  #[inline]
  pub fn set_texture_compression_astc_ldr(mut self, value: VkBool32) -> Self {
    self.textureCompressionASTC_LDR = value;
    self
  }
  #[inline]
  pub fn set_texture_compression_bc(mut self, value: VkBool32) -> Self {
    self.textureCompressionBC = value;
    self
  }
  #[inline]
  pub fn set_occlusion_query_precise(mut self, value: VkBool32) -> Self {
    self.occlusionQueryPrecise = value;
    self
  }
  #[inline]
  pub fn set_pipeline_statistics_query(mut self, value: VkBool32) -> Self {
    self.pipelineStatisticsQuery = value;
    self
  }
  #[inline]
  pub fn set_vertex_pipeline_stores_and_atomics(mut self, value: VkBool32) -> Self {
    self.vertexPipelineStoresAndAtomics = value;
    self
  }
  #[inline]
  pub fn set_fragment_stores_and_atomics(mut self, value: VkBool32) -> Self {
    self.fragmentStoresAndAtomics = value;
    self
  }
  #[inline]
  pub fn set_shader_tessellation_and_geometry_point_size(mut self, value: VkBool32) -> Self {
    self.shaderTessellationAndGeometryPointSize = value;
    self
  }
  #[inline]
  pub fn set_shader_image_gather_extended(mut self, value: VkBool32) -> Self {
    self.shaderImageGatherExtended = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_extended_formats(mut self, value: VkBool32) -> Self {
    self.shaderStorageImageExtendedFormats = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_multisample(mut self, value: VkBool32) -> Self {
    self.shaderStorageImageMultisample = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_read_without_format(mut self, value: VkBool32) -> Self {
    self.shaderStorageImageReadWithoutFormat = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_write_without_format(mut self, value: VkBool32) -> Self {
    self.shaderStorageImageWriteWithoutFormat = value;
    self
  }
  #[inline]
  pub fn set_shader_uniform_buffer_array_dynamic_indexing(mut self, value: VkBool32) -> Self {
    self.shaderUniformBufferArrayDynamicIndexing = value;
    self
  }
  #[inline]
  pub fn set_shader_sampled_image_array_dynamic_indexing(mut self, value: VkBool32) -> Self {
    self.shaderSampledImageArrayDynamicIndexing = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_buffer_array_dynamic_indexing(mut self, value: VkBool32) -> Self {
    self.shaderStorageBufferArrayDynamicIndexing = value;
    self
  }
  #[inline]
  pub fn set_shader_storage_image_array_dynamic_indexing(mut self, value: VkBool32) -> Self {
    self.shaderStorageImageArrayDynamicIndexing = value;
    self
  }
  #[inline]
  pub fn set_shader_clip_distance(mut self, value: VkBool32) -> Self {
    self.shaderClipDistance = value;
    self
  }
  #[inline]
  pub fn set_shader_cull_distance(mut self, value: VkBool32) -> Self {
    self.shaderCullDistance = value;
    self
  }
  #[inline]
  pub fn set_shader_float64(mut self, value: VkBool32) -> Self {
    self.shaderFloat64 = value;
    self
  }
  #[inline]
  pub fn set_shader_int64(mut self, value: VkBool32) -> Self {
    self.shaderInt64 = value;
    self
  }
  #[inline]
  pub fn set_shader_int16(mut self, value: VkBool32) -> Self {
    self.shaderInt16 = value;
    self
  }
  #[inline]
  pub fn set_shader_resource_residency(mut self, value: VkBool32) -> Self {
    self.shaderResourceResidency = value;
    self
  }
  #[inline]
  pub fn set_shader_resource_min_lod(mut self, value: VkBool32) -> Self {
    self.shaderResourceMinLod = value;
    self
  }
  #[inline]
  pub fn set_sparse_binding(mut self, value: VkBool32) -> Self {
    self.sparseBinding = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency_buffer(mut self, value: VkBool32) -> Self {
    self.sparseResidencyBuffer = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency_image2_d(mut self, value: VkBool32) -> Self {
    self.sparseResidencyImage2D = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency_image3_d(mut self, value: VkBool32) -> Self {
    self.sparseResidencyImage3D = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency2_samples(mut self, value: VkBool32) -> Self {
    self.sparseResidency2Samples = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency4_samples(mut self, value: VkBool32) -> Self {
    self.sparseResidency4Samples = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency8_samples(mut self, value: VkBool32) -> Self {
    self.sparseResidency8Samples = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency16_samples(mut self, value: VkBool32) -> Self {
    self.sparseResidency16Samples = value;
    self
  }
  #[inline]
  pub fn set_sparse_residency_aliased(mut self, value: VkBool32) -> Self {
    self.sparseResidencyAliased = value;
    self
  }
  #[inline]
  pub fn set_variable_multisample_rate(mut self, value: VkBool32) -> Self {
    self.variableMultisampleRate = value;
    self
  }
  #[inline]
  pub fn set_inherited_queries(mut self, value: VkBool32) -> Self {
    self.inheritedQueries = value;
    self
  }
  #[inline]
  pub fn get_robust_buffer_access(&self) -> VkBool32 {
    self.robustBufferAccess
  }
  #[inline]
  pub fn get_full_draw_index_uint32(&self) -> VkBool32 {
    self.fullDrawIndexUint32
  }
  #[inline]
  pub fn get_image_cube_array(&self) -> VkBool32 {
    self.imageCubeArray
  }
  #[inline]
  pub fn get_independent_blend(&self) -> VkBool32 {
    self.independentBlend
  }
  #[inline]
  pub fn get_geometry_shader(&self) -> VkBool32 {
    self.geometryShader
  }
  #[inline]
  pub fn get_tessellation_shader(&self) -> VkBool32 {
    self.tessellationShader
  }
  #[inline]
  pub fn get_sample_rate_shading(&self) -> VkBool32 {
    self.sampleRateShading
  }
  #[inline]
  pub fn get_dual_src_blend(&self) -> VkBool32 {
    self.dualSrcBlend
  }
  #[inline]
  pub fn get_logic_op(&self) -> VkBool32 {
    self.logicOp
  }
  #[inline]
  pub fn get_multi_draw_indirect(&self) -> VkBool32 {
    self.multiDrawIndirect
  }
  #[inline]
  pub fn get_draw_indirect_first_instance(&self) -> VkBool32 {
    self.drawIndirectFirstInstance
  }
  #[inline]
  pub fn get_depth_clamp(&self) -> VkBool32 {
    self.depthClamp
  }
  #[inline]
  pub fn get_depth_bias_clamp(&self) -> VkBool32 {
    self.depthBiasClamp
  }
  #[inline]
  pub fn get_fill_mode_non_solid(&self) -> VkBool32 {
    self.fillModeNonSolid
  }
  #[inline]
  pub fn get_depth_bounds(&self) -> VkBool32 {
    self.depthBounds
  }
  #[inline]
  pub fn get_wide_lines(&self) -> VkBool32 {
    self.wideLines
  }
  #[inline]
  pub fn get_large_points(&self) -> VkBool32 {
    self.largePoints
  }
  #[inline]
  pub fn get_alpha_to_one(&self) -> VkBool32 {
    self.alphaToOne
  }
  #[inline]
  pub fn get_multi_viewport(&self) -> VkBool32 {
    self.multiViewport
  }
  #[inline]
  pub fn get_sampler_anisotropy(&self) -> VkBool32 {
    self.samplerAnisotropy
  }
  #[inline]
  pub fn get_texture_compression_etc2(&self) -> VkBool32 {
    self.textureCompressionETC2
  }
  #[inline]
  pub fn get_texture_compression_astc_ldr(&self) -> VkBool32 {
    self.textureCompressionASTC_LDR
  }
  #[inline]
  pub fn get_texture_compression_bc(&self) -> VkBool32 {
    self.textureCompressionBC
  }
  #[inline]
  pub fn get_occlusion_query_precise(&self) -> VkBool32 {
    self.occlusionQueryPrecise
  }
  #[inline]
  pub fn get_pipeline_statistics_query(&self) -> VkBool32 {
    self.pipelineStatisticsQuery
  }
  #[inline]
  pub fn get_vertex_pipeline_stores_and_atomics(&self) -> VkBool32 {
    self.vertexPipelineStoresAndAtomics
  }
  #[inline]
  pub fn get_fragment_stores_and_atomics(&self) -> VkBool32 {
    self.fragmentStoresAndAtomics
  }
  #[inline]
  pub fn get_shader_tessellation_and_geometry_point_size(&self) -> VkBool32 {
    self.shaderTessellationAndGeometryPointSize
  }
  #[inline]
  pub fn get_shader_image_gather_extended(&self) -> VkBool32 {
    self.shaderImageGatherExtended
  }
  #[inline]
  pub fn get_shader_storage_image_extended_formats(&self) -> VkBool32 {
    self.shaderStorageImageExtendedFormats
  }
  #[inline]
  pub fn get_shader_storage_image_multisample(&self) -> VkBool32 {
    self.shaderStorageImageMultisample
  }
  #[inline]
  pub fn get_shader_storage_image_read_without_format(&self) -> VkBool32 {
    self.shaderStorageImageReadWithoutFormat
  }
  #[inline]
  pub fn get_shader_storage_image_write_without_format(&self) -> VkBool32 {
    self.shaderStorageImageWriteWithoutFormat
  }
  #[inline]
  pub fn get_shader_uniform_buffer_array_dynamic_indexing(&self) -> VkBool32 {
    self.shaderUniformBufferArrayDynamicIndexing
  }
  #[inline]
  pub fn get_shader_sampled_image_array_dynamic_indexing(&self) -> VkBool32 {
    self.shaderSampledImageArrayDynamicIndexing
  }
  #[inline]
  pub fn get_shader_storage_buffer_array_dynamic_indexing(&self) -> VkBool32 {
    self.shaderStorageBufferArrayDynamicIndexing
  }
  #[inline]
  pub fn get_shader_storage_image_array_dynamic_indexing(&self) -> VkBool32 {
    self.shaderStorageImageArrayDynamicIndexing
  }
  #[inline]
  pub fn get_shader_clip_distance(&self) -> VkBool32 {
    self.shaderClipDistance
  }
  #[inline]
  pub fn get_shader_cull_distance(&self) -> VkBool32 {
    self.shaderCullDistance
  }
  #[inline]
  pub fn get_shader_float64(&self) -> VkBool32 {
    self.shaderFloat64
  }
  #[inline]
  pub fn get_shader_int64(&self) -> VkBool32 {
    self.shaderInt64
  }
  #[inline]
  pub fn get_shader_int16(&self) -> VkBool32 {
    self.shaderInt16
  }
  #[inline]
  pub fn get_shader_resource_residency(&self) -> VkBool32 {
    self.shaderResourceResidency
  }
  #[inline]
  pub fn get_shader_resource_min_lod(&self) -> VkBool32 {
    self.shaderResourceMinLod
  }
  #[inline]
  pub fn get_sparse_binding(&self) -> VkBool32 {
    self.sparseBinding
  }
  #[inline]
  pub fn get_sparse_residency_buffer(&self) -> VkBool32 {
    self.sparseResidencyBuffer
  }
  #[inline]
  pub fn get_sparse_residency_image2_d(&self) -> VkBool32 {
    self.sparseResidencyImage2D
  }
  #[inline]
  pub fn get_sparse_residency_image3_d(&self) -> VkBool32 {
    self.sparseResidencyImage3D
  }
  #[inline]
  pub fn get_sparse_residency2_samples(&self) -> VkBool32 {
    self.sparseResidency2Samples
  }
  #[inline]
  pub fn get_sparse_residency4_samples(&self) -> VkBool32 {
    self.sparseResidency4Samples
  }
  #[inline]
  pub fn get_sparse_residency8_samples(&self) -> VkBool32 {
    self.sparseResidency8Samples
  }
  #[inline]
  pub fn get_sparse_residency16_samples(&self) -> VkBool32 {
    self.sparseResidency16Samples
  }
  #[inline]
  pub fn get_sparse_residency_aliased(&self) -> VkBool32 {
    self.sparseResidencyAliased
  }
  #[inline]
  pub fn get_variable_multisample_rate(&self) -> VkBool32 {
    self.variableMultisampleRate
  }
  #[inline]
  pub fn get_inherited_queries(&self) -> VkBool32 {
    self.inheritedQueries
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFormatProperties {
  pub linearTilingFeatures: VkFormatFeatureFlags,
  pub optimalTilingFeatures: VkFormatFeatureFlags,
  pub bufferFeatures: VkFormatFeatureFlags,
}
impl VkFormatProperties {
  #[inline]
  pub fn new() -> VkFormatProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_linear_tiling_features(mut self, value: VkFormatFeatureFlags) -> Self {
    self.linearTilingFeatures = value;
    self
  }
  #[inline]
  pub fn set_optimal_tiling_features(mut self, value: VkFormatFeatureFlags) -> Self {
    self.optimalTilingFeatures = value;
    self
  }
  #[inline]
  pub fn set_buffer_features(mut self, value: VkFormatFeatureFlags) -> Self {
    self.bufferFeatures = value;
    self
  }
  #[inline]
  pub fn get_linear_tiling_features(&self) -> VkFormatFeatureFlags {
    self.linearTilingFeatures
  }
  #[inline]
  pub fn get_optimal_tiling_features(&self) -> VkFormatFeatureFlags {
    self.optimalTilingFeatures
  }
  #[inline]
  pub fn get_buffer_features(&self) -> VkFormatFeatureFlags {
    self.bufferFeatures
  }
}
impl Default for VkFormatProperties {
  fn default() -> VkFormatProperties {
    VkFormatProperties::new()
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
  pub fn get_width(&self) -> u32 {
    self.width
  }
  #[inline]
  pub fn get_height(&self) -> u32 {
    self.height
  }
  #[inline]
  pub fn get_depth(&self) -> u32 {
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
  pub fn new() -> VkImageFormatProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_max_extent(mut self, value: VkExtent3D) -> Self {
    self.maxExtent = value;
    self
  }
  #[inline]
  pub fn set_max_mip_levels(mut self, value: u32) -> Self {
    self.maxMipLevels = value;
    self
  }
  #[inline]
  pub fn set_max_array_layers(mut self, value: u32) -> Self {
    self.maxArrayLayers = value;
    self
  }
  #[inline]
  pub fn set_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.sampleCounts = value;
    self
  }
  #[inline]
  pub fn set_max_resource_size(mut self, value: VkDeviceSize) -> Self {
    self.maxResourceSize = value;
    self
  }
  #[inline]
  pub fn get_max_extent(&self) -> VkExtent3D {
    self.maxExtent
  }
  #[inline]
  pub fn get_max_mip_levels(&self) -> u32 {
    self.maxMipLevels
  }
  #[inline]
  pub fn get_max_array_layers(&self) -> u32 {
    self.maxArrayLayers
  }
  #[inline]
  pub fn get_sample_counts(&self) -> VkSampleCountFlags {
    self.sampleCounts
  }
  #[inline]
  pub fn get_max_resource_size(&self) -> VkDeviceSize {
    self.maxResourceSize
  }
}
impl Default for VkImageFormatProperties {
  fn default() -> VkImageFormatProperties {
    VkImageFormatProperties::new()
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
  pub fn new() -> VkPhysicalDeviceLimits {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_max_image_dimension1_d(mut self, value: u32) -> Self {
    self.maxImageDimension1D = value;
    self
  }
  #[inline]
  pub fn set_max_image_dimension2_d(mut self, value: u32) -> Self {
    self.maxImageDimension2D = value;
    self
  }
  #[inline]
  pub fn set_max_image_dimension3_d(mut self, value: u32) -> Self {
    self.maxImageDimension3D = value;
    self
  }
  #[inline]
  pub fn set_max_image_dimension_cube(mut self, value: u32) -> Self {
    self.maxImageDimensionCube = value;
    self
  }
  #[inline]
  pub fn set_max_image_array_layers(mut self, value: u32) -> Self {
    self.maxImageArrayLayers = value;
    self
  }
  #[inline]
  pub fn set_max_texel_buffer_elements(mut self, value: u32) -> Self {
    self.maxTexelBufferElements = value;
    self
  }
  #[inline]
  pub fn set_max_uniform_buffer_range(mut self, value: u32) -> Self {
    self.maxUniformBufferRange = value;
    self
  }
  #[inline]
  pub fn set_max_storage_buffer_range(mut self, value: u32) -> Self {
    self.maxStorageBufferRange = value;
    self
  }
  #[inline]
  pub fn set_max_push_constants_size(mut self, value: u32) -> Self {
    self.maxPushConstantsSize = value;
    self
  }
  #[inline]
  pub fn set_max_memory_allocation_count(mut self, value: u32) -> Self {
    self.maxMemoryAllocationCount = value;
    self
  }
  #[inline]
  pub fn set_max_sampler_allocation_count(mut self, value: u32) -> Self {
    self.maxSamplerAllocationCount = value;
    self
  }
  #[inline]
  pub fn set_buffer_image_granularity(mut self, value: VkDeviceSize) -> Self {
    self.bufferImageGranularity = value;
    self
  }
  #[inline]
  pub fn set_sparse_address_space_size(mut self, value: VkDeviceSize) -> Self {
    self.sparseAddressSpaceSize = value;
    self
  }
  #[inline]
  pub fn set_max_bound_descriptor_sets(mut self, value: u32) -> Self {
    self.maxBoundDescriptorSets = value;
    self
  }
  #[inline]
  pub fn set_max_per_stage_descriptor_samplers(mut self, value: u32) -> Self {
    self.maxPerStageDescriptorSamplers = value;
    self
  }
  #[inline]
  pub fn set_max_per_stage_descriptor_uniform_buffers(mut self, value: u32) -> Self {
    self.maxPerStageDescriptorUniformBuffers = value;
    self
  }
  #[inline]
  pub fn set_max_per_stage_descriptor_storage_buffers(mut self, value: u32) -> Self {
    self.maxPerStageDescriptorStorageBuffers = value;
    self
  }
  #[inline]
  pub fn set_max_per_stage_descriptor_sampled_images(mut self, value: u32) -> Self {
    self.maxPerStageDescriptorSampledImages = value;
    self
  }
  #[inline]
  pub fn set_max_per_stage_descriptor_storage_images(mut self, value: u32) -> Self {
    self.maxPerStageDescriptorStorageImages = value;
    self
  }
  #[inline]
  pub fn set_max_per_stage_descriptor_input_attachments(mut self, value: u32) -> Self {
    self.maxPerStageDescriptorInputAttachments = value;
    self
  }
  #[inline]
  pub fn set_max_per_stage_resources(mut self, value: u32) -> Self {
    self.maxPerStageResources = value;
    self
  }
  #[inline]
  pub fn set_max_descriptor_set_samplers(mut self, value: u32) -> Self {
    self.maxDescriptorSetSamplers = value;
    self
  }
  #[inline]
  pub fn set_max_descriptor_set_uniform_buffers(mut self, value: u32) -> Self {
    self.maxDescriptorSetUniformBuffers = value;
    self
  }
  #[inline]
  pub fn set_max_descriptor_set_uniform_buffers_dynamic(mut self, value: u32) -> Self {
    self.maxDescriptorSetUniformBuffersDynamic = value;
    self
  }
  #[inline]
  pub fn set_max_descriptor_set_storage_buffers(mut self, value: u32) -> Self {
    self.maxDescriptorSetStorageBuffers = value;
    self
  }
  #[inline]
  pub fn set_max_descriptor_set_storage_buffers_dynamic(mut self, value: u32) -> Self {
    self.maxDescriptorSetStorageBuffersDynamic = value;
    self
  }
  #[inline]
  pub fn set_max_descriptor_set_sampled_images(mut self, value: u32) -> Self {
    self.maxDescriptorSetSampledImages = value;
    self
  }
  #[inline]
  pub fn set_max_descriptor_set_storage_images(mut self, value: u32) -> Self {
    self.maxDescriptorSetStorageImages = value;
    self
  }
  #[inline]
  pub fn set_max_descriptor_set_input_attachments(mut self, value: u32) -> Self {
    self.maxDescriptorSetInputAttachments = value;
    self
  }
  #[inline]
  pub fn set_max_vertex_input_attributes(mut self, value: u32) -> Self {
    self.maxVertexInputAttributes = value;
    self
  }
  #[inline]
  pub fn set_max_vertex_input_bindings(mut self, value: u32) -> Self {
    self.maxVertexInputBindings = value;
    self
  }
  #[inline]
  pub fn set_max_vertex_input_attribute_offset(mut self, value: u32) -> Self {
    self.maxVertexInputAttributeOffset = value;
    self
  }
  #[inline]
  pub fn set_max_vertex_input_binding_stride(mut self, value: u32) -> Self {
    self.maxVertexInputBindingStride = value;
    self
  }
  #[inline]
  pub fn set_max_vertex_output_components(mut self, value: u32) -> Self {
    self.maxVertexOutputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_tessellation_generation_level(mut self, value: u32) -> Self {
    self.maxTessellationGenerationLevel = value;
    self
  }
  #[inline]
  pub fn set_max_tessellation_patch_size(mut self, value: u32) -> Self {
    self.maxTessellationPatchSize = value;
    self
  }
  #[inline]
  pub fn set_max_tessellation_control_per_vertex_input_components(mut self, value: u32) -> Self {
    self.maxTessellationControlPerVertexInputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_tessellation_control_per_vertex_output_components(mut self, value: u32) -> Self {
    self.maxTessellationControlPerVertexOutputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_tessellation_control_per_patch_output_components(mut self, value: u32) -> Self {
    self.maxTessellationControlPerPatchOutputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_tessellation_control_total_output_components(mut self, value: u32) -> Self {
    self.maxTessellationControlTotalOutputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_tessellation_evaluation_input_components(mut self, value: u32) -> Self {
    self.maxTessellationEvaluationInputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_tessellation_evaluation_output_components(mut self, value: u32) -> Self {
    self.maxTessellationEvaluationOutputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_geometry_shader_invocations(mut self, value: u32) -> Self {
    self.maxGeometryShaderInvocations = value;
    self
  }
  #[inline]
  pub fn set_max_geometry_input_components(mut self, value: u32) -> Self {
    self.maxGeometryInputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_geometry_output_components(mut self, value: u32) -> Self {
    self.maxGeometryOutputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_geometry_output_vertices(mut self, value: u32) -> Self {
    self.maxGeometryOutputVertices = value;
    self
  }
  #[inline]
  pub fn set_max_geometry_total_output_components(mut self, value: u32) -> Self {
    self.maxGeometryTotalOutputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_fragment_input_components(mut self, value: u32) -> Self {
    self.maxFragmentInputComponents = value;
    self
  }
  #[inline]
  pub fn set_max_fragment_output_attachments(mut self, value: u32) -> Self {
    self.maxFragmentOutputAttachments = value;
    self
  }
  #[inline]
  pub fn set_max_fragment_dual_src_attachments(mut self, value: u32) -> Self {
    self.maxFragmentDualSrcAttachments = value;
    self
  }
  #[inline]
  pub fn set_max_fragment_combined_output_resources(mut self, value: u32) -> Self {
    self.maxFragmentCombinedOutputResources = value;
    self
  }
  #[inline]
  pub fn set_max_compute_shared_memory_size(mut self, value: u32) -> Self {
    self.maxComputeSharedMemorySize = value;
    self
  }
  #[inline]
  pub fn set_max_compute_work_group_count(mut self, value: [u32; 3]) -> Self {
    self.maxComputeWorkGroupCount = value;
    self
  }
  #[inline]
  pub fn set_max_compute_work_group_invocations(mut self, value: u32) -> Self {
    self.maxComputeWorkGroupInvocations = value;
    self
  }
  #[inline]
  pub fn set_max_compute_work_group_size(mut self, value: [u32; 3]) -> Self {
    self.maxComputeWorkGroupSize = value;
    self
  }
  #[inline]
  pub fn set_sub_pixel_precision_bits(mut self, value: u32) -> Self {
    self.subPixelPrecisionBits = value;
    self
  }
  #[inline]
  pub fn set_sub_texel_precision_bits(mut self, value: u32) -> Self {
    self.subTexelPrecisionBits = value;
    self
  }
  #[inline]
  pub fn set_mipmap_precision_bits(mut self, value: u32) -> Self {
    self.mipmapPrecisionBits = value;
    self
  }
  #[inline]
  pub fn set_max_draw_indexed_index_value(mut self, value: u32) -> Self {
    self.maxDrawIndexedIndexValue = value;
    self
  }
  #[inline]
  pub fn set_max_draw_indirect_count(mut self, value: u32) -> Self {
    self.maxDrawIndirectCount = value;
    self
  }
  #[inline]
  pub fn set_max_sampler_lod_bias(mut self, value: f32) -> Self {
    self.maxSamplerLodBias = value;
    self
  }
  #[inline]
  pub fn set_max_sampler_anisotropy(mut self, value: f32) -> Self {
    self.maxSamplerAnisotropy = value;
    self
  }
  #[inline]
  pub fn set_max_viewports(mut self, value: u32) -> Self {
    self.maxViewports = value;
    self
  }
  #[inline]
  pub fn set_max_viewport_dimensions(mut self, value: [u32; 2]) -> Self {
    self.maxViewportDimensions = value;
    self
  }
  #[inline]
  pub fn set_viewport_bounds_range(mut self, value: [f32; 2]) -> Self {
    self.viewportBoundsRange = value;
    self
  }
  #[inline]
  pub fn set_viewport_sub_pixel_bits(mut self, value: u32) -> Self {
    self.viewportSubPixelBits = value;
    self
  }
  #[inline]
  pub fn set_min_memory_map_alignment(mut self, value: usize) -> Self {
    self.minMemoryMapAlignment = value;
    self
  }
  #[inline]
  pub fn set_min_texel_buffer_offset_alignment(mut self, value: VkDeviceSize) -> Self {
    self.minTexelBufferOffsetAlignment = value;
    self
  }
  #[inline]
  pub fn set_min_uniform_buffer_offset_alignment(mut self, value: VkDeviceSize) -> Self {
    self.minUniformBufferOffsetAlignment = value;
    self
  }
  #[inline]
  pub fn set_min_storage_buffer_offset_alignment(mut self, value: VkDeviceSize) -> Self {
    self.minStorageBufferOffsetAlignment = value;
    self
  }
  #[inline]
  pub fn set_min_texel_offset(mut self, value: i32) -> Self {
    self.minTexelOffset = value;
    self
  }
  #[inline]
  pub fn set_max_texel_offset(mut self, value: u32) -> Self {
    self.maxTexelOffset = value;
    self
  }
  #[inline]
  pub fn set_min_texel_gather_offset(mut self, value: i32) -> Self {
    self.minTexelGatherOffset = value;
    self
  }
  #[inline]
  pub fn set_max_texel_gather_offset(mut self, value: u32) -> Self {
    self.maxTexelGatherOffset = value;
    self
  }
  #[inline]
  pub fn set_min_interpolation_offset(mut self, value: f32) -> Self {
    self.minInterpolationOffset = value;
    self
  }
  #[inline]
  pub fn set_max_interpolation_offset(mut self, value: f32) -> Self {
    self.maxInterpolationOffset = value;
    self
  }
  #[inline]
  pub fn set_sub_pixel_interpolation_offset_bits(mut self, value: u32) -> Self {
    self.subPixelInterpolationOffsetBits = value;
    self
  }
  #[inline]
  pub fn set_max_framebuffer_width(mut self, value: u32) -> Self {
    self.maxFramebufferWidth = value;
    self
  }
  #[inline]
  pub fn set_max_framebuffer_height(mut self, value: u32) -> Self {
    self.maxFramebufferHeight = value;
    self
  }
  #[inline]
  pub fn set_max_framebuffer_layers(mut self, value: u32) -> Self {
    self.maxFramebufferLayers = value;
    self
  }
  #[inline]
  pub fn set_framebuffer_color_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.framebufferColorSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_framebuffer_depth_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.framebufferDepthSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_framebuffer_stencil_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.framebufferStencilSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_framebuffer_no_attachments_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.framebufferNoAttachmentsSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_max_color_attachments(mut self, value: u32) -> Self {
    self.maxColorAttachments = value;
    self
  }
  #[inline]
  pub fn set_sampled_image_color_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.sampledImageColorSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_sampled_image_integer_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.sampledImageIntegerSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_sampled_image_depth_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.sampledImageDepthSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_sampled_image_stencil_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.sampledImageStencilSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_storage_image_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.storageImageSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_max_sample_mask_words(mut self, value: u32) -> Self {
    self.maxSampleMaskWords = value;
    self
  }
  #[inline]
  pub fn set_timestamp_compute_and_graphics(mut self, value: VkBool32) -> Self {
    self.timestampComputeAndGraphics = value;
    self
  }
  #[inline]
  pub fn set_timestamp_period(mut self, value: f32) -> Self {
    self.timestampPeriod = value;
    self
  }
  #[inline]
  pub fn set_max_clip_distances(mut self, value: u32) -> Self {
    self.maxClipDistances = value;
    self
  }
  #[inline]
  pub fn set_max_cull_distances(mut self, value: u32) -> Self {
    self.maxCullDistances = value;
    self
  }
  #[inline]
  pub fn set_max_combined_clip_and_cull_distances(mut self, value: u32) -> Self {
    self.maxCombinedClipAndCullDistances = value;
    self
  }
  #[inline]
  pub fn set_discrete_queue_priorities(mut self, value: u32) -> Self {
    self.discreteQueuePriorities = value;
    self
  }
  #[inline]
  pub fn set_point_size_range(mut self, value: [f32; 2]) -> Self {
    self.pointSizeRange = value;
    self
  }
  #[inline]
  pub fn set_line_width_range(mut self, value: [f32; 2]) -> Self {
    self.lineWidthRange = value;
    self
  }
  #[inline]
  pub fn set_point_size_granularity(mut self, value: f32) -> Self {
    self.pointSizeGranularity = value;
    self
  }
  #[inline]
  pub fn set_line_width_granularity(mut self, value: f32) -> Self {
    self.lineWidthGranularity = value;
    self
  }
  #[inline]
  pub fn set_strict_lines(mut self, value: VkBool32) -> Self {
    self.strictLines = value;
    self
  }
  #[inline]
  pub fn set_standard_sample_locations(mut self, value: VkBool32) -> Self {
    self.standardSampleLocations = value;
    self
  }
  #[inline]
  pub fn set_optimal_buffer_copy_offset_alignment(mut self, value: VkDeviceSize) -> Self {
    self.optimalBufferCopyOffsetAlignment = value;
    self
  }
  #[inline]
  pub fn set_optimal_buffer_copy_row_pitch_alignment(mut self, value: VkDeviceSize) -> Self {
    self.optimalBufferCopyRowPitchAlignment = value;
    self
  }
  #[inline]
  pub fn set_non_coherent_atom_size(mut self, value: VkDeviceSize) -> Self {
    self.nonCoherentAtomSize = value;
    self
  }
  #[inline]
  pub fn get_max_image_dimension1_d(&self) -> u32 {
    self.maxImageDimension1D
  }
  #[inline]
  pub fn get_max_image_dimension2_d(&self) -> u32 {
    self.maxImageDimension2D
  }
  #[inline]
  pub fn get_max_image_dimension3_d(&self) -> u32 {
    self.maxImageDimension3D
  }
  #[inline]
  pub fn get_max_image_dimension_cube(&self) -> u32 {
    self.maxImageDimensionCube
  }
  #[inline]
  pub fn get_max_image_array_layers(&self) -> u32 {
    self.maxImageArrayLayers
  }
  #[inline]
  pub fn get_max_texel_buffer_elements(&self) -> u32 {
    self.maxTexelBufferElements
  }
  #[inline]
  pub fn get_max_uniform_buffer_range(&self) -> u32 {
    self.maxUniformBufferRange
  }
  #[inline]
  pub fn get_max_storage_buffer_range(&self) -> u32 {
    self.maxStorageBufferRange
  }
  #[inline]
  pub fn get_max_push_constants_size(&self) -> u32 {
    self.maxPushConstantsSize
  }
  #[inline]
  pub fn get_max_memory_allocation_count(&self) -> u32 {
    self.maxMemoryAllocationCount
  }
  #[inline]
  pub fn get_max_sampler_allocation_count(&self) -> u32 {
    self.maxSamplerAllocationCount
  }
  #[inline]
  pub fn get_buffer_image_granularity(&self) -> VkDeviceSize {
    self.bufferImageGranularity
  }
  #[inline]
  pub fn get_sparse_address_space_size(&self) -> VkDeviceSize {
    self.sparseAddressSpaceSize
  }
  #[inline]
  pub fn get_max_bound_descriptor_sets(&self) -> u32 {
    self.maxBoundDescriptorSets
  }
  #[inline]
  pub fn get_max_per_stage_descriptor_samplers(&self) -> u32 {
    self.maxPerStageDescriptorSamplers
  }
  #[inline]
  pub fn get_max_per_stage_descriptor_uniform_buffers(&self) -> u32 {
    self.maxPerStageDescriptorUniformBuffers
  }
  #[inline]
  pub fn get_max_per_stage_descriptor_storage_buffers(&self) -> u32 {
    self.maxPerStageDescriptorStorageBuffers
  }
  #[inline]
  pub fn get_max_per_stage_descriptor_sampled_images(&self) -> u32 {
    self.maxPerStageDescriptorSampledImages
  }
  #[inline]
  pub fn get_max_per_stage_descriptor_storage_images(&self) -> u32 {
    self.maxPerStageDescriptorStorageImages
  }
  #[inline]
  pub fn get_max_per_stage_descriptor_input_attachments(&self) -> u32 {
    self.maxPerStageDescriptorInputAttachments
  }
  #[inline]
  pub fn get_max_per_stage_resources(&self) -> u32 {
    self.maxPerStageResources
  }
  #[inline]
  pub fn get_max_descriptor_set_samplers(&self) -> u32 {
    self.maxDescriptorSetSamplers
  }
  #[inline]
  pub fn get_max_descriptor_set_uniform_buffers(&self) -> u32 {
    self.maxDescriptorSetUniformBuffers
  }
  #[inline]
  pub fn get_max_descriptor_set_uniform_buffers_dynamic(&self) -> u32 {
    self.maxDescriptorSetUniformBuffersDynamic
  }
  #[inline]
  pub fn get_max_descriptor_set_storage_buffers(&self) -> u32 {
    self.maxDescriptorSetStorageBuffers
  }
  #[inline]
  pub fn get_max_descriptor_set_storage_buffers_dynamic(&self) -> u32 {
    self.maxDescriptorSetStorageBuffersDynamic
  }
  #[inline]
  pub fn get_max_descriptor_set_sampled_images(&self) -> u32 {
    self.maxDescriptorSetSampledImages
  }
  #[inline]
  pub fn get_max_descriptor_set_storage_images(&self) -> u32 {
    self.maxDescriptorSetStorageImages
  }
  #[inline]
  pub fn get_max_descriptor_set_input_attachments(&self) -> u32 {
    self.maxDescriptorSetInputAttachments
  }
  #[inline]
  pub fn get_max_vertex_input_attributes(&self) -> u32 {
    self.maxVertexInputAttributes
  }
  #[inline]
  pub fn get_max_vertex_input_bindings(&self) -> u32 {
    self.maxVertexInputBindings
  }
  #[inline]
  pub fn get_max_vertex_input_attribute_offset(&self) -> u32 {
    self.maxVertexInputAttributeOffset
  }
  #[inline]
  pub fn get_max_vertex_input_binding_stride(&self) -> u32 {
    self.maxVertexInputBindingStride
  }
  #[inline]
  pub fn get_max_vertex_output_components(&self) -> u32 {
    self.maxVertexOutputComponents
  }
  #[inline]
  pub fn get_max_tessellation_generation_level(&self) -> u32 {
    self.maxTessellationGenerationLevel
  }
  #[inline]
  pub fn get_max_tessellation_patch_size(&self) -> u32 {
    self.maxTessellationPatchSize
  }
  #[inline]
  pub fn get_max_tessellation_control_per_vertex_input_components(&self) -> u32 {
    self.maxTessellationControlPerVertexInputComponents
  }
  #[inline]
  pub fn get_max_tessellation_control_per_vertex_output_components(&self) -> u32 {
    self.maxTessellationControlPerVertexOutputComponents
  }
  #[inline]
  pub fn get_max_tessellation_control_per_patch_output_components(&self) -> u32 {
    self.maxTessellationControlPerPatchOutputComponents
  }
  #[inline]
  pub fn get_max_tessellation_control_total_output_components(&self) -> u32 {
    self.maxTessellationControlTotalOutputComponents
  }
  #[inline]
  pub fn get_max_tessellation_evaluation_input_components(&self) -> u32 {
    self.maxTessellationEvaluationInputComponents
  }
  #[inline]
  pub fn get_max_tessellation_evaluation_output_components(&self) -> u32 {
    self.maxTessellationEvaluationOutputComponents
  }
  #[inline]
  pub fn get_max_geometry_shader_invocations(&self) -> u32 {
    self.maxGeometryShaderInvocations
  }
  #[inline]
  pub fn get_max_geometry_input_components(&self) -> u32 {
    self.maxGeometryInputComponents
  }
  #[inline]
  pub fn get_max_geometry_output_components(&self) -> u32 {
    self.maxGeometryOutputComponents
  }
  #[inline]
  pub fn get_max_geometry_output_vertices(&self) -> u32 {
    self.maxGeometryOutputVertices
  }
  #[inline]
  pub fn get_max_geometry_total_output_components(&self) -> u32 {
    self.maxGeometryTotalOutputComponents
  }
  #[inline]
  pub fn get_max_fragment_input_components(&self) -> u32 {
    self.maxFragmentInputComponents
  }
  #[inline]
  pub fn get_max_fragment_output_attachments(&self) -> u32 {
    self.maxFragmentOutputAttachments
  }
  #[inline]
  pub fn get_max_fragment_dual_src_attachments(&self) -> u32 {
    self.maxFragmentDualSrcAttachments
  }
  #[inline]
  pub fn get_max_fragment_combined_output_resources(&self) -> u32 {
    self.maxFragmentCombinedOutputResources
  }
  #[inline]
  pub fn get_max_compute_shared_memory_size(&self) -> u32 {
    self.maxComputeSharedMemorySize
  }
  #[inline]
  pub fn get_max_compute_work_group_count(&self) -> [u32; 3] {
    self.maxComputeWorkGroupCount
  }
  #[inline]
  pub fn get_max_compute_work_group_invocations(&self) -> u32 {
    self.maxComputeWorkGroupInvocations
  }
  #[inline]
  pub fn get_max_compute_work_group_size(&self) -> [u32; 3] {
    self.maxComputeWorkGroupSize
  }
  #[inline]
  pub fn get_sub_pixel_precision_bits(&self) -> u32 {
    self.subPixelPrecisionBits
  }
  #[inline]
  pub fn get_sub_texel_precision_bits(&self) -> u32 {
    self.subTexelPrecisionBits
  }
  #[inline]
  pub fn get_mipmap_precision_bits(&self) -> u32 {
    self.mipmapPrecisionBits
  }
  #[inline]
  pub fn get_max_draw_indexed_index_value(&self) -> u32 {
    self.maxDrawIndexedIndexValue
  }
  #[inline]
  pub fn get_max_draw_indirect_count(&self) -> u32 {
    self.maxDrawIndirectCount
  }
  #[inline]
  pub fn get_max_sampler_lod_bias(&self) -> f32 {
    self.maxSamplerLodBias
  }
  #[inline]
  pub fn get_max_sampler_anisotropy(&self) -> f32 {
    self.maxSamplerAnisotropy
  }
  #[inline]
  pub fn get_max_viewports(&self) -> u32 {
    self.maxViewports
  }
  #[inline]
  pub fn get_max_viewport_dimensions(&self) -> [u32; 2] {
    self.maxViewportDimensions
  }
  #[inline]
  pub fn get_viewport_bounds_range(&self) -> [f32; 2] {
    self.viewportBoundsRange
  }
  #[inline]
  pub fn get_viewport_sub_pixel_bits(&self) -> u32 {
    self.viewportSubPixelBits
  }
  #[inline]
  pub fn get_min_memory_map_alignment(&self) -> usize {
    self.minMemoryMapAlignment
  }
  #[inline]
  pub fn get_min_texel_buffer_offset_alignment(&self) -> VkDeviceSize {
    self.minTexelBufferOffsetAlignment
  }
  #[inline]
  pub fn get_min_uniform_buffer_offset_alignment(&self) -> VkDeviceSize {
    self.minUniformBufferOffsetAlignment
  }
  #[inline]
  pub fn get_min_storage_buffer_offset_alignment(&self) -> VkDeviceSize {
    self.minStorageBufferOffsetAlignment
  }
  #[inline]
  pub fn get_min_texel_offset(&self) -> i32 {
    self.minTexelOffset
  }
  #[inline]
  pub fn get_max_texel_offset(&self) -> u32 {
    self.maxTexelOffset
  }
  #[inline]
  pub fn get_min_texel_gather_offset(&self) -> i32 {
    self.minTexelGatherOffset
  }
  #[inline]
  pub fn get_max_texel_gather_offset(&self) -> u32 {
    self.maxTexelGatherOffset
  }
  #[inline]
  pub fn get_min_interpolation_offset(&self) -> f32 {
    self.minInterpolationOffset
  }
  #[inline]
  pub fn get_max_interpolation_offset(&self) -> f32 {
    self.maxInterpolationOffset
  }
  #[inline]
  pub fn get_sub_pixel_interpolation_offset_bits(&self) -> u32 {
    self.subPixelInterpolationOffsetBits
  }
  #[inline]
  pub fn get_max_framebuffer_width(&self) -> u32 {
    self.maxFramebufferWidth
  }
  #[inline]
  pub fn get_max_framebuffer_height(&self) -> u32 {
    self.maxFramebufferHeight
  }
  #[inline]
  pub fn get_max_framebuffer_layers(&self) -> u32 {
    self.maxFramebufferLayers
  }
  #[inline]
  pub fn get_framebuffer_color_sample_counts(&self) -> VkSampleCountFlags {
    self.framebufferColorSampleCounts
  }
  #[inline]
  pub fn get_framebuffer_depth_sample_counts(&self) -> VkSampleCountFlags {
    self.framebufferDepthSampleCounts
  }
  #[inline]
  pub fn get_framebuffer_stencil_sample_counts(&self) -> VkSampleCountFlags {
    self.framebufferStencilSampleCounts
  }
  #[inline]
  pub fn get_framebuffer_no_attachments_sample_counts(&self) -> VkSampleCountFlags {
    self.framebufferNoAttachmentsSampleCounts
  }
  #[inline]
  pub fn get_max_color_attachments(&self) -> u32 {
    self.maxColorAttachments
  }
  #[inline]
  pub fn get_sampled_image_color_sample_counts(&self) -> VkSampleCountFlags {
    self.sampledImageColorSampleCounts
  }
  #[inline]
  pub fn get_sampled_image_integer_sample_counts(&self) -> VkSampleCountFlags {
    self.sampledImageIntegerSampleCounts
  }
  #[inline]
  pub fn get_sampled_image_depth_sample_counts(&self) -> VkSampleCountFlags {
    self.sampledImageDepthSampleCounts
  }
  #[inline]
  pub fn get_sampled_image_stencil_sample_counts(&self) -> VkSampleCountFlags {
    self.sampledImageStencilSampleCounts
  }
  #[inline]
  pub fn get_storage_image_sample_counts(&self) -> VkSampleCountFlags {
    self.storageImageSampleCounts
  }
  #[inline]
  pub fn get_max_sample_mask_words(&self) -> u32 {
    self.maxSampleMaskWords
  }
  #[inline]
  pub fn get_timestamp_compute_and_graphics(&self) -> VkBool32 {
    self.timestampComputeAndGraphics
  }
  #[inline]
  pub fn get_timestamp_period(&self) -> f32 {
    self.timestampPeriod
  }
  #[inline]
  pub fn get_max_clip_distances(&self) -> u32 {
    self.maxClipDistances
  }
  #[inline]
  pub fn get_max_cull_distances(&self) -> u32 {
    self.maxCullDistances
  }
  #[inline]
  pub fn get_max_combined_clip_and_cull_distances(&self) -> u32 {
    self.maxCombinedClipAndCullDistances
  }
  #[inline]
  pub fn get_discrete_queue_priorities(&self) -> u32 {
    self.discreteQueuePriorities
  }
  #[inline]
  pub fn get_point_size_range(&self) -> [f32; 2] {
    self.pointSizeRange
  }
  #[inline]
  pub fn get_line_width_range(&self) -> [f32; 2] {
    self.lineWidthRange
  }
  #[inline]
  pub fn get_point_size_granularity(&self) -> f32 {
    self.pointSizeGranularity
  }
  #[inline]
  pub fn get_line_width_granularity(&self) -> f32 {
    self.lineWidthGranularity
  }
  #[inline]
  pub fn get_strict_lines(&self) -> VkBool32 {
    self.strictLines
  }
  #[inline]
  pub fn get_standard_sample_locations(&self) -> VkBool32 {
    self.standardSampleLocations
  }
  #[inline]
  pub fn get_optimal_buffer_copy_offset_alignment(&self) -> VkDeviceSize {
    self.optimalBufferCopyOffsetAlignment
  }
  #[inline]
  pub fn get_optimal_buffer_copy_row_pitch_alignment(&self) -> VkDeviceSize {
    self.optimalBufferCopyRowPitchAlignment
  }
  #[inline]
  pub fn get_non_coherent_atom_size(&self) -> VkDeviceSize {
    self.nonCoherentAtomSize
  }
}
impl Default for VkPhysicalDeviceLimits {
  fn default() -> VkPhysicalDeviceLimits {
    VkPhysicalDeviceLimits::new()
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
  pub fn new() -> VkPhysicalDeviceSparseProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_residency_standard2_d_block_shape(mut self, value: VkBool32) -> Self {
    self.residencyStandard2DBlockShape = value;
    self
  }
  #[inline]
  pub fn set_residency_standard2_d_multisample_block_shape(mut self, value: VkBool32) -> Self {
    self.residencyStandard2DMultisampleBlockShape = value;
    self
  }
  #[inline]
  pub fn set_residency_standard3_d_block_shape(mut self, value: VkBool32) -> Self {
    self.residencyStandard3DBlockShape = value;
    self
  }
  #[inline]
  pub fn set_residency_aligned_mip_size(mut self, value: VkBool32) -> Self {
    self.residencyAlignedMipSize = value;
    self
  }
  #[inline]
  pub fn set_residency_non_resident_strict(mut self, value: VkBool32) -> Self {
    self.residencyNonResidentStrict = value;
    self
  }
  #[inline]
  pub fn get_residency_standard2_d_block_shape(&self) -> VkBool32 {
    self.residencyStandard2DBlockShape
  }
  #[inline]
  pub fn get_residency_standard2_d_multisample_block_shape(&self) -> VkBool32 {
    self.residencyStandard2DMultisampleBlockShape
  }
  #[inline]
  pub fn get_residency_standard3_d_block_shape(&self) -> VkBool32 {
    self.residencyStandard3DBlockShape
  }
  #[inline]
  pub fn get_residency_aligned_mip_size(&self) -> VkBool32 {
    self.residencyAlignedMipSize
  }
  #[inline]
  pub fn get_residency_non_resident_strict(&self) -> VkBool32 {
    self.residencyNonResidentStrict
  }
}
impl Default for VkPhysicalDeviceSparseProperties {
  fn default() -> VkPhysicalDeviceSparseProperties {
    VkPhysicalDeviceSparseProperties::new()
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
  pub fn new() -> VkPhysicalDeviceProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_api_version(mut self, value: u32) -> Self {
    self.apiVersion = value;
    self
  }
  #[inline]
  pub fn set_driver_version(mut self, value: u32) -> Self {
    self.driverVersion = value;
    self
  }
  #[inline]
  pub fn set_vendor_id(mut self, value: u32) -> Self {
    self.vendorID = value;
    self
  }
  #[inline]
  pub fn set_device_id(mut self, value: u32) -> Self {
    self.deviceID = value;
    self
  }
  #[inline]
  pub fn set_device_type(mut self, value: VkPhysicalDeviceType) -> Self {
    self.deviceType = value;
    self
  }
  #[inline]
  pub fn set_device_name(mut self, value: [c_char; enums::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize]) -> Self {
    self.deviceName = value;
    self
  }
  #[inline]
  pub fn set_pipeline_cache_uuid(mut self, value: [u8; enums::VK_UUID_SIZE as usize]) -> Self {
    self.pipelineCacheUUID = value;
    self
  }
  #[inline]
  pub fn set_limits(mut self, value: VkPhysicalDeviceLimits) -> Self {
    self.limits = value;
    self
  }
  #[inline]
  pub fn set_sparse_properties(mut self, value: VkPhysicalDeviceSparseProperties) -> Self {
    self.sparseProperties = value;
    self
  }
  #[inline]
  pub fn get_api_version(&self) -> u32 {
    self.apiVersion
  }
  #[inline]
  pub fn get_driver_version(&self) -> u32 {
    self.driverVersion
  }
  #[inline]
  pub fn get_vendor_id(&self) -> u32 {
    self.vendorID
  }
  #[inline]
  pub fn get_device_id(&self) -> u32 {
    self.deviceID
  }
  #[inline]
  pub fn get_device_type(&self) -> VkPhysicalDeviceType {
    self.deviceType
  }
  #[inline]
  pub fn get_device_name(&self) -> [c_char; enums::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize] {
    self.deviceName
  }
  #[inline]
  pub fn get_pipeline_cache_uuid(&self) -> [u8; enums::VK_UUID_SIZE as usize] {
    self.pipelineCacheUUID
  }
  #[inline]
  pub fn get_limits(&self) -> VkPhysicalDeviceLimits {
    self.limits
  }
  #[inline]
  pub fn get_sparse_properties(&self) -> VkPhysicalDeviceSparseProperties {
    self.sparseProperties
  }
}
impl Default for VkPhysicalDeviceProperties {
  fn default() -> VkPhysicalDeviceProperties {
    VkPhysicalDeviceProperties::new()
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
  pub fn new() -> VkQueueFamilyProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_queue_flags(mut self, value: VkQueueFlags) -> Self {
    self.queueFlags = value;
    self
  }
  #[inline]
  pub fn set_queue_count(mut self, value: u32) -> Self {
    self.queueCount = value;
    self
  }
  #[inline]
  pub fn set_timestamp_valid_bits(mut self, value: u32) -> Self {
    self.timestampValidBits = value;
    self
  }
  #[inline]
  pub fn set_min_image_transfer_granularity(mut self, value: VkExtent3D) -> Self {
    self.minImageTransferGranularity = value;
    self
  }
  #[inline]
  pub fn get_queue_flags(&self) -> VkQueueFlags {
    self.queueFlags
  }
  #[inline]
  pub fn get_queue_count(&self) -> u32 {
    self.queueCount
  }
  #[inline]
  pub fn get_timestamp_valid_bits(&self) -> u32 {
    self.timestampValidBits
  }
  #[inline]
  pub fn get_min_image_transfer_granularity(&self) -> VkExtent3D {
    self.minImageTransferGranularity
  }
}
impl Default for VkQueueFamilyProperties {
  fn default() -> VkQueueFamilyProperties {
    VkQueueFamilyProperties::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryType {
  pub propertyFlags: VkMemoryPropertyFlags,
  pub heapIndex: u32,
}
impl VkMemoryType {
  #[inline]
  pub fn new() -> VkMemoryType {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_property_flags(mut self, value: VkMemoryPropertyFlags) -> Self {
    self.propertyFlags = value;
    self
  }
  #[inline]
  pub fn set_heap_index(mut self, value: u32) -> Self {
    self.heapIndex = value;
    self
  }
  #[inline]
  pub fn get_property_flags(&self) -> VkMemoryPropertyFlags {
    self.propertyFlags
  }
  #[inline]
  pub fn get_heap_index(&self) -> u32 {
    self.heapIndex
  }
}
impl Default for VkMemoryType {
  fn default() -> VkMemoryType {
    VkMemoryType::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHeap {
  pub size: VkDeviceSize,
  pub flags: VkMemoryHeapFlags,
}
impl VkMemoryHeap {
  #[inline]
  pub fn new() -> VkMemoryHeap {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_size(mut self, value: VkDeviceSize) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkMemoryHeapFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn get_size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn get_flags(&self) -> VkMemoryHeapFlags {
    self.flags
  }
}
impl Default for VkMemoryHeap {
  fn default() -> VkMemoryHeap {
    VkMemoryHeap::new()
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
  pub fn new() -> VkPhysicalDeviceMemoryProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_memory_type_count(mut self, value: u32) -> Self {
    self.memoryTypeCount = value;
    self
  }
  #[inline]
  pub fn set_memory_types(mut self, value: [VkMemoryType; enums::VK_MAX_MEMORY_TYPES as usize]) -> Self {
    self.memoryTypes = value;
    self
  }
  #[inline]
  pub fn set_memory_heap_count(mut self, value: u32) -> Self {
    self.memoryHeapCount = value;
    self
  }
  #[inline]
  pub fn set_memory_heaps(mut self, value: [VkMemoryHeap; enums::VK_MAX_MEMORY_HEAPS as usize]) -> Self {
    self.memoryHeaps = value;
    self
  }
  #[inline]
  pub fn get_memory_type_count(&self) -> u32 {
    self.memoryTypeCount
  }
  #[inline]
  pub fn get_memory_types(&self) -> [VkMemoryType; enums::VK_MAX_MEMORY_TYPES as usize] {
    self.memoryTypes
  }
  #[inline]
  pub fn get_memory_heap_count(&self) -> u32 {
    self.memoryHeapCount
  }
  #[inline]
  pub fn get_memory_heaps(&self) -> [VkMemoryHeap; enums::VK_MAX_MEMORY_HEAPS as usize] {
    self.memoryHeaps
  }
}
impl Default for VkPhysicalDeviceMemoryProperties {
  fn default() -> VkPhysicalDeviceMemoryProperties {
    VkPhysicalDeviceMemoryProperties::new()
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
pub use types_raw::PFN_vkVoidFunction;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDevice__ {}
pub type VkDevice = VkDispatchableHandle<VkDevice__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceQueueCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEVICE_QUEUE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkDeviceQueueCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_queue_family_index(&self) -> u32 {
    self.queueFamilyIndex
  }
  #[inline]
  pub fn get_queue_count(&self) -> u32 {
    self.queueCount
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEVICE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkDeviceCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_queue_create_info_count(&self) -> u32 {
    self.queueCreateInfoCount
  }
  #[inline]
  pub fn get_enabled_layer_count(&self) -> u32 {
    self.enabledLayerCount
  }
  #[inline]
  pub fn get_enabled_extension_count(&self) -> u32 {
    self.enabledExtensionCount
  }
  #[inline]
  pub fn get_enabled_features(&self) -> Option<&'a VkPhysicalDeviceFeatures> {
    self.pEnabledFeatures
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtensionProperties {
  pub extensionName: [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize],
  pub specVersion: u32,
}
impl VkExtensionProperties {
  #[inline]
  pub fn new() -> VkExtensionProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_extension_name(mut self, value: [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize]) -> Self {
    self.extensionName = value;
    self
  }
  #[inline]
  pub fn set_spec_version(mut self, value: u32) -> Self {
    self.specVersion = value;
    self
  }
  #[inline]
  pub fn get_extension_name(&self) -> [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize] {
    self.extensionName
  }
  #[inline]
  pub fn get_spec_version(&self) -> u32 {
    self.specVersion
  }
}
impl Default for VkExtensionProperties {
  fn default() -> VkExtensionProperties {
    VkExtensionProperties::new()
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
  pub fn new() -> VkLayerProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_layer_name(mut self, value: [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize]) -> Self {
    self.layerName = value;
    self
  }
  #[inline]
  pub fn set_spec_version(mut self, value: u32) -> Self {
    self.specVersion = value;
    self
  }
  #[inline]
  pub fn set_implementation_version(mut self, value: u32) -> Self {
    self.implementationVersion = value;
    self
  }
  #[inline]
  pub fn set_description(mut self, value: [c_char; enums::VK_MAX_DESCRIPTION_SIZE as usize]) -> Self {
    self.description = value;
    self
  }
  #[inline]
  pub fn get_layer_name(&self) -> [c_char; enums::VK_MAX_EXTENSION_NAME_SIZE as usize] {
    self.layerName
  }
  #[inline]
  pub fn get_spec_version(&self) -> u32 {
    self.specVersion
  }
  #[inline]
  pub fn get_implementation_version(&self) -> u32 {
    self.implementationVersion
  }
  #[inline]
  pub fn get_description(&self) -> [c_char; enums::VK_MAX_DESCRIPTION_SIZE as usize] {
    self.description
  }
}
impl Default for VkLayerProperties {
  fn default() -> VkLayerProperties {
    VkLayerProperties::new()
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
pub type VkQueue = VkDispatchableHandle<VkQueue__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSemaphore__ {}
pub type VkSemaphore = VkNonDispatchableHandle<VkSemaphore__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkCommandBuffer__ {}
pub type VkCommandBuffer = VkDispatchableHandle<VkCommandBuffer__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubmitInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_SUBMIT_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_wait_semaphore_count(&self) -> u32 {
    self.waitSemaphoreCount
  }
  #[inline]
  pub fn get_command_buffer_count(&self) -> u32 {
    self.commandBufferCount
  }
  #[inline]
  pub fn get_signal_semaphore_count(&self) -> u32 {
    self.signalSemaphoreCount
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
pub type VkFence = VkNonDispatchableHandle<VkFence__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryAllocateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeIndex: u32,
}
impl VkMemoryAllocateInfo {
  #[inline]
  pub fn new() -> VkMemoryAllocateInfo {
    unsafe {
      VkMemoryAllocateInfo {
        sType: VkStructureType::E_MEMORY_ALLOCATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_allocation_size(&self) -> VkDeviceSize {
    self.allocationSize
  }
  #[inline]
  pub fn get_memory_type_index(&self) -> u32 {
    self.memoryTypeIndex
  }
}
impl Default for VkMemoryAllocateInfo {
  fn default() -> VkMemoryAllocateInfo {
    VkMemoryAllocateInfo::new()
  }
}
unsafe impl RawStruct for VkMemoryAllocateInfo {
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
pub type VkDeviceMemory = VkNonDispatchableHandle<VkDeviceMemory__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMappedMemoryRange {
  sType: VkStructureType,
  pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
}
impl VkMappedMemoryRange {
  #[inline]
  pub fn new() -> VkMappedMemoryRange {
    unsafe {
      VkMappedMemoryRange {
        sType: VkStructureType::E_MAPPED_MEMORY_RANGE,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory(&self) -> VkDeviceMemory {
    self.memory
  }
  #[inline]
  pub fn get_offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn get_size(&self) -> VkDeviceSize {
    self.size
  }
}
impl Default for VkMappedMemoryRange {
  fn default() -> VkMappedMemoryRange {
    VkMappedMemoryRange::new()
  }
}
unsafe impl RawStruct for VkMappedMemoryRange {
  type Raw = types_raw::VkMappedMemoryRange;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_mapped_memory_range() {
  assert_size!(types_raw::VkMappedMemoryRange, VkMappedMemoryRange);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryRequirements {
  pub size: VkDeviceSize,
  pub alignment: VkDeviceSize,
  pub memoryTypeBits: u32,
}
impl VkMemoryRequirements {
  #[inline]
  pub fn new() -> VkMemoryRequirements {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_size(mut self, value: VkDeviceSize) -> Self {
    self.size = value;
    self
  }
  #[inline]
  pub fn set_alignment(mut self, value: VkDeviceSize) -> Self {
    self.alignment = value;
    self
  }
  #[inline]
  pub fn set_memory_type_bits(mut self, value: u32) -> Self {
    self.memoryTypeBits = value;
    self
  }
  #[inline]
  pub fn get_size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn get_alignment(&self) -> VkDeviceSize {
    self.alignment
  }
  #[inline]
  pub fn get_memory_type_bits(&self) -> u32 {
    self.memoryTypeBits
  }
}
impl Default for VkMemoryRequirements {
  fn default() -> VkMemoryRequirements {
    VkMemoryRequirements::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties {
  pub aspectMask: VkImageAspectFlags,
  pub imageGranularity: VkExtent3D,
  pub flags: VkSparseImageFormatFlags,
}
impl VkSparseImageFormatProperties {
  #[inline]
  pub fn new() -> VkSparseImageFormatProperties {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_aspect_mask(mut self, value: VkImageAspectFlags) -> Self {
    self.aspectMask = value;
    self
  }
  #[inline]
  pub fn set_image_granularity(mut self, value: VkExtent3D) -> Self {
    self.imageGranularity = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSparseImageFormatFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn get_aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn get_image_granularity(&self) -> VkExtent3D {
    self.imageGranularity
  }
  #[inline]
  pub fn get_flags(&self) -> VkSparseImageFormatFlags {
    self.flags
  }
}
impl Default for VkSparseImageFormatProperties {
  fn default() -> VkSparseImageFormatProperties {
    VkSparseImageFormatProperties::new()
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
  pub fn new() -> VkSparseImageMemoryRequirements {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_format_properties(mut self, value: VkSparseImageFormatProperties) -> Self {
    self.formatProperties = value;
    self
  }
  #[inline]
  pub fn set_image_mip_tail_first_lod(mut self, value: u32) -> Self {
    self.imageMipTailFirstLod = value;
    self
  }
  #[inline]
  pub fn set_image_mip_tail_size(mut self, value: VkDeviceSize) -> Self {
    self.imageMipTailSize = value;
    self
  }
  #[inline]
  pub fn set_image_mip_tail_offset(mut self, value: VkDeviceSize) -> Self {
    self.imageMipTailOffset = value;
    self
  }
  #[inline]
  pub fn set_image_mip_tail_stride(mut self, value: VkDeviceSize) -> Self {
    self.imageMipTailStride = value;
    self
  }
  #[inline]
  pub fn get_format_properties(&self) -> VkSparseImageFormatProperties {
    self.formatProperties
  }
  #[inline]
  pub fn get_image_mip_tail_first_lod(&self) -> u32 {
    self.imageMipTailFirstLod
  }
  #[inline]
  pub fn get_image_mip_tail_size(&self) -> VkDeviceSize {
    self.imageMipTailSize
  }
  #[inline]
  pub fn get_image_mip_tail_offset(&self) -> VkDeviceSize {
    self.imageMipTailOffset
  }
  #[inline]
  pub fn get_image_mip_tail_stride(&self) -> VkDeviceSize {
    self.imageMipTailStride
  }
}
impl Default for VkSparseImageMemoryRequirements {
  fn default() -> VkSparseImageMemoryRequirements {
    VkSparseImageMemoryRequirements::new()
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
  pub fn get_resource_offset(&self) -> VkDeviceSize {
    self.resourceOffset
  }
  #[inline]
  pub fn get_size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn get_memory(&self) -> Option<VkDeviceMemory> {
    self.memory
  }
  #[inline]
  pub fn get_memory_offset(&self) -> VkDeviceSize {
    self.memoryOffset
  }
  #[inline]
  pub fn get_flags(&self) -> VkSparseMemoryBindFlags {
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
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn get_bind_count(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_image(&self) -> VkImage {
    self.image
  }
  #[inline]
  pub fn get_bind_count(&self) -> u32 {
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
  pub fn get_aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn get_mip_level(&self) -> u32 {
    self.mipLevel
  }
  #[inline]
  pub fn get_array_layer(&self) -> u32 {
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
  pub fn get_x(&self) -> i32 {
    self.x
  }
  #[inline]
  pub fn get_y(&self) -> i32 {
    self.y
  }
  #[inline]
  pub fn get_z(&self) -> i32 {
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
  pub fn get_subresource(&self) -> VkImageSubresource {
    self.subresource
  }
  #[inline]
  pub fn get_offset(&self) -> VkOffset3D {
    self.offset
  }
  #[inline]
  pub fn get_extent(&self) -> VkExtent3D {
    self.extent
  }
  #[inline]
  pub fn get_memory(&self) -> Option<VkDeviceMemory> {
    self.memory
  }
  #[inline]
  pub fn get_memory_offset(&self) -> VkDeviceSize {
    self.memoryOffset
  }
  #[inline]
  pub fn get_flags(&self) -> VkSparseMemoryBindFlags {
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
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_image(&self) -> VkImage {
    self.image
  }
  #[inline]
  pub fn get_bind_count(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindSparseInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_BIND_SPARSE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_wait_semaphore_count(&self) -> u32 {
    self.waitSemaphoreCount
  }
  #[inline]
  pub fn get_buffer_bind_count(&self) -> u32 {
    self.bufferBindCount
  }
  #[inline]
  pub fn get_image_opaque_bind_count(&self) -> u32 {
    self.imageOpaqueBindCount
  }
  #[inline]
  pub fn get_image_bind_count(&self) -> u32 {
    self.imageBindCount
  }
  #[inline]
  pub fn get_signal_semaphore_count(&self) -> u32 {
    self.signalSemaphoreCount
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFenceCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkFenceCreateFlags,
}
impl VkFenceCreateInfo {
  #[inline]
  pub fn new() -> VkFenceCreateInfo {
    unsafe {
      VkFenceCreateInfo {
        sType: VkStructureType::E_FENCE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkFenceCreateFlags {
    self.flags
  }
}
impl Default for VkFenceCreateInfo {
  fn default() -> VkFenceCreateInfo {
    VkFenceCreateInfo::new()
  }
}
unsafe impl RawStruct for VkFenceCreateInfo {
  type Raw = types_raw::VkFenceCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_create_info() {
  assert_size!(types_raw::VkFenceCreateInfo, VkFenceCreateInfo);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkSemaphoreCreateFlags,
}
impl VkSemaphoreCreateInfo {
  #[inline]
  pub fn new() -> VkSemaphoreCreateInfo {
    unsafe {
      VkSemaphoreCreateInfo {
        sType: VkStructureType::E_SEMAPHORE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkSemaphoreCreateFlags {
    self.flags
  }
}
impl Default for VkSemaphoreCreateInfo {
  fn default() -> VkSemaphoreCreateInfo {
    VkSemaphoreCreateInfo::new()
  }
}
unsafe impl RawStruct for VkSemaphoreCreateInfo {
  type Raw = types_raw::VkSemaphoreCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_create_info() {
  assert_size!(types_raw::VkSemaphoreCreateInfo, VkSemaphoreCreateInfo);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkEventCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkEventCreateFlags,
}
impl VkEventCreateInfo {
  #[inline]
  pub fn new() -> VkEventCreateInfo {
    unsafe {
      VkEventCreateInfo {
        sType: VkStructureType::E_EVENT_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkEventCreateFlags {
    self.flags
  }
}
impl Default for VkEventCreateInfo {
  fn default() -> VkEventCreateInfo {
    VkEventCreateInfo::new()
  }
}
unsafe impl RawStruct for VkEventCreateInfo {
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
pub type VkEvent = VkNonDispatchableHandle<VkEvent__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueryPoolCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkQueryPoolCreateFlags,
  pub queryType: VkQueryType,
  pub queryCount: u32,
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}
impl VkQueryPoolCreateInfo {
  #[inline]
  pub fn new() -> VkQueryPoolCreateInfo {
    unsafe {
      VkQueryPoolCreateInfo {
        sType: VkStructureType::E_QUERY_POOL_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkQueryPoolCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_query_type(&self) -> VkQueryType {
    self.queryType
  }
  #[inline]
  pub fn get_query_count(&self) -> u32 {
    self.queryCount
  }
  #[inline]
  pub fn get_pipeline_statistics(&self) -> VkQueryPipelineStatisticFlags {
    self.pipelineStatistics
  }
}
impl Default for VkQueryPoolCreateInfo {
  fn default() -> VkQueryPoolCreateInfo {
    VkQueryPoolCreateInfo::new()
  }
}
unsafe impl RawStruct for VkQueryPoolCreateInfo {
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
pub type VkQueryPool = VkNonDispatchableHandle<VkQueryPool__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_BUFFER_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkBufferCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn get_usage(&self) -> VkBufferUsageFlags {
    self.usage
  }
  #[inline]
  pub fn get_sharing_mode(&self) -> VkSharingMode {
    self.sharingMode
  }
  #[inline]
  pub fn get_queue_family_index_count(&self) -> u32 {
    self.queueFamilyIndexCount
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferViewCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkBufferViewCreateFlags,
  pub buffer: VkBuffer,
  pub format: VkFormat,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
}
impl VkBufferViewCreateInfo {
  #[inline]
  pub fn new() -> VkBufferViewCreateInfo {
    unsafe {
      VkBufferViewCreateInfo {
        sType: VkStructureType::E_BUFFER_VIEW_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkBufferViewCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn get_range(&self) -> VkDeviceSize {
    self.range
  }
}
impl Default for VkBufferViewCreateInfo {
  fn default() -> VkBufferViewCreateInfo {
    VkBufferViewCreateInfo::new()
  }
}
unsafe impl RawStruct for VkBufferViewCreateInfo {
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
pub type VkBufferView = VkNonDispatchableHandle<VkBufferView__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_IMAGE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkImageCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_image_type(&self) -> VkImageType {
    self.imageType
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_extent(&self) -> VkExtent3D {
    self.extent
  }
  #[inline]
  pub fn get_mip_levels(&self) -> u32 {
    self.mipLevels
  }
  #[inline]
  pub fn get_array_layers(&self) -> u32 {
    self.arrayLayers
  }
  #[inline]
  pub fn get_samples(&self) -> VkSampleCountFlagBits {
    self.samples
  }
  #[inline]
  pub fn get_tiling(&self) -> VkImageTiling {
    self.tiling
  }
  #[inline]
  pub fn get_usage(&self) -> VkImageUsageFlags {
    self.usage
  }
  #[inline]
  pub fn get_sharing_mode(&self) -> VkSharingMode {
    self.sharingMode
  }
  #[inline]
  pub fn get_queue_family_index_count(&self) -> u32 {
    self.queueFamilyIndexCount
  }
  #[inline]
  pub fn get_initial_layout(&self) -> VkImageLayout {
    self.initialLayout
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
  pub fn new() -> VkSubresourceLayout {
    unsafe { ::std::mem::zeroed() }
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
  pub fn set_row_pitch(mut self, value: VkDeviceSize) -> Self {
    self.rowPitch = value;
    self
  }
  #[inline]
  pub fn set_array_pitch(mut self, value: VkDeviceSize) -> Self {
    self.arrayPitch = value;
    self
  }
  #[inline]
  pub fn set_depth_pitch(mut self, value: VkDeviceSize) -> Self {
    self.depthPitch = value;
    self
  }
  #[inline]
  pub fn get_offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn get_size(&self) -> VkDeviceSize {
    self.size
  }
  #[inline]
  pub fn get_row_pitch(&self) -> VkDeviceSize {
    self.rowPitch
  }
  #[inline]
  pub fn get_array_pitch(&self) -> VkDeviceSize {
    self.arrayPitch
  }
  #[inline]
  pub fn get_depth_pitch(&self) -> VkDeviceSize {
    self.depthPitch
  }
}
impl Default for VkSubresourceLayout {
  fn default() -> VkSubresourceLayout {
    VkSubresourceLayout::new()
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
  pub fn get_r(&self) -> VkComponentSwizzle {
    self.r
  }
  #[inline]
  pub fn get_g(&self) -> VkComponentSwizzle {
    self.g
  }
  #[inline]
  pub fn get_b(&self) -> VkComponentSwizzle {
    self.b
  }
  #[inline]
  pub fn get_a(&self) -> VkComponentSwizzle {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub viewType: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresourceRange: VkImageSubresourceRange,
}
impl VkImageViewCreateInfo {
  #[inline]
  pub fn new() -> VkImageViewCreateInfo {
    unsafe {
      VkImageViewCreateInfo {
        sType: VkStructureType::E_IMAGE_VIEW_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkImageViewCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_image(&self) -> VkImage {
    self.image
  }
  #[inline]
  pub fn get_view_type(&self) -> VkImageViewType {
    self.viewType
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_components(&self) -> VkComponentMapping {
    self.components
  }
  #[inline]
  pub fn get_subresource_range(&self) -> VkImageSubresourceRange {
    self.subresourceRange
  }
}
impl Default for VkImageViewCreateInfo {
  fn default() -> VkImageViewCreateInfo {
    VkImageViewCreateInfo::new()
  }
}
unsafe impl RawStruct for VkImageViewCreateInfo {
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
pub type VkImageView = VkNonDispatchableHandle<VkImageView__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShaderModuleCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_SHADER_MODULE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkShaderModuleCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_code_size(&self) -> usize {
    self.codeSize
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
pub type VkShaderModule = VkNonDispatchableHandle<VkShaderModule__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineCacheCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_CACHE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineCacheCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_initial_data_size(&self) -> usize {
    self.initialDataSize
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
pub type VkPipelineCache = VkNonDispatchableHandle<VkPipelineCache__>;
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
  pub fn get_constant_id(&self) -> u32 {
    self.constantID
  }
  #[inline]
  pub fn get_offset(&self) -> u32 {
    self.offset
  }
  #[inline]
  pub fn get_size(&self) -> usize {
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
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_map_entry_count(&self) -> u32 {
    self.mapEntryCount
  }
  #[inline]
  pub fn get_data_size(&self) -> usize {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineShaderStageCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_SHADER_STAGE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineShaderStageCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_stage(&self) -> VkShaderStageFlagBits {
    self.stage
  }
  #[inline]
  pub fn get_module(&self) -> VkShaderModule {
    self.module
  }
  #[inline]
  pub fn get_name(&self) -> &'a CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pName) }
  }
  #[inline]
  pub fn get_specialization_info(&self) -> Option<&'a VkSpecializationInfo<'a>> {
    self.pSpecializationInfo
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
  pub fn get_binding(&self) -> u32 {
    self.binding
  }
  #[inline]
  pub fn get_stride(&self) -> u32 {
    self.stride
  }
  #[inline]
  pub fn get_input_rate(&self) -> VkVertexInputRate {
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
  pub fn get_location(&self) -> u32 {
    self.location
  }
  #[inline]
  pub fn get_binding(&self) -> u32 {
    self.binding
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_offset(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineVertexInputStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineVertexInputStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_vertex_binding_description_count(&self) -> u32 {
    self.vertexBindingDescriptionCount
  }
  #[inline]
  pub fn get_vertex_attribute_description_count(&self) -> u32 {
    self.vertexAttributeDescriptionCount
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  pub primitiveRestartEnable: VkBool32,
}
impl VkPipelineInputAssemblyStateCreateInfo {
  #[inline]
  pub fn new() -> VkPipelineInputAssemblyStateCreateInfo {
    unsafe {
      VkPipelineInputAssemblyStateCreateInfo {
        sType: VkStructureType::E_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
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
  pub fn set_primitive_restart_enable(mut self, value: VkBool32) -> Self {
    self.primitiveRestartEnable = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineInputAssemblyStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_topology(&self) -> VkPrimitiveTopology {
    self.topology
  }
  #[inline]
  pub fn get_primitive_restart_enable(&self) -> VkBool32 {
    self.primitiveRestartEnable
  }
}
impl Default for VkPipelineInputAssemblyStateCreateInfo {
  fn default() -> VkPipelineInputAssemblyStateCreateInfo {
    VkPipelineInputAssemblyStateCreateInfo::new()
  }
}
unsafe impl RawStruct for VkPipelineInputAssemblyStateCreateInfo {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineTessellationStateCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patchControlPoints: u32,
}
impl VkPipelineTessellationStateCreateInfo {
  #[inline]
  pub fn new() -> VkPipelineTessellationStateCreateInfo {
    unsafe {
      VkPipelineTessellationStateCreateInfo {
        sType: VkStructureType::E_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineTessellationStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_patch_control_points(&self) -> u32 {
    self.patchControlPoints
  }
}
impl Default for VkPipelineTessellationStateCreateInfo {
  fn default() -> VkPipelineTessellationStateCreateInfo {
    VkPipelineTessellationStateCreateInfo::new()
  }
}
unsafe impl RawStruct for VkPipelineTessellationStateCreateInfo {
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
  pub fn get_x(&self) -> f32 {
    self.x
  }
  #[inline]
  pub fn get_y(&self) -> f32 {
    self.y
  }
  #[inline]
  pub fn get_width(&self) -> f32 {
    self.width
  }
  #[inline]
  pub fn get_height(&self) -> f32 {
    self.height
  }
  #[inline]
  pub fn get_min_depth(&self) -> f32 {
    self.minDepth
  }
  #[inline]
  pub fn get_max_depth(&self) -> f32 {
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
  pub fn get_x(&self) -> i32 {
    self.x
  }
  #[inline]
  pub fn get_y(&self) -> i32 {
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
  pub fn get_width(&self) -> u32 {
    self.width
  }
  #[inline]
  pub fn get_height(&self) -> u32 {
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
  pub fn get_offset(&self) -> VkOffset2D {
    self.offset
  }
  #[inline]
  pub fn get_extent(&self) -> VkExtent2D {
    self.extent
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineViewportStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineViewportStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_viewport_count(&self) -> u32 {
    self.viewportCount
  }
  #[inline]
  pub fn get_scissor_count(&self) -> u32 {
    self.scissorCount
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineRasterizationStateCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
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
impl VkPipelineRasterizationStateCreateInfo {
  #[inline]
  pub fn new() -> VkPipelineRasterizationStateCreateInfo {
    unsafe {
      VkPipelineRasterizationStateCreateInfo {
        sType: VkStructureType::E_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
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
  pub fn set_depth_clamp_enable(mut self, value: VkBool32) -> Self {
    self.depthClampEnable = value;
    self
  }
  #[inline]
  pub fn set_rasterizer_discard_enable(mut self, value: VkBool32) -> Self {
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
  pub fn set_depth_bias_enable(mut self, value: VkBool32) -> Self {
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineRasterizationStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_depth_clamp_enable(&self) -> VkBool32 {
    self.depthClampEnable
  }
  #[inline]
  pub fn get_rasterizer_discard_enable(&self) -> VkBool32 {
    self.rasterizerDiscardEnable
  }
  #[inline]
  pub fn get_polygon_mode(&self) -> VkPolygonMode {
    self.polygonMode
  }
  #[inline]
  pub fn get_cull_mode(&self) -> VkCullModeFlags {
    self.cullMode
  }
  #[inline]
  pub fn get_front_face(&self) -> VkFrontFace {
    self.frontFace
  }
  #[inline]
  pub fn get_depth_bias_enable(&self) -> VkBool32 {
    self.depthBiasEnable
  }
  #[inline]
  pub fn get_depth_bias_constant_factor(&self) -> f32 {
    self.depthBiasConstantFactor
  }
  #[inline]
  pub fn get_depth_bias_clamp(&self) -> f32 {
    self.depthBiasClamp
  }
  #[inline]
  pub fn get_depth_bias_slope_factor(&self) -> f32 {
    self.depthBiasSlopeFactor
  }
  #[inline]
  pub fn get_line_width(&self) -> f32 {
    self.lineWidth
  }
}
impl Default for VkPipelineRasterizationStateCreateInfo {
  fn default() -> VkPipelineRasterizationStateCreateInfo {
    VkPipelineRasterizationStateCreateInfo::new()
  }
}
unsafe impl RawStruct for VkPipelineRasterizationStateCreateInfo {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineMultisampleStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
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
  pub fn set_sample_shading_enable(mut self, value: VkBool32) -> Self {
    self.sampleShadingEnable = value;
    self
  }
  #[inline]
  pub fn set_min_sample_shading(mut self, value: f32) -> Self {
    self.minSampleShading = value;
    self
  }
  #[inline]
  pub fn set_alpha_to_coverage_enable(mut self, value: VkBool32) -> Self {
    self.alphaToCoverageEnable = value;
    self
  }
  #[inline]
  pub fn set_alpha_to_one_enable(mut self, value: VkBool32) -> Self {
    self.alphaToOneEnable = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineMultisampleStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_rasterization_samples(&self) -> VkSampleCountFlagBits {
    self.rasterizationSamples
  }
  #[inline]
  pub fn get_sample_shading_enable(&self) -> VkBool32 {
    self.sampleShadingEnable
  }
  #[inline]
  pub fn get_min_sample_shading(&self) -> f32 {
    self.minSampleShading
  }
  #[inline]
  pub fn get_alpha_to_coverage_enable(&self) -> VkBool32 {
    self.alphaToCoverageEnable
  }
  #[inline]
  pub fn get_alpha_to_one_enable(&self) -> VkBool32 {
    self.alphaToOneEnable
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
  pub fn get_fail_op(&self) -> VkStencilOp {
    self.failOp
  }
  #[inline]
  pub fn get_pass_op(&self) -> VkStencilOp {
    self.passOp
  }
  #[inline]
  pub fn get_depth_fail_op(&self) -> VkStencilOp {
    self.depthFailOp
  }
  #[inline]
  pub fn get_compare_op(&self) -> VkCompareOp {
    self.compareOp
  }
  #[inline]
  pub fn get_compare_mask(&self) -> u32 {
    self.compareMask
  }
  #[inline]
  pub fn get_write_mask(&self) -> u32 {
    self.writeMask
  }
  #[inline]
  pub fn get_reference(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDepthStencilStateCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
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
impl VkPipelineDepthStencilStateCreateInfo {
  #[inline]
  pub fn new() -> VkPipelineDepthStencilStateCreateInfo {
    unsafe {
      VkPipelineDepthStencilStateCreateInfo {
        sType: VkStructureType::E_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
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
  pub fn set_depth_test_enable(mut self, value: VkBool32) -> Self {
    self.depthTestEnable = value;
    self
  }
  #[inline]
  pub fn set_depth_write_enable(mut self, value: VkBool32) -> Self {
    self.depthWriteEnable = value;
    self
  }
  #[inline]
  pub fn set_depth_compare_op(mut self, value: VkCompareOp) -> Self {
    self.depthCompareOp = value;
    self
  }
  #[inline]
  pub fn set_depth_bounds_test_enable(mut self, value: VkBool32) -> Self {
    self.depthBoundsTestEnable = value;
    self
  }
  #[inline]
  pub fn set_stencil_test_enable(mut self, value: VkBool32) -> Self {
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineDepthStencilStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_depth_test_enable(&self) -> VkBool32 {
    self.depthTestEnable
  }
  #[inline]
  pub fn get_depth_write_enable(&self) -> VkBool32 {
    self.depthWriteEnable
  }
  #[inline]
  pub fn get_depth_compare_op(&self) -> VkCompareOp {
    self.depthCompareOp
  }
  #[inline]
  pub fn get_depth_bounds_test_enable(&self) -> VkBool32 {
    self.depthBoundsTestEnable
  }
  #[inline]
  pub fn get_stencil_test_enable(&self) -> VkBool32 {
    self.stencilTestEnable
  }
  #[inline]
  pub fn get_front(&self) -> VkStencilOpState {
    self.front
  }
  #[inline]
  pub fn get_back(&self) -> VkStencilOpState {
    self.back
  }
  #[inline]
  pub fn get_min_depth_bounds(&self) -> f32 {
    self.minDepthBounds
  }
  #[inline]
  pub fn get_max_depth_bounds(&self) -> f32 {
    self.maxDepthBounds
  }
}
impl Default for VkPipelineDepthStencilStateCreateInfo {
  fn default() -> VkPipelineDepthStencilStateCreateInfo {
    VkPipelineDepthStencilStateCreateInfo::new()
  }
}
unsafe impl RawStruct for VkPipelineDepthStencilStateCreateInfo {
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
  pub fn set_blend_enable(mut self, value: VkBool32) -> Self {
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
  pub fn get_blend_enable(&self) -> VkBool32 {
    self.blendEnable
  }
  #[inline]
  pub fn get_src_color_blend_factor(&self) -> VkBlendFactor {
    self.srcColorBlendFactor
  }
  #[inline]
  pub fn get_dst_color_blend_factor(&self) -> VkBlendFactor {
    self.dstColorBlendFactor
  }
  #[inline]
  pub fn get_color_blend_op(&self) -> VkBlendOp {
    self.colorBlendOp
  }
  #[inline]
  pub fn get_src_alpha_blend_factor(&self) -> VkBlendFactor {
    self.srcAlphaBlendFactor
  }
  #[inline]
  pub fn get_dst_alpha_blend_factor(&self) -> VkBlendFactor {
    self.dstAlphaBlendFactor
  }
  #[inline]
  pub fn get_alpha_blend_op(&self) -> VkBlendOp {
    self.alphaBlendOp
  }
  #[inline]
  pub fn get_color_write_mask(&self) -> VkColorComponentFlags {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
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
  pub fn set_logic_op_enable(mut self, value: VkBool32) -> Self {
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineColorBlendStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_logic_op_enable(&self) -> VkBool32 {
    self.logicOpEnable
  }
  #[inline]
  pub fn get_logic_op(&self) -> VkLogicOp {
    self.logicOp
  }
  #[inline]
  pub fn get_attachment_count(&self) -> u32 {
    self.attachmentCount
  }
  #[inline]
  pub fn get_blend_constants(&self) -> [f32; 4] {
    self.blendConstants
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDynamicStateCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineDynamicStateCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_dynamic_state_count(&self) -> u32 {
    self.dynamicStateCount
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
pub type VkPipelineLayout = VkNonDispatchableHandle<VkPipelineLayout__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkRenderPass__ {}
pub type VkRenderPass = VkNonDispatchableHandle<VkRenderPass__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPipeline__ {}
pub type VkPipeline = VkNonDispatchableHandle<VkPipeline__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGraphicsPipelineCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkPipelineCreateFlags,
  stageCount: u32,
  pStages: *const types_raw::VkPipelineShaderStageCreateInfo,
  pub pVertexInputState: &'a VkPipelineVertexInputStateCreateInfo<'a>,
  pub pInputAssemblyState: &'a VkPipelineInputAssemblyStateCreateInfo,
  pub pTessellationState: Option<&'a VkPipelineTessellationStateCreateInfo>,
  pub pViewportState: Option<&'a VkPipelineViewportStateCreateInfo<'a>>,
  pub pRasterizationState: &'a VkPipelineRasterizationStateCreateInfo,
  pub pMultisampleState: Option<&'a VkPipelineMultisampleStateCreateInfo<'a>>,
  pub pDepthStencilState: Option<&'a VkPipelineDepthStencilStateCreateInfo>,
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
        sType: VkStructureType::E_GRAPHICS_PIPELINE_CREATE_INFO,
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
  pub fn set_input_assembly_state(mut self, value: &'a VkPipelineInputAssemblyStateCreateInfo) -> Self {
    self.pInputAssemblyState = value;
    self
  }
  #[inline]
  pub fn set_tessellation_state(mut self, value: Option<&'a VkPipelineTessellationStateCreateInfo>) -> Self {
    self.pTessellationState = value;
    self
  }
  #[inline]
  pub fn set_viewport_state(mut self, value: Option<&'a VkPipelineViewportStateCreateInfo<'a>>) -> Self {
    self.pViewportState = value;
    self
  }
  #[inline]
  pub fn set_rasterization_state(mut self, value: &'a VkPipelineRasterizationStateCreateInfo) -> Self {
    self.pRasterizationState = value;
    self
  }
  #[inline]
  pub fn set_multisample_state(mut self, value: Option<&'a VkPipelineMultisampleStateCreateInfo<'a>>) -> Self {
    self.pMultisampleState = value;
    self
  }
  #[inline]
  pub fn set_depth_stencil_state(mut self, value: Option<&'a VkPipelineDepthStencilStateCreateInfo>) -> Self {
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_stage_count(&self) -> u32 {
    self.stageCount
  }
  #[inline]
  pub fn get_vertex_input_state(&self) -> &'a VkPipelineVertexInputStateCreateInfo<'a> {
    self.pVertexInputState
  }
  #[inline]
  pub fn get_input_assembly_state(&self) -> &'a VkPipelineInputAssemblyStateCreateInfo {
    self.pInputAssemblyState
  }
  #[inline]
  pub fn get_tessellation_state(&self) -> Option<&'a VkPipelineTessellationStateCreateInfo> {
    self.pTessellationState
  }
  #[inline]
  pub fn get_viewport_state(&self) -> Option<&'a VkPipelineViewportStateCreateInfo<'a>> {
    self.pViewportState
  }
  #[inline]
  pub fn get_rasterization_state(&self) -> &'a VkPipelineRasterizationStateCreateInfo {
    self.pRasterizationState
  }
  #[inline]
  pub fn get_multisample_state(&self) -> Option<&'a VkPipelineMultisampleStateCreateInfo<'a>> {
    self.pMultisampleState
  }
  #[inline]
  pub fn get_depth_stencil_state(&self) -> Option<&'a VkPipelineDepthStencilStateCreateInfo> {
    self.pDepthStencilState
  }
  #[inline]
  pub fn get_color_blend_state(&self) -> Option<&'a VkPipelineColorBlendStateCreateInfo<'a>> {
    self.pColorBlendState
  }
  #[inline]
  pub fn get_dynamic_state(&self) -> Option<&'a VkPipelineDynamicStateCreateInfo<'a>> {
    self.pDynamicState
  }
  #[inline]
  pub fn get_layout(&self) -> VkPipelineLayout {
    self.layout
  }
  #[inline]
  pub fn get_render_pass(&self) -> VkRenderPass {
    self.renderPass
  }
  #[inline]
  pub fn get_subpass(&self) -> u32 {
    self.subpass
  }
  #[inline]
  pub fn get_base_pipeline_handle(&self) -> Option<VkPipeline> {
    self.basePipelineHandle
  }
  #[inline]
  pub fn get_base_pipeline_index(&self) -> i32 {
    self.basePipelineIndex
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComputePipelineCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_COMPUTE_PIPELINE_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_stage(&self) -> VkPipelineShaderStageCreateInfo<'a> {
    self.stage
  }
  #[inline]
  pub fn get_layout(&self) -> VkPipelineLayout {
    self.layout
  }
  #[inline]
  pub fn get_base_pipeline_handle(&self) -> Option<VkPipeline> {
    self.basePipelineHandle
  }
  #[inline]
  pub fn get_base_pipeline_index(&self) -> i32 {
    self.basePipelineIndex
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
pub type VkDescriptorSetLayout = VkNonDispatchableHandle<VkDescriptorSetLayout__>;
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
  pub fn get_stage_flags(&self) -> VkShaderStageFlags {
    self.stageFlags
  }
  #[inline]
  pub fn get_offset(&self) -> u32 {
    self.offset
  }
  #[inline]
  pub fn get_size(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineLayoutCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_LAYOUT_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineLayoutCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_set_layout_count(&self) -> u32 {
    self.setLayoutCount
  }
  #[inline]
  pub fn get_push_constant_range_count(&self) -> u32 {
    self.pushConstantRangeCount
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
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
impl VkSamplerCreateInfo {
  #[inline]
  pub fn new() -> VkSamplerCreateInfo {
    unsafe {
      VkSamplerCreateInfo {
        sType: VkStructureType::E_SAMPLER_CREATE_INFO,
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
  pub fn set_anisotropy_enable(mut self, value: VkBool32) -> Self {
    self.anisotropyEnable = value;
    self
  }
  #[inline]
  pub fn set_max_anisotropy(mut self, value: f32) -> Self {
    self.maxAnisotropy = value;
    self
  }
  #[inline]
  pub fn set_compare_enable(mut self, value: VkBool32) -> Self {
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
  pub fn set_unnormalized_coordinates(mut self, value: VkBool32) -> Self {
    self.unnormalizedCoordinates = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkSamplerCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_mag_filter(&self) -> VkFilter {
    self.magFilter
  }
  #[inline]
  pub fn get_min_filter(&self) -> VkFilter {
    self.minFilter
  }
  #[inline]
  pub fn get_mipmap_mode(&self) -> VkSamplerMipmapMode {
    self.mipmapMode
  }
  #[inline]
  pub fn get_address_mode_u(&self) -> VkSamplerAddressMode {
    self.addressModeU
  }
  #[inline]
  pub fn get_address_mode_v(&self) -> VkSamplerAddressMode {
    self.addressModeV
  }
  #[inline]
  pub fn get_address_mode_w(&self) -> VkSamplerAddressMode {
    self.addressModeW
  }
  #[inline]
  pub fn get_mip_lod_bias(&self) -> f32 {
    self.mipLodBias
  }
  #[inline]
  pub fn get_anisotropy_enable(&self) -> VkBool32 {
    self.anisotropyEnable
  }
  #[inline]
  pub fn get_max_anisotropy(&self) -> f32 {
    self.maxAnisotropy
  }
  #[inline]
  pub fn get_compare_enable(&self) -> VkBool32 {
    self.compareEnable
  }
  #[inline]
  pub fn get_compare_op(&self) -> VkCompareOp {
    self.compareOp
  }
  #[inline]
  pub fn get_min_lod(&self) -> f32 {
    self.minLod
  }
  #[inline]
  pub fn get_max_lod(&self) -> f32 {
    self.maxLod
  }
  #[inline]
  pub fn get_border_color(&self) -> VkBorderColor {
    self.borderColor
  }
  #[inline]
  pub fn get_unnormalized_coordinates(&self) -> VkBool32 {
    self.unnormalizedCoordinates
  }
}
impl Default for VkSamplerCreateInfo {
  fn default() -> VkSamplerCreateInfo {
    VkSamplerCreateInfo::new()
  }
}
unsafe impl RawStruct for VkSamplerCreateInfo {
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
pub type VkSampler = VkNonDispatchableHandle<VkSampler__>;
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_binding(&self) -> u32 {
    self.binding
  }
  #[inline]
  pub fn get_descriptor_type(&self) -> VkDescriptorType {
    self.descriptorType
  }
  #[inline]
  pub fn get_descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
  #[inline]
  pub fn get_stage_flags(&self) -> VkShaderStageFlags {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkDescriptorSetLayoutCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_binding_count(&self) -> u32 {
    self.bindingCount
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
  pub fn get_descriptor_count(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DESCRIPTOR_POOL_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkDescriptorPoolCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_max_sets(&self) -> u32 {
    self.maxSets
  }
  #[inline]
  pub fn get_pool_size_count(&self) -> u32 {
    self.poolSizeCount
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
pub type VkDescriptorPool = VkNonDispatchableHandle<VkDescriptorPool__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetAllocateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DESCRIPTOR_SET_ALLOCATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_descriptor_pool(&self) -> VkDescriptorPool {
    self.descriptorPool
  }
  #[inline]
  pub fn get_descriptor_set_count(&self) -> u32 {
    self.descriptorSetCount
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
pub type VkDescriptorSet = VkNonDispatchableHandle<VkDescriptorSet__>;
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
  pub fn get_sampler(&self) -> VkSampler {
    self.sampler
  }
  #[inline]
  pub fn get_image_view(&self) -> VkImageView {
    self.imageView
  }
  #[inline]
  pub fn get_image_layout(&self) -> VkImageLayout {
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
  pub fn get_buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn get_offset(&self) -> VkDeviceSize {
    self.offset
  }
  #[inline]
  pub fn get_range(&self) -> VkDeviceSize {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWriteDescriptorSet<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_WRITE_DESCRIPTOR_SET,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_dst_set(&self) -> VkDescriptorSet {
    self.dstSet
  }
  #[inline]
  pub fn get_dst_binding(&self) -> u32 {
    self.dstBinding
  }
  #[inline]
  pub fn get_dst_array_element(&self) -> u32 {
    self.dstArrayElement
  }
  #[inline]
  pub fn get_descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
  #[inline]
  pub fn get_descriptor_type(&self) -> VkDescriptorType {
    self.descriptorType
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyDescriptorSet {
  sType: VkStructureType,
  pNext: *const c_void,
  pub srcSet: VkDescriptorSet,
  pub srcBinding: u32,
  pub srcArrayElement: u32,
  pub dstSet: VkDescriptorSet,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
}
impl VkCopyDescriptorSet {
  #[inline]
  pub fn new() -> VkCopyDescriptorSet {
    unsafe {
      VkCopyDescriptorSet {
        sType: VkStructureType::E_COPY_DESCRIPTOR_SET,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_src_set(&self) -> VkDescriptorSet {
    self.srcSet
  }
  #[inline]
  pub fn get_src_binding(&self) -> u32 {
    self.srcBinding
  }
  #[inline]
  pub fn get_src_array_element(&self) -> u32 {
    self.srcArrayElement
  }
  #[inline]
  pub fn get_dst_set(&self) -> VkDescriptorSet {
    self.dstSet
  }
  #[inline]
  pub fn get_dst_binding(&self) -> u32 {
    self.dstBinding
  }
  #[inline]
  pub fn get_dst_array_element(&self) -> u32 {
    self.dstArrayElement
  }
  #[inline]
  pub fn get_descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
}
impl Default for VkCopyDescriptorSet {
  fn default() -> VkCopyDescriptorSet {
    VkCopyDescriptorSet::new()
  }
}
unsafe impl RawStruct for VkCopyDescriptorSet {
  type Raw = types_raw::VkCopyDescriptorSet;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_copy_descriptor_set() {
  assert_size!(types_raw::VkCopyDescriptorSet, VkCopyDescriptorSet);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_FRAMEBUFFER_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkFramebufferCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_render_pass(&self) -> VkRenderPass {
    self.renderPass
  }
  #[inline]
  pub fn get_attachment_count(&self) -> u32 {
    self.attachmentCount
  }
  #[inline]
  pub fn get_width(&self) -> u32 {
    self.width
  }
  #[inline]
  pub fn get_height(&self) -> u32 {
    self.height
  }
  #[inline]
  pub fn get_layers(&self) -> u32 {
    self.layers
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
pub type VkFramebuffer = VkNonDispatchableHandle<VkFramebuffer__>;
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
  pub fn get_flags(&self) -> VkAttachmentDescriptionFlags {
    self.flags
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_samples(&self) -> VkSampleCountFlagBits {
    self.samples
  }
  #[inline]
  pub fn get_load_op(&self) -> VkAttachmentLoadOp {
    self.loadOp
  }
  #[inline]
  pub fn get_store_op(&self) -> VkAttachmentStoreOp {
    self.storeOp
  }
  #[inline]
  pub fn get_stencil_load_op(&self) -> VkAttachmentLoadOp {
    self.stencilLoadOp
  }
  #[inline]
  pub fn get_stencil_store_op(&self) -> VkAttachmentStoreOp {
    self.stencilStoreOp
  }
  #[inline]
  pub fn get_initial_layout(&self) -> VkImageLayout {
    self.initialLayout
  }
  #[inline]
  pub fn get_final_layout(&self) -> VkImageLayout {
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
  pub fn get_attachment(&self) -> u32 {
    self.attachment
  }
  #[inline]
  pub fn get_layout(&self) -> VkImageLayout {
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
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_flags(&self) -> VkSubpassDescriptionFlags {
    self.flags
  }
  #[inline]
  pub fn get_pipeline_bind_point(&self) -> VkPipelineBindPoint {
    self.pipelineBindPoint
  }
  #[inline]
  pub fn get_input_attachment_count(&self) -> u32 {
    self.inputAttachmentCount
  }
  #[inline]
  pub fn get_color_attachment_count(&self) -> u32 {
    self.colorAttachmentCount
  }
  #[inline]
  pub fn get_depth_stencil_attachment(&self) -> Option<&'a VkAttachmentReference> {
    self.pDepthStencilAttachment
  }
  #[inline]
  pub fn get_preserve_attachment_count(&self) -> u32 {
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
  pub fn get_src_subpass(&self) -> u32 {
    self.srcSubpass
  }
  #[inline]
  pub fn get_dst_subpass(&self) -> u32 {
    self.dstSubpass
  }
  #[inline]
  pub fn get_src_stage_mask(&self) -> VkPipelineStageFlags {
    self.srcStageMask
  }
  #[inline]
  pub fn get_dst_stage_mask(&self) -> VkPipelineStageFlags {
    self.dstStageMask
  }
  #[inline]
  pub fn get_src_access_mask(&self) -> VkAccessFlags {
    self.srcAccessMask
  }
  #[inline]
  pub fn get_dst_access_mask(&self) -> VkAccessFlags {
    self.dstAccessMask
  }
  #[inline]
  pub fn get_dependency_flags(&self) -> VkDependencyFlags {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassCreateInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_RENDER_PASS_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkRenderPassCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_attachment_count(&self) -> u32 {
    self.attachmentCount
  }
  #[inline]
  pub fn get_subpass_count(&self) -> u32 {
    self.subpassCount
  }
  #[inline]
  pub fn get_dependency_count(&self) -> u32 {
    self.dependencyCount
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandPoolCreateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkCommandPoolCreateFlags,
  pub queueFamilyIndex: u32,
}
impl VkCommandPoolCreateInfo {
  #[inline]
  pub fn new() -> VkCommandPoolCreateInfo {
    unsafe {
      VkCommandPoolCreateInfo {
        sType: VkStructureType::E_COMMAND_POOL_CREATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkCommandPoolCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_queue_family_index(&self) -> u32 {
    self.queueFamilyIndex
  }
}
impl Default for VkCommandPoolCreateInfo {
  fn default() -> VkCommandPoolCreateInfo {
    VkCommandPoolCreateInfo::new()
  }
}
unsafe impl RawStruct for VkCommandPoolCreateInfo {
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
pub type VkCommandPool = VkNonDispatchableHandle<VkCommandPool__>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferAllocateInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub commandPool: VkCommandPool,
  pub level: VkCommandBufferLevel,
  pub commandBufferCount: u32,
}
impl VkCommandBufferAllocateInfo {
  #[inline]
  pub fn new() -> VkCommandBufferAllocateInfo {
    unsafe {
      VkCommandBufferAllocateInfo {
        sType: VkStructureType::E_COMMAND_BUFFER_ALLOCATE_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_command_pool(&self) -> VkCommandPool {
    self.commandPool
  }
  #[inline]
  pub fn get_level(&self) -> VkCommandBufferLevel {
    self.level
  }
  #[inline]
  pub fn get_command_buffer_count(&self) -> u32 {
    self.commandBufferCount
  }
}
impl Default for VkCommandBufferAllocateInfo {
  fn default() -> VkCommandBufferAllocateInfo {
    VkCommandBufferAllocateInfo::new()
  }
}
unsafe impl RawStruct for VkCommandBufferAllocateInfo {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferInheritanceInfo {
  sType: VkStructureType,
  pNext: *const c_void,
  pub renderPass: Option<VkRenderPass>,
  pub subpass: u32,
  pub framebuffer: Option<VkFramebuffer>,
  pub occlusionQueryEnable: VkBool32,
  pub queryFlags: VkQueryControlFlags,
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}
impl VkCommandBufferInheritanceInfo {
  #[inline]
  pub fn new() -> VkCommandBufferInheritanceInfo {
    unsafe {
      VkCommandBufferInheritanceInfo {
        sType: VkStructureType::E_COMMAND_BUFFER_INHERITANCE_INFO,
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
  pub fn set_occlusion_query_enable(mut self, value: VkBool32) -> Self {
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_render_pass(&self) -> Option<VkRenderPass> {
    self.renderPass
  }
  #[inline]
  pub fn get_subpass(&self) -> u32 {
    self.subpass
  }
  #[inline]
  pub fn get_framebuffer(&self) -> Option<VkFramebuffer> {
    self.framebuffer
  }
  #[inline]
  pub fn get_occlusion_query_enable(&self) -> VkBool32 {
    self.occlusionQueryEnable
  }
  #[inline]
  pub fn get_query_flags(&self) -> VkQueryControlFlags {
    self.queryFlags
  }
  #[inline]
  pub fn get_pipeline_statistics(&self) -> VkQueryPipelineStatisticFlags {
    self.pipelineStatistics
  }
}
impl Default for VkCommandBufferInheritanceInfo {
  fn default() -> VkCommandBufferInheritanceInfo {
    VkCommandBufferInheritanceInfo::new()
  }
}
unsafe impl RawStruct for VkCommandBufferInheritanceInfo {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferBeginInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkCommandBufferUsageFlags,
  pub pInheritanceInfo: Option<&'a VkCommandBufferInheritanceInfo>,
}
impl<'a> VkCommandBufferBeginInfo<'a> {
  #[inline]
  pub fn new() -> VkCommandBufferBeginInfo<'a> {
    unsafe {
      VkCommandBufferBeginInfo {
        sType: VkStructureType::E_COMMAND_BUFFER_BEGIN_INFO,
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
  pub fn set_inheritance_info(mut self, value: Option<&'a VkCommandBufferInheritanceInfo>) -> Self {
    self.pInheritanceInfo = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkCommandBufferUsageFlags {
    self.flags
  }
  #[inline]
  pub fn get_inheritance_info(&self) -> Option<&'a VkCommandBufferInheritanceInfo> {
    self.pInheritanceInfo
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
  pub fn get_src_offset(&self) -> VkDeviceSize {
    self.srcOffset
  }
  #[inline]
  pub fn get_dst_offset(&self) -> VkDeviceSize {
    self.dstOffset
  }
  #[inline]
  pub fn get_size(&self) -> VkDeviceSize {
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
  pub fn get_aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn get_mip_level(&self) -> u32 {
    self.mipLevel
  }
  #[inline]
  pub fn get_base_array_layer(&self) -> u32 {
    self.baseArrayLayer
  }
  #[inline]
  pub fn get_layer_count(&self) -> u32 {
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
  pub fn get_src_subresource(&self) -> VkImageSubresourceLayers {
    self.srcSubresource
  }
  #[inline]
  pub fn get_src_offset(&self) -> VkOffset3D {
    self.srcOffset
  }
  #[inline]
  pub fn get_dst_subresource(&self) -> VkImageSubresourceLayers {
    self.dstSubresource
  }
  #[inline]
  pub fn get_dst_offset(&self) -> VkOffset3D {
    self.dstOffset
  }
  #[inline]
  pub fn get_extent(&self) -> VkExtent3D {
    self.extent
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
  pub fn get_src_subresource(&self) -> VkImageSubresourceLayers {
    self.srcSubresource
  }
  #[inline]
  pub fn get_src_offsets(&self) -> [VkOffset3D; 2] {
    self.srcOffsets
  }
  #[inline]
  pub fn get_dst_subresource(&self) -> VkImageSubresourceLayers {
    self.dstSubresource
  }
  #[inline]
  pub fn get_dst_offsets(&self) -> [VkOffset3D; 2] {
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
  pub fn get_buffer_offset(&self) -> VkDeviceSize {
    self.bufferOffset
  }
  #[inline]
  pub fn get_buffer_row_length(&self) -> u32 {
    self.bufferRowLength
  }
  #[inline]
  pub fn get_buffer_image_height(&self) -> u32 {
    self.bufferImageHeight
  }
  #[inline]
  pub fn get_image_subresource(&self) -> VkImageSubresourceLayers {
    self.imageSubresource
  }
  #[inline]
  pub fn get_image_offset(&self) -> VkOffset3D {
    self.imageOffset
  }
  #[inline]
  pub fn get_image_extent(&self) -> VkExtent3D {
    self.imageExtent
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
  pub fn get_depth(&self) -> f32 {
    self.depth
  }
  #[inline]
  pub fn get_stencil(&self) -> u32 {
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
  pub fn get_aspect_mask(&self) -> VkImageAspectFlags {
    self.aspectMask
  }
  #[inline]
  pub fn get_color_attachment(&self) -> u32 {
    self.colorAttachment
  }
  #[inline]
  pub fn get_clear_value(&self) -> VkClearValue {
    self.clearValue
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
  pub fn get_rect(&self) -> VkRect2D {
    self.rect
  }
  #[inline]
  pub fn get_base_array_layer(&self) -> u32 {
    self.baseArrayLayer
  }
  #[inline]
  pub fn get_layer_count(&self) -> u32 {
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
  pub fn get_src_subresource(&self) -> VkImageSubresourceLayers {
    self.srcSubresource
  }
  #[inline]
  pub fn get_src_offset(&self) -> VkOffset3D {
    self.srcOffset
  }
  #[inline]
  pub fn get_dst_subresource(&self) -> VkImageSubresourceLayers {
    self.dstSubresource
  }
  #[inline]
  pub fn get_dst_offset(&self) -> VkOffset3D {
    self.dstOffset
  }
  #[inline]
  pub fn get_extent(&self) -> VkExtent3D {
    self.extent
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassBeginInfo<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_RENDER_PASS_BEGIN_INFO,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_render_pass(&self) -> VkRenderPass {
    self.renderPass
  }
  #[inline]
  pub fn get_framebuffer(&self) -> VkFramebuffer {
    self.framebuffer
  }
  #[inline]
  pub fn get_render_area(&self) -> VkRect2D {
    self.renderArea
  }
  #[inline]
  pub fn get_clear_value_count(&self) -> u32 {
    self.clearValueCount
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
#[cfg(feature = "VK_KHR_surface")]
pub type VkSurfaceKHR = VkNonDispatchableHandle<VkSurfaceKHR__>;
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
  pub fn new() -> VkSurfaceCapabilitiesKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_min_image_count(mut self, value: u32) -> Self {
    self.minImageCount = value;
    self
  }
  #[inline]
  pub fn set_max_image_count(mut self, value: u32) -> Self {
    self.maxImageCount = value;
    self
  }
  #[inline]
  pub fn set_current_extent(mut self, value: VkExtent2D) -> Self {
    self.currentExtent = value;
    self
  }
  #[inline]
  pub fn set_min_image_extent(mut self, value: VkExtent2D) -> Self {
    self.minImageExtent = value;
    self
  }
  #[inline]
  pub fn set_max_image_extent(mut self, value: VkExtent2D) -> Self {
    self.maxImageExtent = value;
    self
  }
  #[inline]
  pub fn set_max_image_array_layers(mut self, value: u32) -> Self {
    self.maxImageArrayLayers = value;
    self
  }
  #[inline]
  pub fn set_supported_transforms(mut self, value: VkSurfaceTransformFlagsKHR) -> Self {
    self.supportedTransforms = value;
    self
  }
  #[inline]
  pub fn set_current_transform(mut self, value: VkSurfaceTransformFlagBitsKHR) -> Self {
    self.currentTransform = value;
    self
  }
  #[inline]
  pub fn set_supported_composite_alpha(mut self, value: VkCompositeAlphaFlagsKHR) -> Self {
    self.supportedCompositeAlpha = value;
    self
  }
  #[inline]
  pub fn set_supported_usage_flags(mut self, value: VkImageUsageFlags) -> Self {
    self.supportedUsageFlags = value;
    self
  }
  #[inline]
  pub fn get_min_image_count(&self) -> u32 {
    self.minImageCount
  }
  #[inline]
  pub fn get_max_image_count(&self) -> u32 {
    self.maxImageCount
  }
  #[inline]
  pub fn get_current_extent(&self) -> VkExtent2D {
    self.currentExtent
  }
  #[inline]
  pub fn get_min_image_extent(&self) -> VkExtent2D {
    self.minImageExtent
  }
  #[inline]
  pub fn get_max_image_extent(&self) -> VkExtent2D {
    self.maxImageExtent
  }
  #[inline]
  pub fn get_max_image_array_layers(&self) -> u32 {
    self.maxImageArrayLayers
  }
  #[inline]
  pub fn get_supported_transforms(&self) -> VkSurfaceTransformFlagsKHR {
    self.supportedTransforms
  }
  #[inline]
  pub fn get_current_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
    self.currentTransform
  }
  #[inline]
  pub fn get_supported_composite_alpha(&self) -> VkCompositeAlphaFlagsKHR {
    self.supportedCompositeAlpha
  }
  #[inline]
  pub fn get_supported_usage_flags(&self) -> VkImageUsageFlags {
    self.supportedUsageFlags
  }
}
#[cfg(feature = "VK_KHR_surface")]
impl Default for VkSurfaceCapabilitiesKHR {
  fn default() -> VkSurfaceCapabilitiesKHR {
    VkSurfaceCapabilitiesKHR::new()
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
  pub fn new() -> VkSurfaceFormatKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_format(mut self, value: VkFormat) -> Self {
    self.format = value;
    self
  }
  #[inline]
  pub fn set_color_space(mut self, value: VkColorSpaceKHR) -> Self {
    self.colorSpace = value;
    self
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_color_space(&self) -> VkColorSpaceKHR {
    self.colorSpace
  }
}
#[cfg(feature = "VK_KHR_surface")]
impl Default for VkSurfaceFormatKHR {
  fn default() -> VkSurfaceFormatKHR {
    VkSurfaceFormatKHR::new()
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
#[cfg(feature = "VK_KHR_swapchain")]
pub type VkSwapchainKHR = VkNonDispatchableHandle<VkSwapchainKHR__>;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_swapchain")]
pub struct VkSwapchainCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_SWAPCHAIN_CREATE_INFO_KHR,
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
  pub fn set_clipped(mut self, value: VkBool32) -> Self {
    self.clipped = value;
    self
  }
  #[inline]
  pub fn set_old_swapchain(mut self, value: Option<VkSwapchainKHR>) -> Self {
    self.oldSwapchain = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkSwapchainCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_surface(&self) -> VkSurfaceKHR {
    self.surface
  }
  #[inline]
  pub fn get_min_image_count(&self) -> u32 {
    self.minImageCount
  }
  #[inline]
  pub fn get_image_format(&self) -> VkFormat {
    self.imageFormat
  }
  #[inline]
  pub fn get_image_color_space(&self) -> VkColorSpaceKHR {
    self.imageColorSpace
  }
  #[inline]
  pub fn get_image_extent(&self) -> VkExtent2D {
    self.imageExtent
  }
  #[inline]
  pub fn get_image_array_layers(&self) -> u32 {
    self.imageArrayLayers
  }
  #[inline]
  pub fn get_image_usage(&self) -> VkImageUsageFlags {
    self.imageUsage
  }
  #[inline]
  pub fn get_image_sharing_mode(&self) -> VkSharingMode {
    self.imageSharingMode
  }
  #[inline]
  pub fn get_queue_family_index_count(&self) -> u32 {
    self.queueFamilyIndexCount
  }
  #[inline]
  pub fn get_pre_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
    self.preTransform
  }
  #[inline]
  pub fn get_composite_alpha(&self) -> VkCompositeAlphaFlagBitsKHR {
    self.compositeAlpha
  }
  #[inline]
  pub fn get_present_mode(&self) -> VkPresentModeKHR {
    self.presentMode
  }
  #[inline]
  pub fn get_clipped(&self) -> VkBool32 {
    self.clipped
  }
  #[inline]
  pub fn get_old_swapchain(&self) -> Option<VkSwapchainKHR> {
    self.oldSwapchain
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_swapchain")]
pub struct VkPresentInfoKHR<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PRESENT_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_wait_semaphore_count(&self) -> u32 {
    self.waitSemaphoreCount
  }
  #[inline]
  pub fn get_swapchain_count(&self) -> u32 {
    self.swapchainCount
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
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayKHR = VkNonDispatchableHandle<VkDisplayKHR__>;
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn new() -> VkDisplayPropertiesKHR<'a> {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_display(mut self, value: VkDisplayKHR) -> Self {
    self.display = value;
    self
  }
  #[inline]
  pub fn set_display_name(mut self, value: &'a AsRef<CStr>) -> Self {
    unsafe {
      self.displayName = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_physical_dimensions(mut self, value: VkExtent2D) -> Self {
    self.physicalDimensions = value;
    self
  }
  #[inline]
  pub fn set_physical_resolution(mut self, value: VkExtent2D) -> Self {
    self.physicalResolution = value;
    self
  }
  #[inline]
  pub fn set_supported_transforms(mut self, value: VkSurfaceTransformFlagsKHR) -> Self {
    self.supportedTransforms = value;
    self
  }
  #[inline]
  pub fn set_plane_reorder_possible(mut self, value: VkBool32) -> Self {
    self.planeReorderPossible = value;
    self
  }
  #[inline]
  pub fn set_persistent_content(mut self, value: VkBool32) -> Self {
    self.persistentContent = value;
    self
  }
  #[inline]
  pub fn get_display(&self) -> VkDisplayKHR {
    self.display
  }
  #[inline]
  pub fn get_display_name(&self) -> &'a CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.displayName) }
  }
  #[inline]
  pub fn get_physical_dimensions(&self) -> VkExtent2D {
    self.physicalDimensions
  }
  #[inline]
  pub fn get_physical_resolution(&self) -> VkExtent2D {
    self.physicalResolution
  }
  #[inline]
  pub fn get_supported_transforms(&self) -> VkSurfaceTransformFlagsKHR {
    self.supportedTransforms
  }
  #[inline]
  pub fn get_plane_reorder_possible(&self) -> VkBool32 {
    self.planeReorderPossible
  }
  #[inline]
  pub fn get_persistent_content(&self) -> VkBool32 {
    self.persistentContent
  }
}
#[cfg(feature = "VK_KHR_display")]
impl<'a> Default for VkDisplayPropertiesKHR<'a> {
  fn default() -> VkDisplayPropertiesKHR<'a> {
    VkDisplayPropertiesKHR::new()
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
  pub fn get_visible_region(&self) -> VkExtent2D {
    self.visibleRegion
  }
  #[inline]
  pub fn get_refresh_rate(&self) -> u32 {
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
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayModeKHR = VkNonDispatchableHandle<VkDisplayModeKHR__>;
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
  pub fn new() -> VkDisplayModePropertiesKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_display_mode(mut self, value: VkDisplayModeKHR) -> Self {
    self.displayMode = value;
    self
  }
  #[inline]
  pub fn set_parameters(mut self, value: VkDisplayModeParametersKHR) -> Self {
    self.parameters = value;
    self
  }
  #[inline]
  pub fn get_display_mode(&self) -> VkDisplayModeKHR {
    self.displayMode
  }
  #[inline]
  pub fn get_parameters(&self) -> VkDisplayModeParametersKHR {
    self.parameters
  }
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayModePropertiesKHR {
  fn default() -> VkDisplayModePropertiesKHR {
    VkDisplayModePropertiesKHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplayModeCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkDisplayModeCreateFlagsKHR,
  pub parameters: VkDisplayModeParametersKHR,
}
#[cfg(feature = "VK_KHR_display")]
impl VkDisplayModeCreateInfoKHR {
  #[inline]
  pub fn new() -> VkDisplayModeCreateInfoKHR {
    unsafe {
      VkDisplayModeCreateInfoKHR {
        sType: VkStructureType::E_DISPLAY_MODE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkDisplayModeCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_parameters(&self) -> VkDisplayModeParametersKHR {
    self.parameters
  }
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayModeCreateInfoKHR {
  fn default() -> VkDisplayModeCreateInfoKHR {
    VkDisplayModeCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl RawStruct for VkDisplayModeCreateInfoKHR {
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
  pub fn new() -> VkDisplayPlaneCapabilitiesKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_supported_alpha(mut self, value: VkDisplayPlaneAlphaFlagsKHR) -> Self {
    self.supportedAlpha = value;
    self
  }
  #[inline]
  pub fn set_min_src_position(mut self, value: VkOffset2D) -> Self {
    self.minSrcPosition = value;
    self
  }
  #[inline]
  pub fn set_max_src_position(mut self, value: VkOffset2D) -> Self {
    self.maxSrcPosition = value;
    self
  }
  #[inline]
  pub fn set_min_src_extent(mut self, value: VkExtent2D) -> Self {
    self.minSrcExtent = value;
    self
  }
  #[inline]
  pub fn set_max_src_extent(mut self, value: VkExtent2D) -> Self {
    self.maxSrcExtent = value;
    self
  }
  #[inline]
  pub fn set_min_dst_position(mut self, value: VkOffset2D) -> Self {
    self.minDstPosition = value;
    self
  }
  #[inline]
  pub fn set_max_dst_position(mut self, value: VkOffset2D) -> Self {
    self.maxDstPosition = value;
    self
  }
  #[inline]
  pub fn set_min_dst_extent(mut self, value: VkExtent2D) -> Self {
    self.minDstExtent = value;
    self
  }
  #[inline]
  pub fn set_max_dst_extent(mut self, value: VkExtent2D) -> Self {
    self.maxDstExtent = value;
    self
  }
  #[inline]
  pub fn get_supported_alpha(&self) -> VkDisplayPlaneAlphaFlagsKHR {
    self.supportedAlpha
  }
  #[inline]
  pub fn get_min_src_position(&self) -> VkOffset2D {
    self.minSrcPosition
  }
  #[inline]
  pub fn get_max_src_position(&self) -> VkOffset2D {
    self.maxSrcPosition
  }
  #[inline]
  pub fn get_min_src_extent(&self) -> VkExtent2D {
    self.minSrcExtent
  }
  #[inline]
  pub fn get_max_src_extent(&self) -> VkExtent2D {
    self.maxSrcExtent
  }
  #[inline]
  pub fn get_min_dst_position(&self) -> VkOffset2D {
    self.minDstPosition
  }
  #[inline]
  pub fn get_max_dst_position(&self) -> VkOffset2D {
    self.maxDstPosition
  }
  #[inline]
  pub fn get_min_dst_extent(&self) -> VkExtent2D {
    self.minDstExtent
  }
  #[inline]
  pub fn get_max_dst_extent(&self) -> VkExtent2D {
    self.maxDstExtent
  }
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayPlaneCapabilitiesKHR {
  fn default() -> VkDisplayPlaneCapabilitiesKHR {
    VkDisplayPlaneCapabilitiesKHR::new()
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
  pub fn new() -> VkDisplayPlanePropertiesKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_current_display(mut self, value: VkDisplayKHR) -> Self {
    self.currentDisplay = value;
    self
  }
  #[inline]
  pub fn set_current_stack_index(mut self, value: u32) -> Self {
    self.currentStackIndex = value;
    self
  }
  #[inline]
  pub fn get_current_display(&self) -> VkDisplayKHR {
    self.currentDisplay
  }
  #[inline]
  pub fn get_current_stack_index(&self) -> u32 {
    self.currentStackIndex
  }
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayPlanePropertiesKHR {
  fn default() -> VkDisplayPlanePropertiesKHR {
    VkDisplayPlanePropertiesKHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display")]
pub struct VkDisplaySurfaceCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkDisplaySurfaceCreateFlagsKHR,
  pub displayMode: VkDisplayModeKHR,
  pub planeIndex: u32,
  pub planeStackIndex: u32,
  pub transform: VkSurfaceTransformFlagBitsKHR,
  pub globalAlpha: f32,
  pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
  pub imageExtent: VkExtent2D,
}
#[cfg(feature = "VK_KHR_display")]
impl VkDisplaySurfaceCreateInfoKHR {
  #[inline]
  pub fn new() -> VkDisplaySurfaceCreateInfoKHR {
    unsafe {
      VkDisplaySurfaceCreateInfoKHR {
        sType: VkStructureType::E_DISPLAY_SURFACE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkDisplaySurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_display_mode(&self) -> VkDisplayModeKHR {
    self.displayMode
  }
  #[inline]
  pub fn get_plane_index(&self) -> u32 {
    self.planeIndex
  }
  #[inline]
  pub fn get_plane_stack_index(&self) -> u32 {
    self.planeStackIndex
  }
  #[inline]
  pub fn get_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
    self.transform
  }
  #[inline]
  pub fn get_global_alpha(&self) -> f32 {
    self.globalAlpha
  }
  #[inline]
  pub fn get_alpha_mode(&self) -> VkDisplayPlaneAlphaFlagBitsKHR {
    self.alphaMode
  }
  #[inline]
  pub fn get_image_extent(&self) -> VkExtent2D {
    self.imageExtent
  }
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplaySurfaceCreateInfoKHR {
  fn default() -> VkDisplaySurfaceCreateInfoKHR {
    VkDisplaySurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
unsafe impl RawStruct for VkDisplaySurfaceCreateInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_display_swapchain")]
pub struct VkDisplayPresentInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub srcRect: VkRect2D,
  pub dstRect: VkRect2D,
  pub persistent: VkBool32,
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl VkDisplayPresentInfoKHR {
  #[inline]
  pub fn new() -> VkDisplayPresentInfoKHR {
    unsafe {
      VkDisplayPresentInfoKHR {
        sType: VkStructureType::E_DISPLAY_PRESENT_INFO_KHR,
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
  pub fn set_persistent(mut self, value: VkBool32) -> Self {
    self.persistent = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_src_rect(&self) -> VkRect2D {
    self.srcRect
  }
  #[inline]
  pub fn get_dst_rect(&self) -> VkRect2D {
    self.dstRect
  }
  #[inline]
  pub fn get_persistent(&self) -> VkBool32 {
    self.persistent
  }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl Default for VkDisplayPresentInfoKHR {
  fn default() -> VkDisplayPresentInfoKHR {
    VkDisplayPresentInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
unsafe impl RawStruct for VkDisplayPresentInfoKHR {
  type Raw = types_raw::VkDisplayPresentInfoKHR;
}
#[cfg(feature = "VK_KHR_display_swapchain")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_present_info_khr() {
  assert_size!(types_raw::VkDisplayPresentInfoKHR, VkDisplayPresentInfoKHR);
}

// feature: VK_KHR_xlib_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub struct VkXlibSurfaceCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkXlibSurfaceCreateFlagsKHR,
  pub dpy: *mut wsi::xlib::Display,
  pub window: wsi::xlib::Window,
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
impl VkXlibSurfaceCreateInfoKHR {
  #[inline]
  pub fn new() -> VkXlibSurfaceCreateInfoKHR {
    unsafe {
      VkXlibSurfaceCreateInfoKHR {
        sType: VkStructureType::E_XLIB_SURFACE_CREATE_INFO_KHR,
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
  pub fn set_window(mut self, value: wsi::xlib::Window) -> Self {
    self.window = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkXlibSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_dpy(&self) -> *mut wsi::xlib::Display {
    self.dpy
  }
  #[inline]
  pub fn get_window(&self) -> wsi::xlib::Window {
    self.window
  }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
impl Default for VkXlibSurfaceCreateInfoKHR {
  fn default() -> VkXlibSurfaceCreateInfoKHR {
    VkXlibSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
unsafe impl RawStruct for VkXlibSurfaceCreateInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub struct VkXcbSurfaceCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkXcbSurfaceCreateFlagsKHR,
  pub connection: *mut wsi::xcb::xcb_connection_t,
  pub window: wsi::xcb::xcb_window_t,
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
impl VkXcbSurfaceCreateInfoKHR {
  #[inline]
  pub fn new() -> VkXcbSurfaceCreateInfoKHR {
    unsafe {
      VkXcbSurfaceCreateInfoKHR {
        sType: VkStructureType::E_XCB_SURFACE_CREATE_INFO_KHR,
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
  pub fn set_window(mut self, value: wsi::xcb::xcb_window_t) -> Self {
    self.window = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkXcbSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_connection(&self) -> *mut wsi::xcb::xcb_connection_t {
    self.connection
  }
  #[inline]
  pub fn get_window(&self) -> wsi::xcb::xcb_window_t {
    self.window
  }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
impl Default for VkXcbSurfaceCreateInfoKHR {
  fn default() -> VkXcbSurfaceCreateInfoKHR {
    VkXcbSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
unsafe impl RawStruct for VkXcbSurfaceCreateInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub struct VkWaylandSurfaceCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkWaylandSurfaceCreateFlagsKHR,
  pub display: *mut wsi::wayland::wl_display,
  pub surface: *mut wsi::wayland::wl_surface,
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
impl VkWaylandSurfaceCreateInfoKHR {
  #[inline]
  pub fn new() -> VkWaylandSurfaceCreateInfoKHR {
    unsafe {
      VkWaylandSurfaceCreateInfoKHR {
        sType: VkStructureType::E_WAYLAND_SURFACE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkWaylandSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_display(&self) -> *mut wsi::wayland::wl_display {
    self.display
  }
  #[inline]
  pub fn get_surface(&self) -> *mut wsi::wayland::wl_surface {
    self.surface
  }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
impl Default for VkWaylandSurfaceCreateInfoKHR {
  fn default() -> VkWaylandSurfaceCreateInfoKHR {
    VkWaylandSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
unsafe impl RawStruct for VkWaylandSurfaceCreateInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub struct VkMirSurfaceCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkMirSurfaceCreateFlagsKHR,
  pub connection: *mut wsi::mir::MirConnection,
  pub mirSurface: *mut wsi::mir::MirSurface,
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
impl VkMirSurfaceCreateInfoKHR {
  #[inline]
  pub fn new() -> VkMirSurfaceCreateInfoKHR {
    unsafe {
      VkMirSurfaceCreateInfoKHR {
        sType: VkStructureType::E_MIR_SURFACE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkMirSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_connection(&self) -> *mut wsi::mir::MirConnection {
    self.connection
  }
  #[inline]
  pub fn get_mir_surface(&self) -> *mut wsi::mir::MirSurface {
    self.mirSurface
  }
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
impl Default for VkMirSurfaceCreateInfoKHR {
  fn default() -> VkMirSurfaceCreateInfoKHR {
    VkMirSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
unsafe impl RawStruct for VkMirSurfaceCreateInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub struct VkAndroidSurfaceCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkAndroidSurfaceCreateFlagsKHR,
  pub window: *mut wsi::android::ANativeWindow,
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
impl VkAndroidSurfaceCreateInfoKHR {
  #[inline]
  pub fn new() -> VkAndroidSurfaceCreateInfoKHR {
    unsafe {
      VkAndroidSurfaceCreateInfoKHR {
        sType: VkStructureType::E_ANDROID_SURFACE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkAndroidSurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_window(&self) -> *mut wsi::android::ANativeWindow {
    self.window
  }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
impl Default for VkAndroidSurfaceCreateInfoKHR {
  fn default() -> VkAndroidSurfaceCreateInfoKHR {
    VkAndroidSurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
unsafe impl RawStruct for VkAndroidSurfaceCreateInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32SurfaceCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkWin32SurfaceCreateFlagsKHR,
  pub hinstance: wsi::win32::HINSTANCE,
  pub hwnd: wsi::win32::HWND,
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkWin32SurfaceCreateInfoKHR {
  #[inline]
  pub fn new() -> VkWin32SurfaceCreateInfoKHR {
    unsafe {
      VkWin32SurfaceCreateInfoKHR {
        sType: VkStructureType::E_WIN32_SURFACE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkWin32SurfaceCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_hinstance(&self) -> wsi::win32::HINSTANCE {
    self.hinstance
  }
  #[inline]
  pub fn get_hwnd(&self) -> wsi::win32::HWND {
    self.hwnd
  }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkWin32SurfaceCreateInfoKHR {
  fn default() -> VkWin32SurfaceCreateInfoKHR {
    VkWin32SurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkWin32SurfaceCreateInfoKHR {
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
#[cfg(feature = "VK_EXT_debug_report")]
pub use types_raw::PFN_vkDebugReportCallbackEXT;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_debug_report")]
pub struct VkDebugReportCallbackCreateInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkDebugReportFlagsEXT,
  pub pfnCallback: PFN_vkDebugReportCallbackEXT,
  pUserData: *mut c_void,
}
#[cfg(feature = "VK_EXT_debug_report")]
impl VkDebugReportCallbackCreateInfoEXT {
  #[inline]
  pub fn new() -> VkDebugReportCallbackCreateInfoEXT {
    unsafe {
      VkDebugReportCallbackCreateInfoEXT {
        sType: VkStructureType::E_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkDebugReportFlagsEXT {
    self.flags
  }
  #[inline]
  pub fn get_pfn_callback(&self) -> PFN_vkDebugReportCallbackEXT {
    self.pfnCallback
  }
  #[inline]
  pub fn get_user_data(&self) -> *mut c_void {
    self.pUserData
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl Default for VkDebugReportCallbackCreateInfoEXT {
  fn default() -> VkDebugReportCallbackCreateInfoEXT {
    VkDebugReportCallbackCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl RawStruct for VkDebugReportCallbackCreateInfoEXT {
  type Raw = types_raw::VkDebugReportCallbackCreateInfoEXT;
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
#[cfg(feature = "VK_EXT_debug_report")]
pub type VkDebugReportCallbackEXT = VkNonDispatchableHandle<VkDebugReportCallbackEXT__>;

// feature: VK_AMD_rasterization_order
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_AMD_rasterization_order")]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
  sType: VkStructureType,
  pNext: *const c_void,
  pub rasterizationOrder: VkRasterizationOrderAMD,
}
#[cfg(feature = "VK_AMD_rasterization_order")]
impl VkPipelineRasterizationStateRasterizationOrderAMD {
  #[inline]
  pub fn new() -> VkPipelineRasterizationStateRasterizationOrderAMD {
    unsafe {
      VkPipelineRasterizationStateRasterizationOrderAMD {
        sType: VkStructureType::E_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_rasterization_order(&self) -> VkRasterizationOrderAMD {
    self.rasterizationOrder
  }
}
#[cfg(feature = "VK_AMD_rasterization_order")]
impl Default for VkPipelineRasterizationStateRasterizationOrderAMD {
  fn default() -> VkPipelineRasterizationStateRasterizationOrderAMD {
    VkPipelineRasterizationStateRasterizationOrderAMD::new()
  }
}
#[cfg(feature = "VK_AMD_rasterization_order")]
unsafe impl RawStruct for VkPipelineRasterizationStateRasterizationOrderAMD {
  type Raw = types_raw::VkPipelineRasterizationStateRasterizationOrderAMD;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerObjectNameInfoEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_object_type(&self) -> VkDebugReportObjectTypeEXT {
    self.objectType
  }
  #[inline]
  pub fn get_object(&self) -> u64 {
    self.object
  }
  #[inline]
  pub fn get_object_name(&self) -> &'a CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pObjectName) }
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerObjectTagInfoEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_object_type(&self) -> VkDebugReportObjectTypeEXT {
    self.objectType
  }
  #[inline]
  pub fn get_object(&self) -> u64 {
    self.object
  }
  #[inline]
  pub fn get_tag_name(&self) -> u64 {
    self.tagName
  }
  #[inline]
  pub fn get_tag_size(&self) -> usize {
    self.tagSize
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_debug_marker")]
pub struct VkDebugMarkerMarkerInfoEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEBUG_MARKER_MARKER_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_marker_name(&self) -> &'a CStr {
    unsafe { ::std::ffi::CStr::from_ptr(self.pMarkerName) }
  }
  #[inline]
  pub fn get_color(&self) -> [f32; 4] {
    self.color
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationImageCreateInfoNV {
  sType: VkStructureType,
  pNext: *const c_void,
  pub dedicatedAllocation: VkBool32,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl VkDedicatedAllocationImageCreateInfoNV {
  #[inline]
  pub fn new() -> VkDedicatedAllocationImageCreateInfoNV {
    unsafe {
      VkDedicatedAllocationImageCreateInfoNV {
        sType: VkStructureType::E_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dedicated_allocation(mut self, value: VkBool32) -> Self {
    self.dedicatedAllocation = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_dedicated_allocation(&self) -> VkBool32 {
    self.dedicatedAllocation
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl Default for VkDedicatedAllocationImageCreateInfoNV {
  fn default() -> VkDedicatedAllocationImageCreateInfoNV {
    VkDedicatedAllocationImageCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl RawStruct for VkDedicatedAllocationImageCreateInfoNV {
  type Raw = types_raw::VkDedicatedAllocationImageCreateInfoNV;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
  sType: VkStructureType,
  pNext: *const c_void,
  pub dedicatedAllocation: VkBool32,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl VkDedicatedAllocationBufferCreateInfoNV {
  #[inline]
  pub fn new() -> VkDedicatedAllocationBufferCreateInfoNV {
    unsafe {
      VkDedicatedAllocationBufferCreateInfoNV {
        sType: VkStructureType::E_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dedicated_allocation(mut self, value: VkBool32) -> Self {
    self.dedicatedAllocation = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_dedicated_allocation(&self) -> VkBool32 {
    self.dedicatedAllocation
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl Default for VkDedicatedAllocationBufferCreateInfoNV {
  fn default() -> VkDedicatedAllocationBufferCreateInfoNV {
    VkDedicatedAllocationBufferCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl RawStruct for VkDedicatedAllocationBufferCreateInfoNV {
  type Raw = types_raw::VkDedicatedAllocationBufferCreateInfoNV;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
  sType: VkStructureType,
  pNext: *const c_void,
  pub image: Option<VkImage>,
  pub buffer: Option<VkBuffer>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl VkDedicatedAllocationMemoryAllocateInfoNV {
  #[inline]
  pub fn new() -> VkDedicatedAllocationMemoryAllocateInfoNV {
    unsafe {
      VkDedicatedAllocationMemoryAllocateInfoNV {
        sType: VkStructureType::E_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_image(&self) -> Option<VkImage> {
    self.image
  }
  #[inline]
  pub fn get_buffer(&self) -> Option<VkBuffer> {
    self.buffer
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl Default for VkDedicatedAllocationMemoryAllocateInfoNV {
  fn default() -> VkDedicatedAllocationMemoryAllocateInfoNV {
    VkDedicatedAllocationMemoryAllocateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl RawStruct for VkDedicatedAllocationMemoryAllocateInfoNV {
  type Raw = types_raw::VkDedicatedAllocationMemoryAllocateInfoNV;
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_FEATURES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_features(mut self, value: VkPhysicalDeviceFeatures) -> Self {
    self.features = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_features(&self) -> VkPhysicalDeviceFeatures {
    self.features
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_features2_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceFeatures2KHR,
    VkPhysicalDeviceFeatures2KHR
  );
}
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
  pub fn new() -> VkPhysicalDeviceProperties2KHR {
    unsafe {
      VkPhysicalDeviceProperties2KHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_PROPERTIES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_properties(mut self, value: VkPhysicalDeviceProperties) -> Self {
    self.properties = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_properties(&self) -> VkPhysicalDeviceProperties {
    self.properties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceProperties2KHR {
  fn default() -> VkPhysicalDeviceProperties2KHR {
    VkPhysicalDeviceProperties2KHR::new()
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
  pub fn new() -> VkFormatProperties2KHR {
    unsafe {
      VkFormatProperties2KHR {
        sType: VkStructureType::E_FORMAT_PROPERTIES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_format_properties(mut self, value: VkFormatProperties) -> Self {
    self.formatProperties = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_format_properties(&self) -> VkFormatProperties {
    self.formatProperties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkFormatProperties2KHR {
  fn default() -> VkFormatProperties2KHR {
    VkFormatProperties2KHR::new()
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
  pub fn new() -> VkImageFormatProperties2KHR {
    unsafe {
      VkImageFormatProperties2KHR {
        sType: VkStructureType::E_IMAGE_FORMAT_PROPERTIES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_image_format_properties(mut self, value: VkImageFormatProperties) -> Self {
    self.imageFormatProperties = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_image_format_properties(&self) -> VkImageFormatProperties {
    self.imageFormatProperties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkImageFormatProperties2KHR {
  fn default() -> VkImageFormatProperties2KHR {
    VkImageFormatProperties2KHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceImageFormatInfo2KHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub format: VkFormat,
  pub eType: VkImageType,
  pub tiling: VkImageTiling,
  pub usage: VkImageUsageFlags,
  pub flags: VkImageCreateFlags,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkPhysicalDeviceImageFormatInfo2KHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceImageFormatInfo2KHR {
    unsafe {
      VkPhysicalDeviceImageFormatInfo2KHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_type(&self) -> VkImageType {
    self.eType
  }
  #[inline]
  pub fn get_tiling(&self) -> VkImageTiling {
    self.tiling
  }
  #[inline]
  pub fn get_usage(&self) -> VkImageUsageFlags {
    self.usage
  }
  #[inline]
  pub fn get_flags(&self) -> VkImageCreateFlags {
    self.flags
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceImageFormatInfo2KHR {
  fn default() -> VkPhysicalDeviceImageFormatInfo2KHR {
    VkPhysicalDeviceImageFormatInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl RawStruct for VkPhysicalDeviceImageFormatInfo2KHR {
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
  pub fn new() -> VkQueueFamilyProperties2KHR {
    unsafe {
      VkQueueFamilyProperties2KHR {
        sType: VkStructureType::E_QUEUE_FAMILY_PROPERTIES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_queue_family_properties(mut self, value: VkQueueFamilyProperties) -> Self {
    self.queueFamilyProperties = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_queue_family_properties(&self) -> VkQueueFamilyProperties {
    self.queueFamilyProperties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkQueueFamilyProperties2KHR {
  fn default() -> VkQueueFamilyProperties2KHR {
    VkQueueFamilyProperties2KHR::new()
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
  pub fn new() -> VkPhysicalDeviceMemoryProperties2KHR {
    unsafe {
      VkPhysicalDeviceMemoryProperties2KHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory_properties(mut self, value: VkPhysicalDeviceMemoryProperties) -> Self {
    self.memoryProperties = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory_properties(&self) -> VkPhysicalDeviceMemoryProperties {
    self.memoryProperties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceMemoryProperties2KHR {
  fn default() -> VkPhysicalDeviceMemoryProperties2KHR {
    VkPhysicalDeviceMemoryProperties2KHR::new()
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
  pub fn new() -> VkSparseImageFormatProperties2KHR {
    unsafe {
      VkSparseImageFormatProperties2KHR {
        sType: VkStructureType::E_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_properties(mut self, value: VkSparseImageFormatProperties) -> Self {
    self.properties = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_properties(&self) -> VkSparseImageFormatProperties {
    self.properties
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkSparseImageFormatProperties2KHR {
  fn default() -> VkSparseImageFormatProperties2KHR {
    VkSparseImageFormatProperties2KHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub format: VkFormat,
  pub eType: VkImageType,
  pub samples: VkSampleCountFlagBits,
  pub usage: VkImageUsageFlags,
  pub tiling: VkImageTiling,
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl VkPhysicalDeviceSparseImageFormatInfo2KHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceSparseImageFormatInfo2KHR {
    unsafe {
      VkPhysicalDeviceSparseImageFormatInfo2KHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_type(&self) -> VkImageType {
    self.eType
  }
  #[inline]
  pub fn get_samples(&self) -> VkSampleCountFlagBits {
    self.samples
  }
  #[inline]
  pub fn get_usage(&self) -> VkImageUsageFlags {
    self.usage
  }
  #[inline]
  pub fn get_tiling(&self) -> VkImageTiling {
    self.tiling
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceSparseImageFormatInfo2KHR {
  fn default() -> VkPhysicalDeviceSparseImageFormatInfo2KHR {
    VkPhysicalDeviceSparseImageFormatInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
unsafe impl RawStruct for VkPhysicalDeviceSparseImageFormatInfo2KHR {
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
  pub fn new() -> VkTextureLODGatherFormatPropertiesAMD {
    unsafe {
      VkTextureLODGatherFormatPropertiesAMD {
        sType: VkStructureType::E_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_supports_texture_gather_lod_bias_amd(mut self, value: VkBool32) -> Self {
    self.supportsTextureGatherLODBiasAMD = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_supports_texture_gather_lod_bias_amd(&self) -> VkBool32 {
    self.supportsTextureGatherLODBiasAMD
  }
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
impl Default for VkTextureLODGatherFormatPropertiesAMD {
  fn default() -> VkTextureLODGatherFormatPropertiesAMD {
    VkTextureLODGatherFormatPropertiesAMD::new()
  }
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
unsafe impl RawStruct for VkTextureLODGatherFormatPropertiesAMD {
  type Raw = types_raw::VkTextureLODGatherFormatPropertiesAMD;
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
  pub fn new() -> VkShaderResourceUsageAMD {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_num_used_vgprs(mut self, value: u32) -> Self {
    self.numUsedVgprs = value;
    self
  }
  #[inline]
  pub fn set_num_used_sgprs(mut self, value: u32) -> Self {
    self.numUsedSgprs = value;
    self
  }
  #[inline]
  pub fn set_lds_size_per_local_work_group(mut self, value: u32) -> Self {
    self.ldsSizePerLocalWorkGroup = value;
    self
  }
  #[inline]
  pub fn set_lds_usage_size_in_bytes(mut self, value: usize) -> Self {
    self.ldsUsageSizeInBytes = value;
    self
  }
  #[inline]
  pub fn set_scratch_mem_usage_in_bytes(mut self, value: usize) -> Self {
    self.scratchMemUsageInBytes = value;
    self
  }
  #[inline]
  pub fn get_num_used_vgprs(&self) -> u32 {
    self.numUsedVgprs
  }
  #[inline]
  pub fn get_num_used_sgprs(&self) -> u32 {
    self.numUsedSgprs
  }
  #[inline]
  pub fn get_lds_size_per_local_work_group(&self) -> u32 {
    self.ldsSizePerLocalWorkGroup
  }
  #[inline]
  pub fn get_lds_usage_size_in_bytes(&self) -> usize {
    self.ldsUsageSizeInBytes
  }
  #[inline]
  pub fn get_scratch_mem_usage_in_bytes(&self) -> usize {
    self.scratchMemUsageInBytes
  }
}
#[cfg(feature = "VK_AMD_shader_info")]
impl Default for VkShaderResourceUsageAMD {
  fn default() -> VkShaderResourceUsageAMD {
    VkShaderResourceUsageAMD::new()
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
  pub fn new() -> VkShaderStatisticsInfoAMD {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_shader_stage_mask(mut self, value: VkShaderStageFlags) -> Self {
    self.shaderStageMask = value;
    self
  }
  #[inline]
  pub fn set_resource_usage(mut self, value: VkShaderResourceUsageAMD) -> Self {
    self.resourceUsage = value;
    self
  }
  #[inline]
  pub fn set_num_physical_vgprs(mut self, value: u32) -> Self {
    self.numPhysicalVgprs = value;
    self
  }
  #[inline]
  pub fn set_num_physical_sgprs(mut self, value: u32) -> Self {
    self.numPhysicalSgprs = value;
    self
  }
  #[inline]
  pub fn set_num_available_vgprs(mut self, value: u32) -> Self {
    self.numAvailableVgprs = value;
    self
  }
  #[inline]
  pub fn set_num_available_sgprs(mut self, value: u32) -> Self {
    self.numAvailableSgprs = value;
    self
  }
  #[inline]
  pub fn set_compute_work_group_size(mut self, value: [u32; 3]) -> Self {
    self.computeWorkGroupSize = value;
    self
  }
  #[inline]
  pub fn get_shader_stage_mask(&self) -> VkShaderStageFlags {
    self.shaderStageMask
  }
  #[inline]
  pub fn get_resource_usage(&self) -> VkShaderResourceUsageAMD {
    self.resourceUsage
  }
  #[inline]
  pub fn get_num_physical_vgprs(&self) -> u32 {
    self.numPhysicalVgprs
  }
  #[inline]
  pub fn get_num_physical_sgprs(&self) -> u32 {
    self.numPhysicalSgprs
  }
  #[inline]
  pub fn get_num_available_vgprs(&self) -> u32 {
    self.numAvailableVgprs
  }
  #[inline]
  pub fn get_num_available_sgprs(&self) -> u32 {
    self.numAvailableSgprs
  }
  #[inline]
  pub fn get_compute_work_group_size(&self) -> [u32; 3] {
    self.computeWorkGroupSize
  }
}
#[cfg(feature = "VK_AMD_shader_info")]
impl Default for VkShaderStatisticsInfoAMD {
  fn default() -> VkShaderStatisticsInfoAMD {
    VkShaderStatisticsInfoAMD::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_multiview")]
pub struct VkRenderPassMultiviewCreateInfoKHX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_subpass_count(&self) -> u32 {
    self.subpassCount
  }
  #[inline]
  pub fn get_dependency_count(&self) -> u32 {
    self.dependencyCount
  }
  #[inline]
  pub fn get_correlation_mask_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_multiview_create_info_khx() {
  assert_size!(
    types_raw::VkRenderPassMultiviewCreateInfoKHX,
    VkRenderPassMultiviewCreateInfoKHX
  );
}
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_multiview(mut self, value: VkBool32) -> Self {
    self.multiview = value;
    self
  }
  #[inline]
  pub fn set_multiview_geometry_shader(mut self, value: VkBool32) -> Self {
    self.multiviewGeometryShader = value;
    self
  }
  #[inline]
  pub fn set_multiview_tessellation_shader(mut self, value: VkBool32) -> Self {
    self.multiviewTessellationShader = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_multiview(&self) -> VkBool32 {
    self.multiview
  }
  #[inline]
  pub fn get_multiview_geometry_shader(&self) -> VkBool32 {
    self.multiviewGeometryShader
  }
  #[inline]
  pub fn get_multiview_tessellation_shader(&self) -> VkBool32 {
    self.multiviewTessellationShader
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_multiview_features_khx() {
  assert_size!(
    types_raw::VkPhysicalDeviceMultiviewFeaturesKHX,
    VkPhysicalDeviceMultiviewFeaturesKHX
  );
}
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
  pub fn new() -> VkPhysicalDeviceMultiviewPropertiesKHX {
    unsafe {
      VkPhysicalDeviceMultiviewPropertiesKHX {
        sType: VkStructureType::E_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_max_multiview_view_count(mut self, value: u32) -> Self {
    self.maxMultiviewViewCount = value;
    self
  }
  #[inline]
  pub fn set_max_multiview_instance_index(mut self, value: u32) -> Self {
    self.maxMultiviewInstanceIndex = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_max_multiview_view_count(&self) -> u32 {
    self.maxMultiviewViewCount
  }
  #[inline]
  pub fn get_max_multiview_instance_index(&self) -> u32 {
    self.maxMultiviewInstanceIndex
  }
}
#[cfg(feature = "VK_KHX_multiview")]
impl Default for VkPhysicalDeviceMultiviewPropertiesKHX {
  fn default() -> VkPhysicalDeviceMultiviewPropertiesKHX {
    VkPhysicalDeviceMultiviewPropertiesKHX::new()
  }
}
#[cfg(feature = "VK_KHX_multiview")]
unsafe impl RawStruct for VkPhysicalDeviceMultiviewPropertiesKHX {
  type Raw = types_raw::VkPhysicalDeviceMultiviewPropertiesKHX;
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
  pub fn new() -> VkExternalImageFormatPropertiesNV {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_image_format_properties(mut self, value: VkImageFormatProperties) -> Self {
    self.imageFormatProperties = value;
    self
  }
  #[inline]
  pub fn set_external_memory_features(mut self, value: VkExternalMemoryFeatureFlagsNV) -> Self {
    self.externalMemoryFeatures = value;
    self
  }
  #[inline]
  pub fn set_export_from_imported_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsNV) -> Self {
    self.exportFromImportedHandleTypes = value;
    self
  }
  #[inline]
  pub fn set_compatible_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsNV) -> Self {
    self.compatibleHandleTypes = value;
    self
  }
  #[inline]
  pub fn get_image_format_properties(&self) -> VkImageFormatProperties {
    self.imageFormatProperties
  }
  #[inline]
  pub fn get_external_memory_features(&self) -> VkExternalMemoryFeatureFlagsNV {
    self.externalMemoryFeatures
  }
  #[inline]
  pub fn get_export_from_imported_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.exportFromImportedHandleTypes
  }
  #[inline]
  pub fn get_compatible_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.compatibleHandleTypes
  }
}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
impl Default for VkExternalImageFormatPropertiesNV {
  fn default() -> VkExternalImageFormatPropertiesNV {
    VkExternalImageFormatPropertiesNV::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory")]
pub struct VkExternalMemoryImageCreateInfoNV {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
#[cfg(feature = "VK_NV_external_memory")]
impl VkExternalMemoryImageCreateInfoNV {
  #[inline]
  pub fn new() -> VkExternalMemoryImageCreateInfoNV {
    unsafe {
      VkExternalMemoryImageCreateInfoNV {
        sType: VkStructureType::E_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.handleTypes
  }
}
#[cfg(feature = "VK_NV_external_memory")]
impl Default for VkExternalMemoryImageCreateInfoNV {
  fn default() -> VkExternalMemoryImageCreateInfoNV {
    VkExternalMemoryImageCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl RawStruct for VkExternalMemoryImageCreateInfoNV {
  type Raw = types_raw::VkExternalMemoryImageCreateInfoNV;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory")]
pub struct VkExportMemoryAllocateInfoNV {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
#[cfg(feature = "VK_NV_external_memory")]
impl VkExportMemoryAllocateInfoNV {
  #[inline]
  pub fn new() -> VkExportMemoryAllocateInfoNV {
    unsafe {
      VkExportMemoryAllocateInfoNV {
        sType: VkStructureType::E_EXPORT_MEMORY_ALLOCATE_INFO_NV,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.handleTypes
  }
}
#[cfg(feature = "VK_NV_external_memory")]
impl Default for VkExportMemoryAllocateInfoNV {
  fn default() -> VkExportMemoryAllocateInfoNV {
    VkExportMemoryAllocateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl RawStruct for VkExportMemoryAllocateInfoNV {
  type Raw = types_raw::VkExportMemoryAllocateInfoNV;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportMemoryWin32HandleInfoNV {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagsNV,
  pub handle: wsi::win32::HANDLE,
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkImportMemoryWin32HandleInfoNV {
  #[inline]
  pub fn new() -> VkImportMemoryWin32HandleInfoNV {
    unsafe {
      VkImportMemoryWin32HandleInfoNV {
        sType: VkStructureType::E_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalMemoryHandleTypeFlagsNV {
    self.handleType
  }
  #[inline]
  pub fn get_handle(&self) -> wsi::win32::HANDLE {
    self.handle
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkImportMemoryWin32HandleInfoNV {
  fn default() -> VkImportMemoryWin32HandleInfoNV {
    VkImportMemoryWin32HandleInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkImportMemoryWin32HandleInfoNV {
  type Raw = types_raw::VkImportMemoryWin32HandleInfoNV;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportMemoryWin32HandleInfoNV {
  sType: VkStructureType,
  pNext: *const c_void,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkExportMemoryWin32HandleInfoNV {
  #[inline]
  pub fn new() -> VkExportMemoryWin32HandleInfoNV {
    unsafe {
      VkExportMemoryWin32HandleInfoNV {
        sType: VkStructureType::E_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_dw_access(mut self, value: wsi::win32::DWORD) -> Self {
    self.dwAccess = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_attributes(&self) -> *const wsi::win32::SECURITY_ATTRIBUTES {
    self.pAttributes
  }
  #[inline]
  pub fn get_dw_access(&self) -> wsi::win32::DWORD {
    self.dwAccess
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkExportMemoryWin32HandleInfoNV {
  fn default() -> VkExportMemoryWin32HandleInfoNV {
    VkExportMemoryWin32HandleInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkExportMemoryWin32HandleInfoNV {
  type Raw = types_raw::VkExportMemoryWin32HandleInfoNV;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_acquire_count(&self) -> u32 {
    self.acquireCount
  }
  #[inline]
  pub fn get_release_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_win32_keyed_mutex_acquire_release_info_nv() {
  assert_size!(
    types_raw::VkWin32KeyedMutexAcquireReleaseInfoNV,
    VkWin32KeyedMutexAcquireReleaseInfoNV
  );
}

// feature: VK_KHX_device_group_creation
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
  pub fn new() -> VkPhysicalDeviceGroupPropertiesKHX {
    unsafe {
      VkPhysicalDeviceGroupPropertiesKHX {
        sType: VkStructureType::E_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_physical_device_count(mut self, value: u32) -> Self {
    self.physicalDeviceCount = value;
    self
  }
  #[inline]
  pub fn set_physical_devices(
    mut self,
    value: [VkPhysicalDevice; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize],
  ) -> Self {
    self.physicalDevices = value;
    self
  }
  #[inline]
  pub fn set_subset_allocation(mut self, value: VkBool32) -> Self {
    self.subsetAllocation = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_physical_device_count(&self) -> u32 {
    self.physicalDeviceCount
  }
  #[inline]
  pub fn get_physical_devices(&self) -> [VkPhysicalDevice; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize] {
    self.physicalDevices
  }
  #[inline]
  pub fn get_subset_allocation(&self) -> VkBool32 {
    self.subsetAllocation
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl Default for VkPhysicalDeviceGroupPropertiesKHX {
  fn default() -> VkPhysicalDeviceGroupPropertiesKHX {
    VkPhysicalDeviceGroupPropertiesKHX::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group_creation")]
pub struct VkDeviceGroupDeviceCreateInfoKHX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEVICE_GROUP_DEVICE_CREATE_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_physical_device_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_device_create_info_khx() {
  assert_size!(
    types_raw::VkDeviceGroupDeviceCreateInfoKHX,
    VkDeviceGroupDeviceCreateInfoKHX
  );
}

// feature: VK_KHX_device_group
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkMemoryAllocateFlagsInfoKHX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkMemoryAllocateFlagsKHX,
  pub deviceMask: u32,
}
#[cfg(feature = "VK_KHX_device_group")]
impl VkMemoryAllocateFlagsInfoKHX {
  #[inline]
  pub fn new() -> VkMemoryAllocateFlagsInfoKHX {
    unsafe {
      VkMemoryAllocateFlagsInfoKHX {
        sType: VkStructureType::E_MEMORY_ALLOCATE_FLAGS_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkMemoryAllocateFlagsKHX {
    self.flags
  }
  #[inline]
  pub fn get_device_mask(&self) -> u32 {
    self.deviceMask
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkMemoryAllocateFlagsInfoKHX {
  fn default() -> VkMemoryAllocateFlagsInfoKHX {
    VkMemoryAllocateFlagsInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl RawStruct for VkMemoryAllocateFlagsInfoKHX {
  type Raw = types_raw::VkMemoryAllocateFlagsInfoKHX;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupRenderPassBeginInfoKHX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_device_mask(&self) -> u32 {
    self.deviceMask
  }
  #[inline]
  pub fn get_device_render_area_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_render_pass_begin_info_khx() {
  assert_size!(
    types_raw::VkDeviceGroupRenderPassBeginInfoKHX,
    VkDeviceGroupRenderPassBeginInfoKHX
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupCommandBufferBeginInfoKHX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub deviceMask: u32,
}
#[cfg(feature = "VK_KHX_device_group")]
impl VkDeviceGroupCommandBufferBeginInfoKHX {
  #[inline]
  pub fn new() -> VkDeviceGroupCommandBufferBeginInfoKHX {
    unsafe {
      VkDeviceGroupCommandBufferBeginInfoKHX {
        sType: VkStructureType::E_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_device_mask(&self) -> u32 {
    self.deviceMask
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkDeviceGroupCommandBufferBeginInfoKHX {
  fn default() -> VkDeviceGroupCommandBufferBeginInfoKHX {
    VkDeviceGroupCommandBufferBeginInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl RawStruct for VkDeviceGroupCommandBufferBeginInfoKHX {
  type Raw = types_raw::VkDeviceGroupCommandBufferBeginInfoKHX;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupSubmitInfoKHX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEVICE_GROUP_SUBMIT_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_wait_semaphore_count(&self) -> u32 {
    self.waitSemaphoreCount
  }
  #[inline]
  pub fn get_command_buffer_count(&self) -> u32 {
    self.commandBufferCount
  }
  #[inline]
  pub fn get_signal_semaphore_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_submit_info_khx() {
  assert_size!(
    types_raw::VkDeviceGroupSubmitInfoKHX,
    VkDeviceGroupSubmitInfoKHX
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupBindSparseInfoKHX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub resourceDeviceIndex: u32,
  pub memoryDeviceIndex: u32,
}
#[cfg(feature = "VK_KHX_device_group")]
impl VkDeviceGroupBindSparseInfoKHX {
  #[inline]
  pub fn new() -> VkDeviceGroupBindSparseInfoKHX {
    unsafe {
      VkDeviceGroupBindSparseInfoKHX {
        sType: VkStructureType::E_DEVICE_GROUP_BIND_SPARSE_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_resource_device_index(&self) -> u32 {
    self.resourceDeviceIndex
  }
  #[inline]
  pub fn get_memory_device_index(&self) -> u32 {
    self.memoryDeviceIndex
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkDeviceGroupBindSparseInfoKHX {
  fn default() -> VkDeviceGroupBindSparseInfoKHX {
    VkDeviceGroupBindSparseInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl RawStruct for VkDeviceGroupBindSparseInfoKHX {
  type Raw = types_raw::VkDeviceGroupBindSparseInfoKHX;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_device_index_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_buffer_memory_device_group_info_khx() {
  assert_size!(
    types_raw::VkBindBufferMemoryDeviceGroupInfoKHX,
    VkBindBufferMemoryDeviceGroupInfoKHX
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindImageMemoryDeviceGroupInfoKHX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_device_index_count(&self) -> u32 {
    self.deviceIndexCount
  }
  #[inline]
  pub fn get_sfr_rect_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_image_memory_device_group_info_khx() {
  assert_size!(
    types_raw::VkBindImageMemoryDeviceGroupInfoKHX,
    VkBindImageMemoryDeviceGroupInfoKHX
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupPresentCapabilitiesKHX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub presentMask: [u32; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize],
  pub modes: VkDeviceGroupPresentModeFlagsKHX,
}
#[cfg(feature = "VK_KHX_device_group")]
impl VkDeviceGroupPresentCapabilitiesKHX {
  #[inline]
  pub fn new() -> VkDeviceGroupPresentCapabilitiesKHX {
    unsafe {
      VkDeviceGroupPresentCapabilitiesKHX {
        sType: VkStructureType::E_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_present_mask(mut self, value: [u32; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize]) -> Self {
    self.presentMask = value;
    self
  }
  #[inline]
  pub fn set_modes(mut self, value: VkDeviceGroupPresentModeFlagsKHX) -> Self {
    self.modes = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_present_mask(&self) -> [u32; enums::VK_MAX_DEVICE_GROUP_SIZE_KHX as usize] {
    self.presentMask
  }
  #[inline]
  pub fn get_modes(&self) -> VkDeviceGroupPresentModeFlagsKHX {
    self.modes
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkDeviceGroupPresentCapabilitiesKHX {
  fn default() -> VkDeviceGroupPresentCapabilitiesKHX {
    VkDeviceGroupPresentCapabilitiesKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl RawStruct for VkDeviceGroupPresentCapabilitiesKHX {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkImageSwapchainCreateInfoKHX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub swapchain: Option<VkSwapchainKHR>,
}
#[cfg(feature = "VK_KHX_device_group")]
impl VkImageSwapchainCreateInfoKHX {
  #[inline]
  pub fn new() -> VkImageSwapchainCreateInfoKHX {
    unsafe {
      VkImageSwapchainCreateInfoKHX {
        sType: VkStructureType::E_IMAGE_SWAPCHAIN_CREATE_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_swapchain(&self) -> Option<VkSwapchainKHR> {
    self.swapchain
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkImageSwapchainCreateInfoKHX {
  fn default() -> VkImageSwapchainCreateInfoKHX {
    VkImageSwapchainCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl RawStruct for VkImageSwapchainCreateInfoKHX {
  type Raw = types_raw::VkImageSwapchainCreateInfoKHX;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkBindImageMemorySwapchainInfoKHX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub swapchain: VkSwapchainKHR,
  pub imageIndex: u32,
}
#[cfg(feature = "VK_KHX_device_group")]
impl VkBindImageMemorySwapchainInfoKHX {
  #[inline]
  pub fn new() -> VkBindImageMemorySwapchainInfoKHX {
    unsafe {
      VkBindImageMemorySwapchainInfoKHX {
        sType: VkStructureType::E_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_swapchain(&self) -> VkSwapchainKHR {
    self.swapchain
  }
  #[inline]
  pub fn get_image_index(&self) -> u32 {
    self.imageIndex
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkBindImageMemorySwapchainInfoKHX {
  fn default() -> VkBindImageMemorySwapchainInfoKHX {
    VkBindImageMemorySwapchainInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl RawStruct for VkBindImageMemorySwapchainInfoKHX {
  type Raw = types_raw::VkBindImageMemorySwapchainInfoKHX;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkAcquireNextImageInfoKHX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub swapchain: VkSwapchainKHR,
  pub timeout: u64,
  pub semaphore: Option<VkSemaphore>,
  pub fence: Option<VkFence>,
  pub deviceMask: u32,
}
#[cfg(feature = "VK_KHX_device_group")]
impl VkAcquireNextImageInfoKHX {
  #[inline]
  pub fn new() -> VkAcquireNextImageInfoKHX {
    unsafe {
      VkAcquireNextImageInfoKHX {
        sType: VkStructureType::E_ACQUIRE_NEXT_IMAGE_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_swapchain(&self) -> VkSwapchainKHR {
    self.swapchain
  }
  #[inline]
  pub fn get_timeout(&self) -> u64 {
    self.timeout
  }
  #[inline]
  pub fn get_semaphore(&self) -> Option<VkSemaphore> {
    self.semaphore
  }
  #[inline]
  pub fn get_fence(&self) -> Option<VkFence> {
    self.fence
  }
  #[inline]
  pub fn get_device_mask(&self) -> u32 {
    self.deviceMask
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkAcquireNextImageInfoKHX {
  fn default() -> VkAcquireNextImageInfoKHX {
    VkAcquireNextImageInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl RawStruct for VkAcquireNextImageInfoKHX {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupPresentInfoKHX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_DEVICE_GROUP_PRESENT_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_swapchain_count(&self) -> u32 {
    self.swapchainCount
  }
  #[inline]
  pub fn get_mode(&self) -> VkDeviceGroupPresentModeFlagBitsKHX {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_group_present_info_khx() {
  assert_size!(
    types_raw::VkDeviceGroupPresentInfoKHX,
    VkDeviceGroupPresentInfoKHX
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHX_device_group")]
pub struct VkDeviceGroupSwapchainCreateInfoKHX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub modes: VkDeviceGroupPresentModeFlagsKHX,
}
#[cfg(feature = "VK_KHX_device_group")]
impl VkDeviceGroupSwapchainCreateInfoKHX {
  #[inline]
  pub fn new() -> VkDeviceGroupSwapchainCreateInfoKHX {
    unsafe {
      VkDeviceGroupSwapchainCreateInfoKHX {
        sType: VkStructureType::E_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_modes(&self) -> VkDeviceGroupPresentModeFlagsKHX {
    self.modes
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkDeviceGroupSwapchainCreateInfoKHX {
  fn default() -> VkDeviceGroupSwapchainCreateInfoKHX {
    VkDeviceGroupSwapchainCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
unsafe impl RawStruct for VkDeviceGroupSwapchainCreateInfoKHX {
  type Raw = types_raw::VkDeviceGroupSwapchainCreateInfoKHX;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_validation_flags")]
pub struct VkValidationFlagsEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_VALIDATION_FLAGS_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_disabled_validation_check_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_validation_flags_ext() {
  assert_size!(types_raw::VkValidationFlagsEXT, VkValidationFlagsEXT);
}

// feature: VK_NN_vi_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub struct VkViSurfaceCreateInfoNN {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkViSurfaceCreateFlagsNN,
  window: *mut c_void,
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
impl VkViSurfaceCreateInfoNN {
  #[inline]
  pub fn new() -> VkViSurfaceCreateInfoNN {
    unsafe {
      VkViSurfaceCreateInfoNN {
        sType: VkStructureType::E_VI_SURFACE_CREATE_INFO_NN,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkViSurfaceCreateFlagsNN {
    self.flags
  }
  #[inline]
  pub fn get_window(&self) -> *mut c_void {
    self.window
  }
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
impl Default for VkViSurfaceCreateInfoNN {
  fn default() -> VkViSurfaceCreateInfoNN {
    VkViSurfaceCreateInfoNN::new()
  }
}
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
unsafe impl RawStruct for VkViSurfaceCreateInfoNN {
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
  pub fn new() -> VkExternalMemoryPropertiesKHR {
    unsafe { ::std::mem::zeroed() }
  }
  #[inline]
  pub fn set_external_memory_features(mut self, value: VkExternalMemoryFeatureFlagsKHR) -> Self {
    self.externalMemoryFeatures = value;
    self
  }
  #[inline]
  pub fn set_export_from_imported_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
    self.exportFromImportedHandleTypes = value;
    self
  }
  #[inline]
  pub fn set_compatible_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
    self.compatibleHandleTypes = value;
    self
  }
  #[inline]
  pub fn get_external_memory_features(&self) -> VkExternalMemoryFeatureFlagsKHR {
    self.externalMemoryFeatures
  }
  #[inline]
  pub fn get_export_from_imported_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.exportFromImportedHandleTypes
  }
  #[inline]
  pub fn get_compatible_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.compatibleHandleTypes
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkExternalMemoryPropertiesKHR {
  fn default() -> VkExternalMemoryPropertiesKHR {
    VkExternalMemoryPropertiesKHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl VkPhysicalDeviceExternalImageFormatInfoKHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalImageFormatInfoKHR {
    unsafe {
      VkPhysicalDeviceExternalImageFormatInfoKHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkPhysicalDeviceExternalImageFormatInfoKHR {
  fn default() -> VkPhysicalDeviceExternalImageFormatInfoKHR {
    VkPhysicalDeviceExternalImageFormatInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl RawStruct for VkPhysicalDeviceExternalImageFormatInfoKHR {
  type Raw = types_raw::VkPhysicalDeviceExternalImageFormatInfoKHR;
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
  pub fn new() -> VkExternalImageFormatPropertiesKHR {
    unsafe {
      VkExternalImageFormatPropertiesKHR {
        sType: VkStructureType::E_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_external_memory_properties(mut self, value: VkExternalMemoryPropertiesKHR) -> Self {
    self.externalMemoryProperties = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_external_memory_properties(&self) -> VkExternalMemoryPropertiesKHR {
    self.externalMemoryProperties
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkExternalImageFormatPropertiesKHR {
  fn default() -> VkExternalImageFormatPropertiesKHR {
    VkExternalImageFormatPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl RawStruct for VkExternalImageFormatPropertiesKHR {
  type Raw = types_raw::VkExternalImageFormatPropertiesKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub struct VkPhysicalDeviceExternalBufferInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkBufferCreateFlags,
  pub usage: VkBufferUsageFlags,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl VkPhysicalDeviceExternalBufferInfoKHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalBufferInfoKHR {
    unsafe {
      VkPhysicalDeviceExternalBufferInfoKHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkBufferCreateFlags {
    self.flags
  }
  #[inline]
  pub fn get_usage(&self) -> VkBufferUsageFlags {
    self.usage
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkPhysicalDeviceExternalBufferInfoKHR {
  fn default() -> VkPhysicalDeviceExternalBufferInfoKHR {
    VkPhysicalDeviceExternalBufferInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl RawStruct for VkPhysicalDeviceExternalBufferInfoKHR {
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
  pub fn new() -> VkExternalBufferPropertiesKHR {
    unsafe {
      VkExternalBufferPropertiesKHR {
        sType: VkStructureType::E_EXTERNAL_BUFFER_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_external_memory_properties(mut self, value: VkExternalMemoryPropertiesKHR) -> Self {
    self.externalMemoryProperties = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_external_memory_properties(&self) -> VkExternalMemoryPropertiesKHR {
    self.externalMemoryProperties
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkExternalBufferPropertiesKHR {
  fn default() -> VkExternalBufferPropertiesKHR {
    VkExternalBufferPropertiesKHR::new()
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
  pub fn new() -> VkPhysicalDeviceIDPropertiesKHR {
    unsafe {
      VkPhysicalDeviceIDPropertiesKHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_ID_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_device_uuid(mut self, value: [u8; enums::VK_UUID_SIZE as usize]) -> Self {
    self.deviceUUID = value;
    self
  }
  #[inline]
  pub fn set_driver_uuid(mut self, value: [u8; enums::VK_UUID_SIZE as usize]) -> Self {
    self.driverUUID = value;
    self
  }
  #[inline]
  pub fn set_device_luid(mut self, value: [u8; enums::VK_LUID_SIZE_KHR as usize]) -> Self {
    self.deviceLUID = value;
    self
  }
  #[inline]
  pub fn set_device_node_mask(mut self, value: u32) -> Self {
    self.deviceNodeMask = value;
    self
  }
  #[inline]
  pub fn set_device_luid_valid(mut self, value: VkBool32) -> Self {
    self.deviceLUIDValid = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_device_uuid(&self) -> [u8; enums::VK_UUID_SIZE as usize] {
    self.deviceUUID
  }
  #[inline]
  pub fn get_driver_uuid(&self) -> [u8; enums::VK_UUID_SIZE as usize] {
    self.driverUUID
  }
  #[inline]
  pub fn get_device_luid(&self) -> [u8; enums::VK_LUID_SIZE_KHR as usize] {
    self.deviceLUID
  }
  #[inline]
  pub fn get_device_node_mask(&self) -> u32 {
    self.deviceNodeMask
  }
  #[inline]
  pub fn get_device_luid_valid(&self) -> VkBool32 {
    self.deviceLUIDValid
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkPhysicalDeviceIDPropertiesKHR {
  fn default() -> VkPhysicalDeviceIDPropertiesKHR {
    VkPhysicalDeviceIDPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
unsafe impl RawStruct for VkPhysicalDeviceIDPropertiesKHR {
  type Raw = types_raw::VkPhysicalDeviceIDPropertiesKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExternalMemoryImageCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl VkExternalMemoryImageCreateInfoKHR {
  #[inline]
  pub fn new() -> VkExternalMemoryImageCreateInfoKHR {
    unsafe {
      VkExternalMemoryImageCreateInfoKHR {
        sType: VkStructureType::E_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl Default for VkExternalMemoryImageCreateInfoKHR {
  fn default() -> VkExternalMemoryImageCreateInfoKHR {
    VkExternalMemoryImageCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl RawStruct for VkExternalMemoryImageCreateInfoKHR {
  type Raw = types_raw::VkExternalMemoryImageCreateInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExternalMemoryBufferCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl VkExternalMemoryBufferCreateInfoKHR {
  #[inline]
  pub fn new() -> VkExternalMemoryBufferCreateInfoKHR {
    unsafe {
      VkExternalMemoryBufferCreateInfoKHR {
        sType: VkStructureType::E_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl Default for VkExternalMemoryBufferCreateInfoKHR {
  fn default() -> VkExternalMemoryBufferCreateInfoKHR {
    VkExternalMemoryBufferCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl RawStruct for VkExternalMemoryBufferCreateInfoKHR {
  type Raw = types_raw::VkExternalMemoryBufferCreateInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory")]
pub struct VkExportMemoryAllocateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_memory")]
impl VkExportMemoryAllocateInfoKHR {
  #[inline]
  pub fn new() -> VkExportMemoryAllocateInfoKHR {
    unsafe {
      VkExportMemoryAllocateInfoKHR {
        sType: VkStructureType::E_EXPORT_MEMORY_ALLOCATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_types(&self) -> VkExternalMemoryHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl Default for VkExportMemoryAllocateInfoKHR {
  fn default() -> VkExportMemoryAllocateInfoKHR {
    VkExportMemoryAllocateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
unsafe impl RawStruct for VkExportMemoryAllocateInfoKHR {
  type Raw = types_raw::VkExportMemoryAllocateInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportMemoryWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkImportMemoryWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkImportMemoryWin32HandleInfoKHR {
    unsafe {
      VkImportMemoryWin32HandleInfoKHR {
        sType: VkStructureType::E_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn get_handle(&self) -> wsi::win32::HANDLE {
    self.handle
  }
  #[inline]
  pub fn get_name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkImportMemoryWin32HandleInfoKHR {
  fn default() -> VkImportMemoryWin32HandleInfoKHR {
    VkImportMemoryWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkImportMemoryWin32HandleInfoKHR {
  type Raw = types_raw::VkImportMemoryWin32HandleInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportMemoryWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkExportMemoryWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkExportMemoryWin32HandleInfoKHR {
    unsafe {
      VkExportMemoryWin32HandleInfoKHR {
        sType: VkStructureType::E_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_attributes(&self) -> *const wsi::win32::SECURITY_ATTRIBUTES {
    self.pAttributes
  }
  #[inline]
  pub fn get_dw_access(&self) -> wsi::win32::DWORD {
    self.dwAccess
  }
  #[inline]
  pub fn get_name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkExportMemoryWin32HandleInfoKHR {
  fn default() -> VkExportMemoryWin32HandleInfoKHR {
    VkExportMemoryWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkExportMemoryWin32HandleInfoKHR {
  type Raw = types_raw::VkExportMemoryWin32HandleInfoKHR;
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
  pub fn new() -> VkMemoryWin32HandlePropertiesKHR {
    unsafe {
      VkMemoryWin32HandlePropertiesKHR {
        sType: VkStructureType::E_MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory_type_bits(mut self, value: u32) -> Self {
    self.memoryTypeBits = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory_type_bits(&self) -> u32 {
    self.memoryTypeBits
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkMemoryWin32HandlePropertiesKHR {
  fn default() -> VkMemoryWin32HandlePropertiesKHR {
    VkMemoryWin32HandlePropertiesKHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkMemoryGetWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkMemoryGetWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkMemoryGetWin32HandleInfoKHR {
    unsafe {
      VkMemoryGetWin32HandleInfoKHR {
        sType: VkStructureType::E_MEMORY_GET_WIN32_HANDLE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory(&self) -> VkDeviceMemory {
    self.memory
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkMemoryGetWin32HandleInfoKHR {
  fn default() -> VkMemoryGetWin32HandleInfoKHR {
    VkMemoryGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkMemoryGetWin32HandleInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkImportMemoryFdInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pub fd: c_int,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl VkImportMemoryFdInfoKHR {
  #[inline]
  pub fn new() -> VkImportMemoryFdInfoKHR {
    unsafe {
      VkImportMemoryFdInfoKHR {
        sType: VkStructureType::E_IMPORT_MEMORY_FD_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn get_fd(&self) -> c_int {
    self.fd
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl Default for VkImportMemoryFdInfoKHR {
  fn default() -> VkImportMemoryFdInfoKHR {
    VkImportMemoryFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl RawStruct for VkImportMemoryFdInfoKHR {
  type Raw = types_raw::VkImportMemoryFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_memory_fd_info_khr() {
  assert_size!(types_raw::VkImportMemoryFdInfoKHR, VkImportMemoryFdInfoKHR);
}
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
  pub fn new() -> VkMemoryFdPropertiesKHR {
    unsafe {
      VkMemoryFdPropertiesKHR {
        sType: VkStructureType::E_MEMORY_FD_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory_type_bits(mut self, value: u32) -> Self {
    self.memoryTypeBits = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory_type_bits(&self) -> u32 {
    self.memoryTypeBits
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl Default for VkMemoryFdPropertiesKHR {
  fn default() -> VkMemoryFdPropertiesKHR {
    VkMemoryFdPropertiesKHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub struct VkMemoryGetFdInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl VkMemoryGetFdInfoKHR {
  #[inline]
  pub fn new() -> VkMemoryGetFdInfoKHR {
    unsafe {
      VkMemoryGetFdInfoKHR {
        sType: VkStructureType::E_MEMORY_GET_FD_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory(&self) -> VkDeviceMemory {
    self.memory
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl Default for VkMemoryGetFdInfoKHR {
  fn default() -> VkMemoryGetFdInfoKHR {
    VkMemoryGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
unsafe impl RawStruct for VkMemoryGetFdInfoKHR {
  type Raw = types_raw::VkMemoryGetFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_get_fd_info_khr() {
  assert_size!(types_raw::VkMemoryGetFdInfoKHR, VkMemoryGetFdInfoKHR);
}

// feature: VK_KHR_win32_keyed_mutex
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_acquire_count(&self) -> u32 {
    self.acquireCount
  }
  #[inline]
  pub fn get_release_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_win32_keyed_mutex_acquire_release_info_khr() {
  assert_size!(
    types_raw::VkWin32KeyedMutexAcquireReleaseInfoKHR,
    VkWin32KeyedMutexAcquireReleaseInfoKHR
  );
}

// feature: VK_KHR_external_semaphore_capabilities
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl VkPhysicalDeviceExternalSemaphoreInfoKHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalSemaphoreInfoKHR {
    unsafe {
      VkPhysicalDeviceExternalSemaphoreInfoKHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl Default for VkPhysicalDeviceExternalSemaphoreInfoKHR {
  fn default() -> VkPhysicalDeviceExternalSemaphoreInfoKHR {
    VkPhysicalDeviceExternalSemaphoreInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
unsafe impl RawStruct for VkPhysicalDeviceExternalSemaphoreInfoKHR {
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
  pub fn new() -> VkExternalSemaphorePropertiesKHR {
    unsafe {
      VkExternalSemaphorePropertiesKHR {
        sType: VkStructureType::E_EXTERNAL_SEMAPHORE_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_export_from_imported_handle_types(mut self, value: VkExternalSemaphoreHandleTypeFlagsKHR) -> Self {
    self.exportFromImportedHandleTypes = value;
    self
  }
  #[inline]
  pub fn set_compatible_handle_types(mut self, value: VkExternalSemaphoreHandleTypeFlagsKHR) -> Self {
    self.compatibleHandleTypes = value;
    self
  }
  #[inline]
  pub fn set_external_semaphore_features(mut self, value: VkExternalSemaphoreFeatureFlagsKHR) -> Self {
    self.externalSemaphoreFeatures = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_export_from_imported_handle_types(&self) -> VkExternalSemaphoreHandleTypeFlagsKHR {
    self.exportFromImportedHandleTypes
  }
  #[inline]
  pub fn get_compatible_handle_types(&self) -> VkExternalSemaphoreHandleTypeFlagsKHR {
    self.compatibleHandleTypes
  }
  #[inline]
  pub fn get_external_semaphore_features(&self) -> VkExternalSemaphoreFeatureFlagsKHR {
    self.externalSemaphoreFeatures
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl Default for VkExternalSemaphorePropertiesKHR {
  fn default() -> VkExternalSemaphorePropertiesKHR {
    VkExternalSemaphorePropertiesKHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore")]
pub struct VkExportSemaphoreCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore")]
impl VkExportSemaphoreCreateInfoKHR {
  #[inline]
  pub fn new() -> VkExportSemaphoreCreateInfoKHR {
    unsafe {
      VkExportSemaphoreCreateInfoKHR {
        sType: VkStructureType::E_EXPORT_SEMAPHORE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_types(&self) -> VkExternalSemaphoreHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
impl Default for VkExportSemaphoreCreateInfoKHR {
  fn default() -> VkExportSemaphoreCreateInfoKHR {
    VkExportSemaphoreCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
unsafe impl RawStruct for VkExportSemaphoreCreateInfoKHR {
  type Raw = types_raw::VkExportSemaphoreCreateInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub flags: VkSemaphoreImportFlagsKHR,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkImportSemaphoreWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkImportSemaphoreWin32HandleInfoKHR {
    unsafe {
      VkImportSemaphoreWin32HandleInfoKHR {
        sType: VkStructureType::E_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_semaphore(&self) -> VkSemaphore {
    self.semaphore
  }
  #[inline]
  pub fn get_flags(&self) -> VkSemaphoreImportFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn get_handle(&self) -> wsi::win32::HANDLE {
    self.handle
  }
  #[inline]
  pub fn get_name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkImportSemaphoreWin32HandleInfoKHR {
  fn default() -> VkImportSemaphoreWin32HandleInfoKHR {
    VkImportSemaphoreWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkImportSemaphoreWin32HandleInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkExportSemaphoreWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkExportSemaphoreWin32HandleInfoKHR {
    unsafe {
      VkExportSemaphoreWin32HandleInfoKHR {
        sType: VkStructureType::E_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_attributes(&self) -> *const wsi::win32::SECURITY_ATTRIBUTES {
    self.pAttributes
  }
  #[inline]
  pub fn get_dw_access(&self) -> wsi::win32::DWORD {
    self.dwAccess
  }
  #[inline]
  pub fn get_name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkExportSemaphoreWin32HandleInfoKHR {
  fn default() -> VkExportSemaphoreWin32HandleInfoKHR {
    VkExportSemaphoreWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkExportSemaphoreWin32HandleInfoKHR {
  type Raw = types_raw::VkExportSemaphoreWin32HandleInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkD3D12FenceSubmitInfoKHR<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_D3D12_FENCE_SUBMIT_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_wait_semaphore_values_count(&self) -> u32 {
    self.waitSemaphoreValuesCount
  }
  #[inline]
  pub fn get_signal_semaphore_values_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_d3_d12_fence_submit_info_khr() {
  assert_size!(
    types_raw::VkD3D12FenceSubmitInfoKHR,
    VkD3D12FenceSubmitInfoKHR
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkSemaphoreGetWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkSemaphoreGetWin32HandleInfoKHR {
    unsafe {
      VkSemaphoreGetWin32HandleInfoKHR {
        sType: VkStructureType::E_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_semaphore(&self) -> VkSemaphore {
    self.semaphore
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkSemaphoreGetWin32HandleInfoKHR {
  fn default() -> VkSemaphoreGetWin32HandleInfoKHR {
    VkSemaphoreGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkSemaphoreGetWin32HandleInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub struct VkImportSemaphoreFdInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub flags: VkSemaphoreImportFlagsKHR,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
  pub fd: c_int,
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl VkImportSemaphoreFdInfoKHR {
  #[inline]
  pub fn new() -> VkImportSemaphoreFdInfoKHR {
    unsafe {
      VkImportSemaphoreFdInfoKHR {
        sType: VkStructureType::E_IMPORT_SEMAPHORE_FD_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_semaphore(&self) -> VkSemaphore {
    self.semaphore
  }
  #[inline]
  pub fn get_flags(&self) -> VkSemaphoreImportFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn get_fd(&self) -> c_int {
    self.fd
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl Default for VkImportSemaphoreFdInfoKHR {
  fn default() -> VkImportSemaphoreFdInfoKHR {
    VkImportSemaphoreFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
unsafe impl RawStruct for VkImportSemaphoreFdInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub struct VkSemaphoreGetFdInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl VkSemaphoreGetFdInfoKHR {
  #[inline]
  pub fn new() -> VkSemaphoreGetFdInfoKHR {
    unsafe {
      VkSemaphoreGetFdInfoKHR {
        sType: VkStructureType::E_SEMAPHORE_GET_FD_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_semaphore(&self) -> VkSemaphore {
    self.semaphore
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalSemaphoreHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl Default for VkSemaphoreGetFdInfoKHR {
  fn default() -> VkSemaphoreGetFdInfoKHR {
    VkSemaphoreGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
unsafe impl RawStruct for VkSemaphoreGetFdInfoKHR {
  type Raw = types_raw::VkSemaphoreGetFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_get_fd_info_khr() {
  assert_size!(types_raw::VkSemaphoreGetFdInfoKHR, VkSemaphoreGetFdInfoKHR);
}

// feature: VK_KHR_push_descriptor
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_max_push_descriptors(mut self, value: u32) -> Self {
    self.maxPushDescriptors = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_max_push_descriptors(&self) -> u32 {
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_storage_buffer16_bit_access(mut self, value: VkBool32) -> Self {
    self.storageBuffer16BitAccess = value;
    self
  }
  #[inline]
  pub fn set_uniform_and_storage_buffer16_bit_access(mut self, value: VkBool32) -> Self {
    self.uniformAndStorageBuffer16BitAccess = value;
    self
  }
  #[inline]
  pub fn set_storage_push_constant16(mut self, value: VkBool32) -> Self {
    self.storagePushConstant16 = value;
    self
  }
  #[inline]
  pub fn set_storage_input_output16(mut self, value: VkBool32) -> Self {
    self.storageInputOutput16 = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_storage_buffer16_bit_access(&self) -> VkBool32 {
    self.storageBuffer16BitAccess
  }
  #[inline]
  pub fn get_uniform_and_storage_buffer16_bit_access(&self) -> VkBool32 {
    self.uniformAndStorageBuffer16BitAccess
  }
  #[inline]
  pub fn get_storage_push_constant16(&self) -> VkBool32 {
    self.storagePushConstant16
  }
  #[inline]
  pub fn get_storage_input_output16(&self) -> VkBool32 {
    self.storageInputOutput16
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device16_bit_storage_features_khr() {
  assert_size!(
    types_raw::VkPhysicalDevice16BitStorageFeaturesKHR,
    VkPhysicalDevice16BitStorageFeaturesKHR
  );
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
  pub fn get_offset(&self) -> VkOffset2D {
    self.offset
  }
  #[inline]
  pub fn get_extent(&self) -> VkExtent2D {
    self.extent
  }
  #[inline]
  pub fn get_layer(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_rectangle_count(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_incremental_present")]
pub struct VkPresentRegionsKHR<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PRESENT_REGIONS_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_swapchain_count(&self) -> u32 {
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
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type VkDescriptorUpdateTemplateKHR = VkNonDispatchableHandle<VkDescriptorUpdateTemplateKHR__>;
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
  pub fn get_dst_binding(&self) -> u32 {
    self.dstBinding
  }
  #[inline]
  pub fn get_dst_array_element(&self) -> u32 {
    self.dstArrayElement
  }
  #[inline]
  pub fn get_descriptor_count(&self) -> u32 {
    self.descriptorCount
  }
  #[inline]
  pub fn get_descriptor_type(&self) -> VkDescriptorType {
    self.descriptorType
  }
  #[inline]
  pub fn get_offset(&self) -> usize {
    self.offset
  }
  #[inline]
  pub fn get_stride(&self) -> usize {
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
#[repr(C)]
#[derive(Copy, Clone)]
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
        sType: VkStructureType::E_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkDescriptorUpdateTemplateCreateFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_descriptor_update_entry_count(&self) -> u32 {
    self.descriptorUpdateEntryCount
  }
  #[inline]
  pub fn get_template_type(&self) -> VkDescriptorUpdateTemplateTypeKHR {
    self.templateType
  }
  #[inline]
  pub fn get_descriptor_set_layout(&self) -> Option<VkDescriptorSetLayout> {
    self.descriptorSetLayout
  }
  #[inline]
  pub fn get_pipeline_bind_point(&self) -> VkPipelineBindPoint {
    self.pipelineBindPoint
  }
  #[inline]
  pub fn get_pipeline_layout(&self) -> Option<VkPipelineLayout> {
    self.pipelineLayout
  }
  #[inline]
  pub fn get_set(&self) -> u32 {
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
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkObjectTableNVX = VkNonDispatchableHandle<VkObjectTableNVX__>;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkIndirectCommandsLayoutNVX__ {}
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkIndirectCommandsLayoutNVX = VkNonDispatchableHandle<VkIndirectCommandsLayoutNVX__>;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkDeviceGeneratedCommandsFeaturesNVX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub computeBindingPointSupport: VkBool32,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkDeviceGeneratedCommandsFeaturesNVX {
  #[inline]
  pub fn new() -> VkDeviceGeneratedCommandsFeaturesNVX {
    unsafe {
      VkDeviceGeneratedCommandsFeaturesNVX {
        sType: VkStructureType::E_DEVICE_GENERATED_COMMANDS_FEATURES_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_compute_binding_point_support(mut self, value: VkBool32) -> Self {
    self.computeBindingPointSupport = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_compute_binding_point_support(&self) -> VkBool32 {
    self.computeBindingPointSupport
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkDeviceGeneratedCommandsFeaturesNVX {
  fn default() -> VkDeviceGeneratedCommandsFeaturesNVX {
    VkDeviceGeneratedCommandsFeaturesNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkDeviceGeneratedCommandsFeaturesNVX {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkDeviceGeneratedCommandsLimitsNVX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub maxIndirectCommandsLayoutTokenCount: u32,
  pub maxObjectEntryCounts: u32,
  pub minSequenceCountBufferOffsetAlignment: u32,
  pub minSequenceIndexBufferOffsetAlignment: u32,
  pub minCommandsTokenBufferOffsetAlignment: u32,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkDeviceGeneratedCommandsLimitsNVX {
  #[inline]
  pub fn new() -> VkDeviceGeneratedCommandsLimitsNVX {
    unsafe {
      VkDeviceGeneratedCommandsLimitsNVX {
        sType: VkStructureType::E_DEVICE_GENERATED_COMMANDS_LIMITS_NVX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_max_indirect_commands_layout_token_count(&self) -> u32 {
    self.maxIndirectCommandsLayoutTokenCount
  }
  #[inline]
  pub fn get_max_object_entry_counts(&self) -> u32 {
    self.maxObjectEntryCounts
  }
  #[inline]
  pub fn get_min_sequence_count_buffer_offset_alignment(&self) -> u32 {
    self.minSequenceCountBufferOffsetAlignment
  }
  #[inline]
  pub fn get_min_sequence_index_buffer_offset_alignment(&self) -> u32 {
    self.minSequenceIndexBufferOffsetAlignment
  }
  #[inline]
  pub fn get_min_commands_token_buffer_offset_alignment(&self) -> u32 {
    self.minCommandsTokenBufferOffsetAlignment
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkDeviceGeneratedCommandsLimitsNVX {
  fn default() -> VkDeviceGeneratedCommandsLimitsNVX {
    VkDeviceGeneratedCommandsLimitsNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkDeviceGeneratedCommandsLimitsNVX {
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
  pub fn get_token_type(&self) -> VkIndirectCommandsTokenTypeNVX {
    self.tokenType
  }
  #[inline]
  pub fn get_buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn get_offset(&self) -> VkDeviceSize {
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
  pub fn get_token_type(&self) -> VkIndirectCommandsTokenTypeNVX {
    self.tokenType
  }
  #[inline]
  pub fn get_binding_unit(&self) -> u32 {
    self.bindingUnit
  }
  #[inline]
  pub fn get_dynamic_count(&self) -> u32 {
    self.dynamicCount
  }
  #[inline]
  pub fn get_divisor(&self) -> u32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkIndirectCommandsLayoutCreateInfoNVX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_pipeline_bind_point(&self) -> VkPipelineBindPoint {
    self.pipelineBindPoint
  }
  #[inline]
  pub fn get_flags(&self) -> VkIndirectCommandsLayoutUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn get_token_count(&self) -> u32 {
    self.tokenCount
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkCmdProcessCommandsInfoNVX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_CMD_PROCESS_COMMANDS_INFO_NVX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_object_table(&self) -> VkObjectTableNVX {
    self.objectTable
  }
  #[inline]
  pub fn get_indirect_commands_layout(&self) -> VkIndirectCommandsLayoutNVX {
    self.indirectCommandsLayout
  }
  #[inline]
  pub fn get_indirect_commands_token_count(&self) -> u32 {
    self.indirectCommandsTokenCount
  }
  #[inline]
  pub fn get_max_sequences_count(&self) -> u32 {
    self.maxSequencesCount
  }
  #[inline]
  pub fn get_target_command_buffer(&self) -> Option<VkCommandBuffer> {
    self.targetCommandBuffer
  }
  #[inline]
  pub fn get_sequences_count_buffer(&self) -> Option<VkBuffer> {
    self.sequencesCountBuffer
  }
  #[inline]
  pub fn get_sequences_count_offset(&self) -> VkDeviceSize {
    self.sequencesCountOffset
  }
  #[inline]
  pub fn get_sequences_index_buffer(&self) -> Option<VkBuffer> {
    self.sequencesIndexBuffer
  }
  #[inline]
  pub fn get_sequences_index_offset(&self) -> VkDeviceSize {
    self.sequencesIndexOffset
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkCmdReserveSpaceForCommandsInfoNVX {
  sType: VkStructureType,
  pNext: *const c_void,
  pub objectTable: VkObjectTableNVX,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
  pub maxSequencesCount: u32,
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl VkCmdReserveSpaceForCommandsInfoNVX {
  #[inline]
  pub fn new() -> VkCmdReserveSpaceForCommandsInfoNVX {
    unsafe {
      VkCmdReserveSpaceForCommandsInfoNVX {
        sType: VkStructureType::E_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_object_table(&self) -> VkObjectTableNVX {
    self.objectTable
  }
  #[inline]
  pub fn get_indirect_commands_layout(&self) -> VkIndirectCommandsLayoutNVX {
    self.indirectCommandsLayout
  }
  #[inline]
  pub fn get_max_sequences_count(&self) -> u32 {
    self.maxSequencesCount
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkCmdReserveSpaceForCommandsInfoNVX {
  fn default() -> VkCmdReserveSpaceForCommandsInfoNVX {
    VkCmdReserveSpaceForCommandsInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
unsafe impl RawStruct for VkCmdReserveSpaceForCommandsInfoNVX {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub struct VkObjectTableCreateInfoNVX<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_OBJECT_TABLE_CREATE_INFO_NVX,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_object_count(&self) -> u32 {
    self.objectCount
  }
  #[inline]
  pub fn get_max_uniform_buffers_per_descriptor(&self) -> u32 {
    self.maxUniformBuffersPerDescriptor
  }
  #[inline]
  pub fn get_max_storage_buffers_per_descriptor(&self) -> u32 {
    self.maxStorageBuffersPerDescriptor
  }
  #[inline]
  pub fn get_max_storage_images_per_descriptor(&self) -> u32 {
    self.maxStorageImagesPerDescriptor
  }
  #[inline]
  pub fn get_max_sampled_images_per_descriptor(&self) -> u32 {
    self.maxSampledImagesPerDescriptor
  }
  #[inline]
  pub fn get_max_pipeline_layouts(&self) -> u32 {
    self.maxPipelineLayouts
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
  pub fn get_flags(&self) -> VkObjectEntryUsageFlagsNVX {
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
  pub fn get_flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn get_pipeline(&self) -> VkPipeline {
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
  pub fn get_flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn get_pipeline_layout(&self) -> VkPipelineLayout {
    self.pipelineLayout
  }
  #[inline]
  pub fn get_descriptor_set(&self) -> VkDescriptorSet {
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
  pub fn get_flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn get_buffer(&self) -> VkBuffer {
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
  pub fn get_flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn get_buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn get_index_type(&self) -> VkIndexType {
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
  pub fn get_flags(&self) -> VkObjectEntryUsageFlagsNVX {
    self.flags
  }
  #[inline]
  pub fn get_pipeline_layout(&self) -> VkPipelineLayout {
    self.pipelineLayout
  }
  #[inline]
  pub fn get_stage_flags(&self) -> VkShaderStageFlags {
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
  pub fn get_xcoeff(&self) -> f32 {
    self.xcoeff
  }
  #[inline]
  pub fn get_ycoeff(&self) -> f32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub struct VkPipelineViewportWScalingStateCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_viewport_w_scaling_enable(mut self, value: VkBool32) -> Self {
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_viewport_w_scaling_enable(&self) -> VkBool32 {
    self.viewportWScalingEnable
  }
  #[inline]
  pub fn get_viewport_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_viewport_w_scaling_state_create_info_nv() {
  assert_size!(
    types_raw::VkPipelineViewportWScalingStateCreateInfoNV,
    VkPipelineViewportWScalingStateCreateInfoNV
  );
}

// feature: VK_EXT_display_surface_counter
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
  pub fn new() -> VkSurfaceCapabilities2EXT {
    unsafe {
      VkSurfaceCapabilities2EXT {
        sType: VkStructureType::E_SURFACE_CAPABILITIES_2_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_min_image_count(mut self, value: u32) -> Self {
    self.minImageCount = value;
    self
  }
  #[inline]
  pub fn set_max_image_count(mut self, value: u32) -> Self {
    self.maxImageCount = value;
    self
  }
  #[inline]
  pub fn set_current_extent(mut self, value: VkExtent2D) -> Self {
    self.currentExtent = value;
    self
  }
  #[inline]
  pub fn set_min_image_extent(mut self, value: VkExtent2D) -> Self {
    self.minImageExtent = value;
    self
  }
  #[inline]
  pub fn set_max_image_extent(mut self, value: VkExtent2D) -> Self {
    self.maxImageExtent = value;
    self
  }
  #[inline]
  pub fn set_max_image_array_layers(mut self, value: u32) -> Self {
    self.maxImageArrayLayers = value;
    self
  }
  #[inline]
  pub fn set_supported_transforms(mut self, value: VkSurfaceTransformFlagsKHR) -> Self {
    self.supportedTransforms = value;
    self
  }
  #[inline]
  pub fn set_current_transform(mut self, value: VkSurfaceTransformFlagBitsKHR) -> Self {
    self.currentTransform = value;
    self
  }
  #[inline]
  pub fn set_supported_composite_alpha(mut self, value: VkCompositeAlphaFlagsKHR) -> Self {
    self.supportedCompositeAlpha = value;
    self
  }
  #[inline]
  pub fn set_supported_usage_flags(mut self, value: VkImageUsageFlags) -> Self {
    self.supportedUsageFlags = value;
    self
  }
  #[inline]
  pub fn set_supported_surface_counters(mut self, value: VkSurfaceCounterFlagsEXT) -> Self {
    self.supportedSurfaceCounters = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_min_image_count(&self) -> u32 {
    self.minImageCount
  }
  #[inline]
  pub fn get_max_image_count(&self) -> u32 {
    self.maxImageCount
  }
  #[inline]
  pub fn get_current_extent(&self) -> VkExtent2D {
    self.currentExtent
  }
  #[inline]
  pub fn get_min_image_extent(&self) -> VkExtent2D {
    self.minImageExtent
  }
  #[inline]
  pub fn get_max_image_extent(&self) -> VkExtent2D {
    self.maxImageExtent
  }
  #[inline]
  pub fn get_max_image_array_layers(&self) -> u32 {
    self.maxImageArrayLayers
  }
  #[inline]
  pub fn get_supported_transforms(&self) -> VkSurfaceTransformFlagsKHR {
    self.supportedTransforms
  }
  #[inline]
  pub fn get_current_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
    self.currentTransform
  }
  #[inline]
  pub fn get_supported_composite_alpha(&self) -> VkCompositeAlphaFlagsKHR {
    self.supportedCompositeAlpha
  }
  #[inline]
  pub fn get_supported_usage_flags(&self) -> VkImageUsageFlags {
    self.supportedUsageFlags
  }
  #[inline]
  pub fn get_supported_surface_counters(&self) -> VkSurfaceCounterFlagsEXT {
    self.supportedSurfaceCounters
  }
}
#[cfg(feature = "VK_EXT_display_surface_counter")]
impl Default for VkSurfaceCapabilities2EXT {
  fn default() -> VkSurfaceCapabilities2EXT {
    VkSurfaceCapabilities2EXT::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDisplayPowerInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub powerState: VkDisplayPowerStateEXT,
}
#[cfg(feature = "VK_EXT_display_control")]
impl VkDisplayPowerInfoEXT {
  #[inline]
  pub fn new() -> VkDisplayPowerInfoEXT {
    unsafe {
      VkDisplayPowerInfoEXT {
        sType: VkStructureType::E_DISPLAY_POWER_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_power_state(&self) -> VkDisplayPowerStateEXT {
    self.powerState
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl Default for VkDisplayPowerInfoEXT {
  fn default() -> VkDisplayPowerInfoEXT {
    VkDisplayPowerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl RawStruct for VkDisplayPowerInfoEXT {
  type Raw = types_raw::VkDisplayPowerInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_power_info_ext() {
  assert_size!(types_raw::VkDisplayPowerInfoEXT, VkDisplayPowerInfoEXT);
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDeviceEventInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub deviceEvent: VkDeviceEventTypeEXT,
}
#[cfg(feature = "VK_EXT_display_control")]
impl VkDeviceEventInfoEXT {
  #[inline]
  pub fn new() -> VkDeviceEventInfoEXT {
    unsafe {
      VkDeviceEventInfoEXT {
        sType: VkStructureType::E_DEVICE_EVENT_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_device_event(&self) -> VkDeviceEventTypeEXT {
    self.deviceEvent
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl Default for VkDeviceEventInfoEXT {
  fn default() -> VkDeviceEventInfoEXT {
    VkDeviceEventInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl RawStruct for VkDeviceEventInfoEXT {
  type Raw = types_raw::VkDeviceEventInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_event_info_ext() {
  assert_size!(types_raw::VkDeviceEventInfoEXT, VkDeviceEventInfoEXT);
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkDisplayEventInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub displayEvent: VkDisplayEventTypeEXT,
}
#[cfg(feature = "VK_EXT_display_control")]
impl VkDisplayEventInfoEXT {
  #[inline]
  pub fn new() -> VkDisplayEventInfoEXT {
    unsafe {
      VkDisplayEventInfoEXT {
        sType: VkStructureType::E_DISPLAY_EVENT_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_display_event(&self) -> VkDisplayEventTypeEXT {
    self.displayEvent
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl Default for VkDisplayEventInfoEXT {
  fn default() -> VkDisplayEventInfoEXT {
    VkDisplayEventInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl RawStruct for VkDisplayEventInfoEXT {
  type Raw = types_raw::VkDisplayEventInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_event_info_ext() {
  assert_size!(types_raw::VkDisplayEventInfoEXT, VkDisplayEventInfoEXT);
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_display_control")]
pub struct VkSwapchainCounterCreateInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub surfaceCounters: VkSurfaceCounterFlagsEXT,
}
#[cfg(feature = "VK_EXT_display_control")]
impl VkSwapchainCounterCreateInfoEXT {
  #[inline]
  pub fn new() -> VkSwapchainCounterCreateInfoEXT {
    unsafe {
      VkSwapchainCounterCreateInfoEXT {
        sType: VkStructureType::E_SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_surface_counters(&self) -> VkSurfaceCounterFlagsEXT {
    self.surfaceCounters
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl Default for VkSwapchainCounterCreateInfoEXT {
  fn default() -> VkSwapchainCounterCreateInfoEXT {
    VkSwapchainCounterCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl RawStruct for VkSwapchainCounterCreateInfoEXT {
  type Raw = types_raw::VkSwapchainCounterCreateInfoEXT;
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
  pub fn get_refresh_duration(&self) -> u64 {
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
  pub fn get_present_id(&self) -> u32 {
    self.presentID
  }
  #[inline]
  pub fn get_desired_present_time(&self) -> u64 {
    self.desiredPresentTime
  }
  #[inline]
  pub fn get_actual_present_time(&self) -> u64 {
    self.actualPresentTime
  }
  #[inline]
  pub fn get_earliest_present_time(&self) -> u64 {
    self.earliestPresentTime
  }
  #[inline]
  pub fn get_present_margin(&self) -> u64 {
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
  pub fn get_present_id(&self) -> u32 {
    self.presentID
  }
  #[inline]
  pub fn get_desired_present_time(&self) -> u64 {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub struct VkPresentTimesInfoGOOGLE<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PRESENT_TIMES_INFO_GOOGLE,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_swapchain_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_times_info_google() {
  assert_size!(
    types_raw::VkPresentTimesInfoGOOGLE,
    VkPresentTimesInfoGOOGLE
  );
}

// feature: VK_NVX_multiview_per_view_attributes
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
  pub fn new() -> VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    unsafe {
      VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
        sType: VkStructureType::E_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_per_view_position_all_components(mut self, value: VkBool32) -> Self {
    self.perViewPositionAllComponents = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_per_view_position_all_components(&self) -> VkBool32 {
    self.perViewPositionAllComponents
  }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
impl Default for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  fn default() -> VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX::new()
  }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
unsafe impl RawStruct for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  type Raw = types_raw::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
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
  pub fn get_x(&self) -> VkViewportCoordinateSwizzleNV {
    self.x
  }
  #[inline]
  pub fn get_y(&self) -> VkViewportCoordinateSwizzleNV {
    self.y
  }
  #[inline]
  pub fn get_z(&self) -> VkViewportCoordinateSwizzleNV {
    self.z
  }
  #[inline]
  pub fn get_w(&self) -> VkViewportCoordinateSwizzleNV {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineViewportSwizzleStateCreateFlagsNV {
    self.flags
  }
  #[inline]
  pub fn get_viewport_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_viewport_swizzle_state_create_info_nv() {
  assert_size!(
    types_raw::VkPipelineViewportSwizzleStateCreateInfoNV,
    VkPipelineViewportSwizzleStateCreateInfoNV
  );
}

// feature: VK_EXT_discard_rectangles
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_max_discard_rectangles(mut self, value: u32) -> Self {
    self.maxDiscardRectangles = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_max_discard_rectangles(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_discard_rectangle_properties_ext() {
  assert_size!(
    types_raw::VkPhysicalDeviceDiscardRectanglePropertiesEXT,
    VkPhysicalDeviceDiscardRectanglePropertiesEXT
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineDiscardRectangleStateCreateFlagsEXT {
    self.flags
  }
  #[inline]
  pub fn get_discard_rectangle_mode(&self) -> VkDiscardRectangleModeEXT {
    self.discardRectangleMode
  }
  #[inline]
  pub fn get_discard_rectangle_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_discard_rectangle_state_create_info_ext() {
  assert_size!(
    types_raw::VkPipelineDiscardRectangleStateCreateInfoEXT,
    VkPipelineDiscardRectangleStateCreateInfoEXT
  );
}

// feature: VK_EXT_conservative_rasterization
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
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
  pub fn set_primitive_underestimation(mut self, value: VkBool32) -> Self {
    self.primitiveUnderestimation = value;
    self
  }
  #[inline]
  pub fn set_conservative_point_and_line_rasterization(mut self, value: VkBool32) -> Self {
    self.conservativePointAndLineRasterization = value;
    self
  }
  #[inline]
  pub fn set_degenerate_triangles_rasterized(mut self, value: VkBool32) -> Self {
    self.degenerateTrianglesRasterized = value;
    self
  }
  #[inline]
  pub fn set_degenerate_lines_rasterized(mut self, value: VkBool32) -> Self {
    self.degenerateLinesRasterized = value;
    self
  }
  #[inline]
  pub fn set_fully_covered_fragment_shader_input_variable(mut self, value: VkBool32) -> Self {
    self.fullyCoveredFragmentShaderInputVariable = value;
    self
  }
  #[inline]
  pub fn set_conservative_rasterization_post_depth_coverage(mut self, value: VkBool32) -> Self {
    self.conservativeRasterizationPostDepthCoverage = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_primitive_overestimation_size(&self) -> f32 {
    self.primitiveOverestimationSize
  }
  #[inline]
  pub fn get_max_extra_primitive_overestimation_size(&self) -> f32 {
    self.maxExtraPrimitiveOverestimationSize
  }
  #[inline]
  pub fn get_extra_primitive_overestimation_size_granularity(&self) -> f32 {
    self.extraPrimitiveOverestimationSizeGranularity
  }
  #[inline]
  pub fn get_primitive_underestimation(&self) -> VkBool32 {
    self.primitiveUnderestimation
  }
  #[inline]
  pub fn get_conservative_point_and_line_rasterization(&self) -> VkBool32 {
    self.conservativePointAndLineRasterization
  }
  #[inline]
  pub fn get_degenerate_triangles_rasterized(&self) -> VkBool32 {
    self.degenerateTrianglesRasterized
  }
  #[inline]
  pub fn get_degenerate_lines_rasterized(&self) -> VkBool32 {
    self.degenerateLinesRasterized
  }
  #[inline]
  pub fn get_fully_covered_fragment_shader_input_variable(&self) -> VkBool32 {
    self.fullyCoveredFragmentShaderInputVariable
  }
  #[inline]
  pub fn get_conservative_rasterization_post_depth_coverage(&self) -> VkBool32 {
    self.conservativeRasterizationPostDepthCoverage
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_conservative_rasterization_properties_ext() {
  assert_size!(
    types_raw::VkPhysicalDeviceConservativeRasterizationPropertiesEXT,
    VkPhysicalDeviceConservativeRasterizationPropertiesEXT
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
  pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
  pub extraPrimitiveOverestimationSize: f32,
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl VkPipelineRasterizationConservativeStateCreateInfoEXT {
  #[inline]
  pub fn new() -> VkPipelineRasterizationConservativeStateCreateInfoEXT {
    unsafe {
      VkPipelineRasterizationConservativeStateCreateInfoEXT {
        sType: VkStructureType::E_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineRasterizationConservativeStateCreateFlagsEXT {
    self.flags
  }
  #[inline]
  pub fn get_conservative_rasterization_mode(&self) -> VkConservativeRasterizationModeEXT {
    self.conservativeRasterizationMode
  }
  #[inline]
  pub fn get_extra_primitive_overestimation_size(&self) -> f32 {
    self.extraPrimitiveOverestimationSize
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl Default for VkPipelineRasterizationConservativeStateCreateInfoEXT {
  fn default() -> VkPipelineRasterizationConservativeStateCreateInfoEXT {
    VkPipelineRasterizationConservativeStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
unsafe impl RawStruct for VkPipelineRasterizationConservativeStateCreateInfoEXT {
  type Raw = types_raw::VkPipelineRasterizationConservativeStateCreateInfoEXT;
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
  pub fn get_x(&self) -> f32 {
    self.x
  }
  #[inline]
  pub fn get_y(&self) -> f32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub struct VkHdrMetadataEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub displayPrimaryRed: VkXYColorEXT,
  pub displayPrimaryGreen: VkXYColorEXT,
  pub displayPrimaryBlue: VkXYColorEXT,
  pub whitePoint: VkXYColorEXT,
  pub maxLuminance: f32,
  pub minLuminance: f32,
  pub maxContentLightLevel: f32,
  pub maxFrameAverageLightLevel: f32,
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl VkHdrMetadataEXT {
  #[inline]
  pub fn new() -> VkHdrMetadataEXT {
    unsafe {
      VkHdrMetadataEXT {
        sType: VkStructureType::E_HDR_METADATA_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_display_primary_red(&self) -> VkXYColorEXT {
    self.displayPrimaryRed
  }
  #[inline]
  pub fn get_display_primary_green(&self) -> VkXYColorEXT {
    self.displayPrimaryGreen
  }
  #[inline]
  pub fn get_display_primary_blue(&self) -> VkXYColorEXT {
    self.displayPrimaryBlue
  }
  #[inline]
  pub fn get_white_point(&self) -> VkXYColorEXT {
    self.whitePoint
  }
  #[inline]
  pub fn get_max_luminance(&self) -> f32 {
    self.maxLuminance
  }
  #[inline]
  pub fn get_min_luminance(&self) -> f32 {
    self.minLuminance
  }
  #[inline]
  pub fn get_max_content_light_level(&self) -> f32 {
    self.maxContentLightLevel
  }
  #[inline]
  pub fn get_max_frame_average_light_level(&self) -> f32 {
    self.maxFrameAverageLightLevel
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl Default for VkHdrMetadataEXT {
  fn default() -> VkHdrMetadataEXT {
    VkHdrMetadataEXT::new()
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
unsafe impl RawStruct for VkHdrMetadataEXT {
  type Raw = types_raw::VkHdrMetadataEXT;
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_hdr_metadata_ext() {
  assert_size!(types_raw::VkHdrMetadataEXT, VkHdrMetadataEXT);
}

// feature: VK_KHR_get_surface_capabilities2
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub surface: VkSurfaceKHR,
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl VkPhysicalDeviceSurfaceInfo2KHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceSurfaceInfo2KHR {
    unsafe {
      VkPhysicalDeviceSurfaceInfo2KHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_surface(&self) -> VkSurfaceKHR {
    self.surface
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl Default for VkPhysicalDeviceSurfaceInfo2KHR {
  fn default() -> VkPhysicalDeviceSurfaceInfo2KHR {
    VkPhysicalDeviceSurfaceInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
unsafe impl RawStruct for VkPhysicalDeviceSurfaceInfo2KHR {
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
  pub fn new() -> VkSurfaceCapabilities2KHR {
    unsafe {
      VkSurfaceCapabilities2KHR {
        sType: VkStructureType::E_SURFACE_CAPABILITIES_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_surface_capabilities(mut self, value: VkSurfaceCapabilitiesKHR) -> Self {
    self.surfaceCapabilities = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_surface_capabilities(&self) -> VkSurfaceCapabilitiesKHR {
    self.surfaceCapabilities
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl Default for VkSurfaceCapabilities2KHR {
  fn default() -> VkSurfaceCapabilities2KHR {
    VkSurfaceCapabilities2KHR::new()
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
  pub fn new() -> VkSurfaceFormat2KHR {
    unsafe {
      VkSurfaceFormat2KHR {
        sType: VkStructureType::E_SURFACE_FORMAT_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_surface_format(mut self, value: VkSurfaceFormatKHR) -> Self {
    self.surfaceFormat = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_surface_format(&self) -> VkSurfaceFormatKHR {
    self.surfaceFormat
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl Default for VkSurfaceFormat2KHR {
  fn default() -> VkSurfaceFormat2KHR {
    VkSurfaceFormat2KHR::new()
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
  pub fn new() -> VkSharedPresentSurfaceCapabilitiesKHR {
    unsafe {
      VkSharedPresentSurfaceCapabilitiesKHR {
        sType: VkStructureType::E_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_shared_present_supported_usage_flags(mut self, value: VkImageUsageFlags) -> Self {
    self.sharedPresentSupportedUsageFlags = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_shared_present_supported_usage_flags(&self) -> VkImageUsageFlags {
    self.sharedPresentSupportedUsageFlags
  }
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
impl Default for VkSharedPresentSurfaceCapabilitiesKHR {
  fn default() -> VkSharedPresentSurfaceCapabilitiesKHR {
    VkSharedPresentSurfaceCapabilitiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
unsafe impl RawStruct for VkSharedPresentSurfaceCapabilitiesKHR {
  type Raw = types_raw::VkSharedPresentSurfaceCapabilitiesKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub struct VkPhysicalDeviceExternalFenceInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl VkPhysicalDeviceExternalFenceInfoKHR {
  #[inline]
  pub fn new() -> VkPhysicalDeviceExternalFenceInfoKHR {
    unsafe {
      VkPhysicalDeviceExternalFenceInfoKHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl Default for VkPhysicalDeviceExternalFenceInfoKHR {
  fn default() -> VkPhysicalDeviceExternalFenceInfoKHR {
    VkPhysicalDeviceExternalFenceInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
unsafe impl RawStruct for VkPhysicalDeviceExternalFenceInfoKHR {
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
  pub fn new() -> VkExternalFencePropertiesKHR {
    unsafe {
      VkExternalFencePropertiesKHR {
        sType: VkStructureType::E_EXTERNAL_FENCE_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_export_from_imported_handle_types(mut self, value: VkExternalFenceHandleTypeFlagsKHR) -> Self {
    self.exportFromImportedHandleTypes = value;
    self
  }
  #[inline]
  pub fn set_compatible_handle_types(mut self, value: VkExternalFenceHandleTypeFlagsKHR) -> Self {
    self.compatibleHandleTypes = value;
    self
  }
  #[inline]
  pub fn set_external_fence_features(mut self, value: VkExternalFenceFeatureFlagsKHR) -> Self {
    self.externalFenceFeatures = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_export_from_imported_handle_types(&self) -> VkExternalFenceHandleTypeFlagsKHR {
    self.exportFromImportedHandleTypes
  }
  #[inline]
  pub fn get_compatible_handle_types(&self) -> VkExternalFenceHandleTypeFlagsKHR {
    self.compatibleHandleTypes
  }
  #[inline]
  pub fn get_external_fence_features(&self) -> VkExternalFenceFeatureFlagsKHR {
    self.externalFenceFeatures
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl Default for VkExternalFencePropertiesKHR {
  fn default() -> VkExternalFencePropertiesKHR {
    VkExternalFencePropertiesKHR::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence")]
pub struct VkExportFenceCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleTypes: VkExternalFenceHandleTypeFlagsKHR,
}
#[cfg(feature = "VK_KHR_external_fence")]
impl VkExportFenceCreateInfoKHR {
  #[inline]
  pub fn new() -> VkExportFenceCreateInfoKHR {
    unsafe {
      VkExportFenceCreateInfoKHR {
        sType: VkStructureType::E_EXPORT_FENCE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_types(&self) -> VkExternalFenceHandleTypeFlagsKHR {
    self.handleTypes
  }
}
#[cfg(feature = "VK_KHR_external_fence")]
impl Default for VkExportFenceCreateInfoKHR {
  fn default() -> VkExportFenceCreateInfoKHR {
    VkExportFenceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence")]
unsafe impl RawStruct for VkExportFenceCreateInfoKHR {
  type Raw = types_raw::VkExportFenceCreateInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkImportFenceWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub fence: VkFence,
  pub flags: VkFenceImportFlagsKHR,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  pub handle: wsi::win32::HANDLE,
  pub name: wsi::win32::LPCWSTR,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkImportFenceWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkImportFenceWin32HandleInfoKHR {
    unsafe {
      VkImportFenceWin32HandleInfoKHR {
        sType: VkStructureType::E_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_fence(&self) -> VkFence {
    self.fence
  }
  #[inline]
  pub fn get_flags(&self) -> VkFenceImportFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn get_handle(&self) -> wsi::win32::HANDLE {
    self.handle
  }
  #[inline]
  pub fn get_name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkImportFenceWin32HandleInfoKHR {
  fn default() -> VkImportFenceWin32HandleInfoKHR {
    VkImportFenceWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkImportFenceWin32HandleInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkExportFenceWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub pAttributes: *const wsi::win32::SECURITY_ATTRIBUTES,
  pub dwAccess: wsi::win32::DWORD,
  pub name: wsi::win32::LPCWSTR,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkExportFenceWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkExportFenceWin32HandleInfoKHR {
    unsafe {
      VkExportFenceWin32HandleInfoKHR {
        sType: VkStructureType::E_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
        ..::std::mem::zeroed()
      }
    }
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_attributes(&self) -> *const wsi::win32::SECURITY_ATTRIBUTES {
    self.pAttributes
  }
  #[inline]
  pub fn get_dw_access(&self) -> wsi::win32::DWORD {
    self.dwAccess
  }
  #[inline]
  pub fn get_name(&self) -> wsi::win32::LPCWSTR {
    self.name
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkExportFenceWin32HandleInfoKHR {
  fn default() -> VkExportFenceWin32HandleInfoKHR {
    VkExportFenceWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkExportFenceWin32HandleInfoKHR {
  type Raw = types_raw::VkExportFenceWin32HandleInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub struct VkFenceGetWin32HandleInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl VkFenceGetWin32HandleInfoKHR {
  #[inline]
  pub fn new() -> VkFenceGetWin32HandleInfoKHR {
    unsafe {
      VkFenceGetWin32HandleInfoKHR {
        sType: VkStructureType::E_FENCE_GET_WIN32_HANDLE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_fence(&self) -> VkFence {
    self.fence
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
impl Default for VkFenceGetWin32HandleInfoKHR {
  fn default() -> VkFenceGetWin32HandleInfoKHR {
    VkFenceGetWin32HandleInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
unsafe impl RawStruct for VkFenceGetWin32HandleInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub struct VkImportFenceFdInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub fence: VkFence,
  pub flags: VkFenceImportFlagsKHR,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
  pub fd: c_int,
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl VkImportFenceFdInfoKHR {
  #[inline]
  pub fn new() -> VkImportFenceFdInfoKHR {
    unsafe {
      VkImportFenceFdInfoKHR {
        sType: VkStructureType::E_IMPORT_FENCE_FD_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_fence(&self) -> VkFence {
    self.fence
  }
  #[inline]
  pub fn get_flags(&self) -> VkFenceImportFlagsKHR {
    self.flags
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn get_fd(&self) -> c_int {
    self.fd
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl Default for VkImportFenceFdInfoKHR {
  fn default() -> VkImportFenceFdInfoKHR {
    VkImportFenceFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
unsafe impl RawStruct for VkImportFenceFdInfoKHR {
  type Raw = types_raw::VkImportFenceFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_fence_fd_info_khr() {
  assert_size!(types_raw::VkImportFenceFdInfoKHR, VkImportFenceFdInfoKHR);
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub struct VkFenceGetFdInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBitsKHR,
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl VkFenceGetFdInfoKHR {
  #[inline]
  pub fn new() -> VkFenceGetFdInfoKHR {
    unsafe {
      VkFenceGetFdInfoKHR {
        sType: VkStructureType::E_FENCE_GET_FD_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_fence(&self) -> VkFence {
    self.fence
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalFenceHandleTypeFlagBitsKHR {
    self.handleType
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl Default for VkFenceGetFdInfoKHR {
  fn default() -> VkFenceGetFdInfoKHR {
    VkFenceGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
unsafe impl RawStruct for VkFenceGetFdInfoKHR {
  type Raw = types_raw::VkFenceGetFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_get_fd_info_khr() {
  assert_size!(types_raw::VkFenceGetFdInfoKHR, VkFenceGetFdInfoKHR);
}

// feature: VK_KHR_maintenance2
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
  pub fn new() -> VkPhysicalDevicePointClippingPropertiesKHR {
    unsafe {
      VkPhysicalDevicePointClippingPropertiesKHR {
        sType: VkStructureType::E_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_point_clipping_behavior(mut self, value: VkPointClippingBehaviorKHR) -> Self {
    self.pointClippingBehavior = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_point_clipping_behavior(&self) -> VkPointClippingBehaviorKHR {
    self.pointClippingBehavior
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl Default for VkPhysicalDevicePointClippingPropertiesKHR {
  fn default() -> VkPhysicalDevicePointClippingPropertiesKHR {
    VkPhysicalDevicePointClippingPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl RawStruct for VkPhysicalDevicePointClippingPropertiesKHR {
  type Raw = types_raw::VkPhysicalDevicePointClippingPropertiesKHR;
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
  pub fn get_subpass(&self) -> u32 {
    self.subpass
  }
  #[inline]
  pub fn get_input_attachment_index(&self) -> u32 {
    self.inputAttachmentIndex
  }
  #[inline]
  pub fn get_aspect_mask(&self) -> VkImageAspectFlags {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_aspect_reference_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_input_attachment_aspect_create_info_khr() {
  assert_size!(
    types_raw::VkRenderPassInputAttachmentAspectCreateInfoKHR,
    VkRenderPassInputAttachmentAspectCreateInfoKHR
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkImageViewUsageCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub usage: VkImageUsageFlags,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl VkImageViewUsageCreateInfoKHR {
  #[inline]
  pub fn new() -> VkImageViewUsageCreateInfoKHR {
    unsafe {
      VkImageViewUsageCreateInfoKHR {
        sType: VkStructureType::E_IMAGE_VIEW_USAGE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_usage(&self) -> VkImageUsageFlags {
    self.usage
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl Default for VkImageViewUsageCreateInfoKHR {
  fn default() -> VkImageViewUsageCreateInfoKHR {
    VkImageViewUsageCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl RawStruct for VkImageViewUsageCreateInfoKHR {
  type Raw = types_raw::VkImageViewUsageCreateInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_maintenance2")]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub domainOrigin: VkTessellationDomainOriginKHR,
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl VkPipelineTessellationDomainOriginStateCreateInfoKHR {
  #[inline]
  pub fn new() -> VkPipelineTessellationDomainOriginStateCreateInfoKHR {
    unsafe {
      VkPipelineTessellationDomainOriginStateCreateInfoKHR {
        sType: VkStructureType::E_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_domain_origin(&self) -> VkTessellationDomainOriginKHR {
    self.domainOrigin
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl Default for VkPipelineTessellationDomainOriginStateCreateInfoKHR {
  fn default() -> VkPipelineTessellationDomainOriginStateCreateInfoKHR {
    VkPipelineTessellationDomainOriginStateCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
unsafe impl RawStruct for VkPipelineTessellationDomainOriginStateCreateInfoKHR {
  type Raw = types_raw::VkPipelineTessellationDomainOriginStateCreateInfoKHR;
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_variable_pointers_storage_buffer(mut self, value: VkBool32) -> Self {
    self.variablePointersStorageBuffer = value;
    self
  }
  #[inline]
  pub fn set_variable_pointers(mut self, value: VkBool32) -> Self {
    self.variablePointers = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_variable_pointers_storage_buffer(&self) -> VkBool32 {
    self.variablePointersStorageBuffer
  }
  #[inline]
  pub fn get_variable_pointers(&self) -> VkBool32 {
    self.variablePointers
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_variable_pointer_features_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceVariablePointerFeaturesKHR,
    VkPhysicalDeviceVariablePointerFeaturesKHR
  );
}

// feature: VK_MVK_ios_surface
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub struct VkIOSSurfaceCreateInfoMVK {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkIOSSurfaceCreateFlagsMVK,
  pView: *const c_void,
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
impl VkIOSSurfaceCreateInfoMVK {
  #[inline]
  pub fn new() -> VkIOSSurfaceCreateInfoMVK {
    unsafe {
      VkIOSSurfaceCreateInfoMVK {
        sType: VkStructureType::E_IOS_SURFACE_CREATE_INFO_MVK,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkIOSSurfaceCreateFlagsMVK {
    self.flags
  }
  #[inline]
  pub fn get_view(&self) -> *const c_void {
    self.pView
  }
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
impl Default for VkIOSSurfaceCreateInfoMVK {
  fn default() -> VkIOSSurfaceCreateInfoMVK {
    VkIOSSurfaceCreateInfoMVK::new()
  }
}
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
unsafe impl RawStruct for VkIOSSurfaceCreateInfoMVK {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub struct VkMacOSSurfaceCreateInfoMVK {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkMacOSSurfaceCreateFlagsMVK,
  pView: *const c_void,
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
impl VkMacOSSurfaceCreateInfoMVK {
  #[inline]
  pub fn new() -> VkMacOSSurfaceCreateInfoMVK {
    unsafe {
      VkMacOSSurfaceCreateInfoMVK {
        sType: VkStructureType::E_MACOS_SURFACE_CREATE_INFO_MVK,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkMacOSSurfaceCreateFlagsMVK {
    self.flags
  }
  #[inline]
  pub fn get_view(&self) -> *const c_void {
    self.pView
  }
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
impl Default for VkMacOSSurfaceCreateInfoMVK {
  fn default() -> VkMacOSSurfaceCreateInfoMVK {
    VkMacOSSurfaceCreateInfoMVK::new()
  }
}
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
unsafe impl RawStruct for VkMacOSSurfaceCreateInfoMVK {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkBufferMemoryRequirementsInfo2KHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub buffer: VkBuffer,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl VkBufferMemoryRequirementsInfo2KHR {
  #[inline]
  pub fn new() -> VkBufferMemoryRequirementsInfo2KHR {
    unsafe {
      VkBufferMemoryRequirementsInfo2KHR {
        sType: VkStructureType::E_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_buffer(&self) -> VkBuffer {
    self.buffer
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkBufferMemoryRequirementsInfo2KHR {
  fn default() -> VkBufferMemoryRequirementsInfo2KHR {
    VkBufferMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl RawStruct for VkBufferMemoryRequirementsInfo2KHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkImageMemoryRequirementsInfo2KHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub image: VkImage,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl VkImageMemoryRequirementsInfo2KHR {
  #[inline]
  pub fn new() -> VkImageMemoryRequirementsInfo2KHR {
    unsafe {
      VkImageMemoryRequirementsInfo2KHR {
        sType: VkStructureType::E_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_image(&self) -> VkImage {
    self.image
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkImageMemoryRequirementsInfo2KHR {
  fn default() -> VkImageMemoryRequirementsInfo2KHR {
    VkImageMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl RawStruct for VkImageMemoryRequirementsInfo2KHR {
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
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub struct VkImageSparseMemoryRequirementsInfo2KHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub image: VkImage,
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl VkImageSparseMemoryRequirementsInfo2KHR {
  #[inline]
  pub fn new() -> VkImageSparseMemoryRequirementsInfo2KHR {
    unsafe {
      VkImageSparseMemoryRequirementsInfo2KHR {
        sType: VkStructureType::E_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_image(&self) -> VkImage {
    self.image
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkImageSparseMemoryRequirementsInfo2KHR {
  fn default() -> VkImageSparseMemoryRequirementsInfo2KHR {
    VkImageSparseMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
unsafe impl RawStruct for VkImageSparseMemoryRequirementsInfo2KHR {
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
  pub fn new() -> VkMemoryRequirements2KHR {
    unsafe {
      VkMemoryRequirements2KHR {
        sType: VkStructureType::E_MEMORY_REQUIREMENTS_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory_requirements(mut self, value: VkMemoryRequirements) -> Self {
    self.memoryRequirements = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory_requirements(&self) -> VkMemoryRequirements {
    self.memoryRequirements
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkMemoryRequirements2KHR {
  fn default() -> VkMemoryRequirements2KHR {
    VkMemoryRequirements2KHR::new()
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
  pub fn new() -> VkSparseImageMemoryRequirements2KHR {
    unsafe {
      VkSparseImageMemoryRequirements2KHR {
        sType: VkStructureType::E_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory_requirements(mut self, value: VkSparseImageMemoryRequirements) -> Self {
    self.memoryRequirements = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory_requirements(&self) -> VkSparseImageMemoryRequirements {
    self.memoryRequirements
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkSparseImageMemoryRequirements2KHR {
  fn default() -> VkSparseImageMemoryRequirements2KHR {
    VkSparseImageMemoryRequirements2KHR::new()
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
  pub fn new() -> VkMemoryDedicatedRequirementsKHR {
    unsafe {
      VkMemoryDedicatedRequirementsKHR {
        sType: VkStructureType::E_MEMORY_DEDICATED_REQUIREMENTS_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_prefers_dedicated_allocation(mut self, value: VkBool32) -> Self {
    self.prefersDedicatedAllocation = value;
    self
  }
  #[inline]
  pub fn set_requires_dedicated_allocation(mut self, value: VkBool32) -> Self {
    self.requiresDedicatedAllocation = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_prefers_dedicated_allocation(&self) -> VkBool32 {
    self.prefersDedicatedAllocation
  }
  #[inline]
  pub fn get_requires_dedicated_allocation(&self) -> VkBool32 {
    self.requiresDedicatedAllocation
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl Default for VkMemoryDedicatedRequirementsKHR {
  fn default() -> VkMemoryDedicatedRequirementsKHR {
    VkMemoryDedicatedRequirementsKHR::new()
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl RawStruct for VkMemoryDedicatedRequirementsKHR {
  type Raw = types_raw::VkMemoryDedicatedRequirementsKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub struct VkMemoryDedicatedAllocateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub image: Option<VkImage>,
  pub buffer: Option<VkBuffer>,
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl VkMemoryDedicatedAllocateInfoKHR {
  #[inline]
  pub fn new() -> VkMemoryDedicatedAllocateInfoKHR {
    unsafe {
      VkMemoryDedicatedAllocateInfoKHR {
        sType: VkStructureType::E_MEMORY_DEDICATED_ALLOCATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_image(&self) -> Option<VkImage> {
    self.image
  }
  #[inline]
  pub fn get_buffer(&self) -> Option<VkBuffer> {
    self.buffer
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl Default for VkMemoryDedicatedAllocateInfoKHR {
  fn default() -> VkMemoryDedicatedAllocateInfoKHR {
    VkMemoryDedicatedAllocateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
unsafe impl RawStruct for VkMemoryDedicatedAllocateInfoKHR {
  type Raw = types_raw::VkMemoryDedicatedAllocateInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub struct VkSamplerReductionModeCreateInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub reductionMode: VkSamplerReductionModeEXT,
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl VkSamplerReductionModeCreateInfoEXT {
  #[inline]
  pub fn new() -> VkSamplerReductionModeCreateInfoEXT {
    unsafe {
      VkSamplerReductionModeCreateInfoEXT {
        sType: VkStructureType::E_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_reduction_mode(&self) -> VkSamplerReductionModeEXT {
    self.reductionMode
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl Default for VkSamplerReductionModeCreateInfoEXT {
  fn default() -> VkSamplerReductionModeCreateInfoEXT {
    VkSamplerReductionModeCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
unsafe impl RawStruct for VkSamplerReductionModeCreateInfoEXT {
  type Raw = types_raw::VkSamplerReductionModeCreateInfoEXT;
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
  pub fn new() -> VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    unsafe {
      VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
        sType: VkStructureType::E_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_filter_minmax_single_component_formats(mut self, value: VkBool32) -> Self {
    self.filterMinmaxSingleComponentFormats = value;
    self
  }
  #[inline]
  pub fn set_filter_minmax_image_component_mapping(mut self, value: VkBool32) -> Self {
    self.filterMinmaxImageComponentMapping = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_filter_minmax_single_component_formats(&self) -> VkBool32 {
    self.filterMinmaxSingleComponentFormats
  }
  #[inline]
  pub fn get_filter_minmax_image_component_mapping(&self) -> VkBool32 {
    self.filterMinmaxImageComponentMapping
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl Default for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  fn default() -> VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
unsafe impl RawStruct for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT;
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
  pub fn get_x(&self) -> f32 {
    self.x
  }
  #[inline]
  pub fn get_y(&self) -> f32 {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkSampleLocationsInfoEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_SAMPLE_LOCATIONS_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_sample_locations_per_pixel(&self) -> VkSampleCountFlagBits {
    self.sampleLocationsPerPixel
  }
  #[inline]
  pub fn get_sample_location_grid_size(&self) -> VkExtent2D {
    self.sampleLocationGridSize
  }
  #[inline]
  pub fn get_sample_locations_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_sample_locations_info_ext() {
  assert_size!(
    types_raw::VkSampleLocationsInfoEXT,
    VkSampleLocationsInfoEXT
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_attachment_index(&self) -> u32 {
    self.attachmentIndex
  }
  #[inline]
  pub fn get_sample_locations_info(&self) -> VkSampleLocationsInfoEXT<'a> {
    self.sampleLocationsInfo
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
#[repr(C)]
#[derive(Copy, Clone)]
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
  pub fn get_subpass_index(&self) -> u32 {
    self.subpassIndex
  }
  #[inline]
  pub fn get_sample_locations_info(&self) -> VkSampleLocationsInfoEXT<'a> {
    self.sampleLocationsInfo
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkRenderPassSampleLocationsBeginInfoEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_attachment_initial_sample_locations_count(&self) -> u32 {
    self.attachmentInitialSampleLocationsCount
  }
  #[inline]
  pub fn get_post_subpass_sample_locations_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_sample_locations_begin_info_ext() {
  assert_size!(
    types_raw::VkRenderPassSampleLocationsBeginInfoEXT,
    VkRenderPassSampleLocationsBeginInfoEXT
  );
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_sample_locations")]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
  pub sampleLocationsEnable: VkBool32,
  pub sampleLocationsInfo: VkSampleLocationsInfoEXT<'a>,
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
  #[inline]
  pub fn new() -> VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
    unsafe {
      VkPipelineSampleLocationsStateCreateInfoEXT {
        sType: VkStructureType::E_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_sample_locations_enable(mut self, value: VkBool32) -> Self {
    self.sampleLocationsEnable = value;
    self
  }
  #[inline]
  pub fn set_sample_locations_info(mut self, value: VkSampleLocationsInfoEXT<'a>) -> Self {
    self.sampleLocationsInfo = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_sample_locations_enable(&self) -> VkBool32 {
    self.sampleLocationsEnable
  }
  #[inline]
  pub fn get_sample_locations_info(&self) -> VkSampleLocationsInfoEXT<'a> {
    self.sampleLocationsInfo
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_sample_locations_state_create_info_ext() {
  assert_size!(
    types_raw::VkPipelineSampleLocationsStateCreateInfoEXT,
    VkPipelineSampleLocationsStateCreateInfoEXT
  );
}
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
  pub fn new() -> VkPhysicalDeviceSampleLocationsPropertiesEXT {
    unsafe {
      VkPhysicalDeviceSampleLocationsPropertiesEXT {
        sType: VkStructureType::E_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_sample_location_sample_counts(mut self, value: VkSampleCountFlags) -> Self {
    self.sampleLocationSampleCounts = value;
    self
  }
  #[inline]
  pub fn set_max_sample_location_grid_size(mut self, value: VkExtent2D) -> Self {
    self.maxSampleLocationGridSize = value;
    self
  }
  #[inline]
  pub fn set_sample_location_coordinate_range(mut self, value: [f32; 2]) -> Self {
    self.sampleLocationCoordinateRange = value;
    self
  }
  #[inline]
  pub fn set_sample_location_sub_pixel_bits(mut self, value: u32) -> Self {
    self.sampleLocationSubPixelBits = value;
    self
  }
  #[inline]
  pub fn set_variable_sample_locations(mut self, value: VkBool32) -> Self {
    self.variableSampleLocations = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_sample_location_sample_counts(&self) -> VkSampleCountFlags {
    self.sampleLocationSampleCounts
  }
  #[inline]
  pub fn get_max_sample_location_grid_size(&self) -> VkExtent2D {
    self.maxSampleLocationGridSize
  }
  #[inline]
  pub fn get_sample_location_coordinate_range(&self) -> [f32; 2] {
    self.sampleLocationCoordinateRange
  }
  #[inline]
  pub fn get_sample_location_sub_pixel_bits(&self) -> u32 {
    self.sampleLocationSubPixelBits
  }
  #[inline]
  pub fn get_variable_sample_locations(&self) -> VkBool32 {
    self.variableSampleLocations
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl Default for VkPhysicalDeviceSampleLocationsPropertiesEXT {
  fn default() -> VkPhysicalDeviceSampleLocationsPropertiesEXT {
    VkPhysicalDeviceSampleLocationsPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
unsafe impl RawStruct for VkPhysicalDeviceSampleLocationsPropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceSampleLocationsPropertiesEXT;
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
  pub fn new() -> VkMultisamplePropertiesEXT {
    unsafe {
      VkMultisamplePropertiesEXT {
        sType: VkStructureType::E_MULTISAMPLE_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_max_sample_location_grid_size(mut self, value: VkExtent2D) -> Self {
    self.maxSampleLocationGridSize = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_max_sample_location_grid_size(&self) -> VkExtent2D {
    self.maxSampleLocationGridSize
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl Default for VkMultisamplePropertiesEXT {
  fn default() -> VkMultisamplePropertiesEXT {
    VkMultisamplePropertiesEXT::new()
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_image_format_list")]
pub struct VkImageFormatListCreateInfoKHR<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_IMAGE_FORMAT_LIST_CREATE_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_view_format_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_format_list_create_info_khr() {
  assert_size!(
    types_raw::VkImageFormatListCreateInfoKHR,
    VkImageFormatListCreateInfoKHR
  );
}

// feature: VK_EXT_blend_operation_advanced
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_advanced_blend_coherent_operations(mut self, value: VkBool32) -> Self {
    self.advancedBlendCoherentOperations = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_advanced_blend_coherent_operations(&self) -> VkBool32 {
    self.advancedBlendCoherentOperations
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_blend_operation_advanced_features_ext() {
  assert_size!(
    types_raw::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT,
    VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
  );
}
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
  pub fn new() -> VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    unsafe {
      VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
        sType: VkStructureType::E_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_advanced_blend_max_color_attachments(mut self, value: u32) -> Self {
    self.advancedBlendMaxColorAttachments = value;
    self
  }
  #[inline]
  pub fn set_advanced_blend_independent_blend(mut self, value: VkBool32) -> Self {
    self.advancedBlendIndependentBlend = value;
    self
  }
  #[inline]
  pub fn set_advanced_blend_non_premultiplied_src_color(mut self, value: VkBool32) -> Self {
    self.advancedBlendNonPremultipliedSrcColor = value;
    self
  }
  #[inline]
  pub fn set_advanced_blend_non_premultiplied_dst_color(mut self, value: VkBool32) -> Self {
    self.advancedBlendNonPremultipliedDstColor = value;
    self
  }
  #[inline]
  pub fn set_advanced_blend_correlated_overlap(mut self, value: VkBool32) -> Self {
    self.advancedBlendCorrelatedOverlap = value;
    self
  }
  #[inline]
  pub fn set_advanced_blend_all_operations(mut self, value: VkBool32) -> Self {
    self.advancedBlendAllOperations = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_advanced_blend_max_color_attachments(&self) -> u32 {
    self.advancedBlendMaxColorAttachments
  }
  #[inline]
  pub fn get_advanced_blend_independent_blend(&self) -> VkBool32 {
    self.advancedBlendIndependentBlend
  }
  #[inline]
  pub fn get_advanced_blend_non_premultiplied_src_color(&self) -> VkBool32 {
    self.advancedBlendNonPremultipliedSrcColor
  }
  #[inline]
  pub fn get_advanced_blend_non_premultiplied_dst_color(&self) -> VkBool32 {
    self.advancedBlendNonPremultipliedDstColor
  }
  #[inline]
  pub fn get_advanced_blend_correlated_overlap(&self) -> VkBool32 {
    self.advancedBlendCorrelatedOverlap
  }
  #[inline]
  pub fn get_advanced_blend_all_operations(&self) -> VkBool32 {
    self.advancedBlendAllOperations
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl Default for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  fn default() -> VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl RawStruct for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  type Raw = types_raw::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub srcPremultiplied: VkBool32,
  pub dstPremultiplied: VkBool32,
  pub blendOverlap: VkBlendOverlapEXT,
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  #[inline]
  pub fn new() -> VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    unsafe {
      VkPipelineColorBlendAdvancedStateCreateInfoEXT {
        sType: VkStructureType::E_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_src_premultiplied(mut self, value: VkBool32) -> Self {
    self.srcPremultiplied = value;
    self
  }
  #[inline]
  pub fn set_dst_premultiplied(mut self, value: VkBool32) -> Self {
    self.dstPremultiplied = value;
    self
  }
  #[inline]
  pub fn set_blend_overlap(mut self, value: VkBlendOverlapEXT) -> Self {
    self.blendOverlap = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_src_premultiplied(&self) -> VkBool32 {
    self.srcPremultiplied
  }
  #[inline]
  pub fn get_dst_premultiplied(&self) -> VkBool32 {
    self.dstPremultiplied
  }
  #[inline]
  pub fn get_blend_overlap(&self) -> VkBlendOverlapEXT {
    self.blendOverlap
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl Default for VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  fn default() -> VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    VkPipelineColorBlendAdvancedStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
unsafe impl RawStruct for VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  type Raw = types_raw::VkPipelineColorBlendAdvancedStateCreateInfoEXT;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
  sType: VkStructureType,
  pNext: *const c_void,
  pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
  pub coverageToColorEnable: VkBool32,
  pub coverageToColorLocation: u32,
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
impl VkPipelineCoverageToColorStateCreateInfoNV {
  #[inline]
  pub fn new() -> VkPipelineCoverageToColorStateCreateInfoNV {
    unsafe {
      VkPipelineCoverageToColorStateCreateInfoNV {
        sType: VkStructureType::E_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
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
  pub fn set_coverage_to_color_enable(mut self, value: VkBool32) -> Self {
    self.coverageToColorEnable = value;
    self
  }
  #[inline]
  pub fn set_coverage_to_color_location(mut self, value: u32) -> Self {
    self.coverageToColorLocation = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineCoverageToColorStateCreateFlagsNV {
    self.flags
  }
  #[inline]
  pub fn get_coverage_to_color_enable(&self) -> VkBool32 {
    self.coverageToColorEnable
  }
  #[inline]
  pub fn get_coverage_to_color_location(&self) -> u32 {
    self.coverageToColorLocation
  }
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
impl Default for VkPipelineCoverageToColorStateCreateInfoNV {
  fn default() -> VkPipelineCoverageToColorStateCreateInfoNV {
    VkPipelineCoverageToColorStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
unsafe impl RawStruct for VkPipelineCoverageToColorStateCreateInfoNV {
  type Raw = types_raw::VkPipelineCoverageToColorStateCreateInfoNV;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub struct VkPipelineCoverageModulationStateCreateInfoNV<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
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
  pub fn set_coverage_modulation_table_enable(mut self, value: VkBool32) -> Self {
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkPipelineCoverageModulationStateCreateFlagsNV {
    self.flags
  }
  #[inline]
  pub fn get_coverage_modulation_mode(&self) -> VkCoverageModulationModeNV {
    self.coverageModulationMode
  }
  #[inline]
  pub fn get_coverage_modulation_table_enable(&self) -> VkBool32 {
    self.coverageModulationTableEnable
  }
  #[inline]
  pub fn get_coverage_modulation_table_count(&self) -> u32 {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_pipeline_coverage_modulation_state_create_info_nv() {
  assert_size!(
    types_raw::VkPipelineCoverageModulationStateCreateInfoNV,
    VkPipelineCoverageModulationStateCreateInfoNV
  );
}

// feature: VK_KHR_bind_memory2
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_bind_memory2")]
pub struct VkBindBufferMemoryInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub buffer: VkBuffer,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl VkBindBufferMemoryInfoKHR {
  #[inline]
  pub fn new() -> VkBindBufferMemoryInfoKHR {
    unsafe {
      VkBindBufferMemoryInfoKHR {
        sType: VkStructureType::E_BIND_BUFFER_MEMORY_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_buffer(&self) -> VkBuffer {
    self.buffer
  }
  #[inline]
  pub fn get_memory(&self) -> VkDeviceMemory {
    self.memory
  }
  #[inline]
  pub fn get_memory_offset(&self) -> VkDeviceSize {
    self.memoryOffset
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl Default for VkBindBufferMemoryInfoKHR {
  fn default() -> VkBindBufferMemoryInfoKHR {
    VkBindBufferMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
unsafe impl RawStruct for VkBindBufferMemoryInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_bind_memory2")]
pub struct VkBindImageMemoryInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub image: VkImage,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl VkBindImageMemoryInfoKHR {
  #[inline]
  pub fn new() -> VkBindImageMemoryInfoKHR {
    unsafe {
      VkBindImageMemoryInfoKHR {
        sType: VkStructureType::E_BIND_IMAGE_MEMORY_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_image(&self) -> VkImage {
    self.image
  }
  #[inline]
  pub fn get_memory(&self) -> VkDeviceMemory {
    self.memory
  }
  #[inline]
  pub fn get_memory_offset(&self) -> VkDeviceSize {
    self.memoryOffset
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl Default for VkBindImageMemoryInfoKHR {
  fn default() -> VkBindImageMemoryInfoKHR {
    VkBindImageMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
unsafe impl RawStruct for VkBindImageMemoryInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionCreateInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
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
impl VkSamplerYcbcrConversionCreateInfoKHR {
  #[inline]
  pub fn new() -> VkSamplerYcbcrConversionCreateInfoKHR {
    unsafe {
      VkSamplerYcbcrConversionCreateInfoKHR {
        sType: VkStructureType::E_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR,
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
  pub fn set_force_explicit_reconstruction(mut self, value: VkBool32) -> Self {
    self.forceExplicitReconstruction = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_format(&self) -> VkFormat {
    self.format
  }
  #[inline]
  pub fn get_ycbcr_model(&self) -> VkSamplerYcbcrModelConversionKHR {
    self.ycbcrModel
  }
  #[inline]
  pub fn get_ycbcr_range(&self) -> VkSamplerYcbcrRangeKHR {
    self.ycbcrRange
  }
  #[inline]
  pub fn get_components(&self) -> VkComponentMapping {
    self.components
  }
  #[inline]
  pub fn get_x_chroma_offset(&self) -> VkChromaLocationKHR {
    self.xChromaOffset
  }
  #[inline]
  pub fn get_y_chroma_offset(&self) -> VkChromaLocationKHR {
    self.yChromaOffset
  }
  #[inline]
  pub fn get_chroma_filter(&self) -> VkFilter {
    self.chromaFilter
  }
  #[inline]
  pub fn get_force_explicit_reconstruction(&self) -> VkBool32 {
    self.forceExplicitReconstruction
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkSamplerYcbcrConversionCreateInfoKHR {
  fn default() -> VkSamplerYcbcrConversionCreateInfoKHR {
    VkSamplerYcbcrConversionCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl RawStruct for VkSamplerYcbcrConversionCreateInfoKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkSamplerYcbcrConversionInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub conversion: VkSamplerYcbcrConversionKHR,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl VkSamplerYcbcrConversionInfoKHR {
  #[inline]
  pub fn new() -> VkSamplerYcbcrConversionInfoKHR {
    unsafe {
      VkSamplerYcbcrConversionInfoKHR {
        sType: VkStructureType::E_SAMPLER_YCBCR_CONVERSION_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_conversion(&self) -> VkSamplerYcbcrConversionKHR {
    self.conversion
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkSamplerYcbcrConversionInfoKHR {
  fn default() -> VkSamplerYcbcrConversionInfoKHR {
    VkSamplerYcbcrConversionInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl RawStruct for VkSamplerYcbcrConversionInfoKHR {
  type Raw = types_raw::VkSamplerYcbcrConversionInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkBindImagePlaneMemoryInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub planeAspect: VkImageAspectFlagBits,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl VkBindImagePlaneMemoryInfoKHR {
  #[inline]
  pub fn new() -> VkBindImagePlaneMemoryInfoKHR {
    unsafe {
      VkBindImagePlaneMemoryInfoKHR {
        sType: VkStructureType::E_BIND_IMAGE_PLANE_MEMORY_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_plane_aspect(&self) -> VkImageAspectFlagBits {
    self.planeAspect
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkBindImagePlaneMemoryInfoKHR {
  fn default() -> VkBindImagePlaneMemoryInfoKHR {
    VkBindImagePlaneMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl RawStruct for VkBindImagePlaneMemoryInfoKHR {
  type Raw = types_raw::VkBindImagePlaneMemoryInfoKHR;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct VkImagePlaneMemoryRequirementsInfoKHR {
  sType: VkStructureType,
  pNext: *const c_void,
  pub planeAspect: VkImageAspectFlagBits,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl VkImagePlaneMemoryRequirementsInfoKHR {
  #[inline]
  pub fn new() -> VkImagePlaneMemoryRequirementsInfoKHR {
    unsafe {
      VkImagePlaneMemoryRequirementsInfoKHR {
        sType: VkStructureType::E_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_plane_aspect(&self) -> VkImageAspectFlagBits {
    self.planeAspect
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkImagePlaneMemoryRequirementsInfoKHR {
  fn default() -> VkImagePlaneMemoryRequirementsInfoKHR {
    VkImagePlaneMemoryRequirementsInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl RawStruct for VkImagePlaneMemoryRequirementsInfoKHR {
  type Raw = types_raw::VkImagePlaneMemoryRequirementsInfoKHR;
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_sampler_ycbcr_conversion(mut self, value: VkBool32) -> Self {
    self.samplerYcbcrConversion = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_sampler_ycbcr_conversion(&self) -> VkBool32 {
    self.samplerYcbcrConversion
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_sampler_ycbcr_conversion_features_khr() {
  assert_size!(
    types_raw::VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR,
    VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
  );
}
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
  pub fn new() -> VkSamplerYcbcrConversionImageFormatPropertiesKHR {
    unsafe {
      VkSamplerYcbcrConversionImageFormatPropertiesKHR {
        sType: VkStructureType::E_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_combined_image_sampler_descriptor_count(mut self, value: u32) -> Self {
    self.combinedImageSamplerDescriptorCount = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_combined_image_sampler_descriptor_count(&self) -> u32 {
    self.combinedImageSamplerDescriptorCount
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkSamplerYcbcrConversionImageFormatPropertiesKHR {
  fn default() -> VkSamplerYcbcrConversionImageFormatPropertiesKHR {
    VkSamplerYcbcrConversionImageFormatPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl RawStruct for VkSamplerYcbcrConversionImageFormatPropertiesKHR {
  type Raw = types_raw::VkSamplerYcbcrConversionImageFormatPropertiesKHR;
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
#[cfg(feature = "VK_EXT_validation_cache")]
pub type VkValidationCacheEXT = VkNonDispatchableHandle<VkValidationCacheEXT__>;
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct VkValidationCacheCreateInfoEXT<'a> {
  sType: VkStructureType,
  pNext: *const c_void,
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
        sType: VkStructureType::E_VALIDATION_CACHE_CREATE_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_flags(&self) -> VkValidationCacheCreateFlagsEXT {
    self.flags
  }
  #[inline]
  pub fn get_initial_data_size(&self) -> usize {
    self.initialDataSize
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub validationCache: VkValidationCacheEXT,
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl VkShaderModuleValidationCacheCreateInfoEXT {
  #[inline]
  pub fn new() -> VkShaderModuleValidationCacheCreateInfoEXT {
    unsafe {
      VkShaderModuleValidationCacheCreateInfoEXT {
        sType: VkStructureType::E_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_validation_cache(&self) -> VkValidationCacheEXT {
    self.validationCache
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl Default for VkShaderModuleValidationCacheCreateInfoEXT {
  fn default() -> VkShaderModuleValidationCacheCreateInfoEXT {
    VkShaderModuleValidationCacheCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl RawStruct for VkShaderModuleValidationCacheCreateInfoEXT {
  type Raw = types_raw::VkShaderModuleValidationCacheCreateInfoEXT;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_global_priority")]
pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub globalPriority: VkQueueGlobalPriorityEXT,
}
#[cfg(feature = "VK_EXT_global_priority")]
impl VkDeviceQueueGlobalPriorityCreateInfoEXT {
  #[inline]
  pub fn new() -> VkDeviceQueueGlobalPriorityCreateInfoEXT {
    unsafe {
      VkDeviceQueueGlobalPriorityCreateInfoEXT {
        sType: VkStructureType::E_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_global_priority(&self) -> VkQueueGlobalPriorityEXT {
    self.globalPriority
  }
}
#[cfg(feature = "VK_EXT_global_priority")]
impl Default for VkDeviceQueueGlobalPriorityCreateInfoEXT {
  fn default() -> VkDeviceQueueGlobalPriorityCreateInfoEXT {
    VkDeviceQueueGlobalPriorityCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_global_priority")]
unsafe impl RawStruct for VkDeviceQueueGlobalPriorityCreateInfoEXT {
  type Raw = types_raw::VkDeviceQueueGlobalPriorityCreateInfoEXT;
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
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "VK_EXT_external_memory_host")]
pub struct VkImportMemoryHostPointerInfoEXT {
  sType: VkStructureType,
  pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBitsKHR,
  pHostPointer: *mut c_void,
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl VkImportMemoryHostPointerInfoEXT {
  #[inline]
  pub fn new() -> VkImportMemoryHostPointerInfoEXT {
    unsafe {
      VkImportMemoryHostPointerInfoEXT {
        sType: VkStructureType::E_IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
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
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *const c_void {
    self.pNext
  }
  #[inline]
  pub fn get_handle_type(&self) -> VkExternalMemoryHandleTypeFlagBitsKHR {
    self.handleType
  }
  #[inline]
  pub fn get_host_pointer(&self) -> *mut c_void {
    self.pHostPointer
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl Default for VkImportMemoryHostPointerInfoEXT {
  fn default() -> VkImportMemoryHostPointerInfoEXT {
    VkImportMemoryHostPointerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
unsafe impl RawStruct for VkImportMemoryHostPointerInfoEXT {
  type Raw = types_raw::VkImportMemoryHostPointerInfoEXT;
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
        sType: VkStructureType::E_MEMORY_HOST_POINTER_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_memory_type_bits(mut self, value: u32) -> Self {
    self.memoryTypeBits = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_memory_type_bits(&self) -> u32 {
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
        sType: VkStructureType::E_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
        ..::std::mem::zeroed()
      }
    }
  }
  #[inline]
  pub fn set_min_imported_host_pointer_alignment(mut self, value: VkDeviceSize) -> Self {
    self.minImportedHostPointerAlignment = value;
    self
  }
  #[inline]
  pub fn get_s_type(&self) -> VkStructureType {
    self.sType
  }
  #[inline]
  pub fn get_next(&self) -> *mut c_void {
    self.pNext
  }
  #[inline]
  pub fn get_min_imported_host_pointer_alignment(&self) -> VkDeviceSize {
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
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_external_memory_host_properties_ext() {
  assert_size!(
    types_raw::VkPhysicalDeviceExternalMemoryHostPropertiesEXT,
    VkPhysicalDeviceExternalMemoryHostPropertiesEXT
  );
}
