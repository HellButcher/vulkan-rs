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

/// Opaque handle to a buffer object
///
/// Buffers represent linear arrays of data which are used for various purposes by
/// binding them to a graphics or compute pipeline via descriptor sets or via
/// certain commands, or by directly specifying them as parameters to certain
/// commands.
///
/// Buffers are represented by `VkBuffer` handles.
///
pub type VkBuffer = VkNonDispatchableHandle<VkBuffer__>;

/// Structure specifying a buffer memory barrier
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `srcAccessMask` is a bitmask of `VkAccessFlagBits` specifying a [source
///     access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks).
///
///   - `dstAccessMask` is a bitmask of `VkAccessFlagBits` specifying a [destination
///     access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks).
///
///   - `srcQueueFamilyIndex` is the source queue family for a [queue family
///     ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers).
///
///   - `dstQueueFamilyIndex` is the destination queue family for a [queue family
///     ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers).
///
///   - `buffer` is a handle to the buffer whose backing memory is affected by the
///     barrier.
///
///   - `offset` is an offset in bytes into the backing memory for `buffer`; this is
///     relative to the base offset as bound to the buffer (see
///     `vkBindBufferMemory`).
///
///   - `size` is a size in bytes of the affected area of backing memory for
///     `buffer`, or `VK_WHOLE_SIZE` to use the range from `offset` to the end of
///     the buffer.
///
/// The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is limited
/// to access to memory through the specified buffer range, via access types in the
/// [source access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks) specified by
/// `srcAccessMask`. If `srcAccessMask` includes `VK_ACCESS_HOST_WRITE_BIT`, memory
/// writes performed by that access type are also made visible, as that access type
/// is not performed through a resource.
///
/// The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
/// limited to access to memory through the specified buffer range, via access types
/// in the [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks). specified by
/// `dstAccessMask`. If `dstAccessMask` includes `VK_ACCESS_HOST_WRITE_BIT` or
/// `VK_ACCESS_HOST_READ_BIT`, available memory writes are also made visible to
/// accesses of those types, as those access types are not performed through a
/// resource.
///
/// If `srcQueueFamilyIndex` is not equal to `dstQueueFamilyIndex`, and
/// `srcQueueFamilyIndex` is equal to the current queue family, then the memory
/// barrier defines a [queue family release
/// operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers-release) for the specified buffer
/// range, and the second access scope includes no access, as if `dstAccessMask` was
/// `0`.
///
/// If `dstQueueFamilyIndex` is not equal to `srcQueueFamilyIndex`, and
/// `dstQueueFamilyIndex` is equal to the current queue family, then the memory
/// barrier defines a [queue family acquire
/// operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers-acquire) for the specified buffer
/// range, and the first access scope includes no access, as if `srcAccessMask` was
/// `0`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
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
}
impl Default for VkBufferMemoryBarrier {
  fn default() -> VkBufferMemoryBarrier {
    VkBufferMemoryBarrier::new()
  }
}
impl RawStruct for VkBufferMemoryBarrier {
  type Raw = types_raw::VkBufferMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_memory_barrier() {
  assert_size!(types_raw::VkBufferMemoryBarrier, VkBufferMemoryBarrier);
}

/// Structure specifying a dispatch indirect command
///
///   - `x` is the number of local workgroups to dispatch in the X dimension.
///
///   - `y` is the number of local workgroups to dispatch in the Y dimension.
///
///   - `z` is the number of local workgroups to dispatch in the Z dimension.
///
/// The members of `VkDispatchIndirectCommand` have the same meaning as the
/// corresponding parameters of `vkCmdDispatch`.
///
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
}
impl Default for VkDispatchIndirectCommand {
  fn default() -> VkDispatchIndirectCommand {
    VkDispatchIndirectCommand::new()
  }
}
impl RawStruct for VkDispatchIndirectCommand {
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
///
///   - `indexCount` is the number of vertices to draw.
///
///   - `instanceCount` is the number of instances to draw.
///
///   - `firstIndex` is the base index within the index buffer.
///
///   - `vertexOffset` is the value added to the vertex index before indexing into
///     the vertex buffer.
///
///   - `firstInstance` is the instance ID of the first instance to draw.
///
/// The members of `VkDrawIndexedIndirectCommand` have the same meaning as the
/// similarly named parameters of `vkCmdDrawIndexed`.
///
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
}
impl Default for VkDrawIndexedIndirectCommand {
  fn default() -> VkDrawIndexedIndirectCommand {
    VkDrawIndexedIndirectCommand::new()
  }
}
impl RawStruct for VkDrawIndexedIndirectCommand {
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
///
///   - `vertexCount` is the number of vertices to draw.
///
///   - `instanceCount` is the number of instances to draw.
///
///   - `firstVertex` is the index of the first vertex to draw.
///
///   - `firstInstance` is the instance ID of the first instance to draw.
///
/// The members of `VkDrawIndirectCommand` have the same meaning as the similarly
/// named parameters of `vkCmdDraw`.
///
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
}
impl Default for VkDrawIndirectCommand {
  fn default() -> VkDrawIndirectCommand {
    VkDrawIndirectCommand::new()
  }
}
impl RawStruct for VkDrawIndirectCommand {
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
///
/// Images represent multidimensional - up to 3 - arrays of data which can: be used
/// for various purposes (e.g. attachments, textures), by binding them to a graphics
/// or compute pipeline via descriptor sets, or by directly specifying them as
/// parameters to certain commands.
///
/// Images are represented by `VkImage` handles.
///
pub type VkImage = VkNonDispatchableHandle<VkImage__>;

/// Structure specifying a image subresource range
///
///   - `aspectMask` is a bitmask of `VkImageAspectFlagBits` specifying which
///     aspect(s) of the image are included in the view.
///
///   - `baseMipLevel` is the first mipmap level accessible to the view.
///
///   - `levelCount` is the number of mipmap levels (starting from `baseMipLevel`)
///     accessible to the view.
///
///   - `baseArrayLayer` is the first array layer accessible to the view.
///
///   - `layerCount` is the number of array layers (starting from `baseArrayLayer`)
///     accessible to the view.
///
/// The number of mipmap levels and array layers must: be a subset of the image
/// subresources in the image. If an application wants to use all mip levels or
/// layers in an image after the `baseMipLevel` or `baseArrayLayer`, it can: set
/// `levelCount` and `layerCount` to the special values `VK_REMAINING_MIP_LEVELS`
/// and `VK_REMAINING_ARRAY_LAYERS` without knowing the exact number of mip levels
/// or layers.
///
/// For cube and cube array image views, the layers of the image view starting at
/// `baseArrayLayer` correspond to faces in the order +X, -X, +Y, -Y, +Z, -Z. For
/// cube arrays, each set of six sequential layers is a single cube, so the number
/// of cube maps in a cube map array view is *pname:layerCount / 6*, and image array
/// layer (`baseArrayLayer` + i) is face index (i mod 6) of cube *i / 6*. If the
/// number of layers in the view, whether set explicitly in `layerCount` or implied
/// by `VK_REMAINING_ARRAY_LAYERS`, is not a multiple of 6, behavior when indexing
/// the last cube is undefined.
///
/// `aspectMask` must: be only `VK_IMAGE_ASPECT_COLOR_BIT`,
/// `VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT` if `format` is a
/// color, depth-only or stencil-only format, respectively, except if `format` is a
/// [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion). If
/// using a depth/stencil format with both depth and stencil components,
/// `aspectMask` must: include at least one of `VK_IMAGE_ASPECT_DEPTH_BIT` and
/// `VK_IMAGE_ASPECT_STENCIL_BIT`, and can: include both.
///
/// When the `VkImageSubresourceRange` structure is used to select a subset of the
/// slices of a 3D image’s mip level in order to create a 2D or 2D array image view
/// of a 3D image created with `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT`,
/// `baseArrayLayer` and `layerCount` specify the first slice index and the number
/// of slices to include in the created image view. Such an image view can: be used
/// as a framebuffer attachment that refers only to the specified range of slices of
/// the selected mip level. However, any layout transitions performed on such an
/// attachment view during a render pass instance still apply to the entire
/// subresource referenced which includes all the slices of the selected mip level.
///
/// When using an imageView of a depth/stencil image to populate a descriptor set
/// (e.g. for sampling in the shader, or for use as an input attachment), the
/// `aspectMask` must: only include one bit and selects whether the imageView is
/// used for depth reads (i.e. using a floating-point sampler or input attachment in
/// the shader) or stencil reads (i.e. using an unsigned integer sampler or input
/// attachment in the shader). When an imageView of a depth/stencil image is used as
/// a depth/stencil framebuffer attachment, the `aspectMask` is ignored and both
/// depth and stencil image subresources are used.
///
/// The `components` member is of type `VkComponentMapping`, and describes a
/// remapping from components of the image to components of the vector returned by
/// shader image instructions. This remapping must: be identity for storage image
/// descriptors, input attachment descriptors, framebuffer attachments, and any
/// `VkImageView` used with a combined image sampler that enables [sampler
/// Y’C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion).
///
/// When creating a `VkImageView`, if [sampler Y’C<sub>B</sub>C<sub>R</sub>
/// conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion) is enabled in the sampler, the
/// `aspectMask` of a `subresourceRange` used by the `VkImageView` must: be
/// `VK_IMAGE_ASPECT_COLOR_BIT`.
///
/// When creating a `VkImageView`, if sampler Y’C<sub>B</sub>C<sub>R</sub>
/// conversion is not enabled in the sampler and the image `format` is
/// [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion), the image
/// must: have been created with `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`, and the
/// `aspectMask` of the ``VkImageView’s `subresourceRange`` must: be
/// `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` or
/// `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`.
///
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
}
impl Default for VkImageSubresourceRange {
  fn default() -> VkImageSubresourceRange {
    VkImageSubresourceRange::new()
  }
}
impl RawStruct for VkImageSubresourceRange {
  type Raw = types_raw::VkImageSubresourceRange;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_subresource_range() {
  assert_size!(types_raw::VkImageSubresourceRange, VkImageSubresourceRange);
}

/// Structure specifying the parameters of an image memory barrier
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `srcAccessMask` is a bitmask of `VkAccessFlagBits` specifying a [source
///     access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks).
///
///   - `dstAccessMask` is a bitmask of `VkAccessFlagBits` specifying a [destination
///     access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks).
///
///   - `oldLayout` is the old layout in an [image layout
///     transition](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-image-layout-transitions).
///
///   - `newLayout` is the new layout in an [image layout
///     transition](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-image-layout-transitions).
///
///   - `srcQueueFamilyIndex` is the source queue family for a [queue family
///     ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers).
///
///   - `dstQueueFamilyIndex` is the destination queue family for a [queue family
///     ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers).
///
///   - `image` is a handle to the image affected by this barrier.
///
///   - `subresourceRange` describes the [image subresource
///     range](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-image-views) within `image` that is affected by this
///     barrier.
///
/// The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is limited
/// to access to memory through the specified image subresource range, via access
/// types in the [source access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks) specified by
/// `srcAccessMask`. If `srcAccessMask` includes `VK_ACCESS_HOST_WRITE_BIT`, memory
/// writes performed by that access type are also made visible, as that access type
/// is not performed through a resource.
///
/// The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
/// limited to access to memory through the specified image subresource range, via
/// access types in the [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks)
/// specified by `dstAccessMask`. If `dstAccessMask` includes
/// `VK_ACCESS_HOST_WRITE_BIT` or `VK_ACCESS_HOST_READ_BIT`, available memory writes
/// are also made visible to accesses of those types, as those access types are not
/// performed through a resource.
///
/// If `srcQueueFamilyIndex` is not equal to `dstQueueFamilyIndex`, and
/// `srcQueueFamilyIndex` is equal to the current queue family, then the memory
/// barrier defines a [queue family release
/// operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers-release) for the specified image
/// subresource range, and the second access scope includes no access, as if
/// `dstAccessMask` was `0`.
///
/// If `dstQueueFamilyIndex` is not equal to `srcQueueFamilyIndex`, and
/// `dstQueueFamilyIndex` is equal to the current queue family, then the memory
/// barrier defines a [queue family acquire
/// operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers-acquire) for the specified image
/// subresource range, and the first access scope includes no access, as if
/// `srcAccessMask` was `0`.
///
/// If `oldLayout` is not equal to `newLayout`, then the memory barrier defines an
/// [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-image-layout-transitions) for the
/// specified image subresource range.
///
/// Layout transitions that are performed via image memory barriers execute in their
/// entirety in [submission order](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-submission-order), relative to
/// other image layout transitions submitted to the same queue, including those
/// performed by [render passes](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass). In effect there is an implicit
/// execution dependency from each such layout transition to all layout transitions
/// previously submitted to the same queue.
///
/// The image layout of each image subresource of a depth/stencil image created with
/// `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` is dependent on the
/// last sample locations used to render to the image subresource as a depth/stencil
/// attachment, thus when the `image` member of an `VkImageMemoryBarrier` is an
/// image created with this flag the application can: chain a
/// `VkSampleLocationsInfoEXT` structure to the `pNext` chain of
/// `VkImageMemoryBarrier` to specify the sample locations to use during the image
/// layout transition.
///
/// If the `VkSampleLocationsInfoEXT` structure in the `pNext` chain of
/// `VkImageMemoryBarrier` does not match the sample location state last used to
/// render to the image subresource range specified by `subresourceRange` or if no
/// `VkSampleLocationsInfoEXT` structure is in the `pNext` chain of
/// `VkImageMemoryBarrier` then the contents of the given image subresource range
/// becomes undefined as if `oldLayout` would equal `VK_IMAGE_LAYOUT_UNDEFINED`.
///
/// If `image` has a multi-planar format and the image is *disjoint*, then including
/// `VK_IMAGE_ASPECT_COLOR_BIT` in the `aspectMask` member of `subresourceRange` is
/// equivalent to including `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`,
/// `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR`, and (for three-plane formats only)
/// `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
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
}
impl Default for VkImageMemoryBarrier {
  fn default() -> VkImageMemoryBarrier {
    VkImageMemoryBarrier::new()
  }
}
impl RawStruct for VkImageMemoryBarrier {
  type Raw = types_raw::VkImageMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_memory_barrier() {
  assert_size!(types_raw::VkImageMemoryBarrier, VkImageMemoryBarrier);
}

/// Structure specifying a global memory barrier
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `srcAccessMask` is a bitmask of `VkAccessFlagBits` specifying a [source
///     access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks).
///
///   - `dstAccessMask` is a bitmask of `VkAccessFlagBits` specifying a [destination
///     access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks).
///
/// The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is limited
/// to access types in the [source access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks)
/// specified by `srcAccessMask`.
///
/// The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
/// limited to access types in the [destination access
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks) specified by `dstAccessMask`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
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
}
impl Default for VkMemoryBarrier {
  fn default() -> VkMemoryBarrier {
    VkMemoryBarrier::new()
  }
}
impl RawStruct for VkMemoryBarrier {
  type Raw = types_raw::VkMemoryBarrier;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_barrier() {
  assert_size!(types_raw::VkMemoryBarrier, VkMemoryBarrier);
}

/// Structure specifying application info
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `pApplicationName` is `NULL` or is a pointer to a null-terminated UTF-8
///     string containing the name of the application.
///
///   - `applicationVersion` is an unsigned integer variable containing the
///     developer-supplied version number of the application.
///
///   - `pEngineName` is `NULL` or is a pointer to a null-terminated UTF-8 string
///     containing the name of the engine (if any) used to create the application.
///
///   - `engineVersion` is an unsigned integer variable containing the
///     developer-supplied version number of the engine used to create the
///     application.
///
///   - `apiVersion` is the version of the Vulkan API against which the application
///     expects to run, encoded as described in the [API Version Numbers and
///     Semantics](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-versionnum) section. If `apiVersion` is 0 the
///     implementation must: ignore it, otherwise if the implementation does not
///     support the requested `apiVersion`, or an effective substitute for
///     `apiVersion`, it must: return `VK_ERROR_INCOMPATIBLE_DRIVER`. The patch
///     version number specified in `apiVersion` is ignored when creating an
///     instance object. Only the major and minor versions of the instance must:
///     match those requested in `apiVersion`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl<'a> Default for VkApplicationInfo<'a> {
  fn default() -> VkApplicationInfo<'a> {
    VkApplicationInfo::new()
  }
}
impl<'a> RawStruct for VkApplicationInfo<'a> {
  type Raw = types_raw::VkApplicationInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_application_info() {
  assert_size!(types_raw::VkApplicationInfo, VkApplicationInfo);
}

/// Structure specifying parameters of a newly created instance
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `pApplicationInfo` is `NULL` or a pointer to an instance of
///     `VkApplicationInfo`. If not `NULL`, this information helps implementations
///     recognize behavior inherent to classes of applications. `VkApplicationInfo`
///     is defined in detail below.
///
///   - `enabledLayerCount` is the number of global layers to enable.
///
///   - `ppEnabledLayerNames` is a pointer to an array of `enabledLayerCount`
///     null-terminated UTF-8 strings containing the names of layers to enable for
///     the created instance. See the [Layers](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#extended-functionality-layers)
///     section for further details.
///
///   - `enabledExtensionCount` is the number of global extensions to enable.
///
///   - `ppEnabledExtensionNames` is a pointer to an array of
///     `enabledExtensionCount` null-terminated UTF-8 strings containing the names
///     of extensions to enable.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl<'a> Default for VkInstanceCreateInfo<'a> {
  fn default() -> VkInstanceCreateInfo<'a> {
    VkInstanceCreateInfo::new()
  }
}
impl<'a> RawStruct for VkInstanceCreateInfo<'a> {
  type Raw = types_raw::VkInstanceCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_instance_create_info() {
  assert_size!(types_raw::VkInstanceCreateInfo, VkInstanceCreateInfo);
}

/// Application-defined memory allocation function
///
///   - `pUserData` is the value specified for `VkAllocationCallbacks::pUserData` in
///     the allocator specified by the application.
///
///   - `size` is the size in bytes of the requested allocation.
///
///   - `alignment` is the requested alignment of the allocation in bytes and must:
///     be a power of two.
///
///   - `allocationScope` is a `VkSystemAllocationScope` value specifying the
///     allocation scope of the lifetime of the allocation, as described
///     [here](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-host-allocation-scope).
///
/// If `pfnAllocation` is unable to allocate the requested memory, it must: return
/// `NULL`. If the allocation was successful, it must: return a valid pointer to
/// memory allocation containing at least `size` bytes, and with the pointer value
/// being a multiple of `alignment`.
///
/// > **Note**
/// >
/// > Correct Vulkan operation cannot: be assumed if the application does not follow
/// > these rules.
/// >
/// > For example, `pfnAllocation` (or `pfnReallocation`) could cause termination of
/// > running Vulkan instance(s) on a failed allocation for debugging purposes,
/// > either directly or indirectly. In these circumstances, it cannot: be assumed
/// > that any part of any affected `VkInstance` objects are going to operate
/// > correctly (even `vkDestroyInstance`), and the application must: ensure it
/// > cleans up properly via other means (e.g. process termination).
///
/// If `pfnAllocation` returns `NULL`, and if the implementation is unable to
/// continue correct processing of the current command without the requested
/// allocation, it must: treat this as a run-time error, and generate
/// `VK_ERROR_OUT_OF_HOST_MEMORY` at the appropriate time for the command in which
/// the condition was detected, as described in [Return
/// Codes](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-errorcodes).
///
/// If the implementation is able to continue correct processing of the current
/// command without the requested allocation, then it may: do so, and must: not
/// generate `VK_ERROR_OUT_OF_HOST_MEMORY` as a result of this failed allocation.
///
pub use types_raw::PFN_vkAllocationFunction;

/// Application-defined memory reallocation function
///
///   - `pUserData` is the value specified for `VkAllocationCallbacks::pUserData` in
///     the allocator specified by the application.
///
///   - `pOriginal` must: be either `NULL` or a pointer previously returned by
///     `pfnReallocation` or `pfnAllocation` of the same allocator.
///
///   - `size` is the size in bytes of the requested allocation.
///
///   - `alignment` is the requested alignment of the allocation in bytes and must:
///     be a power of two.
///
///   - `allocationScope` is a `VkSystemAllocationScope` value specifying the
///     allocation scope of the lifetime of the allocation, as described
///     [here](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-host-allocation-scope).
///
/// `pfnReallocation` must: return an allocation with enough space for `size` bytes,
/// and the contents of the original allocation from bytes zero to min(original
/// size, new size) - 1 must: be preserved in the returned allocation. If `size` is
/// larger than the old size, the contents of the additional space are undefined. If
/// satisfying these requirements involves creating a new allocation, then the old
/// allocation should: be freed.
///
/// If `pOriginal` is `NULL`, then `pfnReallocation` must: behave equivalently to a
/// call to tlink:PFN\_vkAllocationFunction with the same parameter values (without
/// `pOriginal`).
///
/// If `size` is zero, then `pfnReallocation` must: behave equivalently to a call to
/// tlink:PFN\_vkFreeFunction with the same `pUserData` parameter value, and
/// `pMemory` equal to `pOriginal`.
///
/// If `pOriginal` is non-`NULL`, the implementation must: ensure that `alignment`
/// is equal to the `alignment` used to originally allocate `pOriginal`.
///
/// If this function fails and `pOriginal` is non-`NULL` the application must: not
/// free the old allocation.
///
/// `pfnReallocation` must: follow the same [rules for return values as
/// tname:PFN\_vkAllocationFunction](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocationFunction_return_rules).
///
pub use types_raw::PFN_vkReallocationFunction;

/// Application-defined memory free function
///
///   - `pUserData` is the value specified for `VkAllocationCallbacks::pUserData` in
///     the allocator specified by the application.
///
///   - `pMemory` is the allocation to be freed.
///
/// `pMemory` may: be `NULL`, which the callback must: handle safely. If `pMemory`
/// is non-`NULL`, it must: be a pointer previously allocated by `pfnAllocation` or
/// `pfnReallocation`. The application should: free this memory.
///
pub use types_raw::PFN_vkFreeFunction;

/// Application-defined memory allocation notification function
///
///   - `pUserData` is the value specified for `VkAllocationCallbacks::pUserData` in
///     the allocator specified by the application.
///
///   - `size` is the requested size of an allocation.
///
///   - `allocationType` is a `VkInternalAllocationType` value specifying the
///     requested type of an allocation.
///
///   - `allocationScope` is a `VkSystemAllocationScope` value specifying the
///     allocation scope of the lifetime of the allocation, as described
///     [here](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-host-allocation-scope).
///
/// This is a purely informational callback.
///
pub use types_raw::PFN_vkInternalAllocationNotification;

/// Application-defined memory free notification function
///
///   - `pUserData` is the value specified for `VkAllocationCallbacks::pUserData` in
///     the allocator specified by the application.
///
///   - `size` is the requested size of an allocation.
///
///   - `allocationType` is a `VkInternalAllocationType` value specifying the
///     requested type of an allocation.
///
///   - `allocationScope` is a `VkSystemAllocationScope` value specifying the
///     allocation scope of the lifetime of the allocation, as described
///     [here](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-host-allocation-scope).
///
pub use types_raw::PFN_vkInternalFreeNotification;

/// Structure containing callback function pointers for memory allocation
///
/// Allocators are provided by the application as a pointer to a
/// `VkAllocationCallbacks` structure.
///
///   - `pUserData` is a value to be interpreted by the implementation of the
///     callbacks. When any of the callbacks in `VkAllocationCallbacks` are called,
///     the Vulkan implementation will pass this value as the first parameter to the
///     callback. This value can: vary each time an allocator is passed into a
///     command, even when the same object takes an allocator in multiple commands.
///
///   - `pfnAllocation` is a pointer to an application-defined memory allocation
///     function of type tlink:PFN\_vkAllocationFunction.
///
///   - `pfnReallocation` is a pointer to an application-defined memory reallocation
///     function of type tlink:PFN\_vkReallocationFunction.
///
///   - `pfnFree` is a pointer to an application-defined memory free function of
///     type tlink:PFN\_vkFreeFunction.
///
///   - `pfnInternalAllocation` is a pointer to an application-defined function that
///     is called by the implementation when the implementation makes internal
///     allocations, and it is of type tlink:PFN\_vkInternalAllocationNotification.
///
///   - `pfnInternalFree` is a pointer to an application-defined function that is
///     called by the implementation when the implementation frees internal
///     allocations, and it is of type tlink:PFN\_vkInternalFreeNotification.
///
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
}
impl Default for VkAllocationCallbacks {
  fn default() -> VkAllocationCallbacks {
    VkAllocationCallbacks::new()
  }
}
impl RawStruct for VkAllocationCallbacks {
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
///
/// There is no global state in Vulkan and all per-application state is stored in a
/// `VkInstance` object. Creating a `VkInstance` object initializes the Vulkan
/// library and allows the application to pass information about itself to the
/// implementation.
///
/// Instances are represented by `VkInstance` handles.
///
pub type VkInstance = VkDispatchableHandle<VkInstance__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPhysicalDevice__ {}

/// Opaque handle to a physical device object
///
/// Vulkan separates the concept of *physical* and *logical* devices. A physical
/// device usually represents a single device in a system (perhaps made up of
/// several individual hardware devices working together), of which there are a
/// finite number. A logical device represents an application’s view of the device.
///
/// Physical devices are represented by `VkPhysicalDevice` handles.
///
pub type VkPhysicalDevice = VkDispatchableHandle<VkPhysicalDevice__>;

/// Structure describing the fine-grained features that can be supported by an
/// implementation
///
/// The members of the `VkPhysicalDeviceFeatures` structure describe the following
/// features:
///
///   - `robustBufferAccess` indicates that accesses to buffers are bounds-checked
///     against the range of the buffer descriptor (as determined by
///     `VkDescriptorBufferInfo::range`, `VkBufferViewCreateInfo::range`, or the
///     size of the buffer). Out of bounds accesses must: not cause application
///     termination, and the effects of shader loads, stores, and atomics must:
///     conform to an implementation-dependent behavior as described below.
///
///       - A buffer access is considered to be out of bounds if any of the
///         following are true:
///
///           - The pointer was formed by `OpImageTexelPointer` and the coordinate
///             is less than zero or greater than or equal to the number of whole
///             elements in the bound range.
///
///           - The pointer was not formed by `OpImageTexelPointer` and the object
///             pointed to is not wholly contained within the bound range. This
///             includes accesses performed via *variable pointers* where the buffer
///             descriptor being accessed cannot be statically determined.
///             Uninitialized pointers and pointers equal to `OpConstantNull` are
///             treated as pointing to a zero-sized object, so all accesses through
///             such pointers are considered to be out of bounds.
///
///             > **Note**
///             >
///             > If a SPIR-V `OpLoad` instruction loads a structure and the tail
///             > end of the structure is out of bounds, then all members of the
///             > structure are considered out of bounds even if the members at the
///             > end are not statically used.
///
///           - If any buffer access in a given SPIR-V block is determined to be out
///             of bounds, then any other access of the same type (load, store, or
///             atomic) in the same SPIR-V block that accesses an address less than
///             16 bytes away from the out of bounds address may: also be considered
///             out of bounds.
///
///       - Out-of-bounds buffer loads will return any of the following values:
///
///           - Values from anywhere within the memory range(s) bound to the buffer
///             (possibly including bytes of memory past the end of the buffer, up
///             to the end of the bound range).
///
///           - Zero values, or (0,0,0,x) vectors for vector reads where x is a
///             valid value represented in the type of the vector components and
///             may: be any of:
///
///               - 0, 1, or the maximum representable positive integer value, for
///                 signed or unsigned integer components
///
///               - 0.0 or 1.0, for floating-point components
///
///       - Out-of-bounds writes may: modify values within the memory range(s) bound
///         to the buffer, but must: not modify any other memory.
///
///       - Out-of-bounds atomics may: modify values within the memory range(s)
///         bound to the buffer, but must: not modify any other memory, and return
///         an undefined value.
///
///       - Vertex input attributes are considered out of bounds if the offset of
///         the attribute in the bound vertex buffer range plus the size of the
///         attribute is greater than either:
///
///           - `vertexBufferRangeSize`, if `bindingStride` == 0; or
///
///           - (`vertexBufferRangeSize` - (`vertexBufferRangeSize` %
///             `bindingStride`))
///
///         where `vertexBufferRangeSize` is the byte size of the memory range bound
///         to the vertex buffer binding and `bindingStride` is the byte stride of
///         the corresponding vertex input binding. Further, if any vertex input
///         attribute using a specific vertex input binding is out of bounds, then
///         all vertex input attributes using that vertex input binding for that
///         vertex shader invocation are considered out of bounds.
///
///           - If a vertex input attribute is out of bounds, it will be assigned
///             one of the following values:
///
///               - Values from anywhere within the memory range(s) bound to the
///                 buffer, converted according to the format of the attribute.
///
///               - Zero values, format converted according to the format of the
///                 attribute.
///
///               - Zero values, or (0,0,0,x) vectors, as described above.
///
///       - If `robustBufferAccess` is not enabled, out of bounds accesses may:
///         corrupt any memory within the process and cause undefined behavior up to
///         and including application termination.
///
///   - `fullDrawIndexUint32` indicates the full 32-bit range of indices is
///     supported for indexed draw calls when using a `VkIndexType` of
///     `VK_INDEX_TYPE_UINT32`. `maxDrawIndexedIndexValue` is the maximum index
///     value that may: be used (aside from the primitive restart index, which is
///     always 2<sup>32</sup>-1 when the `VkIndexType` is `VK_INDEX_TYPE_UINT32`).
///     If this feature is supported, `maxDrawIndexedIndexValue` must: be
///     2<sup>32</sup>-1; otherwise it must: be no smaller than 2<sup>24</sup>-1.
///     See [maxDrawIndexedIndexValue](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits-maxDrawIndexedIndexValue).
///
///   - `imageCubeArray` indicates whether image views with a `VkImageViewType` of
///     `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY` can: be created, and that the corresponding
///     `SampledCubeArray` and `ImageCubeArray` SPIR-V capabilities can: be used in
///     shader code.
///
///   - `independentBlend` indicates whether the
///     `VkPipelineColorBlendAttachmentState` settings are controlled independently
///     per-attachment. If this feature is not enabled, the
///     `VkPipelineColorBlendAttachmentState` settings for all color attachments
///     must: be identical. Otherwise, a different
///     `VkPipelineColorBlendAttachmentState` can: be provided for each bound color
///     attachment.
///
///   - `geometryShader` indicates whether geometry shaders are supported. If this
///     feature is not enabled, the `VK_SHADER_STAGE_GEOMETRY_BIT` and
///     `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT` enum values must: not be used. This
///     also indicates whether shader modules can: declare the `Geometry`
///     capability.
///
///   - `tessellationShader` indicates whether tessellation control and evaluation
///     shaders are supported. If this feature is not enabled, the
///     `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`,
///     `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`,
///     `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT`,
///     `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`, and
///     `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO` enum values
///     must: not be used. This also indicates whether shader modules can: declare
///     the `Tessellation` capability.
///
///   - `sampleRateShading` indicates whether [Sample
///     Shading](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#primsrast-sampleshading) and multisample interpolation are
///     supported. If this feature is not enabled, the `sampleShadingEnable` member
///     of the `VkPipelineMultisampleStateCreateInfo` structure must: be set to
///     `VK_FALSE` and the `minSampleShading` member is ignored. This also indicates
///     whether shader modules can: declare the `SampleRateShading` capability.
///
///   - `dualSrcBlend` indicates whether blend operations which take two sources are
///     supported. If this feature is not enabled, the `VK_BLEND_FACTOR_SRC1_COLOR`,
///     `VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`, `VK_BLEND_FACTOR_SRC1_ALPHA`, and
///     `VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA` enum values must: not be used as
///     source or destination blending factors. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-dsb).
///
///   - `logicOp` indicates whether logic operations are supported. If this feature
///     is not enabled, the `logicOpEnable` member of the
///     `VkPipelineColorBlendStateCreateInfo` structure must: be set to `VK_FALSE`,
///     and the `logicOp` member is ignored.
///
///   - `multiDrawIndirect` indicates whether multiple draw indirect is supported.
///     If this feature is not enabled, the `drawCount` parameter to the
///     `vkCmdDrawIndirect` and `vkCmdDrawIndexedIndirect` commands must: be 0 or 1.
///     The `maxDrawIndirectCount` member of the `VkPhysicalDeviceLimits` structure
///     must: also be 1 if this feature is not supported. See
///     [maxDrawIndirectCount](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits-maxDrawIndirectCount).
///
///   - `drawIndirectFirstInstance` indicates whether indirect draw calls support
///     the `firstInstance` parameter. If this feature is not enabled, the
///     `firstInstance` member of all `VkDrawIndirectCommand` and
///     `VkDrawIndexedIndirectCommand` structures that are provided to the
///     `vkCmdDrawIndirect` and `vkCmdDrawIndexedIndirect` commands must: be 0.
///
///   - `depthClamp` indicates whether depth clamping is supported. If this feature
///     is not enabled, the `depthClampEnable` member of the
///     `VkPipelineRasterizationStateCreateInfo` structure must: be set to
///     `VK_FALSE`. Otherwise, setting `depthClampEnable` to `VK_TRUE` will enable
///     depth clamping.
///
///   - `depthBiasClamp` indicates whether depth bias clamping is supported. If this
///     feature is not enabled, the `depthBiasClamp` member of the
///     `VkPipelineRasterizationStateCreateInfo` structure must: be set to 0.0
///     unless the `VK_DYNAMIC_STATE_DEPTH_BIAS` dynamic state is enabled, and the
///     `depthBiasClamp` parameter to `vkCmdSetDepthBias` must: be set to 0.0.
///
///   - `fillModeNonSolid` indicates whether point and wireframe fill modes are
///     supported. If this feature is not enabled, the `VK_POLYGON_MODE_POINT` and
///     `VK_POLYGON_MODE_LINE` enum values must: not be used.
///
///   - `depthBounds` indicates whether depth bounds tests are supported. If this
///     feature is not enabled, the `depthBoundsTestEnable` member of the
///     `VkPipelineDepthStencilStateCreateInfo` structure must: be set to
///     `VK_FALSE`. When `depthBoundsTestEnable` is set to `VK_FALSE`, the
///     `minDepthBounds` and `maxDepthBounds` members of the
///     `VkPipelineDepthStencilStateCreateInfo` structure are ignored.
///
///   - `wideLines` indicates whether lines with width other than 1.0 are supported.
///     If this feature is not enabled, the `lineWidth` member of the
///     `VkPipelineRasterizationStateCreateInfo` structure must: be set to 1.0
///     unless the `VK_DYNAMIC_STATE_LINE_WIDTH` dynamic state is enabled, and the
///     `lineWidth` parameter to `vkCmdSetLineWidth` must: be set to 1.0. When this
///     feature is supported, the range and granularity of supported line widths are
///     indicated by the `lineWidthRange` and `lineWidthGranularity` members of the
///     `VkPhysicalDeviceLimits` structure, respectively.
///
///   - `largePoints` indicates whether points with size greater than 1.0 are
///     supported. If this feature is not enabled, only a point size of 1.0 written
///     by a shader is supported. The range and granularity of supported point sizes
///     are indicated by the `pointSizeRange` and `pointSizeGranularity` members of
///     the `VkPhysicalDeviceLimits` structure, respectively.
///
///   - `alphaToOne` indicates whether the implementation is able to replace the
///     alpha value of the color fragment output from the fragment shader with the
///     maximum representable alpha value for fixed-point colors or 1.0 for
///     floating-point colors. If this feature is not enabled, then the
///     `alphaToOneEnable` member of the `VkPipelineMultisampleStateCreateInfo`
///     structure must: be set to `VK_FALSE`. Otherwise setting `alphaToOneEnable`
///     to `VK_TRUE` will enable alpha-to-one behavior.
///
///   - `multiViewport` indicates whether more than one viewport is supported. If
///     this feature is not enabled, the `viewportCount` and `scissorCount` members
///     of the `VkPipelineViewportStateCreateInfo` structure must: be set to 1.
///     Similarly, the `viewportCount` parameter to the `vkCmdSetViewport` command
///     and the `scissorCount` parameter to the `vkCmdSetScissor` command must: be
///     1, and the `firstViewport` parameter to the `vkCmdSetViewport` command and
///     the `firstScissor` parameter to the `vkCmdSetScissor` command must: be 0.
///
///   - `samplerAnisotropy` indicates whether anisotropic filtering is supported. If
///     this feature is not enabled, the `anisotropyEnable` member of the
///     `VkSamplerCreateInfo` structure must: be `VK_FALSE`.
///
///   - `textureCompressionETC2` indicates whether all of the ETC2 and EAC
///     compressed texture formats are supported. If this feature is enabled, then
///     the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
///     and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features must: be
///     supported in `optimalTilingFeatures` for the following formats:
///
///       - `VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK`
///
///       - `VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK`
///
///       - `VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK`
///
///       - `VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK`
///
///       - `VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK`
///
///       - `VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK`
///
///       - `VK_FORMAT_EAC_R11_UNORM_BLOCK`
///
///       - `VK_FORMAT_EAC_R11_SNORM_BLOCK`
///
///       - `VK_FORMAT_EAC_R11G11_UNORM_BLOCK`
///
///       - `VK_FORMAT_EAC_R11G11_SNORM_BLOCK`
///
///     `vkGetPhysicalDeviceFormatProperties` and
///     `vkGetPhysicalDeviceImageFormatProperties` can: be used to check for
///     additional supported properties of individual formats.
///
///   - `textureCompressionASTC_LDR` indicates whether all of the ASTC LDR
///     compressed texture formats are supported. If this feature is enabled, then
///     the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
///     and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features must: be
///     supported in `optimalTilingFeatures` for the following formats:
///
///       - `VK_FORMAT_ASTC_4x4_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_4x4_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_5x4_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_5x4_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_5x5_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_5x5_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_6x5_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_6x5_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_6x6_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_6x6_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_8x5_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_8x5_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_8x6_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_8x6_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_8x8_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_8x8_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_10x5_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_10x5_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_10x6_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_10x6_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_10x8_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_10x8_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_10x10_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_10x10_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_12x10_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_12x10_SRGB_BLOCK`
///
///       - `VK_FORMAT_ASTC_12x12_UNORM_BLOCK`
///
///       - `VK_FORMAT_ASTC_12x12_SRGB_BLOCK`
///
///     `vkGetPhysicalDeviceFormatProperties` and
///     `vkGetPhysicalDeviceImageFormatProperties` can: be used to check for
///     additional supported properties of individual formats.
///
///   - `textureCompressionBC` indicates whether all of the BC compressed texture
///     formats are supported. If this feature is enabled, then the
///     `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
///     `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features must: be
///     supported in `optimalTilingFeatures` for the following formats:
///
///       - `VK_FORMAT_BC1_RGB_UNORM_BLOCK`
///
///       - `VK_FORMAT_BC1_RGB_SRGB_BLOCK`
///
///       - `VK_FORMAT_BC1_RGBA_UNORM_BLOCK`
///
///       - `VK_FORMAT_BC1_RGBA_SRGB_BLOCK`
///
///       - `VK_FORMAT_BC2_UNORM_BLOCK`
///
///       - `VK_FORMAT_BC2_SRGB_BLOCK`
///
///       - `VK_FORMAT_BC3_UNORM_BLOCK`
///
///       - `VK_FORMAT_BC3_SRGB_BLOCK`
///
///       - `VK_FORMAT_BC4_UNORM_BLOCK`
///
///       - `VK_FORMAT_BC4_SNORM_BLOCK`
///
///       - `VK_FORMAT_BC5_UNORM_BLOCK`
///
///       - `VK_FORMAT_BC5_SNORM_BLOCK`
///
///       - `VK_FORMAT_BC6H_UFLOAT_BLOCK`
///
///       - `VK_FORMAT_BC6H_SFLOAT_BLOCK`
///
///       - `VK_FORMAT_BC7_UNORM_BLOCK`
///
///       - `VK_FORMAT_BC7_SRGB_BLOCK`
///
///     `vkGetPhysicalDeviceFormatProperties` and
///     `vkGetPhysicalDeviceImageFormatProperties` can: be used to check for
///     additional supported properties of individual formats.
///
///   - `occlusionQueryPrecise` indicates whether occlusion queries returning actual
///     sample counts are supported. Occlusion queries are created in a
///     `VkQueryPool` by specifying the `queryType` of `VK_QUERY_TYPE_OCCLUSION` in
///     the `VkQueryPoolCreateInfo` structure which is passed to
///     `vkCreateQueryPool`. If this feature is enabled, queries of this type can:
///     enable `VK_QUERY_CONTROL_PRECISE_BIT` in the `flags` parameter to
///     `vkCmdBeginQuery`. If this feature is not supported, the implementation
///     supports only boolean occlusion queries. When any samples are passed,
///     boolean queries will return a non-zero result value, otherwise a result
///     value of zero is returned. When this feature is enabled and
///     `VK_QUERY_CONTROL_PRECISE_BIT` is set, occlusion queries will report the
///     actual number of samples passed.
///
///   - `pipelineStatisticsQuery` indicates whether the pipeline statistics queries
///     are supported. If this feature is not enabled, queries of type
///     `VK_QUERY_TYPE_PIPELINE_STATISTICS` cannot: be created, and none of the
///     `VkQueryPipelineStatisticFlagBits` bits can: be set in the
///     `pipelineStatistics` member of the `VkQueryPoolCreateInfo` structure.
///
///   - `vertexPipelineStoresAndAtomics` indicates whether storage buffers and
///     images support stores and atomic operations in the vertex, tessellation, and
///     geometry shader stages. If this feature is not enabled, all storage image,
///     storage texel buffers, and storage buffer variables used by these stages in
///     shader modules must: be decorated with the `NonWriteable` decoration (or the
///     `readonly` memory qualifier in GLSL).
///
///   - `fragmentStoresAndAtomics` indicates whether storage buffers and images
///     support stores and atomic operations in the fragment shader stage. If this
///     feature is not enabled, all storage image, storage texel buffers, and
///     storage buffer variables used by the fragment stage in shader modules must:
///     be decorated with the `NonWriteable` decoration (or the `readonly` memory
///     qualifier in GLSL).
///
///   - `shaderTessellationAndGeometryPointSize` indicates whether the `PointSize`
///     built-in decoration is available in the tessellation control, tessellation
///     evaluation, and geometry shader stages. If this feature is not enabled,
///     members decorated with the `PointSize` built-in decoration must: not be read
///     from or written to and all points written from a tessellation or geometry
///     shader will have a size of 1.0. This also indicates whether shader modules
///     can: declare the `TessellationPointSize` capability for tessellation control
///     and evaluation shaders, or if the shader modules can: declare the
///     `GeometryPointSize` capability for geometry shaders. An implementation
///     supporting this feature must: also support one or both of the
///     [`tessellationShader`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-tessellationShader) or
///     [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-geometryShader) features.
///
///   - `shaderImageGatherExtended` indicates whether the extended set of image
///     gather instructions are available in shader code. If this feature is not
///     enabled, the `OpImage`\*`Gather` instructions do not support the `Offset`
///     and `ConstOffsets` operands. This also indicates whether shader modules can:
///     declare the `ImageGatherExtended` capability.
///
///   - `shaderStorageImageExtendedFormats` indicates whether the extended storage
///     image formats are available in shader code. If this feature is not enabled,
///     the formats requiring the `StorageImageExtendedFormats` capability are not
///     supported for storage images. This also indicates whether shader modules
///     can: declare the `StorageImageExtendedFormats` capability.
///
///   - `shaderStorageImageMultisample` indicates whether multisampled storage
///     images are supported. If this feature is not enabled, images that are
///     created with a `usage` that includes `VK_IMAGE_USAGE_STORAGE_BIT` must: be
///     created with `samples` equal to `VK_SAMPLE_COUNT_1_BIT`. This also indicates
///     whether shader modules can: declare the `StorageImageMultisample`
///     capability.
///
///   - `shaderStorageImageReadWithoutFormat` indicates whether storage images
///     require a format qualifier to be specified when reading from storage images.
///     If this feature is not enabled, the `OpImageRead` instruction must: not have
///     an `OpTypeImage` of `Unknown`. This also indicates whether shader modules
///     can: declare the `StorageImageReadWithoutFormat` capability.
///
///   - `shaderStorageImageWriteWithoutFormat` indicates whether storage images
///     require a format qualifier to be specified when writing to storage images.
///     If this feature is not enabled, the `OpImageWrite` instruction must: not
///     have an `OpTypeImage` of `Unknown`. This also indicates whether shader
///     modules can: declare the `StorageImageWriteWithoutFormat` capability.
///
///   - `shaderUniformBufferArrayDynamicIndexing` indicates whether arrays of
///     uniform buffers can: be indexed by *dynamically uniform* integer expressions
///     in shader code. If this feature is not enabled, resources with a descriptor
///     type of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` must: be indexed only by
///     constant integral expressions when aggregated into arrays in shader code.
///     This also indicates whether shader modules can: declare the
///     `UniformBufferArrayDynamicIndexing` capability.
///
///   - `shaderSampledImageArrayDynamicIndexing` indicates whether arrays of
///     samplers or sampled images can: be indexed by dynamically uniform integer
///     expressions in shader code. If this feature is not enabled, resources with a
///     descriptor type of `VK_DESCRIPTOR_TYPE_SAMPLER`,
///     `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
///     `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` must: be indexed only by constant
///     integral expressions when aggregated into arrays in shader code. This also
///     indicates whether shader modules can: declare the
///     `SampledImageArrayDynamicIndexing` capability.
///
///   - `shaderStorageBufferArrayDynamicIndexing` indicates whether arrays of
///     storage buffers can: be indexed by dynamically uniform integer expressions
///     in shader code. If this feature is not enabled, resources with a descriptor
///     type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` must: be indexed only by
///     constant integral expressions when aggregated into arrays in shader code.
///     This also indicates whether shader modules can: declare the
///     `StorageBufferArrayDynamicIndexing` capability.
///
///   - `shaderStorageImageArrayDynamicIndexing` indicates whether arrays of storage
///     images can: be indexed by dynamically uniform integer expressions in shader
///     code. If this feature is not enabled, resources with a descriptor type of
///     `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` must: be indexed only by constant
///     integral expressions when aggregated into arrays in shader code. This also
///     indicates whether shader modules can: declare the
///     `StorageImageArrayDynamicIndexing` capability.
///
///   - `shaderClipDistance` indicates whether clip distances are supported in
///     shader code. If this feature is not enabled, any members decorated with the
///     `ClipDistance` built-in decoration must: not be read from or written to in
///     shader modules. This also indicates whether shader modules can: declare the
///     `ClipDistance` capability.
///
///   - `shaderCullDistance` indicates whether cull distances are supported in
///     shader code. If this feature is not enabled, any members decorated with the
///     `CullDistance` built-in decoration must: not be read from or written to in
///     shader modules. This also indicates whether shader modules can: declare the
///     `CullDistance` capability.
///
///   - `shaderFloat64` indicates whether 64-bit floats (doubles) are supported in
///     shader code. If this feature is not enabled, 64-bit floating-point types
///     must: not be used in shader code. This also indicates whether shader modules
///     can: declare the `Float64` capability.
///
///   - `shaderInt64` indicates whether 64-bit integers (signed and unsigned) are
///     supported in shader code. If this feature is not enabled, 64-bit integer
///     types must: not be used in shader code. This also indicates whether shader
///     modules can: declare the `Int64` capability.
///
///   - `shaderInt16` indicates whether 16-bit integers (signed and unsigned) are
///     supported in shader code. If this feature is not enabled, 16-bit integer
///     types must: not be used in shader code. This also indicates whether shader
///     modules can: declare the `Int16` capability.
///
///   - `shaderResourceResidency` indicates whether image operations that return
///     resource residency information are supported in shader code. If this feature
///     is not enabled, the `OpImageSparse`\* instructions must: not be used in
///     shader code. This also indicates whether shader modules can: declare the
///     `SparseResidency` capability. The feature requires at least one of the
///     ptext:sparseResidency\* features to be supported.
///
///   - `shaderResourceMinLod` indicates whether image operations that specify the
///     minimum resource LOD are supported in shader code. If this feature is not
///     enabled, the `MinLod` image operand must: not be used in shader code. This
///     also indicates whether shader modules can: declare the `MinLod` capability.
///
///   - `sparseBinding` indicates whether resource memory can: be managed at opaque
///     sparse block level instead of at the object level. If this feature is not
///     enabled, resource memory must: be bound only on a per-object basis using the
///     `vkBindBufferMemory` and `vkBindImageMemory` commands. In this case, buffers
///     and images must: not be created with `VK_BUFFER_CREATE_SPARSE_BINDING_BIT`
///     and `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` set in the `flags` member of the
///     `VkBufferCreateInfo` and `VkImageCreateInfo` structures, respectively.
///     Otherwise resource memory can: be managed as described in [Sparse Resource
///     Features](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory-sparseresourcefeatures).
///
///   - `sparseResidencyBuffer` indicates whether the device can: access partially
///     resident buffers. If this feature is not enabled, buffers must: not be
///     created with `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags`
///     member of the `VkBufferCreateInfo` structure.
///
///   - `sparseResidencyImage2D` indicates whether the device can: access partially
///     resident 2D images with 1 sample per pixel. If this feature is not enabled,
///     images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples` set to
///     `VK_SAMPLE_COUNT_1_BIT` must: not be created with
///     `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the
///     `VkImageCreateInfo` structure.
///
///   - `sparseResidencyImage3D` indicates whether the device can: access partially
///     resident 3D images. If this feature is not enabled, images with an
///     `imageType` of `VK_IMAGE_TYPE_3D` must: not be created with
///     `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the
///     `VkImageCreateInfo` structure.
///
///   - `sparseResidency2Samples` indicates whether the physical device can: access
///     partially resident 2D images with 2 samples per pixel. If this feature is
///     not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples`
///     set to `VK_SAMPLE_COUNT_2_BIT` must: not be created with
///     `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the
///     `VkImageCreateInfo` structure.
///
///   - `sparseResidency4Samples` indicates whether the physical device can: access
///     partially resident 2D images with 4 samples per pixel. If this feature is
///     not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples`
///     set to `VK_SAMPLE_COUNT_4_BIT` must: not be created with
///     `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the
///     `VkImageCreateInfo` structure.
///
///   - `sparseResidency8Samples` indicates whether the physical device can: access
///     partially resident 2D images with 8 samples per pixel. If this feature is
///     not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples`
///     set to `VK_SAMPLE_COUNT_8_BIT` must: not be created with
///     `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the
///     `VkImageCreateInfo` structure.
///
///   - `sparseResidency16Samples` indicates whether the physical device can: access
///     partially resident 2D images with 16 samples per pixel. If this feature is
///     not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples`
///     set to `VK_SAMPLE_COUNT_16_BIT` must: not be created with
///     `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the
///     `VkImageCreateInfo` structure.
///
///   - `sparseResidencyAliased` indicates whether the physical device can:
///     correctly access data aliased into multiple locations. If this feature is
///     not enabled, the `VK_BUFFER_CREATE_SPARSE_ALIASED_BIT` and
///     `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT` enum values must: not be used in
///     `flags` members of the `VkBufferCreateInfo` and `VkImageCreateInfo`
///     structures, respectively.
///
///   - `variableMultisampleRate` indicates whether all pipelines that will be bound
///     to a command buffer during a subpass with no attachments must: have the same
///     value for `VkPipelineMultisampleStateCreateInfo::rasterizationSamples`. If
///     set to `VK_TRUE`, the implementation supports variable multisample rates in
///     a subpass with no attachments. If set to `VK_FALSE`, then all pipelines
///     bound in such a subpass must: have the same multisample rate. This has no
///     effect in situations where a subpass uses any attachments.
///
///   - `inheritedQueries` indicates whether a secondary command buffer may: be
///     executed while a query is active.
///
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
}
impl Default for VkPhysicalDeviceFeatures {
  fn default() -> VkPhysicalDeviceFeatures {
    VkPhysicalDeviceFeatures::new()
  }
}
impl RawStruct for VkPhysicalDeviceFeatures {
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
///
///   - `linearTilingFeatures` is a bitmask of `VkFormatFeatureFlagBits` specifying
///     features supported by images created with a `tiling` parameter of
///     `VK_IMAGE_TILING_LINEAR`.
///
///   - `optimalTilingFeatures` is a bitmask of `VkFormatFeatureFlagBits` specifying
///     features supported by images created with a `tiling` parameter of
///     `VK_IMAGE_TILING_OPTIMAL`.
///
///   - `bufferFeatures` is a bitmask of `VkFormatFeatureFlagBits` specifying
///     features supported by buffers.
///
/// > **Note**
/// >
/// > If no format feature flags are supported, the format itself is not supported,
/// > and images of that format cannot be created.
///
/// If `format` is a block-compression format, then buffers must: not support any
/// features for the format.
///
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
}
impl Default for VkFormatProperties {
  fn default() -> VkFormatProperties {
    VkFormatProperties::new()
  }
}
impl RawStruct for VkFormatProperties {
  type Raw = types_raw::VkFormatProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_format_properties() {
  assert_size!(types_raw::VkFormatProperties, VkFormatProperties);
}

/// Structure specifying a three-dimensional extent
///
/// A three-dimensional extent is defined by the structure.
///
///   - `width` is the width of the extent.
///
///   - `height` is the height of the extent.
///
///   - `depth` is the depth of the extent.
///
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
}
impl Default for VkExtent3D {
  fn default() -> VkExtent3D {
    VkExtent3D::new()
  }
}
impl RawStruct for VkExtent3D {
  type Raw = types_raw::VkExtent3D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extent3_d() {
  assert_size!(types_raw::VkExtent3D, VkExtent3D);
}

/// Structure specifying a image format properties
///
///   - `maxExtent` are the maximum image dimensions. See the [Allowed Extent
///     Values](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-extentperimagetype) section below for how these values are
///     constrained by `type`.
///
///   - `maxMipLevels` is the maximum number of mipmap levels. `maxMipLevels` must:
///     either be equal to 1 (valid only if `tiling` is `VK_IMAGE_TILING_LINEAR`) or
///     be equal to {lceil}log<sub>2</sub>(max(`width`, `height`, `depth`)){rceil} +
///     1. `width`, `height`, and `depth` are taken from the corresponding members
///     of `maxExtent`.
///
///   - `maxArrayLayers` is the maximum number of array layers. `maxArrayLayers`
///     must: either be equal to 1 or be greater than or equal to the
///     `maxImageArrayLayers` member of `VkPhysicalDeviceLimits`. A value of 1 is
///     valid only if `tiling` is `VK_IMAGE_TILING_LINEAR` or if `type` is
///     `VK_IMAGE_TYPE_3D`.
///
///   - `sampleCounts` is a bitmask of `VkSampleCountFlagBits` specifying all the
///     supported sample counts for this image as described
///     [below](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-supported-sample-counts).
///
///   - `maxResourceSize` is an upper bound on the total image size in bytes,
///     inclusive of all image subresources. Implementations may: have an address
///     space limit on total size of a resource, which is advertised by this
///     property. `maxResourceSize` must: be at least 2<sup>31</sup>.
///
/// > **Note**
/// >
/// > There is no mechanism to query the size of an image before creating it, to
/// > compare that size against `maxResourceSize`. If an application attempts to
/// > create an image that exceeds this limit, the creation will fail or the image
/// > will be invalid. While the advertised limit must: be at least 2<sup>31</sup>,
/// > it may: not be possible to create an image that approaches that size,
/// > particularly for `VK_IMAGE_TYPE_1D`.
///
/// If the combination of parameters to `vkGetPhysicalDeviceImageFormatProperties`
/// is not supported by the implementation for use in `vkCreateImage`, then all
/// members of `VkImageFormatProperties` will be filled with zero.
///
/// > **Note**
/// >
/// > Filling `VkImageFormatProperties` with zero for unsupported formats is an
/// > exception to the usual rule that output structures have undefined contents on
/// > error. This exception was unintentional, but is preserved for backwards
/// > compatibility.
///
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
}
impl Default for VkImageFormatProperties {
  fn default() -> VkImageFormatProperties {
    VkImageFormatProperties::new()
  }
}
impl RawStruct for VkImageFormatProperties {
  type Raw = types_raw::VkImageFormatProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_format_properties() {
  assert_size!(types_raw::VkImageFormatProperties, VkImageFormatProperties);
}

/// Structure reporting implementation-dependent physical device limits
///
///   - `maxImageDimension1D` is the maximum dimension (`width`) supported for all
///     images created with an `imageType` of `VK_IMAGE_TYPE_1D`.
///
///   - `maxImageDimension2D` is the maximum dimension (`width` or `height`)
///     supported for all images created with an `imageType` of `VK_IMAGE_TYPE_2D`
///     and without `VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT` set in `flags`.
///
///   - `maxImageDimension3D` is the maximum dimension (`width`, `height`, or
///     `depth`) supported for all images created with an `imageType` of
///     `VK_IMAGE_TYPE_3D`.
///
///   - `maxImageDimensionCube` is the maximum dimension (`width` or `height`)
///     supported for all images created with an `imageType` of `VK_IMAGE_TYPE_2D`
///     and with `VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT` set in `flags`.
///
///   - `maxImageArrayLayers` is the maximum number of layers (`arrayLayers`) for an
///     image.
///
///   - `maxTexelBufferElements` is the maximum number of addressable texels for a
///     buffer view created on a buffer which was created with the
///     `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT` or
///     `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT` set in the `usage` member of the
///     `VkBufferCreateInfo` structure.
///
///   - `maxUniformBufferRange` is the maximum value that can: be specified in the
///     `range` member of any `VkDescriptorBufferInfo` structures passed to a call
///     to `vkUpdateDescriptorSets` for descriptors of type
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`.
///
///   - `maxStorageBufferRange` is the maximum value that can: be specified in the
///     `range` member of any `VkDescriptorBufferInfo` structures passed to a call
///     to `vkUpdateDescriptorSets` for descriptors of type
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`.
///
///   - `maxPushConstantsSize` is the maximum size, in bytes, of the pool of push
///     constant memory. For each of the push constant ranges indicated by the
///     `pPushConstantRanges` member of the `VkPipelineLayoutCreateInfo` structure,
///     (`offset` + `size`) must: be less than or equal to this limit.
///
///   - `maxMemoryAllocationCount` is the maximum number of device memory
///     allocations, as created by `vkAllocateMemory`, which can: simultaneously
///     exist.
///
///   - `maxSamplerAllocationCount` is the maximum number of sampler objects, as
///     created by `vkCreateSampler`, which can: simultaneously exist on a device.
///
///   - `bufferImageGranularity` is the granularity, in bytes, at which buffer or
///     linear image resources, and optimal image resources can: be bound to
///     adjacent offsets in the same `VkDeviceMemory` object without aliasing. See
///     [Buffer-Image Granularity](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-bufferimagegranularity) for more
///     details.
///
///   - `sparseAddressSpaceSize` is the total amount of address space available, in
///     bytes, for sparse memory resources. This is an upper bound on the sum of the
///     size of all sparse resources, regardless of whether any memory is bound to
///     them.
///
///   - `maxBoundDescriptorSets` is the maximum number of descriptor sets that can:
///     be simultaneously used by a pipeline. All `DescriptorSet` decorations in
///     shader modules must: have a value less than `maxBoundDescriptorSets`. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sets).
///
///   - `maxPerStageDescriptorSamplers` is the maximum number of samplers that can:
///     be accessible to a single shader stage in a pipeline layout. Descriptors
///     with a type of `VK_DESCRIPTOR_TYPE_SAMPLER` or
///     `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` count against this limit. A
///     descriptor is accessible to a shader stage when the `stageFlags` member of
///     the `VkDescriptorSetLayoutBinding` structure has the bit for that shader
///     stage set. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sampler) and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-combinedimagesampler).
///
///   - `maxPerStageDescriptorUniformBuffers` is the maximum number of uniform
///     buffers that can: be accessible to a single shader stage in a pipeline
///     layout. Descriptors with a type of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` count against this limit. A
///     descriptor is accessible to a shader stage when the `stageFlags` member of
///     the `VkDescriptorSetLayoutBinding` structure has the bit for that shader
///     stage set. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformbuffer) and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformbufferdynamic).
///
///   - `maxPerStageDescriptorStorageBuffers` is the maximum number of storage
///     buffers that can: be accessible to a single shader stage in a pipeline
///     layout. Descriptors with a type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` count against this limit. A
///     descriptor is accessible to a pipeline shader stage when the `stageFlags`
///     member of the `VkDescriptorSetLayoutBinding` structure has the bit for that
///     shader stage set. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebuffer) and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebufferdynamic).
///
///   - `maxPerStageDescriptorSampledImages` is the maximum number of sampled images
///     that can: be accessible to a single shader stage in a pipeline layout.
///     Descriptors with a type of `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
///     `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, or
///     `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` count against this limit. A
///     descriptor is accessible to a pipeline shader stage when the `stageFlags`
///     member of the `VkDescriptorSetLayoutBinding` structure has the bit for that
///     shader stage set. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-combinedimagesampler),
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sampledimage), and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer).
///
///   - `maxPerStageDescriptorStorageImages` is the maximum number of storage images
///     that can: be accessible to a single shader stage in a pipeline layout.
///     Descriptors with a type of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or
///     `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` count against this limit. A
///     descriptor is accessible to a pipeline shader stage when the `stageFlags`
///     member of the `VkDescriptorSetLayoutBinding` structure has the bit for that
///     shader stage set. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storageimage), and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer).
///
///   - `maxPerStageDescriptorInputAttachments` is the maximum number of input
///     attachments that can: be accessible to a single shader stage in a pipeline
///     layout. Descriptors with a type of `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`
///     count against this limit. A descriptor is accessible to a pipeline shader
///     stage when the `stageFlags` member of the `VkDescriptorSetLayoutBinding`
///     structure has the bit for that shader stage set. These are only supported
///     for the fragment stage. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-inputattachment).
///
///   - `maxPerStageResources` is the maximum number of resources that can: be
///     accessible to a single shader stage in a pipeline layout. Descriptors with a
///     type of `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
///     `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`,
///     `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`,
///     `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`,
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`, `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`,
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`,
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, or
///     `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` count against this limit. For the
///     fragment shader stage the framebuffer color attachments also count against
///     this limit.
///
///   - `maxDescriptorSetSamplers` is the maximum number of samplers that can: be
///     included in descriptor bindings in a pipeline layout across all pipeline
///     shader stages and descriptor set numbers. Descriptors with a type of
///     `VK_DESCRIPTOR_TYPE_SAMPLER` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
///     count against this limit. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sampler) and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-combinedimagesampler).
///
///   - `maxDescriptorSetUniformBuffers` is the maximum number of uniform buffers
///     that can: be included in descriptor bindings in a pipeline layout across all
///     pipeline shader stages and descriptor set numbers. Descriptors with a type
///     of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` count against this limit. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformbuffer) and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformbufferdynamic).
///
///   - `maxDescriptorSetUniformBuffersDynamic` is the maximum number of dynamic
///     uniform buffers that can: be included in descriptor bindings in a pipeline
///     layout across all pipeline shader stages and descriptor set numbers.
///     Descriptors with a type of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` count
///     against this limit. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformbufferdynamic).
///
///   - `maxDescriptorSetStorageBuffers` is the maximum number of storage buffers
///     that can: be included in descriptor bindings in a pipeline layout across all
///     pipeline shader stages and descriptor set numbers. Descriptors with a type
///     of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` count against this limit. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebuffer) and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebufferdynamic).
///
///   - `maxDescriptorSetStorageBuffersDynamic` is the maximum number of dynamic
///     storage buffers that can: be included in descriptor bindings in a pipeline
///     layout across all pipeline shader stages and descriptor set numbers.
///     Descriptors with a type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` count
///     against this limit. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebufferdynamic).
///
///   - `maxDescriptorSetSampledImages` is the maximum number of sampled images that
///     can: be included in descriptor bindings in a pipeline layout across all
///     pipeline shader stages and descriptor set numbers. Descriptors with a type
///     of `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
///     `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, or
///     `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` count against this limit. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-combinedimagesampler),
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sampledimage), and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer).
///
///   - `maxDescriptorSetStorageImages` is the maximum number of storage images that
///     can: be included in descriptor bindings in a pipeline layout across all
///     pipeline shader stages and descriptor set numbers. Descriptors with a type
///     of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or
///     `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` count against this limit. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storageimage), and
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer).
///
///   - `maxDescriptorSetInputAttachments` is the maximum number of input
///     attachments that can: be included in descriptor bindings in a pipeline
///     layout across all pipeline shader stages and descriptor set numbers.
///     Descriptors with a type of `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` count
///     against this limit. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-inputattachment).
///
///   - `maxVertexInputAttributes` is the maximum number of vertex input attributes
///     that can: be specified for a graphics pipeline. These are described in the
///     array of `VkVertexInputAttributeDescription` structures that are provided at
///     graphics pipeline creation time via the `pVertexAttributeDescriptions`
///     member of the `VkPipelineVertexInputStateCreateInfo` structure. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fxvertex-attrib) and [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fxvertex-input).
///
///   - `maxVertexInputBindings` is the maximum number of vertex buffers that can:
///     be specified for providing vertex attributes to a graphics pipeline. These
///     are described in the array of `VkVertexInputBindingDescription` structures
///     that are provided at graphics pipeline creation time via the
///     `pVertexBindingDescriptions` member of the
///     `VkPipelineVertexInputStateCreateInfo` structure. The `binding` member of
///     `VkVertexInputBindingDescription` must: be less than this limit. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fxvertex-input).
///
///   - `maxVertexInputAttributeOffset` is the maximum vertex input attribute offset
///     that can: be added to the vertex input binding stride. The `offset` member
///     of the `VkVertexInputAttributeDescription` structure must: be less than or
///     equal to this limit. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fxvertex-input).
///
///   - `maxVertexInputBindingStride` is the maximum vertex input binding stride
///     that can: be specified in a vertex input binding. The `stride` member of the
///     `VkVertexInputBindingDescription` structure must: be less than or equal to
///     this limit. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fxvertex-input).
///
///   - `maxVertexOutputComponents` is the maximum number of components of output
///     variables which can: be output by a vertex shader. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#shaders-vertex).
///
///   - `maxTessellationGenerationLevel` is the maximum tessellation generation
///     level supported by the fixed-function tessellation primitive generator. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#tessellation).
///
///   - `maxTessellationPatchSize` is the maximum patch size, in vertices, of
///     patches that can: be processed by the tessellation control shader and
///     tessellation primitive generator. The `patchControlPoints` member of the
///     `VkPipelineTessellationStateCreateInfo` structure specified at pipeline
///     creation time and the value provided in the `OutputVertices` execution mode
///     of shader modules must: be less than or equal to this limit. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#tessellation).
///
///   - `maxTessellationControlPerVertexInputComponents` is the maximum number of
///     components of input variables which can: be provided as per-vertex inputs to
///     the tessellation control shader stage.
///
///   - `maxTessellationControlPerVertexOutputComponents` is the maximum number of
///     components of per-vertex output variables which can: be output from the
///     tessellation control shader stage.
///
///   - `maxTessellationControlPerPatchOutputComponents` is the maximum number of
///     components of per-patch output variables which can: be output from the
///     tessellation control shader stage.
///
///   - `maxTessellationControlTotalOutputComponents` is the maximum total number of
///     components of per-vertex and per-patch output variables which can: be output
///     from the tessellation control shader stage.
///
///   - `maxTessellationEvaluationInputComponents` is the maximum number of
///     components of input variables which can: be provided as per-vertex inputs to
///     the tessellation evaluation shader stage.
///
///   - `maxTessellationEvaluationOutputComponents` is the maximum number of
///     components of per-vertex output variables which can: be output from the
///     tessellation evaluation shader stage.
///
///   - `maxGeometryShaderInvocations` is the maximum invocation count supported for
///     instanced geometry shaders. The value provided in the `Invocations`
///     execution mode of shader modules must: be less than or equal to this limit.
///     See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#geometry).
///
///   - `maxGeometryInputComponents` is the maximum number of components of input
///     variables which can: be provided as inputs to the geometry shader stage.
///
///   - `maxGeometryOutputComponents` is the maximum number of components of output
///     variables which can: be output from the geometry shader stage.
///
///   - `maxGeometryOutputVertices` is the maximum number of vertices which can: be
///     emitted by any geometry shader.
///
///   - `maxGeometryTotalOutputComponents` is the maximum total number of components
///     of output, across all emitted vertices, which can: be output from the
///     geometry shader stage.
///
///   - `maxFragmentInputComponents` is the maximum number of components of input
///     variables which can: be provided as inputs to the fragment shader stage.
///
///   - `maxFragmentOutputAttachments` is the maximum number of output attachments
///     which can: be written to by the fragment shader stage.
///
///   - `maxFragmentDualSrcAttachments` is the maximum number of output attachments
///     which can: be written to by the fragment shader stage when blending is
///     enabled and one of the dual source blend modes is in use. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-dsb) and [dualSrcBlend](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-dualSrcBlend).
///
///   - `maxFragmentCombinedOutputResources` is the total number of storage buffers,
///     storage images, and output buffers which can: be used in the fragment shader
///     stage.
///
///   - `maxComputeSharedMemorySize` is the maximum total storage size, in bytes, of
///     all variables declared with the `WorkgroupLocal` storage class in shader
///     modules (or with the `shared` storage qualifier in GLSL) in the compute
///     shader stage.
///
///   - `maxComputeWorkGroupCount`\[3\] is the maximum number of local workgroups
///     that can: be dispatched by a single dispatch command. These three values
///     represent the maximum number of local workgroups for the X, Y, and Z
///     dimensions, respectively. The workgroup count parameters to the dispatch
///     commands must: be less than or equal to the corresponding limit. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#dispatch).
///
///   - `maxComputeWorkGroupInvocations` is the maximum total number of compute
///     shader invocations in a single local workgroup. The product of the X, Y, and
///     Z sizes as specified by the `LocalSize` execution mode in shader modules and
///     by the object decorated by the `WorkgroupSize` decoration must: be less than
///     or equal to this limit.
///
///   - `maxComputeWorkGroupSize`\[3\] is the maximum size of a local compute
///     workgroup, per dimension. These three values represent the maximum local
///     workgroup size in the X, Y, and Z dimensions, respectively. The `x`, `y`,
///     and `z` sizes specified by the `LocalSize` execution mode and by the object
///     decorated by the `WorkgroupSize` decoration in shader modules must: be less
///     than or equal to the corresponding limit.
///
///   - `subPixelPrecisionBits` is the number of bits of subpixel precision in
///     framebuffer coordinates x<sub>f</sub> and y<sub>f</sub>. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#primsrast).
///
///   - `subTexelPrecisionBits` is the number of bits of precision in the division
///     along an axis of an image used for minification and magnification filters.
///     2<sup>`subTexelPrecisionBits`</sup> is the actual number of divisions along
///     each axis of the image represented. The filtering hardware will snap to
///     these locations when computing the filtered results.
///
///   - `mipmapPrecisionBits` is the number of bits of division that the LOD
///     calculation for mipmap fetching get snapped to when determining the
///     contribution from each mip level to the mip filtered results.
///     2<sup>`mipmapPrecisionBits`</sup> is the actual number of divisions.
///
///     > **Note**
///     >
///     > For example, if this value is 2 bits then when linearly filtering between
///     > two levels, each level could: contribute: 0%, 33%, 66%, or 100% (this is
///     > just an example and the amount of contribution should: be covered by
///     > different equations in the spec).
///
///   - `maxDrawIndexedIndexValue` is the maximum index value that can: be used for
///     indexed draw calls when using 32-bit indices. This excludes the primitive
///     restart index value of 0xFFFFFFFF. See
///     [fullDrawIndexUint32](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-fullDrawIndexUint32).
///
///   - `maxDrawIndirectCount` is the maximum draw count that is supported for
///     indirect draw calls. See
///     [multiDrawIndirect](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-multiDrawIndirect).
///
///   - `maxSamplerLodBias` is the maximum absolute sampler LOD bias. The sum of the
///     `mipLodBias` member of the `VkSamplerCreateInfo` structure and the `Bias`
///     operand of image sampling operations in shader modules (or 0 if no `Bias`
///     operand is provided to an image sampling operation) are clamped to the range
///     \[-`maxSamplerLodBias`,+`maxSamplerLodBias`\]. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-mipLodBias).
///
///   - `maxSamplerAnisotropy` is the maximum degree of sampler anisotropy. The
///     maximum degree of anisotropic filtering used for an image sampling operation
///     is the minimum of the `maxAnisotropy` member of the `VkSamplerCreateInfo`
///     structure and this limit. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-maxAnisotropy).
///
///   - `maxViewports` is the maximum number of active viewports. The
///     `viewportCount` member of the `VkPipelineViewportStateCreateInfo` structure
///     that is provided at pipeline creation must: be less than or equal to this
///     limit.
///
///   - `maxViewportDimensions`\[2\] are the maximum viewport dimensions in the X
///     (width) and Y (height) dimensions, respectively. The maximum viewport
///     dimensions must: be greater than or equal to the largest image which can: be
///     created and used as a framebuffer attachment. See [Controlling the
///     Viewport](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vertexpostproc-viewport).
///
///   - `viewportBoundsRange`\[2\] is the \[minimum, maximum\] range that the
///     corners of a viewport must: be contained in. This range must: be at least
///     \[-2 {times} `size`, 2 {times} `size` - 1\], where `size` =
///     max(`maxViewportDimensions`\[0\], `maxViewportDimensions`\[1\]). See
///     [Controlling the Viewport](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vertexpostproc-viewport).
///
///     > **Note**
///     >
///     > The intent of the `viewportBoundsRange` limit is to allow a maximum sized
///     > viewport to be arbitrarily shifted relative to the output target as long
///     > as at least some portion intersects. This would give a bounds limit of
///     > \[-`size` + 1, 2 {times} `size` - 1\] which would allow all possible
///     > non-empty-set intersections of the output target and the viewport. Since
///     > these numbers are typically powers of two, picking the signed number range
///     > using the smallest possible number of bits ends up with the specified
///     > range.
///
///   - `viewportSubPixelBits` is the number of bits of subpixel precision for
///     viewport bounds. The subpixel precision that floating-point viewport bounds
///     are interpreted at is given by this limit.
///
///   - `minMemoryMapAlignment` is the minimum required: alignment, in bytes, of
///     host visible memory allocations within the host address space. When mapping
///     a memory allocation with `vkMapMemory`, subtracting `offset` bytes from the
///     returned pointer will always produce an integer multiple of this limit. See
///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-device-hostaccess).
///
///   - `minTexelBufferOffsetAlignment` is the minimum required: alignment, in
///     bytes, for the `offset` member of the `VkBufferViewCreateInfo` structure for
///     texel buffers. When a buffer view is created for a buffer which was created
///     with `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT` or
///     `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT` set in the `usage` member of the
///     `VkBufferCreateInfo` structure, the `offset` must: be an integer multiple of
///     this limit.
///
///   - `minUniformBufferOffsetAlignment` is the minimum required: alignment, in
///     bytes, for the `offset` member of the `VkDescriptorBufferInfo` structure for
///     uniform buffers. When a descriptor of type
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` is updated, the `offset` must:
///     be an integer multiple of this limit. Similarly, dynamic offsets for uniform
///     buffers must: be multiples of this limit.
///
///   - `minStorageBufferOffsetAlignment` is the minimum required: alignment, in
///     bytes, for the `offset` member of the `VkDescriptorBufferInfo` structure for
///     storage buffers. When a descriptor of type
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` is updated, the `offset` must:
///     be an integer multiple of this limit. Similarly, dynamic offsets for storage
///     buffers must: be multiples of this limit.
///
///   - `minTexelOffset` is the minimum offset value for the `ConstOffset` image
///     operand of any of the `OpImageSample`\* or `OpImageFetch`\* image
///     instructions.
///
///   - `maxTexelOffset` is the maximum offset value for the `ConstOffset` image
///     operand of any of the `OpImageSample`\* or `OpImageFetch`\* image
///     instructions.
///
///   - `minTexelGatherOffset` is the minimum offset value for the `Offset` or
///     `ConstOffsets` image operands of any of the `OpImage`\*`Gather` image
///     instructions.
///
///   - `maxTexelGatherOffset` is the maximum offset value for the `Offset` or
///     `ConstOffsets` image operands of any of the `OpImage`\*`Gather` image
///     instructions.
///
///   - `minInterpolationOffset` is the minimum negative offset value for the
///     `offset` operand of the `InterpolateAtOffset` extended instruction.
///
///   - `maxInterpolationOffset` is the maximum positive offset value for the
///     `offset` operand of the `InterpolateAtOffset` extended instruction.
///
///   - `subPixelInterpolationOffsetBits` is the number of subpixel fractional bits
///     that the `x` and `y` offsets to the `InterpolateAtOffset` extended
///     instruction may: be rounded to as fixed-point values.
///
///   - `maxFramebufferWidth` is the maximum width for a framebuffer. The `width`
///     member of the `VkFramebufferCreateInfo` structure must: be less than or
///     equal to this limit.
///
///   - `maxFramebufferHeight` is the maximum height for a framebuffer. The `height`
///     member of the `VkFramebufferCreateInfo` structure must: be less than or
///     equal to this limit.
///
///   - `maxFramebufferLayers` is the maximum layer count for a layered framebuffer.
///     The `layers` member of the `VkFramebufferCreateInfo` structure must: be less
///     than or equal to this limit.
///
///   - `framebufferColorSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the color sample counts that are
///     supported for all framebuffer color attachments with floating- or
///     fixed-point formats. There is no limit that indicates the color sample
///     counts that are supported for all color attachments with integer formats.
///
///   - `framebufferDepthSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the supported depth sample counts for all
///     framebuffer depth/stencil attachments, when the format includes a depth
///     component.
///
///   - `framebufferStencilSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the supported stencil sample counts for
///     all framebuffer depth/stencil attachments, when the format includes a
///     stencil component.
///
///   - `framebufferNoAttachmentsSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the supported sample counts for a
///     framebuffer with no attachments.
///
///   - `maxColorAttachments` is the maximum number of color attachments that can:
///     be used by a subpass in a render pass. The `colorAttachmentCount` member of
///     the `VkSubpassDescription` structure must: be less than or equal to this
///     limit.
///
///   - `sampledImageColorSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the sample counts supported for all 2D
///     images created with `VK_IMAGE_TILING_OPTIMAL`, `usage` containing
///     `VK_IMAGE_USAGE_SAMPLED_BIT`, and a non-integer color format.
///
///   - `sampledImageIntegerSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the sample counts supported for all 2D
///     images created with `VK_IMAGE_TILING_OPTIMAL`, `usage` containing
///     `VK_IMAGE_USAGE_SAMPLED_BIT`, and an integer color format.
///
///   - `sampledImageDepthSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the sample counts supported for all 2D
///     images created with `VK_IMAGE_TILING_OPTIMAL`, `usage` containing
///     `VK_IMAGE_USAGE_SAMPLED_BIT`, and a depth format.
///
///   - `sampledImageStencilSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the sample supported for all 2D images
///     created with `VK_IMAGE_TILING_OPTIMAL`, `usage` containing
///     `VK_IMAGE_USAGE_SAMPLED_BIT`, and a stencil format.
///
///   - `storageImageSampleCounts` is a bitmask<sup>1</sup> of
///     `VkSampleCountFlagBits` indicating the sample counts supported for all 2D
///     images created with `VK_IMAGE_TILING_OPTIMAL`, and `usage` containing
///     `VK_IMAGE_USAGE_STORAGE_BIT`.
///
///   - `maxSampleMaskWords` is the maximum number of array elements of a variable
///     decorated with the `SampleMask` built-in decoration.
///
///   - `timestampComputeAndGraphics` indicates support for timestamps on all
///     graphics and compute queues. If this limit is set to `VK_TRUE`, all queues
///     that advertise the `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT` in the
///     `VkQueueFamilyProperties::queueFlags` support
///     `VkQueueFamilyProperties::timestampValidBits` of at least 36. See [Timestamp
///     Queries](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-timestamps).
///
///   - `timestampPeriod` is the number of nanoseconds required: for a timestamp
///     query to be incremented by 1. See [Timestamp Queries](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-timestamps).
///
///   - `maxClipDistances` is the maximum number of clip distances that can: be used
///     in a single shader stage. The size of any array declared with the
///     `ClipDistance` built-in decoration in a shader module must: be less than or
///     equal to this limit.
///
///   - `maxCullDistances` is the maximum number of cull distances that can: be used
///     in a single shader stage. The size of any array declared with the
///     `CullDistance` built-in decoration in a shader module must: be less than or
///     equal to this limit.
///
///   - `maxCombinedClipAndCullDistances` is the maximum combined number of clip and
///     cull distances that can: be used in a single shader stage. The sum of the
///     sizes of any pair of arrays declared with the `ClipDistance` and
///     `CullDistance` built-in decoration used by a single shader stage in a shader
///     module must: be less than or equal to this limit.
///
///   - `discreteQueuePriorities` is the number of discrete priorities that can: be
///     assigned to a queue based on the value of each member of
///     `VkDeviceQueueCreateInfo::pQueuePriorities`. This must: be at least 2, and
///     levels must: be spread evenly over the range, with at least one level at
///     1.0, and another at 0.0. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-priority).
///
///   - `pointSizeRange`\[2\] is the range \[`minimum`,`maximum`\] of supported
///     sizes for points. Values written to variables decorated with the `PointSize`
///     built-in decoration are clamped to this range.
///
///   - `lineWidthRange`\[2\] is the range \[`minimum`,`maximum`\] of supported
///     widths for lines. Values specified by the `lineWidth` member of the
///     `VkPipelineRasterizationStateCreateInfo` or the `lineWidth` parameter to
///     `vkCmdSetLineWidth` are clamped to this range.
///
///   - `pointSizeGranularity` is the granularity of supported point sizes. Not all
///     point sizes in the range defined by `pointSizeRange` are supported. This
///     limit specifies the granularity (or increment) between successive supported
///     point sizes.
///
///   - `lineWidthGranularity` is the granularity of supported line widths. Not all
///     line widths in the range defined by `lineWidthRange` are supported. This
///     limit specifies the granularity (or increment) between successive supported
///     line widths.
///
///   - `strictLines` indicates whether lines are rasterized according to the
///     preferred method of rasterization. If set to `VK_FALSE`, lines may: be
///     rasterized under a relaxed set of rules. If set to `VK_TRUE`, lines are
///     rasterized as per the strict definition. See [Basic Line Segment
///     Rasterization](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#primsrast-lines-basic).
///
///   - `standardSampleLocations` indicates whether rasterization uses the standard
///     sample locations as documented in [Multisampling](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#primsrast-multisampling).
///     If set to `VK_TRUE`, the implementation uses the documented sample
///     locations. If set to `VK_FALSE`, the implementation may: use different
///     sample locations.
///
///   - `optimalBufferCopyOffsetAlignment` is the optimal buffer offset alignment in
///     bytes for `vkCmdCopyBufferToImage` and `vkCmdCopyImageToBuffer`. The per
///     texel alignment requirements are enforced, but applications should: use the
///     optimal alignment for optimal performance and power use.
///
///   - `optimalBufferCopyRowPitchAlignment` is the optimal buffer row pitch
///     alignment in bytes for `vkCmdCopyBufferToImage` and
///     `vkCmdCopyImageToBuffer`. Row pitch is the number of bytes between texels
///     with the same X coordinate in adjacent rows (Y coordinates differ by one).
///     The per texel alignment requirements are enforced, but applications should:
///     use the optimal alignment for optimal performance and power use.
///
///   - `nonCoherentAtomSize` is the size and alignment in bytes that bounds
///     concurrent access to [host-mapped device memory](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-device-hostaccess).
///
///   - `VkPhysicalDeviceDiscardRectanglePropertiesEXT::maxDiscardRectangles` is the
///     maximum number of active discard rectangles. This limit can be queried by
///     setting the `pNext` pointer from a `VkPhysicalDeviceProperties2KHR` object
///     to an instance of `VkPhysicalDeviceDiscardRectanglePropertiesEXT` and using
///     `vkGetPhysicalDeviceProperties2KHR` to fill out the members.
///
///   - `VkPhysicalDevicePointClippingPropertiesKHR::pointClippingBehavior` defines
///     the clipping behavior of points. This limit can be queried by setting the
///     `pNext` pointer from a `VkPhysicalDeviceProperties2KHR` object to an
///     instance of `VkPhysicalDevicePointClippingPropertiesKHR` and using
///     `vkGetPhysicalDeviceProperties2KHR` to fill out the members.
///
/// <!-- end list -->
///
///   - 1
///     For all bitmasks of `VkSampleCountFlagBits`, the sample count limits defined
///     above represent the minimum supported sample counts for each image type.
///     Individual images may: support additional sample counts, which are queried
///     using `vkGetPhysicalDeviceImageFormatProperties` as described in [Supported
///     Sample Counts](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-supported-sample-counts).
///
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
}
impl Default for VkPhysicalDeviceLimits {
  fn default() -> VkPhysicalDeviceLimits {
    VkPhysicalDeviceLimits::new()
  }
}
impl RawStruct for VkPhysicalDeviceLimits {
  type Raw = types_raw::VkPhysicalDeviceLimits;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_physical_device_limits() {
  assert_size!(types_raw::VkPhysicalDeviceLimits, VkPhysicalDeviceLimits);
}

/// Structure specifying physical device sparse memory properties
///
///   - `residencyStandard2DBlockShape` is `VK_TRUE` if the physical device will
///     access all single-sample 2D sparse resources using the standard sparse image
///     block shapes (based on image format), as described in the [Standard Sparse
///     Image Block Shapes (Single Sample)](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory-sparseblockshapessingle)
///     table. If this property is not supported the value returned in the
///     `imageGranularity` member of the `VkSparseImageFormatProperties` structure
///     for single-sample 2D images is not required: to match the standard sparse
///     image block dimensions listed in the table.
///
///   - `residencyStandard2DMultisampleBlockShape` is `VK_TRUE` if the physical
///     device will access all multisample 2D sparse resources using the standard
///     sparse image block shapes (based on image format), as described in the
///     [Standard Sparse Image Block Shapes
///     (MSAA)](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory-sparseblockshapesmsaa) table. If this property is not
///     supported, the value returned in the `imageGranularity` member of the
///     `VkSparseImageFormatProperties` structure for multisample 2D images is not
///     required: to match the standard sparse image block dimensions listed in the
///     table.
///
///   - `residencyStandard3DBlockShape` is `VK_TRUE` if the physical device will
///     access all 3D sparse resources using the standard sparse image block shapes
///     (based on image format), as described in the [Standard Sparse Image Block
///     Shapes (Single Sample)](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory-sparseblockshapessingle) table. If
///     this property is not supported, the value returned in the `imageGranularity`
///     member of the `VkSparseImageFormatProperties` structure for 3D images is not
///     required: to match the standard sparse image block dimensions listed in the
///     table.
///
///   - `residencyAlignedMipSize` is `VK_TRUE` if images with mip level dimensions
///     that are not integer multiples of the corresponding dimensions of the sparse
///     image block may: be placed in the mip tail. If this property is not
///     reported, only mip levels with dimensions smaller than the
///     `imageGranularity` member of the `VkSparseImageFormatProperties` structure
///     will be placed in the mip tail. If this property is reported the
///     implementation is allowed to return
///     `VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT` in the `flags` member of
///     `VkSparseImageFormatProperties`, indicating that mip level dimensions that
///     are not integer multiples of the corresponding dimensions of the sparse
///     image block will be placed in the mip tail.
///
///   - `residencyNonResidentStrict` specifies whether the physical device can:
///     consistently access non-resident regions of a resource. If this property is
///     `VK_TRUE`, access to non-resident regions of resources will be guaranteed to
///     return values as if the resource were populated with 0; writes to
///     non-resident regions will be discarded.
///
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
}
impl Default for VkPhysicalDeviceSparseProperties {
  fn default() -> VkPhysicalDeviceSparseProperties {
    VkPhysicalDeviceSparseProperties::new()
  }
}
impl RawStruct for VkPhysicalDeviceSparseProperties {
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
///
///   - `apiVersion` is the version of Vulkan supported by the device, encoded as
///     described in the [API Version Numbers and
///     Semantics](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-versionnum) section.
///
///   - `driverVersion` is the vendor-specified version of the driver.
///
///   - `vendorID` is a unique identifier for the *vendor* (see below) of the
///     physical device.
///
///   - `deviceID` is a unique identifier for the physical device among devices
///     available from the vendor.
///
///   - `deviceType` is a `VkPhysicalDeviceType` specifying the type of device.
///
///   - `deviceName` is a null-terminated UTF-8 string containing the name of the
///     device.
///
///   - `pipelineCacheUUID` is an array of size `VK_UUID_SIZE`, containing 8-bit
///     values that represent a universally unique identifier for the device.
///
///   - `limits` is the `VkPhysicalDeviceLimits` structure which specifies
///     device-specific limits of the physical device. See
///     [Limits](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits) for details.
///
///   - `sparseProperties` is the `VkPhysicalDeviceSparseProperties` structure which
///     specifies various sparse related properties of the physical device. See
///     [Sparse Properties](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory-physicalprops) for details.
///
/// The `vendorID` and `deviceID` fields are provided to allow applications to adapt
/// to device characteristics that are not adequately exposed by other Vulkan
/// queries. These may: include performance profiles, hardware errata, or other
/// characteristics. In PCI-based implementations, the low sixteen bits of
/// `vendorID` and `deviceID` must: contain (respectively) the PCI vendor and device
/// IDs associated with the hardware device, and the remaining bits must: be set to
/// zero. In non-PCI implementations, the choice of what values to return may: be
/// dictated by operating system or platform policies. It is otherwise at the
/// discretion of the implementer, subject to the following constraints and
/// guidelines:
///
///   - For purposes of physical device identification, the *vendor* of a physical
///     device is the entity responsible for the most salient characteristics of the
///     hardware represented by the physical device handle. In the case of a
///     discrete GPU, this should: be the GPU chipset vendor. In the case of a GPU
///     or other accelerator integrated into a system-on-chip (SoC), this should: be
///     the supplier of the silicon IP used to create the GPU or other accelerator.
///
///   - If the vendor of the physical device has a valid PCI vendor ID issued by
///     [PCI-SIG](#), that ID should: be used to construct `vendorID` as described
///     above for PCI-based implementations. Implementations that do not return a
///     PCI vendor ID in `vendorID` must: return a valid Khronos vendor ID, obtained
///     as described in the [Vulkan Documentation and
///     Extensions](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vulkan-styleguide) document in the section “Registering a
///     Vendor ID with Khronos”. Khronos vendor IDs are allocated starting at
///     0x10000, to distinguish them from the PCI vendor ID namespace.
///
///   - The vendor of the physical device is responsible for selecting `deviceID`.
///     The value selected should: uniquely identify both the device version and any
///     major configuration options (for example, core count in the case of
///     multicore devices). The same device ID should: be used for all physical
///     implementations of that device version and configuration. For example, all
///     uses of a specific silicon IP GPU version and configuration should: use the
///     same device ID, even if those uses occur in different SoCs.
///
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
}
impl Default for VkPhysicalDeviceProperties {
  fn default() -> VkPhysicalDeviceProperties {
    VkPhysicalDeviceProperties::new()
  }
}
impl RawStruct for VkPhysicalDeviceProperties {
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
///
///   - `queueFlags` is a bitmask of `VkQueueFlagBits` indicating capabilities of
///     the queues in this queue family.
///
///   - `queueCount` is the unsigned integer count of queues in this queue family.
///
///   - `timestampValidBits` is the unsigned integer count of meaningful bits in the
///     timestamps written via `vkCmdWriteTimestamp`. The valid range for the count
///     is 36..64 bits, or a value of 0, indicating no support for timestamps. Bits
///     outside the valid range are guaranteed to be zeros.
///
///   - `minImageTransferGranularity` is the minimum granularity supported for image
///     transfer operations on the queues in this queue family.
///
/// The value returned in `minImageTransferGranularity` has a unit of compressed
/// texel blocks for images having a block-compressed format, and a unit of texels
/// otherwise.
///
/// Possible values of `minImageTransferGranularity` are:
///
///   - (0,0,0) which indicates that only whole mip levels must: be transferred
///     using the image transfer operations on the corresponding queues. In this
///     case, the following restrictions apply to all offset and extent parameters
///     of image transfer operations:
///
///       - The `x`, `y`, and `z` members of a `VkOffset3D` parameter must: always
///         be zero.
///
///       - The `width`, `height`, and `depth` members of a `VkExtent3D` parameter
///         must: always match the width, height, and depth of the image subresource
///         corresponding to the parameter, respectively.
///
///   - (A<sub>x</sub>, A<sub>y</sub>, A<sub>z</sub>) where A<sub>x</sub>,
///     A<sub>y</sub>, and A<sub>z</sub> are all integer powers of two. In this case
///     the following restrictions apply to all image transfer operations:
///
///       - `x`, `y`, and `z` of a `VkOffset3D` parameter must: be integer multiples
///         of A<sub>x</sub>, A<sub>y</sub>, and A<sub>z</sub>, respectively.
///
///       - `width` of a `VkExtent3D` parameter must: be an integer multiple of
///         A<sub>x</sub>, or else `x` + `width` must: equal the width of the image
///         subresource corresponding to the parameter.
///
///       - `height` of a `VkExtent3D` parameter must: be an integer multiple of
///         A<sub>y</sub>, or else `y` + `height` must: equal the height of the
///         image subresource corresponding to the parameter.
///
///       - `depth` of a `VkExtent3D` parameter must: be an integer multiple of
///         A<sub>z</sub>, or else `z` + `depth` must: equal the depth of the image
///         subresource corresponding to the parameter.
///
///       - If the format of the image corresponding to the parameters is one of the
///         block-compressed formats then for the purposes of the above calculations
///         the granularity must: be scaled up by the compressed texel block
///         dimensions.
///
/// Queues supporting graphics and/or compute operations must: report (1,1,1) in
/// `minImageTransferGranularity`, meaning that there are no additional restrictions
/// on the granularity of image transfer operations for these queues. Other queues
/// supporting image transfer operations are only required: to support whole mip
/// level transfers, thus `minImageTransferGranularity` for queues belonging to such
/// queue families may: be (0,0,0).
///
/// The [Device Memory](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-device) section describes memory properties queried
/// from the physical device.
///
/// For physical device feature queries see the [Features](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features) chapter.
///
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
}
impl Default for VkQueueFamilyProperties {
  fn default() -> VkQueueFamilyProperties {
    VkQueueFamilyProperties::new()
  }
}
impl RawStruct for VkQueueFamilyProperties {
  type Raw = types_raw::VkQueueFamilyProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_queue_family_properties() {
  assert_size!(types_raw::VkQueueFamilyProperties, VkQueueFamilyProperties);
}

/// Structure specifying memory type
///
///   - `heapIndex` describes which memory heap this memory type corresponds to, and
///     must: be less than `memoryHeapCount` from the
///     `VkPhysicalDeviceMemoryProperties` structure.
///
///   - `propertyFlags` is a bitmask of `VkMemoryPropertyFlagBits` of properties for
///     this memory type.
///
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
}
impl Default for VkMemoryType {
  fn default() -> VkMemoryType {
    VkMemoryType::new()
  }
}
impl RawStruct for VkMemoryType {
  type Raw = types_raw::VkMemoryType;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_type() {
  assert_size!(types_raw::VkMemoryType, VkMemoryType);
}

/// Structure specifying a memory heap
///
///   - `size` is the total memory size in bytes in the heap.
///
///   - `flags` is a bitmask of `VkMemoryHeapFlagBits` specifying attribute flags
///     for the heap.
///
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
}
impl Default for VkMemoryHeap {
  fn default() -> VkMemoryHeap {
    VkMemoryHeap::new()
  }
}
impl RawStruct for VkMemoryHeap {
  type Raw = types_raw::VkMemoryHeap;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_heap() {
  assert_size!(types_raw::VkMemoryHeap, VkMemoryHeap);
}

/// Structure specifying physical device memory properties
///
///   - `memoryTypeCount` is the number of valid elements in the `memoryTypes`
///     array.
///
///   - `memoryTypes` is an array of `VkMemoryType` structures describing the
///     *memory types* that can: be used to access memory allocated from the heaps
///     specified by `memoryHeaps`.
///
///   - `memoryHeapCount` is the number of valid elements in the `memoryHeaps`
///     array.
///
///   - `memoryHeaps` is an array of `VkMemoryHeap` structures describing the
///     *memory heaps* from which memory can: be allocated.
///
/// The `VkPhysicalDeviceMemoryProperties` structure describes a number of *memory
/// heaps* as well as a number of *memory types* that can: be used to access memory
/// allocated in those heaps. Each heap describes a memory resource of a particular
/// size, and each memory type describes a set of memory properties (e.g. host
/// cached vs uncached) that can: be used with a given memory heap. Allocations
/// using a particular memory type will consume resources from the heap indicated by
/// that memory type’s heap index. More than one memory type may: share each heap,
/// and the heaps and memory types provide a mechanism to advertise an accurate size
/// of the physical memory resources while allowing the memory to be used with a
/// variety of different properties.
///
/// The number of memory heaps is given by `memoryHeapCount` and is less than or
/// equal to `VK_MAX_MEMORY_HEAPS`. Each heap is described by an element of the
/// `memoryHeaps` array as a `VkMemoryHeap` structure. The number of memory types
/// available across all memory heaps is given by `memoryTypeCount` and is less than
/// or equal to `VK_MAX_MEMORY_TYPES`. Each memory type is described by an element
/// of the `memoryTypes` array as a `VkMemoryType` structure.
///
/// At least one heap must: include `VK_MEMORY_HEAP_DEVICE_LOCAL_BIT` in
/// `VkMemoryHeap::flags`. If there are multiple heaps that all have similar
/// performance characteristics, they may: all include
/// `VK_MEMORY_HEAP_DEVICE_LOCAL_BIT`. In a unified memory architecture (UMA) system
/// there is often only a single memory heap which is considered to be equally
/// “local” to the host and to the device, and such an implementation must:
/// advertise the heap as device-local.
///
/// Each memory type returned by `vkGetPhysicalDeviceMemoryProperties` must: have
/// its `propertyFlags` set to one of the following values:
///
///   - 0
///
///   - `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
///
///   - `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_CACHED_BIT`
///
///   - `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
///
///   - `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`
///
///   - `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
///
///   - `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_CACHED_BIT`
///
///   - `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` |
///     `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
///
///   - `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` |
///     `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`
///
/// There must: be at least one memory type with both the
/// `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` and `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
/// bits set in its `propertyFlags`. There must: be at least one memory type with
/// the `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` bit set in its `propertyFlags`.
///
/// For each pair of elements **X** and **Y** returned in `memoryTypes`, **X** must:
/// be placed at a lower index position than **Y** if:
///
///   - either the set of bit flags returned in the `propertyFlags` member of **X**
///     is a strict subset of the set of bit flags returned in the `propertyFlags`
///     member of **Y**.
///
///   - or the `propertyFlags` members of **X** and **Y** are equal, and **X**
///     belongs to a memory heap with greater performance (as determined in an
///     implementation-specific manner).
///
/// > **Note**
/// >
/// > There is no ordering requirement between **X** and **Y** elements for the case
/// > their `propertyFlags` members are not in a subset relation. That potentially
/// > allows more than one possible way to order the same set of memory types.
/// > Notice that the [list of all allowed memory property flag
/// > combinations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-device-bitmask-list) is written in the required order.
/// > But if instead `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` was before
/// > `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` |
/// > `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`, the list would still be in the
/// > required order.
///
/// This ordering requirement enables applications to use a simple search loop to
/// select the desired memory type along the lines
/// of:
///
/// ``` c++
/// // Find a memory in `memoryTypeBitsRequirement` that includes all of `requiredProperties`
/// int32_t findProperties(const VkPhysicalDeviceMemoryProperties* pMemoryProperties,
///                        uint32_t memoryTypeBitsRequirement,
///                        VkMemoryPropertyFlags requiredProperties) {
///     const uint32_t memoryCount = pMemoryProperties->memoryTypeCount;
///     for (uint32_t memoryIndex = 0; memoryIndex < memoryCount; ++memoryIndex) {
///         const uint32_t memoryTypeBits = (1 << memoryIndex);
///         const bool isRequiredMemoryType = memoryTypeBitsRequirement & memoryTypeBits;
///
///         const VkMemoryPropertyFlags properties =
///             pMemoryProperties->memoryTypes[memoryIndex].propertyFlags;
///         const bool hasRequiredProperties =
///             (properties & requiredProperties) == requiredProperties;
///
///         if (isRequiredMemoryType && hasRequiredProperties)
///             return static_cast<int32_t>(memoryIndex);
///     }
///
///     // failed to find memory type
///     return -1;
/// }
///
/// // Try to find an optimal memory type, or if it does not exist try fallback memory type
/// // `device` is the VkDevice
/// // `image` is the VkImage that requires memory to be bound
/// // `memoryProperties` properties as returned by vkGetPhysicalDeviceMemoryProperties
/// // `requiredProperties` are the property flags that must be present
/// // `optimalProperties` are the property flags that are preferred by the application
/// VkMemoryRequirements memoryRequirements;
/// vkGetImageMemoryRequirements(device, image, &memoryRequirements);
/// int32_t memoryType =
///     findProperties(&memoryProperties, memoryRequirements.memoryTypeBits, optimalProperties);
/// if (memoryType == -1) // not found; try fallback properties
///     memoryType =
///         findProperties(&memoryProperties, memoryRequirements.memoryTypeBits, requiredProperties);
/// ```
///
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
}
impl Default for VkPhysicalDeviceMemoryProperties {
  fn default() -> VkPhysicalDeviceMemoryProperties {
    VkPhysicalDeviceMemoryProperties::new()
  }
}
impl RawStruct for VkPhysicalDeviceMemoryProperties {
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
///
pub use types_raw::PFN_vkVoidFunction;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkDevice__ {}

/// Opaque handle to a device object
///
/// Logical devices are represented by `VkDevice` handles.
///
pub type VkDevice = VkDispatchableHandle<VkDevice__>;

/// Structure specifying parameters of a newly created device queue
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `queueFamilyIndex` is an unsigned integer indicating the index of the queue
///     family to create on this device. This index corresponds to the index of an
///     element of the `pQueueFamilyProperties` array that was returned by
///     `vkGetPhysicalDeviceQueueFamilyProperties`.
///
///   - `queueCount` is an unsigned integer specifying the number of queues to
///     create in the queue family indicated by `queueFamilyIndex`.
///
///   - `pQueuePriorities` is an array of `queueCount` normalized floating point
///     values, specifying priorities of work that will be submitted to each created
///     queue. See [Queue Priority](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-priority) for more information.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pQueuePriorities = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkDeviceQueueCreateInfo<'a> {
  fn default() -> VkDeviceQueueCreateInfo<'a> {
    VkDeviceQueueCreateInfo::new()
  }
}
impl<'a> RawStruct for VkDeviceQueueCreateInfo<'a> {
  type Raw = types_raw::VkDeviceQueueCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_queue_create_info() {
  assert_size!(types_raw::VkDeviceQueueCreateInfo, VkDeviceQueueCreateInfo);
}

/// Structure specifying parameters of a newly created device
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `queueCreateInfoCount` is the unsigned integer size of the
///     `pQueueCreateInfos` array. Refer to the [Queue
///     Creation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-queue-creation) section below for further details.
///
///   - `pQueueCreateInfos` is a pointer to an array of `VkDeviceQueueCreateInfo`
///     structures describing the queues that are requested to be created along with
///     the logical device. Refer to the [Queue
///     Creation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-queue-creation) section below for further details.
///
///   - `enabledLayerCount` is deprecated and ignored.
///
///   - `ppEnabledLayerNames` is deprecated and ignored. See [Device Layer
///     Deprecation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#extended-functionality-device-layer-deprecation).
///
///   - `enabledExtensionCount` is the number of device extensions to enable.
///
///   - `ppEnabledExtensionNames` is a pointer to an array of
///     `enabledExtensionCount` null-terminated UTF-8 strings containing the names
///     of extensions to enable for the created device. See the
///     [Extensions](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#extended-functionality-extensions) section for further
///     details.
///
///   - `pEnabledFeatures` is `NULL` or a pointer to a `VkPhysicalDeviceFeatures`
///     structure that contains boolean indicators of all the features to be
///     enabled. Refer to the [Features](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features) section for further
///     details.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDeviceCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_queue_create_infos(mut self, value: &'a [VkDeviceQueueCreateInfo<'a>]) -> Self {
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
}
impl<'a> Default for VkDeviceCreateInfo<'a> {
  fn default() -> VkDeviceCreateInfo<'a> {
    VkDeviceCreateInfo::new()
  }
}
impl<'a> RawStruct for VkDeviceCreateInfo<'a> {
  type Raw = types_raw::VkDeviceCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_create_info() {
  assert_size!(types_raw::VkDeviceCreateInfo, VkDeviceCreateInfo);
}

/// Structure specifying a extension properties
///
///   - `extensionName` is a null-terminated string specifying the name of the
///     extension.
///
///   - `specVersion` is the version of this extension. It is an integer,
///     incremented with backward compatible changes.
///
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
}
impl Default for VkExtensionProperties {
  fn default() -> VkExtensionProperties {
    VkExtensionProperties::new()
  }
}
impl RawStruct for VkExtensionProperties {
  type Raw = types_raw::VkExtensionProperties;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extension_properties() {
  assert_size!(types_raw::VkExtensionProperties, VkExtensionProperties);
}

/// Structure specifying layer properties
///
///   - `layerName` is a null-terminated UTF-8 string specifying the name of the
///     layer. Use this name in the `ppEnabledLayerNames` array passed in the
///     `VkInstanceCreateInfo` structure to enable this layer for an instance.
///
///   - `specVersion` is the Vulkan version the layer was written to, encoded as
///     described in the [API Version Numbers and
///     Semantics](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-versionnum) section.
///
///   - `implementationVersion` is the version of this layer. It is an integer,
///     increasing with backward compatible changes.
///
///   - `description` is a null-terminated UTF-8 string providing additional details
///     that can: be used by the application to identify the layer.
///
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
}
impl Default for VkLayerProperties {
  fn default() -> VkLayerProperties {
    VkLayerProperties::new()
  }
}
impl RawStruct for VkLayerProperties {
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
///
/// Creating a logical device also creates the queues associated with that device.
/// The queues to create are described by a set of `VkDeviceQueueCreateInfo`
/// structures that are passed to `vkCreateDevice` in `pQueueCreateInfos`.
///
/// Queues are represented by `VkQueue` handles.
///
pub type VkQueue = VkDispatchableHandle<VkQueue__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkSemaphore__ {}

/// Opaque handle to a semaphore object
///
/// Semaphores are a synchronization primitive that can: be used to insert a
/// dependency between batches submitted to queues. Semaphores have two states -
/// signaled and unsignaled. The state of a semaphore can: be signaled after
/// execution of a batch of commands is completed. A batch can: wait for a semaphore
/// to become signaled before it begins execution, and the semaphore is also
/// unsignaled before the batch begins execution.
///
/// As with most objects in Vulkan, semaphores are an interface to internal data
/// which is typically opaque to applications. This internal data is referred to as
/// a semaphore’s *payload*.
///
/// However, in order to enable communication with agents outside of the current
/// device, it is necessary to be able to export that payload to a commonly
/// understood format, and subsequently import from that format as well.
///
/// The internal data of a semaphore may: include a reference to any resources and
/// pending work associated with signal or unsignal operations performed on that
/// semaphore object. Mechanisms to import and export that internal data to and from
/// semaphores are provided [below](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportSemaphoreCreateInfoKHR). These
/// mechanisms indirectly enable applications to share semaphore state between two
/// or more semaphores and other synchronization primitives across process and API
/// boundaries.
///
/// Semaphores are represented by `VkSemaphore` handles.
///
pub type VkSemaphore = VkNonDispatchableHandle<VkSemaphore__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkCommandBuffer__ {}

/// Opaque handle to a command buffer object
///
/// Command buffers are objects used to record commands which can: be subsequently
/// submitted to a device queue for execution. There are two levels of command
/// buffers - *primary command buffers*, which can: execute secondary command
/// buffers, and which are submitted to queues, and *secondary command buffers*,
/// which can: be executed by primary command buffers, and which are not directly
/// submitted to queues.
///
/// Command buffers are represented by `VkCommandBuffer` handles.
///
pub type VkCommandBuffer = VkDispatchableHandle<VkCommandBuffer__>;

/// Structure specifying a queue submit operation
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `waitSemaphoreCount` is the number of semaphores upon which to wait before
///     executing the command buffers for the batch.
///
///   - `pWaitSemaphores` is a pointer to an array of semaphores upon which to wait
///     before the command buffers for this batch begin execution. If semaphores to
///     wait on are provided, they define a [semaphore wait
///     operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-waiting).
///
///   - `pWaitDstStageMask` is a pointer to an array of pipeline stages at which
///     each corresponding semaphore wait will occur.
///
///   - `commandBufferCount` is the number of command buffers to execute in the
///     batch.
///
///   - `pCommandBuffers` is a pointer to an array of command buffers to execute in
///     the batch.
///
///   - `signalSemaphoreCount` is the number of semaphores to be signaled once the
///     commands specified in `pCommandBuffers` have completed execution.
///
///   - `pSignalSemaphores` is a pointer to an array of semaphores which will be
///     signaled when the command buffers for this batch have completed execution.
///     If semaphores to be signaled are provided, they define a [semaphore signal
///     operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-signaling).
///
/// The order that command buffers appear in `pCommandBuffers` is used to determine
/// [submission order](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-submission-order), and thus all the
/// [implicit ordering guarantees](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-implicit) that respect it. Other
/// than these implicit ordering guarantees and any [explicit synchronization
/// primitives](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization), these command buffers may: overlap or otherwise
/// execute out of order.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_wait_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
    unsafe {
      self.pWaitSemaphores = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_wait_dst_stage_mask(mut self, value: &'a [VkPipelineStageFlags]) -> Self {
    unsafe {
      self.pWaitDstStageMask = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_command_buffers(mut self, value: &'a [VkCommandBuffer]) -> Self {
    unsafe {
      self.pCommandBuffers = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
    unsafe {
      self.pSignalSemaphores = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkSubmitInfo<'a> {
  fn default() -> VkSubmitInfo<'a> {
    VkSubmitInfo::new()
  }
}
impl<'a> RawStruct for VkSubmitInfo<'a> {
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
///
/// Fences are a synchronization primitive that can: be used to insert a dependency
/// from a queue to the host. Fences have two states - signaled and unsignaled. A
/// fence can: be signaled as part of the execution of a [queue
/// submission](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-submission) command. Fences can: be unsignaled on the
/// host with `vkResetFences`. Fences can: be waited on by the host with the
/// `vkWaitForFences` command, and the current state can: be queried with
/// `vkGetFenceStatus`.
///
/// As with most objects in Vulkan, fences are an interface to internal data which
/// is typically opaque to applications. This internal data is referred to as a
/// fence’s *payload*.
///
/// However, in order to enable communication with agents outside of the current
/// device, it is necessary to be able to export that payload to a commonly
/// understood format, and subsequently import from that format as well.
///
/// The internal data of a fence may: include a reference to any resources and
/// pending work associated with signal or unsignal operations performed on that
/// fence object. Mechanisms to import and export that internal data to and from
/// fences are provided [below](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportFenceCreateInfoKHR). These mechanisms
/// indirectly enable applications to share fence state between two or more fences
/// and other synchronization primitives across process and API boundaries.
///
/// Fences are represented by `VkFence` handles.
///
pub type VkFence = VkNonDispatchableHandle<VkFence__>;

/// Structure containing parameters of a memory allocation
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `allocationSize` is the size of the allocation in bytes
///
///   - `memoryTypeIndex` is an index identifying a memory type from the
///     `memoryTypes` array of the `VkPhysicalDeviceMemoryProperties` structure
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkMemoryAllocateInfo {
  fn default() -> VkMemoryAllocateInfo {
    VkMemoryAllocateInfo::new()
  }
}
impl RawStruct for VkMemoryAllocateInfo {
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
///
/// A Vulkan device operates on data in device memory via memory objects that are
/// represented in the API by a `VkDeviceMemory` handle.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkMappedMemoryRange {
  fn default() -> VkMappedMemoryRange {
    VkMappedMemoryRange::new()
  }
}
impl RawStruct for VkMappedMemoryRange {
  type Raw = types_raw::VkMappedMemoryRange;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_mapped_memory_range() {
  assert_size!(types_raw::VkMappedMemoryRange, VkMappedMemoryRange);
}

/// Structure specifying memory requirements
///
///   - `size` is the size, in bytes, of the memory allocation required: for the
///     resource.
///
///   - `alignment` is the alignment, in bytes, of the offset within the allocation
///     required: for the resource.
///
///   - `memoryTypeBits` is a bitmask and contains one bit set for every supported
///     memory type for the resource. Bit `i` is set if and only if the memory type
///     `i` in the `VkPhysicalDeviceMemoryProperties` structure for the physical
///     device is supported for the resource.
///
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
}
impl Default for VkMemoryRequirements {
  fn default() -> VkMemoryRequirements {
    VkMemoryRequirements::new()
  }
}
impl RawStruct for VkMemoryRequirements {
  type Raw = types_raw::VkMemoryRequirements;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_requirements() {
  assert_size!(types_raw::VkMemoryRequirements, VkMemoryRequirements);
}

/// Structure specifying sparse image format properties
///
///   - `aspectMask` is a bitmask `VkImageAspectFlagBits` specifying which aspects
///     of the image the properties apply to.
///
///   - `imageGranularity` is the width, height, and depth of the sparse image block
///     in texels or compressed texel blocks.
///
///   - `flags` is a bitmask of `VkSparseImageFormatFlagBits` specifying additional
///     information about the sparse resource.
///
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
}
impl Default for VkSparseImageFormatProperties {
  fn default() -> VkSparseImageFormatProperties {
    VkSparseImageFormatProperties::new()
  }
}
impl RawStruct for VkSparseImageFormatProperties {
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
///
///   - `formatProperties`.aspectMask is the set of aspects of the image that this
///     sparse memory requirement applies to. This will usually have a single aspect
///     specified. However, depth/stencil images may: have depth and stencil data
///     interleaved in the same sparse block, in which case both
///     `VK_IMAGE_ASPECT_DEPTH_BIT` and `VK_IMAGE_ASPECT_STENCIL_BIT` would be
///     present.
///
///   - `formatProperties`.imageGranularity describes the dimensions of a single
///     bindable sparse image block in texel units. For aspect
///     `VK_IMAGE_ASPECT_METADATA_BIT`, all dimensions will be zero. All metadata is
///     located in the mip tail region.
///
///   - `formatProperties`.flags is a bitmask of `VkSparseImageFormatFlagBits`:
///
///       - If `VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT` is set the image uses a
///         single mip tail region for all array layers.
///
///       - If `VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT` is set the dimensions
///         of mip levels must: be integer multiples of the corresponding dimensions
///         of the sparse image block for levels not located in the mip tail.
///
///       - If `VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT` is set the image
///         uses non-standard sparse image block dimensions. The
///         `formatProperties`.imageGranularity values do not match the standard
///         sparse image block dimension corresponding to the image’s format.
///
///   - `imageMipTailFirstLod` is the first mip level at which image subresources
///     are included in the mip tail region.
///
///   - `imageMipTailSize` is the memory size (in bytes) of the mip tail region. If
///     `formatProperties`.flags contains
///     `VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT`, this is the size of the whole
///     mip tail, otherwise this is the size of the mip tail of a single array
///     layer. This value is guaranteed to be a multiple of the sparse block size in
///     bytes.
///
///   - `imageMipTailOffset` is the opaque memory offset used with
///     `VkSparseImageOpaqueMemoryBindInfo` to bind the mip tail region(s).
///
///   - `imageMipTailStride` is the offset stride between each array-layer’s mip
///     tail, if `formatProperties`.flags does not contain
///     `VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT` (otherwise the value is
///     undefined).
///
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
}
impl Default for VkSparseImageMemoryRequirements {
  fn default() -> VkSparseImageMemoryRequirements {
    VkSparseImageMemoryRequirements::new()
  }
}
impl RawStruct for VkSparseImageMemoryRequirements {
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
///
///   - `resourceOffset` is the offset into the resource.
///
///   - `size` is the size of the memory region to be bound.
///
///   - `memory` is the `VkDeviceMemory` object that the range of the resource is
///     bound to. If `memory` is `VK_NULL_HANDLE`, the range is unbound.
///
///   - `memoryOffset` is the offset into the `VkDeviceMemory` object to bind the
///     resource range to. If `memory` is `VK_NULL_HANDLE`, this value is ignored.
///
///   - `flags` is a bitmask of `VkSparseMemoryBindFlagBits` specifying usage of the
///     binding operation.
///
/// The *binding range* \[`resourceOffset`, `resourceOffset` + `size`) has different
/// constraints based on `flags`. If `flags` contains
/// `VK_SPARSE_MEMORY_BIND_METADATA_BIT`, the binding range must: be within the mip
/// tail region of the metadata aspect. This metadata region is defined by:
///
///   -
///     metadataRegion = \[base, base + `imageMipTailSize`)
///
///   -
///     base = `imageMipTailOffset` + `imageMipTailStride` {times} n
///
/// and `imageMipTailOffset`, `imageMipTailSize`, and `imageMipTailStride` values
/// are from the `VkSparseImageMemoryRequirements` corresponding to the metadata
/// aspect of the image, and n is a valid array layer index for the image,
///
/// `imageMipTailStride` is considered to be zero for aspects where
/// `VkSparseImageMemoryRequirements::formatProperties`.flags contains
/// `VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT`.
///
/// If `flags` does not contain `VK_SPARSE_MEMORY_BIND_METADATA_BIT`, the binding
/// range must: be within the range \[0,`VkMemoryRequirements::size`).
///
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
}
impl Default for VkSparseMemoryBind {
  fn default() -> VkSparseMemoryBind {
    VkSparseMemoryBind::new()
  }
}
impl RawStruct for VkSparseMemoryBind {
  type Raw = types_raw::VkSparseMemoryBind;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_memory_bind() {
  assert_size!(types_raw::VkSparseMemoryBind, VkSparseMemoryBind);
}

/// Structure specifying a sparse buffer memory bind operation
///
/// Memory is bound to `VkBuffer` objects created with the
/// `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` flag using the following structure.
///
///   - `buffer` is the `VkBuffer` object to be bound.
///
///   - `bindCount` is the number of `VkSparseMemoryBind` structures in the `pBinds`
///     array.
///
///   - `pBinds` is a pointer to array of `VkSparseMemoryBind` structures.
///
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
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkSparseBufferMemoryBindInfo<'a> {
  fn default() -> VkSparseBufferMemoryBindInfo<'a> {
    VkSparseBufferMemoryBindInfo::new()
  }
}
impl<'a> RawStruct for VkSparseBufferMemoryBindInfo<'a> {
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
///
/// Memory is bound to opaque regions of `VkImage` objects created with the
/// `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` flag using the following structure.
///
///   - `image` is the `VkImage` object to be bound.
///
///   - `bindCount` is the number of `VkSparseMemoryBind` structures in the `pBinds`
///     array.
///
///   - `pBinds` is a pointer to array of `VkSparseMemoryBind` structures.
///
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
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkSparseImageOpaqueMemoryBindInfo<'a> {
  fn default() -> VkSparseImageOpaqueMemoryBindInfo<'a> {
    VkSparseImageOpaqueMemoryBindInfo::new()
  }
}
impl<'a> RawStruct for VkSparseImageOpaqueMemoryBindInfo<'a> {
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
///
///   - `aspectMask` is a `VkImageAspectFlags` selecting the image *aspect*.
///
///   - `mipLevel` selects the mipmap level.
///
///   - `arrayLayer` selects the array layer.
///
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
}
impl Default for VkImageSubresource {
  fn default() -> VkImageSubresource {
    VkImageSubresource::new()
  }
}
impl RawStruct for VkImageSubresource {
  type Raw = types_raw::VkImageSubresource;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_subresource() {
  assert_size!(types_raw::VkImageSubresource, VkImageSubresource);
}

/// Structure specifying a three-dimensional offset
///
/// A three-dimensional offset is defined by the structure.
///
///   - `x` is the x offset.
///
///   - `y` is the y offset.
///
///   - `z` is the z offset.
///
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
}
impl Default for VkOffset3D {
  fn default() -> VkOffset3D {
    VkOffset3D::new()
  }
}
impl RawStruct for VkOffset3D {
  type Raw = types_raw::VkOffset3D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_offset3_d() {
  assert_size!(types_raw::VkOffset3D, VkOffset3D);
}

/// Structure specifying sparse image memory bind
///
///   - `subresource` is the aspectMask and region of interest in the image.
///
///   - `offset` are the coordinates of the first texel within the image subresource
///     to bind.
///
///   - `extent` is the size in texels of the region within the image subresource to
///     bind. The extent must: be a multiple of the sparse image block dimensions,
///     except when binding sparse image blocks along the edge of an image
///     subresource it can: instead be such that any coordinate of `offset` +
///     `extent` equals the corresponding dimensions of the image subresource.
///
///   - `memory` is the `VkDeviceMemory` object that the sparse image blocks of the
///     image are bound to. If `memory` is `VK_NULL_HANDLE`, the sparse image blocks
///     are unbound.
///
///   - `memoryOffset` is an offset into `VkDeviceMemory` object. If `memory` is
///     `VK_NULL_HANDLE`, this value is ignored.
///
///   - `flags` are sparse memory binding flags.
///
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
}
impl Default for VkSparseImageMemoryBind {
  fn default() -> VkSparseImageMemoryBind {
    VkSparseImageMemoryBind::new()
  }
}
impl RawStruct for VkSparseImageMemoryBind {
  type Raw = types_raw::VkSparseImageMemoryBind;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_sparse_image_memory_bind() {
  assert_size!(types_raw::VkSparseImageMemoryBind, VkSparseImageMemoryBind);
}

/// Structure specifying sparse image memory bind info
///
/// Memory can: be bound to sparse image blocks of `VkImage` objects created with
/// the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag using the following structure.
///
///   - `image` is the `VkImage` object to be bound
///
///   - `bindCount` is the number of `VkSparseImageMemoryBind` structures in pBinds
///     array
///
///   - `pBinds` is a pointer to array of `VkSparseImageMemoryBind` structures
///
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
    unsafe {
      self.pBinds = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkSparseImageMemoryBindInfo<'a> {
  fn default() -> VkSparseImageMemoryBindInfo<'a> {
    VkSparseImageMemoryBindInfo::new()
  }
}
impl<'a> RawStruct for VkSparseImageMemoryBindInfo<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `waitSemaphoreCount` is the number of semaphores upon which to wait before
///     executing the sparse binding operations for the batch.
///
///   - `pWaitSemaphores` is a pointer to an array of semaphores upon which to wait
///     on before the sparse binding operations for this batch begin execution. If
///     semaphores to wait on are provided, they define a [semaphore wait
///     operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-waiting).
///
///   - `bufferBindCount` is the number of sparse buffer bindings to perform in the
///     batch.
///
///   - `pBufferBinds` is a pointer to an array of `VkSparseBufferMemoryBindInfo`
///     structures.
///
///   - `imageOpaqueBindCount` is the number of opaque sparse image bindings to
///     perform.
///
///   - `pImageOpaqueBinds` is a pointer to an array of
///     `VkSparseImageOpaqueMemoryBindInfo` structures, indicating opaque sparse
///     image bindings to perform.
///
///   - `imageBindCount` is the number of sparse image bindings to perform.
///
///   - `pImageBinds` is a pointer to an array of `VkSparseImageMemoryBindInfo`
///     structures, indicating sparse image bindings to perform.
///
///   - `signalSemaphoreCount` is the number of semaphores to be signaled once the
///     sparse binding operations specified by the structure have completed
///     execution.
///
///   - `pSignalSemaphores` is a pointer to an array of semaphores which will be
///     signaled when the sparse binding operations for this batch have completed
///     execution. If semaphores to be signaled are provided, they define a
///     [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-signaling).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_wait_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
    unsafe {
      self.pWaitSemaphores = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_buffer_binds(mut self, value: &'a [VkSparseBufferMemoryBindInfo<'a>]) -> Self {
    unsafe {
      self.pBufferBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_opaque_binds(mut self, value: &'a [VkSparseImageOpaqueMemoryBindInfo<'a>]) -> Self {
    unsafe {
      self.pImageOpaqueBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_binds(mut self, value: &'a [VkSparseImageMemoryBindInfo<'a>]) -> Self {
    unsafe {
      self.pImageBinds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
    unsafe {
      self.pSignalSemaphores = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkBindSparseInfo<'a> {
  fn default() -> VkBindSparseInfo<'a> {
    VkBindSparseInfo::new()
  }
}
impl<'a> RawStruct for VkBindSparseInfo<'a> {
  type Raw = types_raw::VkBindSparseInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_bind_sparse_info() {
  assert_size!(types_raw::VkBindSparseInfo, VkBindSparseInfo);
}

/// Structure specifying parameters of a newly created fence
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkFenceCreateFlagBits` specifying the initial state
///     and behavior of the fence.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkFenceCreateFlags) -> Self {
    self.flags = value;
    self
  }
}
impl Default for VkFenceCreateInfo {
  fn default() -> VkFenceCreateInfo {
    VkFenceCreateInfo::new()
  }
}
impl RawStruct for VkFenceCreateInfo {
  type Raw = types_raw::VkFenceCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_fence_create_info() {
  assert_size!(types_raw::VkFenceCreateInfo, VkFenceCreateInfo);
}

/// Structure specifying parameters of a newly created semaphore
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkSemaphoreCreateFlags) -> Self {
    self.flags = value;
    self
  }
}
impl Default for VkSemaphoreCreateInfo {
  fn default() -> VkSemaphoreCreateInfo {
    VkSemaphoreCreateInfo::new()
  }
}
impl RawStruct for VkSemaphoreCreateInfo {
  type Raw = types_raw::VkSemaphoreCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_semaphore_create_info() {
  assert_size!(types_raw::VkSemaphoreCreateInfo, VkSemaphoreCreateInfo);
}

/// Structure specifying parameters of a newly created event
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkEventCreateFlags) -> Self {
    self.flags = value;
    self
  }
}
impl Default for VkEventCreateInfo {
  fn default() -> VkEventCreateInfo {
    VkEventCreateInfo::new()
  }
}
impl RawStruct for VkEventCreateInfo {
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
///
/// Events are a synchronization primitive that can: be used to insert a
/// fine-grained dependency between commands submitted to the same queue, or between
/// the host and a queue. Events must: not be used to insert a dependency between
/// commands submitted to different queues. Events have two states - signaled and
/// unsignaled. An application can: signal an event, or unsignal it, on either the
/// host or the device. A device can: wait for an event to become signaled before
/// executing further operations. No command exists to wait for an event to become
/// signaled on the host, but the current state of an event can: be queried.
///
/// Events are represented by `VkEvent` handles.
///
pub type VkEvent = VkNonDispatchableHandle<VkEvent__>;

/// Structure specifying parameters of a newly created query pool
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `queryType` is a `VkQueryType` value specifying the type of queries managed
///     by the pool.
///
///   - `queryCount` is the number of queries managed by the pool.
///
///   - `pipelineStatistics` is a bitmask of `VkQueryPipelineStatisticFlagBits`
///     specifying which counters will be returned in queries on the new pool, as
///     described below in [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-pipestats).
///
/// `pipelineStatistics` is ignored if `queryType` is not
/// `VK_QUERY_TYPE_PIPELINE_STATISTICS`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkQueryPoolCreateInfo {
  fn default() -> VkQueryPoolCreateInfo {
    VkQueryPoolCreateInfo::new()
  }
}
impl RawStruct for VkQueryPoolCreateInfo {
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
///
/// Queries are managed using *query pool* objects. Each query pool is a collection
/// of a specific number of queries of a particular type.
///
/// Query pools are represented by `VkQueryPool` handles.
///
pub type VkQueryPool = VkNonDispatchableHandle<VkQueryPool__>;

/// Structure specifying the parameters of a newly created buffer object
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkBufferCreateFlagBits` specifying additional
///     parameters of the buffer.
///
///   - `size` is the size in bytes of the buffer to be created.
///
///   - `usage` is a bitmask of `VkBufferUsageFlagBits` specifying allowed usages of
///     the buffer.
///
///   - `sharingMode` is a `VkSharingMode` value specifying the sharing mode of the
///     buffer when it will be accessed by multiple queue families.
///
///   - `queueFamilyIndexCount` is the number of entries in the
///     `pQueueFamilyIndices` array.
///
///   - `pQueueFamilyIndices` is a list of queue families that will access this
///     buffer (ignored if `sharingMode` is not `VK_SHARING_MODE_CONCURRENT`).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pQueueFamilyIndices = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkBufferCreateInfo<'a> {
  fn default() -> VkBufferCreateInfo<'a> {
    VkBufferCreateInfo::new()
  }
}
impl<'a> RawStruct for VkBufferCreateInfo<'a> {
  type Raw = types_raw::VkBufferCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_create_info() {
  assert_size!(types_raw::VkBufferCreateInfo, VkBufferCreateInfo);
}

/// Structure specifying parameters of a newly created buffer view
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `buffer` is a `VkBuffer` on which the view will be created.
///
///   - `format` is a `VkFormat` describing the format of the data elements in the
///     buffer.
///
///   - `offset` is an offset in bytes from the base address of the buffer. Accesses
///     to the buffer view from shaders use addressing that is relative to this
///     starting offset.
///
///   - `range` is a size in bytes of the buffer view. If `range` is equal to
///     `VK_WHOLE_SIZE`, the range from `offset` to the end of the buffer is used.
///     If `VK_WHOLE_SIZE` is used and the remaining size of the buffer is not a
///     multiple of the element size of `format`, then the nearest smaller multiple
///     is used.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkBufferViewCreateInfo {
  fn default() -> VkBufferViewCreateInfo {
    VkBufferViewCreateInfo::new()
  }
}
impl RawStruct for VkBufferViewCreateInfo {
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
///
/// A *buffer view* represents a contiguous range of a buffer and a specific format
/// to be used to interpret the data. Buffer views are used to enable shaders to
/// access buffer contents interpreted as formatted data. In order to create a valid
/// buffer view, the buffer must: have been created with at least one of the
/// following usage flags:
///
///   - `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT`
///
///   - `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT`
///
/// Buffer views are represented by `VkBufferView` handles.
///
pub type VkBufferView = VkNonDispatchableHandle<VkBufferView__>;

/// Structure specifying the parameters of a newly created image object
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkImageCreateFlagBits` describing additional
///     parameters of the image.
///
///   - `imageType` is a `VkImageType` value specifying the basic dimensionality of
///     the image. Layers in array textures do not count as a dimension for the
///     purposes of the image type.
///
///   - `format` is a `VkFormat` describing the format and type of the data elements
///     that will be contained in the image.
///
///   - `extent` is a `VkExtent3D` describing the number of data elements in each
///     dimension of the base level.
///
///   - `mipLevels` describes the number of levels of detail available for minified
///     sampling of the image.
///
///   - `arrayLayers` is the number of layers in the image.
///
///   - `samples` is the number of sub-data element samples in the image as defined
///     in `VkSampleCountFlagBits`. See [Multisampling](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#primsrast-multisampling).
///
///   - `tiling` is a `VkImageTiling` value specifying the tiling arrangement of the
///     data elements in memory.
///
///   - `usage` is a bitmask of `VkImageUsageFlagBits` describing the intended usage
///     of the image.
///
///   - `sharingMode` is a `VkSharingMode` value specifying the sharing mode of the
///     image when it will be accessed by multiple queue families.
///
///   - `queueFamilyIndexCount` is the number of entries in the
///     `pQueueFamilyIndices` array.
///
///   - `pQueueFamilyIndices` is a list of queue families that will access this
///     image (ignored if `sharingMode` is not `VK_SHARING_MODE_CONCURRENT`).
///
///   - `initialLayout` is a `VkImageLayout` value specifying the initial
///     `VkImageLayout` of all image subresources of the image. See [Image
///     Layouts](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-image-layouts).
///
/// Images created with `tiling` equal to `VK_IMAGE_TILING_LINEAR` have further
/// restrictions on their limits and capabilities compared to images created with
/// `tiling` equal to `VK_IMAGE_TILING_OPTIMAL`. Creation of images with tiling
/// `VK_IMAGE_TILING_LINEAR` may: not be supported unless other parameters meet all
/// of the constraints:
///
///   - `imageType` is `VK_IMAGE_TYPE_2D`
///
///   - `format` is not a depth/stencil format
///
///   - `mipLevels` is 1
///
///   - `arrayLayers` is 1
///
///   - `samples` is `VK_SAMPLE_COUNT_1_BIT`
///
///   - `usage` only includes `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` and/or
///     `VK_IMAGE_USAGE_TRANSFER_DST_BIT`
///
/// Implementations may: support additional limits and capabilities beyond those
/// listed above.
///
/// To query an implementation’s specific capabilities for a given combination of
/// `format`, `imageType`, `tiling`, `usage`,
/// `VkExternalMemoryImageCreateInfoKHR::handleTypes` and `flags`, call
/// `vkGetPhysicalDeviceImageFormatProperties2KHR`. The return value indicates
/// whether that combination of image settings is supported. On success, the
/// `VkImageFormatProperties` output parameter indicates the set of valid `samples`
/// bits and the limits for `extent`, `mipLevels`, and `arrayLayers`.
///
/// To determine the set of valid `usage` bits for a given format, call
/// `vkGetPhysicalDeviceFormatProperties`.
///
/// > **Note**
/// >
/// > For images created without `VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR` a `usage`
/// > bit is valid if it is supported for the format the image is created with.
/// >
/// > For images created with `VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR` a `usage` bit
/// > is valid if it is supported for at least one of the formats a `VkImageView`
/// > created from the image can: have (see [Image Views](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-image-views)
/// > for more detail).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl<'a> Default for VkImageCreateInfo<'a> {
  fn default() -> VkImageCreateInfo<'a> {
    VkImageCreateInfo::new()
  }
}
impl<'a> RawStruct for VkImageCreateInfo<'a> {
  type Raw = types_raw::VkImageCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_create_info() {
  assert_size!(types_raw::VkImageCreateInfo, VkImageCreateInfo);
}

/// Structure specifying subresource layout
///
/// Information about the layout of the image subresource is returned in a
/// `VkSubresourceLayout` structure.
///
///   - `offset` is the byte offset from the start of the image where the image
///     subresource begins.
///
///   - `size` is the size in bytes of the image subresource. `size` includes any
///     extra memory that is required based on `rowPitch`.
///
///   - `rowPitch` describes the number of bytes between each row of texels in an
///     image.
///
///   - `arrayPitch` describes the number of bytes between each array layer of an
///     image.
///
///   - `depthPitch` describes the number of bytes between each slice of 3D image.
///
/// For images created with linear tiling, `rowPitch`, `arrayPitch` and `depthPitch`
/// describe the layout of the image subresource in linear memory. For uncompressed
/// formats, `rowPitch` is the number of bytes between texels with the same x
/// coordinate in adjacent rows (y coordinates differ by one). `arrayPitch` is the
/// number of bytes between texels with the same x and y coordinate in adjacent
/// array layers of the image (array layer values differ by one). `depthPitch` is
/// the number of bytes between texels with the same x and y coordinate in adjacent
/// slices of a 3D image (z coordinates differ by one). Expressed as an addressing
/// formula, the starting byte of a texel in the image subresource has address:
///
/// ``` c
/// // (x,y,z,layer) are in texel coordinates
/// address(x,y,z,layer) = layer*arrayPitch + z*depthPitch + y*rowPitch + x*elementSize + offset
/// ```
///
/// For compressed formats, the `rowPitch` is the number of bytes between compressed
/// texel blocks in adjacent rows. `arrayPitch` is the number of bytes between
/// compressed texel blocks in adjacent array layers. `depthPitch` is the number of
/// bytes between compressed texel blocks in adjacent slices of a 3D image.
///
/// ``` c
/// // (x,y,z,layer) are in compressed texel block coordinates
/// address(x,y,z,layer) = layer*arrayPitch + z*depthPitch + y*rowPitch + x*compressedTexelBlockByteSize + offset;
/// ```
///
/// `arrayPitch` is undefined for images that were not created as arrays.
/// `depthPitch` is defined only for 3D images.
///
/// For *single-plane* color formats, the `aspectMask` member of
/// `VkImageSubresource` must: be `VK_IMAGE_ASPECT_COLOR_BIT`. For depth/stencil
/// formats, `aspectMask` must: be either `VK_IMAGE_ASPECT_DEPTH_BIT` or
/// `VK_IMAGE_ASPECT_STENCIL_BIT`. On implementations that store depth and stencil
/// aspects separately, querying each of these image subresource layouts will return
/// a different `offset` and `size` representing the region of memory used for that
/// aspect. On implementations that store depth and stencil aspects interleaved, the
/// same `offset` and `size` are returned and represent the interleaved memory
/// allocation.
///
/// For [multi-planar
/// formats](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion), the `aspectMask`
/// member of `VkImageSubresource` must: be `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`,
/// `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR`, or (for 3-plane formats only)
/// `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`. Querying each of these image subresource
/// layouts will return a different `offset` and `size` representing the region of
/// memory used for that plane.
///
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
}
impl Default for VkSubresourceLayout {
  fn default() -> VkSubresourceLayout {
    VkSubresourceLayout::new()
  }
}
impl RawStruct for VkSubresourceLayout {
  type Raw = types_raw::VkSubresourceLayout;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subresource_layout() {
  assert_size!(types_raw::VkSubresourceLayout, VkSubresourceLayout);
}

/// Structure specifying a color component mapping
///
///   - `r` is a `VkComponentSwizzle` specifying the component value placed in the R
///     component of the output vector.
///
///   - `g` is a `VkComponentSwizzle` specifying the component value placed in the G
///     component of the output vector.
///
///   - `b` is a `VkComponentSwizzle` specifying the component value placed in the B
///     component of the output vector.
///
///   - `a` is a `VkComponentSwizzle` specifying the component value placed in the A
///     component of the output vector.
///
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
}
impl Default for VkComponentMapping {
  fn default() -> VkComponentMapping {
    VkComponentMapping::new()
  }
}
impl RawStruct for VkComponentMapping {
  type Raw = types_raw::VkComponentMapping;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_component_mapping() {
  assert_size!(types_raw::VkComponentMapping, VkComponentMapping);
}

/// Structure specifying parameters of a newly created image view
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `image` is a `VkImage` on which the view will be created.
///
///   - `viewType` is an `VkImageViewType` value specifying the type of the image
///     view.
///
///   - `format` is a `VkFormat` describing the format and type used to interpret
///     data elements in the image.
///
///   - `components` is a `VkComponentMapping` specifies a remapping of color
///     components (or of depth or stencil components after they have been converted
///     into color components).
///
///   - `subresourceRange` is a `VkImageSubresourceRange` selecting the set of
///     mipmap levels and array layers to be accessible to the view.
///
/// If `image` was created with the `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` flag, and
/// if the `format` of the image is not
/// [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion), `format`
/// can: be different from the image’s format, but if `image` was created without
/// the `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR` flag and they are not
/// equal they must: be *compatible*. Image format compatibility is defined in the
/// [Format Compatibility Classes](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-compatibility-classes) section.
/// Views of compatible formats will have the same mapping between texel coordinates
/// and memory locations irrespective of the `format`, with only the interpretation
/// of the bit pattern changing.
///
/// > **Note**
/// >
/// > Values intended to be used with one view format may: not be exactly preserved
/// > when written or read through a different format. For example, an integer value
/// > that happens to have the bit pattern of a floating point denorm or NaN may: be
/// > flushed or canonicalized when written or read through a view with a floating
/// > point format. Similarly, a value written through a signed normalized format
/// > that has a bit pattern exactly equal to -2<sup>b</sup> may: be changed to
/// > -2<sup>b</sup> + 1 as described in [Conversion from Normalized Fixed-Point to
/// > Floating-Point](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-fixedfpconv).
///
/// If `image` was created with the
/// `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR` flag, `format` must: be
/// *compatible* with the image’s format as described above, or must: be an
/// uncompressed format in which case it must: be size-compatible with the image’s
/// format, meaning that the element size of the uncompressed format must: equal the
/// element size (compressed texel block size) of the image’s format. In this case
/// the resulting image view’s texel dimensions equal the dimensions of the selected
/// mip level divided by the compressed texel block size and rounded up.
///
/// If the image view is to be used with a sampler which supports [sampler
/// Y’C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion), an
/// *identically defined object* of type `VkSamplerYcbcrConversionKHR` to that used
/// to create the sampler must: be passed to `vkCreateImageView` in a
/// `VkSamplerYcbcrConversionInfoKHR` added to the `pNext` chain of
/// `VkImageViewCreateInfo`.
///
/// If the image has a
/// [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion) `format`
/// and `subresourceRange`.aspectMask is `VK_IMAGE_ASPECT_COLOR_BIT`, `format` must:
/// be identical to the image `format`, and the sampler to be used with the image
/// view must: enable [sampler Y’C<sub>B</sub>C<sub>R</sub>
/// conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion).
///
/// If `image` was created with the `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` and the
/// image has a [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion)
/// `format`, and if `subresourceRange`.aspectMask is
/// `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR`, or
/// `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`, `format` must: be
/// [compatible](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-compatible-planes) with the corresponding plane
/// of the image, and the sampler to be used with the image view must: not enable
/// [sampler Y’C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion).
/// The `width` and `height` of the single-plane image view must: be derived from
/// the multi-planar image’s dimensions in the manner listed for [plane
/// compatibility](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-compatible-planes) for the plane.
///
/// Any view of an image plane will have the same mapping between texel coordinates
/// and memory locations as used by the channels of the color aspect, subject to the
/// formulae relating texel coordinates to lower-resolution planes as described in
/// [Chroma Reconstruction](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-chroma-reconstruction). That is, if an R or B
/// plane has a reduced resolution relative to the G plane of the multi-planar
/// image, the image view operates using the (*u<sub>plane</sub>*,
/// *v<sub>plane</sub>*) unnormalized coordinates of the reduced-resolution plane,
/// and these coordinates access the same memory locations as the
/// (*u<sub>color</sub>*, *v<sub>color</sub>*) unnormalized coordinates of the color
/// aspect for which chroma reconstruction operations operate on the same
/// (*u<sub>plane</sub>*, *v<sub>plane</sub>*) or (*i<sub>plane</sub>*,
/// *j<sub>plane</sub>*) coordinates.
///
/// <table>
/// <caption>Image and image view parameter compatibility requirements</caption>
/// <colgroup>
/// <col width="15%" />
/// <col width="35%" />
/// <col width="50%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Dim, Arrayed, MS</th>
/// <th align="left">Image parameters</th>
/// <th align="left">View parameters</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"></td>
/// <td align="left"><p><code>imageType</code> = ci.<code>imageType</code><br />
/// <code>width</code> = ci.<code>extent</code>.width<br />
/// <code>height</code> = ci.<code>extent</code>.height<br />
/// <code>depth</code> = ci.<code>extent</code>.depth<br />
/// <code>arrayLayers</code> = ci.<code>arrayLayers</code><br />
/// <code>samples</code> = ci.<code>samples</code><br />
/// <code>flags</code> = ci.<code>flags</code><br />
/// where ci is the <code>VkImageCreateInfo</code> used to create <code>image</code>.</p></td>
/// <td align="left"><p><code>baseArrayLayer</code>, <code>layerCount</code>, and <code>levelCount</code> are members of the <code>subresourceRange</code> member.</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><strong>1D, 0, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_1D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> = 1<br />
/// <code>depth</code> = 1<br />
/// <code>arrayLayers</code> {geq} 1<br />
/// <code>samples</code> = 1</p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_1D</code><br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> = 1</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p><strong>1D, 1, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_1D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> = 1<br />
/// <code>depth</code> = 1<br />
/// <code>arrayLayers</code> {geq} 1<br />
/// <code>samples</code> = 1</p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_1D_ARRAY</code><br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> {geq} 1</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><strong>2D, 0, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_2D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> {geq} 1<br />
/// <code>depth</code> = 1<br />
/// <code>arrayLayers</code> {geq} 1<br />
/// <code>samples</code> = 1</p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_2D</code><br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> = 1</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p><strong>2D, 1, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_2D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> {geq} 1<br />
/// <code>depth</code> = 1<br />
/// <code>arrayLayers</code> {geq} 1<br />
/// <code>samples</code> = 1</p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_2D_ARRAY</code><br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> {geq} 1</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><strong>2D, 0, 1</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_2D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> {geq} 1<br />
/// <code>depth</code> = 1<br />
/// <code>arrayLayers</code> {geq} 1<br />
/// <code>samples</code> &gt; 1</p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_2D</code><br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> = 1</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p><strong>2D, 1, 1</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_2D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> {geq} 1<br />
/// <code>depth</code> = 1<br />
/// <code>arrayLayers</code> {geq} 1<br />
/// <code>samples</code> &gt; 1</p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_2D_ARRAY</code><br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> {geq} 1</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><strong>CUBE, 0, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_2D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> = <code>width</code><br />
/// <code>depth</code> = 1<br />
/// <code>arrayLayers</code> {geq} 6<br />
/// <code>samples</code> = 1<br />
/// <code>flags</code> includes <code>VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT</code></p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_CUBE</code><br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> = 6</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p><strong>CUBE, 1, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_2D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> = width<br />
/// <code>depth</code> = 1<br />
/// <em>N</em> {geq} 1<br />
/// <code>arrayLayers</code> {geq} 6 {times} <em>N</em><br />
/// <code>samples</code> = 1<br />
/// <code>flags</code> includes <code>VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT</code></p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_CUBE_ARRAY</code><br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> = 6 {times} <em>N</em>, <em>N</em> {geq} 1</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><strong>3D, 0, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_3D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> {geq} 1<br />
/// <code>depth</code> {geq} 1<br />
/// <code>arrayLayers</code> = 1<br />
/// <code>samples</code> = 1</p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_3D</code><br />
/// <code>baseArrayLayer</code> = 0<br />
/// <code>layerCount</code> = 1</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p><strong>3D, 0, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_3D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> {geq} 1<br />
/// <code>depth</code> {geq} 1<br />
/// <code>arrayLayers</code> = 1<br />
/// <code>samples</code> = 1<br />
/// <code>flags</code> includes <code>VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT</code><br />
/// <code>flags</code> does not include <code>VK_IMAGE_CREATE_SPARSE_BINDING_BIT</code>, <code>VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT</code>, and <code>VK_IMAGE_CREATE_SPARSE_ALIASED_BIT</code></p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_2D</code><br />
/// <code>levelCount</code> = 1<br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> = 1</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><strong>3D, 0, 0</strong></p></td>
/// <td align="left"><p><code>imageType</code> = <code>VK_IMAGE_TYPE_3D</code><br />
/// <code>width</code> {geq} 1<br />
/// <code>height</code> {geq} 1<br />
/// <code>depth</code> {geq} 1<br />
/// <code>arrayLayers</code> = 1<br />
/// <code>samples</code> = 1<br />
/// <code>flags</code> includes <code>VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT</code><br />
/// <code>flags</code> does not include <code>VK_IMAGE_CREATE_SPARSE_BINDING_BIT</code>, <code>VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT</code>, and <code>VK_IMAGE_CREATE_SPARSE_ALIASED_BIT</code></p></td>
/// <td align="left"><p><code>viewType</code> = <code>VK_IMAGE_VIEW_TYPE_2D_ARRAY</code><br />
/// <code>levelCount</code> = 1<br />
/// <code>baseArrayLayer</code> {geq} 0<br />
/// <code>layerCount</code> {geq} 1</p></td>
/// </tr>
/// </tbody>
/// </table>
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkImageViewCreateInfo {
  fn default() -> VkImageViewCreateInfo {
    VkImageViewCreateInfo::new()
  }
}
impl RawStruct for VkImageViewCreateInfo {
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
///
/// Image objects are not directly accessed by pipeline shaders for reading or
/// writing image data. Instead, *image views* representing contiguous ranges of the
/// image subresources and containing additional metadata are used for that purpose.
/// Views must: be created on images of compatible types, and must: represent a
/// valid subset of image subresources.
///
/// Image views are represented by `VkImageView` handles.
///
pub type VkImageView = VkNonDispatchableHandle<VkImageView__>;

/// Structure specifying parameters of a newly created shader module
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `codeSize` is the size, in bytes, of the code pointed to by `pCode`.
///
///   - `pCode` points to code that is used to create the shader module. The type
///     and format of the code is determined from the content of the memory
///     addressed by `pCode`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
  pub fn set_code(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pCode = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkShaderModuleCreateInfo<'a> {
  fn default() -> VkShaderModuleCreateInfo<'a> {
    VkShaderModuleCreateInfo::new()
  }
}
impl<'a> RawStruct for VkShaderModuleCreateInfo<'a> {
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
///
/// *Shader modules* contain *shader code* and one or more entry points. Shaders are
/// selected from a shader module by specifying an entry point as part of
/// [pipeline](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines) creation. The stages of a pipeline can: use shaders that
/// come from different modules. The shader code defining a shader module must: be
/// in the SPIR-V format, as described by the [Vulkan Environment for
/// SPIR-V](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#spirvenv) appendix.
///
/// Shader modules are represented by `VkShaderModule` handles.
///
pub type VkShaderModule = VkNonDispatchableHandle<VkShaderModule__>;

/// Structure specifying parameters of a newly created pipeline cache
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `initialDataSize` is the number of bytes in `pInitialData`. If
///     `initialDataSize` is zero, the pipeline cache will initially be empty.
///
///   - `pInitialData` is a pointer to previously retrieved pipeline cache data. If
///     the pipeline cache data is incompatible (as defined below) with the device,
///     the pipeline cache will be initially empty. If `initialDataSize` is zero,
///     `pInitialData` is ignored.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineCacheCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_initial_data(mut self, value: &'a [u8]) -> Self {
    unsafe {
      self.pInitialData = value.as_raw() as *const c_void;
    }
    self
  }
}
impl<'a> Default for VkPipelineCacheCreateInfo<'a> {
  fn default() -> VkPipelineCacheCreateInfo<'a> {
    VkPipelineCacheCreateInfo::new()
  }
}
impl<'a> RawStruct for VkPipelineCacheCreateInfo<'a> {
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
///
/// Pipeline cache objects allow the result of pipeline construction to be reused
/// between pipelines and between runs of an application. Reuse between pipelines is
/// achieved by passing the same pipeline cache object when creating multiple
/// related pipelines. Reuse across runs of an application is achieved by retrieving
/// pipeline cache contents in one run of an application, saving the contents, and
/// using them to preinitialize a pipeline cache on a subsequent run. The contents
/// of the pipeline cache objects are managed by the implementation. Applications
/// can: manage the host memory consumed by a pipeline cache object and control the
/// amount of data retrieved from a pipeline cache object.
///
/// Pipeline cache objects are represented by `VkPipelineCache` handles.
///
pub type VkPipelineCache = VkNonDispatchableHandle<VkPipelineCache__>;

/// Structure specifying a specialization map entry
///
///   - `constantID` is the ID of the specialization constant in SPIR-V.
///
///   - `offset` is the byte offset of the specialization constant value within the
///     supplied data buffer.
///
///   - `size` is the byte size of the specialization constant value within the
///     supplied data buffer.
///
/// If a `constantID` value is not a specialization constant ID used in the shader,
/// that map entry does not affect the behavior of the pipeline.
///
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
}
impl Default for VkSpecializationMapEntry {
  fn default() -> VkSpecializationMapEntry {
    VkSpecializationMapEntry::new()
  }
}
impl RawStruct for VkSpecializationMapEntry {
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
///
///   - `mapEntryCount` is the number of entries in the `pMapEntries` array.
///
///   - `pMapEntries` is a pointer to an array of `VkSpecializationMapEntry` which
///     maps constant IDs to offsets in `pData`.
///
///   - `dataSize` is the byte size of the `pData` buffer.
///
///   - `pData` contains the actual constant values to specialize with.
///
/// `pMapEntries` points to a structure of type `VkSpecializationMapEntry`.
///
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
    unsafe {
      self.pMapEntries = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_data(mut self, value: &'a [u8]) -> Self {
    unsafe {
      self.pData = value.as_raw() as *const c_void;
    }
    self
  }
}
impl<'a> Default for VkSpecializationInfo<'a> {
  fn default() -> VkSpecializationInfo<'a> {
    VkSpecializationInfo::new()
  }
}
impl<'a> RawStruct for VkSpecializationInfo<'a> {
  type Raw = types_raw::VkSpecializationInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_specialization_info() {
  assert_size!(types_raw::VkSpecializationInfo, VkSpecializationInfo);
}

/// Structure specifying parameters of a newly created pipeline shader stage
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `stage` is a `VkShaderStageFlagBits` value specifying a single pipeline
///     stage.
///
///   - `module` is a `VkShaderModule` object that contains the shader for this
///     stage.
///
///   - `pName` is a pointer to a null-terminated UTF-8 string specifying the entry
///     point name of the shader for this stage.
///
///   - `pSpecializationInfo` is a pointer to `VkSpecializationInfo`, as described
///     in [Specialization Constants](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-specialization-constants), and can:
///     be `NULL`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl<'a> Default for VkPipelineShaderStageCreateInfo<'a> {
  fn default() -> VkPipelineShaderStageCreateInfo<'a> {
    VkPipelineShaderStageCreateInfo::new()
  }
}
impl<'a> RawStruct for VkPipelineShaderStageCreateInfo<'a> {
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
///
///   - `binding` is the binding number that this structure describes.
///
///   - `stride` is the distance in bytes between two consecutive elements within
///     the buffer.
///
///   - `inputRate` is a `VkVertexInputRate` value specifying whether vertex
///     attribute addressing is a function of the vertex index or of the instance
///     index.
///
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
}
impl Default for VkVertexInputBindingDescription {
  fn default() -> VkVertexInputBindingDescription {
    VkVertexInputBindingDescription::new()
  }
}
impl RawStruct for VkVertexInputBindingDescription {
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
///
/// Each vertex input attribute is specified by an instance of the
/// `VkVertexInputAttributeDescription` structure.
///
///   - `location` is the shader binding location number for this attribute.
///
///   - `binding` is the binding number which this attribute takes its data from.
///
///   - `format` is the size and type of the vertex attribute data.
///
///   - `offset` is a byte offset of this attribute relative to the start of an
///     element in the vertex input binding.
///
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
}
impl Default for VkVertexInputAttributeDescription {
  fn default() -> VkVertexInputAttributeDescription {
    VkVertexInputAttributeDescription::new()
  }
}
impl RawStruct for VkVertexInputAttributeDescription {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `vertexBindingDescriptionCount` is the number of vertex binding descriptions
///     provided in `pVertexBindingDescriptions`.
///
///   - `pVertexBindingDescriptions` is a pointer to an array of
///     `VkVertexInputBindingDescription` structures.
///
///   - `vertexAttributeDescriptionCount` is the number of vertex attribute
///     descriptions provided in `pVertexAttributeDescriptions`.
///
///   - `pVertexAttributeDescriptions` is a pointer to an array of
///     `VkVertexInputAttributeDescription` structures.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineVertexInputStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_vertex_binding_descriptions(mut self, value: &'a [VkVertexInputBindingDescription]) -> Self {
    unsafe {
      self.pVertexBindingDescriptions = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_vertex_attribute_descriptions(mut self, value: &'a [VkVertexInputAttributeDescription]) -> Self {
    unsafe {
      self.pVertexAttributeDescriptions = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkPipelineVertexInputStateCreateInfo<'a> {
  fn default() -> VkPipelineVertexInputStateCreateInfo<'a> {
    VkPipelineVertexInputStateCreateInfo::new()
  }
}
impl<'a> RawStruct for VkPipelineVertexInputStateCreateInfo<'a> {
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
///
/// Each draw is made up of zero or more vertices and zero or more instances, which
/// are processed by the device and result in the assembly of primitives. Primitives
/// are assembled according to the `pInputAssemblyState` member of the
/// `VkGraphicsPipelineCreateInfo` structure, which is of type
/// `VkPipelineInputAssemblyStateCreateInfo`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `topology` is a `VkPrimitiveTopology` defining the primitive topology, as
///     described below.
///
///   - `primitiveRestartEnable` controls whether a special vertex index value is
///     treated as restarting the assembly of primitives. This enable only applies
///     to indexed draws (`vkCmdDrawIndexed` and `vkCmdDrawIndexedIndirect`), and
///     the special index value is either 0xFFFFFFFF when the `indexType` parameter
///     of `vkCmdBindIndexBuffer` is equal to `VK_INDEX_TYPE_UINT32`, or 0xFFFF when
///     `indexType` is equal to `VK_INDEX_TYPE_UINT16`. Primitive restart is not
///     allowed for “list” topologies.
///
/// Restarting the assembly of primitives discards the most recent index values if
/// those elements formed an incomplete primitive, and restarts the primitive
/// assembly using the subsequent indices, but only assembling the immediately
/// following element through the end of the originally specified elements. The
/// primitive restart index value comparison is performed before adding the
/// `vertexOffset` value to the index value.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkPipelineInputAssemblyStateCreateInfo {
  fn default() -> VkPipelineInputAssemblyStateCreateInfo {
    VkPipelineInputAssemblyStateCreateInfo::new()
  }
}
impl RawStruct for VkPipelineInputAssemblyStateCreateInfo {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `patchControlPoints` number of control points per patch.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkPipelineTessellationStateCreateInfo {
  fn default() -> VkPipelineTessellationStateCreateInfo {
    VkPipelineTessellationStateCreateInfo::new()
  }
}
impl RawStruct for VkPipelineTessellationStateCreateInfo {
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
///
///   - `x` and `y` are the viewport’s upper left corner (x,y).
///
///   - `width` and `height` are the viewport’s width and height, respectively.
///
///   - `minDepth` and `maxDepth` are the depth range for the viewport. It is valid
///     for `minDepth` to be greater than or equal to `maxDepth`.
///
/// The framebuffer depth coordinate `z`<sub>f</sub> may: be represented using
/// either a fixed-point or floating-point representation. However, a floating-point
/// representation must: be used if the depth/stencil attachment has a
/// floating-point depth component. If an m-bit fixed-point representation is used,
/// we assume that it represents each value , where k {elem} { 0, 1, …​,
/// 2<sup>m</sup>-1 }, as k (e.g. 1.0 is represented in binary as a string of all
/// ones).
///
/// The viewport parameters shown in the above equations are found from these values
/// as
///
///   -
///     o<sub>x</sub> = `x` + `width` / 2
///
///   -
///     o<sub>y</sub> = `y` + `height` / 2
///
///   -
///     o<sub>z</sub> = `minDepth`
///
///   -
///     p<sub>x</sub> = `width`
///
///   -
///     p<sub>y</sub> = `height`
///
///   -
///     p<sub>z</sub> = `maxDepth` - `minDepth`.
///
/// The application can: specify a negative term for `height`, which has the effect
/// of negating the y coordinate in clip space before performing the transform. When
/// using a negative `height`, the application should: also adjust the `y` value to
/// point to the lower left corner of the viewport instead of the upper left corner.
/// Using the negative `height` allows the application to avoid having to negate the
/// y component of the `Position` output from the last vertex processing stage in
/// shaders that also target other graphics APIs.
///
/// The width and height of the [implementation-dependent maximum viewport
/// dimensions](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits-maxViewportDimensions) must: be greater than or
/// equal to the width and height of the largest image which can: be created and
/// attached to a framebuffer.
///
/// The floating-point viewport bounds are represented with an
/// [implementation-dependent precision](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits-viewportSubPixelBits).
///
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
}
impl Default for VkViewport {
  fn default() -> VkViewport {
    VkViewport::new()
  }
}
impl RawStruct for VkViewport {
  type Raw = types_raw::VkViewport;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_viewport() {
  assert_size!(types_raw::VkViewport, VkViewport);
}

/// Structure specifying a two-dimensional offset
///
/// A two-dimensional offsets is defined by the structure.
///
///   - `x` is the x offset.
///
///   - `y` is the y offset.
///
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
}
impl Default for VkOffset2D {
  fn default() -> VkOffset2D {
    VkOffset2D::new()
  }
}
impl RawStruct for VkOffset2D {
  type Raw = types_raw::VkOffset2D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_offset2_d() {
  assert_size!(types_raw::VkOffset2D, VkOffset2D);
}

/// Structure specifying a two-dimensional extent
///
/// A two-dimensional extent is defined by the structure.
///
///   - `width` is the width of the extent.
///
///   - `height` is the height of the extent.
///
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
}
impl Default for VkExtent2D {
  fn default() -> VkExtent2D {
    VkExtent2D::new()
  }
}
impl RawStruct for VkExtent2D {
  type Raw = types_raw::VkExtent2D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_extent2_d() {
  assert_size!(types_raw::VkExtent2D, VkExtent2D);
}

/// Structure specifying a two-dimensional subregion
///
/// Rectangles are used to describe a specified rectangular region of pixels within
/// an image or framebuffer. Rectangles include both an offset and an extent of the
/// same dimensionality, as described above. Two-dimensional rectangles are defined
/// by the structure
///
///   - `offset` is a `VkOffset2D` specifying the rectangle offset.
///
///   - `extent` is a `VkExtent2D` specifying the rectangle extent.
///
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
}
impl Default for VkRect2D {
  fn default() -> VkRect2D {
    VkRect2D::new()
  }
}
impl RawStruct for VkRect2D {
  type Raw = types_raw::VkRect2D;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_rect2_d() {
  assert_size!(types_raw::VkRect2D, VkRect2D);
}

/// Structure specifying parameters of a newly created pipeline viewport state
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `viewportCount` is the number of viewports used by the pipeline.
///
///   - `pViewports` is a pointer to an array of `VkViewport` structures, defining
///     the viewport transforms. If the viewport state is dynamic, this member is
///     ignored.
///
///   - `scissorCount` is the number of [scissors](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-scissor) and must: match
///     the number of viewports.
///
///   - `pScissors` is a pointer to an array of `VkRect2D` structures which define
///     the rectangular bounds of the scissor for the corresponding viewport. If the
///     scissor state is dynamic, this member is ignored.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineViewportStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_viewports(mut self, value: &'a [VkViewport]) -> Self {
    unsafe {
      self.pViewports = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_scissors(mut self, value: &'a [VkRect2D]) -> Self {
    unsafe {
      self.pScissors = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkPipelineViewportStateCreateInfo<'a> {
  fn default() -> VkPipelineViewportStateCreateInfo<'a> {
    VkPipelineViewportStateCreateInfo::new()
  }
}
impl<'a> RawStruct for VkPipelineViewportStateCreateInfo<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `depthClampEnable` controls whether to clamp the fragment’s depth values
///     instead of clipping primitives to the z planes of the frustum, as described
///     in [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vertexpostproc-clipping).
///
///   - `rasterizerDiscardEnable` controls whether primitives are discarded
///     immediately before the rasterization stage.
///
///   - `polygonMode` is the triangle rendering mode. See `VkPolygonMode`.
///
///   - `cullMode` is the triangle facing direction used for primitive culling. See
///     `VkCullModeFlagBits`.
///
///   - `frontFace` is a `VkFrontFace` value specifying the front-facing triangle
///     orientation to be used for culling.
///
///   - `depthBiasEnable` controls whether to bias fragment depth values.
///
///   - `depthBiasConstantFactor` is a scalar factor controlling the constant depth
///     value added to each fragment.
///
///   - `depthBiasClamp` is the maximum (or minimum) depth bias of a fragment.
///
///   - `depthBiasSlopeFactor` is a scalar factor applied to a fragment’s slope in
///     depth bias calculations.
///
///   - `lineWidth` is the width of rasterized line segments.
///
/// The application can: also add a
/// `VkPipelineRasterizationStateRasterizationOrderAMD` structure to the `pNext`
/// chain of a `VkPipelineRasterizationStateCreateInfo` structure. This structure
/// enables selecting the rasterization order to use when rendering with the
/// corresponding graphics pipeline as described in [Rasterization
/// Order](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#primrast-order).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkPipelineRasterizationStateCreateInfo {
  fn default() -> VkPipelineRasterizationStateCreateInfo {
    VkPipelineRasterizationStateCreateInfo::new()
  }
}
impl RawStruct for VkPipelineRasterizationStateCreateInfo {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `rasterizationSamples` is a `VkSampleCountFlagBits` specifying the number of
///     samples per pixel used in rasterization.
///
///   - `sampleShadingEnable` can: be used to enable [Sample
///     Shading](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#primsrast-sampleshading).
///
///   - `minSampleShading` specifies a minimum fraction of sample shading if
///     `sampleShadingEnable` is set to `VK_TRUE`.
///
///   - `pSampleMask` is a bitmask of static coverage information that is ANDed with
///     the coverage information generated during rasterization, as described in
///     [Sample Mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-samplemask).
///
///   - `alphaToCoverageEnable` controls whether a temporary coverage value is
///     generated based on the alpha component of the fragment’s first color output
///     as specified in the [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-covg) section.
///
///   - `alphaToOneEnable` controls whether the alpha component of the fragment’s
///     first color output is replaced with one as described in [Multisample
///     Coverage](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-covg).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
  pub fn set_sample_mask(mut self, value: &'a [VkSampleMask]) -> Self {
    unsafe {
      self.pSampleMask = value.as_raw();
    }
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
}
impl<'a> Default for VkPipelineMultisampleStateCreateInfo<'a> {
  fn default() -> VkPipelineMultisampleStateCreateInfo<'a> {
    VkPipelineMultisampleStateCreateInfo::new()
  }
}
impl<'a> RawStruct for VkPipelineMultisampleStateCreateInfo<'a> {
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
///
///   - `failOp` is a `VkStencilOp` value specifying the action performed on samples
///     that fail the stencil test.
///
///   - `passOp` is a `VkStencilOp` value specifying the action performed on samples
///     that pass both the depth and stencil tests.
///
///   - `depthFailOp` is a `VkStencilOp` value specifying the action performed on
///     samples that pass the stencil test and fail the depth test.
///
///   - `compareOp` is a `VkCompareOp` value specifying the comparison operator used
///     in the stencil test.
///
///   - `compareMask` selects the bits of the unsigned integer stencil values
///     participating in the stencil test.
///
///   - `writeMask` selects the bits of the unsigned integer stencil values updated
///     by the stencil test in the stencil framebuffer attachment.
///
///   - `reference` is an integer reference value that is used in the unsigned
///     stencil comparison.
///
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
}
impl Default for VkStencilOpState {
  fn default() -> VkStencilOpState {
    VkStencilOpState::new()
  }
}
impl RawStruct for VkStencilOpState {
  type Raw = types_raw::VkStencilOpState;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_stencil_op_state() {
  assert_size!(types_raw::VkStencilOpState, VkStencilOpState);
}

/// Structure specifying parameters of a newly created pipeline depth stencil state
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `depthTestEnable` controls whether [depth testing](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-depth) is
///     enabled.
///
///   - `depthWriteEnable` controls whether [depth writes](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-depth-write) are
///     enabled when `depthTestEnable` is `VK_TRUE`. Depth writes are always
///     disabled when `depthTestEnable` is `VK_FALSE`.
///
///   - `depthCompareOp` is the comparison operator used in the [depth
///     test](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-depth).
///
///   - `depthBoundsTestEnable` controls whether [depth bounds
///     testing](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-dbt) is enabled.
///
///   - `stencilTestEnable` controls whether [stencil testing](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-stencil) is
///     enabled.
///
///   - `front` and `back` control the parameters of the [stencil
///     test](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-stencil).
///
///   - `minDepthBounds` and `maxDepthBounds` define the range of values used in the
///     [depth bounds test](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-dbt).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkPipelineDepthStencilStateCreateInfo {
  fn default() -> VkPipelineDepthStencilStateCreateInfo {
    VkPipelineDepthStencilStateCreateInfo::new()
  }
}
impl RawStruct for VkPipelineDepthStencilStateCreateInfo {
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
///
///   - `blendEnable` controls whether blending is enabled for the corresponding
///     color attachment. If blending is not enabled, the source fragment’s color
///     for that attachment is passed through unmodified.
///
///   - `srcColorBlendFactor` selects which blend factor is used to determine the
///     source factors (S<sub>r</sub>,S<sub>g</sub>,S<sub>b</sub>).
///
///   - `dstColorBlendFactor` selects which blend factor is used to determine the
///     destination factors (D<sub>r</sub>,D<sub>g</sub>,D<sub>b</sub>).
///
///   - `colorBlendOp` selects which blend operation is used to calculate the RGB
///     values to write to the color attachment.
///
///   - `srcAlphaBlendFactor` selects which blend factor is used to determine the
///     source factor S<sub>a</sub>.
///
///   - `dstAlphaBlendFactor` selects which blend factor is used to determine the
///     destination factor D<sub>a</sub>.
///
///   - `alphaBlendOp` selects which blend operation is use to calculate the alpha
///     values to write to the color attachment.
///
///   - `colorWriteMask` is a bitmask of `VkColorComponentFlagBits` specifying which
///     of the R, G, B, and/or A components are enabled for writing, as described
///     for the [Color Write Mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-color-write-mask).
///
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
}
impl Default for VkPipelineColorBlendAttachmentState {
  fn default() -> VkPipelineColorBlendAttachmentState {
    VkPipelineColorBlendAttachmentState::new()
  }
}
impl RawStruct for VkPipelineColorBlendAttachmentState {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `logicOpEnable` controls whether to apply [Logical
///     Operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-logicop).
///
///   - `logicOp` selects which logical operation to apply.
///
///   - `attachmentCount` is the number of `VkPipelineColorBlendAttachmentState`
///     elements in `pAttachments`. This value must: equal the
///     `colorAttachmentCount` for the subpass in which this pipeline is used.
///
///   - `pAttachments`: is a pointer to array of per target attachment states.
///
///   - `blendConstants` is an array of four values used as the R, G, B, and A
///     components of the blend constant that are used in blending, depending on the
///     [blend factor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blendfactors).
///
/// Each element of the `pAttachments` array is a
/// `VkPipelineColorBlendAttachmentState` structure specifying per-target blending
/// state for each individual color attachment. If the [independent
/// blending](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-independentBlend) feature is not enabled on the
/// device, all `VkPipelineColorBlendAttachmentState` elements in the `pAttachments`
/// array must: be identical.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl<'a> Default for VkPipelineColorBlendStateCreateInfo<'a> {
  fn default() -> VkPipelineColorBlendStateCreateInfo<'a> {
    VkPipelineColorBlendStateCreateInfo::new()
  }
}
impl<'a> RawStruct for VkPipelineColorBlendStateCreateInfo<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `dynamicStateCount` is the number of elements in the `pDynamicStates` array.
///
///   - `pDynamicStates` is an array of `VkDynamicState` values specifying which
///     pieces of pipeline state will use the values from dynamic state commands
///     rather than from pipeline state creation info.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineDynamicStateCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_dynamic_states(mut self, value: &'a [VkDynamicState]) -> Self {
    unsafe {
      self.pDynamicStates = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkPipelineDynamicStateCreateInfo<'a> {
  fn default() -> VkPipelineDynamicStateCreateInfo<'a> {
    VkPipelineDynamicStateCreateInfo::new()
  }
}
impl<'a> RawStruct for VkPipelineDynamicStateCreateInfo<'a> {
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
///
/// Access to descriptor sets from a pipeline is accomplished through a *pipeline
/// layout*. Zero or more descriptor set layouts and zero or more push constant
/// ranges are combined to form a pipeline layout object which describes the
/// complete set of resources that can: be accessed by a pipeline. The pipeline
/// layout represents a sequence of descriptor sets with each having a specific
/// layout. This sequence of layouts is used to determine the interface between
/// shader stages and shader resources. Each pipeline is created using a pipeline
/// layout.
///
/// Pipeline layout objects are represented by `VkPipelineLayout` handles.
///
pub type VkPipelineLayout = VkNonDispatchableHandle<VkPipelineLayout__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkRenderPass__ {}

/// Opaque handle to a render pass object
///
/// A *render pass* represents a collection of attachments, subpasses, and
/// dependencies between the subpasses, and describes how the attachments are used
/// over the course of the subpasses. The use of a render pass in a command buffer
/// is a *render pass instance*.
///
/// Render passes are represented by `VkRenderPass` handles.
///
pub type VkRenderPass = VkNonDispatchableHandle<VkRenderPass__>;
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkPipeline__ {}

/// Opaque handle to a pipeline object
///
/// Compute and graphics pipelines are each represented by `VkPipeline` handles.
///
pub type VkPipeline = VkNonDispatchableHandle<VkPipeline__>;

/// Structure specifying parameters of a newly created graphics pipeline
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkPipelineCreateFlagBits` specifying how the
///     pipeline will be generated.
///
///   - `stageCount` is the number of entries in the `pStages` array.
///
///   - `pStages` is an array of size `stageCount` structures of type
///     `VkPipelineShaderStageCreateInfo` describing the set of the shader stages to
///     be included in the graphics pipeline.
///
///   - `pVertexInputState` is a pointer to an instance of the
///     `VkPipelineVertexInputStateCreateInfo` structure.
///
///   - `pInputAssemblyState` is a pointer to an instance of the
///     `VkPipelineInputAssemblyStateCreateInfo` structure which determines input
///     assembly behavior, as described in [Drawing Commands](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#drawing).
///
///   - `pTessellationState` is a pointer to an instance of the
///     `VkPipelineTessellationStateCreateInfo` structure, and is ignored if the
///     pipeline does not include a tessellation control shader stage and
///     tessellation evaluation shader stage.
///
///   - `pViewportState` is a pointer to an instance of the
///     `VkPipelineViewportStateCreateInfo` structure, and is ignored if the
///     pipeline has rasterization disabled.
///
///   - `pRasterizationState` is a pointer to an instance of the
///     `VkPipelineRasterizationStateCreateInfo` structure.
///
///   - `pMultisampleState` is a pointer to an instance of the
///     `VkPipelineMultisampleStateCreateInfo`, and is ignored if the pipeline has
///     rasterization disabled.
///
///   - `pDepthStencilState` is a pointer to an instance of the
///     `VkPipelineDepthStencilStateCreateInfo` structure, and is ignored if the
///     pipeline has rasterization disabled or if the subpass of the render pass the
///     pipeline is created against does not use a depth/stencil attachment.
///
///   - `pColorBlendState` is a pointer to an instance of the
///     `VkPipelineColorBlendStateCreateInfo` structure, and is ignored if the
///     pipeline has rasterization disabled or if the subpass of the render pass the
///     pipeline is created against does not use any color attachments.
///
///   - `pDynamicState` is a pointer to `VkPipelineDynamicStateCreateInfo` and is
///     used to indicate which properties of the pipeline state object are dynamic
///     and can: be changed independently of the pipeline state. This can: be
///     `NULL`, which means no state in the pipeline is considered dynamic.
///
///   - `layout` is the description of binding locations used by both the pipeline
///     and descriptor sets used with the pipeline.
///
///   - `renderPass` is a handle to a render pass object describing the environment
///     in which the pipeline will be used; the pipeline must: only be used with an
///     instance of any render pass compatible with the one provided. See [Render
///     Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-compatibility) for more information.
///
///   - `subpass` is the index of the subpass in the render pass where this pipeline
///     will be used.
///
///   - `basePipelineHandle` is a pipeline to derive from.
///
///   - `basePipelineIndex` is an index into the `pCreateInfos` parameter to use as
///     a pipeline to derive from.
///
/// The parameters `basePipelineHandle` and `basePipelineIndex` are described in
/// more detail in [Pipeline Derivatives](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-pipeline-derivatives).
///
/// `pStages` points to an array of `VkPipelineShaderStageCreateInfo` structures,
/// which were previously described in [Compute Pipelines](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-compute).
///
/// `pDynamicState` points to a structure of type
/// `VkPipelineDynamicStateCreateInfo`.
///
/// If any shader stage fails to compile, the compile log will be reported back to
/// the application, and `VK_ERROR_INVALID_SHADER_NV` will be generated.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_stages(mut self, value: &'a [VkPipelineShaderStageCreateInfo<'a>]) -> Self {
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
}
impl<'a> Default for VkGraphicsPipelineCreateInfo<'a> {
  fn default() -> VkGraphicsPipelineCreateInfo<'a> {
    VkGraphicsPipelineCreateInfo::new()
  }
}
impl<'a> RawStruct for VkGraphicsPipelineCreateInfo<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkPipelineCreateFlagBits` specifying how the
///     pipeline will be generated.
///
///   - `stage` is a `VkPipelineShaderStageCreateInfo` describing the compute
///     shader.
///
///   - `layout` is the description of binding locations used by both the pipeline
///     and descriptor sets used with the pipeline.
///
///   - `basePipelineHandle` is a pipeline to derive from
///
///   - `basePipelineIndex` is an index into the `pCreateInfos` parameter to use as
///     a pipeline to derive from
///
/// The parameters `basePipelineHandle` and `basePipelineIndex` are described in
/// more detail in [Pipeline Derivatives](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-pipeline-derivatives).
///
/// `stage` points to a structure of type `VkPipelineShaderStageCreateInfo`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl<'a> Default for VkComputePipelineCreateInfo<'a> {
  fn default() -> VkComputePipelineCreateInfo<'a> {
    VkComputePipelineCreateInfo::new()
  }
}
impl<'a> RawStruct for VkComputePipelineCreateInfo<'a> {
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
///
/// A descriptor set layout object is defined by an array of zero or more descriptor
/// bindings. Each individual descriptor binding is specified by a descriptor type,
/// a count (array size) of the number of descriptors in the binding, a set of
/// shader stages that can: access the binding, and (if using immutable samplers) an
/// array of sampler descriptors.
///
/// Descriptor set layout objects are represented by `VkDescriptorSetLayout`
/// handles.
///
pub type VkDescriptorSetLayout = VkNonDispatchableHandle<VkDescriptorSetLayout__>;

/// Structure specifying a push constant range
///
///   - `stageFlags` is a set of stage flags describing the shader stages that will
///     access a range of push constants. If a particular stage is not included in
///     the range, then accessing members of that range of push constants from the
///     corresponding shader stage will result in undefined data being read.
///
///   - `offset` and `size` are the start offset and size, respectively, consumed by
///     the range. Both `offset` and `size` are in units of bytes and must: be a
///     multiple of 4. The layout of the push constant variables is specified in the
///     shader.
///
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
}
impl Default for VkPushConstantRange {
  fn default() -> VkPushConstantRange {
    VkPushConstantRange::new()
  }
}
impl RawStruct for VkPushConstantRange {
  type Raw = types_raw::VkPushConstantRange;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_push_constant_range() {
  assert_size!(types_raw::VkPushConstantRange, VkPushConstantRange);
}

/// Structure specifying the parameters of a newly created pipeline layout object
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `setLayoutCount` is the number of descriptor sets included in the pipeline
///     layout.
///
///   - `pSetLayouts` is a pointer to an array of `VkDescriptorSetLayout` objects.
///
///   - `pushConstantRangeCount` is the number of push constant ranges included in
///     the pipeline layout.
///
///   - `pPushConstantRanges` is a pointer to an array of `VkPushConstantRange`
///     structures defining a set of push constant ranges for use in a single
///     pipeline layout. In addition to descriptor set layouts, a pipeline layout
///     also describes how many push constants can: be accessed by each stage of the
///     pipeline.
///
///     > **Note**
///     >
///     > Push constants represent a high speed path to modify constant data in
///     > pipelines that is expected to outperform memory-backed resource updates.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineLayoutCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_set_layouts(mut self, value: &'a [VkDescriptorSetLayout]) -> Self {
    unsafe {
      self.pSetLayouts = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_push_constant_ranges(mut self, value: &'a [VkPushConstantRange]) -> Self {
    unsafe {
      self.pPushConstantRanges = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkPipelineLayoutCreateInfo<'a> {
  fn default() -> VkPipelineLayoutCreateInfo<'a> {
    VkPipelineLayoutCreateInfo::new()
  }
}
impl<'a> RawStruct for VkPipelineLayoutCreateInfo<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `magFilter` is a `VkFilter` value specifying the magnification filter to
///     apply to lookups.
///
///   - `minFilter` is a `VkFilter` value specifying the minification filter to
///     apply to lookups.
///
///   - `mipmapMode` is a `VkSamplerMipmapMode` value specifying the mipmap filter
///     to apply to lookups.
///
///   - `addressModeU` is a `VkSamplerAddressMode` value specifying the addressing
///     mode for outside \[0..1\] range for U coordinate.
///
///   - `addressModeV` is a `VkSamplerAddressMode` value specifying the addressing
///     mode for outside \[0..1\] range for V coordinate.
///
///   - `addressModeW` is a `VkSamplerAddressMode` value specifying the addressing
///     mode for outside \[0..1\] range for W coordinate.
///
///   - `mipLodBias` is the bias to be added to mipmap LOD (level-of-detail)
///     calculation and bias provided by image sampling functions in SPIR-V, as
///     described in the [Level-of-Detail
///     Operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-level-of-detail-operation) section.
///
///   - `anisotropyEnable` is `VK_TRUE` to enable anisotropic filtering, as
///     described in the [Texel Anisotropic
///     Filtering](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-texel-anisotropic-filtering) section, or `VK_FALSE`
///     otherwise.
///
///   - `maxAnisotropy` is the anisotropy value clamp used by the sampler when
///     `anisotropyEnable` is `VK_TRUE`. If `anisotropyEnable` is `VK_FALSE`,
///     `maxAnisotropy` is ignored.
///
///   - `compareEnable` is `VK_TRUE` to enable comparison against a reference value
///     during lookups, or `VK_FALSE` otherwise.
///
///       - Note: Some implementations will default to shader state if this member
///         does not match.
///
///   - `compareOp` is a `VkCompareOp` value specifying the comparison function to
///     apply to fetched data before filtering as described in the [Depth Compare
///     Operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-depth-compare-operation) section.
///
///   - `minLod` and `maxLod` are the values used to clamp the computed LOD value,
///     as described in the [Level-of-Detail
///     Operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-level-of-detail-operation) section. `maxLod` must: be
///     greater than or equal to `minLod`.
///
///   - `borderColor` is a `VkBorderColor` value specifying the predefined border
///     color to use.
///
///   - `unnormalizedCoordinates` controls whether to use unnormalized or normalized
///     texel coordinates to address texels of the image. When set to `VK_TRUE`, the
///     range of the image coordinates used to lookup the texel is in the range of
///     zero to the image dimensions for x, y and z. When set to `VK_FALSE` the
///     range of image coordinates is zero to one. When `unnormalizedCoordinates` is
///     `VK_TRUE`, samplers have the following requirements:
///
///       - `minFilter` and `magFilter` must: be equal.
///
///       - `mipmapMode` must: be `VK_SAMPLER_MIPMAP_MODE_NEAREST`.
///
///       - `minLod` and `maxLod` must: be zero.
///
///       - `addressModeU` and `addressModeV` must: each be either
///         `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE` or
///         `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER`.
///
///       - `anisotropyEnable` must: be `VK_FALSE`.
///
///       - `compareEnable` must: be `VK_FALSE`.
///
///       - The sampler must: not enable sampler Y’C<sub>B</sub>C<sub>R</sub>
///         conversion.
///
///   - When `unnormalizedCoordinates` is `VK_TRUE`, images the sampler is used with
///     in the shader have the following requirements:
///
///       - The `viewType` must: be either `VK_IMAGE_VIEW_TYPE_1D` or
///         `VK_IMAGE_VIEW_TYPE_2D`.
///
///       - The image view must: have a single layer and a single mip level.
///
///   - When `unnormalizedCoordinates` is `VK_TRUE`, image built-in functions in the
///     shader that use the sampler have the following requirements:
///
///       - The functions must: not use projection.
///
///       - The functions must: not use offsets.
///
/// > **Note**
/// >
/// > `magFilter` values of `VK_FILTER_NEAREST` and `VK_FILTER_LINEAR` directly
/// > correspond to `GL_NEAREST` and `GL_LINEAR` magnification filters. `minFilter`
/// > and `mipmapMode` combine to correspond to the similarly named OpenGL
/// > minification filter of `GL_minFilter_MIPMAP_mipmapMode` (e.g. `minFilter` of
/// > `VK_FILTER_LINEAR` and `mipmapMode` of `VK_SAMPLER_MIPMAP_MODE_NEAREST`
/// > correspond to `GL_LINEAR_MIPMAP_NEAREST`).
/// >
/// > There are no Vulkan filter modes that directly correspond to OpenGL
/// > minification filters of `GL_LINEAR` or `GL_NEAREST`, but they can: be emulated
/// > using `VK_SAMPLER_MIPMAP_MODE_NEAREST`, `minLod` = 0, and `maxLod` = 0.25, and
/// > using `minFilter` = `VK_FILTER_LINEAR` or `minFilter` = `VK_FILTER_NEAREST`,
/// > respectively.
/// >
/// > Note that using a `maxLod` of zero would cause
/// > [magnification](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-texel-filtering) to always be performed, and the
/// > `magFilter` to always be used. This is valid, just not an exact match for
/// > OpenGL behavior. Clamping the maximum LOD to 0.25 allows the {lambda} value to
/// > be non-zero and minification to be performed, while still always rounding down
/// > to the base level. If the `minFilter` and `magFilter` are equal, then using a
/// > `maxLod` of zero also works.
///
/// The maximum number of sampler objects which can: be simultaneously created on a
/// device is implementation-dependent and specified by the
/// [maxSamplerAllocationCount](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits-maxSamplerAllocationCount) member
/// of the `VkPhysicalDeviceLimits` structure. If `maxSamplerAllocationCount` is
/// exceeded, `vkCreateSampler` will return `VK_ERROR_TOO_MANY_OBJECTS`.
///
/// Since `VkSampler` is a non-dispatchable handle type, implementations may: return
/// the same handle for sampler state vectors that are identical. In such cases, all
/// such objects would only count once against the `maxSamplerAllocationCount`
/// limit.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkSamplerCreateInfo {
  fn default() -> VkSamplerCreateInfo {
    VkSamplerCreateInfo::new()
  }
}
impl RawStruct for VkSamplerCreateInfo {
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
///
/// `VkSampler` objects represent the state of an image sampler which is used by the
/// implementation to read image data and apply filtering and other transformations
/// for the shader.
///
/// Samplers are represented by `VkSampler` handles.
///
pub type VkSampler = VkNonDispatchableHandle<VkSampler__>;

/// Structure specifying a descriptor set layout binding
///
///   - `binding` is the binding number of this entry and corresponds to a resource
///     of the same binding number in the shader stages.
///
///   - `descriptorType` is a `VkDescriptorType` specifying which type of resource
///     descriptors are used for this binding.
///
///   - `descriptorCount` is the number of descriptors contained in the binding,
///     accessed in a shader as an array. If `descriptorCount` is zero this binding
///     entry is reserved and the resource must: not be accessed from any stage via
///     this binding within any pipeline using the set layout.
///
///   - `stageFlags` member is a bitmask of `VkShaderStageFlagBits` specifying which
///     pipeline shader stages can: access a resource for this binding.
///     `VK_SHADER_STAGE_ALL` is a shorthand specifying that all defined shader
///     stages, including any additional stages defined by extensions, can: access
///     the resource.
///
///     If a shader stage is not included in `stageFlags`, then a resource must: not
///     be accessed from that stage via this binding within any pipeline using the
///     set layout. Other than input attachments which are limited to the fragment
///     shader, there are no limitations on what combinations of stages can: be used
///     by a descriptor binding, and in particular a binding can: be used by both
///     graphics stages and the compute stage.
///
///   - `pImmutableSamplers` affects initialization of samplers. If `descriptorType`
///     specifies a `VK_DESCRIPTOR_TYPE_SAMPLER` or
///     `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` type descriptor, then
///     `pImmutableSamplers` can: be used to initialize a set of *immutable
///     samplers*. Immutable samplers are permanently bound into the set layout;
///     later binding a sampler into an immutable sampler slot in a descriptor set
///     is not allowed. If `pImmutableSamplers` is not `NULL`, then it is considered
///     to be a pointer to an array of sampler handles that will be consumed by the
///     set layout and used for the corresponding binding. If `pImmutableSamplers`
///     is `NULL`, then the sampler slots are dynamic and sampler handles must: be
///     bound into descriptor sets using this layout. If `descriptorType` is not one
///     of these descriptor types, then `pImmutableSamplers` is ignored.
///
/// The above layout definition allows the descriptor bindings to be specified
/// sparsely such that not all binding numbers between 0 and the maximum binding
/// number need to be specified in the `pBindings` array. Bindings that are not
/// specified have a `descriptorCount` and `stageFlags` of zero, and the
/// `descriptorType` is treated as undefined. However, all binding numbers between 0
/// and the maximum binding number in the
/// `VkDescriptorSetLayoutCreateInfo::pBindings` array may: consume memory in the
/// descriptor set layout even if not all descriptor bindings are used, though it
/// should: not consume additional memory from the descriptor pool.
///
/// > **Note**
/// >
/// > The maximum binding number specified should: be as compact as possible to
/// > avoid wasted memory.
///
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
    unsafe {
      self.pImmutableSamplers = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkDescriptorSetLayoutBinding<'a> {
  fn default() -> VkDescriptorSetLayoutBinding<'a> {
    VkDescriptorSetLayoutBinding::new()
  }
}
impl<'a> RawStruct for VkDescriptorSetLayoutBinding<'a> {
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
///
/// Information about the descriptor set layout is passed in an instance of the
/// `VkDescriptorSetLayoutCreateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkDescriptorSetLayoutCreateFlagBits` specifying
///     options for descriptor set layout creation.
///
///   - `bindingCount` is the number of elements in `pBindings`.
///
///   - `pBindings` is a pointer to an array of `VkDescriptorSetLayoutBinding`
///     structures.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDescriptorSetLayoutCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_bindings(mut self, value: &'a [VkDescriptorSetLayoutBinding<'a>]) -> Self {
    unsafe {
      self.pBindings = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkDescriptorSetLayoutCreateInfo<'a> {
  fn default() -> VkDescriptorSetLayoutCreateInfo<'a> {
    VkDescriptorSetLayoutCreateInfo::new()
  }
}
impl<'a> RawStruct for VkDescriptorSetLayoutCreateInfo<'a> {
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
///
///   - `type` is the type of descriptor.
///
///   - `descriptorCount` is the number of descriptors of that type to allocate.
///
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
}
impl Default for VkDescriptorPoolSize {
  fn default() -> VkDescriptorPoolSize {
    VkDescriptorPoolSize::new()
  }
}
impl RawStruct for VkDescriptorPoolSize {
  type Raw = types_raw::VkDescriptorPoolSize;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_pool_size() {
  assert_size!(types_raw::VkDescriptorPoolSize, VkDescriptorPoolSize);
}

/// Structure specifying parameters of a newly created descriptor pool
///
/// Additional information about the pool is passed in an instance of the
/// `VkDescriptorPoolCreateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkDescriptorPoolCreateFlagBits` specifying certain
///     supported operations on the pool.
///
///   - `maxSets` is the maximum number of descriptor sets that can: be allocated
///     from the pool.
///
///   - `poolSizeCount` is the number of elements in `pPoolSizes`.
///
///   - `pPoolSizes` is a pointer to an array of `VkDescriptorPoolSize` structures,
///     each containing a descriptor type and number of descriptors of that type to
///     be allocated in the pool.
///
/// If multiple `VkDescriptorPoolSize` structures appear in the `pPoolSizes` array
/// then the pool will be created with enough storage for the total number of
/// descriptors of each type.
///
/// Fragmentation of a descriptor pool is possible and may: lead to descriptor set
/// allocation failures. A failure due to fragmentation is defined as failing a
/// descriptor set allocation despite the sum of all outstanding descriptor set
/// allocations from the pool plus the requested allocation requiring no more than
/// the total number of descriptors requested at pool creation. Implementations
/// provide certain guarantees of when fragmentation must: not cause allocation
/// failure, as described below.
///
/// If a descriptor pool has not had any descriptor sets freed since it was created
/// or most recently reset then fragmentation must: not cause an allocation failure
/// (note that this is always the case for a pool created without the
/// `VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT` bit set). Additionally, if
/// all sets allocated from the pool since it was created or most recently reset use
/// the same number of descriptors (of each type) and the requested allocation also
/// uses that same number of descriptors (of each type), then fragmentation must:
/// not cause an allocation failure.
///
/// If an allocation failure occurs due to fragmentation, an application can: create
/// an additional descriptor pool to perform further descriptor set allocations.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pPoolSizes = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkDescriptorPoolCreateInfo<'a> {
  fn default() -> VkDescriptorPoolCreateInfo<'a> {
    VkDescriptorPoolCreateInfo::new()
  }
}
impl<'a> RawStruct for VkDescriptorPoolCreateInfo<'a> {
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
///
/// A *descriptor pool* maintains a pool of descriptors, from which descriptor sets
/// are allocated. Descriptor pools are externally synchronized, meaning that the
/// application must: not allocate and/or free descriptor sets from the same pool in
/// multiple threads simultaneously.
///
/// Descriptor pools are represented by `VkDescriptorPool` handles.
///
pub type VkDescriptorPool = VkNonDispatchableHandle<VkDescriptorPool__>;

/// Structure specifying the allocation parameters for descriptor sets
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `descriptorPool` is the pool which the sets will be allocated from.
///
///   - `descriptorSetCount` determines the number of descriptor sets to be
///     allocated from the pool.
///
///   - `pSetLayouts` is an array of descriptor set layouts, with each member
///     specifying how the corresponding descriptor set is allocated.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_descriptor_pool(mut self, value: VkDescriptorPool) -> Self {
    self.descriptorPool = value;
    self
  }
  #[inline]
  pub fn set_set_layouts(mut self, value: &'a [VkDescriptorSetLayout]) -> Self {
    unsafe {
      self.pSetLayouts = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkDescriptorSetAllocateInfo<'a> {
  fn default() -> VkDescriptorSetAllocateInfo<'a> {
    VkDescriptorSetAllocateInfo::new()
  }
}
impl<'a> RawStruct for VkDescriptorSetAllocateInfo<'a> {
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
///
/// Descriptor sets are allocated from descriptor pool objects, and are represented
/// by `VkDescriptorSet` handles.
///
pub type VkDescriptorSet = VkNonDispatchableHandle<VkDescriptorSet__>;

/// Structure specifying descriptor image info
///
///   - `sampler` is a sampler handle, and is used in descriptor updates for types
///     `VK_DESCRIPTOR_TYPE_SAMPLER` and `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
///     if the binding being updated does not use immutable samplers.
///
///   - `imageView` is an image view handle, and is used in descriptor updates for
///     types `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`,
///     `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`,
///     `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and
///     `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`.
///
///   - `imageLayout` is the layout that the image subresources accessible from
///     `imageView` will be in at the time this descriptor is accessed.
///     `imageLayout` is used in descriptor updates for types
///     `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`,
///     `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and
///     `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`.
///
/// Members of `VkDescriptorImageInfo` that are not used in an update (as described
/// above) are ignored.
///
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
}
impl Default for VkDescriptorImageInfo {
  fn default() -> VkDescriptorImageInfo {
    VkDescriptorImageInfo::new()
  }
}
impl RawStruct for VkDescriptorImageInfo {
  type Raw = types_raw::VkDescriptorImageInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_image_info() {
  assert_size!(types_raw::VkDescriptorImageInfo, VkDescriptorImageInfo);
}

/// Structure specifying descriptor buffer info
///
///   - `buffer` is the buffer resource.
///
///   - `offset` is the offset in bytes from the start of `buffer`. Access to buffer
///     memory via this descriptor uses addressing that is relative to this starting
///     offset.
///
///   - `range` is the size in bytes that is used for this descriptor update, or
///     `VK_WHOLE_SIZE` to use the range from `offset` to the end of the buffer.
///
/// > **Note**
/// >
/// > When setting `range` to `VK_WHOLE_SIZE`, the effective range must: not be
/// > larger than the maximum range for the descriptor type
/// > ([maxUniformBufferRange](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits-maxUniformBufferRange) or
/// > [maxStorageBufferRange](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-limits-maxStorageBufferRange)). This means
/// > that `VK_WHOLE_SIZE` is not typically useful in the common case where uniform
/// > buffer descriptors are suballocated from a buffer that is much larger than
/// > `maxUniformBufferRange`.
///
/// For `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` and
/// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` descriptor types, `offset` is the
/// base offset from which the dynamic offset is applied and `range` is the static
/// size used for all dynamic offsets.
///
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
}
impl Default for VkDescriptorBufferInfo {
  fn default() -> VkDescriptorBufferInfo {
    VkDescriptorBufferInfo::new()
  }
}
impl RawStruct for VkDescriptorBufferInfo {
  type Raw = types_raw::VkDescriptorBufferInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_descriptor_buffer_info() {
  assert_size!(types_raw::VkDescriptorBufferInfo, VkDescriptorBufferInfo);
}

/// Structure specifying the parameters of a descriptor set write operation
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `dstSet` is the destination descriptor set to update.
///
///   - `dstBinding` is the descriptor binding within that set.
///
///   - `dstArrayElement` is the starting element in that array.
///
///   - `descriptorCount` is the number of descriptors to update (the number of
///     elements in `pImageInfo`, `pBufferInfo`, or `pTexelBufferView`).
///
///   - `descriptorType` is a `VkDescriptorType` specifying the type of each
///     descriptor in `pImageInfo`, `pBufferInfo`, or `pTexelBufferView`, as
///     described below. It must: be the same type as that specified in
///     `VkDescriptorSetLayoutBinding` for `dstSet` at `dstBinding`. The type of the
///     descriptor also controls which array the descriptors are taken from.
///
///   - `pImageInfo` points to an array of `VkDescriptorImageInfo` structures or is
///     ignored, as described below.
///
///   - `pBufferInfo` points to an array of `VkDescriptorBufferInfo` structures or
///     is ignored, as described below.
///
///   - `pTexelBufferView` points to an array of `VkBufferView` handles as described
///     in the [Buffer Views](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-buffer-views) section or is ignored, as
///     described below.
///
/// Only one of `pImageInfo`, `pBufferInfo`, or `pTexelBufferView` members is used
/// according to the descriptor type specified in the `descriptorType` member of the
/// containing `VkWriteDescriptorSet` structure, as specified below.
///
/// If the `dstBinding` has fewer than `descriptorCount` array elements remaining
/// starting from `dstArrayElement`, then the remainder will be used to update the
/// subsequent binding - `dstBinding`+1 starting at array element zero. If a binding
/// has a `descriptorCount` of zero, it is skipped. This behavior applies
/// recursively, with the update affecting consecutive bindings as needed to update
/// all `descriptorCount` descriptors.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
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
  pub fn set_descriptor_type(mut self, value: VkDescriptorType) -> Self {
    self.descriptorType = value;
    self
  }
  #[inline]
  pub fn set_image_info(mut self, value: &'a [VkDescriptorImageInfo]) -> Self {
    unsafe {
      self.pImageInfo = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_buffer_info(mut self, value: &'a [VkDescriptorBufferInfo]) -> Self {
    unsafe {
      self.pBufferInfo = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_texel_buffer_view(mut self, value: &'a [VkBufferView]) -> Self {
    unsafe {
      self.pTexelBufferView = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkWriteDescriptorSet<'a> {
  fn default() -> VkWriteDescriptorSet<'a> {
    VkWriteDescriptorSet::new()
  }
}
impl<'a> RawStruct for VkWriteDescriptorSet<'a> {
  type Raw = types_raw::VkWriteDescriptorSet;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_write_descriptor_set() {
  assert_size!(types_raw::VkWriteDescriptorSet, VkWriteDescriptorSet);
}

/// Structure specifying a copy descriptor set operation
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `srcSet`, `srcBinding`, and `srcArrayElement` are the source set, binding,
///     and array element, respectively.
///
///   - `dstSet`, `dstBinding`, and `dstArrayElement` are the destination set,
///     binding, and array element, respectively.
///
///   - `descriptorCount` is the number of descriptors to copy from the source to
///     destination. If `descriptorCount` is greater than the number of remaining
///     array elements in the source or destination binding, those affect
///     consecutive bindings in a manner similar to `VkWriteDescriptorSet` above.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkCopyDescriptorSet {
  fn default() -> VkCopyDescriptorSet {
    VkCopyDescriptorSet::new()
  }
}
impl RawStruct for VkCopyDescriptorSet {
  type Raw = types_raw::VkCopyDescriptorSet;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_copy_descriptor_set() {
  assert_size!(types_raw::VkCopyDescriptorSet, VkCopyDescriptorSet);
}

/// Structure specifying parameters of a newly created framebuffer
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `renderPass` is a render pass that defines what render passes the
///     framebuffer will be compatible with. See [Render Pass
///     Compatibility](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-compatibility) for details.
///
///   - `attachmentCount` is the number of attachments.
///
///   - `pAttachments` is an array of `VkImageView` handles, each of which will be
///     used as the corresponding attachment in a render pass instance.
///
///   - `width`, `height` and `layers` define the dimensions of the framebuffer. If
///     the render pass uses multiview, then `layers` must: be one and each
///     attachment requires a number of layers that is greater than the maximum bit
///     index set in the view mask in the subpasses in which it is used.
///
/// Applications must: ensure that all accesses to memory that backs image
/// subresources used as attachments in a given renderpass instance either
/// happen-before the [load operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops) for those
/// attachments, or happen-after the [store operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops)
/// for those attachments.
///
/// For depth/stencil attachments, each aspect can: be used separately as
/// attachments and non-attachments as long as the non-attachment accesses are also
/// via an image subresource in either the
/// `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR` layout or the
/// `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR` layout, and the
/// attachment resource uses whichever of those two layouts the image accesses do
/// not. Use of non-attachment aspects in this case is only well defined if the
/// attachment is used in the subpass where the non-attachment access is being made,
/// or the layout of the image subresource is constant throughout the entire render
/// pass instance, including the `initialLayout` and `finalLayout`.
///
/// > **Note**
/// >
/// > These restrictions mean that the render pass has full knowledge of all uses of
/// > all of the attachments, so that the implementation is able to make correct
/// > decisions about when and how to perform layout transitions, when to overlap
/// > execution of subpasses, etc.
///
/// It is legal for a subpass to use no color or depth/stencil attachments, and
/// rather use shader side effects such as image stores and atomics to produce an
/// output. In this case, the subpass continues to use the `width`, `height`, and
/// `layers` of the framebuffer to define the dimensions of the rendering area, and
/// the `rasterizationSamples` from each pipeline’s
/// `VkPipelineMultisampleStateCreateInfo` to define the number of samples used in
/// rasterization; however, if `VkPhysicalDeviceFeatures::variableMultisampleRate`
/// is `VK_FALSE`, then all pipelines to be bound with a given zero-attachment
/// subpass must: have the same value for
/// `VkPipelineMultisampleStateCreateInfo::rasterizationSamples`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl<'a> Default for VkFramebufferCreateInfo<'a> {
  fn default() -> VkFramebufferCreateInfo<'a> {
    VkFramebufferCreateInfo::new()
  }
}
impl<'a> RawStruct for VkFramebufferCreateInfo<'a> {
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
///
/// Render passes operate in conjunction with *framebuffers*. Framebuffers represent
/// a collection of specific memory attachments that a render pass instance uses.
///
/// Framebuffers are represented by `VkFramebuffer` handles.
///
pub type VkFramebuffer = VkNonDispatchableHandle<VkFramebuffer__>;

/// Structure specifying an attachment description
///
///   - `flags` is a bitmask of `VkAttachmentDescriptionFlagBits` specifying
///     additional properties of the attachment.
///
///   - `format` is a `VkFormat` value specifying the format of the image that will
///     be used for the attachment.
///
///   - `samples` is the number of samples of the image as defined in
///     `VkSampleCountFlagBits`.
///
///   - `loadOp` is a `VkAttachmentLoadOp` value specifying how the contents of
///     color and depth components of the attachment are treated at the beginning of
///     the subpass where it is first used.
///
///   - `storeOp` is a `VkAttachmentStoreOp` value specifying how the contents of
///     color and depth components of the attachment are treated at the end of the
///     subpass where it is last used.
///
///   - `stencilLoadOp` is a `VkAttachmentLoadOp` value specifying how the contents
///     of stencil components of the attachment are treated at the beginning of the
///     subpass where it is first used.
///
///   - `stencilStoreOp` is a `VkAttachmentStoreOp` value specifying how the
///     contents of stencil components of the attachment are treated at the end of
///     the last subpass where it is used.
///
///   - `initialLayout` is the layout the attachment image subresource will be in
///     when a render pass instance begins.
///
///   - `finalLayout` is the layout the attachment image subresource will be
///     transitioned to when a render pass instance ends. During a render pass
///     instance, an attachment can: use a different layout in each subpass, if
///     desired.
///
/// If the attachment uses a color format, then `loadOp` and `storeOp` are used, and
/// `stencilLoadOp` and `stencilStoreOp` are ignored. If the format has depth and/or
/// stencil components, `loadOp` and `storeOp` apply only to the depth data, while
/// `stencilLoadOp` and `stencilStoreOp` define how the stencil data is handled.
/// `loadOp` and `stencilLoadOp` define the *load operations* that execute as part
/// of the first subpass that uses the attachment. `storeOp` and `stencilStoreOp`
/// define the *store operations* that execute as part of the last subpass that uses
/// the attachment.
///
/// The load operation for each sample in an attachment happens-before any recorded
/// command which accesses the sample in the first subpass where the attachment is
/// used. Load operations for attachments with a depth/stencil format execute in the
/// `VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT` pipeline stage. Load operations for
/// attachments with a color format execute in the
/// `VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.
///
/// The store operation for each sample in an attachment happens-after any recorded
/// command which accesses the sample in the last subpass where the attachment is
/// used. Store operations for attachments with a depth/stencil format execute in
/// the `VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT` pipeline stage. Store operations
/// for attachments with a color format execute in the
/// `VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.
///
/// If an attachment is not used by any subpass, then `loadOp`, `storeOp`,
/// `stencilStoreOp`, and `stencilLoadOp` are ignored, and the attachment’s memory
/// contents will not be modified by execution of a render pass instance.
///
/// The load and store operations apply on the first and last use of each view in
/// the render pass, respectively. If a view index of an attachment is not included
/// in the view mask in any subpass that uses it, then the load and store operations
/// are ignored, and the attachment’s memory contents will not be modified by
/// execution of a render pass instance.
///
/// During a render pass instance, input/color attachments with color formats that
/// have a component size of 8, 16, or 32 bits must: be represented in the
/// attachment’s format throughout the instance. Attachments with other floating- or
/// fixed-point color formats, or with depth components may: be represented in a
/// format with a precision higher than the attachment format, but must: be
/// represented with the same range. When such a component is loaded via the
/// `loadOp`, it will be converted into an implementation-dependent format used by
/// the render pass. Such components must: be converted from the render pass format,
/// to the format of the attachment, before they are resolved or stored at the end
/// of a render pass instance via `storeOp`. Conversions occur as described in
/// [Numeric Representation and Computation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-numerics) and
/// [Fixed-Point Data Conversions](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-fixedconv).
///
/// If `flags` includes `VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT`, then the
/// attachment is treated as if it shares physical memory with another attachment in
/// the same render pass. This information limits the ability of the implementation
/// to reorder certain operations (like layout transitions and the `loadOp`) such
/// that it is not improperly reordered against other uses of the same physical
/// memory via a different attachment. This is described in more detail below.
///
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
}
impl Default for VkAttachmentDescription {
  fn default() -> VkAttachmentDescription {
    VkAttachmentDescription::new()
  }
}
impl RawStruct for VkAttachmentDescription {
  type Raw = types_raw::VkAttachmentDescription;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_attachment_description() {
  assert_size!(types_raw::VkAttachmentDescription, VkAttachmentDescription);
}

/// Structure specifying an attachment reference
///
///   - `attachment` is the index of the attachment of the render pass, and
///     corresponds to the index of the corresponding element in the `pAttachments`
///     array of the `VkRenderPassCreateInfo` structure. If any color or
///     depth/stencil attachments are `VK_ATTACHMENT_UNUSED`, then no writes occur
///     for those attachments.
///
///   - `layout` is a `VkImageLayout` value specifying the layout the attachment
///     uses during the subpass.
///
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
}
impl Default for VkAttachmentReference {
  fn default() -> VkAttachmentReference {
    VkAttachmentReference::new()
  }
}
impl RawStruct for VkAttachmentReference {
  type Raw = types_raw::VkAttachmentReference;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_attachment_reference() {
  assert_size!(types_raw::VkAttachmentReference, VkAttachmentReference);
}

/// Structure specifying a subpass description
///
///   - `flags` is a bitmask of `VkSubpassDescriptionFlagBits` specifying usage of
///     the subpass.
///
///   - `pipelineBindPoint` is a `VkPipelineBindPoint` value specifying whether this
///     is a compute or graphics subpass. Currently, only graphics subpasses are
///     supported.
///
///   - `inputAttachmentCount` is the number of input attachments.
///
///   - `pInputAttachments` is an array of `VkAttachmentReference` structures
///     (defined below) that lists which of the render pass’s attachments can: be
///     read in the fragment shader stage during the subpass, and what layout each
///     attachment will be in during the subpass. Each element of the array
///     corresponds to an input attachment unit number in the shader, i.e. if the
///     shader declares an input variable `layout(input_attachment_index=X, set=Y,
///     binding=Z)` then it uses the attachment provided in
///     `pInputAttachments`\[X\]. Input attachments must: also be bound to the
///     pipeline with a descriptor set, with the input attachment descriptor written
///     in the location (set=Y, binding=Z). Fragment shaders can: use subpass input
///     variables to access the contents of an input attachment at the fragment’s
///     (x, y, layer) framebuffer coordinates.
///
///   - `colorAttachmentCount` is the number of color attachments.
///
///   - `pColorAttachments` is an array of `colorAttachmentCount`
///     `VkAttachmentReference` structures that lists which of the render pass’s
///     attachments will be used as color attachments in the subpass, and what
///     layout each attachment will be in during the subpass. Each element of the
///     array corresponds to a fragment shader output location, i.e. if the shader
///     declared an output variable `layout(location=X)` then it uses the attachment
///     provided in `pColorAttachments`\[X\].
///
///   - `pResolveAttachments` is `NULL` or an array of `colorAttachmentCount`
///     `VkAttachmentReference` structures that lists which of the render pass’s
///     attachments are resolved to at the end of the subpass, and what layout each
///     attachment will be in during the multisample resolve operation. If
///     `pResolveAttachments` is not `NULL`, each of its elements corresponds to a
///     color attachment (the element in `pColorAttachments` at the same index), and
///     a multisample resolve operation is defined for each attachment. At the end
///     of each subpass, multisample resolve operations read the subpass’s color
///     attachments, and resolve the samples for each pixel to the same pixel
///     location in the corresponding resolve attachments, unless the resolve
///     attachment index is `VK_ATTACHMENT_UNUSED`. If the first use of an
///     attachment in a render pass is as a resolve attachment, then the `loadOp` is
///     effectively ignored as the resolve is guaranteed to overwrite all pixels in
///     the render area.
///
///   - `pDepthStencilAttachment` is a pointer to a `VkAttachmentReference`
///     specifying which attachment will be used for depth/stencil data and the
///     layout it will be in during the subpass. Setting the attachment index to
///     `VK_ATTACHMENT_UNUSED` or leaving this pointer as `NULL` indicates that no
///     depth/stencil attachment will be used in the subpass.
///
///   - `preserveAttachmentCount` is the number of preserved attachments.
///
///   - `pPreserveAttachments` is an array of `preserveAttachmentCount` render pass
///     attachment indices describing the attachments that are not used by a
///     subpass, but whose contents must: be preserved throughout the subpass.
///
/// The contents of an attachment within the render area become undefined at the
/// start of a subpass **S** if all of the following conditions are true:
///
///   - The attachment is used as a color, depth/stencil, or resolve attachment in
///     any subpass in the render pass.
///
///   - There is a subpass **S<sub>1</sub>** that uses or preserves the attachment,
///     and a subpass dependency from **S<sub>1</sub>** to **S**.
///
///   - The attachment is not used or preserved in subpass **S**.
///
/// Once the contents of an attachment become undefined in subpass **S**, they
/// remain undefined for subpasses in subpass dependency chains starting with
/// subpass **S** until they are written again. However, they remain valid for
/// subpasses in other subpass dependency chains starting with subpass
/// **S<sub>1</sub>** if those subpasses use or preserve the attachment.
///
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
    unsafe {
      self.pInputAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_color_attachments(mut self, value: &'a [VkAttachmentReference]) -> Self {
    unsafe {
      self.pColorAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_resolve_attachments(mut self, value: &'a [VkAttachmentReference]) -> Self {
    unsafe {
      self.pResolveAttachments = value.as_raw();
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
    unsafe {
      self.pPreserveAttachments = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkSubpassDescription<'a> {
  fn default() -> VkSubpassDescription<'a> {
    VkSubpassDescription::new()
  }
}
impl<'a> RawStruct for VkSubpassDescription<'a> {
  type Raw = types_raw::VkSubpassDescription;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subpass_description() {
  assert_size!(types_raw::VkSubpassDescription, VkSubpassDescription);
}

/// Structure specifying a subpass dependency
///
///   - `srcSubpass` is the subpass index of the first subpass in the dependency, or
///     `VK_SUBPASS_EXTERNAL`.
///
///   - `dstSubpass` is the subpass index of the second subpass in the dependency,
///     or `VK_SUBPASS_EXTERNAL`.
///
///   - `srcStageMask` is a bitmask of `VkPipelineStageFlagBits` specifying the
///     [source stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
///
///   - `dstStageMask` is a bitmask of `VkPipelineStageFlagBits` specifying the
///     [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks)
///
///   - `srcAccessMask` is a bitmask of `VkAccessFlagBits` specifying a [source
///     access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks).
///
///   - `dstAccessMask` is a bitmask of `VkAccessFlagBits` specifying a [destination
///     access mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks).
///
///   - `dependencyFlags` is a bitmask of `VkDependencyFlagBits`.
///
/// If `srcSubpass` is equal to `dstSubpass` then the `VkSubpassDependency`
/// describes a [subpass
/// self-dependency](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-barriers-subpass-self-dependencies),
/// and only constrains the pipeline barriers allowed within a subpass instance.
/// Otherwise, when a render pass instance which includes a subpass dependency is
/// submitted to a queue, it defines a memory dependency between the subpasses
/// identified by `srcSubpass` and `dstSubpass`.
///
/// If `srcSubpass` is equal to `VK_SUBPASS_EXTERNAL`, the first [synchronization
/// scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes) includes commands submitted to the
/// queue before the render pass instance began. Otherwise, the first set of
/// commands includes all commands submitted as part of the subpass instance
/// identified by `srcSubpass` and any load, store or multisample resolve operations
/// on attachments used in `srcSubpass`. In either case, the first synchronization
/// scope is limited to operations on the pipeline stages determined by the [source
/// stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `srcStageMask`.
///
/// If `dstSubpass` is equal to `VK_SUBPASS_EXTERNAL`, the second [synchronization
/// scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes) includes commands submitted after
/// the render pass instance is ended. Otherwise, the second set of commands
/// includes all commands submitted as part of the subpass instance identified by
/// `dstSubpass` and any load, store or multisample resolve operations on
/// attachments used in `dstSubpass`. In either case, the second synchronization
/// scope is limited to operations on the pipeline stages determined by the
/// [destination stage mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by
/// `dstStageMask`.
///
/// The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is limited
/// to access in the pipeline stages determined by the [source stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `srcStageMask`. It is
/// also limited to access types in the [source access
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks) specified by `srcAccessMask`.
///
/// The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
/// limited to access in the pipeline stages determined by the [destination stage
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-masks) specified by `dstStageMask`. It is
/// also limited to access types in the [destination access
/// mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-masks) specified by `dstAccessMask`.
///
/// The [availability and visibility
/// operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-available-and-visible) defined by a
/// subpass dependency affect the execution of [image layout
/// transitions](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-layout-transitions) within the render pass.
///
/// > **Note**
/// >
/// > For non-attachment resources, the memory dependency expressed by subpass
/// > dependency is nearly identical to that of a `VkMemoryBarrier` (with matching
/// > `srcAccessMask`/`dstAccessMask` parameters) submitted as a part of a
/// > `vkCmdPipelineBarrier` (with matching `srcStageMask`/`dstStageMask`
/// > parameters). The only difference being that its scopes are limited to the
/// > identified subpasses rather than potentially affecting everything before and
/// > after.
/// >
/// > For attachments however, subpass dependencies work more like an
/// > `VkImageMemoryBarrier` defined similarly to the `VkMemoryBarrier` above, the
/// > queue family indices set to `VK_QUEUE_FAMILY_IGNORED`, and layouts as follows:
/// >
/// >   - The equivalent to `oldLayout` is the attachment’s layout according to the
/// >     subpass description for `srcSubpass`.
/// >
/// >   - The equivalent to `newLayout` is the attachment’s layout according to the
/// >     subpass description for `dstSubpass`.
///
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
}
impl Default for VkSubpassDependency {
  fn default() -> VkSubpassDependency {
    VkSubpassDependency::new()
  }
}
impl RawStruct for VkSubpassDependency {
  type Raw = types_raw::VkSubpassDependency;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_subpass_dependency() {
  assert_size!(types_raw::VkSubpassDependency, VkSubpassDependency);
}

/// Structure specifying parameters of a newly created render pass
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `attachmentCount` is the number of attachments used by this render pass, or
///     zero indicating no attachments. Attachments are referred to by zero-based
///     indices in the range \[0,`attachmentCount`).
///
///   - `pAttachments` points to an array of `attachmentCount` number of
///     `VkAttachmentDescription` structures describing properties of the
///     attachments, or `NULL` if `attachmentCount` is zero.
///
///   - `subpassCount` is the number of subpasses to create for this render pass.
///     Subpasses are referred to by zero-based indices in the range
///     \[0,`subpassCount`). A render pass must: have at least one subpass.
///
///   - `pSubpasses` points to an array of `subpassCount` number of
///     `VkSubpassDescription` structures describing properties of the subpasses.
///
///   - `dependencyCount` is the number of dependencies between pairs of subpasses,
///     or zero indicating no dependencies.
///
///   - `pDependencies` points to an array of `dependencyCount` number of
///     `VkSubpassDependency` structures describing dependencies between pairs of
///     subpasses, or `NULL` if `dependencyCount` is zero.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkRenderPassCreateFlags) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_attachments(mut self, value: &'a [VkAttachmentDescription]) -> Self {
    unsafe {
      self.pAttachments = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_subpasses(mut self, value: &'a [VkSubpassDescription<'a>]) -> Self {
    unsafe {
      self.pSubpasses = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_dependencies(mut self, value: &'a [VkSubpassDependency]) -> Self {
    unsafe {
      self.pDependencies = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkRenderPassCreateInfo<'a> {
  fn default() -> VkRenderPassCreateInfo<'a> {
    VkRenderPassCreateInfo::new()
  }
}
impl<'a> RawStruct for VkRenderPassCreateInfo<'a> {
  type Raw = types_raw::VkRenderPassCreateInfo;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_render_pass_create_info() {
  assert_size!(types_raw::VkRenderPassCreateInfo, VkRenderPassCreateInfo);
}

/// Structure specifying parameters of a newly created command pool
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkCommandPoolCreateFlagBits` indicating usage
///     behavior for the pool and command buffers allocated from it.
///
///   - `queueFamilyIndex` designates a queue family as described in section [Queue
///     Family Properties](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-queueprops). All command buffers allocated
///     from this command pool must: be submitted on queues from the same queue
///     family.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkCommandPoolCreateInfo {
  fn default() -> VkCommandPoolCreateInfo {
    VkCommandPoolCreateInfo::new()
  }
}
impl RawStruct for VkCommandPoolCreateInfo {
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
///
/// Command pools are opaque objects that command buffer memory is allocated from,
/// and which allow the implementation to amortize the cost of resource creation
/// across multiple command buffers. Command pools are externally synchronized,
/// meaning that a command pool must: not be used concurrently in multiple threads.
/// That includes use via recording commands on any command buffers allocated from
/// the pool, as well as operations that allocate, free, and reset command buffers
/// or the pool itself.
///
/// Command pools are represented by `VkCommandPool` handles.
///
pub type VkCommandPool = VkNonDispatchableHandle<VkCommandPool__>;

/// Structure specifying the allocation parameters for command buffer object
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `commandPool` is the command pool from which the command buffers are
///     allocated.
///
///   - `level` is an `VkCommandBufferLevel` value specifying the command buffer
///     level.
///
///   - `commandBufferCount` is the number of command buffers to allocate from the
///     pool.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkCommandBufferAllocateInfo {
  fn default() -> VkCommandBufferAllocateInfo {
    VkCommandBufferAllocateInfo::new()
  }
}
impl RawStruct for VkCommandBufferAllocateInfo {
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
///
/// If the command buffer is a secondary command buffer, then the
/// `VkCommandBufferInheritanceInfo` structure defines any state that will be
/// inherited from the primary command buffer.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `renderPass` is a `VkRenderPass` object defining which render passes the
///     `VkCommandBuffer` will be [compatible](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-compatibility) with and
///     can: be executed within. If the `VkCommandBuffer` will not be executed
///     within a render pass instance, `renderPass` is ignored.
///
///   - `subpass` is the index of the subpass within the render pass instance that
///     the `VkCommandBuffer` will be executed within. If the `VkCommandBuffer` will
///     not be executed within a render pass instance, `subpass` is ignored.
///
///   - `framebuffer` optionally refers to the `VkFramebuffer` object that the
///     `VkCommandBuffer` will be rendering to if it is executed within a render
///     pass instance. It can: be `VK_NULL_HANDLE` if the framebuffer is not known,
///     or if the `VkCommandBuffer` will not be executed within a render pass
///     instance.
///
///     > **Note**
///     >
///     > Specifying the exact framebuffer that the secondary command buffer will be
///     > executed with may: result in better performance at command buffer
///     > execution time.
///
///   - `occlusionQueryEnable` indicates whether the command buffer can: be executed
///     while an occlusion query is active in the primary command buffer. If this is
///     `VK_TRUE`, then this command buffer can: be executed whether the primary
///     command buffer has an occlusion query active or not. If this is `VK_FALSE`,
///     then the primary command buffer must: not have an occlusion query active.
///
///   - `queryFlags` indicates the query flags that can: be used by an active
///     occlusion query in the primary command buffer when this secondary command
///     buffer is executed. If this value includes the
///     `VK_QUERY_CONTROL_PRECISE_BIT` bit, then the active query can: return
///     boolean results or actual sample counts. If this bit is not set, then the
///     active query must: not use the `VK_QUERY_CONTROL_PRECISE_BIT` bit.
///
///   - `pipelineStatistics` is a bitmask of `VkQueryPipelineStatisticFlagBits`
///     specifying the set of pipeline statistics that can: be counted by an active
///     query in the primary command buffer when this secondary command buffer is
///     executed. If this value includes a given bit, then this command buffer can:
///     be executed whether the primary command buffer has a pipeline statistics
///     query active that includes this bit or not. If this value excludes a given
///     bit, then the active pipeline statistics query must: not be from a query
///     pool that counts that statistic.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl Default for VkCommandBufferInheritanceInfo {
  fn default() -> VkCommandBufferInheritanceInfo {
    VkCommandBufferInheritanceInfo::new()
  }
}
impl RawStruct for VkCommandBufferInheritanceInfo {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkCommandBufferUsageFlagBits` specifying usage
///     behavior for the command buffer.
///
///   - `pInheritanceInfo` is a pointer to a `VkCommandBufferInheritanceInfo`
///     structure, which is used if `commandBuffer` is a secondary command buffer.
///     If this is a primary command buffer, then this value is ignored.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
impl<'a> Default for VkCommandBufferBeginInfo<'a> {
  fn default() -> VkCommandBufferBeginInfo<'a> {
    VkCommandBufferBeginInfo::new()
  }
}
impl<'a> RawStruct for VkCommandBufferBeginInfo<'a> {
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
///
///   - `srcOffset` is the starting offset in bytes from the start of `srcBuffer`.
///
///   - `dstOffset` is the starting offset in bytes from the start of `dstBuffer`.
///
///   - `size` is the number of bytes to copy.
///
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
}
impl Default for VkBufferCopy {
  fn default() -> VkBufferCopy {
    VkBufferCopy::new()
  }
}
impl RawStruct for VkBufferCopy {
  type Raw = types_raw::VkBufferCopy;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_copy() {
  assert_size!(types_raw::VkBufferCopy, VkBufferCopy);
}

/// Structure specifying a image subresource layers
///
///   - `aspectMask` is a combination of `VkImageAspectFlagBits`, selecting the
///     color, depth and/or stencil aspects to be copied.
///
///   - `mipLevel` is the mipmap level to copy from.
///
///   - `baseArrayLayer` and `layerCount` are the starting layer and number of
///     layers to copy.
///
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
}
impl Default for VkImageSubresourceLayers {
  fn default() -> VkImageSubresourceLayers {
    VkImageSubresourceLayers::new()
  }
}
impl RawStruct for VkImageSubresourceLayers {
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
///
///   - `srcSubresource` and `dstSubresource` are `VkImageSubresourceLayers`
///     structures specifying the image subresources of the images used for the
///     source and destination image data, respectively.
///
///   - `srcOffset` and `dstOffset` select the initial `x`, `y`, and `z` offsets in
///     texels of the sub-regions of the source and destination image data.
///
///   - `extent` is the size in texels of the image to copy in `width`, `height` and
///     `depth`.
///
/// For `VK_IMAGE_TYPE_3D` images, copies are performed slice by slice starting with
/// the `z` member of the `srcOffset` or `dstOffset`, and copying `depth` slices.
/// For images with multiple layers, copies are performed layer by layer starting
/// with the `baseArrayLayer` member of the `srcSubresource` or `dstSubresource` and
/// copying `layerCount` layers. Image data can: be copied between images with
/// different image types. If one image is `VK_IMAGE_TYPE_3D` and the other image is
/// `VK_IMAGE_TYPE_2D` with multiple layers, then each slice is copied to or from a
/// different layer.
///
/// Copies involving a [multi-planar image
/// format](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion) specify the region
/// to be copied in terms of the *plane* to be copied, not the coordinates of the
/// multi-planar image. This means that copies accessing the R/B planes of
/// “etext:\_422” format images must: fit the copied region within half the
/// `width` of the parent image, and that copies accessing the R/B planes of
/// “etext:\_420” format images must: fit the copied region within half the
/// `width` and `height` of the parent image.
///
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
}
impl Default for VkImageCopy {
  fn default() -> VkImageCopy {
    VkImageCopy::new()
  }
}
impl RawStruct for VkImageCopy {
  type Raw = types_raw::VkImageCopy;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_copy() {
  assert_size!(types_raw::VkImageCopy, VkImageCopy);
}

/// Structure specifying an image blit operation
///
///   - `srcSubresource` is the subresource to blit from.
///
///   - `srcOffsets` is an array of two `VkOffset3D` structures specifying the
///     bounds of the source region within `srcSubresource`.
///
///   - `dstSubresource` is the subresource to blit into.
///
///   - `dstOffsets` is an array of two `VkOffset3D` structures specifying the
///     bounds of the destination region within `dstSubresource`.
///
/// For each element of the `pRegions` array, a blit operation is performed the
/// specified source and destination regions.
///
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
}
impl Default for VkImageBlit {
  fn default() -> VkImageBlit {
    VkImageBlit::new()
  }
}
impl RawStruct for VkImageBlit {
  type Raw = types_raw::VkImageBlit;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_blit() {
  assert_size!(types_raw::VkImageBlit, VkImageBlit);
}

/// Structure specifying a buffer image copy operation
///
/// For both `vkCmdCopyBufferToImage` and `vkCmdCopyImageToBuffer`, each element of
/// `pRegions` is a structure defined as.
///
///   - `bufferOffset` is the offset in bytes from the start of the buffer object
///     where the image data is copied from or to.
///
///   - `bufferRowLength` and `bufferImageHeight` specify the data in buffer memory
///     as a subregion of a larger two- or three-dimensional image, and control the
///     addressing calculations of data in buffer memory. If either of these values
///     is zero, that aspect of the buffer memory is considered to be tightly packed
///     according to the `imageExtent`.
///
///   - `imageSubresource` is a `VkImageSubresourceLayers` used to specify the
///     specific image subresources of the image used for the source or destination
///     image data.
///
///   - `imageOffset` selects the initial `x`, `y`, `z` offsets in texels of the
///     sub-region of the source or destination image data.
///
///   - `imageExtent` is the size in texels of the image to copy in `width`,
///     `height` and `depth`.
///
/// When copying to or from a depth or stencil aspect, the data in buffer memory
/// uses a layout that is a (mostly) tightly packed representation of the depth or
/// stencil data. Specifically:
///
///   - data copied to or from the stencil aspect of any depth/stencil format is
///     tightly packed with one `VK_FORMAT_S8_UINT` value per texel.
///
///   - data copied to or from the depth aspect of a `VK_FORMAT_D16_UNORM` or
///     `VK_FORMAT_D16_UNORM_S8_UINT` format is tightly packed with one
///     `VK_FORMAT_D16_UNORM` value per texel.
///
///   - data copied to or from the depth aspect of a `VK_FORMAT_D32_SFLOAT` or
///     `VK_FORMAT_D32_SFLOAT_S8_UINT` format is tightly packed with one
///     `VK_FORMAT_D32_SFLOAT` value per texel.
///
///   - data copied to or from the depth aspect of a `VK_FORMAT_X8_D24_UNORM_PACK32`
///     or `VK_FORMAT_D24_UNORM_S8_UINT` format is packed with one 32-bit word per
///     texel with the D24 value in the LSBs of the word, and undefined values in
///     the eight MSBs.
///
/// > **Note**
/// >
/// > To copy both the depth and stencil aspects of a depth/stencil format, two
/// > entries in `pRegions` can: be used, where one specifies the depth aspect in
/// > `imageSubresource`, and the other specifies the stencil aspect.
///
/// Because depth or stencil aspect buffer to image copies may: require format
/// conversions on some implementations, they are not supported on queues that do
/// not support graphics. When copying to a depth aspect, the data in buffer memory
/// must: be in the the range \[0,1\] or undefined results occur.
///
/// Copies are done layer by layer starting with image layer `baseArrayLayer` member
/// of `imageSubresource`. `layerCount` layers are copied from the source image or
/// to the destination image.
///
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
}
impl Default for VkBufferImageCopy {
  fn default() -> VkBufferImageCopy {
    VkBufferImageCopy::new()
  }
}
impl RawStruct for VkBufferImageCopy {
  type Raw = types_raw::VkBufferImageCopy;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_buffer_image_copy() {
  assert_size!(types_raw::VkBufferImageCopy, VkBufferImageCopy);
}

/// Structure specifying a clear color value
///
///   - `float32` are the color clear values when the format of the image or
///     attachment is one of the formats in the [Interpretation of Numeric
///     Format](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-numericformat) table other than signed integer
///     (etext:SINT) or unsigned integer (etext:UINT). Floating point values are
///     automatically converted to the format of the image, with the clear value
///     being treated as linear if the image is sRGB.
///
///   - `int32` are the color clear values when the format of the image or
///     attachment is signed integer (etext:SINT). Signed integer values are
///     converted to the format of the image by casting to the smaller type (with
///     negative 32-bit values mapping to negative values in the smaller type). If
///     the integer clear value is not representable in the target type (e.g. would
///     overflow in conversion to that type), the clear value is undefined.
///
///   - `uint32` are the color clear values when the format of the image or
///     attachment is unsigned integer (etext:UINT). Unsigned integer values are
///     converted to the format of the image by casting to the integer type with
///     fewer bits.
///
/// The four array elements of the clear color map to R, G, B, and A components of
/// image formats, in order.
///
/// If the image has more than one sample, the same value is written to all samples
/// for any pixels being cleared.
///
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
  pub float32: [f32; 4],
  pub int32: [i32; 4],
  pub uint32: [u32; 4],
}
impl RawStruct for VkClearColorValue {
  type Raw = types_raw::VkClearColorValue;
}
#[cfg(test)]
#[test]
fn test_union_size_vk_clear_color_value() {
  assert_size!(types_raw::VkClearColorValue, VkClearColorValue);
}

/// Structure specifying a clear depth stencil value
///
///   - `depth` is the clear value for the depth aspect of the depth/stencil
///     attachment. It is a floating-point value which is automatically converted to
///     the attachment’s format.
///
///   - `stencil` is the clear value for the stencil aspect of the depth/stencil
///     attachment. It is a 32-bit integer value which is converted to the
///     attachment’s format by taking the appropriate number of LSBs.
///
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
}
impl Default for VkClearDepthStencilValue {
  fn default() -> VkClearDepthStencilValue {
    VkClearDepthStencilValue::new()
  }
}
impl RawStruct for VkClearDepthStencilValue {
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
///
///   - `color` specifies the color image clear values to use when clearing a color
///     image or attachment.
///
///   - `depthStencil` specifies the depth and stencil clear values to use when
///     clearing a depth/stencil image or attachment.
///
/// This union is used where part of the API requires either color or depth/stencil
/// clear values, depending on the attachment, and defines the initial clear values
/// in the `VkRenderPassBeginInfo` structure.
///
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
  pub color: VkClearColorValue,
  pub depthStencil: VkClearDepthStencilValue,
}
impl RawStruct for VkClearValue {
  type Raw = types_raw::VkClearValue;
}
#[cfg(test)]
#[test]
fn test_union_size_vk_clear_value() {
  assert_size!(types_raw::VkClearValue, VkClearValue);
}

/// Structure specifying a clear attachment
///
///   - `aspectMask` is a mask selecting the color, depth and/or stencil aspects of
///     the attachment to be cleared. `aspectMask` can: include
///     `VK_IMAGE_ASPECT_COLOR_BIT` for color attachments,
///     `VK_IMAGE_ASPECT_DEPTH_BIT` for depth/stencil attachments with a depth
///     component, and `VK_IMAGE_ASPECT_STENCIL_BIT` for depth/stencil attachments
///     with a stencil component. If the subpass’s depth/stencil attachment is
///     `VK_ATTACHMENT_UNUSED`, then the clear has no effect.
///
///   - `colorAttachment` is only meaningful if `VK_IMAGE_ASPECT_COLOR_BIT` is set
///     in `aspectMask`, in which case it is an index to the `pColorAttachments`
///     array in the `VkSubpassDescription` structure of the current subpass which
///     selects the color attachment to clear. If `colorAttachment` is
///     `VK_ATTACHMENT_UNUSED` then the clear has no effect.
///
///   - `clearValue` is the color or depth/stencil value to clear the attachment to,
///     as described in [Clear Values](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#clears-values) below.
///
/// No memory barriers are needed between `vkCmdClearAttachments` and preceding or
/// subsequent draw or attachment clear commands in the same subpass.
///
/// The `vkCmdClearAttachments` command is not affected by the bound pipeline state.
///
/// Attachments can: also be cleared at the beginning of a render pass instance by
/// setting `loadOp` (or `stencilLoadOp`) of `VkAttachmentDescription` to
/// `VK_ATTACHMENT_LOAD_OP_CLEAR`, as described for `vkCreateRenderPass`.
///
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
}
impl Default for VkClearAttachment {
  fn default() -> VkClearAttachment {
    VkClearAttachment::new()
  }
}
impl RawStruct for VkClearAttachment {
  type Raw = types_raw::VkClearAttachment;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_clear_attachment() {
  assert_size!(types_raw::VkClearAttachment, VkClearAttachment);
}

/// Structure specifying a clear rectangle
///
///   - `rect` is the two-dimensional region to be cleared.
///
///   - `baseArrayLayer` is the first layer to be cleared.
///
///   - `layerCount` is the number of layers to clear.
///
/// The layers \[`baseArrayLayer`, `baseArrayLayer` + `layerCount`) counting from
/// the base layer of the attachment image view are cleared.
///
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
}
impl Default for VkClearRect {
  fn default() -> VkClearRect {
    VkClearRect::new()
  }
}
impl RawStruct for VkClearRect {
  type Raw = types_raw::VkClearRect;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_clear_rect() {
  assert_size!(types_raw::VkClearRect, VkClearRect);
}

/// Structure specifying an image resolve operation
///
///   - `srcSubresource` and `dstSubresource` are `VkImageSubresourceLayers`
///     structures specifying the image subresources of the images used for the
///     source and destination image data, respectively. Resolve of depth/stencil
///     images is not supported.
///
///   - `srcOffset` and `dstOffset` select the initial `x`, `y`, and `z` offsets in
///     texels of the sub-regions of the source and destination image data.
///
///   - `extent` is the size in texels of the source image to resolve in `width`,
///     `height` and `depth`.
///
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
}
impl Default for VkImageResolve {
  fn default() -> VkImageResolve {
    VkImageResolve::new()
  }
}
impl RawStruct for VkImageResolve {
  type Raw = types_raw::VkImageResolve;
}
#[cfg(test)]
#[test]
fn test_struct_size_vk_image_resolve() {
  assert_size!(types_raw::VkImageResolve, VkImageResolve);
}

/// Structure specifying render pass begin info
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `renderPass` is the render pass to begin an instance of.
///
///   - `framebuffer` is the framebuffer containing the attachments that are used
///     with the render pass.
///
///   - `renderArea` is the render area that is affected by the render pass
///     instance, and is described in more detail below.
///
///   - `clearValueCount` is the number of elements in `pClearValues`.
///
///   - `pClearValues` is an array of `VkClearValue` structures that contains clear
///     values for each attachment, if the attachment uses a `loadOp` value of
///     `VK_ATTACHMENT_LOAD_OP_CLEAR` or if the attachment has a depth/stencil
///     format and uses a `stencilLoadOp` value of `VK_ATTACHMENT_LOAD_OP_CLEAR`.
///     The array is indexed by attachment number. Only elements corresponding to
///     cleared attachments are used. Other elements of `pClearValues` are ignored.
///
/// `renderArea` is the render area that is affected by the render pass instance.
/// The effects of attachment load, store and multisample resolve operations are
/// restricted to the pixels whose x and y coordinates fall within the render area
/// on all attachments. The render area extends to all layers of `framebuffer`. The
/// application must: ensure (using scissor if necessary) that all rendering is
/// contained within the render area, otherwise the pixels outside of the render
/// area become undefined and shader side effects may: occur for fragments outside
/// the render area. The render area must: be contained within the framebuffer
/// dimensions.
///
/// When multiview is enabled, the resolve operation at the end of a subpass applies
/// to all views in the view mask.
///
/// > **Note**
/// >
/// > There may: be a performance cost for using a render area smaller than the
/// > framebuffer, unless it matches the render area granularity for the render
/// > pass.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pClearValues = value.as_raw();
    }
    self
  }
}
impl<'a> Default for VkRenderPassBeginInfo<'a> {
  fn default() -> VkRenderPassBeginInfo<'a> {
    VkRenderPassBeginInfo::new()
  }
}
impl<'a> RawStruct for VkRenderPassBeginInfo<'a> {
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
///
/// Native platform surface or window objects are abstracted by surface objects,
/// which are represented by `VkSurfaceKHR` handles:
///
/// The `VK_KHR_surface` extension declares the `VkSurfaceKHR` object, and provides
/// a function for destroying `VkSurfaceKHR` objects. Separate platform-specific
/// extensions each provide a function for creating a `VkSurfaceKHR` object for the
/// respective platform. From the application’s perspective this is an opaque
/// handle, just like the handles of other Vulkan objects.
///
#[cfg(feature = "VK_KHR_surface")]
pub type VkSurfaceKHR = VkNonDispatchableHandle<VkSurfaceKHR__>;

/// Structure describing capabilities of a surface
///
/// The `VkSurfaceCapabilitiesKHR` structure is defined as:
///
///   - `minImageCount` is the minimum number of images the specified device
///     supports for a swapchain created for the surface, and will be at least one.
///
///   - `maxImageCount` is the maximum number of images the specified device
///     supports for a swapchain created for the surface, and will be either 0, or
///     greater than or equal to `minImageCount`. A value of 0 means that there is
///     no limit on the number of images, though there may: be limits related to the
///     total amount of memory used by presentable images.
///
///   - `currentExtent` is the current width and height of the surface, or the
///     special value (0xFFFFFFFF, 0xFFFFFFFF) indicating that the surface size will
///     be determined by the extent of a swapchain targeting the surface.
///
///   - `minImageExtent` contains the smallest valid swapchain extent for the
///     surface on the specified device. The `width` and `height` of the extent will
///     each be less than or equal to the corresponding `width` and `height` of
///     `currentExtent`, unless `currentExtent` has the special value described
///     above.
///
///   - `maxImageExtent` contains the largest valid swapchain extent for the surface
///     on the specified device. The `width` and `height` of the extent will each be
///     greater than or equal to the corresponding `width` and `height` of
///     `minImageExtent`. The `width` and `height` of the extent will each be
///     greater than or equal to the corresponding `width` and `height` of
///     `currentExtent`, unless `currentExtent` has the special value described
///     above.
///
///   - `maxImageArrayLayers` is the maximum number of layers presentable images
///     can: have for a swapchain created for this device and surface, and will be
///     at least one.
///
///   - `supportedTransforms` is a bitmask of `VkSurfaceTransformFlagBitsKHR`
///     indicating the presentation transforms supported for the surface on the
///     specified device. At least one bit will be set.
///
///   - `currentTransform` is `VkSurfaceTransformFlagBitsKHR` value indicating the
///     surface’s current transform relative to the presentation engine’s natural
///     orientation.
///
///   - `supportedCompositeAlpha` is a bitmask of `VkCompositeAlphaFlagBitsKHR`,
///     representing the alpha compositing modes supported by the presentation
///     engine for the surface on the specified device, and at least one bit will be
///     set. Opaque composition can: be achieved in any alpha compositing mode by
///     either using an image format that has no alpha component, or by ensuring
///     that all pixels in the presentable images have an alpha value of 1.0.
///
///   - `supportedUsageFlags` is a bitmask of `VkImageUsageFlagBits` representing
///     the ways the application can: use the presentable images of a swapchain
///     created with `VkPresentModeKHR` set to `VK_PRESENT_MODE_IMMEDIATE_KHR`,
///     `VK_PRESENT_MODE_MAILBOX_KHR`, `VK_PRESENT_MODE_FIFO_KHR` or
///     `VK_PRESENT_MODE_FIFO_RELAXED_KHR` for the surface on the specified device.
///     `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` must: be included in the set but
///     implementations may: support additional usages.
///
/// > **Note**
/// >
/// > Supported usage flags of a presentable image when using
/// > `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
/// > `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` presentation mode are provided
/// > by `VkSharedPresentSurfaceCapabilitiesKHR::sharedPresentSupportedUsageFlags`.
///
/// > **Note**
/// >
/// > Formulas such as min(N, `maxImageCount`) are not correct, since
/// > `maxImageCount` may: be zero.
///
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
}
#[cfg(feature = "VK_KHR_surface")]
impl Default for VkSurfaceCapabilitiesKHR {
  fn default() -> VkSurfaceCapabilitiesKHR {
    VkSurfaceCapabilitiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_surface")]
impl RawStruct for VkSurfaceCapabilitiesKHR {
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
///
/// The `VkSurfaceFormatKHR` structure is defined as:
///
///   - `format` is a `VkFormat` that is compatible with the specified surface.
///
///   - `colorSpace` is a presentation `VkColorSpaceKHR` that is compatible with the
///     surface.
///
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
}
#[cfg(feature = "VK_KHR_surface")]
impl Default for VkSurfaceFormatKHR {
  fn default() -> VkSurfaceFormatKHR {
    VkSurfaceFormatKHR::new()
  }
}
#[cfg(feature = "VK_KHR_surface")]
impl RawStruct for VkSurfaceFormatKHR {
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
///
/// A swapchain object (a.k.a. swapchain) provides the ability to present rendering
/// results to a surface. Swapchain objects are represented by `VkSwapchainKHR`
/// handles:
///
/// A swapchain is an abstraction for an array of presentable images that are
/// associated with a surface. The presentable images are represented by `VkImage`
/// objects created by the platform. One image (which can: be an array image for
/// multiview/stereoscopic-3D surfaces) is displayed at a time, but multiple images
/// can: be queued for presentation. An application renders to the image, and then
/// queues the image for presentation to the surface.
///
/// A native window cannot: be associated with more than one swapchain at a time.
/// Further, swapchains cannot: be created for native windows that have a non-Vulkan
/// graphics API surface associated with them.
///
/// The presentation engine is an abstraction for the platform’s compositor or
/// hardware/software display engine.
///
/// > **Note**
/// >
/// > The presentation engine may: be synchronous or asynchronous with respect to
/// > the application and/or logical device.
/// >
/// > Some implementations may: use the device’s graphics queue or dedicated
/// > presentation hardware to perform presentation.
///
/// The presentable images of a swapchain are owned by the presentation engine. An
/// application can: acquire use of a presentable image from the presentation
/// engine. Use of a presentable image must: occur only after the image is returned
/// by `vkAcquireNextImageKHR`, and before it is presented by `vkQueuePresentKHR`.
/// This includes transitioning the image layout and rendering commands.
///
/// An application can: acquire use of a presentable image with
/// `vkAcquireNextImageKHR`. After acquiring a presentable image and before
/// modifying it, the application must: use a synchronization primitive to ensure
/// that the presentation engine has finished reading from the image. The
/// application can: then transition the image’s layout, queue rendering commands to
/// it, etc. Finally, the application presents the image with `vkQueuePresentKHR`,
/// which releases the acquisition of the image.
///
/// The presentation engine controls the order in which presentable images are
/// acquired for use by the application.
///
/// > **Note**
/// >
/// > This allows the platform to handle situations which require out-of-order
/// > return of images after presentation. At the same time, it allows the
/// > application to generate command buffers referencing all of the images in the
/// > swapchain at initialization time, rather than in its main loop.
///
#[cfg(feature = "VK_KHR_swapchain")]
pub type VkSwapchainKHR = VkNonDispatchableHandle<VkSwapchainKHR__>;

/// Structure specifying parameters of a newly created swapchain object
///
/// The `VkSwapchainCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkSwapchainCreateFlagBitsKHR` indicating parameters
///     of the swapchain creation.
///
///   - `surface` is the surface onto which the swapchain will present images. If
///     the creation succeeds, the swapchain becomes associated with `surface`.
///
///   - `minImageCount` is the minimum number of presentable images that the
///     application needs. The implementation will either create the swapchain with
///     at least that many images, or it will fail to create the swapchain.
///
///   - `imageFormat` is a `VkFormat` value specifying the format the swapchain
///     image(s) will be created with.
///
///   - `imageColorSpace` is a `VkColorSpaceKHR` value specifying the way the
///     swapchain interprets image data.
///
///   - `imageExtent` is the size (in pixels) of the swapchain image(s). The
///     behavior is platform-dependent if the image extent does not match the
///     surface’s `currentExtent` as returned by
///     `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`.
///
/// > **Note**
/// >
/// > On some platforms, it is normal that `maxImageExtent` may: become `(0,
/// > 0)`, for example when the window is minimized. In such a case, it is not
/// > possible to create a swapchain due to the Valid Usage requirements.
///
///   - `imageArrayLayers` is the number of views in a multiview/stereo surface. For
///     non-stereoscopic-3D applications, this value is 1.
///
///   - `imageUsage` is a bitmask of `VkImageUsageFlagBits` describing the intended
///     usage of the (acquired) swapchain images.
///
///   - `imageSharingMode` is the sharing mode used for the image(s) of the
///     swapchain.
///
///   - `queueFamilyIndexCount` is the number of queue families having access to the
///     image(s) of the swapchain when `imageSharingMode` is
///     `VK_SHARING_MODE_CONCURRENT`.
///
///   - `pQueueFamilyIndices` is an array of queue family indices having access to
///     the images(s) of the swapchain when `imageSharingMode` is
///     `VK_SHARING_MODE_CONCURRENT`.
///
///   - `preTransform` is a `VkSurfaceTransformFlagBitsKHR` value describing the
///     transform, relative to the presentation engine’s natural orientation,
///     applied to the image content prior to presentation. If it does not match the
///     `currentTransform` value returned by
///     `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`, the presentation engine will
///     transform the image content as part of the presentation operation.
///
///   - `compositeAlpha` is a `VkCompositeAlphaFlagBitsKHR` value indicating the
///     alpha compositing mode to use when this surface is composited together with
///     other surfaces on certain window systems.
///
///   - `presentMode` is the presentation mode the swapchain will use. A swapchain’s
///     present mode determines how incoming present requests will be processed and
///     queued internally.
///
///   - `clipped` indicates whether the Vulkan implementation is allowed to discard
///     rendering operations that affect regions of the surface that are not
///     visible.
///
///       - If set to `VK_TRUE`, the presentable images associated with the
///         swapchain may: not own all of their pixels. Pixels in the presentable
///         images that correspond to regions of the target surface obscured by
///         another window on the desktop, or subject to some other clipping
///         mechanism will have undefined content when read back. Pixel shaders may:
///         not execute for these pixels, and thus any side effects they would have
///         had will not occur. `VK_TRUE` value does not guarantee any clipping will
///         occur, but allows more optimal presentation methods to be used on some
///         platforms.
///
///       - If set to `VK_FALSE`, presentable images associated with the swapchain
///         will own all of the pixels they contain.
///
/// > **Note**
/// >
/// > Applications should: set this value to `VK_TRUE` if they do not expect to read
/// > back the content of presentable images before presenting them or after
/// > reacquiring them, and if their pixel shaders do not have any side effects that
/// > require them to run for all pixels in the presentable image.
///
///   - `oldSwapchain` is `VK_NULL_HANDLE`, or the existing non-retired swapchain
///     currently associated with `surface`. Providing a valid `oldSwapchain` may:
///     aid in the resource reuse, and also allows the application to still present
///     any images that are already acquired from it.
///
/// Upon calling `vkCreateSwapchainKHR` with an `oldSwapchain` that is not
/// `VK_NULL_HANDLE`, `oldSwapchain` is retired — even if creation of the new
/// swapchain fails. The new swapchain is created in the non-retired state whether
/// or not `oldSwapchain` is `VK_NULL_HANDLE`.
///
/// Upon calling `vkCreateSwapchainKHR` with an `oldSwapchain` that is not
/// `VK_NULL_HANDLE`, any images from `oldSwapchain` that are not acquired by the
/// application may: be freed by the implementation, which may: occur even if
/// creation of the new swapchain fails. The application can: destroy `oldSwapchain`
/// to free all memory associated with `oldSwapchain`.
///
/// > **Note**
/// >
/// > Multiple retired swapchains can: be associated with the same `VkSurfaceKHR`
/// > through multiple uses of `oldSwapchain` that outnumber calls to
/// > `vkDestroySwapchainKHR`.
/// >
/// > After `oldSwapchain` is retired, the application can: pass to
/// > `vkQueuePresentKHR` any images it had already acquired from `oldSwapchain`.
/// > E.g., an application may present an image from the old swapchain before an
/// > image from the new swapchain is ready to be presented. As usual,
/// > `vkQueuePresentKHR` may: fail if `oldSwapchain` has entered a state that
/// > causes `VK_ERROR_OUT_OF_DATE_KHR` to be returned.
/// >
/// > The application can: continue to use a shared presentable image obtained from
/// > `oldSwapchain` until a presentable image is acquired from the new swapchain,
/// > as long as it has not entered a state that causes it to return
/// > `VK_ERROR_OUT_OF_DATE_KHR`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'a> Default for VkSwapchainCreateInfoKHR<'a> {
  fn default() -> VkSwapchainCreateInfoKHR<'a> {
    VkSwapchainCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'a> RawStruct for VkSwapchainCreateInfoKHR<'a> {
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
///
/// The `VkPresentInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `waitSemaphoreCount` is the number of semaphores to wait for before issuing
///     the present request. The number may: be zero.
///
///   - `pWaitSemaphores`, if not `NULL`, is an array of `VkSemaphore` objects with
///     `waitSemaphoreCount` entries, and specifies the semaphores to wait for
///     before issuing the present request.
///
///   - `swapchainCount` is the number of swapchains being presented to by this
///     command.
///
///   - `pSwapchains` is an array of `VkSwapchainKHR` objects with `swapchainCount`
///     entries. A given swapchain must: not appear in this list more than once.
///
///   - `pImageIndices` is an array of indices into the array of each swapchain’s
///     presentable images, with `swapchainCount` entries. Each entry in this array
///     identifies the image to present on the corresponding entry in the
///     `pSwapchains` array.
///
///   - `pResults` is an array of `VkResult` typed elements with `swapchainCount`
///     entries. Applications that do not need per-swapchain results can: use `NULL`
///     for `pResults`. If non-`NULL`, each entry in `pResults` will be set to the
///     `VkResult` for presenting the swapchain corresponding to the same index in
///     `pSwapchains`.
///
/// Before an application can: present an image, the image’s layout must: be
/// transitioned to the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR` layout, or for a shared
/// presentable image the `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` layout.
///
/// > **Note**
/// >
/// > When transitioning the image to `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` or
/// > `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`, there is no need to delay subsequent
/// > processing, or perform any visibility operations (as `vkQueuePresentKHR`
/// > performs automatic visibility operations). To achieve this, the
/// > `dstAccessMask` member of the `VkImageMemoryBarrier` should: be set to `0`,
/// > and the `dstStageMask` parameter should: be set to
/// > `VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_wait_semaphores(mut self, value: &'a [VkSemaphore]) -> Self {
    unsafe {
      self.pWaitSemaphores = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_swapchains(mut self, value: &'a [VkSwapchainKHR]) -> Self {
    unsafe {
      self.pSwapchains = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_image_indices(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pImageIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_results(mut self, value: &'a mut [VkResult]) -> Self {
    unsafe {
      self.pResults = value.as_raw();
    }
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
impl<'a> RawStruct for VkPresentInfoKHR<'a> {
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
///
/// Displays are represented by `VkDisplayKHR` handles:
///
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayKHR = VkNonDispatchableHandle<VkDisplayKHR__>;

/// Structure describing an available display device
///
/// The `VkDisplayPropertiesKHR` structure is defined as:
///
///   - `display` is a handle that is used to refer to the display described here.
///     This handle will be valid for the lifetime of the Vulkan instance.
///
///   - `displayName` is a pointer to a NULL-terminated string containing the name
///     of the display. Generally, this will be the name provided by the display’s
///     EDID. It can: be `NULL` if no suitable name is available. If not `NULL`, the
///     memory it points to must: remain accessible as long as `display` is valid.
///
///   - `physicalDimensions` describes the physical width and height of the visible
///     portion of the display, in millimeters.
///
///   - `physicalResolution` describes the physical, native, or preferred resolution
///     of the display.
///
/// > **Note**
/// >
/// > For devices which have no natural value to return here, implementations
/// > should: return the maximum resolution supported.
///
///   - `supportedTransforms` tells which transforms are supported by this display.
///     This will contain one or more of the bits from `VkSurfaceTransformFlagsKHR`.
///
///   - `planeReorderPossible` tells whether the planes on this display can: have
///     their z order changed. If this is `VK_TRUE`, the application can: re-arrange
///     the planes on this display in any order relative to each other.
///
///   - `persistentContent` tells whether the display supports self-refresh/internal
///     buffering. If this is true, the application can: submit persistent present
///     operations on swapchains created against this display.
///
/// > **Note**
/// >
/// > Persistent presents may: have higher latency, and may: use less power when the
/// > screen content is updated infrequently, or when only a portion of the screen
/// > needs to be updated in most frames.
///
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
}
#[cfg(feature = "VK_KHR_display")]
impl<'a> Default for VkDisplayPropertiesKHR<'a> {
  fn default() -> VkDisplayPropertiesKHR<'a> {
    VkDisplayPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
impl<'a> RawStruct for VkDisplayPropertiesKHR<'a> {
  type Raw = types_raw::VkDisplayPropertiesKHR;
}
#[cfg(feature = "VK_KHR_display")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_properties_khr() {
  assert_size!(types_raw::VkDisplayPropertiesKHR, VkDisplayPropertiesKHR);
}

/// Structure describing display parameters associated with a display mode
///
/// The `VkDisplayModeParametersKHR` structure is defined as:
///
///   - `visibleRegion` is the 2D extents of the visible region.
///
///   - `refreshRate` is a `uint32_t` that is the number of times the display is
///     refreshed each second multiplied by 1000.
///
/// > **Note**
/// >
/// > For example, a 60Hz display mode would report a `refreshRate` of 60,000.
///
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
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayModeParametersKHR {
  fn default() -> VkDisplayModeParametersKHR {
    VkDisplayModeParametersKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
impl RawStruct for VkDisplayModeParametersKHR {
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
///
/// Display modes are represented by `VkDisplayModeKHR` handles:
///
#[cfg(feature = "VK_KHR_display")]
pub type VkDisplayModeKHR = VkNonDispatchableHandle<VkDisplayModeKHR__>;

/// Structure describing display mode properties
///
/// The `VkDisplayModePropertiesKHR` structure is defined as:
///
///   - `displayMode` is a handle to the display mode described in this structure.
///     This handle will be valid for the lifetime of the Vulkan instance.
///
///   - `parameters` is a `VkDisplayModeParametersKHR` structure describing the
///     display parameters associated with `displayMode`.
///
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
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayModePropertiesKHR {
  fn default() -> VkDisplayModePropertiesKHR {
    VkDisplayModePropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
impl RawStruct for VkDisplayModePropertiesKHR {
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
///
/// The `VkDisplayModeCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use, and must: be zero.
///
///   - `parameters` is a `VkDisplayModeParametersKHR` structure describing the
///     display parameters to use in creating the new mode. If the parameters are
///     not compatible with the specified display, the implementation must: return
///     `VK_ERROR_INITIALIZATION_FAILED`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayModeCreateInfoKHR {
  fn default() -> VkDisplayModeCreateInfoKHR {
    VkDisplayModeCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
impl RawStruct for VkDisplayModeCreateInfoKHR {
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
///
/// The `VkDisplayPlaneCapabilitiesKHR` structure is defined as:
///
///   - `supportedAlpha` is a bitmask of `VkDisplayPlaneAlphaFlagBitsKHR` describing
///     the supported alpha blending modes.
///
///   - `minSrcPosition` is the minimum source rectangle offset supported by this
///     plane using the specified mode.
///
///   - `maxSrcPosition` is the maximum source rectangle offset supported by this
///     plane using the specified mode. The `x` and `y` components of
///     `maxSrcPosition` must: each be greater than or equal to the `x` and `y`
///     components of `minSrcPosition`, respectively.
///
///   - `minSrcExtent` is the minimum source rectangle size supported by this plane
///     using the specified mode.
///
///   - `maxSrcExtent` is the maximum source rectangle size supported by this plane
///     using the specified mode.
///
///   - `minDstPosition`, `maxDstPosition`, `minDstExtent`, `maxDstExtent` all have
///     similar semantics to their corresponding ptext:\*Src\* equivalents, but
///     apply to the output region within the mode rather than the input region
///     within the source image. Unlike the ptext:\*Src\* offsets, `minDstPosition`
///     and `maxDstPosition` may: contain negative values.
///
/// The minimum and maximum position and extent fields describe the hardware limits,
/// if any, as they apply to the specified display mode and plane. Vendors may:
/// support displaying a subset of a swapchain’s presentable images on the specified
/// display plane. This is expressed by returning `minSrcPosition`,
/// `maxSrcPosition`, `minSrcExtent`, and `maxSrcExtent` values that indicate a
/// range of possible positions and sizes may: be used to specify the region within
/// the presentable images that source pixels will be read from when creating a
/// swapchain on the specified display mode and plane.
///
/// Vendors may: also support mapping the presentable images’ content to a subset or
/// superset of the visible region in the specified display mode. This is expressed
/// by returning `minDstPosition`, `maxDstPosition`, `minDstExtent` and
/// `maxDstExtent` values that indicate a range of possible positions and sizes may:
/// be used to describe the region within the display mode that the source pixels
/// will be mapped to.
///
/// Other vendors may: support only a 1-1 mapping between pixels in the presentable
/// images and the display mode. This may: be indicated by returning (0,0) for
/// `minSrcPosition`, `maxSrcPosition`, `minDstPosition`, and `maxDstPosition`, and
/// (display mode width, display mode height) for `minSrcExtent`, `maxSrcExtent`,
/// `minDstExtent`, and `maxDstExtent`.
///
/// These values indicate the limits of the hardware’s individual fields. Not all
/// combinations of values within the offset and extent ranges returned in
/// `VkDisplayPlaneCapabilitiesKHR` are guaranteed to be supported. Vendors may:
/// still fail presentation requests that specify unsupported combinations.
///
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
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayPlaneCapabilitiesKHR {
  fn default() -> VkDisplayPlaneCapabilitiesKHR {
    VkDisplayPlaneCapabilitiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
impl RawStruct for VkDisplayPlaneCapabilitiesKHR {
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
///
/// The `VkDisplayPlanePropertiesKHR` structure is defined as:
///
///   - `currentDisplay` is the handle of the display the plane is currently
///     associated with. If the plane is not currently attached to any displays,
///     this will be `VK_NULL_HANDLE`.
///
///   - `currentStackIndex` is the current z-order of the plane. This will be
///     between 0 and the value returned by
///     `vkGetPhysicalDeviceDisplayPlanePropertiesKHR` in `pPropertyCount`.
///
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
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplayPlanePropertiesKHR {
  fn default() -> VkDisplayPlanePropertiesKHR {
    VkDisplayPlanePropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
impl RawStruct for VkDisplayPlanePropertiesKHR {
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
///
/// The `VkDisplaySurfaceCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use, and must: be zero.
///
///   - `displayMode` is a `VkDisplayModeKHR` handle specifying the mode to use when
///     displaying this surface.
///
///   - `planeIndex` is the plane on which this surface appears.
///
///   - `planeStackIndex` is the z-order of the plane.
///
///   - `transform` is a `VkSurfaceTransformFlagBitsKHR` value specifying the
///     transformation to apply to images as part of the scanout operation.
///
///   - `globalAlpha` is the global alpha value. This value is ignored if
///     `alphaMode` is not `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`.
///
///   - `alphaMode` is a `VkDisplayPlaneAlphaFlagBitsKHR` value specifying the type
///     of alpha blending to use.
///
///   - `imageExtent` The size of the presentable images to use with the surface.
///
/// > **Note**
/// >
/// > Creating a display surface must: not modify the state of the displays, planes,
/// > or other resources it names. For example, it must: not apply the specified
/// > mode to be set on the associated display. Application of display configuration
/// > occurs as a side effect of presenting to a display surface.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_display")]
impl Default for VkDisplaySurfaceCreateInfoKHR {
  fn default() -> VkDisplaySurfaceCreateInfoKHR {
    VkDisplaySurfaceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display")]
impl RawStruct for VkDisplaySurfaceCreateInfoKHR {
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
///
/// The `VkDisplayPresentInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `srcRect` is a rectangular region of pixels to present. It must: be a subset
///     of the image being presented. If `VkDisplayPresentInfoKHR` is not specified,
///     this region will be assumed to be the entire presentable image.
///
///   - `dstRect` is a rectangular region within the visible region of the
///     swapchain’s display mode. If `VkDisplayPresentInfoKHR` is not specified,
///     this region will be assumed to be the entire visible region of the visible
///     region of the swapchain’s mode. If the specified rectangle is a subset of
///     the display mode’s visible region, content from display planes below the
///     swapchain’s plane will be visible outside the rectangle. If there are no
///     planes below the swapchain’s, the area outside the specified rectangle will
///     be black. If portions of the specified rectangle are outside of the
///     display’s visible region, pixels mapping only to those portions of the
///     rectangle will be discarded.
///
///   - `persistent`: If this is `VK_TRUE`, the display engine will enable buffered
///     mode on displays that support it. This allows the display engine to stop
///     sending content to the display until a new image is presented. The display
///     will instead maintain a copy of the last presented image. This allows less
///     power to be used, but may: increase presentation latency. If
///     `VkDisplayPresentInfoKHR` is not specified, persistent mode will not be
///     used.
///
/// If the extent of the `srcRect` and `dstRect` are not equal, the presented pixels
/// will be scaled accordingly.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl Default for VkDisplayPresentInfoKHR {
  fn default() -> VkDisplayPresentInfoKHR {
    VkDisplayPresentInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl RawStruct for VkDisplayPresentInfoKHR {
  type Raw = types_raw::VkDisplayPresentInfoKHR;
}
#[cfg(feature = "VK_KHR_display_swapchain")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_present_info_khr() {
  assert_size!(types_raw::VkDisplayPresentInfoKHR, VkDisplayPresentInfoKHR);
}

// feature: VK_KHR_xlib_surface

/// Structure specifying parameters of a newly created Xlib surface object
///
/// The `VkXlibSurfaceCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `dpy` is a pointer to an Xlib `Display` connection to the X server.
///
///   - `window` is an Xlib `Window` to associate the surface with.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkXlibSurfaceCreateInfoKHR {
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
///
/// The `VkXcbSurfaceCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `connection` is a pointer to an `xcb_connection_t` to the X server.
///
///   - `window` is the `xcb_window_t` for the X11 window to associate the surface
///     with.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkXcbSurfaceCreateInfoKHR {
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
///
/// The `VkWaylandSurfaceCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `display` and `surface` are pointers to the Wayland `wl_display` and
///     `wl_surface` to associate the surface with.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkWaylandSurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
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
impl RawStruct for VkWaylandSurfaceCreateInfoKHR {
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
///
/// The `VkMirSurfaceCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `connection` and `surface` are pointers to the `MirConnection` and
///     `MirSurface` for the window to associate the surface with.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkMirSurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
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
impl RawStruct for VkMirSurfaceCreateInfoKHR {
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
///
/// The `VkAndroidSurfaceCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `window` is a pointer to the `ANativeWindow` to associate the surface with.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkAndroidSurfaceCreateFlagsKHR) -> Self {
    self.flags = value;
    self
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
impl RawStruct for VkAndroidSurfaceCreateInfoKHR {
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
///
/// The `VkWin32SurfaceCreateInfoKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `hinstance` and `hwnd` are the Win32 `HINSTANCE` and `HWND` for the window
///     to associate the surface with.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkWin32SurfaceCreateInfoKHR {
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
///
/// The prototype for the `VkDebugReportCallbackCreateInfoEXT::pfnCallback` function
/// implemented by the application is.
///
///   - `flags` indicates the `VkDebugReportFlagBitsEXT` that triggered this
///     callback.
///
///   - `objectType` is a `VkDebugReportObjectTypeEXT` value specifying the type of
///     object being used or created at the time the event was triggered.
///
///   - `object` is the object where the issue was detected. If `objectType` is
///     `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`, `object` is undefined.
///
///   - `location` is a component (layer, driver, loader) defined value that
///     indicates the *location* of the trigger. This is an optional: value.
///
///   - `messageCode` is a layer-defined value indicating what test triggered this
///     callback.
///
///   - `pLayerPrefix` is a null-terminated string that is an abbreviation of the
///     name of the component making the callback. `pLayerPrefix` is only valid for
///     the duration of the callback.
///
///   - `pMessage` is a null-terminated string detailing the trigger conditions.
///     `pMessage` is only valid for the duration of the callback.
///
///   - `pUserData` is the user data given when the `VkDebugReportCallbackEXT` was
///     created.
///
/// The callback must: not call `vkDestroyDebugReportCallbackEXT`.
///
/// The callback returns a basetype:VkBool32, which is interpreted in a
/// layer-specified manner. The application should: always return `VK_FALSE`. The
/// `VK_TRUE` value is reserved for use in layer development.
///
/// `object` must: be a Vulkan object or `VK_NULL_HANDLE`. If `objectType` is not
/// `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT` and `object` is not `VK_NULL_HANDLE`,
/// `object` must: be a Vulkan object of the corresponding type associated with
/// `objectType` as defined in [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#debug-report-object-types).
///
#[cfg(feature = "VK_EXT_debug_report")]
pub use types_raw::PFN_vkDebugReportCallbackEXT;

/// Structure specifying parameters of a newly created debug report callback
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkDebugReportFlagBitsEXT` specifying which event(s)
///     will cause this callback to be called.
///
///   - `pfnCallback` is the application callback function to call.
///
///   - `pUserData` is user data to be passed to the callback.
///
/// For each `VkDebugReportCallbackEXT` that is created the
/// `VkDebugReportCallbackCreateInfoEXT::flags` determine when that
/// `VkDebugReportCallbackCreateInfoEXT::pfnCallback` is called. When an event
/// happens, the implementation will do a bitwise AND of the event’s
/// `VkDebugReportFlagBitsEXT` flags to each `VkDebugReportCallbackEXT` object’s
/// flags. For each non-zero result the corresponding callback will be called. The
/// callback will come directly from the component that detected the event, unless
/// some other layer intercepts the calls for its own purposes (filter them in a
/// different way, log to a system error log, etc.).
///
/// An application may: receive multiple callbacks if multiple
/// `VkDebugReportCallbackEXT` objects were created. A callback will always be
/// executed in the same thread as the originating Vulkan call.
///
/// A callback may be called from multiple threads simultaneously (if the
/// application is making Vulkan calls from multiple threads).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_debug_report")]
impl Default for VkDebugReportCallbackCreateInfoEXT {
  fn default() -> VkDebugReportCallbackCreateInfoEXT {
    VkDebugReportCallbackCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl RawStruct for VkDebugReportCallbackCreateInfoEXT {
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

/// Opaque handle to a debug report callback object
///
/// Debug report callbacks are represented by `VkDebugReportCallbackEXT` handles.
///
#[cfg(feature = "VK_EXT_debug_report")]
pub type VkDebugReportCallbackEXT = VkNonDispatchableHandle<VkDebugReportCallbackEXT__>;

// feature: VK_AMD_rasterization_order

/// Structure defining rasterization order for a graphics pipeline
///
/// The rasterization order to use for a graphics pipeline is specified by adding a
/// `VkPipelineRasterizationStateRasterizationOrderAMD` structure to the `pNext`
/// chain of a `VkPipelineRasterizationStateCreateInfo` structure.
///
/// The `VkPipelineRasterizationStateRasterizationOrderAMD` structure is defined as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `rasterizationOrder` is a `VkRasterizationOrderAMD` value specifying the
///     primitive rasterization order to use.
///
/// If the device extension is not enabled or the application does not request a
/// particular rasterization order through specifying a
/// `VkPipelineRasterizationStateRasterizationOrderAMD` structure then the
/// rasterization order used by the graphics pipeline defaults to
/// `VK_RASTERIZATION_ORDER_STRICT_AMD`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_rasterization_order(mut self, value: VkRasterizationOrderAMD) -> Self {
    self.rasterizationOrder = value;
    self
  }
}
#[cfg(feature = "VK_AMD_rasterization_order")]
impl Default for VkPipelineRasterizationStateRasterizationOrderAMD {
  fn default() -> VkPipelineRasterizationStateRasterizationOrderAMD {
    VkPipelineRasterizationStateRasterizationOrderAMD::new()
  }
}
#[cfg(feature = "VK_AMD_rasterization_order")]
impl RawStruct for VkPipelineRasterizationStateRasterizationOrderAMD {
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

/// Specify parameters of a name to give to an object
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `objectType` is a `VkDebugReportObjectTypeEXT` specifying the type of the
///     object to be named.
///
///   - `object` is the object to be named.
///
///   - `pObjectName` is a null-terminated UTF-8 string specifying the name to apply
///     to `object`.
///
/// Applications may: change the name associated with an object simply by calling
/// `vkDebugMarkerSetObjectNameEXT` again with a new string. To remove a previously
/// set name, `pObjectName` should: be set to an empty string.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'a> Default for VkDebugMarkerObjectNameInfoEXT<'a> {
  fn default() -> VkDebugMarkerObjectNameInfoEXT<'a> {
    VkDebugMarkerObjectNameInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'a> RawStruct for VkDebugMarkerObjectNameInfoEXT<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `objectType` is a `VkDebugReportObjectTypeEXT` specifying the type of the
///     object to be named.
///
///   - `object` is the object to be tagged.
///
///   - `tagName` is a numerical identifier of the tag.
///
///   - `tagSize` is the number of bytes of data to attach to the object.
///
///   - `pTag` is an array of `tagSize` bytes containing the data to be associated
///     with the object.
///
/// The `tagName` parameter gives a name or identifier to the type of data being
/// tagged. This can be used by debugging layers to easily filter for only data that
/// can be used by that implementation.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pTag = value.as_raw() as *const c_void;
    }
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
impl<'a> RawStruct for VkDebugMarkerObjectTagInfoEXT<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `pMarkerName` is a pointer to a null-terminated UTF-8 string that contains
///     the name of the marker.
///
///   - `color` is an optional: RGBA color value that can be associated with the
///     marker. A particular implementation may: choose to ignore this color value.
///     The values contain RGBA values in order, in the range 0.0 to 1.0. If all
///     elements in `color` are set to 0.0 then it is ignored.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'a> Default for VkDebugMarkerMarkerInfoEXT<'a> {
  fn default() -> VkDebugMarkerMarkerInfoEXT<'a> {
    VkDebugMarkerMarkerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_debug_marker")]
impl<'a> RawStruct for VkDebugMarkerMarkerInfoEXT<'a> {
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
///
/// If the `pNext` chain includes a `VkDedicatedAllocationImageCreateInfoNV`
/// structure, then that structure includes an enable controlling whether the image
/// will have a dedicated memory allocation bound to it.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `dedicatedAllocation` indicates whether the image will have a dedicated
///     allocation bound to it.
///
/// > **Note**
/// >
/// > Using a dedicated allocation for color and depth/stencil attachments or other
/// > large images may: improve performance on some devices.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_dedicated_allocation(mut self, value: VkBool32) -> Self {
    self.dedicatedAllocation = value;
    self
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl Default for VkDedicatedAllocationImageCreateInfoNV {
  fn default() -> VkDedicatedAllocationImageCreateInfoNV {
    VkDedicatedAllocationImageCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl RawStruct for VkDedicatedAllocationImageCreateInfoNV {
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

/// Specify that a buffer is bound to a dedicated memory resource
///
/// If the `pNext` chain includes a `VkDedicatedAllocationBufferCreateInfoNV`
/// structure, then that structure includes an enable controlling whether the buffer
/// will have a dedicated memory allocation bound to it.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `dedicatedAllocation` indicates whether the buffer will have a dedicated
///     allocation bound to it.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_dedicated_allocation(mut self, value: VkBool32) -> Self {
    self.dedicatedAllocation = value;
    self
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl Default for VkDedicatedAllocationBufferCreateInfoNV {
  fn default() -> VkDedicatedAllocationBufferCreateInfoNV {
    VkDedicatedAllocationBufferCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl RawStruct for VkDedicatedAllocationBufferCreateInfoNV {
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

/// Specify a dedicated memory allocation resource
///
/// If the `pNext` chain includes a `VkDedicatedAllocationMemoryAllocateInfoNV`
/// structure, then that structure includes a handle of the sole buffer or image
/// resource that the memory can: be bound to.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `image` is `VK_NULL_HANDLE` or a handle of an image which this memory will
///     be bound to.
///
///   - `buffer` is `VK_NULL_HANDLE` or a handle of a buffer which this memory will
///     be bound to.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl Default for VkDedicatedAllocationMemoryAllocateInfoNV {
  fn default() -> VkDedicatedAllocationMemoryAllocateInfoNV {
    VkDedicatedAllocationMemoryAllocateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl RawStruct for VkDedicatedAllocationMemoryAllocateInfoNV {
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

/// Structure describing the fine-grained features that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceFeatures2KHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `features` is a structure of type `VkPhysicalDeviceFeatures` describing the
///     fine-grained features of the Vulkan 1.0 API.
///
/// The `pNext` chain of this structure is used to extend the structure with
/// features defined by extensions. This structure can: be used in
/// `vkGetPhysicalDeviceFeatures2KHR` or can: be in the `pNext` chain of a
/// `VkDeviceCreateInfo` structure, in which case it controls which features are
/// enabled in the device in lieu of `pEnabledFeatures`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_features(mut self, value: VkPhysicalDeviceFeatures) -> Self {
    self.features = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceFeatures2KHR {
  fn default() -> VkPhysicalDeviceFeatures2KHR {
    VkPhysicalDeviceFeatures2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkPhysicalDeviceFeatures2KHR {
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

/// Structure specifying physical device properties
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `properties` is a structure of type `VkPhysicalDeviceProperties` describing
///     the properties of the physical device. This structure is written with the
///     same values as if it were written by `vkGetPhysicalDeviceProperties`.
///
/// The `pNext` chain of this structure is used to extend the structure with
/// properties defined by extensions.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_properties(mut self, value: VkPhysicalDeviceProperties) -> Self {
    self.properties = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceProperties2KHR {
  fn default() -> VkPhysicalDeviceProperties2KHR {
    VkPhysicalDeviceProperties2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkPhysicalDeviceProperties2KHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `formatProperties` is a structure of type `VkFormatProperties` describing
///     features supported by the requested format.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_format_properties(mut self, value: VkFormatProperties) -> Self {
    self.formatProperties = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkFormatProperties2KHR {
  fn default() -> VkFormatProperties2KHR {
    VkFormatProperties2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkFormatProperties2KHR {
  type Raw = types_raw::VkFormatProperties2KHR;
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_format_properties2_khr() {
  assert_size!(types_raw::VkFormatProperties2KHR, VkFormatProperties2KHR);
}

/// Structure specifying a image format properties
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure. The
///     `pNext` chain of `VkImageFormatProperties2KHR` is used to allow the
///     specification of additional capabilities to be returned from
///     `vkGetPhysicalDeviceImageFormatProperties2KHR`.
///
///   - `imageFormatProperties` is an instance of a `VkImageFormatProperties`
///     structure in which capabilities are returned.
///
/// If the combination of parameters to
/// `vkGetPhysicalDeviceImageFormatProperties2KHR` is not supported by the
/// implementation for use in `vkCreateImage`, then all members of
/// `imageFormatProperties` will be filled with zero.
///
/// > **Note**
/// >
/// > Filling `imageFormatProperties` with zero for unsupported formats is an
/// > exception to the usual rule that output structures have undefined contents on
/// > error. This exception was unintentional, but is preserved for backwards
/// > compatibility. This exeption only applies to `imageFormatProperties`, not
/// > `sType`, `pNext`, or any structures chained from `pNext`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_image_format_properties(mut self, value: VkImageFormatProperties) -> Self {
    self.imageFormatProperties = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkImageFormatProperties2KHR {
  fn default() -> VkImageFormatProperties2KHR {
    VkImageFormatProperties2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkImageFormatProperties2KHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure. The
///     `pNext` chain of `VkPhysicalDeviceImageFormatInfo2KHR` is used to provide
///     additional image parameters to
///     `vkGetPhysicalDeviceImageFormatProperties2KHR`.
///
///   - `format` is a `VkFormat` value indicating the image format, corresponding to
///     `VkImageCreateInfo::format`.
///
///   - `type` is a `VkImageType` value indicating the image type, corresponding to
///     `VkImageCreateInfo::imageType`.
///
///   - `tiling` is a `VkImageTiling` value indicating the image tiling,
///     corresponding to `VkImageCreateInfo::tiling`.
///
///   - `usage` is a bitmask of `VkImageUsageFlagBits` indicating the intended usage
///     of the image, corresponding to `VkImageCreateInfo::usage`.
///
///   - `flags` is a bitmask of `VkImageCreateFlagBits` indicating additional
///     parameters of the image, corresponding to `VkImageCreateInfo::flags`.
///
/// The members of `VkPhysicalDeviceImageFormatInfo2KHR` correspond to the arguments
/// to `vkGetPhysicalDeviceImageFormatProperties`, with `sType` and `pNext` added
/// for extensibility.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceImageFormatInfo2KHR {
  fn default() -> VkPhysicalDeviceImageFormatInfo2KHR {
    VkPhysicalDeviceImageFormatInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkPhysicalDeviceImageFormatInfo2KHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `queueFamilyProperties` is a structure of type `VkQueueFamilyProperties`
///     which is populated with the same values as in
///     `vkGetPhysicalDeviceQueueFamilyProperties`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_queue_family_properties(mut self, value: VkQueueFamilyProperties) -> Self {
    self.queueFamilyProperties = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkQueueFamilyProperties2KHR {
  fn default() -> VkQueueFamilyProperties2KHR {
    VkQueueFamilyProperties2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkQueueFamilyProperties2KHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `memoryProperties` is a structure of type `VkPhysicalDeviceMemoryProperties`
///     which is populated with the same values as in
///     `vkGetPhysicalDeviceMemoryProperties`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_memory_properties(mut self, value: VkPhysicalDeviceMemoryProperties) -> Self {
    self.memoryProperties = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceMemoryProperties2KHR {
  fn default() -> VkPhysicalDeviceMemoryProperties2KHR {
    VkPhysicalDeviceMemoryProperties2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkPhysicalDeviceMemoryProperties2KHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `properties` is a structure of type `VkSparseImageFormatProperties` which is
///     populated with the same values as in
///     `vkGetPhysicalDeviceSparseImageFormatProperties`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_properties(mut self, value: VkSparseImageFormatProperties) -> Self {
    self.properties = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkSparseImageFormatProperties2KHR {
  fn default() -> VkSparseImageFormatProperties2KHR {
    VkSparseImageFormatProperties2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkSparseImageFormatProperties2KHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `format` is the image format.
///
///   - `type` is the dimensionality of image.
///
///   - `samples` is the number of samples per texel as defined in
///     `VkSampleCountFlagBits`.
///
///   - `usage` is a bitmask describing the intended usage of the image.
///
///   - `tiling` is the tiling arrangement of the data elements in memory.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl Default for VkPhysicalDeviceSparseImageFormatInfo2KHR {
  fn default() -> VkPhysicalDeviceSparseImageFormatInfo2KHR {
    VkPhysicalDeviceSparseImageFormatInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
impl RawStruct for VkPhysicalDeviceSparseImageFormatInfo2KHR {
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
///
/// To determine if texture gather functions that take explicit LOD and/or bias
/// argument values can be used with a given image format, add
/// `VkImageFormatProperties2KHR` to the `pNext` chain of the
/// `VkPhysicalDeviceImageFormatInfo2KHR` structure and
/// `VkTextureLODGatherFormatPropertiesAMD` to the `pNext` chain of the
/// `VkImageFormatProperties2KHR` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL`.
///
///   - `supportsTextureGatherLODBiasAMD` tells if the image format can be used with
///     texture gather bias/LOD functions, as introduced by the  extension. This
///     field is set by the implementation. User-specified value is ignored.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_supports_texture_gather_lod_bias_amd(mut self, value: VkBool32) -> Self {
    self.supportsTextureGatherLODBiasAMD = value;
    self
  }
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
impl Default for VkTextureLODGatherFormatPropertiesAMD {
  fn default() -> VkTextureLODGatherFormatPropertiesAMD {
    VkTextureLODGatherFormatPropertiesAMD::new()
  }
}
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
impl RawStruct for VkTextureLODGatherFormatPropertiesAMD {
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

/// Resource usage information about a particular shader within a pipeline
///
///   - `numUsedVgprs` is the number of vector instruction general-purpose registers
///     used by this shader.
///
///   - `numUsedSgprs` is the number of scalar instruction general-purpose registers
///     used by this shader.
///
///   - `ldsSizePerLocalWorkGroup` is the maximum local data store size per work
///     group in bytes.
///
///   - `ldsUsageSizeInBytes` is the LDS usage size in bytes per work group by this
///     shader.
///
///   - `scratchMemUsageInBytes` is the scratch memory usage in bytes by this
///     shader.
///
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
}
#[cfg(feature = "VK_AMD_shader_info")]
impl Default for VkShaderResourceUsageAMD {
  fn default() -> VkShaderResourceUsageAMD {
    VkShaderResourceUsageAMD::new()
  }
}
#[cfg(feature = "VK_AMD_shader_info")]
impl RawStruct for VkShaderResourceUsageAMD {
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
///
///   - `shaderStageMask` are the combination of logical shader stages contained
///     within this shader.
///
///   - `resourceUsage` is an instance of `VkShaderResourceUsageAMD` describing
///     internal physical device resources used by this shader.
///
///   - `numPhysicalVgprs` is the maximum number of vector instruction
///     general-purpose registers (VGPRs) available to the physical device.
///
///   - `numPhysicalSgprs` is the maximum number of scalar instruction
///     general-purpose registers (SGPRs) available to the physical device.
///
///   - `numAvailableVgprs` is the maximum limit of VGPRs made available to the
///     shader compiler.
///
///   - `numAvailableSgprs` is the maximum limit of SGPRs made available to the
///     shader compiler.
///
///   - `computeWorkGroupSize` is the local workgroup size of this shader in { X, Y,
///     Z } dimensions.
///
/// Some implementations may merge multiple logical shader stages together in a
/// single shader. In such cases, `shaderStageMask` will contain a bitmask of all of
/// the stages that are active within that shader. Consequently, if specifying those
/// stages as input to `vkGetShaderInfoAMD`, the same output information may: be
/// returned for all such shader stage queries.
///
/// The number of available VGPRs and SGPRs (`numAvailableVgprs` and
/// `numAvailableSgprs` respectively) are the shader-addressable subset of physical
/// registers that is given as a limit to the compiler for register assignment.
/// These values may: further be limited by implementations due to performance
/// optimizations where register pressure is a bottleneck.
///
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
}
#[cfg(feature = "VK_AMD_shader_info")]
impl Default for VkShaderStatisticsInfoAMD {
  fn default() -> VkShaderStatisticsInfoAMD {
    VkShaderStatisticsInfoAMD::new()
  }
}
#[cfg(feature = "VK_AMD_shader_info")]
impl RawStruct for VkShaderStatisticsInfoAMD {
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
///
/// If the `VkRenderPassCreateInfo::pNext` chain includes a
/// `VkRenderPassMultiviewCreateInfoKHX` structure, then that structure includes an
/// array of view masks, view offsets, and correlation masks for the render pass.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `subpassCount` is zero or is the number of subpasses in the render pass.
///
///   - `pViewMasks` points to an array of `subpassCount` number of view masks,
///     where each mask is a bitfield of view indices describing which views
///     rendering is broadcast to in each subpass, when multiview is enabled. If
///     `subpassCount` is zero, each view mask is treated as zero.
///
///   - `dependencyCount` is zero or the number of dependencies in the render pass.
///
///   - `pViewOffsets` points to an array of `dependencyCount` view offsets, one for
///     each dependency. If `dependencyCount` is zero, each dependency’s view offset
///     is treated as zero. Each view offset controls which views in the source
///     subpass the views in the destination subpass depend on.
///
///   - `correlationMaskCount` is zero or a number of correlation masks.
///
///   - `pCorrelationMasks` is an array of view masks indicating sets of views that
///     may: be more efficient to render concurrently.
///
/// When a subpass uses a non-zero view mask, *multiview* functionality is
/// considered to be enabled. Multiview is all-or-nothing for a render pass - that
/// is, either all subpasses must: have a non-zero view mask (though some subpasses
/// may: have only one view) or all must: be zero. Multiview causes all drawing and
/// clear commands in the subpass to behave as if they were broadcast to each view,
/// where a view is represented by one layer of the framebuffer attachments. All
/// draws and clears are broadcast to each *view index* whose bit is set in the view
/// mask. The view index is provided in the `ViewIndex` shader input variable, and
/// color, depth/stencil, and input attachments all read/write the layer of the
/// framebuffer corresponding to the view index.
///
/// If the view mask is zero for all subpasses, multiview is considered to be
/// disabled and all drawing commands execute normally, without this additional
/// broadcasting.
///
/// Some implementations may: not support multiview in conjunction with [geometry
/// shaders](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-multiview-gs) or [tessellation
/// shaders](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-multiview-tess).
///
/// When multiview is enabled, the `VK_DEPENDENCY_VIEW_LOCAL_BIT_KHX` bit in a
/// dependency can: be used to express a view-local dependency, meaning that each
/// view in the destination subpass depends on a single view in the source subpass.
/// Unlike pipeline barriers, a subpass dependency can: potentially have a different
/// view mask in the source subpass and the destination subpass. If the dependency
/// is view-local, then each view (dstView) in the destination subpass depends on
/// the view dstView + pViewOffsets\[dependency\] in the source subpass. If there is
/// not such a view in the source subpass, then this dependency does not affect that
/// view in the destination subpass. If the dependency is not view-local, then all
/// views in the destination subpass depend on all views in the source subpass, and
/// the view offset is ignored. A non-zero view offset is not allowed in a
/// self-dependency.
///
/// The elements of `pCorrelationMasks` are a set of masks of views indicating that
/// views in the same mask may: exhibit spatial coherency between the views, making
/// it more efficient to render them concurrently. Correlation masks must: not have
/// a functional effect on the results of the multiview rendering.
///
/// When multiview is enabled, at the beginning of each subpass all non-render pass
/// state is undefined. In particular, each time `vkCmdBeginRenderPass` or
/// `vkCmdNextSubpass` is called the graphics pipeline must: be bound, any relevant
/// descriptor sets or vertex/index buffers must: be bound, and any relevant dynamic
/// state or push constants must: be set before they are used.
///
/// A multiview subpass can: declare that its shaders will write per-view attributes
/// for all views in a single invocation, by setting the
/// `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` bit in the subpass
/// description. The only supported per-view attributes are position and viewport
/// mask, and per-view position and viewport masks are written to output array
/// variables decorated with `PositionPerViewNV` and `ViewportMaskPerViewNV`,
/// respectively. If is not supported and enabled, `ViewportMaskPerViewNV` must: not
/// be used. Values written to elements of `PositionPerViewNV` and
/// `ViewportMaskPerViewNV` must: not depend on the `ViewIndex`. The shader must:
/// also write to an output variable decorated with `Position`, and the value
/// written to `Position` must: equal the value written to
/// `PositionPerViewNV`\[`ViewIndex`\]. Similarly, if `ViewportMaskPerViewNV` is
/// written to then the shader must: also write to an output variable decorated with
/// `ViewportMaskNV`, and the value written to `ViewportMaskNV` must: equal the
/// value written to `ViewportMaskPerViewNV`\[`ViewIndex`\]. Implementations will
/// either use values taken from `Position` and `ViewportMaskNV` and invoke the
/// shader once for each view, or will use values taken from `PositionPerViewNV` and
/// `ViewportMaskPerViewNV` and invoke the shader fewer times. The values written to
/// `Position` and `ViewportMaskNV` must: not depend on the values written to
/// `PositionPerViewNV` and `ViewportMaskPerViewNV`, or vice versa (to allow
/// compilers to eliminate the unused outputs). All attributes that do not have
/// \*PerViewNV counterparts must: not depend on `ViewIndex`.
///
/// Per-view attributes are all-or-nothing for a subpass. That is, all pipelines
/// compiled against a subpass that includes the
/// `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` bit must: write per-view
/// attributes to the \*PerViewNV\[\] shader outputs, in addition to the
/// non-per-view (e.g. `Position`) outputs. Pipelines compiled against a subpass
/// that does not include this bit must: not include the \*PerViewNV\[\] outputs in
/// their interfaces.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_view_masks(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pViewMasks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_view_offsets(mut self, value: &'a [i32]) -> Self {
    unsafe {
      self.pViewOffsets = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_correlation_masks(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pCorrelationMasks = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHX_multiview")]
impl<'a> Default for VkRenderPassMultiviewCreateInfoKHX<'a> {
  fn default() -> VkRenderPassMultiviewCreateInfoKHX<'a> {
    VkRenderPassMultiviewCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_multiview")]
impl<'a> RawStruct for VkRenderPassMultiviewCreateInfoKHX<'a> {
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

/// Structure describing multiview features that can be supported by an
/// implementation
///
/// The members of the `VkPhysicalDeviceMultiviewFeaturesKHX` structure describe the
/// following features:
///
///   - `multiview` indicates whether the implementation supports multiview
///     rendering within a render pass. If this feature is not enabled, the view
///     mask of each subpass must: always be zero.
///
///   - `multiviewGeometryShader` indicates whether the implementation supports
///     multiview rendering within a render pass, with [geometry
///     shaders](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#geometry). If this feature is not enabled, then a pipeline
///     compiled against a subpass with a non-zero view mask must: not include a
///     geometry shader.
///
///   - `multiviewTessellationShader` indicates whether the implementation supports
///     multiview rendering within a render pass, with [tessellation
///     shaders](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#tessellation). If this feature is not enabled, then a pipeline
///     compiled against a subpass with a non-zero view mask must: not include any
///     tessellation shaders.
///
/// If the `VkPhysicalDeviceMultiviewFeaturesKHX` structure is included in the
/// `pNext` chain of `VkPhysicalDeviceFeatures2KHR`, it is filled with values
/// indicating whether each feature is supported.
/// `VkPhysicalDeviceMultiviewFeaturesKHX` can: also be used in the `pNext` chain of
/// `VkDeviceCreateInfo` to enable the features.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHX_multiview")]
impl Default for VkPhysicalDeviceMultiviewFeaturesKHX {
  fn default() -> VkPhysicalDeviceMultiviewFeaturesKHX {
    VkPhysicalDeviceMultiviewFeaturesKHX::new()
  }
}
#[cfg(feature = "VK_KHX_multiview")]
impl RawStruct for VkPhysicalDeviceMultiviewFeaturesKHX {
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

/// Structure describing multiview limits that can be supported by an implementation
///
/// The members of the `VkPhysicalDeviceMultiviewPropertiesKHX` structure describe
/// the following implementation-dependent limits:
///
///   - `maxMultiviewViewCount` is one greater than the maximum view index that can:
///     be used in a subpass.
///
///   - `maxMultiviewInstanceIndex` is the maximum valid value of instance index
///     allowed to be generated by a drawing command recorded within a subpass of a
///     multiview render pass instance.
///
/// If the `VkPhysicalDeviceMultiviewPropertiesKHX` structure is included in the
/// `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled with the
/// implementation-dependent limits.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHX_multiview")]
impl Default for VkPhysicalDeviceMultiviewPropertiesKHX {
  fn default() -> VkPhysicalDeviceMultiviewPropertiesKHX {
    VkPhysicalDeviceMultiviewPropertiesKHX::new()
  }
}
#[cfg(feature = "VK_KHX_multiview")]
impl RawStruct for VkPhysicalDeviceMultiviewPropertiesKHX {
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

/// Structure specifying external image format properties
///
/// The `VkExternalImageFormatPropertiesNV` structure is defined as:
///
///   - `imageFormatProperties` will be filled in as when calling
///     `vkGetPhysicalDeviceImageFormatProperties`, but the values returned may:
///     vary depending on the external handle type requested.
///
///   - `externalMemoryFeatures` is a bitmask of
///     `VkExternalMemoryFeatureFlagBitsNV`, indicating properties of the external
///     memory handle type
///     (`vkGetPhysicalDeviceExternalImageFormatPropertiesNV::externalHandleType`)
///     being queried, or 0 if the external memory handle type is 0.
///
///   - `exportFromImportedHandleTypes` is a bitmask of
///     `VkExternalMemoryHandleTypeFlagBitsNV` containing a bit set for every
///     external handle type that may: be used to create memory from which the
///     handles of the type specified in
///     `vkGetPhysicalDeviceExternalImageFormatPropertiesNV::externalHandleType`
///     can: be exported, or 0 if the external memory handle type is 0.
///
///   - `compatibleHandleTypes` is a bitmask of
///     `VkExternalMemoryHandleTypeFlagBitsNV` containing a bit set for every
///     external handle type that may: be specified simultaneously with the handle
///     type specified by
///     `vkGetPhysicalDeviceExternalImageFormatPropertiesNV::externalHandleType`
///     when calling `vkAllocateMemory`, or 0 if the external memory handle type is
///     0. `compatibleHandleTypes` will always contain
///     `vkGetPhysicalDeviceExternalImageFormatPropertiesNV::externalHandleType`
///
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
}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
impl Default for VkExternalImageFormatPropertiesNV {
  fn default() -> VkExternalImageFormatPropertiesNV {
    VkExternalImageFormatPropertiesNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
impl RawStruct for VkExternalImageFormatPropertiesNV {
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
///
/// If the `pNext` chain includes a `VkExternalMemoryImageCreateInfoNV` structure,
/// then that structure defines a set of external memory handle types that may: be
/// used as backing store for the image.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleTypes` is a bitmask of `VkExternalMemoryHandleTypeFlagBitsNV`
///     specifying one or more external memory handle types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsNV) -> Self {
    self.handleTypes = value;
    self
  }
}
#[cfg(feature = "VK_NV_external_memory")]
impl Default for VkExternalMemoryImageCreateInfoNV {
  fn default() -> VkExternalMemoryImageCreateInfoNV {
    VkExternalMemoryImageCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory")]
impl RawStruct for VkExternalMemoryImageCreateInfoNV {
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

/// Specify memory handle types that may be exported
///
/// The `VkExportMemoryAllocateInfoNV` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleTypes` is a bitmask of `VkExternalMemoryHandleTypeFlagBitsNV`
///     specifying one or more memory handle types that may: be exported. Multiple
///     handle types may: be requested for the same allocation as long as they are
///     compatible, as reported by
///     `vkGetPhysicalDeviceExternalImageFormatPropertiesNV`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsNV) -> Self {
    self.handleTypes = value;
    self
  }
}
#[cfg(feature = "VK_NV_external_memory")]
impl Default for VkExportMemoryAllocateInfoNV {
  fn default() -> VkExportMemoryAllocateInfoNV {
    VkExportMemoryAllocateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_external_memory")]
impl RawStruct for VkExportMemoryAllocateInfoNV {
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

/// import Win32 memory created on the same physical device
///
/// To import memory created on the same physical device but outside of the current
/// Vulkan instance, add a `VkImportMemoryWin32HandleInfoNV` structure to the
/// `pNext` chain of the `VkMemoryAllocateInfo` structure, specifying a handle to
/// and the type of the memory.
///
/// The `VkImportMemoryWin32HandleInfoNV` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleType` is `0` or a `VkExternalMemoryHandleTypeFlagBitsNV` value
///     specifying the type of memory handle in `handle`.
///
///   - `handle` is a Windows `HANDLE` referring to the memory.
///
/// If `handleType` is `0`, this structure is ignored by consumers of the
/// `VkMemoryAllocateInfo` structure it is chained from.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkImportMemoryWin32HandleInfoNV {
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

/// specify security attributes and access rights for Win32 memory handles
///
/// When `VkExportMemoryAllocateInfoNV::handleTypes` includes
/// `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV`, add a
/// `VkExportMemoryWin32HandleInfoNV` to the `pNext` chain of the
/// `VkExportMemoryAllocateInfoNV` structure to specify security attributes and
/// access rights for the memory object’s external handle.
///
/// The `VkExportMemoryWin32HandleInfoNV` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `pAttributes` is a pointer to a Windows `SECURITY_ATTRIBUTES` structure
///     specifying security attributes of the handle.
///
///   - `dwAccess` is a `DWORD` specifying access rights of the handle.
///
/// If this structure is not present, or if `pAttributes` is set to `NULL`, default
/// security descriptor values will be used, and child processes created by the
/// application will not inherit the handle, as described in the MSDN documentation
/// for “Synchronization Object Security and Access Rights”\[1\]. Further, if the
/// structure is not present, the access rights will be
///
///     DXGI_SHARED_RESOURCE_READ | DXGI_SHARED_RESOURCE_WRITE
///
/// \[1\]
/// [https://msdn.microsoft.com/en-us/library/windows/desktop/ms686670.aspx](#)
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_dw_access(mut self, value: wsi::win32::DWORD) -> Self {
    self.dwAccess = value;
    self
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
impl RawStruct for VkExportMemoryWin32HandleInfoNV {
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

/// use Windows keyex mutex mechanism to synchronize work
///
/// When submitting work that operates on memory imported from a Direct3D 11
/// resource to a queue, the keyed mutex mechanism may: be used in addition to
/// Vulkan semaphores to synchronize the work. Keyed mutexes are a property of a
/// properly created shareable Direct3D 11 resource. They can: only be used if the
/// imported resource was created with the
/// etext:D3D11\_RESOURCE\_MISC\_SHARED\_KEYEDMUTEX flag.
///
/// To acquire keyed mutexes before submitted work and/or release them after, add a
/// `VkWin32KeyedMutexAcquireReleaseInfoNV` structure to the `pNext` chain of the
/// `VkSubmitInfo` structure.
///
/// The `VkWin32KeyedMutexAcquireReleaseInfoNV` structure is defined as:
///
///   - `acquireCount` is the number of entries in the `pAcquireSyncs`,
///     `pAcquireKeys`, and `pAcquireTimeoutMilliseconds` arrays.
///
///   - `pAcquireSyncs` is a pointer to an array of `VkDeviceMemory` objects which
///     were imported from Direct3D 11 resources.
///
///   - `pAcquireKeys` is a pointer to an array of mutex key values to wait for
///     prior to beginning the submitted work. Entries refer to the keyed mutex
///     associated with the corresponding entries in `pAcquireSyncs`.
///
///   - `pAcquireTimeoutMilliseconds` is an array of timeout values, in millisecond
///     units, for each acquire specified in `pAcquireKeys`.
///
///   - `releaseCount` is the number of entries in the `pReleaseSyncs` and
///     `pReleaseKeys` arrays.
///
///   - `pReleaseSyncs` is a pointer to an array of `VkDeviceMemory` objects which
///     were imported from Direct3D 11 resources.
///
///   - `pReleaseKeys` is a pointer to an array of mutex key values to set when the
///     submitted work has completed. Entries refer to the keyed mutex associated
///     with the corresponding entries in `pReleaseSyncs`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_acquire_syncs(mut self, value: &'a [VkDeviceMemory]) -> Self {
    unsafe {
      self.pAcquireSyncs = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_acquire_keys(mut self, value: &'a [u64]) -> Self {
    unsafe {
      self.pAcquireKeys = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_acquire_timeout_milliseconds(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pAcquireTimeoutMilliseconds = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_release_syncs(mut self, value: &'a [VkDeviceMemory]) -> Self {
    unsafe {
      self.pReleaseSyncs = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_release_keys(mut self, value: &'a [u64]) -> Self {
    unsafe {
      self.pReleaseKeys = value.as_raw();
    }
    self
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
impl<'a> RawStruct for VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
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

/// Structure specifying physical device group properties
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `physicalDeviceCount` is the number of physical devices in the group.
///
///   - `physicalDevices` is an array of physical device handles representing all
///     physical devices in the group. The first `physicalDeviceCount` elements of
///     the array will be valid.
///
///   - `subsetAllocation` indicates whether logical devices created from the group
///     support allocating device memory on a subset of devices, via the
///     `deviceMask` member of the `VkMemoryAllocateFlagsInfoKHX`. If this is
///     `VK_FALSE`, then all device memory allocations are made across all physical
///     devices in the group. If `physicalDeviceCount` is `1`, then
///     `subsetAllocation` must: be `VK_FALSE`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl Default for VkPhysicalDeviceGroupPropertiesKHX {
  fn default() -> VkPhysicalDeviceGroupPropertiesKHX {
    VkPhysicalDeviceGroupPropertiesKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl RawStruct for VkPhysicalDeviceGroupPropertiesKHX {
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
///
/// A logical device can: be created that connects to one or more physical devices
/// by including a `VkDeviceGroupDeviceCreateInfoKHX` structure in the `pNext` chain
/// of `VkDeviceCreateInfo`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `physicalDeviceCount` is the number of elements in the `pPhysicalDevices`
///     array.
///
///   - `pPhysicalDevices` is an array of physical device handles belonging to the
///     same device group.
///
/// The elements of the `pPhysicalDevices` array are an ordered list of the physical
/// devices that the logical device represents. These must: be a subset of a single
/// device group, and need not be in the same order as they were enumerated. The
/// order of the physical devices in the `pPhysicalDevices` array determines the
/// *device index* of each physical device, with element i being assigned a device
/// index of i. Certain commands and structures refer to one or more physical
/// devices by using device indices or *device masks* formed using device indices.
///
/// A logical device created without using `VkDeviceGroupDeviceCreateInfoKHX`, or
/// with `physicalDeviceCount` equal to zero, is equivalent to a
/// `physicalDeviceCount` of one and `pPhysicalDevices` pointing to the
/// `physicalDevice` parameter to `vkCreateDevice`. In particular, the device index
/// of that physical device is zero.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_physical_devices(mut self, value: &'a [VkPhysicalDevice]) -> Self {
    unsafe {
      self.pPhysicalDevices = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl<'a> Default for VkDeviceGroupDeviceCreateInfoKHX<'a> {
  fn default() -> VkDeviceGroupDeviceCreateInfoKHX<'a> {
    VkDeviceGroupDeviceCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group_creation")]
impl<'a> RawStruct for VkDeviceGroupDeviceCreateInfoKHX<'a> {
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkMemoryAllocateFlagsInfoKHX {
  fn default() -> VkMemoryAllocateFlagsInfoKHX {
    VkMemoryAllocateFlagsInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl RawStruct for VkMemoryAllocateFlagsInfoKHX {
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

/// Set the initial device mask and render areas for a render pass instance
///
/// If the `pNext` chain of `VkRenderPassBeginInfo` includes a
/// `VkDeviceGroupRenderPassBeginInfoKHX` structure, then that structure includes a
/// device mask and set of render areas for the render pass instance.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `deviceMask` is the device mask for the render pass instance.
///
///   - `deviceRenderAreaCount` is the number of elements in the
///     `pDeviceRenderAreas` array.
///
///   - `pDeviceRenderAreas` is an array of structures of type `VkRect2D` defining
///     the render area for each physical device.
///
/// The `deviceMask` serves several purposes. It is an upper bound on the set of
/// physical devices that can: be used during the render pass instance, and the
/// initial device mask when the render pass instance begins. Render pass attachment
/// load, store, and resolve operations only apply to physical devices included in
/// the device mask. Subpass dependencies only apply to the physical devices in the
/// device mask.
///
/// If `deviceRenderAreaCount` is not zero, then the elements of
/// `pDeviceRenderAreas` override the value of `VkRenderPassBeginInfo::renderArea`,
/// and provide a render area specific to each physical device. These render areas
/// serve the same purpose as `VkRenderPassBeginInfo::renderArea`, including
/// controlling the region of attachments that are cleared by
/// `VK_ATTACHMENT_LOAD_OP_CLEAR` and that are resolved into resolve attachments.
///
/// If this structure is not present, the render pass instance’s device mask is the
/// value of `VkDeviceGroupCommandBufferBeginInfoKHX::deviceMask`. If this structure
/// is not present or if `deviceRenderAreaCount` is zero,
/// `VkRenderPassBeginInfo::renderArea` is used for all physical devices.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_device_mask(mut self, value: u32) -> Self {
    self.deviceMask = value;
    self
  }
  #[inline]
  pub fn set_device_render_areas(mut self, value: &'a [VkRect2D]) -> Self {
    unsafe {
      self.pDeviceRenderAreas = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> Default for VkDeviceGroupRenderPassBeginInfoKHX<'a> {
  fn default() -> VkDeviceGroupRenderPassBeginInfoKHX<'a> {
    VkDeviceGroupRenderPassBeginInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> RawStruct for VkDeviceGroupRenderPassBeginInfoKHX<'a> {
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

/// Set the initial device mask for a command buffer
///
/// If the `pNext` chain of `VkCommandBufferBeginInfo` includes a
/// `VkDeviceGroupCommandBufferBeginInfoKHX` structure, then that structure includes
/// an initial device mask for the command buffer.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `deviceMask` is the initial value of the command buffer’s device mask.
///
/// The initial device mask also acts as an upper bound on the set of devices that
/// can: ever be in the device mask in the command buffer.
///
/// If this structure is not present, the initial value of a command buffer’s device
/// mask is set to include all physical devices in the logical device when the
/// command buffer begins recording.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_device_mask(mut self, value: u32) -> Self {
    self.deviceMask = value;
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkDeviceGroupCommandBufferBeginInfoKHX {
  fn default() -> VkDeviceGroupCommandBufferBeginInfoKHX {
    VkDeviceGroupCommandBufferBeginInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl RawStruct for VkDeviceGroupCommandBufferBeginInfoKHX {
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

/// Structure indicating which physical devices execute semaphore operations and
/// command buffers
///
/// If the `pNext` chain of `VkSubmitInfo` includes a `VkDeviceGroupSubmitInfoKHX`
/// structure, then that structure includes device indices and masks specifying
/// which physical devices execute semaphore operations and command buffers.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `waitSemaphoreCount` is the number of elements in the
///     `pWaitSemaphoreDeviceIndices` array.
///
///   - `pWaitSemaphoreDeviceIndices` is an array of device indices indicating which
///     physical device executes the semaphore wait operation in the corresponding
///     element of `VkSubmitInfo::pWaitSemaphores`.
///
///   - `commandBufferCount` is the number of elements in the
///     `pCommandBufferDeviceMasks` array.
///
///   - `pCommandBufferDeviceMasks` is an array of device masks indicating which
///     physical devices execute the command buffer in the corresponding element of
///     `VkSubmitInfo::pCommandBuffers`. A physical device executes the command
///     buffer if the corresponding bit is set in the mask.
///
///   - `signalSemaphoreCount` is the number of elements in the
///     `pSignalSemaphoreDeviceIndices` array.
///
///   - `pSignalSemaphoreDeviceIndices` is an array of device indices indicating
///     which physical device executes the semaphore signal operation in the
///     corresponding element of `VkSubmitInfo::pSignalSemaphores`.
///
/// If this structure is not present, semaphore operations and command buffers
/// execute on device index zero.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_wait_semaphore_device_indices(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pWaitSemaphoreDeviceIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_command_buffer_device_masks(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pCommandBufferDeviceMasks = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphore_device_indices(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pSignalSemaphoreDeviceIndices = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> Default for VkDeviceGroupSubmitInfoKHX<'a> {
  fn default() -> VkDeviceGroupSubmitInfoKHX<'a> {
    VkDeviceGroupSubmitInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> RawStruct for VkDeviceGroupSubmitInfoKHX<'a> {
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

/// Structure indicating which instances are bound
///
/// If the `pNext` chain of `VkBindSparseInfo` includes a
/// `VkDeviceGroupBindSparseInfoKHX` structure, then that structure includes device
/// indices specifying which instance of the resources and memory are bound.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `resourceDeviceIndex` is a device index indicating which instance of the
///     resource is bound.
///
///   - `memoryDeviceIndex` is a device index indicating which instance of the
///     memory the resource instance is bound to.
///
/// These device indices apply to all buffer and image memory binds included in the
/// batch that points to this structure. The semaphore waits and signals for the
/// batch are executed only by the physical device specified by the
/// `resourceDeviceIndex`.
///
/// If this structure is not present, `resourceDeviceIndex` and `memoryDeviceIndex`
/// are assumed to be zero.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkDeviceGroupBindSparseInfoKHX {
  fn default() -> VkDeviceGroupBindSparseInfoKHX {
    VkDeviceGroupBindSparseInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl RawStruct for VkDeviceGroupBindSparseInfoKHX {
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

/// Structure specifying device within a group to bind to
///
/// If the `pNext` list of `VkBindBufferMemoryInfoKHR` includes a
/// `VkBindBufferMemoryDeviceGroupInfoKHX` structure, then that structure determines
/// how memory is bound to buffers across multiple devices in a device group.
///
/// The `VkBindBufferMemoryDeviceGroupInfoKHX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `deviceIndexCount` is the number of elements in `pDeviceIndices`.
///
///   - `pDeviceIndices` is a pointer to an array of device indices.
///
/// If `deviceIndexCount` is greater than zero, then on device index i the buffer is
/// attached to the instance of `memory` on the physical device with device index
/// pDeviceIndices\[i\].
///
/// If `deviceIndexCount` is zero and `memory` comes from a memory heap with the
/// `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHX` bit set, then it is as if
/// `pDeviceIndices` contains consecutive indices from zero to the number of
/// physical devices in the logical device, minus one. In other words, by default
/// each physical device attaches to its own instance of `memory`.
///
/// If `deviceIndexCount` is zero and `memory` comes from a memory heap without the
/// `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHX` bit set, then it is as if
/// `pDeviceIndices` contains an array of zeros. In other words, by default each
/// physical device attaches to instance zero.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_device_indices(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pDeviceIndices = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> Default for VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
  fn default() -> VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
    VkBindBufferMemoryDeviceGroupInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> RawStruct for VkBindBufferMemoryDeviceGroupInfoKHX<'a> {
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

/// Structure specifying device within a group to bind to
///
/// If the `pNext` list of `VkBindImageMemoryInfoKHR` includes a
/// `VkBindImageMemoryDeviceGroupInfoKHX` structure, then that structure determines
/// how memory is bound to images across multiple devices in a device group.
///
/// The `VkBindImageMemoryDeviceGroupInfoKHX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `deviceIndexCount` is the number of elements in `pDeviceIndices`.
///
///   - `pDeviceIndices` is a pointer to an array of device indices.
///
///   - `SFRRectCount` is the number of elements in `pSFRRects`.
///
///   - `pSFRRects` is a pointer to an array of rectangles describing which regions
///     of the image are attached to each instance of memory.
///
/// If `deviceIndexCount` is greater than zero, then on device index i `image` is
/// attached to the instance of the memory on the physical device with device index
/// pDeviceIndices\[i\].
///
/// Let N be the number of physical devices in the logical device. If `SFRRectCount`
/// is greater than zero, then `pSFRRects` is an array of N<sup>2</sup> rectangles,
/// where the image region specified by the rectangle at element i\*N+j in resource
/// instance i is bound to the memory instance j. The blocks of the memory that are
/// bound to each sparse image block region use an offset in memory, relative to
/// `memoryOffset`, computed as if the whole image were being bound to a contiguous
/// range of memory. In other words, horizontally adjacent image blocks use
/// consecutive blocks of memory, vertically adjacent image blocks are separated by
/// the number of bytes per block multiplied by the width in blocks of `image`, and
/// the block at (0,0) corresponds to memory starting at `memoryOffset`.
///
/// If `SFRRectCount` and `deviceIndexCount` are zero and the memory comes from a
/// memory heap with the `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHX` bit set, then it is
/// as if `pDeviceIndices` contains consecutive indices from zero to the number of
/// physical devices in the logical device, minus one. In other words, by default
/// each physical device attaches to its own instance of the memory.
///
/// If `SFRRectCount` and `deviceIndexCount` are zero and the memory comes from a
/// memory heap without the `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHX` bit set, then it
/// is as if `pDeviceIndices` contains an array of zeros. In other words, by default
/// each physical device attaches to instance zero.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_device_indices(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pDeviceIndices = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_sfr_rects(mut self, value: &'a [VkRect2D]) -> Self {
    unsafe {
      self.pSFRRects = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> Default for VkBindImageMemoryDeviceGroupInfoKHX<'a> {
  fn default() -> VkBindImageMemoryDeviceGroupInfoKHX<'a> {
    VkBindImageMemoryDeviceGroupInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> RawStruct for VkBindImageMemoryDeviceGroupInfoKHX<'a> {
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

/// Present capabilities from other physical devices
///
/// The `VkDeviceGroupPresentCapabilitiesKHX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `presentMask` is an array of masks, where the mask at element i is non-zero
///     if physical device i has a presentation engine, and where bit j is set in
///     element i if physical device i can: present swapchain images from physical
///     device j. If element i is non-zero, then bit i must: be set.
///
///   - `modes` is a bitmask of `VkDeviceGroupPresentModeFlagBitsKHX` indicating
///     which device group presentation modes are supported.
///
/// `modes` always has `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX` set.
///
/// The present mode flags are also used when presenting an image, in
/// `VkDeviceGroupPresentInfoKHX::mode`.
///
/// If a device group only includes a single physical device, then `modes` must:
/// equal `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkDeviceGroupPresentCapabilitiesKHX {
  fn default() -> VkDeviceGroupPresentCapabilitiesKHX {
    VkDeviceGroupPresentCapabilitiesKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl RawStruct for VkDeviceGroupPresentCapabilitiesKHX {
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
///
/// If the `pNext` chain of `VkImageCreateInfo` includes a
/// `VkImageSwapchainCreateInfoKHX` structure, then that structure includes a
/// swapchain handle indicating that the image will be bound to memory from that
/// swapchain.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `swapchain` is `VK_NULL_HANDLE` or a handle of a swapchain that the image
///     will be bound to.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_swapchain(mut self, value: Option<VkSwapchainKHR>) -> Self {
    self.swapchain = value;
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkImageSwapchainCreateInfoKHX {
  fn default() -> VkImageSwapchainCreateInfoKHX {
    VkImageSwapchainCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl RawStruct for VkImageSwapchainCreateInfoKHX {
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

/// Structure specifying swapchain image memory to bind to
///
/// If the `pNext` chain of `VkBindImageMemoryInfoKHR` includes a
/// `VkBindImageMemorySwapchainInfoKHX` structure, then that structure includes a
/// swapchain handle and image index indicating that the image will be bound to
/// memory from that swapchain.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `swapchain` is `VK_NULL_HANDLE` or a swapchain handle.
///
///   - `imageIndex` is an image index within `swapchain`.
///
/// If `swapchain` is not `NULL`, the `swapchain` and `imageIndex` are used to
/// determine the memory that the image is bound to, instead of `memory` and
/// `memoryOffset`.
///
/// Memory can: be bound to a swapchain and use the `pDeviceIndices` or `pSFRRects`
/// members of `VkBindImageMemoryDeviceGroupInfoKHX`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkBindImageMemorySwapchainInfoKHX {
  fn default() -> VkBindImageMemorySwapchainInfoKHX {
    VkBindImageMemorySwapchainInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl RawStruct for VkBindImageMemorySwapchainInfoKHX {
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

/// Structure specifying parameters of the acquire
///
/// The `VkAcquireNextImageInfoKHX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `swapchain` is a non-retired swapchain from which an image is acquired.
///
///   - `timeout` indicates how long the function waits, in nanoseconds, if no image
///     is available.
///
///   - `semaphore` is `VK_NULL_HANDLE` or a semaphore to signal.
///
///   - `fence` is `VK_NULL_HANDLE` or a fence to signal.
///
///   - `deviceMask` is a mask of physical devices for which the swapchain image
///     will be ready to use when the semaphore or fence is signaled.
///
/// If `vkAcquireNextImageKHR` is used, the device mask is considered to include all
/// physical devices in the logical device.
///
/// > **Note**
/// >
/// > `vkAcquireNextImage2KHX` signals at most one semaphore, even if the
/// > application requests waiting for multiple physical devices to be ready via the
/// > `deviceMask`. However, only a single physical device can: wait on that
/// > semaphore, since the semaphore becomes unsignaled when the wait succeeds. For
/// > other physical devices to wait for the image to be ready, it is necessary for
/// > the application to submit semaphore signal operation(s) to that first physical
/// > device to signal additional semaphore(s) after the wait succeeds, which the
/// > other physical device(s) can: wait upon.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkAcquireNextImageInfoKHX {
  fn default() -> VkAcquireNextImageInfoKHX {
    VkAcquireNextImageInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl RawStruct for VkAcquireNextImageInfoKHX {
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

/// Mode and mask controlling which physical devices\\' images are presented
///
/// If the `pNext` chain of `VkPresentInfoKHR` includes a
/// `VkDeviceGroupPresentInfoKHX` structure, then that structure includes an array
/// of device masks and a device group present mode.
///
/// The `VkDeviceGroupPresentInfoKHX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `swapchainCount` is zero or the number of elements in `pDeviceMasks`.
///
///   - `pDeviceMasks` is an array of device masks, one for each element of
///     `VkPresentInfoKHR`::pSwapchains.
///
///   - `mode` is the device group present mode that will be used for this present.
///
/// If `mode` is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX`, then each element of
/// `pDeviceMasks` selects which instance of the swapchain image is presented. Each
/// element of `pDeviceMasks` must: have exactly one bit set, and the corresponding
/// physical device must: have a presentation engine as reported by
/// `VkDeviceGroupPresentCapabilitiesKHX`.
///
/// If `mode` is `VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHX`, then each element of
/// `pDeviceMasks` selects which instance of the swapchain image is presented. Each
/// element of `pDeviceMasks` must: have exactly one bit set, and some physical
/// device in the logical device must: include that bit in its
/// `VkDeviceGroupPresentCapabilitiesKHX::presentMask`.
///
/// If `mode` is `VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHX`, then each element of
/// `pDeviceMasks` selects which instances of the swapchain image are component-wise
/// summed and the sum of those images is presented. If the sum in any component is
/// outside the representable range, the value of that component is undefined. Each
/// element of `pDeviceMasks` must: have a value for which all set bits are set in
/// one of the elements of `VkDeviceGroupPresentCapabilitiesKHX::presentMask`.
///
/// If `mode` is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHX`, then
/// each element of `pDeviceMasks` selects which instance(s) of the swapchain images
/// are presented. For each bit set in each element of `pDeviceMasks`, the
/// corresponding physical device must: have a presentation engine as reported by
/// `VkDeviceGroupPresentCapabilitiesKHX`.
///
/// If `VkDeviceGroupPresentInfoKHX` is not provided or `swapchainCount` is zero
/// then the masks are considered to be `1`. If `VkDeviceGroupPresentInfoKHX` is not
/// provided, `mode` is considered to be
/// `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_device_masks(mut self, value: &'a [u32]) -> Self {
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
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> Default for VkDeviceGroupPresentInfoKHX<'a> {
  fn default() -> VkDeviceGroupPresentInfoKHX<'a> {
    VkDeviceGroupPresentInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl<'a> RawStruct for VkDeviceGroupPresentInfoKHX<'a> {
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

/// Structure specifying parameters of a newly created swapchain object
///
/// If the `pNext` chain of `VkSwapchainCreateInfoKHR` includes a
/// `VkDeviceGroupSwapchainCreateInfoKHX` structure, then that structure includes a
/// set of device group present modes that the swapchain can: be used with.
///
/// The `VkDeviceGroupSwapchainCreateInfoKHX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `modes` is a bitfield of modes that the swapchain can: be used with.
///
/// If this structure is not present, `modes` is considered to be
/// `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_modes(mut self, value: VkDeviceGroupPresentModeFlagsKHX) -> Self {
    self.modes = value;
    self
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl Default for VkDeviceGroupSwapchainCreateInfoKHX {
  fn default() -> VkDeviceGroupSwapchainCreateInfoKHX {
    VkDeviceGroupSwapchainCreateInfoKHX::new()
  }
}
#[cfg(feature = "VK_KHX_device_group")]
impl RawStruct for VkDeviceGroupSwapchainCreateInfoKHX {
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

/// Specify validation checks to disable for a Vulkan instance
///
/// When creating a Vulkan instance for which you wish to disable validation checks,
/// add a `VkValidationFlagsEXT` structure to the `pNext` chain of the
/// `VkInstanceCreateInfo` structure, specifying the checks to be disabled.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `disabledValidationCheckCount` is the number of checks to disable.
///
///   - `pDisabledValidationChecks` is a pointer to an array of
///     `VkValidationCheckEXT` values specifying the validation checks to be
///     disabled.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_disabled_validation_checks(mut self, value: &'a mut [VkValidationCheckEXT]) -> Self {
    unsafe {
      self.pDisabledValidationChecks = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_EXT_validation_flags")]
impl<'a> Default for VkValidationFlagsEXT<'a> {
  fn default() -> VkValidationFlagsEXT<'a> {
    VkValidationFlagsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_flags")]
impl<'a> RawStruct for VkValidationFlagsEXT<'a> {
  type Raw = types_raw::VkValidationFlagsEXT;
}
#[cfg(feature = "VK_EXT_validation_flags")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_validation_flags_ext() {
  assert_size!(types_raw::VkValidationFlagsEXT, VkValidationFlagsEXT);
}

// feature: VK_NN_vi_surface

/// Structure specifying parameters of a newly created VI surface object
///
/// The `VkViSurfaceCreateInfoNN` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `window` is the `nn::vi`::\`NativeWindowHandle\` for the `nn::vi`::\`Layer\`
///     with which to associate the surface.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkViSurfaceCreateFlagsNN) -> Self {
    self.flags = value;
    self
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
impl RawStruct for VkViSurfaceCreateInfoNN {
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
///
///   - `externalMemoryFeatures` is a bitmask of
///     `VkExternalMemoryFeatureFlagBitsKHR` specifying the features of
///     `handleType`.
///
///   - `exportFromImportedHandleTypes` is a bitmask of
///     `VkExternalMemoryHandleTypeFlagBitsKHR` specifying which types of imported
///     handle `handleType` can: be exported from.
///
///   - `compatibleHandleTypes` is a bitmask of
///     `VkExternalMemoryHandleTypeFlagBitsKHR` specifying handle types which can:
///     be specified at the same time as `handleType` when creating an image
///     compatible with external memory.
///
/// `compatibleHandleTypes` must: include at least `handleType`. Inclusion of a
/// handle type in `compatibleHandleTypes` does not imply the values returned in
/// `VkImageFormatProperties2KHR` will be the same when
/// `VkPhysicalDeviceExternalImageFormatInfoKHR::handleType` is set to that type.
/// The application is responsible for querying the capabilities of all handle types
/// intended for concurrent use in a single image and intersecting them to obtain
/// the compatible set of capabilities.
///
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
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkExternalMemoryPropertiesKHR {
  fn default() -> VkExternalMemoryPropertiesKHR {
    VkExternalMemoryPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl RawStruct for VkExternalMemoryPropertiesKHR {
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
///
/// To determine the image capabilities compatible with an external memory handle
/// type, add `VkPhysicalDeviceExternalImageFormatInfoKHR` to the `pNext` chain of
/// the `VkPhysicalDeviceImageFormatInfo2KHR` structure and
/// `VkExternalImageFormatPropertiesKHR` to the `pNext` chain of the
/// `VkImageFormatProperties2KHR` structure.
///
/// The `VkPhysicalDeviceExternalImageFormatInfoKHR` structure is defined as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleType` is a `VkExternalMemoryHandleTypeFlagBitsKHR` value specifying
///     the memory handle type that will be used with the memory associated with the
///     image.
///
/// If `handleType` is 0, `vkGetPhysicalDeviceImageFormatProperties2KHR` will behave
/// as if `VkPhysicalDeviceExternalImageFormatInfoKHR` was not present, and
/// `VkExternalImageFormatPropertiesKHR` will be ignored.
///
/// If `handleType` is not compatible with the parameters specified in
/// `VkPhysicalDeviceImageFormatInfo2KHR` and its `pNext` chain, then
/// `vkGetPhysicalDeviceImageFormatProperties2KHR` returns
/// `VK_ERROR_FORMAT_NOT_SUPPORTED`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkPhysicalDeviceExternalImageFormatInfoKHR {
  fn default() -> VkPhysicalDeviceExternalImageFormatInfoKHR {
    VkPhysicalDeviceExternalImageFormatInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl RawStruct for VkPhysicalDeviceExternalImageFormatInfoKHR {
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

/// Structure specifying supported external handle properties
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `externalMemoryProperties` is an instance of the
///     `VkExternalMemoryPropertiesKHR` structure specifying various capabilities of
///     the external handle type when used with the specified image creation
///     parameters.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_external_memory_properties(mut self, value: VkExternalMemoryPropertiesKHR) -> Self {
    self.externalMemoryProperties = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkExternalImageFormatPropertiesKHR {
  fn default() -> VkExternalImageFormatPropertiesKHR {
    VkExternalImageFormatPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl RawStruct for VkExternalImageFormatPropertiesKHR {
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

/// Structure specifying buffer creation parameters
///
///   - `sType` is the type of this structure
///
///   - `pNext` is NULL or a pointer to an extension-specific structure.
///
///   - `flags` is a bitmask of `VkBufferCreateFlagBits` describing additional
///     parameters of the buffer, corresponding to `VkBufferCreateInfo::flags`.
///
///   - `usage` is a bitmask of `VkBufferUsageFlagBits` describing the intended
///     usage of the buffer, corresponding to `VkBufferCreateInfo::usage`.
///
///   - `handleType` is a `VkExternalMemoryHandleTypeFlagBitsKHR` value specifying
///     the memory handle type that will be used with the memory associated with the
///     buffer.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkPhysicalDeviceExternalBufferInfoKHR {
  fn default() -> VkPhysicalDeviceExternalBufferInfoKHR {
    VkPhysicalDeviceExternalBufferInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl RawStruct for VkPhysicalDeviceExternalBufferInfoKHR {
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
///
///   - `sType` is the type of this structure
///
///   - `pNext` is NULL or a pointer to an extension-specific structure.
///
///   - `externalMemoryProperties` is an instance of the
///     `VkExternalMemoryPropertiesKHR` structure specifying various capabilities of
///     the external handle type when used with the specified buffer creation
///     parameters.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_external_memory_properties(mut self, value: VkExternalMemoryPropertiesKHR) -> Self {
    self.externalMemoryProperties = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkExternalBufferPropertiesKHR {
  fn default() -> VkExternalBufferPropertiesKHR {
    VkExternalBufferPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl RawStruct for VkExternalBufferPropertiesKHR {
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl Default for VkPhysicalDeviceIDPropertiesKHR {
  fn default() -> VkPhysicalDeviceIDPropertiesKHR {
    VkPhysicalDeviceIDPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
impl RawStruct for VkPhysicalDeviceIDPropertiesKHR {
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

/// Specify that an image may be backed by external memory
///
/// To define a set of external memory handle types that may: be used as backing
/// store for an image, add a `VkExternalMemoryImageCreateInfoKHR` structure to the
/// `pNext` chain of the `VkImageCreateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleTypes` is a bitmask of `VkExternalMemoryHandleTypeFlagBitsKHR`
///     specifying one or more external memory handle types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl Default for VkExternalMemoryImageCreateInfoKHR {
  fn default() -> VkExternalMemoryImageCreateInfoKHR {
    VkExternalMemoryImageCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl RawStruct for VkExternalMemoryImageCreateInfoKHR {
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

/// Specify that a buffer may be backed by external memory
///
/// To define a set of external memory handle types that may: be used as backing
/// store for a buffer, add a `VkExternalMemoryBufferCreateInfoKHR` structure to the
/// `pNext` chain of the `VkBufferCreateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleTypes` is a bitmask of `VkExternalMemoryHandleTypeFlagBitsKHR`
///     specifying one or more external memory handle types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl Default for VkExternalMemoryBufferCreateInfoKHR {
  fn default() -> VkExternalMemoryBufferCreateInfoKHR {
    VkExternalMemoryBufferCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl RawStruct for VkExternalMemoryBufferCreateInfoKHR {
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

/// Specify exportable handle types for a device memory object
///
/// When allocating memory that may: be exported to another process or Vulkan
/// instance, add a `VkExportMemoryAllocateInfoKHR` structure to the `pNext` chain
/// of the `VkMemoryAllocateInfo` structure, specifying the handle types that may:
/// be exported.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleTypes` is a bitmask of `VkExternalMemoryHandleTypeFlagBitsKHR`
///     specifying one or more memory handle types the application can: export from
///     the resulting allocation. The application can: request multiple handle types
///     for the same allocation.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalMemoryHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl Default for VkExportMemoryAllocateInfoKHR {
  fn default() -> VkExportMemoryAllocateInfoKHR {
    VkExportMemoryAllocateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory")]
impl RawStruct for VkExportMemoryAllocateInfoKHR {
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

/// import Win32 memory created on the same physical device
///
/// To import memory from a Windows handle, add a `VkImportMemoryWin32HandleInfoKHR`
/// structure to the `pNext` chain of the `VkMemoryAllocateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleType` specifies the type of `handle` or `name`.
///
///   - `handle` is the external handle to import, or `NULL`.
///
///   - `name` is a NULL-terminated UTF-16 string naming the underlying memory
///     resource to import, or `NULL`.
///
/// Importing memory objects from Windows handles does not transfer ownership of the
/// handle to the Vulkan implementation. For handle types defined as NT handles, the
/// application must: release ownership using the `CloseHandle` system call when the
/// handle is no longer needed.
///
/// Applications can: import the same underlying memory into multiple instances of
/// Vulkan, into the same instance from which it was exported, and multiple times
/// into a given Vulkan instance. In all cases, each import operation must: create a
/// distinct `VkDeviceMemory` object.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkImportMemoryWin32HandleInfoKHR {
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

/// Structure specifying additional attributes of Windows handles exported from a
/// memory
///
/// To specify additional attributes of NT handles exported from a memory object,
/// add the `VkExportMemoryWin32HandleInfoKHR` structure to the `pNext` chain of the
/// `VkMemoryAllocateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `pAttributes` is a pointer to a Windows `SECURITY_ATTRIBUTES` structure
///     specifying security attributes of the handle.
///
///   - `dwAccess` is a `DWORD` specifying access rights of the handle.
///
///   - `name` is a NULL-terminated UTF-16 string to associate with the underlying
///     resource referenced by NT handles exported from the created memory.
///
/// If this structure is not present, or if `pAttributes` is set to `NULL`, default
/// security descriptor values will be used, and child processes created by the
/// application will not inherit the handle, as described in the MSDN documentation
/// for “Synchronization Object Security and Access Rights”<sup>1</sup>. Further, if
/// the structure is not present, the access rights will be
///
/// `DXGI_SHARED_RESOURCE_READ` | `DXGI_SHARED_RESOURCE_WRITE`
///
/// for handles of the following types:
///
/// `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`
/// `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR`
///
/// And
///
/// `GENERIC_ALL`
///
/// for handles of the following types:
///
/// `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR`
/// `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR`
///
///   - 1
///     [https://msdn.microsoft.com/en-us/library/windows/desktop/ms686670.aspx](#)
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
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
impl RawStruct for VkExportMemoryWin32HandleInfoKHR {
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

/// Properties of External Memory Windows Handles
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `memoryTypeBits` is a bitmask containing one bit set for every memory type
///     which the specified windows handle can: be imported as.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_memory_type_bits(mut self, value: u32) -> Self {
    self.memoryTypeBits = value;
    self
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
impl RawStruct for VkMemoryWin32HandlePropertiesKHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `memory` is the memory object from which the handle will be exported.
///
///   - `handleType` is the type of handle requested.
///
/// The properties of the handle returned depend on the value of `handleType`. See
/// `VkExternalMemoryHandleTypeFlagBitsKHR` for a description of the properties of
/// the defined external memory handle types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkMemoryGetWin32HandleInfoKHR {
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
///
/// To import memory from a POSIX file descriptor handle, add a
/// `VkImportMemoryFdInfoKHR` structure to the `pNext` chain of the
/// `VkMemoryAllocateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleType` specifies the handle type of `fd`.
///
///   - `fd` is the external handle to import.
///
/// Importing memory from a file descriptor transfers ownership of the file
/// descriptor from the application to the Vulkan implementation. The application
/// must: not perform any operations on the file descriptor after a successful
/// import.
///
/// Applications can: import the same underlying memory into multiple instances of
/// Vulkan, into the same instance from which it was exported, and multiple times
/// into a given Vulkan instance. In all cases, each import operation must: create a
/// distinct `VkDeviceMemory` object.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl Default for VkImportMemoryFdInfoKHR {
  fn default() -> VkImportMemoryFdInfoKHR {
    VkImportMemoryFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl RawStruct for VkImportMemoryFdInfoKHR {
  type Raw = types_raw::VkImportMemoryFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_memory_fd_info_khr() {
  assert_size!(types_raw::VkImportMemoryFdInfoKHR, VkImportMemoryFdInfoKHR);
}

/// Properties of External Memory File Descriptors
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `memoryTypeBits` is a bitmask containing one bit set for every memory type
///     which the specified file descriptor can: be imported as.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_memory_type_bits(mut self, value: u32) -> Self {
    self.memoryTypeBits = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl Default for VkMemoryFdPropertiesKHR {
  fn default() -> VkMemoryFdPropertiesKHR {
    VkMemoryFdPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl RawStruct for VkMemoryFdPropertiesKHR {
  type Raw = types_raw::VkMemoryFdPropertiesKHR;
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_memory_fd_properties_khr() {
  assert_size!(types_raw::VkMemoryFdPropertiesKHR, VkMemoryFdPropertiesKHR);
}

/// Structure describing a POSIX FD semaphore export operation
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `memory` is the memory object from which the handle will be exported.
///
///   - `handleType` is the type of handle requested.
///
/// The properties of the file descriptor exported depend on the value of
/// `handleType`. See `VkExternalMemoryHandleTypeFlagBitsKHR` for a description of
/// the properties of the defined external memory handle types.
///
/// > **Note**
/// >
/// > The size of the exported file may: be larger than the size requested by
/// > `VkMemoryAllocateInfo`::allocationSize. If `handleType` is
/// > `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT`, then the application can:
/// > query the file’s actual size with [lseek(2)](#).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl Default for VkMemoryGetFdInfoKHR {
  fn default() -> VkMemoryGetFdInfoKHR {
    VkMemoryGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_memory_fd")]
impl RawStruct for VkMemoryGetFdInfoKHR {
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
///
/// When submitting work that operates on memory imported from a Direct3D 11
/// resource to a queue, the keyed mutex mechanism may: be used in addition to
/// Vulkan semaphores to synchronize the work. Keyed mutexes are a property of a
/// properly created shareable Direct3D 11 resource. They can: only be used if the
/// imported resource was created with the
/// etext:D3D11\_RESOURCE\_MISC\_SHARED\_KEYEDMUTEX flag.
///
/// To acquire keyed mutexes before submitted work and/or release them after, add a
/// `VkWin32KeyedMutexAcquireReleaseInfoKHR` structure to the `pNext` chain of the
/// `VkSubmitInfo` structure.
///
///   - `acquireCount` is the number of entries in the `pAcquireSyncs`,
///     `pAcquireKeys`, and `pAcquireTimeoutMilliseconds` arrays.
///
///   - `pAcquireSyncs` is a pointer to an array of `VkDeviceMemory` objects which
///     were imported from Direct3D 11 resources.
///
///   - `pAcquireKeys` is a pointer to an array of mutex key values to wait for
///     prior to beginning the submitted work. Entries refer to the keyed mutex
///     associated with the corresponding entries in `pAcquireSyncs`.
///
///   - `pAcquireTimeoutMilliseconds` is an array of timeout values, in millisecond
///     units, for each acquire specified in `pAcquireKeys`.
///
///   - `releaseCount` is the number of entries in the `pReleaseSyncs` and
///     `pReleaseKeys` arrays.
///
///   - `pReleaseSyncs` is a pointer to an array of `VkDeviceMemory` objects which
///     were imported from Direct3D 11 resources.
///
///   - `pReleaseKeys` is a pointer to an array of mutex key values to set when the
///     submitted work has completed. Entries refer to the keyed mutex associated
///     with the corresponding entries in `pReleaseSyncs`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_acquire_syncs(mut self, value: &'a [VkDeviceMemory]) -> Self {
    unsafe {
      self.pAcquireSyncs = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_acquire_keys(mut self, value: &'a [u64]) -> Self {
    unsafe {
      self.pAcquireKeys = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_acquire_timeouts(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pAcquireTimeouts = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_release_syncs(mut self, value: &'a [VkDeviceMemory]) -> Self {
    unsafe {
      self.pReleaseSyncs = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_release_keys(mut self, value: &'a [u64]) -> Self {
    unsafe {
      self.pReleaseKeys = value.as_raw();
    }
    self
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
impl<'a> RawStruct for VkWin32KeyedMutexAcquireReleaseInfoKHR<'a> {
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

/// Structure specifying semaphore creation parameters.
///
///   - `sType` is the type of this structure
///
///   - `pNext` is NULL or a pointer to an extension-specific structure.
///
///   - `handleType` is a `VkExternalSemaphoreHandleTypeFlagBitsKHR` value
///     specifying the external semaphore handle type for which capabilities will be
///     returned.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalSemaphoreHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl Default for VkPhysicalDeviceExternalSemaphoreInfoKHR {
  fn default() -> VkPhysicalDeviceExternalSemaphoreInfoKHR {
    VkPhysicalDeviceExternalSemaphoreInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl RawStruct for VkPhysicalDeviceExternalSemaphoreInfoKHR {
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
///
///   - `exportFromImportedHandleTypes` is a bitmask of
///     `VkExternalSemaphoreHandleTypeFlagBitsKHR` specifying which types of
///     imported handle `handleType` can: be exported from.
///
///   - `compatibleHandleTypes` is a bitmask of
///     `VkExternalSemaphoreHandleTypeFlagBitsKHR` specifying handle types which
///     can: be specified at the same time as `handleType` when creating a
///     semaphore.
///
///   - `externalSemaphoreFeatures` is a bitmask of
///     `VkExternalSemaphoreFeatureFlagBitsKHR` describing the features of
///     `handleType`.
///
/// If `handleType` is not supported by the implementation, then
/// `VkExternalSemaphorePropertiesKHR::externalSemaphoreFeatures` will be set to
/// zero.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl Default for VkExternalSemaphorePropertiesKHR {
  fn default() -> VkExternalSemaphorePropertiesKHR {
    VkExternalSemaphorePropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
impl RawStruct for VkExternalSemaphorePropertiesKHR {
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
///
/// To create a semaphore whose payload can: be exported to external handles, add
/// the `VkExportSemaphoreCreateInfoKHR` structure to the `pNext` chain of the
/// `VkSemaphoreCreateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleTypes` is a bitmask of `VkExternalSemaphoreHandleTypeFlagBitsKHR`
///     specifying one or more semaphore handle types the application can: export
///     from the resulting semaphore. The application can: request multiple handle
///     types for the same semaphore.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalSemaphoreHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
impl Default for VkExportSemaphoreCreateInfoKHR {
  fn default() -> VkExportSemaphoreCreateInfoKHR {
    VkExportSemaphoreCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore")]
impl RawStruct for VkExportSemaphoreCreateInfoKHR {
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

/// Structure specifying Windows handle to import to a semaphore
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `semaphore` is the semaphore into which the payload will be imported.
///
///   - `flags` is a bitmask of `VkSemaphoreImportFlagBitsKHR` specifying additional
///     parameters for the semaphore payload import operation.
///
///   - `handleType` specifies the type of `handle`.
///
///   - `handle` is the external handle to import, or `NULL`.
///
///   - `name` is a NULL-terminated UTF-16 string naming the underlying
///     synchronization primitive to import, or `NULL`.
///
/// The handle types supported by `handleType` are:
///
/// <table>
/// <caption>Handle Types Supported by VkImportSemaphoreWin32HandleInfoKHR</caption>
/// <colgroup>
/// <col width="33%" />
/// <col width="33%" />
/// <col width="33%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Handle Type</th>
/// <th align="left">Transference</th>
/// <th align="left">Permanence Supported</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR</code></p></td>
/// <td align="left"><p>Reference</p></td>
/// <td align="left"><p>Temporary,Permanent</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR</code></p></td>
/// <td align="left"><p>Reference</p></td>
/// <td align="left"><p>Temporary,Permanent</p></td>
/// </tr>
/// <tr class="odd">
/// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR</code></p></td>
/// <td align="left"><p>Reference</p></td>
/// <td align="left"><p>Temporary,Permanent</p></td>
/// </tr>
/// </tbody>
/// </table>
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkImportSemaphoreWin32HandleInfoKHR {
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
///
/// To specify additional attributes of NT handles exported from a semaphore, add
/// the `VkExportSemaphoreWin32HandleInfoKHR` structure to the `pNext` chain of the
/// `VkSemaphoreCreateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `pAttributes` is a pointer to a Windows `SECURITY_ATTRIBUTES` structure
///     specifying security attributes of the handle.
///
///   - `dwAccess` is a `DWORD` specifying access rights of the handle.
///
///   - `name` is a NULL-terminated UTF-16 string to associate with the underlying
///     synchronization primitive referenced by NT handles exported from the created
///     semaphore.
///
/// If this structure is not present, or if `pAttributes` is set to `NULL`, default
/// security descriptor values will be used, and child processes created by the
/// application will not inherit the handle, as described in the MSDN documentation
/// for “Synchronization Object Security and Access Rights”<sup>1</sup>. Further, if
/// the structure is not present, the access rights will be
///
/// `DXGI_SHARED_RESOURCE_READ` | `DXGI_SHARED_RESOURCE_WRITE`
///
/// for handles of the following types:
///
/// `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`
///
/// And
///
/// `GENERIC_ALL`
///
/// for handles of the following types:
///
/// `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR`
///
///   - 1
///     [https://msdn.microsoft.com/en-us/library/windows/desktop/ms686670.aspx](#)
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
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
impl RawStruct for VkExportSemaphoreWin32HandleInfoKHR {
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

/// Structure specifying values for Direct3D 12 fence-backed semaphores
///
/// To specify the values to use when waiting for and signaling semaphores whose
/// [current payload](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-importing) refers to a Direct3D 12
/// fence, add the `VkD3D12FenceSubmitInfoKHR` structure to the `pNext` chain of the
/// `VkSubmitInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `waitSemaphoreValuesCount` is the number of semaphore wait values specified
///     in `pWaitSemaphoreValues`.
///
///   - `pWaitSemaphoreValues` is an array of length `waitSemaphoreValuesCount`
///     containing values for the corresponding semaphores in
///     `VkSubmitInfo::pWaitSemaphores` to wait for.
///
///   - `signalSemaphoreValuesCount` is the number of semaphore signal values
///     specified in `pSignalSemaphoreValues`.
///
///   - `pSignalSemaphoreValues` is an array of length `signalSemaphoreValuesCount`
///     containing values for the corresponding semaphores in
///     `VkSubmitInfo::pSignalSemaphores` to set when signaled.
///
/// If the semaphore in `VkSubmitInfo::pWaitSemaphores` or
/// `VkSubmitInfo::pSignalSemaphores` corresponding to an entry in
/// `pWaitSemaphoreValues` or `pSignalSemaphoreValues` respectively does not
/// currently have a [payload](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-semaphores-payloads) referring to a
/// Direct3D 12 fence, the implementation must: ignore the value in the
/// `pWaitSemaphoreValues` or `pSignalSemaphoreValues` entry.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_wait_semaphore_values(mut self, value: &'a [u64]) -> Self {
    unsafe {
      self.pWaitSemaphoreValues = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_signal_semaphore_values(mut self, value: &'a [u64]) -> Self {
    unsafe {
      self.pSignalSemaphoreValues = value.as_raw();
    }
    self
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
impl<'a> RawStruct for VkD3D12FenceSubmitInfoKHR<'a> {
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

/// Structure describing a Win32 handle semaphore export operation
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `semaphore` is the semaphore from which state will be exported.
///
///   - `handleType` is the type of handle requested.
///
/// The properties of the handle returned depend on the value of `handleType`. See
/// `VkExternalSemaphoreHandleTypeFlagBitsKHR` for a description of the properties
/// of the defined external semaphore handle types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkSemaphoreGetWin32HandleInfoKHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `semaphore` is the semaphore into which the payload will be imported.
///
///   - `flags` is a bitmask of `VkSemaphoreImportFlagBitsKHR` specifying additional
///     parameters for the semaphore payload import operation.
///
///   - `handleType` specifies the type of `fd`.
///
///   - `fd` is the external handle to import.
///
/// The handle types supported by `handleType` are:
///
/// <table>
/// <caption>Handle Types Supported by VkImportSemaphoreFdInfoKHR</caption>
/// <colgroup>
/// <col width="33%" />
/// <col width="33%" />
/// <col width="33%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Handle Type</th>
/// <th align="left">Transference</th>
/// <th align="left">Permanence Supported</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR</code></p></td>
/// <td align="left"><p>Reference</p></td>
/// <td align="left"><p>Temporary,Permanent</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR</code></p></td>
/// <td align="left"><p>Copy</p></td>
/// <td align="left"><p>Temporary</p></td>
/// </tr>
/// </tbody>
/// </table>
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl Default for VkImportSemaphoreFdInfoKHR {
  fn default() -> VkImportSemaphoreFdInfoKHR {
    VkImportSemaphoreFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl RawStruct for VkImportSemaphoreFdInfoKHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `semaphore` is the semaphore from which state will be exported.
///
///   - `handleType` is the type of handle requested.
///
/// The properties of the file descriptor returned depend on the value of
/// `handleType`. See `VkExternalSemaphoreHandleTypeFlagBitsKHR` for a description
/// of the properties of the defined external semaphore handle types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl Default for VkSemaphoreGetFdInfoKHR {
  fn default() -> VkSemaphoreGetFdInfoKHR {
    VkSemaphoreGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
impl RawStruct for VkSemaphoreGetFdInfoKHR {
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
///
/// The `VkPhysicalDevicePushDescriptorPropertiesKHR` structure is defined as.
///
/// The members of the `VkPhysicalDevicePushDescriptorPropertiesKHR` structure
/// describe the following implementation-dependent limits:
///
///   - `maxPushDescriptors` is the maximum number of descriptors that can: be used
///     in a descriptor set created with
///     `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_max_push_descriptors(mut self, value: u32) -> Self {
    self.maxPushDescriptors = value;
    self
  }
}
#[cfg(feature = "VK_KHR_push_descriptor")]
impl Default for VkPhysicalDevicePushDescriptorPropertiesKHR {
  fn default() -> VkPhysicalDevicePushDescriptorPropertiesKHR {
    VkPhysicalDevicePushDescriptorPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_push_descriptor")]
impl RawStruct for VkPhysicalDevicePushDescriptorPropertiesKHR {
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_16bit_storage")]
impl Default for VkPhysicalDevice16BitStorageFeaturesKHR {
  fn default() -> VkPhysicalDevice16BitStorageFeaturesKHR {
    VkPhysicalDevice16BitStorageFeaturesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_16bit_storage")]
impl RawStruct for VkPhysicalDevice16BitStorageFeaturesKHR {
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

/// Structure containing a rectangle, including layer, changed by vkQueuePresentKHR
/// for a given VkImage
///
/// The `VkRectLayerKHR` structure is defined as:
///
///   - `offset` is the origin of the rectangle, in pixels.
///
///   - `extent` is the size of the rectangle, in pixels.
///
///   - `layer` is the layer of the image. For images with only one layer, the value
///     of `layer` must: be 0.
///
/// Some platforms allow the size of a surface to change, and then scale the pixels
/// of the image to fit the surface. `VkRectLayerKHR` specifies pixels of the
/// swapchain’s image(s), which will be constant for the life of the swapchain.
///
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
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl Default for VkRectLayerKHR {
  fn default() -> VkRectLayerKHR {
    VkRectLayerKHR::new()
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl RawStruct for VkRectLayerKHR {
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
///
/// For a given image and swapchain, the region to present is specified by the
/// `VkPresentRegionKHR` structure, which is defined as:
///
///   - `rectangleCount` is the number of rectangles in `pRectangles`, or zero if
///     the entire image has changed and should be presented.
///
///   - `pRectangles` is either `NULL` or a pointer to an array of `VkRectLayerKHR`
///     structures. The `VkRectLayerKHR` structure is the framebuffer coordinates,
///     plus layer, of a portion of a presentable image that has changed and must:
///     be presented. If non-`NULL`, each entry in `pRectangles` is a rectangle of
///     the given image that has changed since the last image was presented to the
///     given swapchain.
///
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
    unsafe {
      self.pRectangles = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'a> Default for VkPresentRegionKHR<'a> {
  fn default() -> VkPresentRegionKHR<'a> {
    VkPresentRegionKHR::new()
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'a> RawStruct for VkPresentRegionKHR<'a> {
  type Raw = types_raw::VkPresentRegionKHR;
}
#[cfg(feature = "VK_KHR_incremental_present")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_region_khr() {
  assert_size!(types_raw::VkPresentRegionKHR, VkPresentRegionKHR);
}

/// Structure hint of rectangular regions changed by vkQueuePresentKHR
///
/// When the `VK_KHR_incremental_present` extension is enabled, additional fields
/// can: be specified that allow an application to specify that only certain
/// rectangular regions of the presentable images of a swapchain are changed. This
/// is an optimization hint that a presentation engine may: use to only update the
/// region of a surface that is actually changing. The application still must:
/// ensure that all pixels of a presented image contain the desired values, in case
/// the presentation engine ignores this hint. An application can: provide this hint
/// by including the `VkPresentRegionsKHR` structure in the `pNext` chain of the
/// `VkPresentInfoKHR` structure.
///
/// The `VkPresentRegionsKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `swapchainCount` is the number of swapchains being presented to by this
///     command.
///
///   - `pRegions` is `NULL` or a pointer to an array of `VkPresentRegionKHR`
///     elements with `swapchainCount` entries. If not `NULL`, each element of
///     `pRegions` contains the region that has changed since the last present to
///     the swapchain in the corresponding entry in the
///     `VkPresentInfoKHR::pSwapchains` array.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_regions(mut self, value: &'a [VkPresentRegionKHR<'a>]) -> Self {
    unsafe {
      self.pRegions = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'a> Default for VkPresentRegionsKHR<'a> {
  fn default() -> VkPresentRegionsKHR<'a> {
    VkPresentRegionsKHR::new()
  }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl<'a> RawStruct for VkPresentRegionsKHR<'a> {
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

/// Opaque handle to a descriptor update template
///
/// A descriptor update template specifies a mapping from descriptor update
/// information in host memory to descriptors in a descriptor set. It is designed to
/// avoid passing redundant information to the driver when frequently updating the
/// same set of descriptors in descriptor sets.
///
/// Descriptor update template objects are represented by
/// `VkDescriptorUpdateTemplateKHR` handles.
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type VkDescriptorUpdateTemplateKHR = VkNonDispatchableHandle<VkDescriptorUpdateTemplateKHR__>;

/// Describes a single descriptor update of the descriptor update template
///
///   - `dstBinding` is the descriptor binding to update when using this descriptor
///     update template.
///
///   - `dstArrayElement` is the starting element in the array belonging to
///     `dstBinding`.
///
///   - `descriptorCount` is the number of descriptors to update. If
///     `descriptorCount` is greater than the number of remaining array elements in
///     the destination binding, those affect consecutive bindings in a manner
///     similar to `VkWriteDescriptorSet` above.
///
///   - `descriptorType` is a `VkDescriptorType` specifying the type of the
///     descriptor.
///
///   - `offset` is the offset in bytes of the first binding in the raw data
///     structure.
///
///   - `stride` is the stride in bytes between two consecutive array elements of
///     the descriptor update informations in the raw data structure. The actual
///     pointer ptr for each array element j of update entry i is computed using the
///     following
///     formula:
///
///     ``` c++
///         const char *ptr = (const char *)pData + pDescriptorUpdateEntries[i].offset + j * pDescriptorUpdateEntries[i].stride
///     ```
///
///     The stride is useful in case the bindings are stored in structs along with
///     other data.
///
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
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl Default for VkDescriptorUpdateTemplateEntryKHR {
  fn default() -> VkDescriptorUpdateTemplateEntryKHR {
    VkDescriptorUpdateTemplateEntryKHR::new()
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl RawStruct for VkDescriptorUpdateTemplateEntryKHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `descriptorUpdateEntryCount` is the number of elements in the
///     `pDescriptorUpdateEntries` array.
///
///   - `pDescriptorUpdateEntries` is a pointer to an array of
///     `VkDescriptorUpdateTemplateEntryKHR` structures describing the descriptors
///     to be updated by the descriptor update template.
///
///   - `templateType` Specifies the type of the descriptor update template. If set
///     to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR` it can: only be
///     used to update descriptor sets with a fixed `descriptorSetLayout`. If set to
///     `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR` it can: only be
///     used to push descriptor sets using the provided `pipelineBindPoint`,
///     `pipelineLayout`, and `set` number.
///
///   - `descriptorSetLayout` is the descriptor set layout the parameter update
///     template will be used with. All descriptor sets which are going to be
///     updated through the newly created descriptor update template must: be
///     created with this layout. `descriptorSetLayout` is the descriptor set layout
///     used to build the descriptor update template. All descriptor sets which are
///     going to be updated through the newly created descriptor update template
///     must: be created with a layout that matches (is the same as, or defined
///     identically to) this layout. This parameter is ignored if `templateType` is
///     not `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR`.
///
///   - `pipelineBindPoint` is a `VkPipelineBindPoint` indicating whether the
///     descriptors will be used by graphics pipelines or compute pipelines. This
///     parameter is ignored if `templateType` is not
///     `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
///
///   - `pipelineLayout` is a `VkPipelineLayout` object used to program the
///     bindings. This parameter is ignored if `templateType` is not
///     `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
///
///   - `set` is the set number of the descriptor set in the pipeline layout that
///     will be updated. This parameter is ignored if `templateType` is not
///     `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkDescriptorUpdateTemplateCreateFlagsKHR) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_descriptor_update_entries(mut self, value: &'a [VkDescriptorUpdateTemplateEntryKHR]) -> Self {
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
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<'a> Default for VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
  fn default() -> VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
    VkDescriptorUpdateTemplateCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<'a> RawStruct for VkDescriptorUpdateTemplateCreateInfoKHR<'a> {
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
///
/// The device-side bindings are registered inside a table:
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkObjectTableNVX = VkNonDispatchableHandle<VkObjectTableNVX__>;
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum VkIndirectCommandsLayoutNVX__ {}

/// Opaque handle to an indirect commands layout object
///
/// The device-side command generation happens through an iterative processing of an
/// atomic sequence comprised of command tokens, which are represented by:
///
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub type VkIndirectCommandsLayoutNVX = VkNonDispatchableHandle<VkIndirectCommandsLayoutNVX__>;

/// Structure specifying physical device support
///
/// The `VkDeviceGeneratedCommandsFeaturesNVX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `computeBindingPointSupport` indicates whether the `VkObjectTableNVX`
///     supports entries with `VK_OBJECT_ENTRY_USAGE_GRAPHICS_BIT_NVX` bit set and
///     `VkIndirectCommandsLayoutNVX` supports `VK_PIPELINE_BIND_POINT_COMPUTE`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_compute_binding_point_support(mut self, value: VkBool32) -> Self {
    self.computeBindingPointSupport = value;
    self
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkDeviceGeneratedCommandsFeaturesNVX {
  fn default() -> VkDeviceGeneratedCommandsFeaturesNVX {
    VkDeviceGeneratedCommandsFeaturesNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkDeviceGeneratedCommandsFeaturesNVX {
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
///
/// The `VkDeviceGeneratedCommandsLimitsNVX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `maxIndirectCommandsLayoutTokenCount` the maximum number of tokens in
///     `VkIndirectCommandsLayoutNVX`.
///
///   - `maxObjectEntryCounts` the maximum number of entries per resource type in
///     `VkObjectTableNVX`.
///
///   - `minSequenceCountBufferOffsetAlignment` the minimum alignment for memory
///     addresses optionally used in `vkCmdProcessCommandsNVX`.
///
///   - `minSequenceIndexBufferOffsetAlignment` the minimum alignment for memory
///     addresses optionally used in `vkCmdProcessCommandsNVX`.
///
///   - `minCommandsTokenBufferOffsetAlignment` the minimum alignment for memory
///     addresses optionally used in `vkCmdProcessCommandsNVX`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkDeviceGeneratedCommandsLimitsNVX {
  fn default() -> VkDeviceGeneratedCommandsLimitsNVX {
    VkDeviceGeneratedCommandsLimitsNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkDeviceGeneratedCommandsLimitsNVX {
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
///
/// The `VkIndirectCommandsTokenNVX` structure specifies the input data for a token
/// at processing time.
///
///   - `tokenType` specifies the token command type.
///
///   - `buffer` specifies the `VkBuffer` storing the functional arguments for each
///     squence. These argumetns can be written by the device.
///
///   - `offset` specified an offset into `buffer` where the arguments start.
///
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkIndirectCommandsTokenNVX {
  fn default() -> VkIndirectCommandsTokenNVX {
    VkIndirectCommandsTokenNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkIndirectCommandsTokenNVX {
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
///
/// The `VkIndirectCommandsLayoutTokenNVX` structure specifies details to the
/// function arguments that need to be known at layout creation time:
///
///   - `type` specifies the token command type.
///
///   - `bindingUnit` has a different meaning depending on the type, please refer
///     pseudo code further down for details.
///
///   - `dynamicCount` has a different meaning depending on the type, please refer
///     pseudo code further down for details.
///
///   - `divisor` defines the rate at which the input data buffers are accessed.
///
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkIndirectCommandsLayoutTokenNVX {
  fn default() -> VkIndirectCommandsLayoutTokenNVX {
    VkIndirectCommandsLayoutTokenNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkIndirectCommandsLayoutTokenNVX {
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
///
/// The `VkIndirectCommandsLayoutCreateInfoNVX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `pipelineBindPoint` is the `VkPipelineBindPoint` that this layout targets.
///
///   - `flags` is a bitmask of `VkIndirectCommandsLayoutUsageFlagBitsNVX`
///     specifying usage hints of this layout.
///
///   - `tokenCount` is the length of the individual command sequnce.
///
///   - `pTokens` is an array describing each command token in detail. See
///     `VkIndirectCommandsTokenTypeNVX` and `VkIndirectCommandsLayoutTokenNVX`
///     below for details.
///
/// The following code illustrates some of the key
/// flags:
///
/// ``` c
/// void cmdProcessAllSequences(cmd, objectTable, indirectCommandsLayout, pIndirectCommandsTokens, sequencesCount, indexbuffer, indexbufferoffset)
/// {
///   for (s = 0; s < sequencesCount; s++)
///   {
///     sequence = s;
///
///     if (indirectCommandsLayout.flags & VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NVX) {
///       sequence = incoherent_implementation_dependent_permutation[ sequence ];
///     }
///     if (indirectCommandsLayout.flags & VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NVX) {
///       sequence = indexbuffer.load_uint32( sequence * sizeof(uint32_t) + indexbufferoffset);
///     }
///
///     cmdProcessSequence( cmd, objectTable, indirectCommandsLayout, pIndirectCommandsTokens, sequence );
///   }
/// }
/// ```
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pTokens = value.as_raw();
    }
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
impl<'a> RawStruct for VkIndirectCommandsLayoutCreateInfoNVX<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `objectTable` is the `VkObjectTableNVX` to be used for the generation
///     process. Only registered objects at the time
///     `vkCmdReserveSpaceForCommandsNVX` is called, will be taken into account for
///     the reservation.
///
///   - `indirectCommandsLayout` is the `VkIndirectCommandsLayoutNVX` that provides
///     the command sequence to generate.
///
///   - `indirectCommandsTokenCount` defines the number of input tokens used.
///
///   - `pIndirectCommandsTokens` provides an array of `VkIndirectCommandsTokenNVX`
///     that reference the input data for each token command.
///
///   - `maxSequencesCount` is the maximum number of sequences for which command
///     buffer space will be reserved. If `sequencesCountBuffer` is
///     `VK_NULL_HANDLE`, this is also the actual number of sequences generated.
///
///   - `targetCommandBuffer` can: be the secondary `VkCommandBuffer` in which the
///     commands should be recorded. If `targetCommandBuffer` is `NULL` an implicit
///     reservation as well as execution takes place on the processing
///     `VkCommandBuffer`.
///
///   - `sequencesCountBuffer` can: be `VkBuffer` from which the actual amount of
///     sequences is sourced from as ftext:uint32\_t value.
///
///   - `sequencesCountOffset` is the byte offset into `sequencesCountBuffer` where
///     the count value is stored.
///
///   - `sequencesIndexBuffer` must: be set if ``indirectCommandsLayout’s
///     `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT`` is set and
///     provides the used sequence indices as ftext:uint32\_t array. Otherwise it
///     must: be `VK_NULL_HANDLE`.
///
///   - `sequencesIndexOffset` is the byte offset into `sequencesIndexBuffer` where
///     the index values start.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> Default for VkCmdProcessCommandsInfoNVX<'a> {
  fn default() -> VkCmdProcessCommandsInfoNVX<'a> {
    VkCmdProcessCommandsInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> RawStruct for VkCmdProcessCommandsInfoNVX<'a> {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `objectTable` is the `VkObjectTableNVX` to be used for the generation
///     process. Only registered objects at the time
///     `vkCmdReserveSpaceForCommandsNVX` is called, will be taken into account for
///     the reservation.
///
///   - `indirectCommandsLayout` is the `VkIndirectCommandsLayoutNVX` that must:
///     also be used at generation time.
///
///   - `maxSequencesCount` is the maximum number of sequences for which command
///     buffer space will be reserved.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkCmdReserveSpaceForCommandsInfoNVX {
  fn default() -> VkCmdReserveSpaceForCommandsInfoNVX {
    VkCmdReserveSpaceForCommandsInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkCmdReserveSpaceForCommandsInfoNVX {
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
///
/// The `VkObjectTableCreateInfoNVX` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `objectCount` is the number of entry configurations that the object table
///     supports.
///
///   - `pObjectEntryTypes` is an array of `VkObjectEntryTypeNVX` values providing
///     the entry type of a given configuration.
///
///   - `pObjectEntryCounts` is an array of counts of how many objects can be
///     registered in the table.
///
///   - `pObjectEntryUsageFlags` is an array of bitmasks of
///     `VkObjectEntryUsageFlagBitsNVX` specifying the binding usage of the entry.
///
///   - `maxUniformBuffersPerDescriptor` is the maximum number of
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` used by any single registered
///     `VkDescriptorSet` in this table.
///
///   - `maxStorageBuffersPerDescriptor` is the maximum number of
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` used by any single registered
///     `VkDescriptorSet` in this table.
///
///   - `maxStorageImagesPerDescriptor` is the maximum number of
///     `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` or
///     `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` used by any single registered
///     `VkDescriptorSet` in this table.
///
///   - `maxSampledImagesPerDescriptor` is the maximum number of
///     `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
///     `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` or
///     `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` used by any single registered
///     `VkDescriptorSet` in this table.
///
///   - `maxPipelineLayouts` is the maximum number of unique `VkPipelineLayout` used
///     by any registered `VkDescriptorSet` or `VkPipeline` in this table.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_object_entry_types(mut self, value: &'a [VkObjectEntryTypeNVX]) -> Self {
    unsafe {
      self.pObjectEntryTypes = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_object_entry_counts(mut self, value: &'a [u32]) -> Self {
    unsafe {
      self.pObjectEntryCounts = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_object_entry_usage_flags(mut self, value: &'a [VkObjectEntryUsageFlagsNVX]) -> Self {
    unsafe {
      self.pObjectEntryUsageFlags = value.as_raw();
    }
    self
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> Default for VkObjectTableCreateInfoNVX<'a> {
  fn default() -> VkObjectTableCreateInfoNVX<'a> {
    VkObjectTableCreateInfoNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl<'a> RawStruct for VkObjectTableCreateInfoNVX<'a> {
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
///
/// Common to all resource entries are:
///
///   - `type` defines the entry type
///
///   - `flags` defines which `VkPipelineBindPoint` the resource can be used with.
///     Some entry types allow only a single flag to be set.
///
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTableEntryNVX {
  fn default() -> VkObjectTableEntryNVX {
    VkObjectTableEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkObjectTableEntryNVX {
  type Raw = types_raw::VkObjectTableEntryNVX;
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_object_table_entry_nvx() {
  assert_size!(types_raw::VkObjectTableEntryNVX, VkObjectTableEntryNVX);
}

/// Parameters of an object table pipeline entry
///
///   - `pipeline` specifies the `VkPipeline` that this resource entry references.
///
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTablePipelineEntryNVX {
  fn default() -> VkObjectTablePipelineEntryNVX {
    VkObjectTablePipelineEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkObjectTablePipelineEntryNVX {
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
///
///   - `pipelineLayout` specifies the `VkPipelineLayout` that the `descriptorSet`
///     is used with.
///
///   - `descriptorSet` specifies the `VkDescriptorSet` that can be bound with this
///     entry.
///
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTableDescriptorSetEntryNVX {
  fn default() -> VkObjectTableDescriptorSetEntryNVX {
    VkObjectTableDescriptorSetEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkObjectTableDescriptorSetEntryNVX {
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
///
///   - `buffer` specifies the `VkBuffer` that can be bound as vertex bufer
///
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTableVertexBufferEntryNVX {
  fn default() -> VkObjectTableVertexBufferEntryNVX {
    VkObjectTableVertexBufferEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkObjectTableVertexBufferEntryNVX {
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
///
///   - `buffer` specifies the `VkBuffer` that can be bound as index buffer
///
///   - `indexType` specifies the `VkIndexType` used with this index buffer
///
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTableIndexBufferEntryNVX {
  fn default() -> VkObjectTableIndexBufferEntryNVX {
    VkObjectTableIndexBufferEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkObjectTableIndexBufferEntryNVX {
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
///
///   - `pipelineLayout` specifies the `VkPipelineLayout` that the pushconstants are
///     used with
///
///   - `stageFlags` specifies the `VkShaderStageFlags` that the pushconstants are
///     used with
///
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
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl Default for VkObjectTablePushConstantEntryNVX {
  fn default() -> VkObjectTablePushConstantEntryNVX {
    VkObjectTablePushConstantEntryNVX::new()
  }
}
#[cfg(feature = "VK_NVX_device_generated_commands")]
impl RawStruct for VkObjectTablePushConstantEntryNVX {
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
///
/// The `VkViewportWScalingNV` structure is defined as:
///
///   - `xcoeff` and `ycoeff` are the viewport’s W scaling factor for x and y
///     respectively.
///
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
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl Default for VkViewportWScalingNV {
  fn default() -> VkViewportWScalingNV {
    VkViewportWScalingNV::new()
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl RawStruct for VkViewportWScalingNV {
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
///
/// The `VkPipelineViewportWScalingStateCreateInfoNV` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `viewportWScalingEnable` controls whether viewport **W** scaling is enabled.
///
///   - `viewportCount` is the number of viewports used by **W** scaling, and must:
///     match the number of viewports in the pipeline if viewport **W** scaling is
///     enabled.
///
///   - `pViewportWScalings` is a pointer to an array of `VkViewportWScalingNV`
///     structures, which define the **W** scaling parameters for the corresponding
///     viewport. If the viewport **W** scaling state is dynamic, this member is
///     ignored.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_viewport_w_scaling_enable(mut self, value: VkBool32) -> Self {
    self.viewportWScalingEnable = value;
    self
  }
  #[inline]
  pub fn set_viewport_w_scalings(mut self, value: &'a [VkViewportWScalingNV]) -> Self {
    unsafe {
      self.pViewportWScalings = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl<'a> Default for VkPipelineViewportWScalingStateCreateInfoNV<'a> {
  fn default() -> VkPipelineViewportWScalingStateCreateInfoNV<'a> {
    VkPipelineViewportWScalingStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl<'a> RawStruct for VkPipelineViewportWScalingStateCreateInfoNV<'a> {
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

/// Structure describing capabilities of a surface
///
/// The `VkSurfaceCapabilities2EXT` structure is defined as:
///
/// All members of `VkSurfaceCapabilities2EXT` are identical to the corresponding
/// members of `VkSurfaceCapabilitiesKHR` where one exists. The remaining members
/// are:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `supportedSurfaceCounters` is a bitmask of `VkSurfaceCounterFlagBitsEXT`
///     indicating the supported surface counter types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_display_surface_counter")]
impl Default for VkSurfaceCapabilities2EXT {
  fn default() -> VkSurfaceCapabilities2EXT {
    VkSurfaceCapabilities2EXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_surface_counter")]
impl RawStruct for VkSurfaceCapabilities2EXT {
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
///
/// The `VkDisplayPowerInfoEXT` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `powerState` is a `VkDisplayPowerStateEXT` value specifying the new power
///     state of the display.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_power_state(mut self, value: VkDisplayPowerStateEXT) -> Self {
    self.powerState = value;
    self
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl Default for VkDisplayPowerInfoEXT {
  fn default() -> VkDisplayPowerInfoEXT {
    VkDisplayPowerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl RawStruct for VkDisplayPowerInfoEXT {
  type Raw = types_raw::VkDisplayPowerInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_power_info_ext() {
  assert_size!(types_raw::VkDisplayPowerInfoEXT, VkDisplayPowerInfoEXT);
}

/// Describe a device event to create
///
/// The `VkDeviceEventInfoEXT` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `device` is a `VkDeviceEventTypeEXT` value specifying when the fence will be
///     signaled.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_device_event(mut self, value: VkDeviceEventTypeEXT) -> Self {
    self.deviceEvent = value;
    self
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl Default for VkDeviceEventInfoEXT {
  fn default() -> VkDeviceEventInfoEXT {
    VkDeviceEventInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl RawStruct for VkDeviceEventInfoEXT {
  type Raw = types_raw::VkDeviceEventInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_device_event_info_ext() {
  assert_size!(types_raw::VkDeviceEventInfoEXT, VkDeviceEventInfoEXT);
}

/// Describe a display event to create
///
/// The `VkDisplayEventInfoEXT` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `displayEvent` is a `VkDisplayEventTypeEXT` specifying when the fence will
///     be signaled.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_display_event(mut self, value: VkDisplayEventTypeEXT) -> Self {
    self.displayEvent = value;
    self
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl Default for VkDisplayEventInfoEXT {
  fn default() -> VkDisplayEventInfoEXT {
    VkDisplayEventInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl RawStruct for VkDisplayEventInfoEXT {
  type Raw = types_raw::VkDisplayEventInfoEXT;
}
#[cfg(feature = "VK_EXT_display_control")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_display_event_info_ext() {
  assert_size!(types_raw::VkDisplayEventInfoEXT, VkDisplayEventInfoEXT);
}

/// Specify the surface counters desired
///
/// To enable surface counters when creating a swapchain, add
/// `VkSwapchainCounterCreateInfoEXT` to the `pNext` chain of
/// `VkSwapchainCreateInfoKHR`. `VkSwapchainCounterCreateInfoEXT` is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `surfaceCounters` is a bitmask of `VkSurfaceCounterFlagBitsEXT` specifying
///     surface counters to enable for the swapchain.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_surface_counters(mut self, value: VkSurfaceCounterFlagsEXT) -> Self {
    self.surfaceCounters = value;
    self
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl Default for VkSwapchainCounterCreateInfoEXT {
  fn default() -> VkSwapchainCounterCreateInfoEXT {
    VkSwapchainCounterCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_display_control")]
impl RawStruct for VkSwapchainCounterCreateInfoEXT {
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

/// Structure containing the RC duration of a display
///
/// The `VkRefreshCycleDurationGOOGLE` structure is defined as:
///
///   - `refreshDuration` is the number of nanoseconds from the start of one refresh
///     cycle to the next.
///
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
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl Default for VkRefreshCycleDurationGOOGLE {
  fn default() -> VkRefreshCycleDurationGOOGLE {
    VkRefreshCycleDurationGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl RawStruct for VkRefreshCycleDurationGOOGLE {
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
///
/// The `VkPastPresentationTimingGOOGLE` structure is defined as:
///
///   - `presentID` is an application-provided value that was given to a previous
///     `vkQueuePresentKHR` command via `VkPresentTimeGOOGLE::presentID` (see
///     below). It can: be used to uniquely identify a previous present with the
///     `vkQueuePresentKHR` command.
///
///   - `desiredPresentTime` is an application-provided value that was given to a
///     previous `vkQueuePresentKHR` command via
///     `VkPresentTimeGOOGLE::desiredPresentTime`. If non-zero, it was used by the
///     application to indicate that an image not be presented any sooner than
///     `desiredPresentTime`.
///
///   - `actualPresentTime` is the time when the image of the `swapchain` was
///     actually displayed.
///
///   - `earliestPresentTime` is the time when the image of the `swapchain` could
///     have been displayed. This may: differ from `actualPresentTime` if the
///     application requested that the image be presented no sooner than
///     `VkPresentTimeGOOGLE::desiredPresentTime`.
///
///   - `presentMargin` is an indication of how early the `vkQueuePresentKHR`
///     command was processed compared to how soon it needed to be processed, and
///     still be presented at `earliestPresentTime`.
///
/// The results for a given `swapchain` and `presentID` are only returned once from
/// `vkGetPastPresentationTimingGOOGLE`.
///
/// The application can: use the `VkPastPresentationTimingGOOGLE` values to
/// occasionally adjust its timing. For example, if `actualPresentTime` is later
/// than expected (e.g. one `refreshDuration` late), the application may increase
/// its target IPD to a higher multiple of `refreshDuration` (e.g. decrease its
/// frame rate from 60Hz to 30Hz). If `actualPresentTime` and `earliestPresentTime`
/// are consistently different, and if `presentMargin` is consistently large enough,
/// the application may decrease its target IPD to a smaller multiple of
/// `refreshDuration` (e.g. increase its frame rate from 30Hz to 60Hz). If
/// `actualPresentTime` and `earliestPresentTime` are same, and if `presentMargin`
/// is consistently high, the application may delay the start of its
/// input-render-present loop in order to decrease the latency between user input
/// and the corresponding present (always leaving some margin in case a new image
/// takes longer to render than the previous image). An application that desires its
/// target IPD to always be the same as `refreshDuration`, can also adjust features
/// until `actualPresentTime` is never late and `presentMargin` is satisfactory.
///
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
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl Default for VkPastPresentationTimingGOOGLE {
  fn default() -> VkPastPresentationTimingGOOGLE {
    VkPastPresentationTimingGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl RawStruct for VkPastPresentationTimingGOOGLE {
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
///
/// The `VkPresentTimeGOOGLE` structure is defined as:
///
///   - `presentID` is an application-provided identification value, that can: be
///     used with the results of `vkGetPastPresentationTimingGOOGLE`, in order to
///     uniquely identify this present. In order to be useful to the application, it
///     should: be unique within some period of time that is meaningful to the
///     application.
///
///   - `desiredPresentTime` indicates that the image given should: not be displayed
///     to the user any earlier than this time. `desiredPresentTime` is a time in
///     nanoseconds, relative to a monotonically-increasing clock (e.g.
///     `CLOCK_MONOTONIC` (see clock\_gettime(2)) on Android and Linux). A value of
///     zero indicates that the presentation engine may: display the image at any
///     time. This is useful when the application desires to provide `presentID`,
///     but doesn’t need a specific `desiredPresentTime`.
///
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
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl Default for VkPresentTimeGOOGLE {
  fn default() -> VkPresentTimeGOOGLE {
    VkPresentTimeGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl RawStruct for VkPresentTimeGOOGLE {
  type Raw = types_raw::VkPresentTimeGOOGLE;
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_present_time_google() {
  assert_size!(types_raw::VkPresentTimeGOOGLE, VkPresentTimeGOOGLE);
}

/// The earliest time each image should be presented
///
/// When the extension is enabled, additional fields can: be specified that allow an
/// application to specify the earliest time that an image should be displayed. This
/// allows an application to avoid stutter that is caused by an image being
/// displayed earlier than planned. Such stuttering can occur with both fixed and
/// variable-refresh-rate displays, because stuttering occurs when the geometry is
/// not correctly positioned for when the image is displayed. An application can:
/// instruct the presentation engine that an image should not be displayed earlier
/// than a specified time by including the `VkPresentTimesInfoGOOGLE` structure in
/// the `pNext` chain of the `VkPresentInfoKHR` structure.
///
/// The `VkPresentTimesInfoGOOGLE` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `swapchainCount` is the number of swapchains being presented to by this
///     command.
///
///   - `pTimes` is `NULL` or a pointer to an array of `VkPresentTimeGOOGLE`
///     elements with `swapchainCount` entries. If not `NULL`, each element of
///     `pTimes` contains the earliest time to present the image corresponding to
///     the entry in the `VkPresentInfoKHR::pImageIndices` array.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_times(mut self, value: &'a [VkPresentTimeGOOGLE]) -> Self {
    unsafe {
      self.pTimes = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl<'a> Default for VkPresentTimesInfoGOOGLE<'a> {
  fn default() -> VkPresentTimesInfoGOOGLE<'a> {
    VkPresentTimesInfoGOOGLE::new()
  }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl<'a> RawStruct for VkPresentTimesInfoGOOGLE<'a> {
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

/// Structure describing multiview limits that can be supported by an implementation
///
/// The `VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX` structure is
/// defined as.
///
/// The members of the `VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`
/// structure describe the following implementation-dependent limits:
///
///   - `perViewPositionAllComponents` is `VK_TRUE` if the implementation supports
///     per-view position values that differ in components other than the X
///     component.
///
/// If the `VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX` structure is
/// included in the `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled
/// with the implementation-dependent limits.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_per_view_position_all_components(mut self, value: VkBool32) -> Self {
    self.perViewPositionAllComponents = value;
    self
  }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
impl Default for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
  fn default() -> VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX::new()
  }
}
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
impl RawStruct for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
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

/// Structure specifying a viewport swizzle
///
///   - `x` is a `VkViewportCoordinateSwizzleNV` value specifying the swizzle
///     operation to apply to the x component of the primitive
///
///   - `y` is a `VkViewportCoordinateSwizzleNV` value specifying the swizzle
///     operation to apply to the y component of the primitive
///
///   - `z` is a `VkViewportCoordinateSwizzleNV` value specifying the swizzle
///     operation to apply to the z component of the primitive
///
///   - `w` is a `VkViewportCoordinateSwizzleNV` value specifying the swizzle
///     operation to apply to the w component of the primitive
///
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
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl Default for VkViewportSwizzleNV {
  fn default() -> VkViewportSwizzleNV {
    VkViewportSwizzleNV::new()
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl RawStruct for VkViewportSwizzleNV {
  type Raw = types_raw::VkViewportSwizzleNV;
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_viewport_swizzle_nv() {
  assert_size!(types_raw::VkViewportSwizzleNV, VkViewportSwizzleNV);
}

/// Structure specifying swizzle applied to primitive clip coordinates
///
/// Each primitive sent to a given viewport has a swizzle and optional: negation
/// applied to its clip coordinates. The swizzle that is applied depends on the
/// viewport index, and is controlled by the
/// `VkPipelineViewportSwizzleStateCreateInfoNV` pipeline state.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `viewportCount` is the number of viewport swizzles used by the pipeline.
///
///   - `pViewportSwizzles` is a pointer to an array of `VkViewportSwizzleNV`
///     structures, defining the viewport swizzles.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkPipelineViewportSwizzleStateCreateFlagsNV) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_viewport_swizzles(mut self, value: &'a [VkViewportSwizzleNV]) -> Self {
    unsafe {
      self.pViewportSwizzles = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl<'a> Default for VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
  fn default() -> VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
    VkPipelineViewportSwizzleStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl<'a> RawStruct for VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
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

/// Structure describing discard rectangle limits that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceDiscardRectanglePropertiesEXT` structure is defined as.
///
/// The members of the `VkPhysicalDeviceDiscardRectanglePropertiesEXT` structure
/// describe the following implementation-dependent limits:
///
///   - `maxDiscardRectangles` is the maximum number of discard rectangles that can:
///     be specified.
///
/// If the `VkPhysicalDeviceDiscardRectanglePropertiesEXT` structure is included in
/// the `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled with the
/// implementation-dependent limits.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_max_discard_rectangles(mut self, value: u32) -> Self {
    self.maxDiscardRectangles = value;
    self
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl Default for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
  fn default() -> VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    VkPhysicalDeviceDiscardRectanglePropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl RawStruct for VkPhysicalDeviceDiscardRectanglePropertiesEXT {
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

/// Structure specifying discard rectangle
///
/// The discard rectangles test determines if fragment’s framebuffer coordinates
/// (x<sub>f</sub>,y<sub>f</sub>) are inclusive or exclusive to a set of
/// discard-space rectangles. The discard rectangles are set with the
/// `VkPipelineDiscardRectangleStateCreateInfoEXT` pipeline state, which is defined
/// as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `discardRectangleMode` is the mode used to determine whether fragments that
///     lie within the discard rectangle are discarded or not.
///
///   - `discardRectangleCount` is the number of discard rectangles used by the
///     pipeline.
///
///   - `pDiscardRectangles` is a pointer to an array of `VkRect2D` structures,
///     defining the discard rectangles. If the discard rectangle state is dynamic,
///     this member is ignored.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pDiscardRectangles = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl<'a> Default for VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
  fn default() -> VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
    VkPipelineDiscardRectangleStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_discard_rectangles")]
impl<'a> RawStruct for VkPipelineDiscardRectangleStateCreateInfoEXT<'a> {
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

/// Structure describing conservative raster properties that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceConservativeRasterizationPropertiesEXT` structure is
/// defined as.
///
/// The members of the `VkPhysicalDeviceConservativeRasterizationPropertiesEXT`
/// structure describe the following implementation-dependent limits:
///
///   - `primitiveOverestimationSize` is the size in pixels the generating primitive
///     is increased at each of its edges during conservative rasterization
///     overestimation mode. Even with a size of 0.0, conservative rasterization
///     overestimation rules still apply and if any part of the pixel rectangle is
///     covered by the generating primitive, fragments are generated for the entire
///     pixel. However implementations may: make the pixel coverage area even more
///     conservative by increasing the size of the generating primitive.
///
///   - `maxExtraPrimitiveOverestimationSize` is the maximum size in pixels of extra
///     overestimation the implementation supports in the pipeline state. A value of
///     0.0 means the implementation does not support any additional overestimation
///     of the generating primitive during conservative rasterization. A value above
///     0.0 allows the application to further increase the size of the generating
///     primitive during conservative rasterization overestimation.
///
///   - `extraPrimitiveOverestimationSizeGranularity` is the granularity of extra
///     overestimation that can be specified in the pipeline state between 0.0 and
///     `maxExtraPrimitiveOverestimationSize` inclusive. A value of 0.0 means the
///     implementation can use the smallest representable non-zero value in the
///     screen space pixel fixed-point grid.
///
///   - `primitiveUnderestimation` is true if the implementation supports the
///     `VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT` conservative
///     rasterization mode in addition to
///     `VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT`. Otherwise the
///     implementation only supports
///     `VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT`.
///
///   - `conservativePointAndLineRasterization` is true if the implementation
///     supports conservative rasterization of point and line primitives as well as
///     triangle primitives. Otherwise the implementation only supports triangle
///     primitives.
///
///   - `degenerateTrianglesRasterized` is false if the implementation culls
///     primitives generated from triangles that become zero area after they are
///     quantized to the fixed-point rasterization pixel grid.
///     `degenerateTrianglesRasterized` is true if these primitives are not culled
///     and the provoking vertex attributes and depth value are used for the
///     fragments. The primitive area calculation is done on the primitive generated
///     from the clipped triangle if applicable. Zero area primitives are backfacing
///     and the application can: enable backface culling if desired.
///
///   - `degenerateLinesRasterized` is false if the implementation culls lines that
///     become zero length after they are quantized to the fixed-point rasterization
///     pixel grid. `degenerateLinesRasterized` is true if zero length lines are not
///     culled and the provoking vertex attributes and depth value are used for the
///     fragments.
///
///   - `fullyCoveredFragmentShaderInputVariable` is true if the implementation
///     supports the SPIR-V builtin fragment shader input variable FullyCoveredEXT
///     which indicates that conservative rasterization is enabled and the fragment
///     pixel square is fully covered by the generating primitive.
///
///   - `conservativeRasterizationPostDepthCoverage` is true if the implementation
///     supports conservative rasterization with the
///     [`PostDepthCoverage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#shaders-fragment-earlytest-postdepthcoverage)
///     execution mode enabled. When supported the `SampleMask` built-in input
///     variable will reflect the coverage after the early per-fragment depth and
///     stencil tests are applied even when conservative rasterization is enabled.
///     Otherwise
///     [`PostDepthCoverage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#shaders-fragment-earlytest-postdepthcoverage)
///     execution mode must: not be used when conservative rasterization is enabled.
///
/// If the `VkPhysicalDeviceConservativeRasterizationPropertiesEXT` structure is
/// included in the `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled
/// with the implementation-dependent limits and properties.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
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
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl Default for VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
  fn default() -> VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
    VkPhysicalDeviceConservativeRasterizationPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl RawStruct for VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
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

/// Structure specifying conservative raster state
///
/// Polygon rasterization can: be made conservative by setting
/// `conservativeRasterizationMode` to
/// `VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT` or
/// `VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT` in
/// `VkPipelineRasterizationConservativeStateCreateInfoEXT`. The
/// `VkPipelineRasterizationConservativeStateCreateInfoEXT` state is set by adding
/// an instance of this structure to the `pNext` chain of an instance of the
/// `VkPipelineRasterizationStateCreateInfo` structure when creating the graphics
/// pipeline. Enabling these modes also affects line and point rasterization if the
/// implementation sets
/// `VkPhysicalDeviceConservativeRasterizationPropertiesEXT::conservativePointAndLineRasterization`
/// to `VK_TRUE`.
///
/// `VkPipelineRasterizationConservativeStateCreateInfoEXT` is defined as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `conservativeRasterizationMode` is the conservative rasterization mode to
///     use.
///
///   - `extraPrimitiveOverestimationSize` is the extra size in pixels to increase
///     the generating primitive during conservative rasterization at each of its
///     edges in `X` and `Y` equally in screen space beyond the base overestimation
///     specified in
///     `VkPhysicalDeviceConservativeRasterizationPropertiesEXT::primitiveOverestimationSize`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl Default for VkPipelineRasterizationConservativeStateCreateInfoEXT {
  fn default() -> VkPipelineRasterizationConservativeStateCreateInfoEXT {
    VkPipelineRasterizationConservativeStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_conservative_rasterization")]
impl RawStruct for VkPipelineRasterizationConservativeStateCreateInfoEXT {
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

/// structure to specify X,Y chromaticity coordinates
///
/// Chromaticity coordinates x and y are as specified in CIE 15:2004 “Calculation of
/// chromaticity coordinates” (Section 7.3) and are limited to between 0 and 1 for
/// real colors for the mastering display.
///
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
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl Default for VkXYColorEXT {
  fn default() -> VkXYColorEXT {
    VkXYColorEXT::new()
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl RawStruct for VkXYColorEXT {
  type Raw = types_raw::VkXYColorEXT;
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_xy_color_ext() {
  assert_size!(types_raw::VkXYColorEXT, VkXYColorEXT);
}

/// structure to specify Hdr metadata
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `displayPrimaryRed` is the mastering display’s red primary in chromaticity
///     coordinates
///
///   - `displayPrimaryGreen` is the mastering display’s green primary in
///     chromaticity coordinates
///
///   - `displayPrimaryBlue` is the mastering display’s blue primary in chromaticity
///     coordinates
///
///   - `whitePoint` is the mastering display’s white-point in chromaticity
///     coordinates
///
///   - `maxLuminance` is the maximum luminance of the mastering display in nits
///
///   - `minLuminance` is the minimum luminance of the mastering display in nits
///
///   - `maxContentLightLevel` is content’s maximum luminance in nits
///
///   - `maxFrameAverageLightLevel` is the maximum frame average light level in nits
///
/// > **Note**
/// >
/// > The validity and use of this data is outside the scope of Vulkan and thus no
/// > Valid Usage is given.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl Default for VkHdrMetadataEXT {
  fn default() -> VkHdrMetadataEXT {
    VkHdrMetadataEXT::new()
  }
}
#[cfg(feature = "VK_EXT_hdr_metadata")]
impl RawStruct for VkHdrMetadataEXT {
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
///
/// The `VkPhysicalDeviceSurfaceInfo2KHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `surface` is the surface that will be associated with the swapchain.
///
/// The members of `VkPhysicalDeviceSurfaceInfo2KHR` correspond to the arguments to
/// `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`, with `sType` and `pNext` added for
/// extensibility.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_surface(mut self, value: VkSurfaceKHR) -> Self {
    self.surface = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl Default for VkPhysicalDeviceSurfaceInfo2KHR {
  fn default() -> VkPhysicalDeviceSurfaceInfo2KHR {
    VkPhysicalDeviceSurfaceInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl RawStruct for VkPhysicalDeviceSurfaceInfo2KHR {
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
///
/// The `VkSurfaceCapabilities2KHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `surfaceCapabilities` is a structure of type `VkSurfaceCapabilitiesKHR`
///     describing the capabilities of the specified surface.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_surface_capabilities(mut self, value: VkSurfaceCapabilitiesKHR) -> Self {
    self.surfaceCapabilities = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl Default for VkSurfaceCapabilities2KHR {
  fn default() -> VkSurfaceCapabilities2KHR {
    VkSurfaceCapabilities2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl RawStruct for VkSurfaceCapabilities2KHR {
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
///
/// The `VkSurfaceFormat2KHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `surfaceFormat` is an instance of `VkSurfaceFormatKHR` describing a
///     format-color space pair that is compatible with the specified surface.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_surface_format(mut self, value: VkSurfaceFormatKHR) -> Self {
    self.surfaceFormat = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl Default for VkSurfaceFormat2KHR {
  fn default() -> VkSurfaceFormat2KHR {
    VkSurfaceFormat2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
impl RawStruct for VkSurfaceFormat2KHR {
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
///
/// The `VkSharedPresentSurfaceCapabilitiesKHR` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `sharedPresentSupportedUsageFlags` is a bitmask of `VkImageUsageFlagBits`
///     representing the ways the application can: use the shared presentable image
///     from a swapchain created with `VkPresentModeKHR` set to
///     `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
///     `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` for the surface on the
///     specified device. `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` must: be included in
///     the set but implementations may: support additional usages.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_shared_present_supported_usage_flags(mut self, value: VkImageUsageFlags) -> Self {
    self.sharedPresentSupportedUsageFlags = value;
    self
  }
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
impl Default for VkSharedPresentSurfaceCapabilitiesKHR {
  fn default() -> VkSharedPresentSurfaceCapabilitiesKHR {
    VkSharedPresentSurfaceCapabilitiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
impl RawStruct for VkSharedPresentSurfaceCapabilitiesKHR {
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

/// Structure specifying fence creation parameters.
///
///   - `sType` is the type of this structure
///
///   - `pNext` is NULL or a pointer to an extension-specific structure.
///
///   - `handleType` is a `VkExternalFenceHandleTypeFlagBitsKHR` value indicating an
///     external fence handle type for which capabilities will be returned.
///
/// > **Note**
/// >
/// > Handles of type `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR` generated by
/// > the implementation may represent either Linux Sync Files or Android Fences at
/// > the implementation’s discretion. Applications should: only use operations
/// > defined for both types of file descriptors, unless they know via means
/// > external to Vulkan the type of the file descriptor, or are prepared to deal
/// > with the system-defined operation failures resulting from using the wrong
/// > type.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalFenceHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl Default for VkPhysicalDeviceExternalFenceInfoKHR {
  fn default() -> VkPhysicalDeviceExternalFenceInfoKHR {
    VkPhysicalDeviceExternalFenceInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl RawStruct for VkPhysicalDeviceExternalFenceInfoKHR {
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
///
///   - `exportFromImportedHandleTypes` is a bitmask of
///     `VkExternalFenceHandleTypeFlagBitsKHR` indicating which types of imported
///     handle `handleType` can: be exported from.
///
///   - `compatibleHandleTypes` is a bitmask of
///     `VkExternalFenceHandleTypeFlagBitsKHR` specifying handle types which can: be
///     specified at the same time as `handleType` when creating a fence.
///
///   - `externalFenceFeatures` is a bitmask of `VkExternalFenceFeatureFlagBitsKHR`
///     indicating the features of `handleType`.
///
/// If `handleType` is not supported by the implementation, then
/// `VkExternalFencePropertiesKHR::externalFenceFeatures` will be set to zero.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl Default for VkExternalFencePropertiesKHR {
  fn default() -> VkExternalFencePropertiesKHR {
    VkExternalFencePropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
impl RawStruct for VkExternalFencePropertiesKHR {
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
///
/// To create a fence whose payload can: be exported to external handles, add the
/// `VkExportFenceCreateInfoKHR` structure to the `pNext` chain of the
/// `VkFenceCreateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleTypes` is a bitmask of `VkExternalFenceHandleTypeFlagBitsKHR`
///     specifying one or more fence handle types the application can: export from
///     the resulting fence. The application can: request multiple handle types for
///     the same fence.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_types(mut self, value: VkExternalFenceHandleTypeFlagsKHR) -> Self {
    self.handleTypes = value;
    self
  }
}
#[cfg(feature = "VK_KHR_external_fence")]
impl Default for VkExportFenceCreateInfoKHR {
  fn default() -> VkExportFenceCreateInfoKHR {
    VkExportFenceCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence")]
impl RawStruct for VkExportFenceCreateInfoKHR {
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

/// (None)
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `fence` is the fence into which the state will be imported.
///
///   - `flags` is a bitmask of `VkFenceImportFlagBitsKHR` specifying additional
///     parameters for the fence payload import operation.
///
///   - `handleType` specifies the type of `handle`.
///
///   - `handle` is the external handle to import, or `NULL`.
///
///   - `name` is the NULL-terminated UTF-16 string naming the underlying
///     synchronization primitive to import, or `NULL`.
///
/// The handle types supported by `handleType` are:
///
/// <table>
/// <caption>Handle Types Supported by VkImportFenceWin32HandleInfoKHR</caption>
/// <colgroup>
/// <col width="33%" />
/// <col width="33%" />
/// <col width="33%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Handle Type</th>
/// <th align="left">Transference</th>
/// <th align="left">Permanence Supported</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"><p><code>VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR</code></p></td>
/// <td align="left"><p>Reference</p></td>
/// <td align="left"><p>Temporary,Permanent</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><code>VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR</code></p></td>
/// <td align="left"><p>Reference</p></td>
/// <td align="left"><p>Temporary,Permanent</p></td>
/// </tr>
/// </tbody>
/// </table>
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkImportFenceWin32HandleInfoKHR {
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
///
/// To specify additional attributes of NT handles exported from a fence, add the
/// `VkExportFenceWin32HandleInfoKHR` structure to the `pNext` chain of the
/// `VkFenceCreateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `pAttributes` is a pointer to a Windows `SECURITY_ATTRIBUTES` structure
///     specifying security attributes of the handle.
///
///   - `dwAccess` is a `DWORD` specifying access rights of the handle.
///
///   - `name` is a NULL-terminated UTF-16 string to associate with the underlying
///     synchronization primitive referenced by NT handles exported from the created
///     fence.
///
/// If this structure is not present, or if `pAttributes` is set to `NULL`, default
/// security descriptor values will be used, and child processes created by the
/// application will not inherit the handle, as described in the MSDN documentation
/// for “Synchronization Object Security and Access Rights”<sup>1</sup>. Further, if
/// the structure is not present, the access rights will be
///
/// `DXGI_SHARED_RESOURCE_READ` | `DXGI_SHARED_RESOURCE_WRITE`
///
/// for handles of the following types:
///
/// `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`
///
///   - 1
///     [https://msdn.microsoft.com/en-us/library/windows/desktop/ms686670.aspx](#)
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
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
impl RawStruct for VkExportFenceWin32HandleInfoKHR {
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

/// Structure describing a Win32 handle fence export operation
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `fence` is the fence from which state will be exported.
///
///   - `handleType` is the type of handle requested.
///
/// The properties of the handle returned depend on the value of `handleType`. See
/// `VkExternalFenceHandleTypeFlagBitsKHR` for a description of the properties of
/// the defined external fence handle types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
impl RawStruct for VkFenceGetWin32HandleInfoKHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `fence` is the fence into which the payload will be imported.
///
///   - `flags` is a bitmask of `VkFenceImportFlagBitsKHR` specifying additional
///     parameters for the fence payload import operation.
///
///   - `handleType` specifies the type of `fd`.
///
///   - `fd` is the external handle to import.
///
/// The handle types supported by `handleType` are:
///
/// <table>
/// <caption>Handle Types Supported by VkImportFenceFdInfoKHR</caption>
/// <colgroup>
/// <col width="33%" />
/// <col width="33%" />
/// <col width="33%" />
/// </colgroup>
/// <thead>
/// <tr class="header">
/// <th align="left">Handle Type</th>
/// <th align="left">Transference</th>
/// <th align="left">Permanence Supported</th>
/// </tr>
/// </thead>
/// <tbody>
/// <tr class="odd">
/// <td align="left"><p><code>VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR</code></p></td>
/// <td align="left"><p>Reference</p></td>
/// <td align="left"><p>Temporary,Permanent</p></td>
/// </tr>
/// <tr class="even">
/// <td align="left"><p><code>VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR</code></p></td>
/// <td align="left"><p>Copy</p></td>
/// <td align="left"><p>Temporary</p></td>
/// </tr>
/// </tbody>
/// </table>
///
/// If `handleType` is `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR`, the special
/// value `-1` for `fd` is treated like a valid sync file descriptor referring to an
/// object that has already signaled. The import operation will succeed and the
/// `VkFence` will have a temporarily imported payload as if a valid file descriptor
/// had been provided.
///
/// > **Note**
/// >
/// > This special behavior for importing an invalid sync file descriptor allows
/// > easier interoperability with other system software which uses the convention
/// > that an invalid sync file descriptor represents work that has already
/// > completed and doesn’t need to be waited for. It is consistent with the option
/// > for implementations to return a `-1` file descriptor when exporting a
/// > `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR` from a `VkFence` which is
/// > signaled.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl Default for VkImportFenceFdInfoKHR {
  fn default() -> VkImportFenceFdInfoKHR {
    VkImportFenceFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl RawStruct for VkImportFenceFdInfoKHR {
  type Raw = types_raw::VkImportFenceFdInfoKHR;
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_import_fence_fd_info_khr() {
  assert_size!(types_raw::VkImportFenceFdInfoKHR, VkImportFenceFdInfoKHR);
}

/// Structure describing a POSIX FD fence export operation
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `fence` is the fence from which state will be exported.
///
///   - `handleType` is the type of handle requested.
///
/// The properties of the file descriptor returned depend on the value of
/// `handleType`. See `VkExternalFenceHandleTypeFlagBitsKHR` for a description of
/// the properties of the defined external fence handle types.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl Default for VkFenceGetFdInfoKHR {
  fn default() -> VkFenceGetFdInfoKHR {
    VkFenceGetFdInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_external_fence_fd")]
impl RawStruct for VkFenceGetFdInfoKHR {
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
///
/// The `VkPhysicalDevicePointClippingPropertiesKHR` structure is defined as.
///
/// The members of the `VkPhysicalDevicePointClippingPropertiesKHR` structure
/// describe the following implementation-dependent limit:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `pointClippingBehavior` is the point clipping behavior supported by the
///     implementation, and is of type `VkPointClippingBehaviorKHR`.
///
/// If the `VkPhysicalDevicePointClippingPropertiesKHR` structure is included in the
/// `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled with the
/// implementation-dependent limits.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_point_clipping_behavior(mut self, value: VkPointClippingBehaviorKHR) -> Self {
    self.pointClippingBehavior = value;
    self
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl Default for VkPhysicalDevicePointClippingPropertiesKHR {
  fn default() -> VkPhysicalDevicePointClippingPropertiesKHR {
    VkPhysicalDevicePointClippingPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl RawStruct for VkPhysicalDevicePointClippingPropertiesKHR {
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

/// Structure specifying a subpass/input attachment pair and an aspect mask that
/// can: be read.
///
/// The `VkInputAttachmentAspectReferenceKHR` structure specifies an aspect mask for
/// a specific input attachment of a specific subpass in the render pass.
///
/// The `subpass` and `inputAttachment` index into the render pass as:
///
/// `pCreateInfo::pSubpasses`\[`subpass`\].`pInputAttachments`\[`inputAttachment`\]
///
///   - `subpass` is an index into the `pSubpasses` array of the parent
///     `VkRenderPassCreateInfo` structure.
///
///   - `inputAttachment` is an index into the `pInputAttachments` of the specified
///     subpass.
///
///   - `aspectMask` is a mask of which aspect(s) can: be accessed within the
///     specified subpass.
///
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
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl Default for VkInputAttachmentAspectReferenceKHR {
  fn default() -> VkInputAttachmentAspectReferenceKHR {
    VkInputAttachmentAspectReferenceKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl RawStruct for VkInputAttachmentAspectReferenceKHR {
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
///
/// To specify which aspects of an input attachment can: be read add a
/// `VkRenderPassInputAttachmentAspectCreateInfoKHR` structure to the `pNext` chain
/// of the `VkRenderPassCreateInfo` structure:
///
/// The `VkRenderPassInputAttachmentAspectCreateInfoKHR` structure is defined as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `aspectReferenceCount` is the number of elements in the pAspectReferences
///     array.
///
///   - `pAspectReferences` points to an array of `aspectReferenceCount` number of
///     `VkInputAttachmentAspectReferenceKHR` structures describing which aspect(s)
///     can: be accessed for a given input attachment within a given subpass.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_aspect_references(mut self, value: &'a [VkInputAttachmentAspectReferenceKHR]) -> Self {
    unsafe {
      self.pAspectReferences = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'a> Default for VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
  fn default() -> VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
    VkRenderPassInputAttachmentAspectCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl<'a> RawStruct for VkRenderPassInputAttachmentAspectCreateInfoKHR<'a> {
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

/// Specify the intended usage of an image view
///
/// The set of usages for the created image view can: be restricted compared to the
/// parent image’s `usage` flags by chaining a `VkImageViewUsageCreateInfoKHR`
/// structure through the `pNext` member to `VkImageViewCreateInfo`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `usage` is a bitmask describing the allowed usages of the image view. See
///     `VkImageUsageFlagBits` for a description of the supported bits.
///
/// When this structure is chained to `VkImageViewCreateInfo` the `usage` field
/// overrides the implicit `usage` parameter inherited from image creation time and
/// its value is used instead for the purposes of determining the valid usage
/// conditions of `VkImageViewCreateInfo`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_usage(mut self, value: VkImageUsageFlags) -> Self {
    self.usage = value;
    self
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl Default for VkImageViewUsageCreateInfoKHR {
  fn default() -> VkImageViewUsageCreateInfoKHR {
    VkImageViewUsageCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl RawStruct for VkImageViewUsageCreateInfoKHR {
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

/// Structure specifying the orientation of the tessellation domain
///
/// The `VkPipelineTessellationDomainOriginStateCreateInfoKHR` structure is defined
/// as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `domainOrigin` controls the origin of the tessellation domain space, and is
///     of type `VkTessellationDomainOriginKHR`.
///
/// If the `VkPipelineTessellationDomainOriginStateCreateInfoKHR` structure is
/// included in the `pNext` chain of `VkPipelineTessellationStateCreateInfo`, it
/// controls the origin of the tessellation domain. If this structure is not
/// present, it is as if `domainOrigin` were
/// `VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_domain_origin(mut self, value: VkTessellationDomainOriginKHR) -> Self {
    self.domainOrigin = value;
    self
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl Default for VkPipelineTessellationDomainOriginStateCreateInfoKHR {
  fn default() -> VkPipelineTessellationDomainOriginStateCreateInfoKHR {
    VkPipelineTessellationDomainOriginStateCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_maintenance2")]
impl RawStruct for VkPipelineTessellationDomainOriginStateCreateInfoKHR {
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

/// Structure describing variable pointers features that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceVariablePointerFeaturesKHR` structure is defined as.
///
/// The members of the `VkPhysicalDeviceVariablePointerFeaturesKHR` structure
/// describe the following features:
///
///   - `variablePointersStorageBuffer` indicates whether the implementation
///     supports the SPIR-V VariablePointersStorageBuffer capability. When this
///     feature is not enabled, shader modules must: not declare the
///     SPV\_KHR\_variable\_pointers extension or the VariablePointersStorageBuffer
///     capability.
///
///   - `variablePointers` indicates whether the implementation supports the SPIR-V
///     VariablePointers capability. When this feature is not enabled, shader
///     modules must: not declare the VariablePointers capability.
///
/// If the `VkPhysicalDeviceVariablePointerFeaturesKHR` structure is included in the
/// `pNext` chain of `VkPhysicalDeviceFeatures2KHR`, it is filled with values
/// indicating whether each feature is supported.
/// `VkPhysicalDeviceVariablePointerFeaturesKHR` can: also be used in the `pNext`
/// chain of `VkDeviceCreateInfo` to enable the features.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_variable_pointers")]
impl Default for VkPhysicalDeviceVariablePointerFeaturesKHR {
  fn default() -> VkPhysicalDeviceVariablePointerFeaturesKHR {
    VkPhysicalDeviceVariablePointerFeaturesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_variable_pointers")]
impl RawStruct for VkPhysicalDeviceVariablePointerFeaturesKHR {
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

/// Structure specifying parameters of a newly created iOS surface object
///
/// The `VkIOSSurfaceCreateInfoMVK` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `pView` is a reference to a `UIView` object which will display this surface.
///     This `UIView` must: be backed by a `CALayer` instance of type
///     `CAMetalLayer`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkIOSSurfaceCreateFlagsMVK) -> Self {
    self.flags = value;
    self
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
impl RawStruct for VkIOSSurfaceCreateInfoMVK {
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
///
/// The `VkMacOSSurfaceCreateInfoMVK` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `pView` is a reference to a `NSView` object which will display this surface.
///     This `NSView` must: be backed by a `CALayer` instance of type
///     `CAMetalLayer`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkMacOSSurfaceCreateFlagsMVK) -> Self {
    self.flags = value;
    self
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
impl RawStruct for VkMacOSSurfaceCreateInfoMVK {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `buffer` is the buffer to query.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_buffer(mut self, value: VkBuffer) -> Self {
    self.buffer = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkBufferMemoryRequirementsInfo2KHR {
  fn default() -> VkBufferMemoryRequirementsInfo2KHR {
    VkBufferMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl RawStruct for VkBufferMemoryRequirementsInfo2KHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `image` is the image to query.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkImageMemoryRequirementsInfo2KHR {
  fn default() -> VkImageMemoryRequirementsInfo2KHR {
    VkImageMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl RawStruct for VkImageMemoryRequirementsInfo2KHR {
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_image(mut self, value: VkImage) -> Self {
    self.image = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkImageSparseMemoryRequirementsInfo2KHR {
  fn default() -> VkImageSparseMemoryRequirementsInfo2KHR {
    VkImageSparseMemoryRequirementsInfo2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl RawStruct for VkImageSparseMemoryRequirementsInfo2KHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `memoryRequirements` is a structure of type `VkMemoryRequirements`
///     describing the memory requirements of the resource.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_memory_requirements(mut self, value: VkMemoryRequirements) -> Self {
    self.memoryRequirements = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkMemoryRequirements2KHR {
  fn default() -> VkMemoryRequirements2KHR {
    VkMemoryRequirements2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl RawStruct for VkMemoryRequirements2KHR {
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_memory_requirements(mut self, value: VkSparseImageMemoryRequirements) -> Self {
    self.memoryRequirements = value;
    self
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl Default for VkSparseImageMemoryRequirements2KHR {
  fn default() -> VkSparseImageMemoryRequirements2KHR {
    VkSparseImageMemoryRequirements2KHR::new()
  }
}
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
impl RawStruct for VkSparseImageMemoryRequirements2KHR {
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
///
/// To determine the dedicated allocation requirements of a buffer or image
/// resource, add a `VkMemoryDedicatedRequirementsKHR` structure to the `pNext`
/// chain of the `VkMemoryRequirements2KHR` structure passed as the
/// `pMemoryRequirements` parameter of `vkGetBufferMemoryRequirements2KHR` or
/// `vkGetImageMemoryRequirements2KHR`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `prefersDedicatedAllocation` indicates that the implementation would prefer
///     a dedicated allocation for this resource. The application is still free to
///     suballocate the resource but it may: get better performance if a dedicated
///     allocation is used.
///
///   - `requiresDedicatedAllocation` indicates that a dedicated allocation is
///     required for this resource.
///
/// If the `VkMemoryDedicatedRequirementsKHR` structure is included in the `pNext`
/// chain of the `VkMemoryRequirements2KHR` structure passed as the
/// `pMemoryRequirements` parameter of a `vkGetBufferMemoryRequirements2KHR` call,
/// `requiresDedicatedAllocation` may: be `VK_TRUE` under one of the following
/// conditions:
///
///   - The `pNext` chain of `VkBufferCreateInfo` for the call to `vkCreateBuffer`
///     used to create the buffer being queried contained an instance of
///     `VkExternalMemoryBufferCreateInfoKHR`, and any of the handle types specified
///     in `VkExternalMemoryBufferCreateInfoKHR::handleTypes` requires dedicated
///     allocation, as reported by `vkGetPhysicalDeviceExternalBufferPropertiesKHR`
///     in
///     `VkExternalBufferPropertiesKHR::externalMemoryProperties`::\`externalMemoryFeatures\`,
///     the `requiresDedicatedAllocation` field will be set to `VK_TRUE`.
///
/// In all other cases, `requiresDedicatedAllocation` must: be set to `VK_FALSE` by
/// the implementation whenever a `VkMemoryDedicatedRequirementsKHR` structure is
/// included in the `pNext` chain of the `VkMemoryRequirements2KHR` structure passed
/// to a call to `vkGetBufferMemoryRequirements2KHR`.
///
/// If the `VkMemoryDedicatedRequirementsKHR` structure is included in the `pNext`
/// chain of the `VkMemoryRequirements2KHR` structure passed as the
/// `pMemoryRequirements` parameter of a `vkGetBufferMemoryRequirements2KHR` call
/// and `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` was set in `VkBufferCreateInfo::flags`
/// when `buffer` was created then the implementation must: set both
/// `prefersDedicatedAllocation` and `requiresDedicatedAllocation` to `VK_FALSE`.
///
/// If the `VkMemoryDedicatedRequirementsKHR` structure is included in the `pNext`
/// chain of the `VkMemoryRequirements2KHR` structure passed as the
/// `pMemoryRequirements` parameter of a `vkGetImageMemoryRequirements2KHR` call,
/// `requiresDedicatedAllocation` may: be `VK_TRUE` under one of the following
/// conditions:
///
///   - The `pNext` chain of `VkImageCreateInfo` for the call to `vkCreateImage`
///     used to create the image being queried contained an instance of
///     `VkExternalMemoryImageCreateInfoKHR`, and any of the handle types specified
///     in `VkExternalMemoryImageCreateInfoKHR::handleTypes` requires dedicated
///     allocation, as reported by `vkGetPhysicalDeviceImageFormatProperties2KHR` in
///     `VkExternalImageFormatPropertiesKHR::externalMemoryProperties`::\`externalMemoryFeatures\`,
///     the `requiresDedicatedAllocation` field will be set to `VK_TRUE`.
///
/// In all other cases, `requiresDedicatedAllocation` must: be set to `VK_FALSE` by
/// the implementation whenever a `VkMemoryDedicatedRequirementsKHR` structure is
/// included in the `pNext` chain of the `VkMemoryRequirements2KHR` structure passed
/// to a call to `vkGetImageMemoryRequirements2KHR`.
///
/// If the `VkMemoryDedicatedRequirementsKHR` structure is included in the `pNext`
/// chain of the `VkMemoryRequirements2KHR` structure passed as the
/// `pMemoryRequirements` parameter of a `vkGetImageMemoryRequirements2KHR` call and
/// `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` was set in `VkImageCreateInfo::flags` when
/// `image` was created then the implementation must: set both
/// `prefersDedicatedAllocation` and `requiresDedicatedAllocation` to `VK_FALSE`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl Default for VkMemoryDedicatedRequirementsKHR {
  fn default() -> VkMemoryDedicatedRequirementsKHR {
    VkMemoryDedicatedRequirementsKHR::new()
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl RawStruct for VkMemoryDedicatedRequirementsKHR {
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

/// Specify a dedicated memory allocation resource
///
/// If the `pNext` chain includes a `VkMemoryDedicatedAllocateInfoKHR` structure,
/// then that structure includes a handle of the sole buffer or image resource that
/// the memory can: be bound to.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `image` is `VK_NULL_HANDLE` or a handle of an image which this memory will
///     be bound to.
///
///   - `buffer` is `VK_NULL_HANDLE` or a handle of a buffer which this memory will
///     be bound to.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl Default for VkMemoryDedicatedAllocateInfoKHR {
  fn default() -> VkMemoryDedicatedAllocateInfoKHR {
    VkMemoryDedicatedAllocateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_dedicated_allocation")]
impl RawStruct for VkMemoryDedicatedAllocateInfoKHR {
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

/// Structure specifying sampler reduction mode
///
/// If the `pNext` chain of `VkSamplerCreateInfo` includes a
/// `VkSamplerReductionModeCreateInfoEXT` structure, then that structure includes a
/// mode that controls how texture filtering combines texel values.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `reductionMode` is an enum of type `VkSamplerReductionModeEXT` that controls
///     how texture filtering combines texel values.
///
/// If this structure is not present, `reductionMode` is considered to be
/// `VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_reduction_mode(mut self, value: VkSamplerReductionModeEXT) -> Self {
    self.reductionMode = value;
    self
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl Default for VkSamplerReductionModeCreateInfoEXT {
  fn default() -> VkSamplerReductionModeCreateInfoEXT {
    VkSamplerReductionModeCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl RawStruct for VkSamplerReductionModeCreateInfoEXT {
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

/// Structure describing sampler filter minmax limits that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT` structure is defined as.
///
/// The members of the `VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT` structure
/// describe the following implementation-dependent limits:
///
///   - `filterMinmaxSingleComponentFormats` is a boolean value indicating whether a
///     minimum set of required formats support min/max filtering.
///
///   - `filterMinmaxImageComponentMapping` is a boolean value indicating whether
///     the implementation supports non-identity component mapping of the image when
///     doing min/max filtering.
///
/// If the `VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT` structure is included
/// in the `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled with the
/// implementation-dependent limits.
///
/// If `filterMinmaxSingleComponentFormats` is `VK_TRUE`, the following formats
/// must: support the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT`
/// feature with `VK_IMAGE_TILING_OPTIMAL`, if they support
/// `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`.
///
///   - `VK_FORMAT_R8_UNORM`
///
///   - `VK_FORMAT_R8_SNORM`
///
///   - `VK_FORMAT_R16_UNORM`
///
///   - `VK_FORMAT_R16_SNORM`
///
///   - `VK_FORMAT_R16_SFLOAT`
///
///   - `VK_FORMAT_R32_SFLOAT`
///
///   - `VK_FORMAT_D16_UNORM`
///
///   - `VK_FORMAT_X8_D24_UNORM_PACK32`
///
///   - `VK_FORMAT_D32_SFLOAT`
///
///   - `VK_FORMAT_D16_UNORM_S8_UINT`
///
///   - `VK_FORMAT_D24_UNORM_S8_UINT`
///
///   - `VK_FORMAT_D32_SFLOAT_S8_UINT`
///
/// If the format is a depth/stencil format, this bit only indicates that the depth
/// aspect (not the stencil aspect) of an image of this format supports min/max
/// filtering, and that min/max filtering of the depth aspect is supported when
/// depth compare is disabled in the sampler.
///
/// If `filterMinmaxImageComponentMapping` is `VK_FALSE` the component mapping of
/// the image view used with min/max filtering must: have been created with the `r`
/// component set to `VK_COMPONENT_SWIZZLE_IDENTITY`. Only the `r` component of the
/// sampled image value is defined and the other component values are undefined. If
/// `filterMinmaxImageComponentMapping` is `VK_TRUE` this restriction does not apply
/// and image component mapping works as normal.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl Default for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
  fn default() -> VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
impl RawStruct for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
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

/// Structure specifying the coordinates of a sample location
///
///   - `x` is the horizontal coordinate of the sample’s location.
///
///   - `y` is the vertical coordinate of the sample’s location.
///
/// The domain space of the sample location coordinates has an upper-left origin
/// within the pixel in framebuffer space.
///
/// The values specified in a `VkSampleLocationEXT` structure are always clamped to
/// the implementation-dependent sample location coordinate range
/// \[`sampleLocationCoordinateRange`\[0\],`sampleLocationCoordinateRange`\[1\]\]
/// that can: be queried by chaining the
/// `VkPhysicalDeviceSampleLocationsPropertiesEXT` structure to the `pNext` chain of
/// `VkPhysicalDeviceProperties2KHR`.
///
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
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl Default for VkSampleLocationEXT {
  fn default() -> VkSampleLocationEXT {
    VkSampleLocationEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl RawStruct for VkSampleLocationEXT {
  type Raw = types_raw::VkSampleLocationEXT;
}
#[cfg(feature = "VK_EXT_sample_locations")]
#[cfg(test)]
#[test]
fn test_struct_size_vk_sample_location_ext() {
  assert_size!(types_raw::VkSampleLocationEXT, VkSampleLocationEXT);
}

/// Structure specifying a set of sample locations
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `sampleLocationsPerPixel` is a `VkSampleCountFlagBits` specifying the number
///     of sample locations per pixel.
///
///   - `sampleLocationGridSize` is the size of the sample location grid to select
///     custom sample locations for.
///
///   - `sampleLocationsCount` is the number of sample locations in
///     `pSampleLocations`.
///
///   - `pSampleLocations` is an array of `sampleLocationsCount`
///     `VkSampleLocationEXT` structures.
///
/// This structure can: be used either to specify the sample locations to be used
/// for rendering or to specify the set of sample locations an image subresource has
/// been last rendered with for the purposes of layout transitions of depth/stencil
/// images created with `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT`.
///
/// The sample locations in `pSampleLocations` specify `sampleLocationsPerPixel`
/// number of sample locations for each pixel in the grid of the size specified in
/// `sampleLocationGridSize`. The sample location for sample i at the pixel grid
/// location (x,y) is taken from `pSampleLocations`\[(x + y \*
/// `sampleLocationGridSize`.width) \* `sampleLocationsPerPixel` + i\].
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pSampleLocations = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> Default for VkSampleLocationsInfoEXT<'a> {
  fn default() -> VkSampleLocationsInfoEXT<'a> {
    VkSampleLocationsInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> RawStruct for VkSampleLocationsInfoEXT<'a> {
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

/// Structure specifying the sample locations state to use in the initial layout
/// transition of attachments
///
///   - `attachmentIndex` is the index of the attachment for which the sample
///     locations state is provided.
///
///   - `sampleLocationsInfo` is the sample locations state to use for the layout
///     transition of the given attachment from the initial layout of the attachment
///     to the image layout specified for the attachment in the first subpass using
///     it.
///
/// If the image referenced by the framebuffer attachment at index `attachmentIndex`
/// was not created with `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT`
/// then the values specified in `sampleLocationsInfo` are ignored.
///
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
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> Default for VkAttachmentSampleLocationsEXT<'a> {
  fn default() -> VkAttachmentSampleLocationsEXT<'a> {
    VkAttachmentSampleLocationsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> RawStruct for VkAttachmentSampleLocationsEXT<'a> {
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
///
///   - `subpassIndex` is the index of the subpass for which the sample locations
///     state is provided.
///
///   - `sampleLocationsInfo` is the sample locations state to use for the layout
///     transition of the depth/stencil attachment away from the image layout the
///     attachment is used with in the subpass specified in `subpassIndex`.
///
/// If the image referenced by the depth/stencil attachment used in the subpass
/// identified by `subpassIndex` was not created with
/// `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` or if the subpass
/// does not use a depth/stencil attachment, and
/// `VkPhysicalDeviceSampleLocationsPropertiesEXT::variableSampleLocations` is
/// `VK_TRUE` then the values specified in `sampleLocationsInfo` are ignored.
///
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
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> Default for VkSubpassSampleLocationsEXT<'a> {
  fn default() -> VkSubpassSampleLocationsEXT<'a> {
    VkSubpassSampleLocationsEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> RawStruct for VkSubpassSampleLocationsEXT<'a> {
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
///
/// The image layout of the depth aspect of a depth/stencil attachment referring to
/// an image created with
/// `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` is dependent on the
/// last sample locations used to render to the image subresource, thus preserving
/// the contents of such depth/stencil attachments across subpass boundaries
/// requires the application to specify these sample locations whenever a layout
/// transition of the attachment may: occur. This information can: be provided by
/// chaining an instance of the `VkRenderPassSampleLocationsBeginInfoEXT` structure
/// to the `pNext` chain of `VkRenderPassBeginInfo`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `attachmentInitialSampleLocationsCount` is the number of elements in the
///     `pAttachmentInitialSampleLocations` array.
///
///   - `pAttachmentInitialSampleLocations` is an array of
///     `attachmentInitialSampleLocationsCount` `VkAttachmentSampleLocationsEXT`
///     structures specifying the attachment indices and their corresponding sample
///     location state. Each element of `pAttachmentInitialSampleLocations` can:
///     specify the sample location state to use in the automatic layout transition
///     performed to transition a depth/stencil attachment from the initial layout
///     of the attachment to the image layout specified for the attachment in the
///     first subpass using it.
///
///   - `postSubpassSampleLocationsCount` is the number of elements in the
///     `pPostSubpassSampleLocations` array.
///
///   - `pPostSubpassSampleLocations` is an array of
///     `postSubpassSampleLocationsCount` `VkSubpassSampleLocationsEXT` structures
///     specifying the subpass indices and their corresponding sample location
///     state. Each element of `pPostSubpassSampleLocations` can: specify the sample
///     location state to use in the automatic layout transition performed to
///     transition the depth/stencil attachment used by the specified subpass to the
///     image layout specified in a dependent subpass or to the final layout of the
///     attachment in case the specified subpass is the last subpass using that
///     attachment. In addition, if
///     `VkPhysicalDeviceSampleLocationsPropertiesEXT::variableSampleLocations` is
///     `VK_FALSE`, each element of `pPostSubpassSampleLocations` must: specify the
///     sample location state that matches the sample locations used by all
///     pipelines that will be bound to a command buffer during the specified
///     subpass. If `variableSampleLocations` is `VK_TRUE`, the sample locations
///     used for rasterization do not depend on `pPostSubpassSampleLocations`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_attachment_initial_sample_locations(mut self, value: &'a [VkAttachmentSampleLocationsEXT<'a>]) -> Self {
    unsafe {
      self.pAttachmentInitialSampleLocations = value.as_raw();
    }
    self
  }
  #[inline]
  pub fn set_post_subpass_sample_locations(mut self, value: &'a [VkSubpassSampleLocationsEXT<'a>]) -> Self {
    unsafe {
      self.pPostSubpassSampleLocations = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> Default for VkRenderPassSampleLocationsBeginInfoEXT<'a> {
  fn default() -> VkRenderPassSampleLocationsBeginInfoEXT<'a> {
    VkRenderPassSampleLocationsBeginInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> RawStruct for VkRenderPassSampleLocationsBeginInfoEXT<'a> {
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

/// Structure specifying sample locations for a pipeline
///
/// Applications can: also control the sample locations used for rasterization.
///
/// If the `pNext` chain of the `VkPipelineMultisampleStateCreateInfo` structure
/// specified at pipeline creation time includes an instance of the
/// `VkPipelineSampleLocationsStateCreateInfoEXT` structure, then that structure
/// controls the sample locations used when rasterizing primitives with the
/// pipeline.
///
/// The `VkPipelineSampleLocationsStateCreateInfoEXT` structure is defined as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `sampleLocationsEnable` controls whether custom sample locations are used.
///     If `sampleLocationsEnable` is `VK_FALSE`, the default sample locations are
///     used and the values specified in `sampleLocationsInfo` are ignored.
///
///   - `sampleLocationsInfo` is the sample locations to use during rasterization if
///     `sampleLocationsEnable` is `VK_TRUE` and the graphics pipeline isn’t created
///     with `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> Default for VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
  fn default() -> VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
    VkPipelineSampleLocationsStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl<'a> RawStruct for VkPipelineSampleLocationsStateCreateInfoEXT<'a> {
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

/// Structure describing sample location limits that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceSampleLocationsPropertiesEXT` structure is defined as.
///
/// The members of the `VkPhysicalDeviceSampleLocationsPropertiesEXT` structure
/// describe the following implementation-dependent limits:
///
///   - `sampleLocationSampleCounts` is a bitmask of `VkSampleCountFlagBits`
///     indicating the sample counts supporting custom sample locations.
///
///   - `maxSampleLocationGridSize` is the maximum size of the pixel grid in which
///     sample locations can: vary that is supported for all sample counts in
///     `sampleLocationSampleCounts`.
///
///   - `sampleLocationCoordinateRange`\[2\] is the range of supported sample
///     location coordinates.
///
///   - `sampleLocationSubPixelBits` is the number of bits of subpixel precision for
///     sample locations.
///
///   - `variableSampleLocations` indicates whether the sample locations used by all
///     pipelines that will be bound to a command buffer during a subpass must:
///     match. If set to `VK_TRUE`, the implementation supports variable sample
///     locations in a subpass. If set to `VK_FALSE`, then the sample locations
///     must: stay constant in any given subpass.
///
/// If the `VkPhysicalDeviceSampleLocationsPropertiesEXT` structure is included in
/// the `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled with the
/// implementation-dependent limits.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl Default for VkPhysicalDeviceSampleLocationsPropertiesEXT {
  fn default() -> VkPhysicalDeviceSampleLocationsPropertiesEXT {
    VkPhysicalDeviceSampleLocationsPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl RawStruct for VkPhysicalDeviceSampleLocationsPropertiesEXT {
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

/// Structure returning information about sample count specific additional
/// multisampling capabilities
///
/// The `VkMultisamplePropertiesEXT` structure is defined as
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `maxSampleLocationGridSize` is the maximum size of the pixel grid in which
///     sample locations can: vary.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_max_sample_location_grid_size(mut self, value: VkExtent2D) -> Self {
    self.maxSampleLocationGridSize = value;
    self
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl Default for VkMultisamplePropertiesEXT {
  fn default() -> VkMultisamplePropertiesEXT {
    VkMultisamplePropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_sample_locations")]
impl RawStruct for VkMultisamplePropertiesEXT {
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
///
/// If the `pNext` list of `VkImageCreateInfo` includes a
/// `VkImageFormatListCreateInfoKHR` structure, then that structure contains a list
/// of all formats that can: be used when creating views of this image.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `viewFormatCount` is the number of entries in the `pViewFormats` array.
///
///   - `pViewFormats` is an array which lists of all formats which can: be used
///     when creating views of this image.
///
/// If `viewFormatCount` is zero, `pViewFormats` is ignored and the image is created
/// as if the `VkImageFormatListCreateInfoKHR` structure were not included in the
/// `pNext` list of `VkImageCreateInfo`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_view_formats(mut self, value: &'a [VkFormat]) -> Self {
    unsafe {
      self.pViewFormats = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_KHR_image_format_list")]
impl<'a> Default for VkImageFormatListCreateInfoKHR<'a> {
  fn default() -> VkImageFormatListCreateInfoKHR<'a> {
    VkImageFormatListCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_image_format_list")]
impl<'a> RawStruct for VkImageFormatListCreateInfoKHR<'a> {
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

/// Structure describing advanced blending features that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT` structure is defined as.
///
/// The members of the `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT` structure
/// describe the following features:
///
///   - `advancedBlendCoherentOperations` indicates whether blending using [advanced
///     blend operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blend-advanced) is guaranteed to execute
///     atomically and in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#drawing-primitive-order). If this is
///     `VK_TRUE`, `VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is treated
///     the same as `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`, and advanced blending
///     needs no additional synchronization over basic blending. If this is
///     `VK_FALSE`, then memory dependencies are required to guarantee order between
///     two advanced blending operations that occur on the same sample.
///
/// If the `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT` structure is included
/// in the `pNext` chain of `VkPhysicalDeviceFeatures2KHR`, it is filled with values
/// indicating whether each feature is supported.
/// `VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT` can: also be used in `pNext`
/// chain of `VkDeviceCreateInfo` to enable the features.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_advanced_blend_coherent_operations(mut self, value: VkBool32) -> Self {
    self.advancedBlendCoherentOperations = value;
    self
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl Default for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
  fn default() -> VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl RawStruct for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
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

/// Structure describing advanced blending limits that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT` structure is defined
/// as.
///
/// The members of the `VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT`
/// structure describe the following implementation-dependent limits:
///
///   - `advancedBlendMaxColorAttachments` is one greater than the highest color
///     attachment index that can: be used in a subpass, for a pipeline that uses an
///     [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blend-advanced).
///
///   - `advancedBlendIndependentBlend` indicates whether advanced blend operations
///     can: vary per-attachment.
///
///   - `advancedBlendNonPremultipliedSrcColor` indicates whether the source color
///     can: be treated as non-premultiplied. If this is `VK_FALSE`, then
///     `VkPipelineColorBlendAdvancedStateCreateInfoEXT::srcPremultiplied` must: be
///     `VK_TRUE`.
///
///   - `advancedBlendNonPremultipliedDstColor` indicates whether the destination
///     color can: be treated as non-premultiplied. If this is `VK_FALSE`, then
///     `VkPipelineColorBlendAdvancedStateCreateInfoEXT::dstPremultiplied` must: be
///     `VK_TRUE`.
///
///   - `advancedBlendCorrelatedOverlap` indicates whether the overlap mode can: be
///     treated as correlated. If this is `VK_FALSE`, then
///     `VkPipelineColorBlendAdvancedStateCreateInfoEXT::blendOverlap` must: be
///     `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
///
///   - `advancedBlendAllOperations` indicates whether all advanced blend operation
///     enums are supported. See the valid usage of
///     `VkPipelineColorBlendAttachmentState`.
///
/// If the `VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT` structure is
/// included in the `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled
/// with the implementation-dependent limits.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl Default for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
  fn default() -> VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl RawStruct for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
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

/// Structure specifying parameters that affect advanced blend operations
///
/// If the `pNext` chain of `VkPipelineColorBlendStateCreateInfo` includes a
/// `VkPipelineColorBlendAdvancedStateCreateInfoEXT` structure, then that structure
/// includes parameters that affect advanced blend operations.
///
/// The `VkPipelineColorBlendAdvancedStateCreateInfoEXT` structure is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `srcPremultiplied` specifies whether the source color of the blend operation
///     is treated as premultiplied.
///
///   - `dstPremultiplied` specifies whether the destination color of the blend
///     operation is treated as premultiplied.
///
///   - `blendOverlap` is a `VkBlendOverlapEXT` value specifying how the source and
///     destination sample’s coverage is correlated.
///
/// If this structure is not present, `srcPremultiplied` and `dstPremultiplied` are
/// both considered to be `VK_TRUE`, and `blendOverlap` is considered to be
/// `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl Default for VkPipelineColorBlendAdvancedStateCreateInfoEXT {
  fn default() -> VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    VkPipelineColorBlendAdvancedStateCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
impl RawStruct for VkPipelineColorBlendAdvancedStateCreateInfoEXT {
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

/// Structure specifying whether fragment coverage replaces a color
///
/// If the `pNext` chain of `VkPipelineMultisampleStateCreateInfo` includes a
/// `VkPipelineCoverageToColorStateCreateInfoNV` structure, then that structure
/// controls whether the fragment coverage is substituted for a fragment color
/// output and, if so, which output is replaced.
///
/// The `VkPipelineCoverageToColorStateCreateInfoNV` structure is defined as.
///
///   - `sType` is the type of this structure
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure
///
///   - `flags` is reserved for future use.
///
///   - `coverageToColorEnable` controls whether the fragment coverage value
///     replaces a fragment color output.
///
///   - `coverageToColorLocation` controls which fragment shader color output value
///     is replaced.
///
/// If `coverageToColorEnable` is `VK_TRUE`, the fragment coverage information is
/// treated as a bitmask with one bit for each sample (as in the [Sample
/// Mask](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-samplemask) section), and this bitmask replaces the first
/// component of the color value corresponding to the fragment shader output
/// location with `Location` equal to `coverageToColorLocation` and `Index` equal to
/// zero. If the color attachment format has fewer bits than the sample coverage,
/// the low bits of the sample coverage bitmask are taken without any clamping. If
/// the color attachment format has more bits than the sample coverage, the high
/// bits of the sample coverage bitmask are filled with zeros.
///
/// If [Sample Shading](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#primsrast-sampleshading) is in use, the coverage bitmask
/// only has bits set for samples that correspond to the fragment shader invocation
/// that shades those samples.
///
/// This pipeline stage occurs after sample counting and before blending, and is
/// always performed after fragment shading regardless of the setting of
/// `EarlyFragmentTests`.
///
/// If `coverageToColorEnable` is `VK_FALSE`, these operations are skipped. If this
/// structure is not present, it is as if `coverageToColorEnable` is `VK_FALSE`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
impl Default for VkPipelineCoverageToColorStateCreateInfoNV {
  fn default() -> VkPipelineCoverageToColorStateCreateInfoNV {
    VkPipelineCoverageToColorStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
impl RawStruct for VkPipelineCoverageToColorStateCreateInfoNV {
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

/// Structure specifying parameters controlling coverage modulation
///
/// As part of coverage reduction, fragment color values can: also be modulated
/// (multiplied) by a value that is a function of fraction of covered rasterization
/// samples associated with that color sample.
///
/// Pipeline state controlling coverage reduction is specified through the members
/// of the `VkPipelineCoverageModulationStateCreateInfoNV` structure.
///
/// The `VkPipelineCoverageModulationStateCreateInfoNV` structure is defined as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `coverageModulationMode` controls which color components are modulated and
///     is of type `VkCoverageModulationModeNV`.
///
///   - `coverageModulationTableEnable` controls whether the modulation factor is
///     looked up from a table in `pCoverageModulationTable`.
///
///   - `coverageModulationTableCount` is the number of elements in
///     `pCoverageModulationTable`.
///
///   - `pCoverageModulationTable` is a table of modulation factors containing a
///     value for each number of covered samples.
///
/// If `coverageModulationTableEnable` is `VK_FALSE`, then for each color sample the
/// associated bits of the fragment’s coverage are counted and divided by the number
/// of associated bits to produce a modulation factor R in the range (0,1\] (a value
/// of zero would have been killed due to a color coverage of 0). Specifically:
///
///   - N = value of `rasterizationSamples`
///
///   - M = value of `VkAttachmentDescription::samples` for any color attachments
///
///   - R = popcount(associated coverage bits) / (N / M)
///
/// If `coverageModulationTableEnable` is `VK_TRUE`, the value R is computed using a
/// programmable lookup table. The lookup table has N / M elements, and the element
/// of the table is selected by:
///
///   - R = `pCoverageModulationTable`\[popcount(associated coverage bits)-1\]
///
/// Note that the table does not have an entry for popcount(associated coverage
/// bits) = 0, because such samples would have been killed.
///
/// The values of `pCoverageModulationTable` may: be rounded to an
/// implementation-dependent precision, which is at least as fine as 1 / N, and
/// clamped to \[0,1\].
///
/// For each color attachment with a floating point or normalized color format, each
/// fragment output color value is replicated to M values which can: each be
/// modulated (multiplied) by that color sample’s associated value of R. Which
/// components are modulated is controlled by `coverageModulationMode`.
///
/// If this structure is not present, it is as if coverageModulationMode is
/// `VK_COVERAGE_MODULATION_MODE_NONE_NV`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
    unsafe {
      self.pCoverageModulationTable = value.as_raw();
    }
    self
  }
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
impl<'a> Default for VkPipelineCoverageModulationStateCreateInfoNV<'a> {
  fn default() -> VkPipelineCoverageModulationStateCreateInfoNV<'a> {
    VkPipelineCoverageModulationStateCreateInfoNV::new()
  }
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
impl<'a> RawStruct for VkPipelineCoverageModulationStateCreateInfoNV<'a> {
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

/// Structure specifying how to bind a buffer to memory
///
/// `VkBindBufferMemoryInfoKHR` contains members corresponding to the parameters of
/// `vkBindBufferMemory`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `buffer` is the buffer to be attached to memory.
///
///   - `memory` is a `VkDeviceMemory` object describing the device memory to
///     attach.
///
///   - `memoryOffset` is the start offset of the region of `memory` which is to be
///     bound to the buffer. The number of bytes returned in the
///     `VkMemoryRequirements::size` member in `memory`, starting from
///     `memoryOffset` bytes, will be bound to the specified buffer.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl Default for VkBindBufferMemoryInfoKHR {
  fn default() -> VkBindBufferMemoryInfoKHR {
    VkBindBufferMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl RawStruct for VkBindBufferMemoryInfoKHR {
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
///
/// `VkBindImageMemoryInfoKHR` contains members corresponding to the parameters of
/// `vkBindImageMemory`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `image` is the image to be attached to memory.
///
///   - `memory` is a `VkDeviceMemory` object describing the device memory to
///     attach.
///
///   - `memoryOffset` is the start offset of the region of `memory` which is to be
///     bound to the image. The number of bytes returned in the
///     `VkMemoryRequirements::size` member in `memory`, starting from
///     `memoryOffset` bytes, will be bound to the specified image.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl Default for VkBindImageMemoryInfoKHR {
  fn default() -> VkBindImageMemoryInfoKHR {
    VkBindImageMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_bind_memory2")]
impl RawStruct for VkBindImageMemoryInfoKHR {
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
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `format` is the format of the image from which color information will be
///     retrieved.
///
///   - `ycbcrModel` describes the color matrix for conversion between color models.
///
///   - `ycbcrRange` describes whether the encoded values have headroom and foot
///     room, or whether the encoding uses the full numerical range.
///
///   - `components` applies a *swizzle* based on `VkComponentSwizzle` enums prior
///     to range expansion and color model conversion.
///
///   - `xChromaOffset` describes the [sample
///     location](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-chroma-reconstruction) associated with downsampled
///     chroma channels in the x dimension. `xChromaOffset` has no effect for
///     formats in which chroma channels are the same resolution as the luma
///     channel.
///
///   - `yChromaOffset` describes the [sample
///     location](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-chroma-reconstruction) associated with downsampled
///     chroma channels in the y dimension. `yChromaOffset` has no effect for
///     formats in which the chroma channels are not downsampled vertically.
///
///   - `chromaFilter` is the filter for chroma reconstruction.
///
///   - `forceExplicitReconstruction` can: be used to ensure that reconstruction is
///     done explicitly, if supported.
///
/// > **Note**
/// >
/// > Setting `forceExplicitReconstruction` to `VK_TRUE` may: have a performance
/// > penalty on implementations where explicit reconstruction is not the default
/// > mode of operation.
///
/// If `chromaFilter` is `VK_FILTER_NEAREST`, chroma samples are reconstructed to
/// luma channel resolution using nearest-neighbour sampling. Otherwise, chroma
/// samples are reconstructed using interpolation. More details can be found in [the
/// description of sampler Y’C<sub>B</sub>C<sub>R</sub>
/// conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-sampler-YCbCr-conversion) in the [Image
/// Operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures) chapter.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
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
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkSamplerYcbcrConversionCreateInfoKHR {
  fn default() -> VkSamplerYcbcrConversionCreateInfoKHR {
    VkSamplerYcbcrConversionCreateInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl RawStruct for VkSamplerYcbcrConversionCreateInfoKHR {
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

/// A sampler Y’C<sub>B</sub>C<sub>R</sub> conversion is an opaque representation of
/// a device-specific sampler Y’C<sub>B</sub>C<sub>R</sub> conversion description,
/// represented as a `VkSamplerYcbcrConversionKHR` handle.
///
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type VkSamplerYcbcrConversionKHR = VkNonDispatchableHandle<VkSamplerYcbcrConversionKHR__>;

/// Structure specifying Y'CbCr conversion to a sampler or image view
///
/// To create a sampler with Y’C<sub>B</sub>C<sub>R</sub> conversion enabled, add a
/// `VkSamplerYcbcrConversionInfoKHR` to the `pNext` chain of the
/// `VkSamplerCreateInfo` structure. To create a sampler
/// Y’C<sub>B</sub>C<sub>R</sub> conversion, the [`samplerYcbcrConversion`
/// feature](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features-sampler-YCbCr-conversion) must: be enabled.
/// Conversion must: be fixed at pipeline creation time, through use of a combined
/// image sampler with an immutable sampler in `VkDescriptorSetLayoutBinding`.
///
/// A `VkSamplerYcbcrConversionInfoKHR` must: be provided for samplers to be used
/// with image views that access `VK_IMAGE_ASPECT_COLOR_BIT` if the format appears
/// in [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion).
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `conversion` is a `VkSamplerYcbcrConversionKHR` handle created with
///     `vkCreateSamplerYcbcrConversionKHR`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_conversion(mut self, value: VkSamplerYcbcrConversionKHR) -> Self {
    self.conversion = value;
    self
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkSamplerYcbcrConversionInfoKHR {
  fn default() -> VkSamplerYcbcrConversionInfoKHR {
    VkSamplerYcbcrConversionInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl RawStruct for VkSamplerYcbcrConversionInfoKHR {
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

/// Structure specifying how to bind an image plane to memory
///
/// In order to bind *planes* of a *disjoint image*, include a
/// `VkBindImagePlaneMemoryInfoKHR` structure in the `pNext` chain of
/// `VkBindImageMemoryInfoKHR`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `planeAspect` is the aspect of the disjoint image plane to bind.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_plane_aspect(mut self, value: VkImageAspectFlagBits) -> Self {
    self.planeAspect = value;
    self
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkBindImagePlaneMemoryInfoKHR {
  fn default() -> VkBindImagePlaneMemoryInfoKHR {
    VkBindImagePlaneMemoryInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl RawStruct for VkBindImagePlaneMemoryInfoKHR {
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

/// Structure specifying image plane for memory requirements
///
/// To determine the memory requirements for a plane of a disjoint image, add a
/// `VkImagePlaneMemoryRequirementsInfoKHR` to the `pNext` chain of the
/// `VkImageMemoryRequirementsInfo2KHR` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `planeAspect` is the aspect corresponding to the image plane to query.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_plane_aspect(mut self, value: VkImageAspectFlagBits) -> Self {
    self.planeAspect = value;
    self
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkImagePlaneMemoryRequirementsInfoKHR {
  fn default() -> VkImagePlaneMemoryRequirementsInfoKHR {
    VkImagePlaneMemoryRequirementsInfoKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl RawStruct for VkImagePlaneMemoryRequirementsInfoKHR {
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

/// Structure describing Y'CbCr conversion features that can be supported by an
/// implementation
///
/// The `VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR` structure is defined as.
///
/// The members of the `VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR` structure
/// describe the following feature:
///
///   - `samplerYcbcrConversion` indicates whether the implementation supports
///     [sampler Y’C<sub>B</sub>C<sub>R</sub>
///     conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion). If `samplerYcbcrConversion` is
///     `VK_FALSE`, sampler Y’C<sub>B</sub>C<sub>R</sub> conversion is not
///     supported, and samplers using sampler Y’C<sub>B</sub>C<sub>R</sub>
///     conversion must: not be used.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_sampler_ycbcr_conversion(mut self, value: VkBool32) -> Self {
    self.samplerYcbcrConversion = value;
    self
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
  fn default() -> VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
    VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl RawStruct for VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
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

/// Structure specifying combined image sampler descriptor count for multi-planar
/// images
///
/// To determine the number of combined image samplers required to support a
/// multi-planar format, add `VkSamplerYcbcrConversionImageFormatPropertiesKHR` to
/// the `pNext` chain of the `VkImageFormatProperties2KHR` structure in a call to
/// `vkGetPhysicalDeviceImageFormatProperties2KHR`.
///
/// The `VkSamplerYcbcrConversionImageFormatPropertiesKHR` structure is defined as.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `combinedImageSamplerDescriptorCount` is the number of combined image
///     sampler descriptors that the implementation uses to access the format.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_combined_image_sampler_descriptor_count(mut self, value: u32) -> Self {
    self.combinedImageSamplerDescriptorCount = value;
    self
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl Default for VkSamplerYcbcrConversionImageFormatPropertiesKHR {
  fn default() -> VkSamplerYcbcrConversionImageFormatPropertiesKHR {
    VkSamplerYcbcrConversionImageFormatPropertiesKHR::new()
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl RawStruct for VkSamplerYcbcrConversionImageFormatPropertiesKHR {
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

/// Opaque handle to a validation cache object
///
/// Validation cache objects allow the result of internal validation to be reused,
/// both within a single application run and between multiple runs. Reuse within a
/// single run is achieved by passing the same validation cache object when creating
/// supported Vulkan objects. Reuse across runs of an application is achieved by
/// retrieving validation cache contents in one run of an application, saving the
/// contents, and using them to preinitialize a validation cache on a subsequent
/// run. The contents of the validation cache objects are managed by the validation
/// layers. Applications can: manage the host memory consumed by a validation cache
/// object and control the amount of data retrieved from a validation cache object.
///
/// Validation cache objects are represented by `VkValidationCacheEXT` handles.
///
#[cfg(feature = "VK_EXT_validation_cache")]
pub type VkValidationCacheEXT = VkNonDispatchableHandle<VkValidationCacheEXT__>;

/// Structure specifying parameters of a newly created validation cache
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `flags` is reserved for future use.
///
///   - `initialDataSize` is the number of bytes in `pInitialData`. If
///     `initialDataSize` is zero, the validation cache will initially be empty.
///
///   - `pInitialData` is a pointer to previously retrieved validation cache data.
///     If the validation cache data is incompatible (as defined below) with the
///     device, the validation cache will be initially empty. If `initialDataSize`
///     is zero, `pInitialData` is ignored.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_flags(mut self, value: VkValidationCacheCreateFlagsEXT) -> Self {
    self.flags = value;
    self
  }
  #[inline]
  pub fn set_initial_data(mut self, value: &'a [u8]) -> Self {
    unsafe {
      self.pInitialData = value.as_raw() as *const c_void;
    }
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
impl<'a> RawStruct for VkValidationCacheCreateInfoEXT<'a> {
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
///
/// To use a `VkValidationCacheEXT` to cache shader validation results, add a
/// `VkShaderModuleValidationCacheCreateInfoEXT` to the `pNext` chain of the
/// `VkShaderModuleCreateInfo` structure, specifying the cache object to use.
///
/// The VkShaderModuleValidationCacheCreateInfoEXT struct is defined as:
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `validationCache` is the validation cache object from which the results of
///     prior validation attempts will be written, and to which new validation
///     results for this VkShaderModule will be written (if not already present).
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_validation_cache(mut self, value: VkValidationCacheEXT) -> Self {
    self.validationCache = value;
    self
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl Default for VkShaderModuleValidationCacheCreateInfoEXT {
  fn default() -> VkShaderModuleValidationCacheCreateInfoEXT {
    VkShaderModuleValidationCacheCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl RawStruct for VkShaderModuleValidationCacheCreateInfoEXT {
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

/// Specify a system wide priority
///
/// A queue can: be created with a system-wide priority by including a
/// `VkDeviceQueueGlobalPriorityCreateInfoEXT` structure in the `pNext` chain of
/// `VkDeviceQueueCreateInfo`.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `globalPriority` is the system-wide priority associated to this queue as
///     specified by `VkQueueGlobalPriorityEXT`
///
/// A queue created without specifying `VkDeviceQueueGlobalPriorityCreateInfoEXT`
/// will default to `VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT`.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_global_priority(mut self, value: VkQueueGlobalPriorityEXT) -> Self {
    self.globalPriority = value;
    self
  }
}
#[cfg(feature = "VK_EXT_global_priority")]
impl Default for VkDeviceQueueGlobalPriorityCreateInfoEXT {
  fn default() -> VkDeviceQueueGlobalPriorityCreateInfoEXT {
    VkDeviceQueueGlobalPriorityCreateInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_global_priority")]
impl RawStruct for VkDeviceQueueGlobalPriorityCreateInfoEXT {
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

/// import memory from a host pointer
///
/// To import memory from a host pointer, add a `VkImportMemoryHostPointerInfoEXT`
/// structure to the `pNext` chain of the `VkMemoryAllocateInfo` structure.
///
///   - `sType` is the type of this structure.
///
///   - `pNext` is `NULL` or a pointer to an extension-specific structure.
///
///   - `handleType` specifies the handle type.
///
///   - `pHostPointer` is the host pointer to import from.
///
/// Importing memory from a host pointer shares ownership of the memory between the
/// host and the Vulkan implementation. The application can: continue to access the
/// memory through the host pointer but it is the application’s responsibility to
/// synchronize device and non-device access to the underlying memory as defined in
/// [Host Access to Device Memory Objects](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-device-hostaccess).
///
/// Applications can: import the same underlying memory into multiple instances of
/// Vulkan and multiple times into a given Vulkan instance. However, implementations
/// may: fail to import the same underlying memory multiple times into a given
/// physical device due to platform constraints.
///
/// Importing memory from a particular host pointer may: not be possible due to
/// additional platform-specific restrictions beyond the scope of this specification
/// in which case the implementation must: fail the memory import operation with the
/// error code `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`.
///
/// The application must: ensure that the imported memory range remains valid and
/// accessible for the lifetime of the imported memory object.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_handle_type(mut self, value: VkExternalMemoryHandleTypeFlagBitsKHR) -> Self {
    self.handleType = value;
    self
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl Default for VkImportMemoryHostPointerInfoEXT {
  fn default() -> VkImportMemoryHostPointerInfoEXT {
    VkImportMemoryHostPointerInfoEXT::new()
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl RawStruct for VkImportMemoryHostPointerInfoEXT {
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_memory_type_bits(mut self, value: u32) -> Self {
    self.memoryTypeBits = value;
    self
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl Default for VkMemoryHostPointerPropertiesEXT {
  fn default() -> VkMemoryHostPointerPropertiesEXT {
    VkMemoryHostPointerPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl RawStruct for VkMemoryHostPointerPropertiesEXT {
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
///
/// The `VkPhysicalDeviceExternalMemoryHostPropertiesEXT` structure is defined as.
///
/// The members of the `VkPhysicalDeviceExternalMemoryHostPropertiesEXT` structure
/// describe the following implementation-dependent limits:
///
///   - `minImportedHostPointerAlignment` is the minimum required: alignment, in
///     bytes, for the base address and size of host pointers that can: be imported
///     to a Vulkan memory object.
///
/// If the `VkPhysicalDeviceExternalMemoryHostPropertiesEXT` structure is included
/// in the `pNext` chain of `VkPhysicalDeviceProperties2KHR`, it is filled with the
/// implementation-dependent limits.
///
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
  pub fn set_s_type(mut self, value: VkStructureType) -> Self {
    self.sType = value;
    self
  }
  #[inline]
  pub fn set_min_imported_host_pointer_alignment(mut self, value: VkDeviceSize) -> Self {
    self.minImportedHostPointerAlignment = value;
    self
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl Default for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
  fn default() -> VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    VkPhysicalDeviceExternalMemoryHostPropertiesEXT::new()
  }
}
#[cfg(feature = "VK_EXT_external_memory_host")]
impl RawStruct for VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
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
