/* GENERATED FILE */

// feature: VK_VERSION_1_0

/// Return API version number for Vulkan 1.0
///
/// `VK_API_VERSION_1_0` returns the API version number for Vulkan 1.0. The patch
/// version number in this macro will always be zero. The supported patch version
/// for a physical device can: be queried with `vkGetPhysicalDeviceProperties`.
///
pub const VK_API_VERSION_1_0: u32 = vk_make_version!(1, 0, 0);

/// Vulkan header file version number
///
/// `VK_HEADER_VERSION` is the version number of the vulkan.h header. This value is
/// currently kept synchronized with the release number of the Specification.
/// However, it is not guaranteed to remain synchronized, since most Specification
/// updates have no effect on vulkan.h.
///
pub const VK_HEADER_VERSION: u32 = 69;

// API Constants
//////////////////
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const VK_UUID_SIZE: u32 = 16;

// feature: VK_KHR_external_memory_capabilities
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub const VK_LUID_SIZE_KHR: u32 = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const VK_MAX_DESCRIPTION_SIZE: u32 = 256;
pub const VK_MAX_MEMORY_TYPES: u32 = 32;
pub const VK_MAX_MEMORY_HEAPS: u32 = 16;
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0f32;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0u32;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0u32;
pub const VK_WHOLE_SIZE: u64 = !0u64;
pub const VK_ATTACHMENT_UNUSED: u32 = !0u32;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0u32;

// feature: VK_KHR_external_memory
#[cfg(feature = "VK_KHR_external_memory")]
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = !0u32 - 1;

// feature: VK_EXT_queue_family_foreign
#[cfg(feature = "VK_EXT_queue_family_foreign")]
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = !0u32 - 2;
pub const VK_SUBPASS_EXTERNAL: u32 = !0u32;

// feature: VK_KHX_device_group_creation
#[cfg(feature = "VK_KHX_device_group_creation")]
pub const VK_MAX_DEVICE_GROUP_SIZE_KHX: u32 = 32;

// feature: VK_VERSION_1_0

define_enum! {

  /// Layout of image and image subresources
  ///
  /// The set of image layouts consists of.
  ///
  /// The type(s) of device access supported by each layout are:
  ///
  ///   - `VK_IMAGE_LAYOUT_UNDEFINED` does not support device access. This layout
  ///     must: only be used as the `initialLayout` member of `VkImageCreateInfo` or
  ///     `VkAttachmentDescription`, or as the `oldLayout` in an image transition.
  ///     When transitioning out of this layout, the contents of the memory are not
  ///     guaranteed to be preserved.
  ///
  ///   - `VK_IMAGE_LAYOUT_PREINITIALIZED` does not support device access. This layout
  ///     must: only be used as the `initialLayout` member of `VkImageCreateInfo` or
  ///     `VkAttachmentDescription`, or as the `oldLayout` in an image transition.
  ///     When transitioning out of this layout, the contents of the memory are
  ///     preserved. This layout is intended to be used as the initial layout for an
  ///     image whose contents are written by the host, and hence the data can: be
  ///     written to memory immediately, without first executing a layout transition.
  ///     Currently, `VK_IMAGE_LAYOUT_PREINITIALIZED` is only useful with
  ///     `VK_IMAGE_TILING_LINEAR` images because there is not a standard layout
  ///     defined for `VK_IMAGE_TILING_OPTIMAL` images.
  ///
  ///   - `VK_IMAGE_LAYOUT_GENERAL` supports all types of device access.
  ///
  ///   - `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL` must: only be used as a color or
  ///     resolve attachment in a `VkFramebuffer`. This layout is valid only for image
  ///     subresources of images created with the
  ///     `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` usage bit enabled.
  ///
  ///   - `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL` must: only be used as a
  ///     depth/stencil attachment in a `VkFramebuffer`. This layout is valid only for
  ///     image subresources of images created with the
  ///     `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` usage bit enabled.
  ///
  ///   - `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL` must: only be used as a
  ///     read-only depth/stencil attachment in a `VkFramebuffer` and/or as a
  ///     read-only image in a shader (which can: be read as a sampled image, combined
  ///     image/sampler and/or input attachment). This layout is valid only for image
  ///     subresources of images created with the
  ///     `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` usage bit enabled. Only image
  ///     subresources of images created with `VK_IMAGE_USAGE_SAMPLED_BIT` can: be
  ///     used as a sampled image or combined image/sampler in a shader. Similarly,
  ///     only image subresources of images created with
  ///     `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT` can: be used as input attachments.
  ///
  ///   - `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR`: must: only
  ///     be used as a depth/stencil attachment in a `VkFramebuffer`, where the depth
  ///     aspect is read-only, and/or as a read-only image in a shader (which can: be
  ///     read as a sampled image, combined image/sampler and/or input attachment)
  ///     where only the depth aspect is accessed. This layout is valid only for image
  ///     subresources of images created with the
  ///     `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` usage bit enabled. Only image
  ///     subresources of images created with `VK_IMAGE_USAGE_SAMPLED_BIT` can: be
  ///     used as a sampled image or combined image/sampler in a shader. Similarly,
  ///     only image subresources of images created with
  ///     `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT` can: be used as input attachments.
  ///
  ///   - `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR`: must: only
  ///     be used as a depth/stencil attachment in a `VkFramebuffer`, where the
  ///     stencil aspect is read-only, and/or as a read-only image in a shader (which
  ///     can: be read as a sampled image, combined image/sampler and/or input
  ///     attachment) where only the stencil aspect is accessed. This layout is valid
  ///     only for image subresources of images created with the
  ///     `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` usage bit enabled. Only image
  ///     subresources of images created with `VK_IMAGE_USAGE_SAMPLED_BIT` can: be
  ///     used as a sampled image or combined image/sampler in a shader. Similarly,
  ///     only image subresources of images created with
  ///     `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT` can: be used as input attachments.
  ///
  ///   - `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL` must: only be used as a read-only
  ///     image in a shader (which can: be read as a sampled image, combined
  ///     image/sampler and/or input attachment). This layout is valid only for image
  ///     subresources of images created with the `VK_IMAGE_USAGE_SAMPLED_BIT` or
  ///     `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT` usage bit enabled.
  ///
  ///   - `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` must: only be used as a source image
  ///     of a transfer command (see the definition of
  ///     [`VK_PIPELINE_STAGE_TRANSFER_BIT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-transfer)).
  ///     This layout is valid only for image subresources of images created with the
  ///     `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage bit enabled.
  ///
  ///   - `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` must: only be used as a destination
  ///     image of a transfer command. This layout is valid only for image
  ///     subresources of images created with the `VK_IMAGE_USAGE_TRANSFER_DST_BIT`
  ///     usage bit enabled.
  ///
  ///   - `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR` must: only be used for presenting a
  ///     presentable image for display. A swapchain’s image must: be transitioned to
  ///     this layout before calling `vkQueuePresentKHR`, and must: be transitioned
  ///     away from this layout after calling `vkAcquireNextImageKHR`.
  ///
  ///   - `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` is valid only for shared presentable
  ///     images, and must: be used for any usage the image supports.
  ///
  /// For each mechanism of accessing an image in the API, there is a parameter or
  /// structure member that controls the image layout used to access the image. For
  /// transfer commands, this is a parameter to the command (see [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#clears) and
  /// [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#copies)). For use as a framebuffer attachment, this is a member in the
  /// substructures of the `VkRenderPassCreateInfo` (see [Render Pass](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass)).
  /// For use in a descriptor set, this is a member in the `VkDescriptorImageInfo`
  /// structure (see [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-updates)). At the time that any command
  /// buffer command accessing an image executes on any queue, the layouts of the
  /// image subresources that are accessed must: all match the layout specified via
  /// the API controlling those accesses.
  ///
  /// The image layout of each image subresource must: be well-defined at each point
  /// in the image subresource’s lifetime. This means that when performing a layout
  /// transition on the image subresource, the old layout value must: either equal the
  /// current layout of the image subresource (at the time the transition executes),
  /// or else be `VK_IMAGE_LAYOUT_UNDEFINED` (implying that the contents of the image
  /// subresource need not be preserved). The new layout used in a transition must:
  /// not be `VK_IMAGE_LAYOUT_UNDEFINED` or `VK_IMAGE_LAYOUT_PREINITIALIZED`.
  ///
  /// The image layout of each image subresource of a depth/stencil image created with
  /// `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` is dependent on the
  /// last sample locations used to render to the image subresource as a depth/stencil
  /// attachment, thus applications must: provide the same sample locations that were
  /// last used to render to the given image subresource whenever a layout transition
  /// of the image subresource happens, otherwise the contents of the depth aspect of
  /// the image subresource become undefined.
  ///
  /// In addition, depth reads from a depth/stencil attachment referring to an image
  /// subresource range of a depth/stencil image created with
  /// `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` using different
  /// sample locations than what have been last used to perform depth writes to the
  /// image subresources of the same image subresource range produce undefined
  /// results.
  ///
  /// Similarly, depth writes to a depth/stencil attachment referring to an image
  /// subresource range of a depth/stencil image created with
  /// `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` using different
  /// sample locations than what have been last used to perform depth writes to the
  /// image subresources of the same image subresource range make the contents of the
  /// depth aspect of those image subresources undefined.
  ///
  pub enum VkImageLayout {
    E_UNDEFINED = 0,
    E_GENERAL = 1,
    E_COLOR_ATTACHMENT_OPTIMAL = 2,
    E_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    E_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    E_SHADER_READ_ONLY_OPTIMAL = 5,
    E_TRANSFER_SRC_OPTIMAL = 6,
    E_TRANSFER_DST_OPTIMAL = 7,
    E_PREINITIALIZED = 8,

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    E_PRESENT_SRC_KHR = 1000001002,

    // feature: VK_KHR_shared_presentable_image
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    E_SHARED_PRESENT_KHR = 1000111000,

    // feature: VK_KHR_maintenance2
    #[cfg(feature = "VK_KHR_maintenance2")]
    E_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR = 1000117000,
    #[cfg(feature = "VK_KHR_maintenance2")]
    E_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR = 1000117001
  }
}

define_enum! {

  /// Specify how contents of an attachment are treated at the beginning of a subpass
  ///
  /// Possible values of `VkAttachmentDescription::loadOp` and `stencilLoadOp`,
  /// specifying how the contents of the attachment are treated.
  ///
  ///   - `VK_ATTACHMENT_LOAD_OP_LOAD` specifies that the previous contents of the
  ///     image within the render area will be preserved. For attachments with a
  ///     depth/stencil format, this uses the access type
  ///     `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT`. For attachments with a color
  ///     format, this uses the access type `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`.
  ///
  ///   - `VK_ATTACHMENT_LOAD_OP_CLEAR` specifies that the contents within the render
  ///     area will be cleared to a uniform value, which is specified when a render
  ///     pass instance is begun. For attachments with a depth/stencil format, this
  ///     uses the access type `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`. For
  ///     attachments with a color format, this uses the access type
  ///     `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.
  ///
  ///   - `VK_ATTACHMENT_LOAD_OP_DONT_CARE` specifies that the previous contents
  ///     within the area need not be preserved; the contents of the attachment will
  ///     be undefined inside the render area. For attachments with a depth/stencil
  ///     format, this uses the access type
  ///     `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`. For attachments with a color
  ///     format, this uses the access type `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.
  ///
  pub enum VkAttachmentLoadOp {
    E_LOAD = 0,
    E_CLEAR = 1,
    E_DONT_CARE = 2
  }
}

define_enum! {

  /// Specify how contents of an attachment are treated at the end of a subpass
  ///
  /// Possible values of `VkAttachmentDescription::storeOp` and `stencilStoreOp`,
  /// specifying how the contents of the attachment are treated.
  ///
  ///   - `VK_ATTACHMENT_STORE_OP_STORE` specifies the contents generated during the
  ///     render pass and within the render area are written to memory. For
  ///     attachments with a depth/stencil format, this uses the access type
  ///     `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`. For attachments with a color
  ///     format, this uses the access type `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.
  ///
  ///   - `VK_ATTACHMENT_STORE_OP_DONT_CARE` specifies the contents within the render
  ///     area are not needed after rendering, and may: be discarded; the contents of
  ///     the attachment will be undefined inside the render area. For attachments
  ///     with a depth/stencil format, this uses the access type
  ///     `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`. For attachments with a color
  ///     format, this uses the access type `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.
  ///
  pub enum VkAttachmentStoreOp {
    E_STORE = 0,
    E_DONT_CARE = 1
  }
}

define_enum! {

  /// Specifies the type of an image object
  ///
  /// Possible values of `VkImageCreateInfo::imageType`, specifying the basic
  /// dimensionality of an image.
  ///
  ///   - `VK_IMAGE_TYPE_1D` specifies a one-dimensional image.
  ///
  ///   - `VK_IMAGE_TYPE_2D` specifies a two-dimensional image.
  ///
  ///   - `VK_IMAGE_TYPE_3D` specifies a three-dimensional image.
  ///
  pub enum VkImageType {
    E_1D = 0,
    E_2D = 1,
    E_3D = 2
  }
}

define_enum! {

  /// Specifies the tiling arrangement of data in an image
  ///
  /// Possible values of `VkImageCreateInfo::tiling`, specifying the tiling
  /// arrangement of data elements in an image.
  ///
  ///   - `VK_IMAGE_TILING_OPTIMAL` specifies optimal tiling (texels are laid out in
  ///     an implementation-dependent arrangement, for more optimal memory access).
  ///
  ///   - `VK_IMAGE_TILING_LINEAR` specifies linear tiling (texels are laid out in
  ///     memory in row-major order, possibly with some padding on each row).
  ///
  pub enum VkImageTiling {
    E_OPTIMAL = 0,
    E_LINEAR = 1
  }
}

define_enum! {

  /// Image view types
  ///
  /// The exact image view type is partially implicit, based on the image’s type and
  /// sample count, as well as the view creation parameters as described in the [image
  /// view compatibility table](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-image-views-compatibility) for
  /// `vkCreateImageView`. This table also shows which SPIR-V `OpTypeImage` `Dim` and
  /// `Arrayed` parameters correspond to each image view type.
  ///
  pub enum VkImageViewType {
    E_1D = 0,
    E_2D = 1,
    E_3D = 2,
    E_CUBE = 3,
    E_1D_ARRAY = 4,
    E_2D_ARRAY = 5,
    E_CUBE_ARRAY = 6
  }
}

define_enum! {

  /// Enumerant specifying a command buffer level
  ///
  /// Possible values of `VkCommandBufferAllocateInfo::level`, specifying the command
  /// buffer level.
  ///
  ///   - `VK_COMMAND_BUFFER_LEVEL_PRIMARY` specifies a primary command buffer.
  ///
  ///   - `VK_COMMAND_BUFFER_LEVEL_SECONDARY` specifies a secondary command buffer.
  ///
  pub enum VkCommandBufferLevel {
    E_PRIMARY = 0,
    E_SECONDARY = 1
  }
}

define_enum! {

  /// Specify how a component is swizzled
  ///
  /// Possible values of the members of `VkComponentMapping`, specifying the component
  /// values placed in each component of the output vector.
  ///
  ///   - `VK_COMPONENT_SWIZZLE_IDENTITY` specifies that the component is set to the
  ///     identity swizzle.
  ///
  ///   - `VK_COMPONENT_SWIZZLE_ZERO` specifies that the component is set to zero.
  ///
  ///   - `VK_COMPONENT_SWIZZLE_ONE` specifies that the component is set to either 1
  ///     or 1.0, depending on whether the type of the image view format is integer or
  ///     floating-point respectively, as determined by the [Format
  ///     Definition](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-definition) section for each `VkFormat`.
  ///
  ///   - `VK_COMPONENT_SWIZZLE_R` specifies that the component is set to the value of
  ///     the R component of the image.
  ///
  ///   - `VK_COMPONENT_SWIZZLE_G` specifies that the component is set to the value of
  ///     the G component of the image.
  ///
  ///   - `VK_COMPONENT_SWIZZLE_B` specifies that the component is set to the value of
  ///     the B component of the image.
  ///
  ///   - `VK_COMPONENT_SWIZZLE_A` specifies that the component is set to the value of
  ///     the A component of the image.
  ///
  /// Setting the identity swizzle on a component is equivalent to setting the
  /// identity mapping on that component. That is:
  ///
  /// <table>
  /// <caption>Component Mappings Equivalent To <code>VK_COMPONENT_SWIZZLE_IDENTITY</code></caption>
  /// <colgroup>
  /// <col width="50%" />
  /// <col width="50%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">Component</th>
  /// <th align="left">Identity Mapping</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>components</code>.r</p></td>
  /// <td align="left"><p><code>VK_COMPONENT_SWIZZLE_R</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>components</code>.g</p></td>
  /// <td align="left"><p><code>VK_COMPONENT_SWIZZLE_G</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>components</code>.b</p></td>
  /// <td align="left"><p><code>VK_COMPONENT_SWIZZLE_B</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>components</code>.a</p></td>
  /// <td align="left"><p><code>VK_COMPONENT_SWIZZLE_A</code></p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  pub enum VkComponentSwizzle {
    E_IDENTITY = 0,
    E_ZERO = 1,
    E_ONE = 2,
    E_R = 3,
    E_G = 4,
    E_B = 5,
    E_A = 6
  }
}

define_enum! {

  /// Specifies the type of a descriptor in a descriptor set
  ///
  /// The type of descriptors in a descriptor set is specified by
  /// `VkWriteDescriptorSet::descriptorType`, which must: be one of the values.
  ///
  ///   - `VK_DESCRIPTOR_TYPE_SAMPLER` specifies a [sampler
  ///     descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sampler).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` specifies a [combined image
  ///     sampler descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-combinedimagesampler).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` specifies a [storage image
  ///     descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storageimage).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` specifies a [sampled image
  ///     descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sampledimage).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` specifies a [uniform texel buffer
  ///     descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` specifies a [storage texel buffer
  ///     descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` specifies a [uniform buffer
  ///     descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformbuffer).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` specifies a [storage buffer
  ///     descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebuffer).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` specifies a [dynamic uniform
  ///     buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformbufferdynamic).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` specifies a [dynamic storage
  ///     buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebufferdynamic).
  ///
  ///   - `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` specifies a [input attachment
  ///     descriptor](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-inputattachment).
  ///
  /// When a descriptor set is updated via elements of `VkWriteDescriptorSet`, members
  /// of `pImageInfo`, `pBufferInfo` and `pTexelBufferView` are only accessed by the
  /// implementation when they correspond to descriptor type being defined - otherwise
  /// they are ignored. The members accessed are as follows for each descriptor type:
  ///
  ///   - For `VK_DESCRIPTOR_TYPE_SAMPLER`, only the `sample` member of each element
  ///     of `VkWriteDescriptorSet::pImageInfo` is accessed.
  ///
  ///   - For `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`,
  ///     or `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, only the `imageView` and
  ///     `imageLayout` members of each element of `VkWriteDescriptorSet::pImageInfo`
  ///     are accessed.
  ///
  ///   - For `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, all members of each element
  ///     of `VkWriteDescriptorSet::pImageInfo` are accessed.
  ///
  ///   - For `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`,
  ///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`,
  ///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, or
  ///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, all members of each element of
  ///     `VkWriteDescriptorSet::pBufferInfo` are accessed.
  ///
  ///   - For `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` or
  ///     `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`, each element of
  ///     `VkWriteDescriptorSet::pTexelBufferView` is accessed.
  ///
  pub enum VkDescriptorType {
    E_SAMPLER = 0,
    E_COMBINED_IMAGE_SAMPLER = 1,
    E_SAMPLED_IMAGE = 2,
    E_STORAGE_IMAGE = 3,
    E_UNIFORM_TEXEL_BUFFER = 4,
    E_STORAGE_TEXEL_BUFFER = 5,
    E_UNIFORM_BUFFER = 6,
    E_STORAGE_BUFFER = 7,
    E_UNIFORM_BUFFER_DYNAMIC = 8,
    E_STORAGE_BUFFER_DYNAMIC = 9,
    E_INPUT_ATTACHMENT = 10
  }
}

define_enum! {

  /// Specify the type of queries managed by a query pool
  ///
  /// Possible values of `VkQueryPoolCreateInfo::queryType`, specifying the type of
  /// queries managed by the pool.
  ///
  ///   - `VK_QUERY_TYPE_OCCLUSION` specifies an [occlusion
  ///     query](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-occlusion).
  ///
  ///   - `VK_QUERY_TYPE_PIPELINE_STATISTICS` specifies a [pipeline statistics
  ///     query](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-pipestats).
  ///
  ///   - `VK_QUERY_TYPE_TIMESTAMP` specifies a [timestamp
  ///     query](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-timestamps).
  ///
  pub enum VkQueryType {
    E_OCCLUSION = 0,
    E_PIPELINE_STATISTICS = 1,
    E_TIMESTAMP = 2
  }
}

define_enum! {

  /// Specify border color used for texture lookups
  ///
  /// Possible values of `VkSamplerCreateInfo::borderColor`, specifying the border
  /// color used for texture lookups.
  ///
  ///   - `VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK` specifies a transparent,
  ///     floating-point format, black color.
  ///
  ///   - `VK_BORDER_COLOR_INT_TRANSPARENT_BLACK` specifies a transparent, integer
  ///     format, black color.
  ///
  ///   - `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK` specifies an opaque, floating-point
  ///     format, black color.
  ///
  ///   - `VK_BORDER_COLOR_INT_OPAQUE_BLACK` specifies an opaque, integer format,
  ///     black color.
  ///
  ///   - `VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE` specifies an opaque, floating-point
  ///     format, white color.
  ///
  ///   - `VK_BORDER_COLOR_INT_OPAQUE_WHITE` specifies an opaque, integer format,
  ///     white color.
  ///
  /// These colors are described in detail in [Texel
  /// Replacement](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-texel-replacement).
  ///
  pub enum VkBorderColor {
    E_FLOAT_TRANSPARENT_BLACK = 0,
    E_INT_TRANSPARENT_BLACK = 1,
    E_FLOAT_OPAQUE_BLACK = 2,
    E_INT_OPAQUE_BLACK = 3,
    E_FLOAT_OPAQUE_WHITE = 4,
    E_INT_OPAQUE_WHITE = 5
  }
}

define_enum! {

  /// Specify the bind point of a pipeline object to a command buffer
  ///
  /// Possible values of `vkCmdBindPipeline::pipelineBindPoint`, specifying the bind
  /// point of a pipeline object.
  ///
  ///   - `VK_PIPELINE_BIND_POINT_COMPUTE` specifies binding as a compute pipeline.
  ///
  ///   - `VK_PIPELINE_BIND_POINT_GRAPHICS` specifies binding as a graphics pipeline.
  ///
  pub enum VkPipelineBindPoint {
    E_GRAPHICS = 0,
    E_COMPUTE = 1
  }
}

define_enum! {

  /// Encode pipeline cache version
  ///
  /// Possible values of the second group of four bytes in the header returned by
  /// `vkGetPipelineCacheData`, encoding the pipeline cache version.
  ///
  ///   - `VK_PIPELINE_CACHE_HEADER_VERSION_ONE` specifies version one of the pipeline
  ///     cache.
  ///
  pub enum VkPipelineCacheHeaderVersion {
    E_ONE = 1
  }
}

define_enum! {

  /// Supported primitive topologies
  ///
  /// *Primitive topology* determines how consecutive vertices are organized into
  /// primitives, and determines the type of primitive that is used at the beginning
  /// of the graphics pipeline. The effective topology for later stages of the
  /// pipeline is altered by tessellation or geometry shading (if either is in use)
  /// and depends on the execution modes of those shaders. Supported topologies are
  /// defined by `VkPrimitiveTopology` and include.
  ///
  pub enum VkPrimitiveTopology {
    E_POINT_LIST = 0,
    E_LINE_LIST = 1,
    E_LINE_STRIP = 2,
    E_TRIANGLE_LIST = 3,
    E_TRIANGLE_STRIP = 4,
    E_TRIANGLE_FAN = 5,
    E_LINE_LIST_WITH_ADJACENCY = 6,
    E_LINE_STRIP_WITH_ADJACENCY = 7,
    E_TRIANGLE_LIST_WITH_ADJACENCY = 8,
    E_TRIANGLE_STRIP_WITH_ADJACENCY = 9,
    E_PATCH_LIST = 10
  }
}

define_enum! {

  /// Buffer and image sharing modes
  ///
  /// Buffer and image objects are created with a *sharing mode* controlling how they
  /// can: be accessed from queues.
  ///
  ///   - `VK_SHARING_MODE_EXCLUSIVE` specifies that access to any range or image
  ///     subresource of the object will be exclusive to a single queue family at a
  ///     time.
  ///
  ///   - `VK_SHARING_MODE_CONCURRENT` specifies that concurrent access to any range
  ///     or image subresource of the object from multiple queue families is
  ///     supported.
  ///
  /// > **Note**
  /// >
  /// > `VK_SHARING_MODE_CONCURRENT` may: result in lower performance access to the
  /// > buffer or image than `VK_SHARING_MODE_EXCLUSIVE`.
  ///
  /// Ranges of buffers and image subresources of image objects created using
  /// `VK_SHARING_MODE_EXCLUSIVE` must: only be accessed by queues in the same queue
  /// family at any given time. In order for a different queue family to be able to
  /// interpret the memory contents of a range or image subresource, the application
  /// must: perform a [queue family ownership
  /// transfer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-queue-transfers).
  ///
  /// Upon creation, resources using `VK_SHARING_MODE_EXCLUSIVE` are not owned by any
  /// queue family. A buffer or image memory barrier is not required to acquire
  /// *ownership* when no queue family owns the resource - it is implicitly acquired
  /// upon first use within a queue.
  ///
  /// > **Note**
  /// >
  /// > Images still require a [layout transition](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-image-layouts) from
  /// > `VK_IMAGE_LAYOUT_UNDEFINED` or `VK_IMAGE_LAYOUT_PREINITIALIZED` before being
  /// > used on the first queue.
  ///
  /// A queue family can: take ownership of an image subresource or buffer range of a
  /// resource created with `VK_SHARING_MODE_EXCLUSIVE`, without an ownership
  /// transfer, in the same way as for a resource that was just created; however,
  /// taking ownership in this way has the effect that the contents of the image
  /// subresource or buffer range are undefined.
  ///
  /// Ranges of buffers and image subresources of image objects created using
  /// `VK_SHARING_MODE_CONCURRENT` must: only be accessed by queues from the queue
  /// families specified through the `queueFamilyIndexCount` and `pQueueFamilyIndices`
  /// members of the corresponding create info structures.
  ///
  pub enum VkSharingMode {
    E_EXCLUSIVE = 0,
    E_CONCURRENT = 1
  }
}

define_enum! {

  /// Type of index buffer indices
  ///
  /// Possible values of `vkCmdBindIndexBuffer::indexType`, specifying the size of
  /// indices.
  ///
  ///   - `VK_INDEX_TYPE_UINT16` specifies that indices are 16-bit unsigned integer
  ///     values.
  ///
  ///   - `VK_INDEX_TYPE_UINT32` specifies that indices are 32-bit unsigned integer
  ///     values.
  ///
  pub enum VkIndexType {
    E_UINT16 = 0,
    E_UINT32 = 1
  }
}

define_enum! {

  /// Specify filters used for texture lookups
  ///
  /// Possible values of the `VkSamplerCreateInfo::magFilter` and `minFilter`
  /// parameters, specifying filters used for texture lookups, are.
  ///
  ///   - `VK_FILTER_NEAREST` specifies nearest filtering.
  ///
  ///   - `VK_FILTER_LINEAR` specifies linear filtering.
  ///
  ///   - `VK_FILTER_CUBIC_IMG` specifies cubic filtering.
  ///
  /// These filters are described in detail in [Texel
  /// Filtering](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-texel-filtering).
  ///
  pub enum VkFilter {
    E_NEAREST = 0,
    E_LINEAR = 1,

    // feature: VK_IMG_filter_cubic
    #[cfg(feature = "VK_IMG_filter_cubic")]
    E_CUBIC_IMG = 1000015000
  }
}

define_enum! {

  /// Specify mipmap mode used for texture lookups
  ///
  /// Possible values of the `VkSamplerCreateInfo::mipmapMode`, specifying the mipmap
  /// mode used for texture lookups.
  ///
  ///   - `VK_SAMPLER_MIPMAP_MODE_NEAREST` specifies nearest filtering.
  ///
  ///   - `VK_SAMPLER_MIPMAP_MODE_LINEAR` specifies linear filtering.
  ///
  /// These modes are described in detail in [Texel
  /// Filtering](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-texel-filtering).
  ///
  pub enum VkSamplerMipmapMode {
    E_NEAREST = 0,
    E_LINEAR = 1
  }
}

define_enum! {

  /// Specify behavior of sampling with texture coordinates outside an image
  ///
  /// Possible values of the `VkSamplerCreateInfo`::ptext:addressMode\* parameters,
  /// specifying the behavior of sampling with coordinates outside the range \[0,1\]
  /// for the respective u, v, or w coordinate as defined in the [Wrapping
  /// Operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-wrapping-operation) section.
  ///
  ///   - `VK_SAMPLER_ADDRESS_MODE_REPEAT` specifies that the repeat wrap mode will be
  ///     used.
  ///
  ///   - `VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT` specifies that the mirrored repeat
  ///     wrap mode will be used.
  ///
  ///   - `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE` specifies that the clamp to edge
  ///     wrap mode will be used.
  ///
  ///   - `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER` specifies that the clamp to border
  ///     wrap mode will be used.
  ///
  ///   - `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` specifies that the mirror
  ///     clamp to edge wrap mode will be used. This is only valid if the  extension
  ///     is enabled.
  ///
  pub enum VkSamplerAddressMode {
    E_REPEAT = 0,
    E_MIRRORED_REPEAT = 1,
    E_CLAMP_TO_EDGE = 2,
    E_CLAMP_TO_BORDER = 3,

    // feature: VK_KHR_sampler_mirror_clamp_to_edge
    #[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
    E_MIRROR_CLAMP_TO_EDGE = 4
  }
}

define_enum! {

  /// Stencil comparison function
  ///
  /// Possible values of VkStencilOpState::\`compareOp\`, specifying the stencil
  /// comparison function.
  ///
  ///   - `VK_COMPARE_OP_NEVER` specifies that the test never passes.
  ///
  ///   - `VK_COMPARE_OP_LESS` specifies that the test passes when R \< S.
  ///
  ///   - `VK_COMPARE_OP_EQUAL` specifies that the test passes when R = S.
  ///
  ///   - `VK_COMPARE_OP_LESS_OR_EQUAL` specifies that the test passes when R {leq} S.
  ///
  ///   - `VK_COMPARE_OP_GREATER` specifies that the test passes when R \> S.
  ///
  ///   - `VK_COMPARE_OP_NOT_EQUAL` specifies that the test passes when R {neq} S.
  ///
  ///   - `VK_COMPARE_OP_GREATER_OR_EQUAL` specifies that the test passes when R {geq}
  ///     S.
  ///
  ///   - `VK_COMPARE_OP_ALWAYS` specifies that the test always passes.
  ///
  pub enum VkCompareOp {
    E_NEVER = 0,
    E_LESS = 1,
    E_EQUAL = 2,
    E_LESS_OR_EQUAL = 3,
    E_GREATER = 4,
    E_NOT_EQUAL = 5,
    E_GREATER_OR_EQUAL = 6,
    E_ALWAYS = 7
  }
}

define_enum! {

  /// Control polygon rasterization mode
  ///
  /// Possible values of the `VkPipelineRasterizationStateCreateInfo::polygonMode`
  /// property of the currently active pipeline, specifying the method of
  /// rasterization for polygons.
  ///
  ///   - `VK_POLYGON_MODE_POINT` specifies that polygon vertices are drawn as points.
  ///
  ///   - `VK_POLYGON_MODE_LINE` specifies that polygon edges are drawn as line
  ///     segments.
  ///
  ///   - `VK_POLYGON_MODE_FILL` specifies that polygons are rendered using the
  ///     polygon rasterization rules in this section.
  ///
  ///   - `VK_POLYGON_MODE_FILL_RECTANGLE_NV` specifies that polygons are rendered
  ///     using polygon rasterization rules, modified to consider a sample within the
  ///     primitive if the sample location is inside the axis-aligned bounding box of
  ///     the triangle after projection. Note that the barycentric weights used in
  ///     attribute interpolation can: extend outside the range \[0,1\] when these
  ///     primitives are shaded. Special treatment is given to a sample position on
  ///     the boundary edge of the bounding box. In such a case, if two rectangles lie
  ///     on either side of a common edge (with identical endpoints) on which a sample
  ///     position lies, then exactly one of the triangles must: produce a fragment
  ///     that covers that sample during rasterization.
  ///
  ///     Polygons rendered in `VK_POLYGON_MODE_FILL_RECTANGLE_NV` mode may: be
  ///     clipped by the frustum or by user clip planes. If clipping is applied, the
  ///     triangle is culled rather than clipped.
  ///
  ///     Area calculation and facingness are determined for
  ///     `VK_POLYGON_MODE_FILL_RECTANGLE_NV` mode using the triangle’s vertices.
  ///
  /// These modes affect only the final rasterization of polygons: in particular, a
  /// polygon’s vertices are shaded and the polygon is clipped and possibly culled
  /// before these modes are applied.
  ///
  pub enum VkPolygonMode {
    E_FILL = 0,
    E_LINE = 1,
    E_POINT = 2,

    // feature: VK_NV_fill_rectangle
    #[cfg(feature = "VK_NV_fill_rectangle")]
    E_FILL_RECTANGLE_NV = 1000153000
  }
}

define_bitmask! {

  /// Bitmask controlling triangle culling
  ///
  /// Once the orientation of triangles is determined, they are culled according to
  /// the `VkPipelineRasterizationStateCreateInfo::cullMode` property of the currently
  /// active pipeline. Possible values are.
  ///
  ///   - `VK_CULL_MODE_NONE` specifies that no triangles are discarded
  ///
  ///   - `VK_CULL_MODE_FRONT_BIT` specifies that front-facing triangles are discarded
  ///
  ///   - `VK_CULL_MODE_BACK_BIT` specifies that back-facing triangles are discarded
  ///
  ///   - `VK_CULL_MODE_FRONT_AND_BACK` specifies that all triangles are discarded.
  ///
  /// Following culling, fragments are produced for any triangles which have not been
  /// discarded.
  ///
  pub enum VkCullModeFlagBits {
    BIT_NONE = 0,
    BIT_FRONT = 1<<0,
    BIT_BACK = 1<<1,
    BIT_FRONT_AND_BACK = 0x00000003
  }
}

define_enum! {

  /// Interpret polygon front-facing orientation
  ///
  /// The first step of polygon rasterization is to determine whether the triangle is
  /// *back-facing* or *front-facing*. This determination is made based on the sign of
  /// the (clipped or unclipped) polygon’s area computed in framebuffer coordinates.
  /// One way to compute this area is:
  ///
  /// a = -{1 \\over 2}\\sum\_{i=0}^{n-1} x\_f^i y\_f^{i \\oplus 1} - x\_f^{i \\oplus
  /// 1} y\_f^i
  ///
  /// a = -{1 \\over 2}\\sum\_{i=0}^{n-1} x\_f^i y\_f^{i \\oplus 1} - x\_f^{i \\oplus
  /// 1} y\_f^i
  ///
  /// where and are the x and y framebuffer coordinates of the ith vertex of the
  /// n-vertex polygon (vertices are numbered starting at zero for the purposes of
  /// this computation) and i {oplus} 1 is (i + 1) mod n.
  ///
  /// The interpretation of the sign of a is determined by the
  /// `VkPipelineRasterizationStateCreateInfo::frontFace` property of the currently
  /// active pipeline. Possible values are.
  ///
  ///   - `VK_FRONT_FACE_COUNTER_CLOCKWISE` specifies that a triangle with positive
  ///     area is considered front-facing.
  ///
  ///   - `VK_FRONT_FACE_CLOCKWISE` specifies that a triangle with negative area is
  ///     considered front-facing.
  ///
  /// Any triangle which is not front-facing is back-facing, including zero-area
  /// triangles.
  ///
  pub enum VkFrontFace {
    E_COUNTER_CLOCKWISE = 0,
    E_CLOCKWISE = 1
  }
}

define_enum! {

  /// Framebuffer blending factors
  ///
  /// The source and destination color and alpha blending factors are selected from
  /// the enum.
  ///
  /// The semantics of each enum value is described in the table below:
  ///
  /// <table>
  /// <caption>Blend Factors</caption>
  /// <colgroup>
  /// <col width="59%" />
  /// <col width="28%" />
  /// <col width="12%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">VkBlendFactor</th>
  /// <th align="left">RGB Blend Factors (S<sub>r</sub>,S<sub>g</sub>,S<sub>b</sub>) or (D<sub>r</sub>,D<sub>g</sub>,D<sub>b</sub>)</th>
  /// <th align="left">Alpha Blend Factor (S<sub>a</sub> or D<sub>a</sub>)</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ZERO</code></p></td>
  /// <td align="left"><p>(0,0,0)</p></td>
  /// <td align="left"><p>0</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE</code></p></td>
  /// <td align="left"><p>(1,1,1)</p></td>
  /// <td align="left"><p>1</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_SRC_COLOR</code></p></td>
  /// <td align="left"><p>(R<sub>s0</sub>,G<sub>s0</sub>,B<sub>s0</sub>)</p></td>
  /// <td align="left"><p>A<sub>s0</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR</code></p></td>
  /// <td align="left"><p>(1-R<sub>s0</sub>,1-G<sub>s0</sub>,1-B<sub>s0</sub>)</p></td>
  /// <td align="left"><p>1-A<sub>s0</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_DST_COLOR</code></p></td>
  /// <td align="left"><p>(R<sub>d</sub>,G<sub>d</sub>,B<sub>d</sub>)</p></td>
  /// <td align="left"><p>A<sub>d</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR</code></p></td>
  /// <td align="left"><p>(1-R<sub>d</sub>,1-G<sub>d</sub>,1-B<sub>d</sub>)</p></td>
  /// <td align="left"><p>1-A<sub>d</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_SRC_ALPHA</code></p></td>
  /// <td align="left"><p>(A<sub>s0</sub>,A<sub>s0</sub>,A<sub>s0</sub>)</p></td>
  /// <td align="left"><p>A<sub>s0</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA</code></p></td>
  /// <td align="left"><p>(1-A<sub>s0</sub>,1-A<sub>s0</sub>,1-A<sub>s0</sub>)</p></td>
  /// <td align="left"><p>1-A<sub>s0</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_DST_ALPHA</code></p></td>
  /// <td align="left"><p>(A<sub>d</sub>,A<sub>d</sub>,A<sub>d</sub>)</p></td>
  /// <td align="left"><p>A<sub>d</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA</code></p></td>
  /// <td align="left"><p>(1-A<sub>d</sub>,1-A<sub>d</sub>,1-A<sub>d</sub>)</p></td>
  /// <td align="left"><p>1-A<sub>d</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_CONSTANT_COLOR</code></p></td>
  /// <td align="left"><p>(R<sub>c</sub>,G<sub>c</sub>,B<sub>c</sub>)</p></td>
  /// <td align="left"><p>A<sub>c</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR</code></p></td>
  /// <td align="left"><p>(1-R<sub>c</sub>,1-G<sub>c</sub>,1-B<sub>c</sub>)</p></td>
  /// <td align="left"><p>1-A<sub>c</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_CONSTANT_ALPHA</code></p></td>
  /// <td align="left"><p>(A<sub>c</sub>,A<sub>c</sub>,A<sub>c</sub>)</p></td>
  /// <td align="left"><p>A<sub>c</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA</code></p></td>
  /// <td align="left"><p>(1-A<sub>c</sub>,1-A<sub>c</sub>,1-A<sub>c</sub>)</p></td>
  /// <td align="left"><p>1-A<sub>c</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_SRC_ALPHA_SATURATE</code></p></td>
  /// <td align="left"><p>(f,f,f); f = min(A<sub>s0</sub>,1-A<sub>d</sub>)</p></td>
  /// <td align="left"><p>1</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_SRC1_COLOR</code></p></td>
  /// <td align="left"><p>(R<sub>s1</sub>,G<sub>s1</sub>,B<sub>s1</sub>)</p></td>
  /// <td align="left"><p>A<sub>s1</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR</code></p></td>
  /// <td align="left"><p>(1-R<sub>s1</sub>,1-G<sub>s1</sub>,1-B<sub>s1</sub>)</p></td>
  /// <td align="left"><p>1-A<sub>s1</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_SRC1_ALPHA</code></p></td>
  /// <td align="left"><p>(A<sub>s1</sub>,A<sub>s1</sub>,A<sub>s1</sub>)</p></td>
  /// <td align="left"><p>A<sub>s1</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA</code></p></td>
  /// <td align="left"><p>(1-A<sub>s1</sub>,1-A<sub>s1</sub>,1-A<sub>s1</sub>)</p></td>
  /// <td align="left"><p>1-A<sub>s1</sub></p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  /// In this table, the following conventions are used:
  ///
  ///   - R<sub>s0</sub>,G<sub>s0</sub>,B<sub>s0</sub> and A<sub>s0</sub> represent
  ///     the first source color R, G, B, and A components, respectively, for the
  ///     fragment output location corresponding to the color attachment being
  ///     blended.
  ///
  ///   - R<sub>s1</sub>,G<sub>s1</sub>,B<sub>s1</sub> and A<sub>s1</sub> represent
  ///     the second source color R, G, B, and A components, respectively, used in
  ///     dual source blending modes, for the fragment output location corresponding
  ///     to the color attachment being blended.
  ///
  ///   - R<sub>d</sub>,G<sub>d</sub>,B<sub>d</sub> and A<sub>d</sub> represent the R,
  ///     G, B, and A components of the destination color. That is, the color
  ///     currently in the corresponding color attachment for this fragment/sample.
  ///
  ///   - R<sub>c</sub>,G<sub>c</sub>,B<sub>c</sub> and A<sub>c</sub> represent the
  ///     blend constant R, G, B, and A components, respectively.
  ///
  pub enum VkBlendFactor {
    E_ZERO = 0,
    E_ONE = 1,
    E_SRC_COLOR = 2,
    E_ONE_MINUS_SRC_COLOR = 3,
    E_DST_COLOR = 4,
    E_ONE_MINUS_DST_COLOR = 5,
    E_SRC_ALPHA = 6,
    E_ONE_MINUS_SRC_ALPHA = 7,
    E_DST_ALPHA = 8,
    E_ONE_MINUS_DST_ALPHA = 9,
    E_CONSTANT_COLOR = 10,
    E_ONE_MINUS_CONSTANT_COLOR = 11,
    E_CONSTANT_ALPHA = 12,
    E_ONE_MINUS_CONSTANT_ALPHA = 13,
    E_SRC_ALPHA_SATURATE = 14,
    E_SRC1_COLOR = 15,
    E_ONE_MINUS_SRC1_COLOR = 16,
    E_SRC1_ALPHA = 17,
    E_ONE_MINUS_SRC1_ALPHA = 18
  }
}

define_enum! {

  /// Framebuffer blending operations
  ///
  /// Once the source and destination blend factors have been selected, they along
  /// with the source and destination components are passed to the blending
  /// operations. RGB and alpha components can: use different operations. Possible
  /// values of `VkBlendOp`, specifying the operations.
  ///
  /// The semantics of each basic blend operations is described in the table below:
  ///
  /// <table style="width:100%;">
  /// <caption>Basic Blend Operations</caption>
  /// <colgroup>
  /// <col width="44%" />
  /// <col width="30%" />
  /// <col width="24%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">VkBlendOp</th>
  /// <th align="left">RGB Components</th>
  /// <th align="left">Alpha Component</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_OP_ADD</code></p></td>
  /// <td align="left"><p>R = R<sub>s0</sub> {times} S<sub>r</sub> + R<sub>d</sub> {times} D<sub>r</sub><br />
  /// G = G<sub>s0</sub> {times} S<sub>g</sub> + G<sub>d</sub> {times} D<sub>g</sub><br />
  /// B = B<sub>s0</sub> {times} S<sub>b</sub> + B<sub>d</sub> {times} D<sub>b</sub></p></td>
  /// <td align="left"><p>A = A<sub>s0</sub> {times} S<sub>a</sub> + A<sub>d</sub> {times} D<sub>a</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_OP_SUBTRACT</code></p></td>
  /// <td align="left"><p>R = R<sub>s0</sub> {times} S<sub>r</sub> - R<sub>d</sub> {times} D<sub>r</sub><br />
  /// G = G<sub>s0</sub> {times} S<sub>g</sub> - G<sub>d</sub> {times} D<sub>g</sub><br />
  /// B = B<sub>s0</sub> {times} S<sub>b</sub> - B<sub>d</sub> {times} D<sub>b</sub></p></td>
  /// <td align="left"><p>A = A<sub>s0</sub> {times} S<sub>a</sub> - A<sub>d</sub> {times} D<sub>a</sub></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_OP_REVERSE_SUBTRACT</code></p></td>
  /// <td align="left"><p>R = R<sub>d</sub> {times} D<sub>r</sub> - R<sub>s0</sub> {times} S<sub>r</sub><br />
  /// G = G<sub>d</sub> {times} D<sub>g</sub> - G<sub>s0</sub> {times} S<sub>g</sub><br />
  /// B = B<sub>d</sub> {times} D<sub>b</sub> - B<sub>s0</sub> {times} S<sub>b</sub></p></td>
  /// <td align="left"><p>A = A<sub>d</sub> {times} D<sub>a</sub> - A<sub>s0</sub> {times} S<sub>a</sub></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_OP_MIN</code></p></td>
  /// <td align="left"><p>R = min(R<sub>s0</sub>,R<sub>d</sub>)<br />
  /// G = min(G<sub>s0</sub>,G<sub>d</sub>)<br />
  /// B = min(B<sub>s0</sub>,B<sub>d</sub>)</p></td>
  /// <td align="left"><p>A = min(A<sub>s0</sub>,A<sub>d</sub>)</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_OP_MAX</code></p></td>
  /// <td align="left"><p>R = max(R<sub>s0</sub>,R<sub>d</sub>)<br />
  /// G = max(G<sub>s0</sub>,G<sub>d</sub>)<br />
  /// B = max(B<sub>s0</sub>,B<sub>d</sub>)</p></td>
  /// <td align="left"><p>A = max(A<sub>s0</sub>,A<sub>d</sub>)</p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  /// In this table, the following conventions are used:
  ///
  ///   - R<sub>s0</sub>, G<sub>s0</sub>, B<sub>s0</sub> and A<sub>s0</sub> represent
  ///     the first source color R, G, B, and A components, respectively.
  ///
  ///   - R<sub>d</sub>, G<sub>d</sub>, B<sub>d</sub> and A<sub>d</sub> represent the
  ///     R, G, B, and A components of the destination color. That is, the color
  ///     currently in the corresponding color attachment for this fragment/sample.
  ///
  ///   - S<sub>r</sub>, S<sub>g</sub>, S<sub>b</sub> and S<sub>a</sub> represent the
  ///     source blend factor R, G, B, and A components, respectively.
  ///
  ///   - D<sub>r</sub>, D<sub>g</sub>, D<sub>b</sub> and D<sub>a</sub> represent the
  ///     destination blend factor R, G, B, and A components, respectively.
  ///
  /// The blending operation produces a new set of values R, G, B and A, which are
  /// written to the framebuffer attachment. If blending is not enabled for this
  /// attachment, then R, G, B and A are assigned R<sub>s0</sub>, G<sub>s0</sub>,
  /// B<sub>s0</sub> and A<sub>s0</sub>, respectively.
  ///
  /// If the color attachment is fixed-point, the components of the source and
  /// destination values and blend factors are each clamped to \[0,1\] or \[-1,1\]
  /// respectively for an unsigned normalized or signed normalized color attachment
  /// prior to evaluating the blend operations. If the color attachment is
  /// floating-point, no clamping occurs.
  ///
  pub enum VkBlendOp {
    E_ADD = 0,
    E_SUBTRACT = 1,
    E_REVERSE_SUBTRACT = 2,
    E_MIN = 3,
    E_MAX = 4,

    // feature: VK_EXT_blend_operation_advanced
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_ZERO_EXT = 1000148000,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_SRC_EXT = 1000148001,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_DST_EXT = 1000148002,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_SRC_OVER_EXT = 1000148003,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_DST_OVER_EXT = 1000148004,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_SRC_IN_EXT = 1000148005,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_DST_IN_EXT = 1000148006,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_SRC_OUT_EXT = 1000148007,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_DST_OUT_EXT = 1000148008,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_SRC_ATOP_EXT = 1000148009,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_DST_ATOP_EXT = 1000148010,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_XOR_EXT = 1000148011,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_MULTIPLY_EXT = 1000148012,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_SCREEN_EXT = 1000148013,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_OVERLAY_EXT = 1000148014,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_DARKEN_EXT = 1000148015,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_LIGHTEN_EXT = 1000148016,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_COLORDODGE_EXT = 1000148017,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_COLORBURN_EXT = 1000148018,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_HARDLIGHT_EXT = 1000148019,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_SOFTLIGHT_EXT = 1000148020,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_DIFFERENCE_EXT = 1000148021,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_EXCLUSION_EXT = 1000148022,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_INVERT_EXT = 1000148023,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_INVERT_RGB_EXT = 1000148024,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_LINEARDODGE_EXT = 1000148025,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_LINEARBURN_EXT = 1000148026,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_VIVIDLIGHT_EXT = 1000148027,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_LINEARLIGHT_EXT = 1000148028,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_PINLIGHT_EXT = 1000148029,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_HARDMIX_EXT = 1000148030,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_HSL_HUE_EXT = 1000148031,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_HSL_SATURATION_EXT = 1000148032,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_HSL_COLOR_EXT = 1000148033,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_HSL_LUMINOSITY_EXT = 1000148034,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_PLUS_EXT = 1000148035,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_PLUS_CLAMPED_EXT = 1000148036,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_PLUS_CLAMPED_ALPHA_EXT = 1000148037,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_PLUS_DARKER_EXT = 1000148038,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_MINUS_EXT = 1000148039,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_MINUS_CLAMPED_EXT = 1000148040,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_CONTRAST_EXT = 1000148041,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_INVERT_OVG_EXT = 1000148042,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_RED_EXT = 1000148043,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_GREEN_EXT = 1000148044,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_BLUE_EXT = 1000148045
  }
}

define_enum! {

  /// Stencil comparison function
  ///
  /// Possible values of the `failOp`, `passOp`, and `depthFailOp` members of
  /// `VkStencilOpState`, specifying what happens to the stored stencil value if this
  /// or certain subsequent tests fail or pass.
  ///
  ///   - `VK_STENCIL_OP_KEEP` keeps the current value.
  ///
  ///   - `VK_STENCIL_OP_ZERO` sets the value to 0.
  ///
  ///   - `VK_STENCIL_OP_REPLACE` sets the value to `reference`.
  ///
  ///   - `VK_STENCIL_OP_INCREMENT_AND_CLAMP` increments the current value and clamps
  ///     to the maximum representable unsigned value.
  ///
  ///   - `VK_STENCIL_OP_DECREMENT_AND_CLAMP` decrements the current value and clamps
  ///     to 0.
  ///
  ///   - `VK_STENCIL_OP_INVERT` bitwise-inverts the current value.
  ///
  ///   - `VK_STENCIL_OP_INCREMENT_AND_WRAP` increments the current value and wraps to
  ///     0 when the maximum value would have been exceeded.
  ///
  ///   - `VK_STENCIL_OP_DECREMENT_AND_WRAP` decrements the current value and wraps to
  ///     the maximum possible value when the value would go below 0.
  ///
  /// For purposes of increment and decrement, the stencil bits are considered as an
  /// unsigned integer.
  ///
  /// If the stencil test fails, the sample’s coverage bit is cleared in the fragment.
  /// If there is no stencil framebuffer attachment, stencil modification cannot:
  /// occur, and it is as if the stencil tests always pass.
  ///
  /// If the stencil test passes, the `writeMask` member of the `VkStencilOpState`
  /// structures controls how the updated stencil value is written to the stencil
  /// framebuffer attachment.
  ///
  /// The least significant s bits of `writeMask`, where s is the number of bits in
  /// the stencil framebuffer attachment, specify an integer mask. Where a 1 appears
  /// in this mask, the corresponding bit in the stencil value in the depth/stencil
  /// attachment is written; where a 0 appears, the bit is not written. The
  /// `writeMask` value uses either the front-facing or back-facing state based on the
  /// facingness of the fragment. Fragments generated by front-facing primitives use
  /// the front mask and fragments generated by back-facing primitives use the back
  /// mask.
  ///
  pub enum VkStencilOp {
    E_KEEP = 0,
    E_ZERO = 1,
    E_REPLACE = 2,
    E_INCREMENT_AND_CLAMP = 3,
    E_DECREMENT_AND_CLAMP = 4,
    E_INVERT = 5,
    E_INCREMENT_AND_WRAP = 6,
    E_DECREMENT_AND_WRAP = 7
  }
}

define_enum! {

  /// Framebuffer logical operations
  ///
  /// Logical operations are controlled by the `logicOpEnable` and `logicOp` members
  /// of `VkPipelineColorBlendStateCreateInfo`. If `logicOpEnable` is `VK_TRUE`, then
  /// a logical operation selected by `logicOp` is applied between each color
  /// attachment and the fragment’s corresponding output value, and blending of all
  /// attachments is treated as if it were disabled. Any attachments using color
  /// formats for which logical operations are not supported simply pass through the
  /// color values unmodified. The logical operation is applied independently for each
  /// of the red, green, blue, and alpha components. The `logicOp` is selected from
  /// the following operations.
  ///
  /// The logical operations supported by Vulkan are summarized in the following table
  /// in which
  ///
  ///   - {lnot} is bitwise invert,
  ///
  ///   - {land} is bitwise and,
  ///
  ///   - {lor} is bitwise or,
  ///
  ///   - {oplus} is bitwise exclusive or,
  ///
  ///   - s is the fragment’s R<sub>s0</sub>, G<sub>s0</sub>, B<sub>s0</sub> or
  ///     A<sub>s0</sub> component value for the fragment output corresponding to the
  ///     color attachment being updated, and
  ///
  ///   - d is the color attachment’s R, G, B or A component value:
  ///
  /// <table>
  /// <caption>Logical Operations</caption>
  /// <colgroup>
  /// <col width="50%" />
  /// <col width="50%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">Mode</th>
  /// <th align="left">Operation</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_LOGIC_OP_CLEAR</code></p></td>
  /// <td align="left"><p>0</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_LOGIC_OP_AND</code></p></td>
  /// <td align="left"><p>s {land} d</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_LOGIC_OP_AND_REVERSE</code></p></td>
  /// <td align="left"><p>s {land} {lnot} d</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_LOGIC_OP_COPY</code></p></td>
  /// <td align="left"><p>s</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_LOGIC_OP_AND_INVERTED</code></p></td>
  /// <td align="left"><p>{lnot} s {land} d</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_LOGIC_OP_NO_OP</code></p></td>
  /// <td align="left"><p>d</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_LOGIC_OP_XOR</code></p></td>
  /// <td align="left"><p>s {oplus} d</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_LOGIC_OP_OR</code></p></td>
  /// <td align="left"><p>s {lor} d</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_LOGIC_OP_NOR</code></p></td>
  /// <td align="left"><p>{lnot} (s {lor} d)</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_LOGIC_OP_EQUIVALENT</code></p></td>
  /// <td align="left"><p>{lnot} (s {oplus} d)</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_LOGIC_OP_INVERT</code></p></td>
  /// <td align="left"><p>{lnot} d</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_LOGIC_OP_OR_REVERSE</code></p></td>
  /// <td align="left"><p>s {lor} {lnot} d</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_LOGIC_OP_COPY_INVERTED</code></p></td>
  /// <td align="left"><p>{lnot} s</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_LOGIC_OP_OR_INVERTED</code></p></td>
  /// <td align="left"><p>{lnot} s {lor} d</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_LOGIC_OP_NAND</code></p></td>
  /// <td align="left"><p>{lnot} (s {land} d)</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_LOGIC_OP_SET</code></p></td>
  /// <td align="left"><p>all 1s</p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  /// The result of the logical operation is then written to the color attachment as
  /// controlled by the component write mask, described in [Blend
  /// Operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blendoperations).
  ///
  pub enum VkLogicOp {
    E_CLEAR = 0,
    E_AND = 1,
    E_AND_REVERSE = 2,
    E_COPY = 3,
    E_AND_INVERTED = 4,
    E_NO_OP = 5,
    E_XOR = 6,
    E_OR = 7,
    E_NOR = 8,
    E_EQUIVALENT = 9,
    E_INVERT = 10,
    E_OR_REVERSE = 11,
    E_COPY_INVERTED = 12,
    E_OR_INVERTED = 13,
    E_NAND = 14,
    E_SET = 15
  }
}

define_enum! {

  /// Allocation type
  ///
  /// The `allocationType` parameter to the `pfnInternalAllocation` and
  /// `pfnInternalFree` functions may: be one of the following values.
  ///
  ///   - `VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE` specifies that the allocation is
  ///     intended for execution by the host.
  ///
  pub enum VkInternalAllocationType {
    E_EXECUTABLE = 0
  }
}

define_enum! {

  /// Allocation scope
  ///
  /// Each allocation has an *allocation scope* which defines its lifetime and which
  /// object it is associated with. Possible values passed to the `allocationScope`
  /// parameter of the callback functions specified by `VkAllocationCallbacks`,
  /// indicating the allocation scope.
  ///
  ///   - `VK_SYSTEM_ALLOCATION_SCOPE_COMMAND` specifies that the allocation is scoped
  ///     to the duration of the Vulkan command.
  ///
  ///   - `VK_SYSTEM_ALLOCATION_SCOPE_OBJECT` specifies that the allocation is scoped
  ///     to the lifetime of the Vulkan object that is being created or used.
  ///
  ///   - `VK_SYSTEM_ALLOCATION_SCOPE_CACHE` specifies that the allocation is scoped
  ///     to the lifetime of a `VkPipelineCache` or `VkValidationCacheEXT` object.
  ///
  ///   - `VK_SYSTEM_ALLOCATION_SCOPE_DEVICE` specifies that the allocation is scoped
  ///     to the lifetime of the Vulkan device.
  ///
  ///   - `VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE` specifies that the allocation is
  ///     scoped to the lifetime of the Vulkan instance.
  ///
  /// Most Vulkan commands operate on a single object, or there is a sole object that
  /// is being created or manipulated. When an allocation uses an allocation scope of
  /// `VK_SYSTEM_ALLOCATION_SCOPE_OBJECT` or `VK_SYSTEM_ALLOCATION_SCOPE_CACHE`, the
  /// allocation is scoped to the object being created or manipulated.
  ///
  /// When an implementation requires host memory, it will make callbacks to the
  /// application using the most specific allocator and allocation scope available:
  ///
  ///   - If an allocation is scoped to the duration of a command, the allocator will
  ///     use the `VK_SYSTEM_ALLOCATION_SCOPE_COMMAND` allocation scope. The most
  ///     specific allocator available is used: if the object being created or
  ///     manipulated has an allocator, that object’s allocator will be used, else if
  ///     the parent `VkDevice` has an allocator it will be used, else if the parent
  ///     `VkInstance` has an allocator it will be used. Else,
  ///
  ///   - If an allocation is associated with an object of type `VkValidationCacheEXT`
  ///     or `VkPipelineCache`, the allocator will use the
  ///     `VK_SYSTEM_ALLOCATION_SCOPE_CACHE` allocation scope. The most specific
  ///     allocator available is used (cache, else device, else instance). Else,
  ///
  ///   - If an allocation is scoped to the lifetime of an object, that object is
  ///     being created or manipulated by the command, and that object’s type is not
  ///     `VkDevice` or `VkInstance`, the allocator will use an allocation scope of
  ///     `VK_SYSTEM_ALLOCATION_SCOPE_OBJECT`. The most specific allocator available
  ///     is used (object, else device, else instance). Else,
  ///
  ///   - If an allocation is scoped to the lifetime of a device, the allocator will
  ///     use an allocation scope of `VK_SYSTEM_ALLOCATION_SCOPE_DEVICE`. The most
  ///     specific allocator available is used (device, else instance). Else,
  ///
  ///   - If the allocation is scoped to the lifetime of an instance and the instance
  ///     has an allocator, its allocator will be used with an allocation scope of
  ///     `VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE`.
  ///
  ///   - Otherwise an implementation will allocate memory through an alternative
  ///     mechanism that is unspecified.
  ///
  pub enum VkSystemAllocationScope {
    E_COMMAND = 0,
    E_OBJECT = 1,
    E_CACHE = 2,
    E_DEVICE = 3,
    E_INSTANCE = 4
  }
}

define_enum! {

  /// Supported physical device types
  ///
  /// The physical device types which may: be returned in
  /// `VkPhysicalDeviceProperties::deviceType` are.
  ///
  ///   - `VK_PHYSICAL_DEVICE_TYPE_OTHER` - the device does not match any other
  ///     available types.
  ///
  ///   - `VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU` - the device is typically one
  ///     embedded in or tightly coupled with the host.
  ///
  ///   - `VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU` - the device is typically a separate
  ///     processor connected to the host via an interlink.
  ///
  ///   - `VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU` - the device is typically a virtual
  ///     node in a virtualization environment.
  ///
  ///   - `VK_PHYSICAL_DEVICE_TYPE_CPU` - the device is typically running on the same
  ///     processors as the host.
  ///
  /// The physical device type is advertised for informational purposes only, and does
  /// not directly affect the operation of the system. However, the device type may:
  /// correlate with other advertised properties or capabilities of the system, such
  /// as how many memory heaps there are.
  ///
  pub enum VkPhysicalDeviceType {
    E_OTHER = 0,
    E_INTEGRATED_GPU = 1,
    E_DISCRETE_GPU = 2,
    E_VIRTUAL_GPU = 3,
    E_CPU = 4
  }
}

define_enum! {

  /// Specify rate at which vertex attributes are pulled from buffers
  ///
  /// Possible values of `VkVertexInputBindingDescription::inputRate`, specifying the
  /// rate at which vertex attributes are pulled from buffers.
  ///
  ///   - `VK_VERTEX_INPUT_RATE_VERTEX` specifies that vertex attribute addressing is
  ///     a function of the vertex index.
  ///
  ///   - `VK_VERTEX_INPUT_RATE_INSTANCE` specifies that vertex attribute addressing
  ///     is a function of the instance index.
  ///
  pub enum VkVertexInputRate {
    E_VERTEX = 0,
    E_INSTANCE = 1
  }
}

define_enum! {

  /// Available image formats
  ///
  /// Image formats which can: be passed to, and may: be returned from Vulkan
  /// commands.
  ///
  ///   - `VK_FORMAT_UNDEFINED` indicates that the format is not specified.
  ///
  ///   - `VK_FORMAT_R4G4_UNORM_PACK8` specifies a two-component, 8-bit packed
  ///     unsigned normalized format that has a 4-bit R component in bits 4..7, and a
  ///     4-bit G component in bits 0..3.
  ///
  ///   - `VK_FORMAT_R4G4B4A4_UNORM_PACK16` specifies a four-component, 16-bit packed
  ///     unsigned normalized format that has a 4-bit R component in bits 12..15, a
  ///     4-bit G component in bits 8..11, a 4-bit B component in bits 4..7, and a
  ///     4-bit A component in bits 0..3.
  ///
  ///   - `VK_FORMAT_B4G4R4A4_UNORM_PACK16` specifies a four-component, 16-bit packed
  ///     unsigned normalized format that has a 4-bit B component in bits 12..15, a
  ///     4-bit G component in bits 8..11, a 4-bit R component in bits 4..7, and a
  ///     4-bit A component in bits 0..3.
  ///
  ///   - `VK_FORMAT_R5G6B5_UNORM_PACK16` specifies a three-component, 16-bit packed
  ///     unsigned normalized format that has a 5-bit R component in bits 11..15, a
  ///     6-bit G component in bits 5..10, and a 5-bit B component in bits 0..4.
  ///
  ///   - `VK_FORMAT_B5G6R5_UNORM_PACK16` specifies a three-component, 16-bit packed
  ///     unsigned normalized format that has a 5-bit B component in bits 11..15, a
  ///     6-bit G component in bits 5..10, and a 5-bit R component in bits 0..4.
  ///
  ///   - `VK_FORMAT_R5G5B5A1_UNORM_PACK16` specifies a four-component, 16-bit packed
  ///     unsigned normalized format that has a 5-bit R component in bits 11..15, a
  ///     5-bit G component in bits 6..10, a 5-bit B component in bits 1..5, and a
  ///     1-bit A component in bit 0.
  ///
  ///   - `VK_FORMAT_B5G5R5A1_UNORM_PACK16` specifies a four-component, 16-bit packed
  ///     unsigned normalized format that has a 5-bit B component in bits 11..15, a
  ///     5-bit G component in bits 6..10, a 5-bit R component in bits 1..5, and a
  ///     1-bit A component in bit 0.
  ///
  ///   - `VK_FORMAT_A1R5G5B5_UNORM_PACK16` specifies a four-component, 16-bit packed
  ///     unsigned normalized format that has a 1-bit A component in bit 15, a 5-bit R
  ///     component in bits 10..14, a 5-bit G component in bits 5..9, and a 5-bit B
  ///     component in bits 0..4.
  ///
  ///   - `VK_FORMAT_R8_UNORM` specifies a one-component, 8-bit unsigned normalized
  ///     format that has a single 8-bit R component.
  ///
  ///   - `VK_FORMAT_R8_SNORM` specifies a one-component, 8-bit signed normalized
  ///     format that has a single 8-bit R component.
  ///
  ///   - `VK_FORMAT_R8_USCALED` specifies a one-component, 8-bit unsigned scaled
  ///     integer format that has a single 8-bit R component.
  ///
  ///   - `VK_FORMAT_R8_SSCALED` specifies a one-component, 8-bit signed scaled
  ///     integer format that has a single 8-bit R component.
  ///
  ///   - `VK_FORMAT_R8_UINT` specifies a one-component, 8-bit unsigned integer format
  ///     that has a single 8-bit R component.
  ///
  ///   - `VK_FORMAT_R8_SINT` specifies a one-component, 8-bit signed integer format
  ///     that has a single 8-bit R component.
  ///
  ///   - `VK_FORMAT_R8_SRGB` specifies a one-component, 8-bit unsigned normalized
  ///     format that has a single 8-bit R component stored with sRGB nonlinear
  ///     encoding.
  ///
  ///   - `VK_FORMAT_R8G8_UNORM` specifies a two-component, 16-bit unsigned normalized
  ///     format that has an 8-bit R component in byte 0, and an 8-bit G component in
  ///     byte 1.
  ///
  ///   - `VK_FORMAT_R8G8_SNORM` specifies a two-component, 16-bit signed normalized
  ///     format that has an 8-bit R component in byte 0, and an 8-bit G component in
  ///     byte 1.
  ///
  ///   - `VK_FORMAT_R8G8_USCALED` specifies a two-component, 16-bit unsigned scaled
  ///     integer format that has an 8-bit R component in byte 0, and an 8-bit G
  ///     component in byte 1.
  ///
  ///   - `VK_FORMAT_R8G8_SSCALED` specifies a two-component, 16-bit signed scaled
  ///     integer format that has an 8-bit R component in byte 0, and an 8-bit G
  ///     component in byte 1.
  ///
  ///   - `VK_FORMAT_R8G8_UINT` specifies a two-component, 16-bit unsigned integer
  ///     format that has an 8-bit R component in byte 0, and an 8-bit G component in
  ///     byte 1.
  ///
  ///   - `VK_FORMAT_R8G8_SINT` specifies a two-component, 16-bit signed integer
  ///     format that has an 8-bit R component in byte 0, and an 8-bit G component in
  ///     byte 1.
  ///
  ///   - `VK_FORMAT_R8G8_SRGB` specifies a two-component, 16-bit unsigned normalized
  ///     format that has an 8-bit R component stored with sRGB nonlinear encoding in
  ///     byte 0, and an 8-bit G component stored with sRGB nonlinear encoding in byte
  ///     1.
  ///
  ///   - `VK_FORMAT_R8G8B8_UNORM` specifies a three-component, 24-bit unsigned
  ///     normalized format that has an 8-bit R component in byte 0, an 8-bit G
  ///     component in byte 1, and an 8-bit B component in byte 2.
  ///
  ///   - `VK_FORMAT_R8G8B8_SNORM` specifies a three-component, 24-bit signed
  ///     normalized format that has an 8-bit R component in byte 0, an 8-bit G
  ///     component in byte 1, and an 8-bit B component in byte 2.
  ///
  ///   - `VK_FORMAT_R8G8B8_USCALED` specifies a three-component, 24-bit unsigned
  ///     scaled format that has an 8-bit R component in byte 0, an 8-bit G component
  ///     in byte 1, and an 8-bit B component in byte 2.
  ///
  ///   - `VK_FORMAT_R8G8B8_SSCALED` specifies a three-component, 24-bit signed scaled
  ///     format that has an 8-bit R component in byte 0, an 8-bit G component in byte
  ///     1, and an 8-bit B component in byte 2.
  ///
  ///   - `VK_FORMAT_R8G8B8_UINT` specifies a three-component, 24-bit unsigned integer
  ///     format that has an 8-bit R component in byte 0, an 8-bit G component in byte
  ///     1, and an 8-bit B component in byte 2.
  ///
  ///   - `VK_FORMAT_R8G8B8_SINT` specifies a three-component, 24-bit signed integer
  ///     format that has an 8-bit R component in byte 0, an 8-bit G component in byte
  ///     1, and an 8-bit B component in byte 2.
  ///
  ///   - `VK_FORMAT_R8G8B8_SRGB` specifies a three-component, 24-bit unsigned
  ///     normalized format that has an 8-bit R component stored with sRGB nonlinear
  ///     encoding in byte 0, an 8-bit G component stored with sRGB nonlinear encoding
  ///     in byte 1, and an 8-bit B component stored with sRGB nonlinear encoding in
  ///     byte 2.
  ///
  ///   - `VK_FORMAT_B8G8R8_UNORM` specifies a three-component, 24-bit unsigned
  ///     normalized format that has an 8-bit B component in byte 0, an 8-bit G
  ///     component in byte 1, and an 8-bit R component in byte 2.
  ///
  ///   - `VK_FORMAT_B8G8R8_SNORM` specifies a three-component, 24-bit signed
  ///     normalized format that has an 8-bit B component in byte 0, an 8-bit G
  ///     component in byte 1, and an 8-bit R component in byte 2.
  ///
  ///   - `VK_FORMAT_B8G8R8_USCALED` specifies a three-component, 24-bit unsigned
  ///     scaled format that has an 8-bit B component in byte 0, an 8-bit G component
  ///     in byte 1, and an 8-bit R component in byte 2.
  ///
  ///   - `VK_FORMAT_B8G8R8_SSCALED` specifies a three-component, 24-bit signed scaled
  ///     format that has an 8-bit B component in byte 0, an 8-bit G component in byte
  ///     1, and an 8-bit R component in byte 2.
  ///
  ///   - `VK_FORMAT_B8G8R8_UINT` specifies a three-component, 24-bit unsigned integer
  ///     format that has an 8-bit B component in byte 0, an 8-bit G component in byte
  ///     1, and an 8-bit R component in byte 2.
  ///
  ///   - `VK_FORMAT_B8G8R8_SINT` specifies a three-component, 24-bit signed integer
  ///     format that has an 8-bit B component in byte 0, an 8-bit G component in byte
  ///     1, and an 8-bit R component in byte 2.
  ///
  ///   - `VK_FORMAT_B8G8R8_SRGB` specifies a three-component, 24-bit unsigned
  ///     normalized format that has an 8-bit B component stored with sRGB nonlinear
  ///     encoding in byte 0, an 8-bit G component stored with sRGB nonlinear encoding
  ///     in byte 1, and an 8-bit R component stored with sRGB nonlinear encoding in
  ///     byte 2.
  ///
  ///   - `VK_FORMAT_R8G8B8A8_UNORM` specifies a four-component, 32-bit unsigned
  ///     normalized format that has an 8-bit R component in byte 0, an 8-bit G
  ///     component in byte 1, an 8-bit B component in byte 2, and an 8-bit A
  ///     component in byte 3.
  ///
  ///   - `VK_FORMAT_R8G8B8A8_SNORM` specifies a four-component, 32-bit signed
  ///     normalized format that has an 8-bit R component in byte 0, an 8-bit G
  ///     component in byte 1, an 8-bit B component in byte 2, and an 8-bit A
  ///     component in byte 3.
  ///
  ///   - `VK_FORMAT_R8G8B8A8_USCALED` specifies a four-component, 32-bit unsigned
  ///     scaled format that has an 8-bit R component in byte 0, an 8-bit G component
  ///     in byte 1, an 8-bit B component in byte 2, and an 8-bit A component in byte
  ///     3.
  ///
  ///   - `VK_FORMAT_R8G8B8A8_SSCALED` specifies a four-component, 32-bit signed
  ///     scaled format that has an 8-bit R component in byte 0, an 8-bit G component
  ///     in byte 1, an 8-bit B component in byte 2, and an 8-bit A component in byte
  ///     3.
  ///
  ///   - `VK_FORMAT_R8G8B8A8_UINT` specifies a four-component, 32-bit unsigned
  ///     integer format that has an 8-bit R component in byte 0, an 8-bit G component
  ///     in byte 1, an 8-bit B component in byte 2, and an 8-bit A component in byte
  ///     3.
  ///
  ///   - `VK_FORMAT_R8G8B8A8_SINT` specifies a four-component, 32-bit signed integer
  ///     format that has an 8-bit R component in byte 0, an 8-bit G component in byte
  ///     1, an 8-bit B component in byte 2, and an 8-bit A component in byte 3.
  ///
  ///   - `VK_FORMAT_R8G8B8A8_SRGB` specifies a four-component, 32-bit unsigned
  ///     normalized format that has an 8-bit R component stored with sRGB nonlinear
  ///     encoding in byte 0, an 8-bit G component stored with sRGB nonlinear encoding
  ///     in byte 1, an 8-bit B component stored with sRGB nonlinear encoding in byte
  ///     2, and an 8-bit A component in byte 3.
  ///
  ///   - `VK_FORMAT_B8G8R8A8_UNORM` specifies a four-component, 32-bit unsigned
  ///     normalized format that has an 8-bit B component in byte 0, an 8-bit G
  ///     component in byte 1, an 8-bit R component in byte 2, and an 8-bit A
  ///     component in byte 3.
  ///
  ///   - `VK_FORMAT_B8G8R8A8_SNORM` specifies a four-component, 32-bit signed
  ///     normalized format that has an 8-bit B component in byte 0, an 8-bit G
  ///     component in byte 1, an 8-bit R component in byte 2, and an 8-bit A
  ///     component in byte 3.
  ///
  ///   - `VK_FORMAT_B8G8R8A8_USCALED` specifies a four-component, 32-bit unsigned
  ///     scaled format that has an 8-bit B component in byte 0, an 8-bit G component
  ///     in byte 1, an 8-bit R component in byte 2, and an 8-bit A component in byte
  ///     3.
  ///
  ///   - `VK_FORMAT_B8G8R8A8_SSCALED` specifies a four-component, 32-bit signed
  ///     scaled format that has an 8-bit B component in byte 0, an 8-bit G component
  ///     in byte 1, an 8-bit R component in byte 2, and an 8-bit A component in byte
  ///     3.
  ///
  ///   - `VK_FORMAT_B8G8R8A8_UINT` specifies a four-component, 32-bit unsigned
  ///     integer format that has an 8-bit B component in byte 0, an 8-bit G component
  ///     in byte 1, an 8-bit R component in byte 2, and an 8-bit A component in byte
  ///     3.
  ///
  ///   - `VK_FORMAT_B8G8R8A8_SINT` specifies a four-component, 32-bit signed integer
  ///     format that has an 8-bit B component in byte 0, an 8-bit G component in byte
  ///     1, an 8-bit R component in byte 2, and an 8-bit A component in byte 3.
  ///
  ///   - `VK_FORMAT_B8G8R8A8_SRGB` specifies a four-component, 32-bit unsigned
  ///     normalized format that has an 8-bit B component stored with sRGB nonlinear
  ///     encoding in byte 0, an 8-bit G component stored with sRGB nonlinear encoding
  ///     in byte 1, an 8-bit R component stored with sRGB nonlinear encoding in byte
  ///     2, and an 8-bit A component in byte 3.
  ///
  ///   - `VK_FORMAT_A8B8G8R8_UNORM_PACK32` specifies a four-component, 32-bit packed
  ///     unsigned normalized format that has an 8-bit A component in bits 24..31, an
  ///     8-bit B component in bits 16..23, an 8-bit G component in bits 8..15, and an
  ///     8-bit R component in bits 0..7.
  ///
  ///   - `VK_FORMAT_A8B8G8R8_SNORM_PACK32` specifies a four-component, 32-bit packed
  ///     signed normalized format that has an 8-bit A component in bits 24..31, an
  ///     8-bit B component in bits 16..23, an 8-bit G component in bits 8..15, and an
  ///     8-bit R component in bits 0..7.
  ///
  ///   - `VK_FORMAT_A8B8G8R8_USCALED_PACK32` specifies a four-component, 32-bit
  ///     packed unsigned scaled integer format that has an 8-bit A component in bits
  ///     24..31, an 8-bit B component in bits 16..23, an 8-bit G component in bits
  ///     8..15, and an 8-bit R component in bits 0..7.
  ///
  ///   - `VK_FORMAT_A8B8G8R8_SSCALED_PACK32` specifies a four-component, 32-bit
  ///     packed signed scaled integer format that has an 8-bit A component in bits
  ///     24..31, an 8-bit B component in bits 16..23, an 8-bit G component in bits
  ///     8..15, and an 8-bit R component in bits 0..7.
  ///
  ///   - `VK_FORMAT_A8B8G8R8_UINT_PACK32` specifies a four-component, 32-bit packed
  ///     unsigned integer format that has an 8-bit A component in bits 24..31, an
  ///     8-bit B component in bits 16..23, an 8-bit G component in bits 8..15, and an
  ///     8-bit R component in bits 0..7.
  ///
  ///   - `VK_FORMAT_A8B8G8R8_SINT_PACK32` specifies a four-component, 32-bit packed
  ///     signed integer format that has an 8-bit A component in bits 24..31, an 8-bit
  ///     B component in bits 16..23, an 8-bit G component in bits 8..15, and an 8-bit
  ///     R component in bits 0..7.
  ///
  ///   - `VK_FORMAT_A8B8G8R8_SRGB_PACK32` specifies a four-component, 32-bit packed
  ///     unsigned normalized format that has an 8-bit A component in bits 24..31, an
  ///     8-bit B component stored with sRGB nonlinear encoding in bits 16..23, an
  ///     8-bit G component stored with sRGB nonlinear encoding in bits 8..15, and an
  ///     8-bit R component stored with sRGB nonlinear encoding in bits 0..7.
  ///
  ///   - `VK_FORMAT_A2R10G10B10_UNORM_PACK32` specifies a four-component, 32-bit
  ///     packed unsigned normalized format that has a 2-bit A component in bits
  ///     30..31, a 10-bit R component in bits 20..29, a 10-bit G component in bits
  ///     10..19, and a 10-bit B component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2R10G10B10_SNORM_PACK32` specifies a four-component, 32-bit
  ///     packed signed normalized format that has a 2-bit A component in bits 30..31,
  ///     a 10-bit R component in bits 20..29, a 10-bit G component in bits 10..19,
  ///     and a 10-bit B component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2R10G10B10_USCALED_PACK32` specifies a four-component, 32-bit
  ///     packed unsigned scaled integer format that has a 2-bit A component in bits
  ///     30..31, a 10-bit R component in bits 20..29, a 10-bit G component in bits
  ///     10..19, and a 10-bit B component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2R10G10B10_SSCALED_PACK32` specifies a four-component, 32-bit
  ///     packed signed scaled integer format that has a 2-bit A component in bits
  ///     30..31, a 10-bit R component in bits 20..29, a 10-bit G component in bits
  ///     10..19, and a 10-bit B component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2R10G10B10_UINT_PACK32` specifies a four-component, 32-bit
  ///     packed unsigned integer format that has a 2-bit A component in bits 30..31,
  ///     a 10-bit R component in bits 20..29, a 10-bit G component in bits 10..19,
  ///     and a 10-bit B component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2R10G10B10_SINT_PACK32` specifies a four-component, 32-bit
  ///     packed signed integer format that has a 2-bit A component in bits 30..31, a
  ///     10-bit R component in bits 20..29, a 10-bit G component in bits 10..19, and
  ///     a 10-bit B component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2B10G10R10_UNORM_PACK32` specifies a four-component, 32-bit
  ///     packed unsigned normalized format that has a 2-bit A component in bits
  ///     30..31, a 10-bit B component in bits 20..29, a 10-bit G component in bits
  ///     10..19, and a 10-bit R component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2B10G10R10_SNORM_PACK32` specifies a four-component, 32-bit
  ///     packed signed normalized format that has a 2-bit A component in bits 30..31,
  ///     a 10-bit B component in bits 20..29, a 10-bit G component in bits 10..19,
  ///     and a 10-bit R component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2B10G10R10_USCALED_PACK32` specifies a four-component, 32-bit
  ///     packed unsigned scaled integer format that has a 2-bit A component in bits
  ///     30..31, a 10-bit B component in bits 20..29, a 10-bit G component in bits
  ///     10..19, and a 10-bit R component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2B10G10R10_SSCALED_PACK32` specifies a four-component, 32-bit
  ///     packed signed scaled integer format that has a 2-bit A component in bits
  ///     30..31, a 10-bit B component in bits 20..29, a 10-bit G component in bits
  ///     10..19, and a 10-bit R component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2B10G10R10_UINT_PACK32` specifies a four-component, 32-bit
  ///     packed unsigned integer format that has a 2-bit A component in bits 30..31,
  ///     a 10-bit B component in bits 20..29, a 10-bit G component in bits 10..19,
  ///     and a 10-bit R component in bits 0..9.
  ///
  ///   - `VK_FORMAT_A2B10G10R10_SINT_PACK32` specifies a four-component, 32-bit
  ///     packed signed integer format that has a 2-bit A component in bits 30..31, a
  ///     10-bit B component in bits 20..29, a 10-bit G component in bits 10..19, and
  ///     a 10-bit R component in bits 0..9.
  ///
  ///   - `VK_FORMAT_R16_UNORM` specifies a one-component, 16-bit unsigned normalized
  ///     format that has a single 16-bit R component.
  ///
  ///   - `VK_FORMAT_R16_SNORM` specifies a one-component, 16-bit signed normalized
  ///     format that has a single 16-bit R component.
  ///
  ///   - `VK_FORMAT_R16_USCALED` specifies a one-component, 16-bit unsigned scaled
  ///     integer format that has a single 16-bit R component.
  ///
  ///   - `VK_FORMAT_R16_SSCALED` specifies a one-component, 16-bit signed scaled
  ///     integer format that has a single 16-bit R component.
  ///
  ///   - `VK_FORMAT_R16_UINT` specifies a one-component, 16-bit unsigned integer
  ///     format that has a single 16-bit R component.
  ///
  ///   - `VK_FORMAT_R16_SINT` specifies a one-component, 16-bit signed integer format
  ///     that has a single 16-bit R component.
  ///
  ///   - `VK_FORMAT_R16_SFLOAT` specifies a one-component, 16-bit signed
  ///     floating-point format that has a single 16-bit R component.
  ///
  ///   - `VK_FORMAT_R16G16_UNORM` specifies a two-component, 32-bit unsigned
  ///     normalized format that has a 16-bit R component in bytes 0..1, and a 16-bit
  ///     G component in bytes 2..3.
  ///
  ///   - `VK_FORMAT_R16G16_SNORM` specifies a two-component, 32-bit signed normalized
  ///     format that has a 16-bit R component in bytes 0..1, and a 16-bit G component
  ///     in bytes 2..3.
  ///
  ///   - `VK_FORMAT_R16G16_USCALED` specifies a two-component, 32-bit unsigned scaled
  ///     integer format that has a 16-bit R component in bytes 0..1, and a 16-bit G
  ///     component in bytes 2..3.
  ///
  ///   - `VK_FORMAT_R16G16_SSCALED` specifies a two-component, 32-bit signed scaled
  ///     integer format that has a 16-bit R component in bytes 0..1, and a 16-bit G
  ///     component in bytes 2..3.
  ///
  ///   - `VK_FORMAT_R16G16_UINT` specifies a two-component, 32-bit unsigned integer
  ///     format that has a 16-bit R component in bytes 0..1, and a 16-bit G component
  ///     in bytes 2..3.
  ///
  ///   - `VK_FORMAT_R16G16_SINT` specifies a two-component, 32-bit signed integer
  ///     format that has a 16-bit R component in bytes 0..1, and a 16-bit G component
  ///     in bytes 2..3.
  ///
  ///   - `VK_FORMAT_R16G16_SFLOAT` specifies a two-component, 32-bit signed
  ///     floating-point format that has a 16-bit R component in bytes 0..1, and a
  ///     16-bit G component in bytes 2..3.
  ///
  ///   - `VK_FORMAT_R16G16B16_UNORM` specifies a three-component, 48-bit unsigned
  ///     normalized format that has a 16-bit R component in bytes 0..1, a 16-bit G
  ///     component in bytes 2..3, and a 16-bit B component in bytes 4..5.
  ///
  ///   - `VK_FORMAT_R16G16B16_SNORM` specifies a three-component, 48-bit signed
  ///     normalized format that has a 16-bit R component in bytes 0..1, a 16-bit G
  ///     component in bytes 2..3, and a 16-bit B component in bytes 4..5.
  ///
  ///   - `VK_FORMAT_R16G16B16_USCALED` specifies a three-component, 48-bit unsigned
  ///     scaled integer format that has a 16-bit R component in bytes 0..1, a 16-bit
  ///     G component in bytes 2..3, and a 16-bit B component in bytes 4..5.
  ///
  ///   - `VK_FORMAT_R16G16B16_SSCALED` specifies a three-component, 48-bit signed
  ///     scaled integer format that has a 16-bit R component in bytes 0..1, a 16-bit
  ///     G component in bytes 2..3, and a 16-bit B component in bytes 4..5.
  ///
  ///   - `VK_FORMAT_R16G16B16_UINT` specifies a three-component, 48-bit unsigned
  ///     integer format that has a 16-bit R component in bytes 0..1, a 16-bit G
  ///     component in bytes 2..3, and a 16-bit B component in bytes 4..5.
  ///
  ///   - `VK_FORMAT_R16G16B16_SINT` specifies a three-component, 48-bit signed
  ///     integer format that has a 16-bit R component in bytes 0..1, a 16-bit G
  ///     component in bytes 2..3, and a 16-bit B component in bytes 4..5.
  ///
  ///   - `VK_FORMAT_R16G16B16_SFLOAT` specifies a three-component, 48-bit signed
  ///     floating-point format that has a 16-bit R component in bytes 0..1, a 16-bit
  ///     G component in bytes 2..3, and a 16-bit B component in bytes 4..5.
  ///
  ///   - `VK_FORMAT_R16G16B16A16_UNORM` specifies a four-component, 64-bit unsigned
  ///     normalized format that has a 16-bit R component in bytes 0..1, a 16-bit G
  ///     component in bytes 2..3, a 16-bit B component in bytes 4..5, and a 16-bit A
  ///     component in bytes 6..7.
  ///
  ///   - `VK_FORMAT_R16G16B16A16_SNORM` specifies a four-component, 64-bit signed
  ///     normalized format that has a 16-bit R component in bytes 0..1, a 16-bit G
  ///     component in bytes 2..3, a 16-bit B component in bytes 4..5, and a 16-bit A
  ///     component in bytes 6..7.
  ///
  ///   - `VK_FORMAT_R16G16B16A16_USCALED` specifies a four-component, 64-bit unsigned
  ///     scaled integer format that has a 16-bit R component in bytes 0..1, a 16-bit
  ///     G component in bytes 2..3, a 16-bit B component in bytes 4..5, and a 16-bit
  ///     A component in bytes 6..7.
  ///
  ///   - `VK_FORMAT_R16G16B16A16_SSCALED` specifies a four-component, 64-bit signed
  ///     scaled integer format that has a 16-bit R component in bytes 0..1, a 16-bit
  ///     G component in bytes 2..3, a 16-bit B component in bytes 4..5, and a 16-bit
  ///     A component in bytes 6..7.
  ///
  ///   - `VK_FORMAT_R16G16B16A16_UINT` specifies a four-component, 64-bit unsigned
  ///     integer format that has a 16-bit R component in bytes 0..1, a 16-bit G
  ///     component in bytes 2..3, a 16-bit B component in bytes 4..5, and a 16-bit A
  ///     component in bytes 6..7.
  ///
  ///   - `VK_FORMAT_R16G16B16A16_SINT` specifies a four-component, 64-bit signed
  ///     integer format that has a 16-bit R component in bytes 0..1, a 16-bit G
  ///     component in bytes 2..3, a 16-bit B component in bytes 4..5, and a 16-bit A
  ///     component in bytes 6..7.
  ///
  ///   - `VK_FORMAT_R16G16B16A16_SFLOAT` specifies a four-component, 64-bit signed
  ///     floating-point format that has a 16-bit R component in bytes 0..1, a 16-bit
  ///     G component in bytes 2..3, a 16-bit B component in bytes 4..5, and a 16-bit
  ///     A component in bytes 6..7.
  ///
  ///   - `VK_FORMAT_R32_UINT` specifies a one-component, 32-bit unsigned integer
  ///     format that has a single 32-bit R component.
  ///
  ///   - `VK_FORMAT_R32_SINT` specifies a one-component, 32-bit signed integer format
  ///     that has a single 32-bit R component.
  ///
  ///   - `VK_FORMAT_R32_SFLOAT` specifies a one-component, 32-bit signed
  ///     floating-point format that has a single 32-bit R component.
  ///
  ///   - `VK_FORMAT_R32G32_UINT` specifies a two-component, 64-bit unsigned integer
  ///     format that has a 32-bit R component in bytes 0..3, and a 32-bit G component
  ///     in bytes 4..7.
  ///
  ///   - `VK_FORMAT_R32G32_SINT` specifies a two-component, 64-bit signed integer
  ///     format that has a 32-bit R component in bytes 0..3, and a 32-bit G component
  ///     in bytes 4..7.
  ///
  ///   - `VK_FORMAT_R32G32_SFLOAT` specifies a two-component, 64-bit signed
  ///     floating-point format that has a 32-bit R component in bytes 0..3, and a
  ///     32-bit G component in bytes 4..7.
  ///
  ///   - `VK_FORMAT_R32G32B32_UINT` specifies a three-component, 96-bit unsigned
  ///     integer format that has a 32-bit R component in bytes 0..3, a 32-bit G
  ///     component in bytes 4..7, and a 32-bit B component in bytes 8..11.
  ///
  ///   - `VK_FORMAT_R32G32B32_SINT` specifies a three-component, 96-bit signed
  ///     integer format that has a 32-bit R component in bytes 0..3, a 32-bit G
  ///     component in bytes 4..7, and a 32-bit B component in bytes 8..11.
  ///
  ///   - `VK_FORMAT_R32G32B32_SFLOAT` specifies a three-component, 96-bit signed
  ///     floating-point format that has a 32-bit R component in bytes 0..3, a 32-bit
  ///     G component in bytes 4..7, and a 32-bit B component in bytes 8..11.
  ///
  ///   - `VK_FORMAT_R32G32B32A32_UINT` specifies a four-component, 128-bit unsigned
  ///     integer format that has a 32-bit R component in bytes 0..3, a 32-bit G
  ///     component in bytes 4..7, a 32-bit B component in bytes 8..11, and a 32-bit A
  ///     component in bytes 12..15.
  ///
  ///   - `VK_FORMAT_R32G32B32A32_SINT` specifies a four-component, 128-bit signed
  ///     integer format that has a 32-bit R component in bytes 0..3, a 32-bit G
  ///     component in bytes 4..7, a 32-bit B component in bytes 8..11, and a 32-bit A
  ///     component in bytes 12..15.
  ///
  ///   - `VK_FORMAT_R32G32B32A32_SFLOAT` specifies a four-component, 128-bit signed
  ///     floating-point format that has a 32-bit R component in bytes 0..3, a 32-bit
  ///     G component in bytes 4..7, a 32-bit B component in bytes 8..11, and a 32-bit
  ///     A component in bytes 12..15.
  ///
  ///   - `VK_FORMAT_R64_UINT` specifies a one-component, 64-bit unsigned integer
  ///     format that has a single 64-bit R component.
  ///
  ///   - `VK_FORMAT_R64_SINT` specifies a one-component, 64-bit signed integer format
  ///     that has a single 64-bit R component.
  ///
  ///   - `VK_FORMAT_R64_SFLOAT` specifies a one-component, 64-bit signed
  ///     floating-point format that has a single 64-bit R component.
  ///
  ///   - `VK_FORMAT_R64G64_UINT` specifies a two-component, 128-bit unsigned integer
  ///     format that has a 64-bit R component in bytes 0..7, and a 64-bit G component
  ///     in bytes 8..15.
  ///
  ///   - `VK_FORMAT_R64G64_SINT` specifies a two-component, 128-bit signed integer
  ///     format that has a 64-bit R component in bytes 0..7, and a 64-bit G component
  ///     in bytes 8..15.
  ///
  ///   - `VK_FORMAT_R64G64_SFLOAT` specifies a two-component, 128-bit signed
  ///     floating-point format that has a 64-bit R component in bytes 0..7, and a
  ///     64-bit G component in bytes 8..15.
  ///
  ///   - `VK_FORMAT_R64G64B64_UINT` specifies a three-component, 192-bit unsigned
  ///     integer format that has a 64-bit R component in bytes 0..7, a 64-bit G
  ///     component in bytes 8..15, and a 64-bit B component in bytes 16..23.
  ///
  ///   - `VK_FORMAT_R64G64B64_SINT` specifies a three-component, 192-bit signed
  ///     integer format that has a 64-bit R component in bytes 0..7, a 64-bit G
  ///     component in bytes 8..15, and a 64-bit B component in bytes 16..23.
  ///
  ///   - `VK_FORMAT_R64G64B64_SFLOAT` specifies a three-component, 192-bit signed
  ///     floating-point format that has a 64-bit R component in bytes 0..7, a 64-bit
  ///     G component in bytes 8..15, and a 64-bit B component in bytes 16..23.
  ///
  ///   - `VK_FORMAT_R64G64B64A64_UINT` specifies a four-component, 256-bit unsigned
  ///     integer format that has a 64-bit R component in bytes 0..7, a 64-bit G
  ///     component in bytes 8..15, a 64-bit B component in bytes 16..23, and a 64-bit
  ///     A component in bytes 24..31.
  ///
  ///   - `VK_FORMAT_R64G64B64A64_SINT` specifies a four-component, 256-bit signed
  ///     integer format that has a 64-bit R component in bytes 0..7, a 64-bit G
  ///     component in bytes 8..15, a 64-bit B component in bytes 16..23, and a 64-bit
  ///     A component in bytes 24..31.
  ///
  ///   - `VK_FORMAT_R64G64B64A64_SFLOAT` specifies a four-component, 256-bit signed
  ///     floating-point format that has a 64-bit R component in bytes 0..7, a 64-bit
  ///     G component in bytes 8..15, a 64-bit B component in bytes 16..23, and a
  ///     64-bit A component in bytes 24..31.
  ///
  ///   - `VK_FORMAT_B10G11R11_UFLOAT_PACK32` specifies a three-component, 32-bit
  ///     packed unsigned floating-point format that has a 10-bit B component in bits
  ///     22..31, an 11-bit G component in bits 11..21, an 11-bit R component in bits
  ///     0..10. See [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-fp10) and [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-fp11).
  ///
  ///   - `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32` specifies a three-component, 32-bit
  ///     packed unsigned floating-point format that has a 5-bit shared exponent in
  ///     bits 27..31, a 9-bit B component mantissa in bits 18..26, a 9-bit G
  ///     component mantissa in bits 9..17, and a 9-bit R component mantissa in bits
  ///     0..8.
  ///
  ///   - `VK_FORMAT_D16_UNORM` specifies a one-component, 16-bit unsigned normalized
  ///     format that has a single 16-bit depth component.
  ///
  ///   - `VK_FORMAT_X8_D24_UNORM_PACK32` specifies a two-component, 32-bit format
  ///     that has 24 unsigned normalized bits in the depth component and,
  ///     optionally:, 8 bits that are unused.
  ///
  ///   - `VK_FORMAT_D32_SFLOAT` specifies a one-component, 32-bit signed
  ///     floating-point format that has 32-bits in the depth component.
  ///
  ///   - `VK_FORMAT_S8_UINT` specifies a one-component, 8-bit unsigned integer format
  ///     that has 8-bits in the stencil component.
  ///
  ///   - `VK_FORMAT_D16_UNORM_S8_UINT` specifies a two-component, 24-bit format that
  ///     has 16 unsigned normalized bits in the depth component and 8 unsigned
  ///     integer bits in the stencil component.
  ///
  ///   - `VK_FORMAT_D24_UNORM_S8_UINT` specifies a two-component, 32-bit packed
  ///     format that has 8 unsigned integer bits in the stencil component, and 24
  ///     unsigned normalized bits in the depth component.
  ///
  ///   - `VK_FORMAT_D32_SFLOAT_S8_UINT` specifies a two-component format that has 32
  ///     signed float bits in the depth component and 8 unsigned integer bits in the
  ///     stencil component. There are optionally: 24-bits that are unused.
  ///
  ///   - `VK_FORMAT_BC1_RGB_UNORM_BLOCK` specifies a three-component,
  ///     block-compressed format where each 64-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RGB texel data. This format has
  ///     no alpha and is considered opaque.
  ///
  ///   - `VK_FORMAT_BC1_RGB_SRGB_BLOCK` specifies a three-component, block-compressed
  ///     format where each 64-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGB texel data with sRGB nonlinear
  ///     encoding. This format has no alpha and is considered opaque.
  ///
  ///   - `VK_FORMAT_BC1_RGBA_UNORM_BLOCK` specifies a four-component,
  ///     block-compressed format where each 64-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RGB texel data, and provides 1
  ///     bit of alpha.
  ///
  ///   - `VK_FORMAT_BC1_RGBA_SRGB_BLOCK` specifies a four-component, block-compressed
  ///     format where each 64-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGB texel data with sRGB nonlinear
  ///     encoding, and provides 1 bit of alpha.
  ///
  ///   - `VK_FORMAT_BC2_UNORM_BLOCK` specifies a four-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGBA texel data with the first 64 bits
  ///     encoding alpha values followed by 64 bits encoding RGB values.
  ///
  ///   - `VK_FORMAT_BC2_SRGB_BLOCK` specifies a four-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGBA texel data with the first 64 bits
  ///     encoding alpha values followed by 64 bits encoding RGB values with sRGB
  ///     nonlinear encoding.
  ///
  ///   - `VK_FORMAT_BC3_UNORM_BLOCK` specifies a four-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGBA texel data with the first 64 bits
  ///     encoding alpha values followed by 64 bits encoding RGB values.
  ///
  ///   - `VK_FORMAT_BC3_SRGB_BLOCK` specifies a four-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGBA texel data with the first 64 bits
  ///     encoding alpha values followed by 64 bits encoding RGB values with sRGB
  ///     nonlinear encoding.
  ///
  ///   - `VK_FORMAT_BC4_UNORM_BLOCK` specifies a one-component, block-compressed
  ///     format where each 64-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized red texel data.
  ///
  ///   - `VK_FORMAT_BC4_SNORM_BLOCK` specifies a one-component, block-compressed
  ///     format where each 64-bit compressed texel block encodes a 4{times}4
  ///     rectangle of signed normalized red texel data.
  ///
  ///   - `VK_FORMAT_BC5_UNORM_BLOCK` specifies a two-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RG texel data with the first 64 bits
  ///     encoding red values followed by 64 bits encoding green values.
  ///
  ///   - `VK_FORMAT_BC5_SNORM_BLOCK` specifies a two-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of signed normalized RG texel data with the first 64 bits encoding
  ///     red values followed by 64 bits encoding green values.
  ///
  ///   - `VK_FORMAT_BC6H_UFLOAT_BLOCK` specifies a three-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned floating-point RGB texel data.
  ///
  ///   - `VK_FORMAT_BC6H_SFLOAT_BLOCK` specifies a three-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of signed floating-point RGB texel data.
  ///
  ///   - `VK_FORMAT_BC7_UNORM_BLOCK` specifies a four-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_BC7_SRGB_BLOCK` specifies a four-component, block-compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK` specifies a three-component, ETC2
  ///     compressed format where each 64-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RGB texel data. This format has
  ///     no alpha and is considered opaque.
  ///
  ///   - `VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK` specifies a three-component, ETC2
  ///     compressed format where each 64-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RGB texel data with sRGB
  ///     nonlinear encoding. This format has no alpha and is considered opaque.
  ///
  ///   - `VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK` specifies a four-component, ETC2
  ///     compressed format where each 64-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RGB texel data, and provides 1
  ///     bit of alpha.
  ///
  ///   - `VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK` specifies a four-component, ETC2
  ///     compressed format where each 64-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RGB texel data with sRGB
  ///     nonlinear encoding, and provides 1 bit of alpha.
  ///
  ///   - `VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK` specifies a four-component, ETC2
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RGBA texel data with the first 64
  ///     bits encoding alpha values followed by 64 bits encoding RGB values.
  ///
  ///   - `VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK` specifies a four-component, ETC2
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RGBA texel data with the first 64
  ///     bits encoding alpha values followed by 64 bits encoding RGB values with sRGB
  ///     nonlinear encoding applied.
  ///
  ///   - `VK_FORMAT_EAC_R11_UNORM_BLOCK` specifies a one-component, ETC2 compressed
  ///     format where each 64-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized red texel data.
  ///
  ///   - `VK_FORMAT_EAC_R11_SNORM_BLOCK` specifies a one-component, ETC2 compressed
  ///     format where each 64-bit compressed texel block encodes a 4{times}4
  ///     rectangle of signed normalized red texel data.
  ///
  ///   - `VK_FORMAT_EAC_R11G11_UNORM_BLOCK` specifies a two-component, ETC2
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     4{times}4 rectangle of unsigned normalized RG texel data with the first 64
  ///     bits encoding red values followed by 64 bits encoding green values.
  ///
  ///   - `VK_FORMAT_EAC_R11G11_SNORM_BLOCK` specifies a two-component, ETC2
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     4{times}4 rectangle of signed normalized RG texel data with the first 64
  ///     bits encoding red values followed by 64 bits encoding green values.
  ///
  ///   - `VK_FORMAT_ASTC_4x4_UNORM_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_4x4_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 4{times}4
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_5x4_UNORM_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 5{times}4
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_5x4_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 5{times}4
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_5x5_UNORM_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 5{times}5
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_5x5_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 5{times}5
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_6x5_UNORM_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 6{times}5
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_6x5_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 6{times}5
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_6x6_UNORM_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 6{times}6
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_6x6_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 6{times}6
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_8x5_UNORM_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes an 8{times}5
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_8x5_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes an 8{times}5
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_8x6_UNORM_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes an 8{times}6
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_8x6_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes an 8{times}6
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_8x8_UNORM_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes an 8{times}8
  ///     rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_8x8_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes an 8{times}8
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_10x5_UNORM_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     10{times}5 rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_10x5_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 10{times}5
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_10x6_UNORM_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     10{times}6 rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_10x6_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 10{times}6
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_10x8_UNORM_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     10{times}8 rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_10x8_SRGB_BLOCK` specifies a four-component, ASTC compressed
  ///     format where each 128-bit compressed texel block encodes a 10{times}8
  ///     rectangle of unsigned normalized RGBA texel data with sRGB nonlinear
  ///     encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_10x10_UNORM_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     10{times}10 rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_10x10_SRGB_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     10{times}10 rectangle of unsigned normalized RGBA texel data with sRGB
  ///     nonlinear encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_12x10_UNORM_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     12{times}10 rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_12x10_SRGB_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     12{times}10 rectangle of unsigned normalized RGBA texel data with sRGB
  ///     nonlinear encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_ASTC_12x12_UNORM_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     12{times}12 rectangle of unsigned normalized RGBA texel data.
  ///
  ///   - `VK_FORMAT_ASTC_12x12_SRGB_BLOCK` specifies a four-component, ASTC
  ///     compressed format where each 128-bit compressed texel block encodes a
  ///     12{times}12 rectangle of unsigned normalized RGBA texel data with sRGB
  ///     nonlinear encoding applied to the RGB components.
  ///
  ///   - `VK_FORMAT_G8B8G8R8_422_UNORM_KHR` specifies a four-component, 32-bit format
  ///     containing a pair of G components, an R component, and a B component,
  ///     collectively encoding a 2{times}1 rectangle of unsigned normalized RGB texel
  ///     data. One G value is present at each *i* coordinate, with the B and R values
  ///     shared across both G values and thus recorded at half the horizontal
  ///     resolution of the image. This format has an 8-bit G component for the even
  ///     *i* coordinate in byte 0, an 8-bit B component in byte 1, an 8-bit G
  ///     component for the odd *i* coordinate in byte 2, and an 8-bit R component in
  ///     byte 3. Images in this format must: be defined with a width that is a
  ///     multiple of two. For the purposes of the constraints on copy extents, this
  ///     format is treated as a compressed format with a 2{times}1 compressed texel
  ///     block.
  ///
  ///   - `VK_FORMAT_B8G8R8G8_422_UNORM_KHR` specifies a four-component, 32-bit format
  ///     containing a pair of G components, an R component, and a B component,
  ///     collectively encoding a 2{times}1 rectangle of unsigned normalized RGB texel
  ///     data. One G value is present at each *i* coordinate, with the B and R values
  ///     shared across both G values and thus recorded at half the horizontal
  ///     resolution of the image. This format has an 8-bit B component in byte 0, an
  ///     8-bit G component for the even *i* coordinate in byte 1, an 8-bit R
  ///     component in byte 2, and an 8-bit G component for the odd *i* coordinate in
  ///     byte 3. Images in this format must: be defined with a width that is a
  ///     multiple of two. For the purposes of the constraints on copy extents, this
  ///     format is treated as a compressed format with a 2{times}1 compressed texel
  ///     block.
  ///
  ///   - `VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has an 8-bit G component in plane 0, an 8-bit B
  ///     component in plane 1, and an 8-bit R component in plane 2. The horizontal
  ///     and vertical dimensions of the R and B planes are halved relative to the
  ///     image dimensions, and each R and B component is shared with the G components
  ///     for which and . The location of each plane when this image is in linear
  ///     layout can be determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane,
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane. Images in this format
  ///     must: be defined with a width and height that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has an 8-bit G component in plane 0, and a
  ///     two-component, 16-bit BR plane 1 consisting of an 8-bit B component in byte
  ///     0 and an 8-bit R component in byte 1. The horizontal and vertical dimensions
  ///     of the BR plane is halved relative to the image dimensions, and each R and B
  ///     value is shared with the G components for which and . The location of each
  ///     plane when this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, and `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the BR plane. Images
  ///     in this format must: be defined with a width and height that is a multiple
  ///     of two.
  ///
  ///   - `VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has an 8-bit G component in plane 0, an 8-bit B
  ///     component in plane 1, and an 8-bit R component in plane 2. The horizontal
  ///     dimension of the R and B plane is halved relative to the image dimensions,
  ///     and each R and B value is shared with the G components for which . The
  ///     location of each plane when this image is in linear layout can be determined
  ///     via `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`
  ///     for the G plane, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane. Images in this format
  ///     must: be defined with a width that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has an 8-bit G component in plane 0, and a
  ///     two-component, 16-bit BR plane 1 consisting of an 8-bit B component in byte
  ///     0 and an 8-bit R component in byte 1. The horizontal dimensions of the BR
  ///     plane is halved relative to the image dimensions, and each R and B value is
  ///     shared with the G components for which . The location of each plane when
  ///     this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, and `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the BR plane. Images
  ///     in this format must: be defined with a width that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has an 8-bit G component in plane 0, an 8-bit B
  ///     component in plane 1, and an 8-bit R component in plane 2. Each plane has
  ///     the same dimensions and each R, G and B component contributes to a single
  ///     texel. The location of each plane when this image is in linear layout can be
  ///     determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane,
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane.
  ///
  ///   - `VK_FORMAT_R10X6_UNORM_PACK16_KHR` specifies a one-component, 16-bit
  ///     unsigned normalized format that has a single 10-bit R component in the top
  ///     10 bits of a 16-bit word, with the bottom 6 bits set to 0.
  ///
  ///   - `VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR` specifies a two-component, 32-bit
  ///     unsigned normalized format that has a 10-bit R component in the top 10 bits
  ///     of the word in bytes 0..1, and a 10-bit G component in the top 10 bits of
  ///     the word in bytes 2..3, with the bottom 6 bits of each word set to 0.
  ///
  ///   - `VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR` specifies a
  ///     four-component, 64-bit unsigned normalized format that has a 10-bit R
  ///     component in the top 10 bits of the word in bytes 0..1, a 10-bit G component
  ///     in the top 10 bits of the word in bytes 2..3, a 10-bit B component in the
  ///     top 10 bits of the word in bytes 4..5, and a 10-bit A component in the top
  ///     10 bits of the word in bytes 6..7, with the bottom 6 bits of each word set
  ///     to 0.
  ///
  ///   - `VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR` specifies a
  ///     four-component, 64-bit format containing a pair of G components, an R
  ///     component, and a B component, collectively encoding a 2{times}1 rectangle of
  ///     unsigned normalized RGB texel data. One G value is present at each *i*
  ///     coordinate, with the B and R values shared across both G values and thus
  ///     recorded at half the horizontal resolution of the image. This format has a
  ///     10-bit G component for the even *i* coordinate in the top 10 bits of the
  ///     word in bytes 0..1, a 10-bit B component in the top 10 bits of the word in
  ///     bytes 2..3, a 10-bit G component for the odd *i* coordinate in the top 10
  ///     bits of the word in bytes 4..5, and a 10-bit R component in the top 10 bits
  ///     of the word in bytes 6..7, with the bottom 6 bits of each word set to 0.
  ///     Images in this format must: be defined with a width that is a multiple of
  ///     two. For the purposes of the constraints on copy extents, this format is
  ///     treated as a compressed format with a 2{times}1 compressed texel block.
  ///
  ///   - `VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR` specifies a
  ///     four-component, 64-bit format containing a pair of G components, an R
  ///     component, and a B component, collectively encoding a 2{times}1 rectangle of
  ///     unsigned normalized RGB texel data. One G value is present at each *i*
  ///     coordinate, with the B and R values shared across both G values and thus
  ///     recorded at half the horizontal resolution of the image. This format has a
  ///     10-bit B component in the top 10 bits of the word in bytes 0..1, a 10-bit G
  ///     component for the even *i* coordinate in the top 10 bits of the word in
  ///     bytes 2..3, a 10-bit R component in the top 10 bits of the word in bytes
  ///     4..5, and a 10-bit G component for the odd *i* coordinate in the top 10 bits
  ///     of the word in bytes 6..7, with the bottom 6 bits of each word set to 0.
  ///     Images in this format must: be defined with a width that is a multiple of
  ///     two. For the purposes of the constraints on copy extents, this format is
  ///     treated as a compressed format with a 2{times}1 compressed texel block.
  ///
  ///   - `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 10-bit G component in
  ///     the top 10 bits of each 16-bit word of plane 0, a 10-bit B component in the
  ///     top 10 bits of each 16-bit word of plane 1, and a 10-bit R component in the
  ///     top 10 bits of each 16-bit word of plane 2, with the bottom 6 bits of each
  ///     word set to 0. The horizontal and vertical dimensions of the R and B planes
  ///     are halved relative to the image dimensions, and each R and B component is
  ///     shared with the G components for which and . The location of each plane when
  ///     this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane. Images in this format
  ///     must: be defined with a width and height that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 10-bit G component in
  ///     the top 10 bits of each 16-bit word of plane 0, and a two-component, 32-bit
  ///     BR plane 1 consisting of a 10-bit B component in the top 10 bits of the word
  ///     in bytes 0..1, and a 10-bit R component in the top 10 bits of the word in
  ///     bytes 2..3, the bottom 6 bits of each word set to 0. The horizontal and
  ///     vertical dimensions of the BR plane is halved relative to the image
  ///     dimensions, and each R and B value is shared with the G components for which
  ///     and . The location of each plane when this image is in linear layout can be
  ///     determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the BR plane. Images in this format
  ///     must: be defined with a width and height that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 10-bit G component in
  ///     the top 10 bits of each 16-bit word of plane 0, a 10-bit B component in the
  ///     top 10 bits of each 16-bit word of plane 1, and a 10-bit R component in the
  ///     top 10 bits of each 16-bit word of plane 2, with the bottom 6 bits of each
  ///     word set to 0. The horizontal dimension of the R and B plane is halved
  ///     relative to the image dimensions, and each R and B value is shared with the
  ///     G components for which . The location of each plane when this image is in
  ///     linear layout can be determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane,
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane. Images in this format
  ///     must: be defined with a width that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 10-bit G component in
  ///     the top 10 bits of each 16-bit word of plane 0, and a two-component, 32-bit
  ///     BR plane 1 consisting of a 10-bit B component in the top 10 bits of the word
  ///     in bytes 0..1, and a 10-bit R component in the top 10 bits of the word in
  ///     bytes 2..3, the bottom 6 bits of each word set to 0. The horizontal
  ///     dimensions of the BR plane is halved relative to the image dimensions, and
  ///     each R and B value is shared with the G components for which . The location
  ///     of each plane when this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, and `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the BR plane. Images
  ///     in this format must: be defined with a width that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 10-bit G component in
  ///     the top 10 bits of each 16-bit word of plane 0, a 10-bit B component in the
  ///     top 10 bits of each 16-bit word of plane 1, and a 10-bit R component in the
  ///     top 10 bits of each 16-bit word of plane 2, with the bottom 6 bits of each
  ///     word set to 0. Each plane has the same dimensions and each R, G and B
  ///     component contributes to a single texel. The location of each plane when
  ///     this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane.
  ///
  ///   - `VK_FORMAT_R12X4_UNORM_PACK16_KHR` specifies a one-component, 16-bit
  ///     unsigned normalized format that has a single 12-bit R component in the top
  ///     12 bits of a 16-bit word, with the bottom 4 bits set to 0.
  ///
  ///   - `VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR` specifies a two-component, 32-bit
  ///     unsigned normalized format that has a 12-bit R component in the top 12 bits
  ///     of the word in bytes 0..1, and a 12-bit G component in the top 12 bits of
  ///     the word in bytes 2..3, with the bottom 4 bits of each word set to 0.
  ///
  ///   - `VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR` specifies a
  ///     four-component, 64-bit unsigned normalized format that has a 12-bit R
  ///     component in the top 12 bits of the word in bytes 0..1, a 12-bit G component
  ///     in the top 12 bits of the word in bytes 2..3, a 12-bit B component in the
  ///     top 12 bits of the word in bytes 4..5, and a 12-bit A component in the top
  ///     12 bits of the word in bytes 6..7, with the bottom 4 bits of each word set
  ///     to 0.
  ///
  ///   - `VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR` specifies a
  ///     four-component, 64-bit format containing a pair of G components, an R
  ///     component, and a B component, collectively encoding a 2{times}1 rectangle of
  ///     unsigned normalized RGB texel data. One G value is present at each *i*
  ///     coordinate, with the B and R values shared across both G values and thus
  ///     recorded at half the horizontal resolution of the image. This format has a
  ///     12-bit G component for the even *i* coordinate in the top 12 bits of the
  ///     word in bytes 0..1, a 12-bit B component in the top 12 bits of the word in
  ///     bytes 2..3, a 12-bit G component for the odd *i* coordinate in the top 12
  ///     bits of the word in bytes 4..5, and a 12-bit R component in the top 12 bits
  ///     of the word in bytes 6..7, with the bottom 4 bits of each word set to 0.
  ///     Images in this format must: be defined with a width that is a multiple of
  ///     two. For the purposes of the constraints on copy extents, this format is
  ///     treated as a compressed format with a 2{times}1 compressed texel block.
  ///
  ///   - `VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR` specifies a
  ///     four-component, 64-bit format containing a pair of G components, an R
  ///     component, and a B component, collectively encoding a 2{times}1 rectangle of
  ///     unsigned normalized RGB texel data. One G value is present at each *i*
  ///     coordinate, with the B and R values shared across both G values and thus
  ///     recorded at half the horizontal resolution of the image. This format has a
  ///     12-bit B component in the top 12 bits of the word in bytes 0..1, a 12-bit G
  ///     component for the even *i* coordinate in the top 12 bits of the word in
  ///     bytes 2..3, a 12-bit R component in the top 12 bits of the word in bytes
  ///     4..5, and a 12-bit G component for the odd *i* coordinate in the top 12 bits
  ///     of the word in bytes 6..7, with the bottom 4 bits of each word set to 0.
  ///     Images in this format must: be defined with a width that is a multiple of
  ///     two. For the purposes of the constraints on copy extents, this format is
  ///     treated as a compressed format with a 2{times}1 compressed texel block.
  ///
  ///   - `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 12-bit G component in
  ///     the top 12 bits of each 16-bit word of plane 0, a 12-bit B component in the
  ///     top 12 bits of each 16-bit word of plane 1, and a 12-bit R component in the
  ///     top 12 bits of each 16-bit word of plane 2, with the bottom 4 bits of each
  ///     word set to 0. The horizontal and vertical dimensions of the R and B planes
  ///     are halved relative to the image dimensions, and each R and B component is
  ///     shared with the G components for which and . The location of each plane when
  ///     this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane. Images in this format
  ///     must: be defined with a width and height that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 12-bit G component in
  ///     the top 12 bits of each 16-bit word of plane 0, and a two-component, 32-bit
  ///     BR plane 1 consisting of a 12-bit B component in the top 12 bits of the word
  ///     in bytes 0..1, and a 12-bit R component in the top 12 bits of the word in
  ///     bytes 2..3, the bottom 4 bits of each word set to 0. The horizontal and
  ///     vertical dimensions of the BR plane is halved relative to the image
  ///     dimensions, and each R and B value is shared with the G components for which
  ///     and . The location of each plane when this image is in linear layout can be
  ///     determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the BR plane. Images in this format
  ///     must: be defined with a width and height that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 12-bit G component in
  ///     the top 12 bits of each 16-bit word of plane 0, a 12-bit B component in the
  ///     top 12 bits of each 16-bit word of plane 1, and a 12-bit R component in the
  ///     top 12 bits of each 16-bit word of plane 2, with the bottom 4 bits of each
  ///     word set to 0. The horizontal dimension of the R and B plane is halved
  ///     relative to the image dimensions, and each R and B value is shared with the
  ///     G components for which . The location of each plane when this image is in
  ///     linear layout can be determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane,
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane. Images in this format
  ///     must: be defined with a width that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 12-bit G component in
  ///     the top 12 bits of each 16-bit word of plane 0, and a two-component, 32-bit
  ///     BR plane 1 consisting of a 12-bit B component in the top 12 bits of the word
  ///     in bytes 0..1, and a 12-bit R component in the top 12 bits of the word in
  ///     bytes 2..3, the bottom 4 bits of each word set to 0. The horizontal
  ///     dimensions of the BR plane is halved relative to the image dimensions, and
  ///     each R and B value is shared with the G components for which . The location
  ///     of each plane when this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, and `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the BR plane. Images
  ///     in this format must: be defined with a width that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR` specifies a
  ///     unsigned normalized *multi-planar format* that has a 12-bit G component in
  ///     the top 12 bits of each 16-bit word of plane 0, a 12-bit B component in the
  ///     top 12 bits of each 16-bit word of plane 1, and a 12-bit R component in the
  ///     top 12 bits of each 16-bit word of plane 2, with the bottom 4 bits of each
  ///     word set to 0. Each plane has the same dimensions and each R, G and B
  ///     component contributes to a single texel. The location of each plane when
  ///     this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane.
  ///
  ///   - `VK_FORMAT_G16B16G16R16_422_UNORM_KHR` specifies a four-component, 64-bit
  ///     format containing a pair of G components, an R component, and a B component,
  ///     collectively encoding a 2{times}1 rectangle of unsigned normalized RGB texel
  ///     data. One G value is present at each *i* coordinate, with the B and R values
  ///     shared across both G values and thus recorded at half the horizontal
  ///     resolution of the image. This format has a 16-bit G component for the even
  ///     *i* coordinate in the word in bytes 0..1, a 16-bit B component in the word
  ///     in bytes 2..3, a 16-bit G component for the odd *i* coordinate in the word
  ///     in bytes 4..5, and a 16-bit R component in the word in bytes 6..7. Images in
  ///     this format must: be defined with a width that is a multiple of two. For the
  ///     purposes of the constraints on copy extents, this format is treated as a
  ///     compressed format with a 2{times}1 compressed texel block.
  ///
  ///   - `VK_FORMAT_B16G16R16G16_422_UNORM_KHR` specifies a four-component, 64-bit
  ///     format containing a pair of G components, an R component, and a B component,
  ///     collectively encoding a 2{times}1 rectangle of unsigned normalized RGB texel
  ///     data. One G value is present at each *i* coordinate, with the B and R values
  ///     shared across both G values and thus recorded at half the horizontal
  ///     resolution of the image. This format has a 16-bit B component in the word in
  ///     bytes 0..1, a 16-bit G component for the even *i* coordinate in the word in
  ///     bytes 2..3, a 16-bit R component in the word in bytes 4..5, and a 16-bit G
  ///     component for the odd *i* coordinate in the word in bytes 6..7. Images in
  ///     this format must: be defined with a width that is a multiple of two. For the
  ///     purposes of the constraints on copy extents, this format is treated as a
  ///     compressed format with a 2{times}1 compressed texel block.
  ///
  ///   - `VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has a 16-bit G component in each 16-bit word of
  ///     plane 0, a 16-bit B component in each 16-bit word of plane 1, and a 16-bit R
  ///     component in each 16-bit word of plane 2. The horizontal and vertical
  ///     dimensions of the R and B planes are halved relative to the image
  ///     dimensions, and each R and B component is shared with the G components for
  ///     which and . The location of each plane when this image is in linear layout
  ///     can be determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane,
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane. Images in this format
  ///     must: be defined with a width and height that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has a 16-bit G component in each 16-bit word of
  ///     plane 0, and a two-component, 32-bit BR plane 1 consisting of a 16-bit B
  ///     component in the word in bytes 0..1, and a 16-bit R component in the word in
  ///     bytes 2..3. The horizontal and vertical dimensions of the BR plane is halved
  ///     relative to the image dimensions, and each R and B value is shared with the
  ///     G components for which and . The location of each plane when this image is
  ///     in linear layout can be determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the BR plane. Images in this format
  ///     must: be defined with a width and height that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has a 16-bit G component in each 16-bit word of
  ///     plane 0, a 16-bit B component in each 16-bit word of plane 1, and a 16-bit R
  ///     component in each 16-bit word of plane 2. The horizontal dimension of the R
  ///     and B plane is halved relative to the image dimensions, and each R and B
  ///     value is shared with the G components for which . The location of each plane
  ///     when this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane. Images in this format
  ///     must: be defined with a width that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has a 16-bit G component in each 16-bit word of
  ///     plane 0, and a two-component, 32-bit BR plane 1 consisting of a 16-bit B
  ///     component in the word in bytes 0..1, and a 16-bit R component in the word in
  ///     bytes 2..3. The horizontal dimensions of the BR plane is halved relative to
  ///     the image dimensions, and each R and B value is shared with the G components
  ///     for which . The location of each plane when this image is in linear layout
  ///     can be determined via `vkGetImageSubresourceLayout`, using
  ///     `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for the G plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the BR plane. Images in this format
  ///     must: be defined with a width that is a multiple of two.
  ///
  ///   - `VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR` specifies a unsigned normalized
  ///     *multi-planar format* that has a 16-bit G component in each 16-bit word of
  ///     plane 0, a 16-bit B component in each 16-bit word of plane 1, and a 16-bit R
  ///     component in each 16-bit word of plane 2. Each plane has the same dimensions
  ///     and each R, G and B component contributes to a single texel. The location of
  ///     each plane when this image is in linear layout can be determined via
  ///     `vkGetImageSubresourceLayout`, using `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR` for
  ///     the G plane, `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR` for the B plane, and
  ///     `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR` for the R plane.
  ///
  pub enum VkFormat {
    E_UNDEFINED = 0,
    E_R4G4_UNORM_PACK8 = 1,
    E_R4G4B4A4_UNORM_PACK16 = 2,
    E_B4G4R4A4_UNORM_PACK16 = 3,
    E_R5G6B5_UNORM_PACK16 = 4,
    E_B5G6R5_UNORM_PACK16 = 5,
    E_R5G5B5A1_UNORM_PACK16 = 6,
    E_B5G5R5A1_UNORM_PACK16 = 7,
    E_A1R5G5B5_UNORM_PACK16 = 8,
    E_R8_UNORM = 9,
    E_R8_SNORM = 10,
    E_R8_USCALED = 11,
    E_R8_SSCALED = 12,
    E_R8_UINT = 13,
    E_R8_SINT = 14,
    E_R8_SRGB = 15,
    E_R8G8_UNORM = 16,
    E_R8G8_SNORM = 17,
    E_R8G8_USCALED = 18,
    E_R8G8_SSCALED = 19,
    E_R8G8_UINT = 20,
    E_R8G8_SINT = 21,
    E_R8G8_SRGB = 22,
    E_R8G8B8_UNORM = 23,
    E_R8G8B8_SNORM = 24,
    E_R8G8B8_USCALED = 25,
    E_R8G8B8_SSCALED = 26,
    E_R8G8B8_UINT = 27,
    E_R8G8B8_SINT = 28,
    E_R8G8B8_SRGB = 29,
    E_B8G8R8_UNORM = 30,
    E_B8G8R8_SNORM = 31,
    E_B8G8R8_USCALED = 32,
    E_B8G8R8_SSCALED = 33,
    E_B8G8R8_UINT = 34,
    E_B8G8R8_SINT = 35,
    E_B8G8R8_SRGB = 36,
    E_R8G8B8A8_UNORM = 37,
    E_R8G8B8A8_SNORM = 38,
    E_R8G8B8A8_USCALED = 39,
    E_R8G8B8A8_SSCALED = 40,
    E_R8G8B8A8_UINT = 41,
    E_R8G8B8A8_SINT = 42,
    E_R8G8B8A8_SRGB = 43,
    E_B8G8R8A8_UNORM = 44,
    E_B8G8R8A8_SNORM = 45,
    E_B8G8R8A8_USCALED = 46,
    E_B8G8R8A8_SSCALED = 47,
    E_B8G8R8A8_UINT = 48,
    E_B8G8R8A8_SINT = 49,
    E_B8G8R8A8_SRGB = 50,
    E_A8B8G8R8_UNORM_PACK32 = 51,
    E_A8B8G8R8_SNORM_PACK32 = 52,
    E_A8B8G8R8_USCALED_PACK32 = 53,
    E_A8B8G8R8_SSCALED_PACK32 = 54,
    E_A8B8G8R8_UINT_PACK32 = 55,
    E_A8B8G8R8_SINT_PACK32 = 56,
    E_A8B8G8R8_SRGB_PACK32 = 57,
    E_A2R10G10B10_UNORM_PACK32 = 58,
    E_A2R10G10B10_SNORM_PACK32 = 59,
    E_A2R10G10B10_USCALED_PACK32 = 60,
    E_A2R10G10B10_SSCALED_PACK32 = 61,
    E_A2R10G10B10_UINT_PACK32 = 62,
    E_A2R10G10B10_SINT_PACK32 = 63,
    E_A2B10G10R10_UNORM_PACK32 = 64,
    E_A2B10G10R10_SNORM_PACK32 = 65,
    E_A2B10G10R10_USCALED_PACK32 = 66,
    E_A2B10G10R10_SSCALED_PACK32 = 67,
    E_A2B10G10R10_UINT_PACK32 = 68,
    E_A2B10G10R10_SINT_PACK32 = 69,
    E_R16_UNORM = 70,
    E_R16_SNORM = 71,
    E_R16_USCALED = 72,
    E_R16_SSCALED = 73,
    E_R16_UINT = 74,
    E_R16_SINT = 75,
    E_R16_SFLOAT = 76,
    E_R16G16_UNORM = 77,
    E_R16G16_SNORM = 78,
    E_R16G16_USCALED = 79,
    E_R16G16_SSCALED = 80,
    E_R16G16_UINT = 81,
    E_R16G16_SINT = 82,
    E_R16G16_SFLOAT = 83,
    E_R16G16B16_UNORM = 84,
    E_R16G16B16_SNORM = 85,
    E_R16G16B16_USCALED = 86,
    E_R16G16B16_SSCALED = 87,
    E_R16G16B16_UINT = 88,
    E_R16G16B16_SINT = 89,
    E_R16G16B16_SFLOAT = 90,
    E_R16G16B16A16_UNORM = 91,
    E_R16G16B16A16_SNORM = 92,
    E_R16G16B16A16_USCALED = 93,
    E_R16G16B16A16_SSCALED = 94,
    E_R16G16B16A16_UINT = 95,
    E_R16G16B16A16_SINT = 96,
    E_R16G16B16A16_SFLOAT = 97,
    E_R32_UINT = 98,
    E_R32_SINT = 99,
    E_R32_SFLOAT = 100,
    E_R32G32_UINT = 101,
    E_R32G32_SINT = 102,
    E_R32G32_SFLOAT = 103,
    E_R32G32B32_UINT = 104,
    E_R32G32B32_SINT = 105,
    E_R32G32B32_SFLOAT = 106,
    E_R32G32B32A32_UINT = 107,
    E_R32G32B32A32_SINT = 108,
    E_R32G32B32A32_SFLOAT = 109,
    E_R64_UINT = 110,
    E_R64_SINT = 111,
    E_R64_SFLOAT = 112,
    E_R64G64_UINT = 113,
    E_R64G64_SINT = 114,
    E_R64G64_SFLOAT = 115,
    E_R64G64B64_UINT = 116,
    E_R64G64B64_SINT = 117,
    E_R64G64B64_SFLOAT = 118,
    E_R64G64B64A64_UINT = 119,
    E_R64G64B64A64_SINT = 120,
    E_R64G64B64A64_SFLOAT = 121,
    E_B10G11R11_UFLOAT_PACK32 = 122,
    E_E5B9G9R9_UFLOAT_PACK32 = 123,
    E_D16_UNORM = 124,
    E_X8_D24_UNORM_PACK32 = 125,
    E_D32_SFLOAT = 126,
    E_S8_UINT = 127,
    E_D16_UNORM_S8_UINT = 128,
    E_D24_UNORM_S8_UINT = 129,
    E_D32_SFLOAT_S8_UINT = 130,
    E_BC1_RGB_UNORM_BLOCK = 131,
    E_BC1_RGB_SRGB_BLOCK = 132,
    E_BC1_RGBA_UNORM_BLOCK = 133,
    E_BC1_RGBA_SRGB_BLOCK = 134,
    E_BC2_UNORM_BLOCK = 135,
    E_BC2_SRGB_BLOCK = 136,
    E_BC3_UNORM_BLOCK = 137,
    E_BC3_SRGB_BLOCK = 138,
    E_BC4_UNORM_BLOCK = 139,
    E_BC4_SNORM_BLOCK = 140,
    E_BC5_UNORM_BLOCK = 141,
    E_BC5_SNORM_BLOCK = 142,
    E_BC6H_UFLOAT_BLOCK = 143,
    E_BC6H_SFLOAT_BLOCK = 144,
    E_BC7_UNORM_BLOCK = 145,
    E_BC7_SRGB_BLOCK = 146,
    E_ETC2_R8G8B8_UNORM_BLOCK = 147,
    E_ETC2_R8G8B8_SRGB_BLOCK = 148,
    E_ETC2_R8G8B8A1_UNORM_BLOCK = 149,
    E_ETC2_R8G8B8A1_SRGB_BLOCK = 150,
    E_ETC2_R8G8B8A8_UNORM_BLOCK = 151,
    E_ETC2_R8G8B8A8_SRGB_BLOCK = 152,
    E_EAC_R11_UNORM_BLOCK = 153,
    E_EAC_R11_SNORM_BLOCK = 154,
    E_EAC_R11G11_UNORM_BLOCK = 155,
    E_EAC_R11G11_SNORM_BLOCK = 156,
    E_ASTC_4x4_UNORM_BLOCK = 157,
    E_ASTC_4x4_SRGB_BLOCK = 158,
    E_ASTC_5x4_UNORM_BLOCK = 159,
    E_ASTC_5x4_SRGB_BLOCK = 160,
    E_ASTC_5x5_UNORM_BLOCK = 161,
    E_ASTC_5x5_SRGB_BLOCK = 162,
    E_ASTC_6x5_UNORM_BLOCK = 163,
    E_ASTC_6x5_SRGB_BLOCK = 164,
    E_ASTC_6x6_UNORM_BLOCK = 165,
    E_ASTC_6x6_SRGB_BLOCK = 166,
    E_ASTC_8x5_UNORM_BLOCK = 167,
    E_ASTC_8x5_SRGB_BLOCK = 168,
    E_ASTC_8x6_UNORM_BLOCK = 169,
    E_ASTC_8x6_SRGB_BLOCK = 170,
    E_ASTC_8x8_UNORM_BLOCK = 171,
    E_ASTC_8x8_SRGB_BLOCK = 172,
    E_ASTC_10x5_UNORM_BLOCK = 173,
    E_ASTC_10x5_SRGB_BLOCK = 174,
    E_ASTC_10x6_UNORM_BLOCK = 175,
    E_ASTC_10x6_SRGB_BLOCK = 176,
    E_ASTC_10x8_UNORM_BLOCK = 177,
    E_ASTC_10x8_SRGB_BLOCK = 178,
    E_ASTC_10x10_UNORM_BLOCK = 179,
    E_ASTC_10x10_SRGB_BLOCK = 180,
    E_ASTC_12x10_UNORM_BLOCK = 181,
    E_ASTC_12x10_SRGB_BLOCK = 182,
    E_ASTC_12x12_UNORM_BLOCK = 183,
    E_ASTC_12x12_SRGB_BLOCK = 184,

    // feature: VK_IMG_format_pvrtc
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    E_PVRTC1_2BPP_UNORM_BLOCK_IMG = 1000054000,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    E_PVRTC1_4BPP_UNORM_BLOCK_IMG = 1000054001,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    E_PVRTC2_2BPP_UNORM_BLOCK_IMG = 1000054002,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    E_PVRTC2_4BPP_UNORM_BLOCK_IMG = 1000054003,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    E_PVRTC1_2BPP_SRGB_BLOCK_IMG = 1000054004,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    E_PVRTC1_4BPP_SRGB_BLOCK_IMG = 1000054005,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    E_PVRTC2_2BPP_SRGB_BLOCK_IMG = 1000054006,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    E_PVRTC2_4BPP_SRGB_BLOCK_IMG = 1000054007,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G8B8G8R8_422_UNORM_KHR = 1000156000,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_B8G8R8G8_422_UNORM_KHR = 1000156001,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G8_B8_R8_3PLANE_420_UNORM_KHR = 1000156002,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G8_B8R8_2PLANE_420_UNORM_KHR = 1000156003,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G8_B8_R8_3PLANE_422_UNORM_KHR = 1000156004,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G8_B8R8_2PLANE_422_UNORM_KHR = 1000156005,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G8_B8_R8_3PLANE_444_UNORM_KHR = 1000156006,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_R10X6_UNORM_PACK16_KHR = 1000156007,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_R10X6G10X6_UNORM_2PACK16_KHR = 1000156008,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR = 1000156009,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR = 1000156010,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR = 1000156011,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR = 1000156012,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR = 1000156013,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR = 1000156014,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR = 1000156015,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR = 1000156016,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_R12X4_UNORM_PACK16_KHR = 1000156017,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_R12X4G12X4_UNORM_2PACK16_KHR = 1000156018,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR = 1000156019,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR = 1000156020,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR = 1000156021,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR = 1000156022,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR = 1000156023,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR = 1000156024,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR = 1000156025,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR = 1000156026,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G16B16G16R16_422_UNORM_KHR = 1000156027,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_B16G16R16G16_422_UNORM_KHR = 1000156028,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G16_B16_R16_3PLANE_420_UNORM_KHR = 1000156029,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G16_B16R16_2PLANE_420_UNORM_KHR = 1000156030,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G16_B16_R16_3PLANE_422_UNORM_KHR = 1000156031,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G16_B16R16_2PLANE_422_UNORM_KHR = 1000156032,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_G16_B16_R16_3PLANE_444_UNORM_KHR = 1000156033
  }
}

define_enum! {

  /// Vulkan structure types (pname:stype)
  ///
  /// Structure types supported by the Vulkan API include.
  ///
  /// Each value corresponds to a particular structure with a `sType` member with a
  /// matching name. As a general rule, the name of each `VkStructureType` value is
  /// obtained by taking the name of the structure, stripping the leading etext:Vk,
  /// prefixing each capital letter with etext:\_, converting the entire resulting
  /// string to upper case, and prefixing it with etext:VK\_STRUCTURE\_TYPE\_. For
  /// example, structures of type `VkImageCreateInfo` correspond to a
  /// `VkStructureType` of `VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO`, and thus its `sType`
  /// member must: equal that when it is passed to the API.
  ///
  /// The values `VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO` and
  /// `VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO` are reserved for internal use by
  /// the loader, and do not have corresponding Vulkan structures in this
  /// specification.
  ///
  pub enum VkStructureType {
    E_APPLICATION_INFO = 0,
    E_INSTANCE_CREATE_INFO = 1,
    E_DEVICE_QUEUE_CREATE_INFO = 2,
    E_DEVICE_CREATE_INFO = 3,
    E_SUBMIT_INFO = 4,
    E_MEMORY_ALLOCATE_INFO = 5,
    E_MAPPED_MEMORY_RANGE = 6,
    E_BIND_SPARSE_INFO = 7,
    E_FENCE_CREATE_INFO = 8,
    E_SEMAPHORE_CREATE_INFO = 9,
    E_EVENT_CREATE_INFO = 10,
    E_QUERY_POOL_CREATE_INFO = 11,
    E_BUFFER_CREATE_INFO = 12,
    E_BUFFER_VIEW_CREATE_INFO = 13,
    E_IMAGE_CREATE_INFO = 14,
    E_IMAGE_VIEW_CREATE_INFO = 15,
    E_SHADER_MODULE_CREATE_INFO = 16,
    E_PIPELINE_CACHE_CREATE_INFO = 17,
    E_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    E_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    E_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    E_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    E_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    E_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    E_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    E_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    E_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    E_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    E_GRAPHICS_PIPELINE_CREATE_INFO = 28,
    E_COMPUTE_PIPELINE_CREATE_INFO = 29,
    E_PIPELINE_LAYOUT_CREATE_INFO = 30,
    E_SAMPLER_CREATE_INFO = 31,
    E_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    E_DESCRIPTOR_POOL_CREATE_INFO = 33,
    E_DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    E_WRITE_DESCRIPTOR_SET = 35,
    E_COPY_DESCRIPTOR_SET = 36,
    E_FRAMEBUFFER_CREATE_INFO = 37,
    E_RENDER_PASS_CREATE_INFO = 38,
    E_COMMAND_POOL_CREATE_INFO = 39,
    E_COMMAND_BUFFER_ALLOCATE_INFO = 40,
    E_COMMAND_BUFFER_INHERITANCE_INFO = 41,
    E_COMMAND_BUFFER_BEGIN_INFO = 42,
    E_RENDER_PASS_BEGIN_INFO = 43,
    E_BUFFER_MEMORY_BARRIER = 44,
    E_IMAGE_MEMORY_BARRIER = 45,
    E_MEMORY_BARRIER = 46,
    E_LOADER_INSTANCE_CREATE_INFO = 47,
    E_LOADER_DEVICE_CREATE_INFO = 48,

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    E_SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
    #[cfg(feature = "VK_KHR_swapchain")]
    E_PRESENT_INFO_KHR = 1000001001,

    // feature: VK_KHR_display
    #[cfg(feature = "VK_KHR_display")]
    E_DISPLAY_MODE_CREATE_INFO_KHR = 1000002000,
    #[cfg(feature = "VK_KHR_display")]
    E_DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001,

    // feature: VK_KHR_display_swapchain
    #[cfg(feature = "VK_KHR_display_swapchain")]
    E_DISPLAY_PRESENT_INFO_KHR = 1000003000,

    // feature: VK_KHR_xlib_surface
    #[cfg(feature = "VK_KHR_xlib_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
    E_XLIB_SURFACE_CREATE_INFO_KHR = 1000004000,

    // feature: VK_KHR_xcb_surface
    #[cfg(feature = "VK_KHR_xcb_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
    E_XCB_SURFACE_CREATE_INFO_KHR = 1000005000,

    // feature: VK_KHR_wayland_surface
    #[cfg(feature = "VK_KHR_wayland_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
    E_WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000,

    // feature: VK_KHR_mir_surface
    #[cfg(feature = "VK_KHR_mir_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
    E_MIR_SURFACE_CREATE_INFO_KHR = 1000007000,

    // feature: VK_KHR_android_surface
    #[cfg(feature = "VK_KHR_android_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
    E_ANDROID_SURFACE_CREATE_INFO_KHR = 1000008000,

    // feature: VK_KHR_win32_surface
    #[cfg(feature = "VK_KHR_win32_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,

    // feature: VK_EXT_debug_report
    #[cfg(feature = "VK_EXT_debug_report")]
    E_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = 1000011000,

    // feature: VK_AMD_rasterization_order
    #[cfg(feature = "VK_AMD_rasterization_order")]
    E_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD = 1000018000,

    // feature: VK_EXT_debug_marker
    #[cfg(feature = "VK_EXT_debug_marker")]
    E_DEBUG_MARKER_OBJECT_NAME_INFO_EXT = 1000022000,
    #[cfg(feature = "VK_EXT_debug_marker")]
    E_DEBUG_MARKER_OBJECT_TAG_INFO_EXT = 1000022001,
    #[cfg(feature = "VK_EXT_debug_marker")]
    E_DEBUG_MARKER_MARKER_INFO_EXT = 1000022002,

    // feature: VK_NV_dedicated_allocation
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    E_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV = 1000026000,
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    E_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV = 1000026001,
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    E_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV = 1000026002,

    // feature: VK_AMD_texture_gather_bias_lod
    #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
    E_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD = 1000041000,

    // feature: VK_KHX_multiview
    #[cfg(feature = "VK_KHX_multiview")]
    E_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX = 1000053000,
    #[cfg(feature = "VK_KHX_multiview")]
    E_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX = 1000053001,
    #[cfg(feature = "VK_KHX_multiview")]
    E_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX = 1000053002,

    // feature: VK_NV_external_memory
    #[cfg(feature = "VK_NV_external_memory")]
    E_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV = 1000056000,
    #[cfg(feature = "VK_NV_external_memory")]
    E_EXPORT_MEMORY_ALLOCATE_INFO_NV = 1000056001,

    // feature: VK_NV_external_memory_win32
    #[cfg(feature = "VK_NV_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057000,
    #[cfg(feature = "VK_NV_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057001,

    // feature: VK_NV_win32_keyed_mutex
    #[cfg(feature = "VK_NV_win32_keyed_mutex")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV = 1000058000,

    // feature: VK_KHR_get_physical_device_properties2
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_PHYSICAL_DEVICE_FEATURES_2_KHR = 1000059000,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_PHYSICAL_DEVICE_PROPERTIES_2_KHR = 1000059001,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_FORMAT_PROPERTIES_2_KHR = 1000059002,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059003,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR = 1000059004,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_QUEUE_FAMILY_PROPERTIES_2_KHR = 1000059005,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR = 1000059006,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059007,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    E_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR = 1000059008,

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    E_MEMORY_ALLOCATE_FLAGS_INFO_KHX = 1000060000,
    #[cfg(feature = "VK_KHX_device_group")]
    E_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX = 1000060003,
    #[cfg(feature = "VK_KHX_device_group")]
    E_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX = 1000060004,
    #[cfg(feature = "VK_KHX_device_group")]
    E_DEVICE_GROUP_SUBMIT_INFO_KHX = 1000060005,
    #[cfg(feature = "VK_KHX_device_group")]
    E_DEVICE_GROUP_BIND_SPARSE_INFO_KHX = 1000060006,
    #[cfg(feature = "VK_KHX_device_group")]
    E_ACQUIRE_NEXT_IMAGE_INFO_KHX = 1000060010,
    #[cfg(feature = "VK_KHX_device_group")]
    E_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHX = 1000060013,
    #[cfg(feature = "VK_KHX_device_group")]
    E_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHX = 1000060014,
    #[cfg(feature = "VK_KHX_device_group")]
    E_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX = 1000060007,
    #[cfg(feature = "VK_KHX_device_group")]
    E_IMAGE_SWAPCHAIN_CREATE_INFO_KHX = 1000060008,
    #[cfg(feature = "VK_KHX_device_group")]
    E_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX = 1000060009,
    #[cfg(feature = "VK_KHX_device_group")]
    E_DEVICE_GROUP_PRESENT_INFO_KHX = 1000060011,
    #[cfg(feature = "VK_KHX_device_group")]
    E_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX = 1000060012,

    // feature: VK_EXT_validation_flags
    #[cfg(feature = "VK_EXT_validation_flags")]
    E_VALIDATION_FLAGS_EXT = 1000061000,

    // feature: VK_NN_vi_surface
    #[cfg(feature = "VK_NN_vi_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
    E_VI_SURFACE_CREATE_INFO_NN = 1000062000,

    // feature: VK_KHX_device_group_creation
    #[cfg(feature = "VK_KHX_device_group_creation")]
    E_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX = 1000070000,
    #[cfg(feature = "VK_KHX_device_group_creation")]
    E_DEVICE_GROUP_DEVICE_CREATE_INFO_KHX = 1000070001,

    // feature: VK_KHR_external_memory_capabilities
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    E_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR = 1000071000,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    E_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR = 1000071001,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    E_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR = 1000071002,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    E_EXTERNAL_BUFFER_PROPERTIES_KHR = 1000071003,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    E_PHYSICAL_DEVICE_ID_PROPERTIES_KHR = 1000071004,

    // feature: VK_KHR_external_memory
    #[cfg(feature = "VK_KHR_external_memory")]
    E_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR = 1000072000,
    #[cfg(feature = "VK_KHR_external_memory")]
    E_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR = 1000072001,
    #[cfg(feature = "VK_KHR_external_memory")]
    E_EXPORT_MEMORY_ALLOCATE_INFO_KHR = 1000072002,

    // feature: VK_KHR_external_memory_win32
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR = 1000073000,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR = 1000073001,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_MEMORY_WIN32_HANDLE_PROPERTIES_KHR = 1000073002,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_MEMORY_GET_WIN32_HANDLE_INFO_KHR = 1000073003,

    // feature: VK_KHR_external_memory_fd
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    E_IMPORT_MEMORY_FD_INFO_KHR = 1000074000,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    E_MEMORY_FD_PROPERTIES_KHR = 1000074001,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    E_MEMORY_GET_FD_INFO_KHR = 1000074002,

    // feature: VK_KHR_win32_keyed_mutex
    #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR = 1000075000,

    // feature: VK_KHR_external_semaphore_capabilities
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    E_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR = 1000076000,
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    E_EXTERNAL_SEMAPHORE_PROPERTIES_KHR = 1000076001,

    // feature: VK_KHR_external_semaphore
    #[cfg(feature = "VK_KHR_external_semaphore")]
    E_EXPORT_SEMAPHORE_CREATE_INFO_KHR = 1000077000,

    // feature: VK_KHR_external_semaphore_win32
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR = 1000078000,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR = 1000078001,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_D3D12_FENCE_SUBMIT_INFO_KHR = 1000078002,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR = 1000078003,

    // feature: VK_KHR_external_semaphore_fd
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    E_IMPORT_SEMAPHORE_FD_INFO_KHR = 1000079000,
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    E_SEMAPHORE_GET_FD_INFO_KHR = 1000079001,

    // feature: VK_KHR_push_descriptor
    #[cfg(feature = "VK_KHR_push_descriptor")]
    E_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR = 1000080000,

    // feature: VK_KHR_16bit_storage
    #[cfg(feature = "VK_KHR_16bit_storage")]
    E_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR = 1000083000,

    // feature: VK_KHR_incremental_present
    #[cfg(feature = "VK_KHR_incremental_present")]
    E_PRESENT_REGIONS_KHR = 1000084000,

    // feature: VK_KHR_descriptor_update_template
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    E_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR = 1000085000,

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    E_OBJECT_TABLE_CREATE_INFO_NVX = 1000086000,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    E_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX = 1000086001,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    E_CMD_PROCESS_COMMANDS_INFO_NVX = 1000086002,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    E_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX = 1000086003,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    E_DEVICE_GENERATED_COMMANDS_LIMITS_NVX = 1000086004,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    E_DEVICE_GENERATED_COMMANDS_FEATURES_NVX = 1000086005,

    // feature: VK_NV_clip_space_w_scaling
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    E_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV = 1000087000,

    // feature: VK_EXT_display_surface_counter
    #[cfg(feature = "VK_EXT_display_surface_counter")]
    E_SURFACE_CAPABILITIES_2_EXT = 1000090000,

    // feature: VK_EXT_display_control
    #[cfg(feature = "VK_EXT_display_control")]
    E_DISPLAY_POWER_INFO_EXT = 1000091000,
    #[cfg(feature = "VK_EXT_display_control")]
    E_DEVICE_EVENT_INFO_EXT = 1000091001,
    #[cfg(feature = "VK_EXT_display_control")]
    E_DISPLAY_EVENT_INFO_EXT = 1000091002,
    #[cfg(feature = "VK_EXT_display_control")]
    E_SWAPCHAIN_COUNTER_CREATE_INFO_EXT = 1000091003,

    // feature: VK_GOOGLE_display_timing
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    E_PRESENT_TIMES_INFO_GOOGLE = 1000092000,

    // feature: VK_NVX_multiview_per_view_attributes
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    E_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX = 1000097000,

    // feature: VK_NV_viewport_swizzle
    #[cfg(feature = "VK_NV_viewport_swizzle")]
    E_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV = 1000098000,

    // feature: VK_EXT_discard_rectangles
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    E_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT = 1000099000,
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    E_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT = 1000099001,

    // feature: VK_EXT_conservative_rasterization
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    E_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT = 1000101000,
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    E_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT = 1000101001,

    // feature: VK_EXT_hdr_metadata
    #[cfg(feature = "VK_EXT_hdr_metadata")]
    E_HDR_METADATA_EXT = 1000105000,

    // feature: VK_KHR_shared_presentable_image
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    E_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR = 1000111000,

    // feature: VK_KHR_external_fence_capabilities
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    E_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR = 1000112000,
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    E_EXTERNAL_FENCE_PROPERTIES_KHR = 1000112001,

    // feature: VK_KHR_external_fence
    #[cfg(feature = "VK_KHR_external_fence")]
    E_EXPORT_FENCE_CREATE_INFO_KHR = 1000113000,

    // feature: VK_KHR_external_fence_win32
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR = 1000114000,
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR = 1000114001,
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    E_FENCE_GET_WIN32_HANDLE_INFO_KHR = 1000114002,

    // feature: VK_KHR_external_fence_fd
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    E_IMPORT_FENCE_FD_INFO_KHR = 1000115000,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    E_FENCE_GET_FD_INFO_KHR = 1000115001,

    // feature: VK_KHR_maintenance2
    #[cfg(feature = "VK_KHR_maintenance2")]
    E_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR = 1000117000,
    #[cfg(feature = "VK_KHR_maintenance2")]
    E_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR = 1000117001,
    #[cfg(feature = "VK_KHR_maintenance2")]
    E_IMAGE_VIEW_USAGE_CREATE_INFO_KHR = 1000117002,
    #[cfg(feature = "VK_KHR_maintenance2")]
    E_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR = 1000117003,

    // feature: VK_KHR_get_surface_capabilities2
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    E_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR = 1000119000,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    E_SURFACE_CAPABILITIES_2_KHR = 1000119001,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    E_SURFACE_FORMAT_2_KHR = 1000119002,

    // feature: VK_KHR_variable_pointers
    #[cfg(feature = "VK_KHR_variable_pointers")]
    E_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR = 1000120000,

    // feature: VK_MVK_ios_surface
    #[cfg(feature = "VK_MVK_ios_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
    E_IOS_SURFACE_CREATE_INFO_MVK = 1000122000,

    // feature: VK_MVK_macos_surface
    #[cfg(feature = "VK_MVK_macos_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
    E_MACOS_SURFACE_CREATE_INFO_MVK = 1000123000,

    // feature: VK_KHR_dedicated_allocation
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    E_MEMORY_DEDICATED_REQUIREMENTS_KHR = 1000127000,
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    E_MEMORY_DEDICATED_ALLOCATE_INFO_KHR = 1000127001,

    // feature: VK_EXT_sampler_filter_minmax
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    E_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT = 1000130000,
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    E_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT = 1000130001,

    // feature: VK_EXT_sample_locations
    #[cfg(feature = "VK_EXT_sample_locations")]
    E_SAMPLE_LOCATIONS_INFO_EXT = 1000143000,
    #[cfg(feature = "VK_EXT_sample_locations")]
    E_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT = 1000143001,
    #[cfg(feature = "VK_EXT_sample_locations")]
    E_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT = 1000143002,
    #[cfg(feature = "VK_EXT_sample_locations")]
    E_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT = 1000143003,
    #[cfg(feature = "VK_EXT_sample_locations")]
    E_MULTISAMPLE_PROPERTIES_EXT = 1000143004,

    // feature: VK_KHR_get_memory_requirements2
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    E_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146000,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    E_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146001,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    E_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146002,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    E_MEMORY_REQUIREMENTS_2_KHR = 1000146003,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    E_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR = 1000146004,

    // feature: VK_KHR_image_format_list
    #[cfg(feature = "VK_KHR_image_format_list")]
    E_IMAGE_FORMAT_LIST_CREATE_INFO_KHR = 1000147000,

    // feature: VK_EXT_blend_operation_advanced
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT = 1000148000,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT = 1000148001,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    E_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT = 1000148002,

    // feature: VK_NV_fragment_coverage_to_color
    #[cfg(feature = "VK_NV_fragment_coverage_to_color")]
    E_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV = 1000149000,

    // feature: VK_NV_framebuffer_mixed_samples
    #[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
    E_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV = 1000152000,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR = 1000156000,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_SAMPLER_YCBCR_CONVERSION_INFO_KHR = 1000156001,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_BIND_IMAGE_PLANE_MEMORY_INFO_KHR = 1000156002,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR = 1000156003,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR = 1000156004,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR = 1000156005,

    // feature: VK_KHR_bind_memory2
    #[cfg(feature = "VK_KHR_bind_memory2")]
    E_BIND_BUFFER_MEMORY_INFO_KHR = 1000157000,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    E_BIND_IMAGE_MEMORY_INFO_KHR = 1000157001,

    // feature: VK_EXT_validation_cache
    #[cfg(feature = "VK_EXT_validation_cache")]
    E_VALIDATION_CACHE_CREATE_INFO_EXT = 1000160000,
    #[cfg(feature = "VK_EXT_validation_cache")]
    E_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT = 1000160001,

    // feature: VK_EXT_global_priority
    #[cfg(feature = "VK_EXT_global_priority")]
    E_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT = 1000174000,

    // feature: VK_EXT_external_memory_host
    #[cfg(feature = "VK_EXT_external_memory_host")]
    E_IMPORT_MEMORY_HOST_POINTER_INFO_EXT = 1000178000,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    E_MEMORY_HOST_POINTER_PROPERTIES_EXT = 1000178001,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    E_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT = 1000178002
  }
}

define_enum! {

  /// Specify how commands in the first subpass of a render pass are provided
  ///
  /// Possible values of `vkCmdBeginRenderPass::contents`, specifying how the commands
  /// in the first subpass will be provided.
  ///
  ///   - `VK_SUBPASS_CONTENTS_INLINE` specifies that the contents of the subpass will
  ///     be recorded inline in the primary command buffer, and secondary command
  ///     buffers must: not be executed within the subpass.
  ///
  ///   - `VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS` specifies that the contents
  ///     are recorded in secondary command buffers that will be called from the
  ///     primary command buffer, and `vkCmdExecuteCommands` is the only valid command
  ///     on the command buffer until `vkCmdNextSubpass` or `vkCmdEndRenderPass`.
  ///
  pub enum VkSubpassContents {
    E_INLINE = 0,
    E_SECONDARY_COMMAND_BUFFERS = 1
  }
}

define_enum! {

  /// Vulkan command return codes
  ///
  /// While the core Vulkan API is not designed to capture incorrect usage, some
  /// circumstances still require return codes. Commands in Vulkan return their status
  /// via return codes that are in one of two categories:
  ///
  ///   - Successful completion codes are returned when a command needs to communicate
  ///     success or status information. All successful completion codes are
  ///     non-negative values.
  ///
  ///   - Run time error codes are returned when a command needs to communicate a
  ///     failure that could only be detected at run time. All run time error codes
  ///     are negative values.
  ///
  /// All return codes in Vulkan are reported via `VkResult` return values.
  ///
  ///   - `VK_SUCCESS` Command successfully completed
  ///
  ///   - `VK_NOT_READY` A fence or query has not yet completed
  ///
  ///   - `VK_TIMEOUT` A wait operation has not completed in the specified time
  ///
  ///   - `VK_EVENT_SET` An event is signaled
  ///
  ///   - `VK_EVENT_RESET` An event is unsignaled
  ///
  ///   - `VK_INCOMPLETE` A return array was too small for the result
  ///
  ///   - `VK_SUBOPTIMAL_KHR` A swapchain no longer matches the surface properties
  ///     exactly, but can: still be used to present to the surface successfully.
  ///
  /// <!-- end list -->
  ///
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY` A host memory allocation has failed.
  ///
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY` A device memory allocation has failed.
  ///
  ///   - `VK_ERROR_INITIALIZATION_FAILED` Initialization of an object could not be
  ///     completed for implementation-specific reasons.
  ///
  ///   - `VK_ERROR_DEVICE_LOST` The logical or physical device has been lost. See
  ///     [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-lost-device)
  ///
  ///   - `VK_ERROR_MEMORY_MAP_FAILED` Mapping of a memory object has failed.
  ///
  ///   - `VK_ERROR_LAYER_NOT_PRESENT` A requested layer is not present or could not
  ///     be loaded.
  ///
  ///   - `VK_ERROR_EXTENSION_NOT_PRESENT` A requested extension is not supported.
  ///
  ///   - `VK_ERROR_FEATURE_NOT_PRESENT` A requested feature is not supported.
  ///
  ///   - `VK_ERROR_INCOMPATIBLE_DRIVER` The requested version of Vulkan is not
  ///     supported by the driver or is otherwise incompatible for
  ///     implementation-specific reasons.
  ///
  ///   - `VK_ERROR_TOO_MANY_OBJECTS` Too many objects of the type have already been
  ///     created.
  ///
  ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED` A requested format is not supported on this
  ///     device.
  ///
  ///   - `VK_ERROR_FRAGMENTED_POOL` A pool allocation has failed due to fragmentation
  ///     of the pool’s memory. This must: only be returned if no attempt to allocate
  ///     host or device memory was made to accomodate the new allocation. This
  ///     should: be returned in preference to `VK_ERROR_OUT_OF_POOL_MEMORY_KHR`, but
  ///     only if the implementation is certain that the pool allocation failure was
  ///     due to fragmentation.
  ///
  ///   - `VK_ERROR_SURFACE_LOST_KHR` A surface is no longer available.
  ///
  ///   - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR` The requested window is already in use
  ///     by Vulkan or another API in a manner which prevents it from being used
  ///     again.
  ///
  ///   - `VK_ERROR_OUT_OF_DATE_KHR` A surface has changed in such a way that it is no
  ///     longer compatible with the swapchain, and further presentation requests
  ///     using the swapchain will fail. Applications must: query the new surface
  ///     properties and recreate their swapchain if they wish to continue presenting
  ///     to the surface.
  ///
  ///   - `VK_ERROR_INCOMPATIBLE_DISPLAY_KHR` The display used by a swapchain does not
  ///     use the same presentable image layout, or is incompatible in a way that
  ///     prevents sharing an image.
  ///
  ///   - `VK_ERROR_INVALID_SHADER_NV` One or more shaders failed to compile or link.
  ///     More details are reported back to the application via  if enabled.
  ///
  ///   - `VK_ERROR_OUT_OF_POOL_MEMORY_KHR` A pool memory allocation has failed. This
  ///     must: only be returned if no attempt to allocate host or device memory was
  ///     made to accomodate the new allocation. If the failure was definitely due to
  ///     fragmentation of the pool, `VK_ERROR_FRAGMENTED_POOL` should: be returned
  ///     instead.
  ///
  /// If a command returns a run time error, unless otherwise specified any output
  /// parameters will have undefined contents, except that if the output parameter is
  /// a structure with `sType` and `pNext` fields, those fields will be unmodified.
  /// Any structures chained from `pNext` will also have undefined contents, except
  /// that `sType` and `pNext` will be unmodified.
  ///
  /// Out of memory errors do not damage any currently existing Vulkan objects.
  /// Objects that have already been successfully created can: still be used by the
  /// application.
  ///
  /// Performance-critical commands generally do not have return codes. If a run time
  /// error occurs in such commands, the implementation will defer reporting the error
  /// until a specified point. For commands that record into command buffers
  /// (ftext:vkCmd\*) run time errors are reported by `vkEndCommandBuffer`.
  ///
  pub enum VkResult {
    E_SUCCESS = 0,
    E_NOT_READY = 1,
    E_TIMEOUT = 2,
    E_EVENT_SET = 3,
    E_EVENT_RESET = 4,
    E_INCOMPLETE = 5,
    E_ERROR_OUT_OF_HOST_MEMORY = !0,
    E_ERROR_OUT_OF_DEVICE_MEMORY = !1,
    E_ERROR_INITIALIZATION_FAILED = !2,
    E_ERROR_DEVICE_LOST = !3,
    E_ERROR_MEMORY_MAP_FAILED = !4,
    E_ERROR_LAYER_NOT_PRESENT = !5,
    E_ERROR_EXTENSION_NOT_PRESENT = !6,
    E_ERROR_FEATURE_NOT_PRESENT = !7,
    E_ERROR_INCOMPATIBLE_DRIVER = !8,
    E_ERROR_TOO_MANY_OBJECTS = !9,
    E_ERROR_FORMAT_NOT_SUPPORTED = !10,
    E_ERROR_FRAGMENTED_POOL = !11,

    // feature: VK_KHR_surface
    #[cfg(feature = "VK_KHR_surface")]
    E_ERROR_SURFACE_LOST_KHR = !999999999,
    #[cfg(feature = "VK_KHR_surface")]
    E_ERROR_NATIVE_WINDOW_IN_USE_KHR = !1000000000,

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    E_SUBOPTIMAL_KHR = 1000001003,
    #[cfg(feature = "VK_KHR_swapchain")]
    E_ERROR_OUT_OF_DATE_KHR = !1000001003,

    // feature: VK_KHR_display_swapchain
    #[cfg(feature = "VK_KHR_display_swapchain")]
    E_ERROR_INCOMPATIBLE_DISPLAY_KHR = !1000003000,

    // feature: VK_EXT_debug_report
    #[cfg(feature = "VK_EXT_debug_report")]
    E_ERROR_VALIDATION_FAILED_EXT = !1000011000,

    // feature: VK_NV_glsl_shader
    #[cfg(feature = "VK_NV_glsl_shader")]
    E_ERROR_INVALID_SHADER_NV = !1000011999,

    // feature: VK_KHR_maintenance1
    #[cfg(feature = "VK_KHR_maintenance1")]
    E_ERROR_OUT_OF_POOL_MEMORY_KHR = !1000068999,

    // feature: VK_KHR_external_memory
    #[cfg(feature = "VK_KHR_external_memory")]
    E_ERROR_INVALID_EXTERNAL_HANDLE_KHR = !1000072002,

    // feature: VK_EXT_global_priority
    #[cfg(feature = "VK_EXT_global_priority")]
    E_ERROR_NOT_PERMITTED_EXT = !1000174000
  }
}

define_enum! {

  /// Indicate which dynamic state is taken from dynamic state commands
  ///
  /// The source of different pieces of dynamic state is specified by the
  /// `VkPipelineDynamicStateCreateInfo::pDynamicStates` property of the currently
  /// active pipeline, each of whose elements must: be one of the values.
  ///
  ///   - `VK_DYNAMIC_STATE_VIEWPORT` specifies that the `pViewports` state in
  ///     `VkPipelineViewportStateCreateInfo` will be ignored and must: be set
  ///     dynamically with `vkCmdSetViewport` before any draw commands. The number of
  ///     viewports used by a pipeline is still specified by the `viewportCount`
  ///     member of `VkPipelineViewportStateCreateInfo`.
  ///
  ///   - `VK_DYNAMIC_STATE_SCISSOR` specifies that the `pScissors` state in
  ///     `VkPipelineViewportStateCreateInfo` will be ignored and must: be set
  ///     dynamically with `vkCmdSetScissor` before any draw commands. The number of
  ///     scissor rectangles used by a pipeline is still specified by the
  ///     `scissorCount` member of `VkPipelineViewportStateCreateInfo`.
  ///
  ///   - `VK_DYNAMIC_STATE_LINE_WIDTH` specifies that the `lineWidth` state in
  ///     `VkPipelineRasterizationStateCreateInfo` will be ignored and must: be set
  ///     dynamically with `vkCmdSetLineWidth` before any draw commands that generate
  ///     line primitives for the rasterizer.
  ///
  ///   - `VK_DYNAMIC_STATE_DEPTH_BIAS` specifies that the `depthBiasConstantFactor`,
  ///     `depthBiasClamp` and `depthBiasSlopeFactor` states in
  ///     `VkPipelineRasterizationStateCreateInfo` will be ignored and must: be set
  ///     dynamically with `vkCmdSetDepthBias` before any draws are performed with
  ///     `depthBiasEnable` in `VkPipelineRasterizationStateCreateInfo` set to
  ///     `VK_TRUE`.
  ///
  ///   - `VK_DYNAMIC_STATE_BLEND_CONSTANTS` specifies that the `blendConstants` state
  ///     in `VkPipelineColorBlendStateCreateInfo` will be ignored and must: be set
  ///     dynamically with `vkCmdSetBlendConstants` before any draws are performed
  ///     with a pipeline state with `VkPipelineColorBlendAttachmentState` member
  ///     `blendEnable` set to `VK_TRUE` and any of the blend functions using a
  ///     constant blend color.
  ///
  ///   - `VK_DYNAMIC_STATE_DEPTH_BOUNDS` specifies that the `minDepthBounds` and
  ///     `maxDepthBounds` states of `VkPipelineDepthStencilStateCreateInfo` will be
  ///     ignored and must: be set dynamically with `vkCmdSetDepthBounds` before any
  ///     draws are performed with a pipeline state with
  ///     `VkPipelineDepthStencilStateCreateInfo` member `depthBoundsTestEnable` set
  ///     to `VK_TRUE`.
  ///
  ///   - `VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK` specifies that the `compareMask`
  ///     state in `VkPipelineDepthStencilStateCreateInfo` for both `front` and `back`
  ///     will be ignored and must: be set dynamically with
  ///     `vkCmdSetStencilCompareMask` before any draws are performed with a pipeline
  ///     state with `VkPipelineDepthStencilStateCreateInfo` member
  ///     `stencilTestEnable` set to `VK_TRUE`
  ///
  ///   - `VK_DYNAMIC_STATE_STENCIL_WRITE_MASK` specifies that the `writeMask` state
  ///     in `VkPipelineDepthStencilStateCreateInfo` for both `front` and `back` will
  ///     be ignored and must: be set dynamically with `vkCmdSetStencilWriteMask`
  ///     before any draws are performed with a pipeline state with
  ///     `VkPipelineDepthStencilStateCreateInfo` member `stencilTestEnable` set to
  ///     `VK_TRUE`
  ///
  ///   - `VK_DYNAMIC_STATE_STENCIL_REFERENCE` specifies that the `reference` state in
  ///     `VkPipelineDepthStencilStateCreateInfo` for both `front` and `back` will be
  ///     ignored and must: be set dynamically with `vkCmdSetStencilReference` before
  ///     any draws are performed with a pipeline state with
  ///     `VkPipelineDepthStencilStateCreateInfo` member `stencilTestEnable` set to
  ///     `VK_TRUE`
  ///
  ///   - `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` specifies that the
  ///     `pViewportScalings` state in `VkPipelineViewportWScalingStateCreateInfoNV`
  ///     will be ignored and must: be set dynamically with
  ///     `vkCmdSetViewportWScalingNV` before any draws are performed with a pipeline
  ///     state with `VkPipelineViewportWScalingStateCreateInfo` member
  ///     `viewportScalingEnable` set to `VK_TRUE`
  ///
  ///   - `VK_DYNAMIC_STATE_DISCARD_RECTANGLES_EXT` specifies that the
  ///     `pDiscardRectangles` state in `VkPipelineDiscardRectangleStateCreateInfoEXT`
  ///     will be ignored and must: be set dynamically with
  ///     `vkCmdSetDiscardRectangleEXT` before any draw or clear commands. The
  ///     `VkDiscardRectangleModeEXT` and the number of active discard rectangles is
  ///     still specified by the `discardRectangleMode` and `discardRectangleCount`
  ///     members of `VkPipelineDiscardRectangleStateCreateInfoEXT`.
  ///
  ///   - `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` specifies that the
  ///     `sampleLocationsInfo` state in `VkPipelineSampleLocationsStateCreateInfoEXT`
  ///     will be ignored and must: be set dynamically with
  ///     `vkCmdSetSampleLocationsEXT` before any draw or clear commands. Enabling
  ///     custom sample locations is still indicated by the `sampleLocationsEnable`
  ///     member of `VkPipelineSampleLocationsStateCreateInfoEXT`.
  ///
  pub enum VkDynamicState {
    E_VIEWPORT = 0,
    E_SCISSOR = 1,
    E_LINE_WIDTH = 2,
    E_DEPTH_BIAS = 3,
    E_BLEND_CONSTANTS = 4,
    E_DEPTH_BOUNDS = 5,
    E_STENCIL_COMPARE_MASK = 6,
    E_STENCIL_WRITE_MASK = 7,
    E_STENCIL_REFERENCE = 8,

    // feature: VK_NV_clip_space_w_scaling
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    E_VIEWPORT_W_SCALING_NV = 1000087000,

    // feature: VK_EXT_discard_rectangles
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    E_DISCARD_RECTANGLE_EXT = 1000099000,

    // feature: VK_EXT_sample_locations
    #[cfg(feature = "VK_EXT_sample_locations")]
    E_SAMPLE_LOCATIONS_EXT = 1000143000
  }
}

// feature: VK_KHR_descriptor_update_template

#[cfg(feature = "VK_KHR_descriptor_update_template")]
define_enum! {

  /// Indicates the valid usage of the descriptor update template
  ///
  /// The descriptor update template type is determined by the
  /// `VkDescriptorUpdateTemplateCreateInfoKHR::templateType` property, which takes
  /// the following values.
  ///
  ///   - `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR` specifies that the
  ///     descriptor update template will be used for descriptor set updates only.
  ///
  ///   - `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR` specifies that the
  ///     descriptor update template will be used for push descriptor updates only.
  ///
  pub enum VkDescriptorUpdateTemplateTypeKHR {
    E_DESCRIPTOR_SET_KHR = 0,
    E_PUSH_DESCRIPTORS_KHR = 1
  }
}

// feature: VK_VERSION_1_0

define_enum! {

  /// Specify an enumeration to track object handle types
  ///
  /// The `VkObjectType` enumeration defines values, each of which corresponds to a
  /// specific Vulkan handle type. These values can: be used to associate debug
  /// information with a particular type of object through one or more extensions.
  ///
  /// <table>
  /// <caption>VkObjectType and Vulkan Handle Relationship</caption>
  /// <colgroup>
  /// <col width="60%" />
  /// <col width="39%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left"><code>VkObjectType</code></th>
  /// <th align="left">Vulkan Handle Type</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_UNKNOWN</code></p></td>
  /// <td align="left"><p>Unknown/Undefined Handle</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_INSTANCE</code></p></td>
  /// <td align="left"><p><code>VkInstance</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_PHYSICAL_DEVICE</code></p></td>
  /// <td align="left"><p><code>VkPhysicalDevice</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DEVICE</code></p></td>
  /// <td align="left"><p><code>VkDevice</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_QUEUE</code></p></td>
  /// <td align="left"><p><code>VkQueue</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_SEMAPHORE</code></p></td>
  /// <td align="left"><p><code>VkSemaphore</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_COMMAND_BUFFER</code></p></td>
  /// <td align="left"><p><code>VkCommandBuffer</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_FENCE</code></p></td>
  /// <td align="left"><p><code>VkFence</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DEVICE_MEMORY</code></p></td>
  /// <td align="left"><p><code>VkDeviceMemory</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_BUFFER</code></p></td>
  /// <td align="left"><p><code>VkBuffer</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_IMAGE</code></p></td>
  /// <td align="left"><p><code>VkImage</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_EVENT</code></p></td>
  /// <td align="left"><p><code>VkEvent</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_QUERY_POOL</code></p></td>
  /// <td align="left"><p><code>VkQueryPool</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_BUFFER_VIEW</code></p></td>
  /// <td align="left"><p><code>VkBufferView</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_IMAGE_VIEW</code></p></td>
  /// <td align="left"><p><code>VkImageView</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_SHADER_MODULE</code></p></td>
  /// <td align="left"><p><code>VkShaderModule</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_PIPELINE_CACHE</code></p></td>
  /// <td align="left"><p><code>VkPipelineCache</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_PIPELINE_LAYOUT</code></p></td>
  /// <td align="left"><p><code>VkPipelineLayout</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_RENDER_PASS</code></p></td>
  /// <td align="left"><p><code>VkRenderPass</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_PIPELINE</code></p></td>
  /// <td align="left"><p><code>VkPipeline</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT</code></p></td>
  /// <td align="left"><p><code>VkDescriptorSetLayout</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_SAMPLER</code></p></td>
  /// <td align="left"><p><code>VkSampler</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DESCRIPTOR_POOL</code></p></td>
  /// <td align="left"><p><code>VkDescriptorPool</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DESCRIPTOR_SET</code></p></td>
  /// <td align="left"><p><code>VkDescriptorSet</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_FRAMEBUFFER</code></p></td>
  /// <td align="left"><p><code>VkFramebuffer</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_COMMAND_POOL</code></p></td>
  /// <td align="left"><p><code>VkCommandPool</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_SURFACE_KHR</code></p></td>
  /// <td align="left"><p><code>VkSurfaceKHR</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_SWAPCHAIN_KHR</code></p></td>
  /// <td align="left"><p><code>VkSwapchainKHR</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DISPLAY_KHR</code></p></td>
  /// <td align="left"><p><code>VkDisplayKHR</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DISPLAY_MODE_KHR</code></p></td>
  /// <td align="left"><p><code>VkDisplayModeKHR</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT</code></p></td>
  /// <td align="left"><p><code>VkDebugReportCallbackEXT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR</code></p></td>
  /// <td align="left"><p><code>VkDescriptorUpdateTemplateKHR</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_OBJECT_TABLE_NVX</code></p></td>
  /// <td align="left"><p><code>VkObjectTableNVX</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX</code></p></td>
  /// <td align="left"><p><code>VkIndirectCommandsLayoutNVX</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_OBJECT_TYPE_VALIDATION_CACHE_EXT</code></p></td>
  /// <td align="left"><p><code>VkValidationCacheEXT</code></p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  pub enum VkObjectType {
    E_UNKNOWN = 0,
    E_INSTANCE = 1,
    E_PHYSICAL_DEVICE = 2,
    E_DEVICE = 3,
    E_QUEUE = 4,
    E_SEMAPHORE = 5,
    E_COMMAND_BUFFER = 6,
    E_FENCE = 7,
    E_DEVICE_MEMORY = 8,
    E_BUFFER = 9,
    E_IMAGE = 10,
    E_EVENT = 11,
    E_QUERY_POOL = 12,
    E_BUFFER_VIEW = 13,
    E_IMAGE_VIEW = 14,
    E_SHADER_MODULE = 15,
    E_PIPELINE_CACHE = 16,
    E_PIPELINE_LAYOUT = 17,
    E_RENDER_PASS = 18,
    E_PIPELINE = 19,
    E_DESCRIPTOR_SET_LAYOUT = 20,
    E_SAMPLER = 21,
    E_DESCRIPTOR_POOL = 22,
    E_DESCRIPTOR_SET = 23,
    E_FRAMEBUFFER = 24,
    E_COMMAND_POOL = 25,

    // feature: VK_KHR_surface
    #[cfg(feature = "VK_KHR_surface")]
    E_SURFACE_KHR = 1000000000,

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    E_SWAPCHAIN_KHR = 1000001000,

    // feature: VK_KHR_display
    #[cfg(feature = "VK_KHR_display")]
    E_DISPLAY_KHR = 1000002000,
    #[cfg(feature = "VK_KHR_display")]
    E_DISPLAY_MODE_KHR = 1000002001,

    // feature: VK_EXT_debug_report
    #[cfg(feature = "VK_EXT_debug_report")]
    E_DEBUG_REPORT_CALLBACK_EXT = 1000011000,

    // feature: VK_KHR_descriptor_update_template
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    E_DESCRIPTOR_UPDATE_TEMPLATE_KHR = 1000085000,

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    E_OBJECT_TABLE_NVX = 1000086000,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    E_INDIRECT_COMMANDS_LAYOUT_NVX = 1000086001,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_SAMPLER_YCBCR_CONVERSION_KHR = 1000156000,

    // feature: VK_EXT_validation_cache
    #[cfg(feature = "VK_EXT_validation_cache")]
    E_VALIDATION_CACHE_EXT = 1000160000
  }
}

define_bitmask! {

  /// Bitmask specifying capabilities of queues in a queue family
  ///
  /// Bits which may: be set in `VkQueueFamilyProperties::queueFlags` indicating
  /// capabilities of queues in a queue family are.
  ///
  ///   - `VK_QUEUE_GRAPHICS_BIT` indicates that queues in this queue family support
  ///     graphics operations.
  ///
  ///   - `VK_QUEUE_COMPUTE_BIT` indicates that queues in this queue family support
  ///     compute operations.
  ///
  ///   - `VK_QUEUE_TRANSFER_BIT` indicates that queues in this queue family support
  ///     transfer operations.
  ///
  ///   - `VK_QUEUE_SPARSE_BINDING_BIT` indicates that queues in this queue family
  ///     support sparse memory management operations (see [Sparse
  ///     Resources](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory)). If any of the sparse resource features are
  ///     enabled, then at least one queue family must: support this bit.
  ///
  /// If an implementation exposes any queue family that supports graphics operations,
  /// at least one queue family of at least one physical device exposed by the
  /// implementation must: support both graphics and compute operations.
  ///
  /// > **Note**
  /// >
  /// > All commands that are allowed on a queue that supports transfer operations are
  /// > also allowed on a queue that supports either graphics or compute operations.
  /// > Thus, if the capabilities of a queue family include `VK_QUEUE_GRAPHICS_BIT` or
  /// > `VK_QUEUE_COMPUTE_BIT`, then reporting the `VK_QUEUE_TRANSFER_BIT` capability
  /// > separately for that queue family is optional:.
  ///
  /// For further details see [Queues](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#devsandqueues-queues).
  ///
  pub enum VkQueueFlagBits {
    BIT_GRAPHICS = 1<<0,
    BIT_COMPUTE = 1<<1,
    BIT_TRANSFER = 1<<2,
    BIT_SPARSE_BINDING = 1<<3
  }
}

define_bitmask! {

  /// Bitmask specifying properties for a memory type
  ///
  /// Bits which may: be set in `VkMemoryType::propertyFlags`, indicating properties
  /// of a memory heap.
  ///
  ///   - `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT` bit indicates that memory allocated
  ///     with this type is the most efficient for device access. This property will
  ///     be set if and only if the memory type belongs to a heap with the
  ///     `VK_MEMORY_HEAP_DEVICE_LOCAL_BIT` set.
  ///
  ///   - `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` bit indicates that memory allocated
  ///     with this type can: be mapped for host access using `vkMapMemory`.
  ///
  ///   - `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT` bit indicates that the host cache
  ///     management commands `vkFlushMappedMemoryRanges` and
  ///     `vkInvalidateMappedMemoryRanges` are not needed to flush host writes to the
  ///     device or make device writes visible to the host, respectively.
  ///
  ///   - `VK_MEMORY_PROPERTY_HOST_CACHED_BIT` bit indicates that memory allocated
  ///     with this type is cached on the host. Host memory accesses to uncached
  ///     memory are slower than to cached memory, however uncached memory is always
  ///     host coherent.
  ///
  ///   - `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT` bit indicates that the memory type
  ///     only allows device access to the memory. Memory types must: not have both
  ///     `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT` and
  ///     `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` set. Additionally, the object’s
  ///     backing memory may: be provided by the implementation lazily as specified in
  ///     [Lazily Allocated Memory](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory-device-lazy_allocation).
  ///
  pub enum VkMemoryPropertyFlagBits {
    BIT_DEVICE_LOCAL = 1<<0,
    BIT_HOST_VISIBLE = 1<<1,
    BIT_HOST_COHERENT = 1<<2,
    BIT_HOST_CACHED = 1<<3,
    BIT_LAZILY_ALLOCATED = 1<<4
  }
}

define_bitmask! {

  /// Bitmask specifying attribute flags for a heap
  ///
  /// Bits which may: be set in `VkMemoryHeap::flags`, indicating attribute flags for
  /// the heap.
  ///
  ///   - `VK_MEMORY_HEAP_DEVICE_LOCAL_BIT` indicates that the heap corresponds to
  ///     device local memory. Device local memory may: have different performance
  ///     characteristics than host local memory, and may: support different memory
  ///     property flags.
  ///
  ///   - `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHX` indicates that in a logical device
  ///     representing more than one physical device, there is a per-physical device
  ///     instance of the heap memory. By default, an allocation from such a heap will
  ///     be replicated to each physical device’s instance of the heap.
  ///
  pub enum VkMemoryHeapFlagBits {
    BIT_DEVICE_LOCAL = 1<<0,

    // feature: VK_KHX_device_group_creation
    #[cfg(feature = "VK_KHX_device_group_creation")]
    BIT_MULTI_INSTANCE_BIT_KHX = 1<<1
  }
}

define_bitmask! {

  /// Bitmask specifying memory access types that will participate in a memory
  /// dependency
  ///
  /// Memory in Vulkan can: be accessed from within shader invocations and via some
  /// fixed-function stages of the pipeline. The *access type* is a function of the
  /// [descriptor type](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets) used, or how a fixed-function stage accesses
  /// memory. Each access type corresponds to a bit flag in `VkAccessFlagBits`.
  ///
  /// Some synchronization commands take sets of access types as parameters to define
  /// the [access scopes](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) of a memory
  /// dependency. If a synchronization command includes a source access mask, its
  /// first [access scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) only includes
  /// accesses via the access types specified in that mask. Similarly, if a
  /// synchronization command includes a destination access mask, its second [access
  /// scope](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) only includes accesses via
  /// the access types specified in that mask.
  ///
  /// Access types that can: be set in an access mask include.
  ///
  ///   - `VK_ACCESS_INDIRECT_COMMAND_READ_BIT` specifies read access to an indirect
  ///     command structure read as part of an indirect drawing or dispatch command.
  ///
  ///   - `VK_ACCESS_INDEX_READ_BIT` specifies read access to an index buffer as part
  ///     of an indexed drawing command, bound by `vkCmdBindIndexBuffer`.
  ///
  ///   - `VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT` specifies read access to a vertex
  ///     buffer as part of a drawing command, bound by `vkCmdBindVertexBuffers`.
  ///
  ///   - `VK_ACCESS_UNIFORM_READ_BIT` specifies read access to a [uniform
  ///     buffer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformbuffer).
  ///
  ///   - `VK_ACCESS_INPUT_ATTACHMENT_READ_BIT` specifies read access to an [input
  ///     attachment](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass) within a render pass during fragment shading.
  ///
  ///   - `VK_ACCESS_SHADER_READ_BIT` specifies read access to a [storage
  ///     buffer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebuffer), [uniform texel
  ///     buffer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer), [storage texel
  ///     buffer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer), [sampled
  ///     image](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sampledimage), or [storage
  ///     image](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storageimage).
  ///
  ///   - `VK_ACCESS_SHADER_WRITE_BIT` specifies write access to a [storage
  ///     buffer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagebuffer), [storage texel
  ///     buffer](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer), or [storage
  ///     image](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storageimage).
  ///
  ///   - `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT` specifies read access to a [color
  ///     attachment](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass), such as via [blending](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blending),
  ///     [logic operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-logicop), or via certain [subpass load
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops). It does not include [advanced blend
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blend-advanced).
  ///
  ///   - `VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is similar to
  ///     `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`, but also includes [advanced blend
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blend-advanced).
  ///
  ///   - `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT` specifies write access to a [color or
  ///     resolve attachment](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass) during a [render pass](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass) or via
  ///     certain [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops).
  ///
  ///   - `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT` specifies read access to a
  ///     [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass), via [depth or stencil
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-ds-state) or via certain [subpass load
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops).
  ///
  ///   - `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT` specifies write access to a
  ///     [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass), via [depth or stencil
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fragops-ds-state) or via certain [subpass load and store
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops).
  ///
  ///   - `VK_ACCESS_TRANSFER_READ_BIT` specifies read access to an image or buffer in
  ///     a [copy](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#copies) operation.
  ///
  ///   - `VK_ACCESS_TRANSFER_WRITE_BIT` specifies write access to an image or buffer
  ///     in a [clear](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#clears) or [copy](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#copies) operation.
  ///
  ///   - `VK_ACCESS_HOST_READ_BIT` specifies read access by a host operation.
  ///     Accesses of this type are not performed through a resource, but directly on
  ///     memory.
  ///
  ///   - `VK_ACCESS_HOST_WRITE_BIT` specifies write access by a host operation.
  ///     Accesses of this type are not performed through a resource, but directly on
  ///     memory.
  ///
  ///   - `VK_ACCESS_MEMORY_READ_BIT` specifies read access via non-specific entities.
  ///     These entities include the Vulkan device and host, but may: also include
  ///     entities external to the Vulkan device or otherwise not part of the core
  ///     Vulkan pipeline. When included in a destination access mask, makes all
  ///     available writes visible to all future read accesses on entities known to
  ///     the Vulkan device.
  ///
  ///   - `VK_ACCESS_MEMORY_WRITE_BIT` specifies write access via non-specific
  ///     entities. These entities include the Vulkan device and host, but may: also
  ///     include entities external to the Vulkan device or otherwise not part of the
  ///     core Vulkan pipeline. When included in a source access mask, all writes that
  ///     are performed by entities known to the Vulkan device are made available.
  ///     When included in a destination access mask, makes all available writes
  ///     visible to all future write accesses on entities known to the Vulkan device.
  ///
  ///   - `VK_ACCESS_COMMAND_PROCESS_READ_BIT_NVX` specifies reads from `VkBuffer`
  ///     inputs to `vkCmdProcessCommandsNVX`.
  ///
  ///   - `VK_ACCESS_COMMAND_PROCESS_WRITE_BIT_NVX` specifies writes to the target
  ///     command buffer in `vkCmdProcessCommandsNVX`.
  ///
  /// Certain access types are only performed by a subset of pipeline stages. Any
  /// synchronization command that takes both stage masks and access masks uses both
  /// to define the [access scopes](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) -
  /// only the specified access types performed by the specified stages are included
  /// in the access scope. An application must: not specify an access flag in a
  /// synchronization command if it does not include a pipeline stage in the
  /// corresponding stage mask that is able to perform accesses of that type. The
  /// following table lists, for each access flag, which pipeline stages can: perform
  /// that type of access.
  ///
  /// <table>
  /// <caption>Supported access types</caption>
  /// <colgroup>
  /// <col width="50%" />
  /// <col width="50%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">Access flag</th>
  /// <th align="left">Supported pipeline stages</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_INDIRECT_COMMAND_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_INDEX_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_VERTEX_INPUT_BIT</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_VERTEX_INPUT_BIT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_UNIFORM_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_VERTEX_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT</code>, or <code>VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_INPUT_ATTACHMENT_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_SHADER_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_VERTEX_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT</code>, or <code>VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_SHADER_WRITE_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_VERTEX_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT</code>, <code>VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT</code>, or <code>VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_COLOR_ATTACHMENT_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT</code>, or <code>VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT</code>, or <code>VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_TRANSFER_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_TRANSFER_BIT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_TRANSFER_WRITE_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_TRANSFER_BIT</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_HOST_READ_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_HOST_BIT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_HOST_WRITE_BIT</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_HOST_BIT</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_MEMORY_READ_BIT</code></p></td>
  /// <td align="left"><p>N/A</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_MEMORY_WRITE_BIT</code></p></td>
  /// <td align="left"><p>N/A</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_ACCESS_COMMAND_PROCESS_READ_BIT_NVX</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_ACCESS_COMMAND_PROCESS_WRITE_BIT_NVX</code></p></td>
  /// <td align="left"><p><code>VK_PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX</code></p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  /// If a memory object does not have the `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
  /// property, then `vkFlushMappedMemoryRanges` must: be called in order to guarantee
  /// that writes to the memory object from the host are made visible to the
  /// `VK_ACCESS_HOST_WRITE_BIT` [access type](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-types), where
  /// it can: be further made available to the device by [synchronization
  /// commands](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization). Similarly, `vkInvalidateMappedMemoryRanges` must:
  /// be called to guarantee that writes which are visible to the
  /// `VK_ACCESS_HOST_READ_BIT` [access type](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-types) are made
  /// visible to host operations.
  ///
  /// If the memory object does have the `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`
  /// property flag, writes to the memory object from the host are automatically made
  /// visible to the `VK_ACCESS_HOST_WRITE_BIT` [access
  /// type](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-types). Similarly, writes made visible to the
  /// `VK_ACCESS_HOST_READ_BIT` [access type](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-access-types) are
  /// automatically made visible to the host.
  ///
  /// > **Note**
  /// >
  /// > The `vkQueueSubmit` command [automatically guarantees that host writes flushed
  /// > to `VK_ACCESS_HOST_WRITE_BIT` are made
  /// > available](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-submission-host-writes) if they were flushed
  /// > before the command executed, so in most cases an explicit memory barrier is
  /// > not needed for this case. In the few circumstances where a submit does not
  /// > occur between the host write and the device read access, writes can: be made
  /// > available by using an explicit memory barrier.
  ///
  pub enum VkAccessFlagBits {
    BIT_INDIRECT_COMMAND_READ = 1<<0,
    BIT_INDEX_READ = 1<<1,
    BIT_VERTEX_ATTRIBUTE_READ = 1<<2,
    BIT_UNIFORM_READ = 1<<3,
    BIT_INPUT_ATTACHMENT_READ = 1<<4,
    BIT_SHADER_READ = 1<<5,
    BIT_SHADER_WRITE = 1<<6,
    BIT_COLOR_ATTACHMENT_READ = 1<<7,
    BIT_COLOR_ATTACHMENT_WRITE = 1<<8,
    BIT_DEPTH_STENCIL_ATTACHMENT_READ = 1<<9,
    BIT_DEPTH_STENCIL_ATTACHMENT_WRITE = 1<<10,
    BIT_TRANSFER_READ = 1<<11,
    BIT_TRANSFER_WRITE = 1<<12,
    BIT_HOST_READ = 1<<13,
    BIT_HOST_WRITE = 1<<14,
    BIT_MEMORY_READ = 1<<15,
    BIT_MEMORY_WRITE = 1<<16,

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    BIT_COMMAND_PROCESS_READ_BIT_NVX = 1<<17,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    BIT_COMMAND_PROCESS_WRITE_BIT_NVX = 1<<18,

    // feature: VK_EXT_blend_operation_advanced
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    BIT_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 1<<19
  }
}

define_bitmask! {

  /// Bitmask specifying allowed usage of a buffer
  ///
  /// Bits which can: be set in `VkBufferCreateInfo::usage`, specifying usage behavior
  /// of a buffer.
  ///
  ///   - `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` specifies that the buffer can: be used as
  ///     the source of a *transfer command* (see the definition of
  ///     [`VK_PIPELINE_STAGE_TRANSFER_BIT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-stages-transfer)).
  ///
  ///   - `VK_BUFFER_USAGE_TRANSFER_DST_BIT` specifies that the buffer can: be used as
  ///     the destination of a transfer command.
  ///
  ///   - `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT` specifies that the buffer can: be
  ///     used to create a `VkBufferView` suitable for occupying a `VkDescriptorSet`
  ///     slot of type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
  ///
  ///   - `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT` specifies that the buffer can: be
  ///     used to create a `VkBufferView` suitable for occupying a `VkDescriptorSet`
  ///     slot of type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
  ///
  ///   - `VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT` specifies that the buffer can: be used
  ///     in a `VkDescriptorBufferInfo` suitable for occupying a `VkDescriptorSet`
  ///     slot either of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
  ///     `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`.
  ///
  ///   - `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT` specifies that the buffer can: be used
  ///     in a `VkDescriptorBufferInfo` suitable for occupying a `VkDescriptorSet`
  ///     slot either of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
  ///     `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`.
  ///
  ///   - `VK_BUFFER_USAGE_INDEX_BUFFER_BIT` specifies that the buffer is suitable for
  ///     passing as the `buffer` parameter to `vkCmdBindIndexBuffer`.
  ///
  ///   - `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT` specifies that the buffer is suitable
  ///     for passing as an element of the `pBuffers` array to
  ///     `vkCmdBindVertexBuffers`.
  ///
  ///   - `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` specifies that the buffer is suitable
  ///     for passing as the `buffer` parameter to `vkCmdDrawIndirect`,
  ///     `vkCmdDrawIndexedIndirect`, or `vkCmdDispatchIndirect`. It is also suitable
  ///     for passing as the `buffer` member of `VkIndirectCommandsTokenNVX`, or
  ///     `sequencesCountBuffer` or `sequencesIndexBuffer` member of
  ///     `VkCmdProcessCommandsInfoNVX`
  ///
  pub enum VkBufferUsageFlagBits {
    BIT_TRANSFER_SRC = 1<<0,
    BIT_TRANSFER_DST = 1<<1,
    BIT_UNIFORM_TEXEL_BUFFER = 1<<2,
    BIT_STORAGE_TEXEL_BUFFER = 1<<3,
    BIT_UNIFORM_BUFFER = 1<<4,
    BIT_STORAGE_BUFFER = 1<<5,
    BIT_INDEX_BUFFER = 1<<6,
    BIT_VERTEX_BUFFER = 1<<7,
    BIT_INDIRECT_BUFFER = 1<<8
  }
}

define_bitmask! {

  /// Bitmask specifying additional parameters of a buffer
  ///
  /// Bits which can: be set in `VkBufferCreateInfo::flags`, specifying additional
  /// parameters of a buffer.
  ///
  ///   - `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` specifies that the buffer will be
  ///     backed using sparse memory binding.
  ///
  ///   - `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` specifies that the buffer can: be
  ///     partially backed using sparse memory binding. Buffers created with this flag
  ///     must: also be created with the `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` flag.
  ///
  ///   - `VK_BUFFER_CREATE_SPARSE_ALIASED_BIT` specifies that the buffer will be
  ///     backed using sparse memory binding with memory ranges that might also
  ///     simultaneously be backing another buffer (or another portion of the same
  ///     buffer). Buffers created with this flag must: also be created with the
  ///     `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` flag.
  ///
  /// See [Sparse Resource Features](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory-sparseresourcefeatures) and
  /// [Physical Device Features](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-features) for details of the sparse memory
  /// features supported on a device.
  ///
  pub enum VkBufferCreateFlagBits {
    BIT_SPARSE_BINDING = 1<<0,
    BIT_SPARSE_RESIDENCY = 1<<1,
    BIT_SPARSE_ALIASED = 1<<2
  }
}

define_bitmask! {

  /// Bitmask specifying a pipeline stage
  ///
  /// Commands and structures which need to specify one or more shader stages do so
  /// using a bitmask whose bits correspond to stages. Bits which can: be set to
  /// specify shader stages are.
  ///
  ///   - `VK_SHADER_STAGE_VERTEX_BIT` specifies the vertex stage.
  ///
  ///   - `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT` specifies the tessellation
  ///     control stage.
  ///
  ///   - `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT` specifies the tessellation
  ///     evaluation stage.
  ///
  ///   - `VK_SHADER_STAGE_GEOMETRY_BIT` specifies the geometry stage.
  ///
  ///   - `VK_SHADER_STAGE_FRAGMENT_BIT` specifies the fragment stage.
  ///
  ///   - `VK_SHADER_STAGE_COMPUTE_BIT` specifies the compute stage.
  ///
  ///   - `VK_SHADER_STAGE_ALL_GRAPHICS` is a combination of bits used as shorthand to
  ///     specify all graphics stages defined above (excluding the compute stage).
  ///
  ///   - `VK_SHADER_STAGE_ALL` is a combination of bits used as shorthand to specify
  ///     all shader stages supported by the device, including all additional stages
  ///     which are introduced by extensions.
  ///
  pub enum VkShaderStageFlagBits {
    BIT_VERTEX = 1<<0,
    BIT_TESSELLATION_CONTROL = 1<<1,
    BIT_TESSELLATION_EVALUATION = 1<<2,
    BIT_GEOMETRY = 1<<3,
    BIT_FRAGMENT = 1<<4,
    BIT_COMPUTE = 1<<5,
    BIT_ALL_GRAPHICS = 0x0000001F,
    BIT_ALL = 0x7FFFFFFF
  }
}

define_bitmask! {

  /// Bitmask specifying intended usage of an image
  ///
  /// Bits which can: be set in `VkImageCreateInfo::usage`, specifying intended usage
  /// of an image.
  ///
  ///   - `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` specifies that the image can: be used as
  ///     the source of a transfer command.
  ///
  ///   - `VK_IMAGE_USAGE_TRANSFER_DST_BIT` specifies that the image can: be used as
  ///     the destination of a transfer command.
  ///
  ///   - `VK_IMAGE_USAGE_SAMPLED_BIT` specifies that the image can: be used to create
  ///     a `VkImageView` suitable for occupying a `VkDescriptorSet` slot either of
  ///     type `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` or
  ///     `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and be sampled by a shader.
  ///
  ///   - `VK_IMAGE_USAGE_STORAGE_BIT` specifies that the image can: be used to create
  ///     a `VkImageView` suitable for occupying a `VkDescriptorSet` slot of type
  ///     `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
  ///
  ///   - `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` specifies that the image can: be used
  ///     to create a `VkImageView` suitable for use as a color or resolve attachment
  ///     in a `VkFramebuffer`.
  ///
  ///   - `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` specifies that the image can:
  ///     be used to create a `VkImageView` suitable for use as a depth/stencil
  ///     attachment in a `VkFramebuffer`.
  ///
  ///   - `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT` specifies that the memory bound to
  ///     this image will have been allocated with the
  ///     `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT` (see [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#memory) for more
  ///     detail). This bit can: be set for any image that can: be used to create a
  ///     `VkImageView` suitable for use as a color, resolve, depth/stencil, or input
  ///     attachment.
  ///
  ///   - `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT` specifies that the image can: be used
  ///     to create a `VkImageView` suitable for occupying `VkDescriptorSet` slot of
  ///     type `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`; be read from a shader as an
  ///     input attachment; and be used as an input attachment in a framebuffer.
  ///
  pub enum VkImageUsageFlagBits {
    BIT_TRANSFER_SRC = 1<<0,
    BIT_TRANSFER_DST = 1<<1,
    BIT_SAMPLED = 1<<2,
    BIT_STORAGE = 1<<3,
    BIT_COLOR_ATTACHMENT = 1<<4,
    BIT_DEPTH_STENCIL_ATTACHMENT = 1<<5,
    BIT_TRANSIENT_ATTACHMENT = 1<<6,
    BIT_INPUT_ATTACHMENT = 1<<7
  }
}

define_bitmask! {

  /// Bitmask specifying additional parameters of an image
  ///
  /// Bits which can: be set in `VkImageCreateInfo::flags`, specifying additional
  /// parameters of an image.
  ///
  ///   - `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` specifies that the image will be backed
  ///     using sparse memory binding.
  ///
  ///   - `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` specifies that the image can: be
  ///     partially backed using sparse memory binding. Images created with this flag
  ///     must: also be created with the `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` flag.
  ///
  ///   - `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT` specifies that the image will be backed
  ///     using sparse memory binding with memory ranges that might also
  ///     simultaneously be backing another image (or another portion of the same
  ///     image). Images created with this flag must: also be created with the
  ///     `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` flag
  ///
  ///   - `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` specifies that the image can: be used
  ///     to create a `VkImageView` with a different format from the image. For
  ///     [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion)
  ///     formats, `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` indicates that a `VkImageView`
  ///     can be created of a *plane* of the image.
  ///
  ///   - `VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT` specifies that the image can: be used
  ///     to create a `VkImageView` of type `VK_IMAGE_VIEW_TYPE_CUBE` or
  ///     `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`.
  ///
  ///   - `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR` specifies that the image can:
  ///     be used to create a `VkImageView` of type `VK_IMAGE_VIEW_TYPE_2D` or
  ///     `VK_IMAGE_VIEW_TYPE_2D_ARRAY`.
  ///
  ///   - `VK_IMAGE_CREATE_BIND_SFR_BIT_KHX` specifies that the image can: be used
  ///     with a non-zero value of the `SFRRectCount` member of a
  ///     `VkBindImageMemoryDeviceGroupInfoKHX` structure passed into
  ///     `vkBindImageMemory2KHR`. This flag also has the effect of making the image
  ///     use the standard sparse image block dimensions.
  ///
  ///   - `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR` indicates that the
  ///     image having a compressed format can: be used to create a `VkImageView` with
  ///     an uncompressed format where each texel in the image view corresponds to a
  ///     compressed texel block of the image.
  ///
  ///   - `VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR` indicates that the image can: be
  ///     created with usage flags that are not supported for the format the image is
  ///     created with but are supported for at least one format a `VkImageView`
  ///     created from the image can: have.
  ///
  ///   - `VK_IMAGE_CREATE_DISJOINT_BIT_KHR` indicates that an image with a
  ///     [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-requiring-sampler-ycbcr-conversion)
  ///     must: have each plane separately bound to memory, rather than having a
  ///     single memory binding for the whole image; the presence of this bit
  ///     distinguishes a *disjoint image* from an image without this bit set.
  ///
  ///   - `VK_IMAGE_CREATE_ALIAS_BIT_KHR` indicates that two images created with the
  ///     same creation parameters and aliased to the same memory can: interpret the
  ///     contents of the memory consistently with each other, subject to the rules
  ///     described in the [Memory Aliasing](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#resources-memory-aliasing) section. This
  ///     flag further indicates that each plane of a *disjoint* image can: share an
  ///     in-memory non-linear representation with single-plane images, and that a
  ///     single-plane image can: share an in-memory non-linear representation with a
  ///     plane of a multi-planar disjoint image, according to the rules in
  ///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#features-formats-compatible-planes).
  ///
  ///   - `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` specifies that
  ///     an image with a depth or depth/stencil format can: be used with custom
  ///     sample locations when used as a depth/stencil attachment.
  ///
  /// If any of the bits `VK_IMAGE_CREATE_SPARSE_BINDING_BIT`,
  /// `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT`, or `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT`
  /// are set, `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT` must: not also be set.
  ///
  /// See [Sparse Resource Features](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory-sparseresourcefeatures) and [Sparse
  /// Physical Device Features](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory-physicalfeatures) for more details.
  ///
  pub enum VkImageCreateFlagBits {
    BIT_SPARSE_BINDING = 1<<0,
    BIT_SPARSE_RESIDENCY = 1<<1,
    BIT_SPARSE_ALIASED = 1<<2,
    BIT_MUTABLE_FORMAT = 1<<3,
    BIT_CUBE_COMPATIBLE = 1<<4,

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    BIT_BIND_SFR_BIT_KHX = 1<<6,

    // feature: VK_KHR_maintenance1
    #[cfg(feature = "VK_KHR_maintenance1")]
    BIT_2D_ARRAY_COMPATIBLE_BIT_KHR = 1<<5,

    // feature: VK_KHR_maintenance2
    #[cfg(feature = "VK_KHR_maintenance2")]
    BIT_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR = 1<<7,
    #[cfg(feature = "VK_KHR_maintenance2")]
    BIT_EXTENDED_USAGE_BIT_KHR = 1<<8,

    // feature: VK_EXT_sample_locations
    #[cfg(feature = "VK_EXT_sample_locations")]
    BIT_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 1<<12,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_DISJOINT_BIT_KHR = 1<<9,

    // feature: VK_KHR_bind_memory2
    #[cfg(feature = "VK_KHR_bind_memory2")]
    BIT_ALIAS_BIT_KHR = 1<<10
  }
}

define_bitmask! {

  /// Bitmask controlling how a pipeline is created
  ///
  /// Possible values of the `flags` member of `VkGraphicsPipelineCreateInfo` and
  /// `VkComputePipelineCreateInfo`, specifying how a pipeline is created.
  ///
  ///   - `VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT` specifies that the created
  ///     pipeline will not be optimized. Using this flag may: reduce the time taken
  ///     to create the pipeline.
  ///
  ///   - `VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT` specifies that the pipeline to be
  ///     created is allowed to be the parent of a pipeline that will be created in a
  ///     subsequent call to `vkCreateGraphicsPipelines` or
  ///     `vkCreateComputePipelines`.
  ///
  ///   - `VK_PIPELINE_CREATE_DERIVATIVE_BIT` specifies that the pipeline to be
  ///     created will be a child of a previously created parent pipeline.
  ///
  ///   - `VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHX` specifies that any
  ///     shader input variables decorated as `DeviceIndex` will be assigned values as
  ///     if they were decorated as `ViewIndex`.
  ///
  ///   - `VK_PIPELINE_CREATE_DISPATCH_BASE_KHX` specifies that a compute pipeline
  ///     can: be used with `vkCmdDispatchBaseKHX` with a non-zero base workgroup.
  ///
  /// It is valid to set both `VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT` and
  /// `VK_PIPELINE_CREATE_DERIVATIVE_BIT`. This allows a pipeline to be both a parent
  /// and possibly a child in a pipeline hierarchy. See [Pipeline
  /// Derivatives](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#pipelines-pipeline-derivatives) for more information.
  ///
  pub enum VkPipelineCreateFlagBits {
    BIT_DISABLE_OPTIMIZATION = 1<<0,
    BIT_ALLOW_DERIVATIVES = 1<<1,
    BIT_DERIVATIVE = 1<<2,

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    BIT_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHX = 1<<3,
    #[cfg(feature = "VK_KHX_device_group")]
    BIT_DISPATCH_BASE_KHX = 1<<4
  }
}

define_bitmask! {

  /// Bitmask controlling which components are written to the framebuffer
  ///
  /// Bits which can: be set in `VkPipelineColorBlendAttachmentState::colorWriteMask`
  /// to determine whether the final color values R, G, B and A are written to the
  /// framebuffer attachment are.
  ///
  ///   - `VK_COLOR_COMPONENT_R_BIT` specifies that the R value is written to the
  ///     color attachment for the appropriate sample. Otherwise, the value in memory
  ///     is unmodified.
  ///
  ///   - `VK_COLOR_COMPONENT_G_BIT` specifies that the G value is written to the
  ///     color attachment for the appropriate sample. Otherwise, the value in memory
  ///     is unmodified.
  ///
  ///   - `VK_COLOR_COMPONENT_B_BIT` specifies that the B value is written to the
  ///     color attachment for the appropriate sample. Otherwise, the value in memory
  ///     is unmodified.
  ///
  ///   - `VK_COLOR_COMPONENT_A_BIT` specifies that the A value is written to the
  ///     color attachment for the appropriate sample. Otherwise, the value in memory
  ///     is unmodified.
  ///
  /// The color write mask operation is applied regardless of whether blending is
  /// enabled.
  ///
  pub enum VkColorComponentFlagBits {
    BIT_R = 1<<0,
    BIT_G = 1<<1,
    BIT_B = 1<<2,
    BIT_A = 1<<3
  }
}

define_bitmask! {

  /// Bitmask specifying initial state and behavior of a fence
  ///
  ///   - `VK_FENCE_CREATE_SIGNALED_BIT` specifies that the fence object is created in
  ///     the signaled state. Otherwise, it is created in the unsignaled state.
  ///
  pub enum VkFenceCreateFlagBits {
    BIT_SIGNALED = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying features supported by a buffer
  ///
  /// Bits which can: be set in the `VkFormatProperties` features
  /// `linearTilingFeatures`, `optimalTilingFeatures`, and `bufferFeatures` are.
  ///
  /// The following bits may: be set in `linearTilingFeatures` and
  /// `optimalTilingFeatures`, specifying that the features are supported by
  /// [images](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImage) or [image views](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageView) created with the queried
  /// `vkGetPhysicalDeviceFormatProperties::format`:
  ///
  ///   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT` specifies that an image view can: be
  ///     [sampled from](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-sampledimage).
  ///
  ///   - `VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT` specifies that an image view can: be
  ///     used as a [storage images](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#descriptorsets-storageimage).
  ///
  ///   - `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT` specifies that an image view
  ///     can: be used as storage image that supports atomic operations.
  ///
  ///   - `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` specifies that an image view can:
  ///     be used as a framebuffer color attachment and as an input attachment.
  ///
  ///   - `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT` specifies that an image view
  ///     can: be used as a framebuffer color attachment that supports blending and as
  ///     an input attachment.
  ///
  ///   - `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT` specifies that an image
  ///     view can: be used as a framebuffer depth/stencil attachment and as an input
  ///     attachment.
  ///
  ///   - `VK_FORMAT_FEATURE_BLIT_SRC_BIT` specifies that an image can: be used as
  ///     `srcImage` for the `vkCmdBlitImage` command.
  ///
  ///   - `VK_FORMAT_FEATURE_BLIT_DST_BIT` specifies that an image can: be used as
  ///     `dstImage` for the `vkCmdBlitImage` command.
  ///
  ///   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` specifies that if
  ///     `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT` is also set, an image view can: be
  ///     used with a sampler that has either of `magFilter` or `minFilter` set to
  ///     `VK_FILTER_LINEAR`, or `mipmapMode` set to `VK_SAMPLER_MIPMAP_MODE_LINEAR`.
  ///     If `VK_FORMAT_FEATURE_BLIT_SRC_BIT` is also set, an image can be used as the
  ///     `srcImage` to `vkCmdBlitImage` with a `filter` of `VK_FILTER_LINEAR`. This
  ///     bit must: only be exposed for formats that also support the
  ///     `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT` or `VK_FORMAT_FEATURE_BLIT_SRC_BIT`.
  ///
  ///     If the format being queried is a depth/stencil format, this bit only
  ///     indicates that the depth aspect (not the stencil aspect) of an image of this
  ///     format supports linear filtering, and that linear filtering of the depth
  ///     aspect is supported whether depth compare is enabled in the sampler or not.
  ///     If this bit is not present, linear filtering with depth compare disabled is
  ///     unsupported and linear filtering with depth compare enabled is supported,
  ///     but may: compute the filtered value in an implementation-dependent manner
  ///     which differs from the normal rules of linear filtering. The resulting value
  ///     must: be in the range \[0,1\] and should: be proportional to, or a weighted
  ///     average of, the number of comparison passes or failures.
  ///
  ///   - `VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR` specifies that an image can: be
  ///     used as a source image for [copy commands](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#copies).
  ///
  ///   - `VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR` specifies that an image can: be
  ///     used as a destination image for [copy commands](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#copies) and [clear
  ///     commands](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#clears).
  ///
  ///   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT` specifies `VkImage`
  ///     can: be used as a sampled image with a min or max
  ///     `VkSamplerReductionModeEXT`. This bit must: only be exposed for formats that
  ///     also support the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`.
  ///
  ///   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG` specifies that
  ///     `VkImage` can: be used with a sampler that has either of `magFilter` or
  ///     `minFilter` set to `VK_FILTER_CUBIC_IMG`, or be the source image for a blit
  ///     with `filter` set to `VK_FILTER_CUBIC_IMG`. This bit must: only be exposed
  ///     for formats that also support the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`. If
  ///     the format being queried is a depth/stencil format, this only indicates that
  ///     the depth aspect is cubic filterable.
  ///
  ///   - `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR` specifies that an
  ///     application can: define a [sampler Y’C<sub>B</sub>C<sub>R</sub>
  ///     conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source, and
  ///     that an image of this format can: be used with a
  ///     `VkSamplerYcbcrConversionCreateInfoKHR` `xChromaOffset` and/or
  ///     `yChromaOffset` of `VK_CHROMA_LOCATION_MIDPOINT_KHR`. Otherwise both
  ///     `xChromaOffset` and `yChromaOffset` must: be
  ///     `VK_CHROMA_LOCATION_COSITED_EVEN_KHR`. If a format does not incorporate
  ///     chroma downsampling (it is not a “422” or “420” format) but the
  ///     implementation supports sampler Y’C<sub>B</sub>C<sub>R</sub> conversion for
  ///     this format, the implementation must: set
  ///     `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR`.
  ///
  ///   - `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR` specifies that an
  ///     application can: define a [sampler Y’C<sub>B</sub>C<sub>R</sub>
  ///     conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source, and
  ///     that an image of this format can: be used with a
  ///     `VkSamplerYcbcrConversionCreateInfoKHR` `xChromaOffset` and/or
  ///     `yChromaOffset` of `VK_CHROMA_LOCATION_COSITED_EVEN_KHR`. Otherwise both
  ///     `xChromaOffset` and `yChromaOffset` must: be
  ///     `VK_CHROMA_LOCATION_MIDPOINT_KHR`. If neither
  ///     `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR` nor
  ///     `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR` is set, the application
  ///     must: not define a [sampler Y’C<sub>B</sub>C<sub>R</sub>
  ///     conversion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source.
  ///
  ///   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR`
  ///     specifies that the format can do linear sampler filtering (min/magFilter)
  ///     whilst sampler Y’C<sub>B</sub>C<sub>R</sub> conversion is
  ///     enabled.
  ///
  ///   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR`
  ///     specifies that the format can have different chroma, min, and mag
  ///     filters.
  ///
  ///   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR`
  ///     specifies that reconstruction is explicit, as described in
  ///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-chroma-reconstruction). If this bit is not present,
  ///     reconstruction is implicit by
  ///     default.
  ///
  ///   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR`
  ///     specifies that reconstruction can: be forcibly made explicit by setting
  ///     `VkSamplerYcbcrConversionCreateInfoKHR::forceExplicitReconstruction` to
  ///     `VK_TRUE`.
  ///
  ///   - `VK_FORMAT_FEATURE_DISJOINT_BIT_KHR` specifies that a multi-planar image
  ///     can: have the `VK_IMAGE_CREATE_DISJOINT_BIT_KHR` set during image creation.
  ///     An implementation must: not set `VK_FORMAT_FEATURE_DISJOINT_BIT_KHR` for
  ///     *single-plane formats*.
  ///
  /// The following bits may: be set in `bufferFeatures`, specifying that the features
  /// are supported by [buffers](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBuffer) or [buffer views](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferView) created
  /// with the queried `vkGetPhysicalDeviceProperties::format`:
  ///
  ///   - `VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT` specifies that the format can:
  ///     be used to create a buffer view that can: be bound to a
  ///     `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` descriptor.
  ///
  ///   - `VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT` specifies that the format can:
  ///     be used to create a buffer view that can: be bound to a
  ///     `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` descriptor.
  ///
  ///   - `VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT` specifies that atomic
  ///     operations are supported on `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` with
  ///     this format.
  ///
  ///   - `VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT` specifies that the format can: be used
  ///     as a vertex attribute format (`VkVertexInputAttributeDescription::format`).
  ///
  pub enum VkFormatFeatureFlagBits {
    BIT_SAMPLED_IMAGE = 1<<0,
    BIT_STORAGE_IMAGE = 1<<1,
    BIT_STORAGE_IMAGE_ATOMIC = 1<<2,
    BIT_UNIFORM_TEXEL_BUFFER = 1<<3,
    BIT_STORAGE_TEXEL_BUFFER = 1<<4,
    BIT_STORAGE_TEXEL_BUFFER_ATOMIC = 1<<5,
    BIT_VERTEX_BUFFER = 1<<6,
    BIT_COLOR_ATTACHMENT = 1<<7,
    BIT_COLOR_ATTACHMENT_BLEND = 1<<8,
    BIT_DEPTH_STENCIL_ATTACHMENT = 1<<9,
    BIT_BLIT_SRC = 1<<10,
    BIT_BLIT_DST = 1<<11,
    BIT_SAMPLED_IMAGE_FILTER_LINEAR = 1<<12,

    // feature: VK_IMG_filter_cubic
    #[cfg(feature = "VK_IMG_filter_cubic")]
    BIT_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 1<<13,

    // feature: VK_KHR_maintenance1
    #[cfg(feature = "VK_KHR_maintenance1")]
    BIT_TRANSFER_SRC_BIT_KHR = 1<<14,
    #[cfg(feature = "VK_KHR_maintenance1")]
    BIT_TRANSFER_DST_BIT_KHR = 1<<15,

    // feature: VK_EXT_sampler_filter_minmax
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    BIT_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT = 1<<16,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_MIDPOINT_CHROMA_SAMPLES_BIT_KHR = 1<<17,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR = 1<<18,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR = 1<<19,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR = 1<<20,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR = 1<<21,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_DISJOINT_BIT_KHR = 1<<22,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_COSITED_CHROMA_SAMPLES_BIT_KHR = 1<<23
  }
}

define_bitmask! {

  /// Bitmask specifying constraints on a query
  ///
  /// Bits which can: be set in `vkCmdBeginQuery::flags`, specifying constraints on
  /// the types of queries that can: be performed.
  ///
  ///   - `VK_QUERY_CONTROL_PRECISE_BIT` specifies the precision of [occlusion
  ///     queries](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-occlusion).
  ///
  pub enum VkQueryControlFlagBits {
    BIT_PRECISE = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying how and when query results are returned
  ///
  /// Bits which can: be set in `vkGetQueryPoolResults::flags` and
  /// `vkCmdCopyQueryPoolResults::flags`, specifying how and when results are
  /// returned.
  ///
  ///   - `VK_QUERY_RESULT_64_BIT` specifies the results will be written as an array
  ///     of 64-bit unsigned integer values. If this bit is not set, the results will
  ///     be written as an array of 32-bit unsigned integer values.
  ///
  ///   - `VK_QUERY_RESULT_WAIT_BIT` specifies that Vulkan will wait for each query’s
  ///     status to become available before retrieving its results.
  ///
  ///   - `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` specifies that the availability
  ///     status accompanies the results.
  ///
  ///   - `VK_QUERY_RESULT_PARTIAL_BIT` specifies that returning partial results is
  ///     acceptable.
  ///
  pub enum VkQueryResultFlagBits {
    BIT_64 = 1<<0,
    BIT_WAIT = 1<<1,
    BIT_WITH_AVAILABILITY = 1<<2,
    BIT_PARTIAL = 1<<3
  }
}

define_bitmask! {

  /// Bitmask specifying usage behavior for command buffer
  ///
  /// Bits which can: be set in `VkCommandBufferBeginInfo::flags` to specify usage
  /// behavior for a command buffer are.
  ///
  ///   - `VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT` specifies that each recording
  ///     of the command buffer will only be submitted once, and the command buffer
  ///     will be reset and recorded again between each submission.
  ///
  ///   - `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` specifies that a
  ///     secondary command buffer is considered to be entirely inside a render pass.
  ///     If this is a primary command buffer, then this bit is ignored.
  ///
  ///   - `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT` specifies that a command
  ///     buffer can: be resubmitted to a queue while it is in the *pending state*,
  ///     and recorded into multiple primary command buffers.
  ///
  pub enum VkCommandBufferUsageFlagBits {
    BIT_ONE_TIME_SUBMIT = 1<<0,
    BIT_RENDER_PASS_CONTINUE = 1<<1,
    BIT_SIMULTANEOUS_USE = 1<<2
  }
}

define_bitmask! {

  /// Bitmask specifying queried pipeline statistics
  ///
  /// Bits which can: be set to individually enable pipeline statistics counters for
  /// query pools with `VkQueryPoolCreateInfo::pipelineStatistics`, and for secondary
  /// command buffers with `VkCommandBufferInheritanceInfo::pipelineStatistics`.
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT` specifies that
  ///     queries managed by the pool will count the number of vertices processed by
  ///     the [input assembly](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#drawing) stage. Vertices corresponding to incomplete
  ///     primitives may: contribute to the count.
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT` specifies that
  ///     queries managed by the pool will count the number of primitives processed by
  ///     the [input assembly](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#drawing) stage. If primitive restart is enabled,
  ///     restarting the primitive topology has no effect on the count. Incomplete
  ///     primitives may: be counted.
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT` specifies that
  ///     queries managed by the pool will count the number of vertex shader
  ///     invocations. This counter’s value is incremented each time a vertex shader
  ///     is [invoked](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#shaders-vertex-execution).
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT` specifies that
  ///     queries managed by the pool will count the number of geometry shader
  ///     invocations. This counter’s value is incremented each time a geometry shader
  ///     is [invoked](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#shaders-geometry-execution). In the case of [instanced
  ///     geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#geometry-invocations), the geometry shader invocations
  ///     count is incremented for each separate instanced invocation.
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT` specifies that
  ///     queries managed by the pool will count the number of primitives generated by
  ///     geometry shader invocations. The counter’s value is incremented each time
  ///     the geometry shader emits a primitive. Restarting primitive topology using
  ///     the SPIR-V instructions `OpEndPrimitive` or `OpEndStreamPrimitive` has no
  ///     effect on the geometry shader output primitives count.
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT` specifies that
  ///     queries managed by the pool will count the number of primitives processed by
  ///     the [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vertexpostproc-clipping) stage of the pipeline.
  ///     The counter’s value is incremented each time a primitive reaches the
  ///     primitive clipping stage.
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT` specifies that queries
  ///     managed by the pool will count the number of primitives output by the
  ///     [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vertexpostproc-clipping) stage of the pipeline. The
  ///     counter’s value is incremented each time a primitive passes the primitive
  ///     clipping stage. The actual number of primitives output by the primitive
  ///     clipping stage for a particular input primitive is implementation-dependent
  ///     but must: satisfy the following conditions:
  ///
  ///       - If at least one vertex of the input primitive lies inside the clipping
  ///         volume, the counter is incremented by one or more.
  ///
  ///       - Otherwise, the counter is incremented by zero or more.
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT` specifies that
  ///     queries managed by the pool will count the number of fragment shader
  ///     invocations. The counter’s value is incremented each time the fragment
  ///     shader is [invoked](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#shaders-fragment-execution).
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT`
  ///     specifies that queries managed by the pool will count the number of patches
  ///     processed by the tessellation control shader. The counter’s value is
  ///     incremented once for each patch for which a tessellation control shader is
  ///     [invoked](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#shaders-tessellation-control-execution).
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT`
  ///     specifies that queries managed by the pool will count the number of
  ///     invocations of the tessellation evaluation shader. The counter’s value is
  ///     incremented each time the tessellation evaluation shader is
  ///     [invoked](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#shaders-tessellation-evaluation-execution).
  ///
  ///   - `VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT` specifies that
  ///     queries managed by the pool will count the number of compute shader
  ///     invocations. The counter’s value is incremented every time the compute
  ///     shader is invoked. Implementations may: skip the execution of certain
  ///     compute shader invocations or execute additional compute shader invocations
  ///     for implementation-dependent reasons as long as the results of rendering
  ///     otherwise remain unchanged.
  ///
  /// These values are intended to measure relative statistics on one implementation.
  /// Various device architectures will count these values differently. Any or all
  /// counters may: be affected by the issues described in [Query
  /// Operation](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#queries-operation-undefined).
  ///
  /// > **Note**
  /// >
  /// > For example, tile-based rendering devices may: need to replay the scene
  /// > multiple times, affecting some of the counts.
  ///
  /// If a pipeline has `rasterizerDiscardEnable` enabled, implementations may:
  /// discard primitives after the final vertex processing stage. As a result, if
  /// `rasterizerDiscardEnable` is enabled, the clipping input and output primitives
  /// counters may: not be incremented.
  ///
  /// When a pipeline statistics query finishes, the result for that query is marked
  /// as available. The application can: copy the result to a buffer (via
  /// `vkCmdCopyQueryPoolResults`), or request it be put into host memory (via
  /// `vkGetQueryPoolResults`).
  ///
  pub enum VkQueryPipelineStatisticFlagBits {
    BIT_INPUT_ASSEMBLY_VERTICES = 1<<0,
    BIT_INPUT_ASSEMBLY_PRIMITIVES = 1<<1,
    BIT_VERTEX_SHADER_INVOCATIONS = 1<<2,
    BIT_GEOMETRY_SHADER_INVOCATIONS = 1<<3,
    BIT_GEOMETRY_SHADER_PRIMITIVES = 1<<4,
    BIT_CLIPPING_INVOCATIONS = 1<<5,
    BIT_CLIPPING_PRIMITIVES = 1<<6,
    BIT_FRAGMENT_SHADER_INVOCATIONS = 1<<7,
    BIT_TESSELLATION_CONTROL_SHADER_PATCHES = 1<<8,
    BIT_TESSELLATION_EVALUATION_SHADER_INVOCATIONS = 1<<9,
    BIT_COMPUTE_SHADER_INVOCATIONS = 1<<10
  }
}

define_bitmask! {

  /// Bitmask specifying which aspects of an image are included in a view
  ///
  /// Bits which can: be set in an aspect mask to specify aspects of an image for
  /// purposes such as identifying a subresource.
  ///
  ///   - `VK_IMAGE_ASPECT_COLOR_BIT` specifies the color aspect.
  ///
  ///   - `VK_IMAGE_ASPECT_DEPTH_BIT` specifies the depth aspect.
  ///
  ///   - `VK_IMAGE_ASPECT_STENCIL_BIT` specifies the stencil aspect.
  ///
  ///   - `VK_IMAGE_ASPECT_METADATA_BIT` specifies the metadata aspect, used for
  ///     sparse [sparse resource](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#sparsememory) operations.
  ///
  pub enum VkImageAspectFlagBits {
    BIT_COLOR = 1<<0,
    BIT_DEPTH = 1<<1,
    BIT_STENCIL = 1<<2,
    BIT_METADATA = 1<<3,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_PLANE_0_BIT_KHR = 1<<4,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_PLANE_1_BIT_KHR = 1<<5,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIT_PLANE_2_BIT_KHR = 1<<6
  }
}

define_bitmask! {

  /// Bitmask specifying additional information about a sparse image resource
  ///
  /// Bits which can: be set in `VkSparseImageFormatProperties::flags`, specifying
  /// additional information about the sparse resource.
  ///
  ///   - `VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT` specifies that the image uses a
  ///     single mip tail region for all array layers.
  ///
  ///   - `VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT` specifies that the first mip
  ///     level whose dimensions are not integer multiples of the corresponding
  ///     dimensions of the sparse image block begins the mip tail region.
  ///
  ///   - `VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT` specifies that the image
  ///     uses non-standard sparse image block dimensions, and the `imageGranularity`
  ///     values do not match the standard sparse image block dimensions for the given
  ///     format.
  ///
  pub enum VkSparseImageFormatFlagBits {
    BIT_SINGLE_MIPTAIL = 1<<0,
    BIT_ALIGNED_MIP_SIZE = 1<<1,
    BIT_NONSTANDARD_BLOCK_SIZE = 1<<2
  }
}

define_bitmask! {

  /// Bitmask specifying usage of a sparse memory binding operation
  ///
  /// Bits which can: be set in `VkSparseMemoryBind::flags`, specifying usage of a
  /// sparse memory binding operation.
  ///
  ///   - `VK_SPARSE_MEMORY_BIND_METADATA_BIT` specifies that the memory being bound
  ///     is only for the metadata aspect.
  ///
  pub enum VkSparseMemoryBindFlagBits {
    BIT_METADATA = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying pipeline stages
  ///
  /// Several of the synchronization commands include pipeline stage parameters,
  /// restricting the [synchronization scopes](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-dependencies-scopes)
  /// for that command to just those stages. This allows fine grained control over the
  /// exact execution dependencies and accesses performed by action commands.
  /// Implementations should: use these pipeline stages to avoid unnecessary stalls or
  /// cache flushing.
  ///
  /// Bits which can be set, specifying pipeline stages.
  ///
  ///   - `VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT` specifies the stage of the pipeline
  ///     where any commands are initially received by the queue.
  ///
  ///   - `VK_PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX` specifies the stage of the
  ///     pipeline where device-side generation of commands via
  ///     `vkCmdProcessCommandsNVX` is handled.
  ///
  ///   - `VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT` specifies the stage of the pipeline
  ///     where Draw/DispatchIndirect data structures are consumed. This stage also
  ///     includes reading commands written by `vkCmdProcessCommandsNVX`.
  ///
  ///   - `VK_PIPELINE_STAGE_VERTEX_INPUT_BIT` specifies the stage of the pipeline
  ///     where vertex and index buffers are consumed.
  ///
  ///   - `VK_PIPELINE_STAGE_VERTEX_SHADER_BIT` specifies the vertex shader stage.
  ///
  ///   - `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` specifies the
  ///     tessellation control shader stage.
  ///
  ///   - `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT` specifies the
  ///     tessellation evaluation shader stage.
  ///
  ///   - `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT` specifies the geometry shader stage.
  ///
  ///   - `VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT` specifies the fragment shader stage.
  ///
  ///   - `VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT` specifies the stage of the
  ///     pipeline where early fragment tests (depth and stencil tests before fragment
  ///     shading) are performed. This stage also includes [subpass load
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops) for framebuffer attachments with a
  ///     depth/stencil format.
  ///
  ///   - `VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT` specifies the stage of the
  ///     pipeline where late fragment tests (depth and stencil tests after fragment
  ///     shading) are performed. This stage also includes [subpass store
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops) for framebuffer attachments with a
  ///     depth/stencil format.
  ///
  ///   - `VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` specifies the stage of the
  ///     pipeline after blending where the final color values are output from the
  ///     pipeline. This stage also includes [subpass load and store
  ///     operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#renderpass-load-store-ops) and multisample resolve operations
  ///     for framebuffer attachments with a color format.
  ///
  ///   - `VK_PIPELINE_STAGE_TRANSFER_BIT` specifies the execution of copy commands.
  ///     This includes the operations resulting from all [copy commands](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#copies),
  ///     [clear commands](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#clears) (with the exception of `vkCmdClearAttachments`),
  ///     and `vkCmdCopyQueryPoolResults`.
  ///
  ///   - `VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT` specifies the execution of a compute
  ///     shader.
  ///
  ///   - `VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT` specifies the final stage in the
  ///     pipeline where operations generated by all commands complete execution.
  ///
  ///   - `VK_PIPELINE_STAGE_HOST_BIT` specifies a pseudo-stage indicating execution
  ///     on the host of reads/writes of device memory. This stage is not invoked by
  ///     any commands recorded in a command buffer.
  ///
  ///   - `VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT` specifies the execution of all graphics
  ///     pipeline stages, and is equivalent to the logical OR of:
  ///
  ///       - `VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_VERTEX_INPUT_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_VERTEX_SHADER_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT`
  ///
  ///       - `VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT`
  ///
  ///   - `VK_PIPELINE_STAGE_ALL_COMMANDS_BIT` is equivalent to the logical OR of
  ///     every other pipeline stage flag that is supported on the queue it is used
  ///     with.
  ///
  /// > **Note**
  /// >
  /// > An execution dependency with only `VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT` in
  /// > the destination stage mask will only prevent that stage from executing in
  /// > subsequently submitted commands. As this stage does not perform any actual
  /// > execution, this is not observable - in effect, it does not delay processing of
  /// > subsequent commands. Similarly an execution dependency with only
  /// > `VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT` in the source stage mask will effectively
  /// > not wait for any prior commands to complete.
  /// >
  /// > When defining a memory dependency, using only
  /// > `VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT` or `VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT`
  /// > would never make any accesses available and/or visible because these stages do
  /// > not access memory.
  /// >
  /// > `VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT` and `VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT`
  /// > are useful for accomplishing layout transitions and queue ownership operations
  /// > when the required execution dependency is satisfied by other means - for
  /// > example, semaphore operations between queues.
  ///
  pub enum VkPipelineStageFlagBits {
    BIT_TOP_OF_PIPE = 1<<0,
    BIT_DRAW_INDIRECT = 1<<1,
    BIT_VERTEX_INPUT = 1<<2,
    BIT_VERTEX_SHADER = 1<<3,
    BIT_TESSELLATION_CONTROL_SHADER = 1<<4,
    BIT_TESSELLATION_EVALUATION_SHADER = 1<<5,
    BIT_GEOMETRY_SHADER = 1<<6,
    BIT_FRAGMENT_SHADER = 1<<7,
    BIT_EARLY_FRAGMENT_TESTS = 1<<8,
    BIT_LATE_FRAGMENT_TESTS = 1<<9,
    BIT_COLOR_ATTACHMENT_OUTPUT = 1<<10,
    BIT_COMPUTE_SHADER = 1<<11,
    BIT_TRANSFER = 1<<12,
    BIT_BOTTOM_OF_PIPE = 1<<13,
    BIT_HOST = 1<<14,
    BIT_ALL_GRAPHICS = 1<<15,
    BIT_ALL_COMMANDS = 1<<16,

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    BIT_COMMAND_PROCESS_BIT_NVX = 1<<17
  }
}

define_bitmask! {

  /// Bitmask specifying usage behavior for a command pool
  ///
  /// Bits which can: be set in `VkCommandPoolCreateInfo::flags` to specify usage
  /// behavior for a command pool are.
  ///
  ///   - `VK_COMMAND_POOL_CREATE_TRANSIENT_BIT` indicates that command buffers
  ///     allocated from the pool will be short-lived, meaning that they will be reset
  ///     or freed in a relatively short timeframe. This flag may: be used by the
  ///     implementation to control memory allocation behavior within the pool.
  ///
  ///   - `VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT` allows any command buffer
  ///     allocated from a pool to be individually reset to the [initial
  ///     state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle); either by calling `vkResetCommandBuffer`,
  ///     or via the implicit reset when calling `vkBeginCommandBuffer`. If this flag
  ///     is not set on a pool, then `vkResetCommandBuffer` must: not be called for
  ///     any command buffer allocated from that pool.
  ///
  pub enum VkCommandPoolCreateFlagBits {
    BIT_TRANSIENT = 1<<0,
    BIT_RESET_COMMAND_BUFFER = 1<<1
  }
}

define_bitmask! {

  /// Bitmask controlling behavior of a command pool reset
  ///
  /// Bits which can: be set in `vkResetCommandPool::flags` to control the reset
  /// operation are.
  ///
  ///   - `VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT` specifies that resetting a
  ///     command pool recycles all of the resources from the command pool back to the
  ///     system.
  ///
  pub enum VkCommandPoolResetFlagBits {
    BIT_RELEASE_RESOURCES = 1<<0
  }
}

define_bitmask! {

  /// Bitmask controlling behavior of a command buffer reset
  ///
  /// Bits which can: be set in `vkResetCommandBuffer::flags` to control the reset
  /// operation are.
  ///
  ///   - `VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT` specifies that most or all
  ///     memory resources currently owned by the command buffer should: be returned
  ///     to the parent command pool. If this flag is not set, then the command buffer
  ///     may: hold onto memory resources and reuse them when recording commands.
  ///     `commandBuffer` is moved to the [initial state](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#commandbuffers-lifecycle).
  ///
  pub enum VkCommandBufferResetFlagBits {
    BIT_RELEASE_RESOURCES = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying sample counts supported for an image used for storage
  /// operations
  ///
  /// Bits which may: be set in the sample count limits returned by
  /// `VkPhysicalDeviceLimits`, as well as in other queries and structures
  /// representing image sample counts.
  ///
  ///   - `VK_SAMPLE_COUNT_1_BIT` specifies an image with one sample per pixel.
  ///
  ///   - `VK_SAMPLE_COUNT_2_BIT` specifies an image with 2 samples per pixel.
  ///
  ///   - `VK_SAMPLE_COUNT_4_BIT` specifies an image with 4 samples per pixel.
  ///
  ///   - `VK_SAMPLE_COUNT_8_BIT` specifies an image with 8 samples per pixel.
  ///
  ///   - `VK_SAMPLE_COUNT_16_BIT` specifies an image with 16 samples per pixel.
  ///
  ///   - `VK_SAMPLE_COUNT_32_BIT` specifies an image with 32 samples per pixel.
  ///
  ///   - `VK_SAMPLE_COUNT_64_BIT` specifies an image with 64 samples per pixel.
  ///
  pub enum VkSampleCountFlagBits {
    BIT_1 = 1<<0,
    BIT_2 = 1<<1,
    BIT_4 = 1<<2,
    BIT_8 = 1<<3,
    BIT_16 = 1<<4,
    BIT_32 = 1<<5,
    BIT_64 = 1<<6
  }
}

define_bitmask! {

  /// Bitmask specifying additional properties of an attachment
  ///
  /// Bits which can: be set in `VkAttachmentDescription::flags` describing additional
  /// properties of the attachment are.
  ///
  ///   - `VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT` specifies that the attachment
  ///     aliases the same device memory as other attachments.
  ///
  pub enum VkAttachmentDescriptionFlagBits {
    BIT_MAY_ALIAS = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying sets of stencil state for which to update the compare mask
  ///
  /// Bits which can: be set in the `vkCmdSetStencilCompareMask::faceMask` parameter,
  /// and similar parameters of other commands specifying which stencil state to
  /// update stencil masks for.
  ///
  ///   - `VK_STENCIL_FACE_FRONT_BIT` specifies that only the front set of stencil
  ///     state is updated.
  ///
  ///   - `VK_STENCIL_FACE_BACK_BIT` specifies that only the back set of stencil state
  ///     is updated.
  ///
  ///   - `VK_STENCIL_FRONT_AND_BACK` is the combination of
  ///     `VK_STENCIL_FACE_FRONT_BIT` and `VK_STENCIL_FACE_BACK_BIT`, and specifies
  ///     that both sets of stencil state are updated.
  ///
  pub enum VkStencilFaceFlagBits {
    BIT_FACE_FRONT = 1<<0,
    BIT_FACE_BACK = 1<<1,
    BIT_FRONT_AND_BACK = 0x00000003
  }
}

define_bitmask! {

  /// Bitmask specifying certain supported operations on a descriptor pool
  ///
  /// Bits which can: be set in `VkDescriptorPoolCreateInfo::flags` to enable
  /// operations on a descriptor pool are.
  ///
  ///   - `VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT` specifies that
  ///     descriptor sets can: return their individual allocations to the pool, i.e.
  ///     all of `vkAllocateDescriptorSets`, `vkFreeDescriptorSets`, and
  ///     `vkResetDescriptorPool` are allowed. Otherwise, descriptor sets allocated
  ///     from the pool must: not be individually freed back to the pool, i.e. only
  ///     `vkAllocateDescriptorSets` and `vkResetDescriptorPool` are allowed.
  ///
  pub enum VkDescriptorPoolCreateFlagBits {
    BIT_FREE_DESCRIPTOR_SET = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying how execution and memory dependencies are formed
  ///
  /// Bits which can: be set in vkCmdPipelineBarrier::\`dependencyFlags\`, specifying
  /// how execution and memory dependencies are formed.
  ///
  ///   - `VK_DEPENDENCY_BY_REGION_BIT` specifies that dependencies will be
  ///     [framebuffer-local](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-framebuffer-regions).
  ///
  ///   - `VK_DEPENDENCY_VIEW_LOCAL_BIT_KHX` specifies that a [subpass has more than
  ///     one view](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-pipeline-barriers-subpass-self-dependencies).
  ///
  ///   - `VK_DEPENDENCY_DEVICE_GROUP_BIT_KHX` specifies that dependencies are
  ///     [non-device-local dependency](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#synchronization-device-local-dependencies).
  ///
  pub enum VkDependencyFlagBits {
    BIT_BY_REGION = 1<<0,

    // feature: VK_KHX_multiview
    #[cfg(feature = "VK_KHX_multiview")]
    BIT_VIEW_LOCAL_BIT_KHX = 1<<1,

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    BIT_DEVICE_GROUP_BIT_KHX = 1<<2
  }
}

// feature: VK_KHR_surface

#[cfg(feature = "VK_KHR_surface")]
define_enum! {

  /// presentation mode supported for a surface
  ///
  /// Possible values of elements of the
  /// `vkGetPhysicalDeviceSurfacePresentModesKHR::pPresentModes` array, indicating the
  /// supported presentation modes for a surface, are:
  ///
  ///   - `VK_PRESENT_MODE_IMMEDIATE_KHR` indicates that the presentation engine does
  ///     not wait for a vertical blanking period to update the current image, meaning
  ///     this mode may: result in visible tearing. No internal queuing of
  ///     presentation requests is needed, as the requests are applied immediately.
  ///
  ///   - `VK_PRESENT_MODE_MAILBOX_KHR` indicates that the presentation engine waits
  ///     for the next vertical blanking period to update the current image. Tearing
  ///     cannot: be observed. An internal single-entry queue is used to hold pending
  ///     presentation requests. If the queue is full when a new presentation request
  ///     is received, the new request replaces the existing entry, and any images
  ///     associated with the prior entry become available for re-use by the
  ///     application. One request is removed from the queue and processed during each
  ///     vertical blanking period in which the queue is non-empty.
  ///
  ///   - `VK_PRESENT_MODE_FIFO_KHR` indicates that the presentation engine waits for
  ///     the next vertical blanking period to update the current image. Tearing
  ///     cannot: be observed. An internal queue is used to hold pending presentation
  ///     requests. New requests are appended to the end of the queue, and one request
  ///     is removed from the beginning of the queue and processed during each
  ///     vertical blanking period in which the queue is non-empty. This is the only
  ///     value of `presentMode` that is required: to be supported.
  ///
  ///   - `VK_PRESENT_MODE_FIFO_RELAXED_KHR` indicates that the presentation engine
  ///     generally waits for the next vertical blanking period to update the current
  ///     image. If a vertical blanking period has already passed since the last
  ///     update of the current image then the presentation engine does not wait for
  ///     another vertical blanking period for the update, meaning this mode may:
  ///     result in visible tearing in this case. This mode is useful for reducing
  ///     visual stutter with an application that will mostly present a new image
  ///     before the next vertical blanking period, but may occasionally be late, and
  ///     present a new image just after the next vertical blanking period. An
  ///     internal queue is used to hold pending presentation requests. New requests
  ///     are appended to the end of the queue, and one request is removed from the
  ///     beginning of the queue and processed during or after each vertical blanking
  ///     period in which the queue is non-empty.
  ///
  ///   - `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` indicates that the presentation
  ///     engine and application have concurrent access to a single image, which is
  ///     referred to as a *shared presentable image*. The presentation engine is only
  ///     required to update the current image after a new presentation request is
  ///     received. Therefore the application must: make a presentation request
  ///     whenever an update is required. However, the presentation engine may: update
  ///     the current image at any point, meaning this mode may: result in visible
  ///     tearing.
  ///
  ///   - `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` indicates that the
  ///     presentation engine and application have concurrent access to a single
  ///     image, which is referred to as a *shared presentable image*. The
  ///     presentation engine periodically updates the current image on its regular
  ///     refresh cycle. The application is only required to make one initial
  ///     presentation request, after which the presentation engine must: update the
  ///     current image without any need for further presentation requests. The
  ///     application can: indicate the image contents have been updated by making a
  ///     presentation request, but this does not guarantee the timing of when it will
  ///     be updated. This mode may: result in visible tearing if rendering to the
  ///     image is not timed correctly.
  ///
  /// The supported `VkImageUsageFlagBits` of the presentable images of a swapchain
  /// created for a surface may: differ depending on the presentation mode, and can be
  /// determined as per the table below:
  ///
  /// <table>
  /// <caption>Presentable image usage queries</caption>
  /// <colgroup>
  /// <col width="50%" />
  /// <col width="50%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">Presentation mode</th>
  /// <th align="left">Image usage flags</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_PRESENT_MODE_IMMEDIATE_KHR</code></p></td>
  /// <td align="left"><p><code>VkSurfaceCapabilitiesKHR::supportedUsageFlags</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_PRESENT_MODE_MAILBOX_KHR</code></p></td>
  /// <td align="left"><p><code>VkSurfaceCapabilitiesKHR::supportedUsageFlags</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_PRESENT_MODE_FIFO_KHR</code></p></td>
  /// <td align="left"><p><code>VkSurfaceCapabilitiesKHR::supportedUsageFlags</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_PRESENT_MODE_FIFO_RELAXED_KHR</code></p></td>
  /// <td align="left"><p><code>VkSurfaceCapabilitiesKHR::supportedUsageFlags</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR</code></p></td>
  /// <td align="left"><p><code>VkSharedPresentSurfaceCapabilitiesKHR::sharedPresentSupportedUsageFlags</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR</code></p></td>
  /// <td align="left"><p><code>VkSharedPresentSurfaceCapabilitiesKHR::sharedPresentSupportedUsageFlags</code></p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  /// > **Note**
  /// >
  /// > For reference, the mode indicated by `VK_PRESENT_MODE_FIFO_KHR` is equivalent
  /// > to the behavior of {wgl|glX|egl}SwapBuffers with a swap interval of 1, while
  /// > the mode indicated by `VK_PRESENT_MODE_FIFO_RELAXED_KHR` is equivalent to the
  /// > behavior of {wgl|glX}SwapBuffers with a swap interval of -1 (from the
  /// > {WGL|GLX}\_EXT\_swap\_control\_tear extensions).
  ///
  pub enum VkPresentModeKHR {
    E_IMMEDIATE_KHR = 0,
    E_MAILBOX_KHR = 1,
    E_FIFO_KHR = 2,
    E_FIFO_RELAXED_KHR = 3,

    // feature: VK_KHR_shared_presentable_image
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    E_SHARED_DEMAND_REFRESH_KHR = 1000111000,
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    E_SHARED_CONTINUOUS_REFRESH_KHR = 1000111001
  }
}

#[cfg(feature = "VK_KHR_surface")]
define_enum! {

  /// supported color space of the presentation engine
  ///
  /// Possible values of `VkSurfaceFormatKHR::colorSpace`, specifying supported color
  /// spaces of a presentation engine, are:
  ///
  ///   - `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR` indicates support for the sRGB color
  ///     space.
  ///
  ///   - `VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT` indicates support for the
  ///     Display-P3 color space and applies an sRGB-like transfer function (defined
  ///     below).
  ///
  ///   - `VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT` indicates support for the extended
  ///     sRGB color space and applies a linear transfer function.
  ///
  ///   - `VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT` indicates support for the
  ///     extended sRGB color space and applies an sRGB transfer function.
  ///
  ///   - `VK_COLOR_SPACE_DCI_P3_LINEAR_EXT` indicates support for the DCI-P3 color
  ///     space and applies a linear OETF.
  ///
  ///   - `VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT` indicates support for the DCI-P3 color
  ///     space and applies the Gamma 2.6 OETF.
  ///
  ///   - `VK_COLOR_SPACE_BT709_LINEAR_EXT` indicates support for the BT709 color
  ///     space and applies a linear OETF.
  ///
  ///   - `VK_COLOR_SPACE_BT709_NONLINEAR_EXT` indicates support for the BT709 color
  ///     space and applies the SMPTE 170M OETF.
  ///
  ///   - `VK_COLOR_SPACE_BT2020_LINEAR_EXT` indicates support for the BT2020 color
  ///     space and applies a linear OETF.
  ///
  ///   - `VK_COLOR_SPACE_HDR10_ST2084_EXT` indicates support for the HDR10 (BT2020
  ///     color) space and applies the SMPTE ST2084 Perceptual Quantizer (PQ) OETF.
  ///
  ///   - `VK_COLOR_SPACE_DOLBYVISION_EXT` indicates support for the Dolby Vision
  ///     (BT2020 color space), proprietary encoding, and applies the SMPTE ST2084
  ///     OETF.
  ///
  ///   - `VK_COLOR_SPACE_HDR10_HLG_EXT` indicates support for the HDR10 (BT2020 color
  ///     space) and applies the Hybrid Log Gamma (HLG) OETF.
  ///
  ///   - `VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT` indicates support for the AdobeRGB
  ///     color space and applies a linear OETF.
  ///
  ///   - `VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT` indicates support for the AdobeRGB
  ///     color space and applies the Gamma 2.2 OETF.
  ///
  ///   - `VK_COLOR_SPACE_PASS_THROUGH_EXT` indicates that color components are used
  ///     “as is”. This is intended to allow applications to supply data for color
  ///     spaces not described here.
  ///
  /// The color components of Non-linear color space swap chain images have had the
  /// appropriate transfer function applied. Vulkan requires that all implementations
  /// support the sRGB transfer function when using an SRGB pixel format. Other
  /// transfer functions, such as SMPTE 170M or SMPTE2084, must: not be performed by
  /// the implementation, but can: be performed by the application shader. This
  /// extension defines enums for `VkColorSpaceKHR` that correspond to the following
  /// color spaces:
  ///
  /// <table style="width:100%;">
  /// <caption>Color Spaces and Attributes</caption>
  /// <colgroup>
  /// <col width="16%" />
  /// <col width="16%" />
  /// <col width="16%" />
  /// <col width="16%" />
  /// <col width="16%" />
  /// <col width="16%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">Name</th>
  /// <th align="left">Red Primary</th>
  /// <th align="left">Green Primary</th>
  /// <th align="left">Blue Primary</th>
  /// <th align="left">White-point</th>
  /// <th align="left">Transfer function</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p>DCI-P3</p></td>
  /// <td align="left"><p>0.680, 0.320</p></td>
  /// <td align="left"><p>0.265, 0.690</p></td>
  /// <td align="left"><p>0.150, 0.060</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>Gamma 2.6</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p>Display-P3</p></td>
  /// <td align="left"><p>0.680, 0.320</p></td>
  /// <td align="left"><p>0.265, 0.690</p></td>
  /// <td align="left"><p>0.150, 0.060</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>Display-P3</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p>BT709</p></td>
  /// <td align="left"><p>0.640, 0.330</p></td>
  /// <td align="left"><p>0.300, 0.600</p></td>
  /// <td align="left"><p>0.150, 0.060</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>SMPTE 170M</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p>sRGB</p></td>
  /// <td align="left"><p>0.640, 0.330</p></td>
  /// <td align="left"><p>0.300, 0.600</p></td>
  /// <td align="left"><p>0.150, 0.060</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>sRGB</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p>extended sRGB</p></td>
  /// <td align="left"><p>0.640, 0.330</p></td>
  /// <td align="left"><p>0.300, 0.600</p></td>
  /// <td align="left"><p>0.150, 0.060</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>extended sRGB</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p>HDR10_ST2084</p></td>
  /// <td align="left"><p>0.708, 0.292</p></td>
  /// <td align="left"><p>0.170, 0.797</p></td>
  /// <td align="left"><p>0.131, 0.046</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>ST2084</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p>DOLBYVISION</p></td>
  /// <td align="left"><p>0.708, 0.292</p></td>
  /// <td align="left"><p>0.170, 0.797</p></td>
  /// <td align="left"><p>0.131, 0.046</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>ST2084</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p>HDR10_HLG</p></td>
  /// <td align="left"><p>0.708, 0.292</p></td>
  /// <td align="left"><p>0.170, 0.797</p></td>
  /// <td align="left"><p>0.131, 0.046</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>HLG</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p>AdobeRGB</p></td>
  /// <td align="left"><p>0.640, 0.330</p></td>
  /// <td align="left"><p>0.210, 0.710</p></td>
  /// <td align="left"><p>0.150, 0.060</p></td>
  /// <td align="left"><p>0.3127, 0.3290 (D65)</p></td>
  /// <td align="left"><p>AdobeRGB</p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  /// For Opto-Electrical Transfer Function (OETF), unless otherwise specified, the
  /// values of L and E are defined as:
  ///
  /// L - linear luminance of image for conventional colorimetry
  ///
  /// E - corresponding electrical signal (value stored in memory)
  ///
  pub enum VkColorSpaceKHR {
    E_SRGB_NONLINEAR_KHR = 0,

    // feature: VK_EXT_swapchain_colorspace
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_DISPLAY_P3_NONLINEAR_EXT = 1000104001,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_EXTENDED_SRGB_LINEAR_EXT = 1000104002,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_DCI_P3_LINEAR_EXT = 1000104003,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_DCI_P3_NONLINEAR_EXT = 1000104004,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_BT709_LINEAR_EXT = 1000104005,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_BT709_NONLINEAR_EXT = 1000104006,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_BT2020_LINEAR_EXT = 1000104007,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_HDR10_ST2084_EXT = 1000104008,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_DOLBYVISION_EXT = 1000104009,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_HDR10_HLG_EXT = 1000104010,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_ADOBERGB_LINEAR_EXT = 1000104011,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_ADOBERGB_NONLINEAR_EXT = 1000104012,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_PASS_THROUGH_EXT = 1000104013,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    E_EXTENDED_SRGB_NONLINEAR_EXT = 1000104014
  }
}

// feature: VK_KHR_display

#[cfg(feature = "VK_KHR_display")]
define_bitmask! {

  /// Alpha blending type
  ///
  /// Possible values of `VkDisplaySurfaceCreateInfoKHR::alphaMode`, specifying the
  /// type of alpha blending to use on a display, are:
  ///
  ///   - `VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR` specifies that the source image will
  ///     be treated as opaque.
  ///
  ///   - `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR` specifies that a global alpha value
  ///     must: be specified that will be applied to all pixels in the source image.
  ///
  ///   - `VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR` specifies that the alpha value
  ///     will be determined by the alpha channel of the source image’s pixels. If the
  ///     source format contains no alpha values, no blending will be applied. The
  ///     source alpha values are not premultiplied into the source image’s other
  ///     color channels.
  ///
  ///   - `VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR` is equivalent to
  ///     `VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR`, except the source alpha values
  ///     are assumed to be premultiplied into the source image’s other color
  ///     channels.
  ///
  pub enum VkDisplayPlaneAlphaFlagBitsKHR {
    BIT_OPAQUE_BIT_KHR = 1<<0,
    BIT_GLOBAL_BIT_KHR = 1<<1,
    BIT_PER_PIXEL_BIT_KHR = 1<<2,
    BIT_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 1<<3
  }
}

// feature: VK_KHR_surface

#[cfg(feature = "VK_KHR_surface")]
define_bitmask! {

  /// alpha compositing modes supported on a device
  ///
  /// The `supportedCompositeAlpha` member is of type `VkCompositeAlphaFlagBitsKHR`,
  /// which contains the following values:
  ///
  /// These values are described as follows:
  ///
  ///   - `VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR`: The alpha channel, if it exists, of the
  ///     images is ignored in the compositing process. Instead, the image is treated
  ///     as if it has a constant alpha of 1.0.
  ///
  ///   - `VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR`: The alpha channel, if it
  ///     exists, of the images is respected in the compositing process. The non-alpha
  ///     channels of the image are expected to already be multiplied by the alpha
  ///     channel by the application.
  ///
  ///   - `VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR`: The alpha channel, if it
  ///     exists, of the images is respected in the compositing process. The non-alpha
  ///     channels of the image are not expected to already be multiplied by the alpha
  ///     channel by the application; instead, the compositor will multiply the
  ///     non-alpha channels of the image by the alpha channel during compositing.
  ///
  ///   - `VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR`: The way in which the presentation
  ///     engine treats the alpha channel in the images is unknown to the Vulkan API.
  ///     Instead, the application is responsible for setting the composite alpha
  ///     blending mode using native window system commands. If the application does
  ///     not set the blending mode using native window system commands, then a
  ///     platform-specific default will be used.
  ///
  pub enum VkCompositeAlphaFlagBitsKHR {
    BIT_OPAQUE_BIT_KHR = 1<<0,
    BIT_PRE_MULTIPLIED_BIT_KHR = 1<<1,
    BIT_POST_MULTIPLIED_BIT_KHR = 1<<2,
    BIT_INHERIT_BIT_KHR = 1<<3
  }
}

#[cfg(feature = "VK_KHR_surface")]
define_bitmask! {

  /// presentation transforms supported on a device
  ///
  /// Bits which may: be set in `VkSurfaceCapabilitiesKHR::supportedTransforms`
  /// indicating the presentation transforms supported for the surface on the
  /// specified device, and possible values of
  /// `VkSurfaceCapabilitiesKHR::currentTransform` is indicating the surface’s current
  /// transform relative to the presentation engine’s natural orientation, are:
  ///
  ///   - `VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR` indicates that image content is
  ///     presented without being transformed.
  ///
  ///   - `VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR` indicates that image content is
  ///     rotated 90 degrees clockwise.
  ///
  ///   - `VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR` indicates that image content is
  ///     rotated 180 degrees clockwise.
  ///
  ///   - `VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR` indicates that image content is
  ///     rotated 270 degrees clockwise.
  ///
  ///   - `VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR` indicates that image
  ///     content is mirrored horizontally.
  ///
  ///   - `VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR` indicates that
  ///     image content is mirrored horizontally, then rotated 90 degrees clockwise.
  ///
  ///   - `VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR` indicates that
  ///     image content is mirrored horizontally, then rotated 180 degrees clockwise.
  ///
  ///   - `VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR` indicates that
  ///     image content is mirrored horizontally, then rotated 270 degrees clockwise.
  ///
  ///   - `VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR` indicates that the presentation
  ///     transform is not specified, and is instead determined by platform-specific
  ///     considerations and mechanisms outside Vulkan.
  ///
  pub enum VkSurfaceTransformFlagBitsKHR {
    BIT_IDENTITY_BIT_KHR = 1<<0,
    BIT_ROTATE_90_BIT_KHR = 1<<1,
    BIT_ROTATE_180_BIT_KHR = 1<<2,
    BIT_ROTATE_270_BIT_KHR = 1<<3,
    BIT_HORIZONTAL_MIRROR_BIT_KHR = 1<<4,
    BIT_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 1<<5,
    BIT_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 1<<6,
    BIT_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 1<<7,
    BIT_INHERIT_BIT_KHR = 1<<8
  }
}

// feature: VK_EXT_debug_report

#[cfg(feature = "VK_EXT_debug_report")]
define_bitmask! {

  /// Bitmask specifying events which cause a debug report callback
  ///
  /// Bits which can: be set in `VkDebugReportCallbackCreateInfoEXT::flags`,
  /// specifying events which cause a debug report.
  ///
  ///   - `VK_DEBUG_REPORT_ERROR_BIT_EXT` specifies that an error that may cause
  ///     undefined results, including an application crash.
  ///
  ///   - `VK_DEBUG_REPORT_WARNING_BIT_EXT` specifies use of Vulkan that may: expose
  ///     an app bug. Such cases may not be immediately harmful, such as a fragment
  ///     shader outputting to a location with no attachment. Other cases may: point
  ///     to behavior that is almost certainly bad when unintended such as using an
  ///     image whose memory has not been filled. In general if you see a warning but
  ///     you know that the behavior is intended/desired, then simply ignore the
  ///     warning.
  ///
  ///   - `VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT` specifies a potentially
  ///     non-optimal use of Vulkan, e.g. using `vkCmdClearColorImage` when setting
  ///     `VkAttachmentDescription::loadOp` to `VK_ATTACHMENT_LOAD_OP_CLEAR` would
  ///     have worked.
  ///
  ///   - `VK_DEBUG_REPORT_INFORMATION_BIT_EXT` specifies an informational message
  ///     such as resource details that may be handy when debugging an application.
  ///
  ///   - `VK_DEBUG_REPORT_DEBUG_BIT_EXT` specifies diagnostic information from the
  ///     implementation and layers.
  ///
  pub enum VkDebugReportFlagBitsEXT {
    BIT_INFORMATION_BIT_EXT = 1<<0,
    BIT_WARNING_BIT_EXT = 1<<1,
    BIT_PERFORMANCE_WARNING_BIT_EXT = 1<<2,
    BIT_ERROR_BIT_EXT = 1<<3,
    BIT_DEBUG_BIT_EXT = 1<<4
  }
}

#[cfg(feature = "VK_EXT_debug_report")]
define_enum! {

  /// Specify the type of an object handle
  ///
  /// Possible values passed to the `objectType` parameter of the callback function
  /// specified by `VkDebugReportCallbackCreateInfoEXT::pfnCallback`, specifying the
  /// type of object handle being reported.
  ///
  /// <table>
  /// <caption>VkDebugReportObjectTypeEXT and Vulkan Handle Relationship</caption>
  /// <colgroup>
  /// <col width="60%" />
  /// <col width="39%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left"><code>VkDebugReportObjectTypeEXT</code></th>
  /// <th align="left">Vulkan Handle Type</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT</code></p></td>
  /// <td align="left"><p>Unknown/Undefined Handle</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT</code></p></td>
  /// <td align="left"><p><code>VkInstance</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT</code></p></td>
  /// <td align="left"><p><code>VkPhysicalDevice</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT</code></p></td>
  /// <td align="left"><p><code>VkDevice</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT</code></p></td>
  /// <td align="left"><p><code>VkQueue</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT</code></p></td>
  /// <td align="left"><p><code>VkSemaphore</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT</code></p></td>
  /// <td align="left"><p><code>VkCommandBuffer</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT</code></p></td>
  /// <td align="left"><p><code>VkFence</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT</code></p></td>
  /// <td align="left"><p><code>VkDeviceMemory</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT</code></p></td>
  /// <td align="left"><p><code>VkBuffer</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT</code></p></td>
  /// <td align="left"><p><code>VkImage</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT</code></p></td>
  /// <td align="left"><p><code>VkEvent</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT</code></p></td>
  /// <td align="left"><p><code>VkQueryPool</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT</code></p></td>
  /// <td align="left"><p><code>VkBufferView</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT</code></p></td>
  /// <td align="left"><p><code>VkImageView</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT</code></p></td>
  /// <td align="left"><p><code>VkShaderModule</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT</code></p></td>
  /// <td align="left"><p><code>VkPipelineCache</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT</code></p></td>
  /// <td align="left"><p><code>VkPipelineLayout</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT</code></p></td>
  /// <td align="left"><p><code>VkRenderPass</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT</code></p></td>
  /// <td align="left"><p><code>VkPipeline</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT</code></p></td>
  /// <td align="left"><p><code>VkDescriptorSetLayout</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT</code></p></td>
  /// <td align="left"><p><code>VkSampler</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT</code></p></td>
  /// <td align="left"><p><code>VkDescriptorPool</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT</code></p></td>
  /// <td align="left"><p><code>VkDescriptorSet</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT</code></p></td>
  /// <td align="left"><p><code>VkFramebuffer</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT</code></p></td>
  /// <td align="left"><p><code>VkCommandPool</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT</code></p></td>
  /// <td align="left"><p><code>VkSurfaceKHR</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT</code></p></td>
  /// <td align="left"><p><code>VkSwapchainKHR</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT</code></p></td>
  /// <td align="left"><p><code>VkDebugReportCallbackEXT</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT</code></p></td>
  /// <td align="left"><p><code>VkDisplayKHR</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT</code></p></td>
  /// <td align="left"><p><code>VkDisplayModeKHR</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_OBJECT_TABLE_NVX_EXT</code></p></td>
  /// <td align="left"><p><code>VkObjectTableNVX</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX_EXT</code></p></td>
  /// <td align="left"><p><code>VkIndirectCommandsLayoutNVX</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT</code></p></td>
  /// <td align="left"><p><code>VkDescriptorUpdateTemplateKHR</code></p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  /// > **Note**
  /// >
  /// > The primary expected use of `VK_ERROR_VALIDATION_FAILED_EXT` is for validation
  /// > layer testing. It is not expected that an application would see this error
  /// > code during normal use of the validation layers.
  ///
  pub enum VkDebugReportObjectTypeEXT {
    E_UNKNOWN_EXT = 0,
    E_INSTANCE_EXT = 1,
    E_PHYSICAL_DEVICE_EXT = 2,
    E_DEVICE_EXT = 3,
    E_QUEUE_EXT = 4,
    E_SEMAPHORE_EXT = 5,
    E_COMMAND_BUFFER_EXT = 6,
    E_FENCE_EXT = 7,
    E_DEVICE_MEMORY_EXT = 8,
    E_BUFFER_EXT = 9,
    E_IMAGE_EXT = 10,
    E_EVENT_EXT = 11,
    E_QUERY_POOL_EXT = 12,
    E_BUFFER_VIEW_EXT = 13,
    E_IMAGE_VIEW_EXT = 14,
    E_SHADER_MODULE_EXT = 15,
    E_PIPELINE_CACHE_EXT = 16,
    E_PIPELINE_LAYOUT_EXT = 17,
    E_RENDER_PASS_EXT = 18,
    E_PIPELINE_EXT = 19,
    E_DESCRIPTOR_SET_LAYOUT_EXT = 20,
    E_SAMPLER_EXT = 21,
    E_DESCRIPTOR_POOL_EXT = 22,
    E_DESCRIPTOR_SET_EXT = 23,
    E_FRAMEBUFFER_EXT = 24,
    E_COMMAND_POOL_EXT = 25,
    E_SURFACE_KHR_EXT = 26,
    E_SWAPCHAIN_KHR_EXT = 27,
    E_DEBUG_REPORT_CALLBACK_EXT_EXT = 28,
    E_DISPLAY_KHR_EXT = 29,
    E_DISPLAY_MODE_KHR_EXT = 30,
    E_OBJECT_TABLE_NVX_EXT = 31,
    E_INDIRECT_COMMANDS_LAYOUT_NVX_EXT = 32,
    E_VALIDATION_CACHE_EXT_EXT = 33,

    // feature: VK_KHR_descriptor_update_template
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    E_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT = 1000085000,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    E_SAMPLER_YCBCR_CONVERSION_KHR_EXT = 1000156000
  }
}

// feature: VK_AMD_rasterization_order

#[cfg(feature = "VK_AMD_rasterization_order")]
define_enum! {

  /// Specify rasterization order for a graphics pipeline
  ///
  /// Possible values of
  /// `VkPipelineRasterizationStateRasterizationOrderAMD::rasterizationOrder`,
  /// specifying the primitive rasterization order.
  ///
  ///   - `VK_RASTERIZATION_ORDER_STRICT_AMD` specifies that operations for each
  ///     primitive in a subpass must: occur in [primitive
  ///     order](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#drawing-primitive-order).
  ///
  ///   - `VK_RASTERIZATION_ORDER_RELAXED_AMD` specifies that operations for each
  ///     primitive in a subpass may: not occur in [primitive
  ///     order](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#drawing-primitive-order).
  ///
  pub enum VkRasterizationOrderAMD {
    E_STRICT_AMD = 0,
    E_RELAXED_AMD = 1
  }
}

// feature: VK_NV_external_memory_capabilities

#[cfg(feature = "VK_NV_external_memory_capabilities")]
define_bitmask! {

  /// Bitmask specifying external memory handle types
  ///
  /// Possible values of `VkImportMemoryWin32HandleInfoNV::handleType`, specifying the
  /// type of an external memory handle, are:
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV` indicates a handle
  ///     to memory returned by `vkGetMemoryWin32HandleNV`.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV` indicates a handle to
  ///     memory returned by `vkGetMemoryWin32HandleNV`, or one duplicated from such a
  ///     handle using `DuplicateHandle()`.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV` indicates a valid NT
  ///     handle to memory returned by `IDXGIResource1::ftext:CreateSharedHandle()`,
  ///     or a handle duplicated from such a handle using `DuplicateHandle()`.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV` indicates a handle
  ///     to memory returned by `IDXGIResource::GetSharedHandle()`.
  ///
  pub enum VkExternalMemoryHandleTypeFlagBitsNV {
    BIT_OPAQUE_WIN32_BIT_NV = 1<<0,
    BIT_OPAQUE_WIN32_KMT_BIT_NV = 1<<1,
    BIT_D3D11_IMAGE_BIT_NV = 1<<2,
    BIT_D3D11_IMAGE_KMT_BIT_NV = 1<<3
  }
}

#[cfg(feature = "VK_NV_external_memory_capabilities")]
define_bitmask! {

  /// Bitmask specifying external memory features
  ///
  /// Bits which can: be set in
  /// `VkExternalMemoryFeatureFlagBitsNV::externalMemoryFeatures`, indicating
  /// properties of the external memory handle type, are:
  ///
  ///   - `VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV` indicates that external
  ///     memory of the specified type must: be created as a dedicated allocation when
  ///     used in the manner specified.
  ///
  ///   - `VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV` indicates that the
  ///     implementation supports exporting handles of the specified type.
  ///
  ///   - `VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV` indicates that the
  ///     implementation supports importing handles of the specified type.
  ///
  pub enum VkExternalMemoryFeatureFlagBitsNV {
    BIT_DEDICATED_ONLY_BIT_NV = 1<<0,
    BIT_EXPORTABLE_BIT_NV = 1<<1,
    BIT_IMPORTABLE_BIT_NV = 1<<2
  }
}

// feature: VK_EXT_validation_flags

#[cfg(feature = "VK_EXT_validation_flags")]
define_enum! {

  /// Specify validation checks to disable
  ///
  /// Possible values of elements of the
  /// `VkValidationFlagsEXT::pDisabledValidationChecks` array, specifying validation
  /// checks to be disabled.
  ///
  ///   - `VK_VALIDATION_CHECK_ALL_EXT` specifies that all validation checks are
  ///     disabled.
  ///
  ///   - `VK_VALIDATION_CHECK_SHADERS_EXT` specifies that shader validation is
  ///     disabled.
  ///
  pub enum VkValidationCheckEXT {
    E_ALL_EXT = 0,
    E_SHADERS_EXT = 1
  }
}

// feature: VK_NVX_device_generated_commands

#[cfg(feature = "VK_NVX_device_generated_commands")]
define_bitmask! {

  /// Bitmask specifying allowed usage of a indirect commands layout
  ///
  /// Bits which can: be set in `VkIndirectCommandsLayoutCreateInfoNVX::flags`,
  /// specifying usage hints of an indirect command layout, are:
  ///
  ///   - `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NVX` indicates
  ///     that the processing of sequences can: happen at an implementation-dependent
  ///     order, which is not guaranteed to be coherent across multiple invocations.
  ///
  ///   - `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_SPARSE_SEQUENCES_BIT_NVX` indicates that
  ///     there is likely a high difference between allocated number of sequences and
  ///     actually used.
  ///
  ///   - `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EMPTY_EXECUTIONS_BIT_NVX` indicates that
  ///     there are likely many draw or dispatch calls that are zero-sized (zero grid
  ///     dimension, no primitives to render).
  ///
  ///   - `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NVX` indicates that
  ///     the input data for the sequences is not implicitly indexed from
  ///     0..sequencesUsed but a user provided `VkBuffer` encoding the index is
  ///     provided.
  ///
  pub enum VkIndirectCommandsLayoutUsageFlagBitsNVX {
    BIT_UNORDERED_SEQUENCES_BIT_NVX = 1<<0,
    BIT_SPARSE_SEQUENCES_BIT_NVX = 1<<1,
    BIT_EMPTY_EXECUTIONS_BIT_NVX = 1<<2,
    BIT_INDEXED_SEQUENCES_BIT_NVX = 1<<3
  }
}

#[cfg(feature = "VK_NVX_device_generated_commands")]
define_bitmask! {

  /// Bitmask specifying allowed usage of an object entry
  ///
  /// Bits which can: be set in elements of the
  /// `VkObjectTableCreateInfoNVX::pObjectEntryUsageFlags` array, specifying binding
  /// usage of an entry, are:
  ///
  ///   - `VK_OBJECT_ENTRY_USAGE_GRAPHICS_BIT_NVX` indicates that the resource is
  ///     bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
  ///
  ///   - `VK_OBJECT_ENTRY_USAGE_COMPUTE_BIT_NVX` indicates that the resource is bound
  ///     to `VK_PIPELINE_BIND_POINT_COMPUTE`
  ///
  pub enum VkObjectEntryUsageFlagBitsNVX {
    BIT_GRAPHICS_BIT_NVX = 1<<0,
    BIT_COMPUTE_BIT_NVX = 1<<1
  }
}

#[cfg(feature = "VK_NVX_device_generated_commands")]
define_enum! {

  /// Enum specifying
  ///
  /// Possible values of those elements of the
  /// `VkIndirectCommandsLayoutCreateInfoNVX::pTokens` array which specify command
  /// tokens (other elements of the array specify command parameters) are:
  ///
  /// <table>
  /// <caption>Supported indirect command tokens</caption>
  /// <colgroup>
  /// <col width="67%" />
  /// <col width="32%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">Token type</th>
  /// <th align="left">Equivalent command</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_INDIRECT_COMMANDS_TOKEN_TYPE_PIPELINE_NVX</code></p></td>
  /// <td align="left"><p><code>vkCmdBindPipeline</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_INDIRECT_COMMANDS_TOKEN_TYPE_DESCRIPTOR_SET_NVX</code></p></td>
  /// <td align="left"><p><code>vkCmdBindDescriptorSets</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NVX</code></p></td>
  /// <td align="left"><p><code>vkCmdBindIndexBuffer</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NVX</code></p></td>
  /// <td align="left"><p><code>vkCmdBindVertexBuffers</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NVX</code></p></td>
  /// <td align="left"><p><code>vkCmdPushConstants</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NVX</code></p></td>
  /// <td align="left"><p><code>vkCmdDrawIndexedIndirect</code></p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NVX</code></p></td>
  /// <td align="left"><p><code>vkCmdDrawIndirect</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_NVX</code></p></td>
  /// <td align="left"><p><code>vkCmdDispatchIndirect</code></p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  pub enum VkIndirectCommandsTokenTypeNVX {
    E_PIPELINE_NVX = 0,
    E_DESCRIPTOR_SET_NVX = 1,
    E_INDEX_BUFFER_NVX = 2,
    E_VERTEX_BUFFER_NVX = 3,
    E_PUSH_CONSTANT_NVX = 4,
    E_DRAW_INDEXED_NVX = 5,
    E_DRAW_NVX = 6,
    E_DISPATCH_NVX = 7
  }
}

#[cfg(feature = "VK_NVX_device_generated_commands")]
define_enum! {

  /// Enum specifying object table entry type
  ///
  /// Possible values of elements of the
  /// `VkObjectTableCreateInfoNVX::pObjectEntryTypes` array, specifying the entry type
  /// of a configuration, are:
  ///
  ///   - `VK_OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX` indicates a `VkDescriptorSet`
  ///     resource entry that is registered via `VkObjectTableDescriptorSetEntryNVX`.
  ///
  ///   - `VK_OBJECT_ENTRY_TYPE_PIPELINE_NVX` indicates a `VkPipeline` resource entry
  ///     that is registered via `VkObjectTablePipelineEntryNVX`.
  ///
  ///   - `VK_OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX` indicates a `VkBuffer` resource
  ///     entry that is registered via `VkObjectTableIndexBufferEntryNVX`.
  ///
  ///   - `VK_OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX` indicates a `VkBuffer` resource
  ///     entry that is registered via `VkObjectTableVertexBufferEntryNVX`.
  ///
  ///   - `VK_OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX` indicates the resource entry is
  ///     registered via `VkObjectTablePushConstantEntryNVX`.
  ///
  pub enum VkObjectEntryTypeNVX {
    E_DESCRIPTOR_SET_NVX = 0,
    E_PIPELINE_NVX = 1,
    E_INDEX_BUFFER_NVX = 2,
    E_VERTEX_BUFFER_NVX = 3,
    E_PUSH_CONSTANT_NVX = 4
  }
}

// feature: VK_VERSION_1_0

define_bitmask! {

  /// Bitmask specifying descriptor set layout properties
  ///
  /// Bits which can: be set in `VkDescriptorSetLayoutCreateInfo::flags` to specify
  /// options for descriptor set layout are.
  ///
  ///   - `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` specifies that
  ///     descriptor sets must: not be allocated using this layout, and descriptors
  ///     are instead pushed by `vkCmdPushDescriptorSetKHR`.
  ///
  pub enum VkDescriptorSetLayoutCreateFlagBits {

    // feature: VK_KHR_push_descriptor
    #[cfg(feature = "VK_KHR_push_descriptor")]
    BIT_PUSH_DESCRIPTOR_BIT_KHR = 1<<0
  }
}

// feature: VK_KHR_external_memory_capabilities

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
define_bitmask! {

  /// Bit specifying external memory handle types
  ///
  /// Possible values of `VkPhysicalDeviceExternalImageFormatInfoKHR::handleType`,
  /// specifying an external memory handle type.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR` specifies a POSIX file
  ///     descriptor handle that has only limited valid usage outside of Vulkan and
  ///     other compatible APIs. It must: be compatible with the POSIX system calls
  ///     ftext:dup, ftext:dup2, ftext:close, and the non-standard system call
  ///     ftext:dup3. Additionally, it must: be transportable over a socket using an
  ///     `SCM_RIGHTS` control message. It owns a reference to the underlying memory
  ///     resource represented by its Vulkan memory object.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR` specifies an NT handle
  ///     that has only limited valid usage outside of Vulkan and other compatible
  ///     APIs. It must: be compatible with the functions ftext:DuplicateHandle,
  ///     ftext:CloseHandle, ftext:CompareObjectHandles, ftext:GetHandleInformation,
  ///     and ftext:SetHandleInformation. It owns a reference to the underlying memory
  ///     resource represented by its Vulkan memory object.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR` specifies a global
  ///     share handle that has only limited valid usage outside of Vulkan and other
  ///     compatible APIs. It is not compatible with any native APIs. It does not own
  ///     a reference to the underlying memory resource represented its Vulkan memory
  ///     object, and will therefore become invalid when all Vulkan memory objects
  ///     associated with it are destroyed.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR` specifies an NT
  ///     handle returned by `IDXGIResource1::CreateSharedHandle` referring to a
  ///     Direct3D 10 or 11 texture resource. It owns a reference to the memory used
  ///     by the Direct3D resource.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR` specifies a
  ///     global share handle returned by `IDXGIResource::GetSharedHandle` referring
  ///     to a Direct3D 10 or 11 texture resource. It does not own a reference to the
  ///     underlying Direct3D resource, and will therefore become invalid when all
  ///     Vulkan memory objects and Direct3D resources associated with it are
  ///     destroyed.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR` specifies an NT handle
  ///     returned by `ID3D12Device::CreateSharedHandle` referring to a Direct3D 12
  ///     heap resource. It owns a reference to the resources used by the Direct3D
  ///     heap.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR` specifies an NT
  ///     handle returned by `ID3D12Device::CreateSharedHandle` referring to a
  ///     Direct3D 12 committed resource. It owns a reference to the memory used by
  ///     the Direct3D resource.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT` specifies a host
  ///     pointer returned by a host memory allocation command. It does not own a
  ///     reference to the underlying memory resource, and will therefore become
  ///     invalid if the host memory is freed.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`
  ///     specifies a host pointer to *host mapped foreign memory*. It does not own a
  ///     reference to the underlying memory resource, and will therefore become
  ///     invalid if the foreign memory is unmapped or otherwise becomes no longer
  ///     available.
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT` is a file descriptor for a
  ///     Linux dma\_buf. It owns a reference to the underlying memory resource
  ///     represented by its Vulkan memory object.
  ///
  /// Some external memory handle types can only be shared within the same underlying
  /// physical device and/or the same driver version, as defined in the following
  /// table:
  ///
  /// <table>
  /// <caption>External memory handle types compatibility</caption>
  /// <colgroup>
  /// <col width="33%" />
  /// <col width="33%" />
  /// <col width="33%" />
  /// </colgroup>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p>Handle type</p></td>
  /// <td align="left"><p><code>VkPhysicalDeviceIDPropertiesKHR::driverUUID</code></p></td>
  /// <td align="left"><p><code>VkPhysicalDeviceIDPropertiesKHR::deviceUUID</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT</code></p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT</code></p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT</code></p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  /// > **Note**
  /// >
  /// > The above table does not restrict the drivers and devices with which
  /// > `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT` and
  /// > `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT` may: be
  /// > shared, as these handle types inherently mean memory that does not come from
  /// > the same device, as they import memory from the host or a foreign device,
  /// > respectively.
  ///
  /// > **Note**
  /// >
  /// > Even though the above table does not restrict the drivers and devices with
  /// > which `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT` may: be shared, query
  /// > mechanisms exist in the Vulkan API that prevent the import of incompatible
  /// > dma-bufs (such as `vkGetMemoryFdPropertiesKHR`) and that prevent incompatible
  /// > usage of dma-bufs (such as `VkPhysicalDeviceExternalBufferInfoKHR` and
  /// > `VkPhysicalDeviceExternalImageFormatInfoKHR`).
  ///
  pub enum VkExternalMemoryHandleTypeFlagBitsKHR {
    BIT_OPAQUE_FD_BIT_KHR = 1<<0,
    BIT_OPAQUE_WIN32_BIT_KHR = 1<<1,
    BIT_OPAQUE_WIN32_KMT_BIT_KHR = 1<<2,
    BIT_D3D11_TEXTURE_BIT_KHR = 1<<3,
    BIT_D3D11_TEXTURE_KMT_BIT_KHR = 1<<4,
    BIT_D3D12_HEAP_BIT_KHR = 1<<5,
    BIT_D3D12_RESOURCE_BIT_KHR = 1<<6,

    // feature: VK_EXT_external_memory_dma_buf
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    BIT_DMA_BUF_BIT_EXT = 1<<9,

    // feature: VK_EXT_external_memory_host
    #[cfg(feature = "VK_EXT_external_memory_host")]
    BIT_HOST_ALLOCATION_BIT_EXT = 1<<7,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    BIT_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT = 1<<8
  }
}

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
define_bitmask! {

  /// Bitmask specifying features of an external memory handle type
  ///
  /// Bits which may: be set in
  /// `VkExternalMemoryPropertiesKHR::externalMemoryFeatures`, specifying features of
  /// an external memory handle type.
  ///
  ///   - `VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR` specifies that images or
  ///     buffers created with the specified parameters and handle type must: use the
  ///     mechanisms defined in the  extension to create (or import) a dedicated
  ///     allocation for the image or buffer.
  ///
  ///   - `VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR` specifies that handles of
  ///     this type can: be exported from Vulkan memory objects.
  ///
  ///   - `VK_INTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR` specifies that handles of
  ///     this type can: be imported as Vulkan memory objects.
  ///
  /// Because their semantics in external APIs roughly align with that of an image or
  /// buffer with a dedicated allocation in Vulkan, implementations are required: to
  /// report `VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR` for the following
  /// external handle types:
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR`
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR`
  ///
  ///   - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR`
  ///
  pub enum VkExternalMemoryFeatureFlagBitsKHR {
    BIT_DEDICATED_ONLY_BIT_KHR = 1<<0,
    BIT_EXPORTABLE_BIT_KHR = 1<<1,
    BIT_IMPORTABLE_BIT_KHR = 1<<2
  }
}

// feature: VK_KHR_external_semaphore_capabilities

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
define_bitmask! {

  /// Bitmask of valid external semaphore handle types
  ///
  /// Bits which may: be set in
  /// `VkPhysicalDeviceExternalSemaphoreInfoKHR::handleType`, specifying an external
  /// semaphore handle type.
  ///
  ///   - `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR` specifies a POSIX file
  ///     descriptor handle that has only limited valid usage outside of Vulkan and
  ///     other compatible APIs. It must: be compatible with the POSIX system calls
  ///     `dup`, `dup2`, `close`, and the non-standard system call `dup3`.
  ///     Additionally, it must: be transportable over a socket using an `SCM_RIGHTS`
  ///     control message. It owns a reference to the underlying synchronization
  ///     primitive represented by its Vulkan semaphore object.
  ///
  ///   - `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR` specifies an NT
  ///     handle that has only limited valid usage outside of Vulkan and other
  ///     compatible APIs. It must: be compatible with the functions
  ///     `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`,
  ///     `GetHandleInformation`, and `SetHandleInformation`. It owns a reference to
  ///     the underlying synchronization primitive represented by its Vulkan semaphore
  ///     object.
  ///
  ///   - `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR` specifies a
  ///     global share handle that has only limited valid usage outside of Vulkan and
  ///     other compatible APIs. It is not compatible with any native APIs. It does
  ///     not own a reference to the underlying synchronization primitive represented
  ///     its Vulkan semaphore object, and will therefore become invalid when all
  ///     Vulkan semaphore objects associated with it are destroyed.
  ///
  ///   - `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR` specifies an NT
  ///     handle returned by `ID3D12Device::CreateSharedHandle` referring to a
  ///     Direct3D 12 fence. It owns a reference to the underlying synchronization
  ///     primitive associated with the Direct3D fence.
  ///
  ///   - `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR` specifies a POSIX file
  ///     descriptor handle to a Linux Sync File or Android Fence object. It can be
  ///     used with any native API accepting a valid sync file or fence as input. It
  ///     owns a reference to the underlying synchronization primitive associated with
  ///     the file descriptor. Implementations which support importing this handle
  ///     type must: accept any type of sync or fence FD supported by the native
  ///     system they are running on.
  ///
  /// > **Note**
  /// >
  /// > Handles of type `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR` generated
  /// > by the implementation may represent either Linux Sync Files or Android Fences
  /// > at the implementation’s discretion. Applications should: only use operations
  /// > defined for both types of file descriptors, unless they know via means
  /// > external to Vulkan the type of the file descriptor, or are prepared to deal
  /// > with the system-defined operation failures resulting from using the wrong
  /// > type.
  ///
  /// Some external semaphore handle types can only be shared within the same
  /// underlying physical device and/or the same driver version, as defined in the
  /// following table:
  ///
  /// <table>
  /// <caption>External semaphore handle types compatibility</caption>
  /// <colgroup>
  /// <col width="33%" />
  /// <col width="33%" />
  /// <col width="33%" />
  /// </colgroup>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p>Handle type</p></td>
  /// <td align="left"><p><code>VkPhysicalDeviceIDPropertiesKHR::driverUUID</code></p></td>
  /// <td align="left"><p><code>VkPhysicalDeviceIDPropertiesKHR::deviceUUID</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_FENCE_FD_BIT_KHR</code></p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  pub enum VkExternalSemaphoreHandleTypeFlagBitsKHR {
    BIT_OPAQUE_FD_BIT_KHR = 1<<0,
    BIT_OPAQUE_WIN32_BIT_KHR = 1<<1,
    BIT_OPAQUE_WIN32_KMT_BIT_KHR = 1<<2,
    BIT_D3D12_FENCE_BIT_KHR = 1<<3,
    BIT_SYNC_FD_BIT_KHR = 1<<4
  }
}

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
define_bitmask! {

  /// Bitfield describing features of an external semaphore handle type
  ///
  /// Possible values of
  /// `VkExternalSemaphorePropertiesKHR::externalSemaphoreFeatures`, specifying the
  /// features of an external semaphore handle type.
  ///
  ///   - `VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR` specifies that handles of
  ///     this type can: be exported from Vulkan semaphore objects.
  ///
  ///   - `VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR` specifies that handles of
  ///     this type can: be imported as Vulkan semaphore objects.
  ///
  pub enum VkExternalSemaphoreFeatureFlagBitsKHR {
    BIT_EXPORTABLE_BIT_KHR = 1<<0,
    BIT_IMPORTABLE_BIT_KHR = 1<<1
  }
}

// feature: VK_KHR_external_semaphore

#[cfg(feature = "VK_KHR_external_semaphore")]
define_bitmask! {
  pub enum VkSemaphoreImportFlagBitsKHR {
    BIT_TEMPORARY_BIT_KHR = 1<<0
  }
}

// feature: VK_KHR_external_fence_capabilities

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
define_bitmask! {

  /// Bitmask of valid external fence handle types
  ///
  /// Bits which may: be set in `VkPhysicalDeviceExternalFenceInfoKHR::handleType`,
  /// and in the `exportFromImportedHandleTypes` and `compatibleHandleTypes` members
  /// of `VkExternalFencePropertiesKHR`, to indicate external fence handle types.
  ///
  ///   - `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR` indicates a POSIX file
  ///     descriptor handle that has only limited valid usage outside of Vulkan and
  ///     other compatible APIs. It must: be compatible with the POSIX system calls
  ///     `dup`, `dup2`, `close`, and the non-standard system call `dup3`.
  ///     Additionally, it must: be transportable over a socket using an `SCM_RIGHTS`
  ///     control message. It owns a reference to the underlying synchronization
  ///     primitive represented by its Vulkan fence object.
  ///
  ///   - `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR` indicates an NT handle
  ///     that has only limited valid usage outside of Vulkan and other compatible
  ///     APIs. It must: be compatible with the functions `DuplicateHandle`,
  ///     `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`, and
  ///     `SetHandleInformation`. It owns a reference to the underlying
  ///     synchronization primitive represented by its Vulkan fence object.
  ///
  ///   - `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR` indicates a global
  ///     share handle that has only limited valid usage outside of Vulkan and other
  ///     compatible APIs. It is not compatible with any native APIs. It does not own
  ///     a reference to the underlying synchronization primitive represented by its
  ///     Vulkan fence object, and will therefore become invalid when all Vulkan fence
  ///     objects associated with it are destroyed.
  ///
  ///   - `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR` indicates a POSIX file
  ///     descriptor handle to a Linux Sync File or Android Fence. It can be used with
  ///     any native API accepting a valid sync file or fence as input. It owns a
  ///     reference to the underlying synchronization primitive associated with the
  ///     file descriptor. Implementations which support importing this handle type
  ///     must: accept any type of sync or fence FD supported by the native system
  ///     they are running on.
  ///
  /// Some external fence handle types can only be shared within the same underlying
  /// physical device and/or the same driver version, as defined in the following
  /// table:
  ///
  /// <table>
  /// <caption>External fence handle types compatibility</caption>
  /// <colgroup>
  /// <col width="33%" />
  /// <col width="33%" />
  /// <col width="33%" />
  /// </colgroup>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p>Handle type</p></td>
  /// <td align="left"><p><code>VkPhysicalDeviceIDPropertiesKHR::driverUUID</code></p></td>
  /// <td align="left"><p><code>VkPhysicalDeviceIDPropertiesKHR::deviceUUID</code></p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR</code></p></td>
  /// <td align="left"><p>Must match</p></td>
  /// <td align="left"><p>Must match</p></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR</code></p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// <td align="left"><p>No restriction</p></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  pub enum VkExternalFenceHandleTypeFlagBitsKHR {
    BIT_OPAQUE_FD_BIT_KHR = 1<<0,
    BIT_OPAQUE_WIN32_BIT_KHR = 1<<1,
    BIT_OPAQUE_WIN32_KMT_BIT_KHR = 1<<2,
    BIT_SYNC_FD_BIT_KHR = 1<<3
  }
}

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
define_bitmask! {

  /// Bitfield describing features of an external fence handle type
  ///
  /// Bits which may: be set in `VkExternalFencePropertiesKHR::externalFenceFeatures`,
  /// indicating features of a fence external handle type.
  ///
  ///   - `VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR` indicates handles of this
  ///     type can: be exported from Vulkan fence objects.
  ///
  ///   - `VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR` indicates handles of this
  ///     type can: be imported to Vulkan fence objects.
  ///
  pub enum VkExternalFenceFeatureFlagBitsKHR {
    BIT_EXPORTABLE_BIT_KHR = 1<<0,
    BIT_IMPORTABLE_BIT_KHR = 1<<1
  }
}

// feature: VK_KHR_external_fence

#[cfg(feature = "VK_KHR_external_fence")]
define_bitmask! {
  pub enum VkFenceImportFlagBitsKHR {
    BIT_TEMPORARY_BIT_KHR = 1<<0
  }
}

// feature: VK_EXT_display_surface_counter

#[cfg(feature = "VK_EXT_display_surface_counter")]
define_bitmask! {

  /// Surface-relative counter types
  ///
  /// Bits which can: be set in `VkSurfaceCapabilities2EXT::supportedSurfaceCounters`,
  /// indicating supported surface counter types, are:
  ///
  ///   - `VK_SURFACE_COUNTER_VBLANK_EXT` indicates a counter incrementing once every
  ///     time a vertical blanking period occurs on the display associated with the
  ///     surface.
  ///
  pub enum VkSurfaceCounterFlagBitsEXT {
    BIT_VBLANK_EXT = 1<<0
  }
}

// feature: VK_EXT_display_control

#[cfg(feature = "VK_EXT_display_control")]
define_enum! {

  /// Possible power states for a display
  ///
  /// Possible values of `VkDisplayPowerInfoEXT::powerState`, specifying the new power
  /// state of a display, are:
  ///
  ///   - `VK_DISPLAY_POWER_STATE_OFF_EXT` specifies that the display is powered down.
  ///
  ///   - `VK_DISPLAY_POWER_STATE_SUSPEND_EXT` specifies that the display is put into
  ///     a low power mode, from which it may: be able to transition back to
  ///     `VK_DISPLAY_POWER_STATE_ON_EXT` more quickly than if it were in
  ///     `VK_DISPLAY_POWER_STATE_OFF_EXT`. This state may: be the same as
  ///     `VK_DISPLAY_POWER_STATE_OFF_EXT`.
  ///
  ///   - `VK_DISPLAY_POWER_STATE_ON_EXT` specifies that the display is powered on.
  ///
  pub enum VkDisplayPowerStateEXT {
    E_OFF_EXT = 0,
    E_SUSPEND_EXT = 1,
    E_ON_EXT = 2
  }
}

#[cfg(feature = "VK_EXT_display_control")]
define_enum! {

  /// Events that can occur on a device object
  ///
  /// Possible values of `VkDeviceEventInfoEXT::device`, specifying when a fence will
  /// be signaled, are:
  ///
  ///   - `VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT` specifies that the fence is
  ///     signaled when a display is plugged into or unplugged from the specified
  ///     device. Applications can: use this notification to determine when they need
  ///     to re-enumerate the available displays on a device.
  ///
  pub enum VkDeviceEventTypeEXT {
    E_DISPLAY_HOTPLUG_EXT = 0
  }
}

#[cfg(feature = "VK_EXT_display_control")]
define_enum! {

  /// Events that can occur on a display object
  ///
  /// Possible values of `VkDisplayEventInfoEXT::displayEvent`, specifying when a
  /// fence will be signaled, are:
  ///
  ///   - `VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT` specifies that the fence is
  ///     signaled when the first pixel of the next display refresh cycle leaves the
  ///     display engine for the display.
  ///
  pub enum VkDisplayEventTypeEXT {
    E_FIRST_PIXEL_OUT_EXT = 0
  }
}

// feature: VK_KHX_device_group

#[cfg(feature = "VK_KHX_device_group")]
define_bitmask! {
  pub enum VkPeerMemoryFeatureFlagBitsKHX {
    BIT_COPY_SRC_BIT_KHX = 1<<0,
    BIT_COPY_DST_BIT_KHX = 1<<1,
    BIT_GENERIC_SRC_BIT_KHX = 1<<2,
    BIT_GENERIC_DST_BIT_KHX = 1<<3
  }
}

#[cfg(feature = "VK_KHX_device_group")]
define_bitmask! {
  pub enum VkMemoryAllocateFlagBitsKHX {
    BIT_DEVICE_MASK_BIT_KHX = 1<<0
  }
}

#[cfg(feature = "VK_KHX_device_group")]
define_bitmask! {

  /// Bitmask specifying supported device group present modes
  ///
  /// Bits which may: be set in `VkDeviceGroupPresentCapabilitiesKHX::modes` to
  /// indicate which device group presentation modes are supported are:
  ///
  ///   - `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX` indicates that any physical
  ///     device with a presentation engine can: present its own swapchain images.
  ///
  ///   - `VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHX` indicates that any physical
  ///     device with a presentation engine can: present swapchain images from any
  ///     physical device in its `presentMask`.
  ///
  ///   - `VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHX` indicates that any physical
  ///     device with a presentation engine can: present the sum of swapchain images
  ///     from any physical devices in its `presentMask`.
  ///
  ///   - `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHX` indicates that
  ///     multiple physical devices with a presentation engine can: each present their
  ///     own swapchain images.
  ///
  pub enum VkDeviceGroupPresentModeFlagBitsKHX {
    BIT_LOCAL_BIT_KHX = 1<<0,
    BIT_REMOTE_BIT_KHX = 1<<1,
    BIT_SUM_BIT_KHX = 1<<2,
    BIT_LOCAL_MULTI_DEVICE_BIT_KHX = 1<<3
  }
}

// feature: VK_KHR_swapchain

#[cfg(feature = "VK_KHR_swapchain")]
define_bitmask! {

  /// Bitmask controlling swapchain creation
  ///
  /// Bits which can: be set in `VkSwapchainCreateInfoKHR::flags`, specifying
  /// parameters of swapchain creation, are:
  ///
  ///   - `VK_SWAPCHAIN_CREATE_BIND_SFR_BIT_KHX` specifies that images created from
  ///     the swapchain (i.e. with the `swapchain` member of
  ///     `VkImageSwapchainCreateInfoKHX` set to this swapchain’s handle) must: use
  ///     `VK_IMAGE_CREATE_BIND_SFR_BIT_KHX`.
  ///
  pub enum VkSwapchainCreateFlagBitsKHR {

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    BIT_BIND_SFR_BIT_KHX = 1<<0
  }
}

// feature: VK_NV_viewport_swizzle

#[cfg(feature = "VK_NV_viewport_swizzle")]
define_enum! {

  /// Specify how a viewport coordinate is swizzled
  ///
  /// Possible values of the `VkViewportSwizzleNV::x`, `y`, `z`, and `w` members,
  /// specifying swizzling of the corresponding components of primitives.
  ///
  /// These values are described in detail in [Viewport
  /// Swizzle](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vertexpostproc-viewport-swizzle).
  ///
  pub enum VkViewportCoordinateSwizzleNV {
    E_POSITIVE_X_NV = 0,
    E_NEGATIVE_X_NV = 1,
    E_POSITIVE_Y_NV = 2,
    E_NEGATIVE_Y_NV = 3,
    E_POSITIVE_Z_NV = 4,
    E_NEGATIVE_Z_NV = 5,
    E_POSITIVE_W_NV = 6,
    E_NEGATIVE_W_NV = 7
  }
}

// feature: VK_EXT_discard_rectangles

#[cfg(feature = "VK_EXT_discard_rectangles")]
define_enum! {

  /// Specify the discard rectangle mode
  ///
  /// Possible values of
  /// `VkPipelineDiscardRectangleStateCreateInfoEXT::discardRectangleMode`, specifying
  /// the behavior of the discard rectangle test.
  ///
  ///   - `VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT` specifies that a fragment within
  ///     any discard rectangle satisfies the test.
  ///
  ///   - `VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT` specifies that a fragment not
  ///     within any of the discard rectangles satisfies the test.
  ///
  pub enum VkDiscardRectangleModeEXT {
    E_INCLUSIVE_EXT = 0,
    E_EXCLUSIVE_EXT = 1
  }
}

// feature: VK_VERSION_1_0

define_bitmask! {

  /// Bitmask specifying usage of a subpass
  ///
  /// Bits which can: be set in `VkSubpassDescription::flags`, specifying usage of the
  /// subpass.
  ///
  ///   - `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` specifies that shaders
  ///     compiled for this subpass write the attributes for all views in a single
  ///     invocation of each vertex processing stage. All pipelines compiled against a
  ///     subpass that includes this bit must: write per-view attributes to the
  ///     code:\*PerViewNV\[\] shader outputs, in addition to the non-per-view (e.g.
  ///     `Position`) outputs.
  ///
  ///   - `VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX` specifies that
  ///     shaders compiled for this subpass use per-view positions which only differ
  ///     in value in the x component. Per-view viewport mask can: also be used.
  ///
  pub enum VkSubpassDescriptionFlagBits {

    // feature: VK_NVX_multiview_per_view_attributes
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    BIT_PER_VIEW_ATTRIBUTES_BIT_NVX = 1<<0,
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    BIT_PER_VIEW_POSITION_X_ONLY_BIT_NVX = 1<<1
  }
}

// feature: VK_KHR_maintenance2

#[cfg(feature = "VK_KHR_maintenance2")]
define_enum! {

  /// Enum specifying the point clipping behaviour
  ///
  /// Possible values of
  /// `VkPhysicalDevicePointClippingPropertiesKHR::pointClippingBehavior`, specifying
  /// clipping behavior of a point primitive whose vertex lies outside the clip
  /// volume.
  ///
  ///   - `VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR` specifies that the
  ///     primitive is discarded if the vertex lies outside any clip plane, including
  ///     the planes bounding the view volume.
  ///
  ///   - `VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR` specifies that the
  ///     primitive is discarded only if the vertex lies outside any user clip plane.
  ///
  pub enum VkPointClippingBehaviorKHR {
    E_ALL_CLIP_PLANES_KHR = 0,
    E_USER_CLIP_PLANES_ONLY_KHR = 1
  }
}

// feature: VK_EXT_sampler_filter_minmax

#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
define_enum! {

  /// Specify reduction mode for texture filtering
  ///
  /// Reduction modes are specified by `VkSamplerReductionModeEXT`, which takes
  /// values.
  ///
  ///   - `VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT` indicates that texel values
  ///     are combined by computing a weighted average of values in the footprint,
  ///     using weights as specified in [the image operations
  ///     chapter](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-unnormalized-to-integer).
  ///
  ///   - `VK_SAMPLER_REDUCTION_MODE_MIN_EXT` indicates that texel values are combined
  ///     by taking the component-wise minimum of values in the footprint with
  ///     non-zero weights.
  ///
  ///   - `VK_SAMPLER_REDUCTION_MODE_MAX_EXT` indicates that texel values are combined
  ///     by taking the component-wise maximum of values in the footprint with
  ///     non-zero weights.
  ///
  pub enum VkSamplerReductionModeEXT {
    E_WEIGHTED_AVERAGE_EXT = 0,
    E_MIN_EXT = 1,
    E_MAX_EXT = 2
  }
}

// feature: VK_KHR_maintenance2

#[cfg(feature = "VK_KHR_maintenance2")]
define_enum! {

  /// Enum describing tessellation domain origin
  ///
  /// The possible tessellation domain origins are specified by the
  /// `VkTessellationDomainOriginKHR` enumeration.
  ///
  ///   - `VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR` indicates that the origin of
  ///     the domain space is in the upper left corner, as shown in figure
  ///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#img-tessellation-topology-ul).
  ///
  ///   - `VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR` indicates that the origin of
  ///     the domain space is in the lower left corner, as shown in figure
  ///     [???](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#img-tessellation-topology-ll).
  ///
  /// This enum affects how the `VertexOrderCw` and `VertexOrderCcw` tessellation
  /// execution modes are interpreted, since the winding is defined relative to the
  /// orientation of the domain.
  ///
  pub enum VkTessellationDomainOriginKHR {
    E_UPPER_LEFT_KHR = 0,
    E_LOWER_LEFT_KHR = 1
  }
}

// feature: VK_KHR_sampler_ycbcr_conversion

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
define_enum! {

  /// Color model component of a color space
  ///
  /// `VkSamplerYcbcrModelConversionKHR` defines the conversion from the source color
  /// model to the shader color model. Possible values are.
  ///
  ///   - `VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR` specifies that the
  ///     input values to the conversion are unmodified.
  ///
  ///   - `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR` specifies no model
  ///     conversion but the inputs are range expanded as for
  ///     Y’C<sub>B</sub>C<sub>R</sub>.
  ///
  ///   - `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR` specifies the color model
  ///     conversion from Y’C<sub>B</sub>C<sub>R</sub> to R’G’B' defined in BT.709 and
  ///     described in the “BT.709 Y’C<sub>B</sub>C<sub>R</sub> conversion” section of
  ///     the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#data-format).
  ///
  ///   - `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR` specifies the color model
  ///     conversion from Y’C<sub>B</sub>C<sub>R</sub> to R’G’B' defined in BT.601 and
  ///     described in the “BT.601 Y’C<sub>B</sub>C<sub>R</sub> conversion” section of
  ///     the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#data-format).
  ///
  ///   - `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR` specifies the color model
  ///     conversion from Y’C<sub>B</sub>C<sub>R</sub> to R’G’B' defined in BT.2020
  ///     and described in the “BT.2020 Y’C<sub>B</sub>C<sub>R</sub> conversion”
  ///     section of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#data-format).
  ///
  /// In the etext:VK\_SAMPLER\_YCBCR\_MODEL\_CONVERSION\_YCBCR\_\*\_KHR color models,
  /// for the input to the sampler Y’C<sub>B</sub>C<sub>R</sub> range expansion and
  /// model conversion:
  ///
  ///   - the Y (Y' luma) channel corresponds to the G channel of an RGB image.
  ///
  ///   - the CB (C<sub>B</sub> or “U” blue color difference) channel corresponds to
  ///     the B channel of an RGB image.
  ///
  ///   - the CR (C<sub>R</sub> or “V” red color difference) channel corresponds to
  ///     the R channel of an RGB image.
  ///
  ///   - the alpha channel, if present, is not modified by color model conversion.
  ///
  /// These rules reflect the mapping of channels after the channel swizzle operation
  /// (controlled by `VkSamplerYcbcrConversionCreateInfoKHR::components`).
  ///
  /// > **Note**
  /// >
  /// > For example, an “YUVA” 32-bit format comprising four 8-bit channels can be
  /// > implemented as `VK_FORMAT_R8G8B8A8_UNORM` with a component mapping:
  /// >
  /// >   - `components`.a = `VK_COMPONENT_SWIZZLE_IDENTITY`
  /// >
  /// >   - `components`.r = `VK_COMPONENT_SWIZZLE_B`
  /// >
  /// >   - `components`.g = `VK_COMPONENT_SWIZZLE_R`
  /// >
  /// >   - `components`.b = `VK_COMPONENT_SWIZZLE_G`
  ///
  pub enum VkSamplerYcbcrModelConversionKHR {
    E_RGB_IDENTITY_KHR = 0,
    E_YCBCR_IDENTITY_KHR = 1,
    E_YCBCR_709_KHR = 2,
    E_YCBCR_601_KHR = 3,
    E_YCBCR_2020_KHR = 4
  }
}

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
define_enum! {

  /// Range of encoded values in a color space
  ///
  /// The `VkSamplerYcbcrRangeKHR` enum describes whether color channels are encoded
  /// using the full range of numerical values or whether values are reserved for
  /// headroom and foot room. `VkSamplerYcbcrRangeKHR` is defined as.
  ///
  ///   - `VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR` indicates that the full range of the
  ///     encoded values are valid and interpreted according to the ITU “full range”
  ///     quantization rules.
  ///
  ///   - `VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR` indicates that headroom and foot
  ///     room are reserved in the numerical range of encoded values, and the
  ///     remaining values are expanded according to the ITU “narrow range”
  ///     quantization rules.
  ///
  /// The formulae for these conversions is described in the [Sampler
  /// Y’C<sub>B</sub>C<sub>R</sub> Range
  /// Expansion](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures-sampler-YCbCr-conversion-rangeexpand) section of the [Image
  /// Operations](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#textures) chapter.
  ///
  /// No range modification takes place if `ycbcrModel` is
  /// `VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR`; the `ycbcrRange` field of
  /// `VkSamplerYcbcrConversionCreateInfoKHR` is ignored in this case.
  ///
  pub enum VkSamplerYcbcrRangeKHR {
    E_ITU_FULL_KHR = 0,
    E_ITU_NARROW_KHR = 1
  }
}

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
define_enum! {

  /// Position of downsampled chroma samples
  ///
  /// The `VkChromaLocationKHR` enum, which defines the location of downsampled chroma
  /// channel samples relative to the luma samples, is defined as.
  ///
  ///   - `VK_CHROMA_LOCATION_COSITED_EVEN_KHR` indicates that downsampled chroma
  ///     samples are aligned with luma samples with even coordinates.
  ///
  ///   - `VK_CHROMA_LOCATION_MIDPOINT_KHR` indicates that downsampled chroma samples
  ///     are located half way between each even luma sample and the nearest higher
  ///     odd luma sample.
  ///
  pub enum VkChromaLocationKHR {
    E_COSITED_EVEN_KHR = 0,
    E_MIDPOINT_KHR = 1
  }
}

// feature: VK_EXT_blend_operation_advanced

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
define_enum! {

  /// Enumerant specifying the blend overlap parameter
  ///
  /// When blending using advanced blend operations, we expect that the R, G, and B
  /// components of premultiplied source and destination color inputs be stored as the
  /// product of non-premultiplied R, G, and B component values and the A component of
  /// the color. If any R, G, or B component of a premultiplied input color is
  /// non-zero and the A component is zero, the color is considered ill-formed, and
  /// the corresponding component of the blend result is undefined.
  ///
  /// The weighting functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> are
  /// defined in table [Advanced Blend Overlap
  /// Modes](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#framebuffer-blend-advanced-overlap-modes). In these functions, the A
  /// components of the source and destination colors are taken to indicate the
  /// portion of the pixel covered by the fragment (source) and the fragments
  /// previously accumulated in the pixel (destination). The functions p<sub>0</sub>,
  /// p<sub>1</sub>, and p<sub>2</sub> approximate the relative portion of the pixel
  /// covered by the intersection of the source and destination, covered only by the
  /// source, and covered only by the destination, respectively.
  ///
  /// Possible values of
  /// `VkPipelineColorBlendAdvancedStateCreateInfoEXT::blendOverlap`, specifying the
  /// blend overlap functions, are:
  ///
  ///   - `VK_BLEND_OVERLAP_UNCORRELATED_EXT` specifies that there is no correlation
  ///     between the source and destination coverage.
  ///
  ///   - `VK_BLEND_OVERLAP_CONJOINT_EXT` specifies that the source and destination
  ///     coverage are considered to have maximal overlap.
  ///
  ///   - `VK_BLEND_OVERLAP_DISJOINT_EXT` specifies that the source and destination
  ///     coverage are considered to have minimal overlap.
  ///
  /// <table>
  /// <caption>Advanced Blend Overlap Modes</caption>
  /// <colgroup>
  /// <col width="50%" />
  /// <col width="50%" />
  /// </colgroup>
  /// <thead>
  /// <tr class="header">
  /// <th align="left">Overlap Mode</th>
  /// <th align="left">Weighting Equations</th>
  /// </tr>
  /// </thead>
  /// <tbody>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_OVERLAP_UNCORRELATED_EXT</code></p></td>
  /// <td align="left"></td>
  /// </tr>
  /// <tr class="even">
  /// <td align="left"><p><code>VK_BLEND_OVERLAP_CONJOINT_EXT</code></p></td>
  /// <td align="left"></td>
  /// </tr>
  /// <tr class="odd">
  /// <td align="left"><p><code>VK_BLEND_OVERLAP_DISJOINT_EXT</code></p></td>
  /// <td align="left"></td>
  /// </tr>
  /// </tbody>
  /// </table>
  ///
  pub enum VkBlendOverlapEXT {
    E_UNCORRELATED_EXT = 0,
    E_DISJOINT_EXT = 1,
    E_CONJOINT_EXT = 2
  }
}

// feature: VK_NV_framebuffer_mixed_samples

#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
define_enum! {

  /// Specify the discard rectangle mode
  ///
  /// Possible values of
  /// `VkPipelineCoverageModulationStateCreateInfoNV::coverageModulationMode`,
  /// specifying which color components are modulated.
  ///
  ///   - `VK_COVERAGE_MODULATION_MODE_NONE_NV` specifies that no components are
  ///     multiplied by the modulation factor.
  ///
  ///   - `VK_COVERAGE_MODULATION_MODE_RGB_NV` specifies that the red, green, and blue
  ///     components are multiplied by the modulation factor.
  ///
  ///   - `VK_COVERAGE_MODULATION_MODE_ALPHA_NV` specifies that the alpha component is
  ///     multiplied by the modulation factor.
  ///
  ///   - `VK_COVERAGE_MODULATION_MODE_RGBA_NV` specifies that all components are
  ///     multiplied by the modulation factor.
  ///
  pub enum VkCoverageModulationModeNV {
    E_NONE_NV = 0,
    E_RGB_NV = 1,
    E_ALPHA_NV = 2,
    E_RGBA_NV = 3
  }
}

// feature: VK_EXT_validation_cache

#[cfg(feature = "VK_EXT_validation_cache")]
define_enum! {

  /// Encode validation cache version
  ///
  /// Possible values of the second group of four bytes in the header returned by
  /// `vkGetValidationCacheDataEXT`, encoding the validation cache version, are.
  ///
  ///   - `VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT` specifies version one of the
  ///     validation cache.
  ///
  pub enum VkValidationCacheHeaderVersionEXT {
    E_ONE_EXT = 1
  }
}

// feature: VK_AMD_shader_info

#[cfg(feature = "VK_AMD_shader_info")]
define_enum! {
  pub enum VkShaderInfoTypeAMD {
    E_STATISTICS_AMD = 0,
    E_BINARY_AMD = 1,
    E_DISASSEMBLY_AMD = 2
  }
}

// feature: VK_EXT_global_priority

#[cfg(feature = "VK_EXT_global_priority")]
define_enum! {

  /// Values specifying a system-wide queue priority
  ///
  /// Possible values of `VkDeviceQueueGlobalPriorityCreateInfoEXT::globalPriority`,
  /// specifying a system-wide priority level are.
  ///
  /// Priority values are sorted in ascending order. A comparison operation on the
  /// enum values can be used to determine the priority order.
  ///
  ///   - `VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT` is below the system default. Useful for
  ///     non-interactive tasks.
  ///
  ///   - `VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT` is the system default priority.
  ///
  ///   - `VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT` is above the system default.
  ///
  ///   - `VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT` is the highest priority. Useful for
  ///     critical tasks.
  ///
  pub enum VkQueueGlobalPriorityEXT {
    E_LOW_EXT = 128,
    E_MEDIUM_EXT = 256,
    E_HIGH_EXT = 512,
    E_REALTIME_EXT = 1024
  }
}

// feature: VK_EXT_conservative_rasterization

#[cfg(feature = "VK_EXT_conservative_rasterization")]
define_enum! {

  /// Specify the conservative rasterization mode
  ///
  /// Possible values of
  /// `VkPipelineRasterizationConservativeStateCreateInfoEXT::conservativeRasterizationMode`,
  /// specifying the conservative rasterization mode are.
  ///
  ///   - `VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT` specifies that
  ///     conservative rasterization is disabled and rasterization proceeds as normal.
  ///
  ///   - `VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT` specifies that
  ///     conservative rasterization is enabled in overestimation mode.
  ///
  ///   - `VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT` specifies that
  ///     conservative rasterization is enabled in underestimation mode.
  ///
  pub enum VkConservativeRasterizationModeEXT {
    E_DISABLED_EXT = 0,
    E_OVERESTIMATE_EXT = 1,
    E_UNDERESTIMATE_EXT = 2
  }
}

// feature: VK_KHR_surface

// VK_KHR_surface
///////////////////
#[cfg(feature = "VK_KHR_surface")]
pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
#[cfg(feature = "VK_KHR_surface")]
pub const VK_KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface\0";
#[cfg(feature = "VK_KHR_surface")]
pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VkColorSpaceKHR::E_SRGB_NONLINEAR_KHR;

// feature: VK_KHR_swapchain

// VK_KHR_swapchain
/////////////////////
#[cfg(feature = "VK_KHR_swapchain")]
pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 68;
#[cfg(feature = "VK_KHR_swapchain")]
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain\0";

// feature: VK_KHR_display

// VK_KHR_display
///////////////////
#[cfg(feature = "VK_KHR_display")]
pub const VK_KHR_DISPLAY_SPEC_VERSION: u32 = 21;
#[cfg(feature = "VK_KHR_display")]
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &str = "VK_KHR_display\0";

// feature: VK_KHR_display_swapchain

// VK_KHR_display_swapchain
/////////////////////////////
#[cfg(feature = "VK_KHR_display_swapchain")]
pub const VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 9;
#[cfg(feature = "VK_KHR_display_swapchain")]
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_display_swapchain\0";

// feature: VK_KHR_xlib_surface

// VK_KHR_xlib_surface
////////////////////////
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
#[cfg(feature = "VK_KHR_xlib_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &str = "VK_KHR_xlib_surface\0";

// feature: VK_KHR_xcb_surface

// VK_KHR_xcb_surface
///////////////////////
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
#[cfg(feature = "VK_KHR_xcb_surface")]
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &str = "VK_KHR_xcb_surface\0";

// feature: VK_KHR_wayland_surface

// VK_KHR_wayland_surface
///////////////////////////
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
#[cfg(feature = "VK_KHR_wayland_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &str = "VK_KHR_wayland_surface\0";

// feature: VK_KHR_mir_surface

// VK_KHR_mir_surface
///////////////////////
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub const VK_KHR_MIR_SURFACE_SPEC_VERSION: u32 = 4;
#[cfg(feature = "VK_KHR_mir_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub const VK_KHR_MIR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_mir_surface\0";

// feature: VK_KHR_android_surface

// VK_KHR_android_surface
///////////////////////////
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub const VK_KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;
#[cfg(feature = "VK_KHR_android_surface")]
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub const VK_KHR_ANDROID_SURFACE_EXTENSION_NAME: &str = "VK_KHR_android_surface\0";

// feature: VK_KHR_win32_surface

// VK_KHR_win32_surface
/////////////////////////
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
#[cfg(feature = "VK_KHR_win32_surface")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &str = "VK_KHR_win32_surface\0";

// feature: VK_EXT_debug_report

// VK_EXT_debug_report
////////////////////////
#[cfg(feature = "VK_EXT_debug_report")]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 9;
#[cfg(feature = "VK_EXT_debug_report")]
pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &str = "VK_EXT_debug_report\0";
#[cfg(feature = "VK_EXT_debug_report")]
pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT: VkStructureType =
  VkStructureType::E_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
#[cfg(feature = "VK_EXT_debug_report")]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT::E_DEBUG_REPORT_CALLBACK_EXT_EXT;

// feature: VK_NV_glsl_shader

// VK_NV_glsl_shader
//////////////////////
#[cfg(feature = "VK_NV_glsl_shader")]
pub const VK_NV_GLSL_SHADER_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_glsl_shader")]
pub const VK_NV_GLSL_SHADER_EXTENSION_NAME: &str = "VK_NV_glsl_shader\0";

// feature: VK_EXT_depth_range_unrestricted

// VK_EXT_depth_range_unrestricted
////////////////////////////////////
#[cfg(feature = "VK_EXT_depth_range_unrestricted")]
pub const VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_depth_range_unrestricted")]
pub const VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME: &str = "VK_EXT_depth_range_unrestricted\0";

// feature: VK_KHR_sampler_mirror_clamp_to_edge

// VK_KHR_sampler_mirror_clamp_to_edge
////////////////////////////////////////
#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME: &str = "VK_KHR_sampler_mirror_clamp_to_edge\0";

// feature: VK_IMG_filter_cubic

// VK_IMG_filter_cubic
////////////////////////
#[cfg(feature = "VK_IMG_filter_cubic")]
pub const VK_IMG_FILTER_CUBIC_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_IMG_filter_cubic")]
pub const VK_IMG_FILTER_CUBIC_EXTENSION_NAME: &str = "VK_IMG_filter_cubic\0";

// feature: VK_AMD_rasterization_order

// VK_AMD_rasterization_order
///////////////////////////////
#[cfg(feature = "VK_AMD_rasterization_order")]
pub const VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_rasterization_order")]
pub const VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &str = "VK_AMD_rasterization_order\0";

// feature: VK_AMD_shader_trinary_minmax

// VK_AMD_shader_trinary_minmax
/////////////////////////////////
#[cfg(feature = "VK_AMD_shader_trinary_minmax")]
pub const VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_shader_trinary_minmax")]
pub const VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME: &str = "VK_AMD_shader_trinary_minmax\0";

// feature: VK_AMD_shader_explicit_vertex_parameter

// VK_AMD_shader_explicit_vertex_parameter
////////////////////////////////////////////
#[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
pub const VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
pub const VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME: &str = "VK_AMD_shader_explicit_vertex_parameter\0";

// feature: VK_EXT_debug_marker

// VK_EXT_debug_marker
////////////////////////
#[cfg(feature = "VK_EXT_debug_marker")]
pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
#[cfg(feature = "VK_EXT_debug_marker")]
pub const VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &str = "VK_EXT_debug_marker\0";

// feature: VK_AMD_gcn_shader

// VK_AMD_gcn_shader
//////////////////////
#[cfg(feature = "VK_AMD_gcn_shader")]
pub const VK_AMD_GCN_SHADER_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_gcn_shader")]
pub const VK_AMD_GCN_SHADER_EXTENSION_NAME: &str = "VK_AMD_gcn_shader\0";

// feature: VK_NV_dedicated_allocation

// VK_NV_dedicated_allocation
///////////////////////////////
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub const VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub const VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &str = "VK_NV_dedicated_allocation\0";

// feature: VK_AMD_draw_indirect_count

// VK_AMD_draw_indirect_count
///////////////////////////////
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub const VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub const VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &str = "VK_AMD_draw_indirect_count\0";

// feature: VK_AMD_negative_viewport_height

// VK_AMD_negative_viewport_height
////////////////////////////////////
#[cfg(feature = "VK_AMD_negative_viewport_height")]
pub const VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_negative_viewport_height")]
pub const VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME: &str = "VK_AMD_negative_viewport_height\0";

// feature: VK_AMD_gpu_shader_half_float

// VK_AMD_gpu_shader_half_float
/////////////////////////////////
#[cfg(feature = "VK_AMD_gpu_shader_half_float")]
pub const VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_gpu_shader_half_float")]
pub const VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME: &str = "VK_AMD_gpu_shader_half_float\0";

// feature: VK_AMD_shader_ballot

// VK_AMD_shader_ballot
/////////////////////////
#[cfg(feature = "VK_AMD_shader_ballot")]
pub const VK_AMD_SHADER_BALLOT_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_shader_ballot")]
pub const VK_AMD_SHADER_BALLOT_EXTENSION_NAME: &str = "VK_AMD_shader_ballot\0";

// feature: VK_AMD_texture_gather_bias_lod

// VK_AMD_texture_gather_bias_lod
///////////////////////////////////
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &str = "VK_AMD_texture_gather_bias_lod\0";

// feature: VK_AMD_shader_info

// VK_AMD_shader_info
///////////////////////
#[cfg(feature = "VK_AMD_shader_info")]
pub const VK_AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_shader_info")]
pub const VK_AMD_SHADER_INFO_EXTENSION_NAME: &str = "VK_AMD_shader_info\0";

// feature: VK_AMD_shader_image_load_store_lod

// VK_AMD_shader_image_load_store_lod
///////////////////////////////////////
#[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
pub const VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
pub const VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME: &str = "VK_AMD_shader_image_load_store_lod\0";

// feature: VK_KHX_multiview

// VK_KHX_multiview
/////////////////////
#[cfg(feature = "VK_KHX_multiview")]
pub const VK_KHX_MULTIVIEW_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHX_multiview")]
pub const VK_KHX_MULTIVIEW_EXTENSION_NAME: &str = "VK_KHX_multiview\0";

// feature: VK_IMG_format_pvrtc

// VK_IMG_format_pvrtc
////////////////////////
#[cfg(feature = "VK_IMG_format_pvrtc")]
pub const VK_IMG_FORMAT_PVRTC_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_IMG_format_pvrtc")]
pub const VK_IMG_FORMAT_PVRTC_EXTENSION_NAME: &str = "VK_IMG_format_pvrtc\0";

// feature: VK_NV_external_memory_capabilities

// VK_NV_external_memory_capabilities
///////////////////////////////////////
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &str = "VK_NV_external_memory_capabilities\0";

// feature: VK_NV_external_memory

// VK_NV_external_memory
//////////////////////////
#[cfg(feature = "VK_NV_external_memory")]
pub const VK_NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_external_memory")]
pub const VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME: &str = "VK_NV_external_memory\0";

// feature: VK_NV_external_memory_win32

// VK_NV_external_memory_win32
////////////////////////////////
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &str = "VK_NV_external_memory_win32\0";

// feature: VK_NV_win32_keyed_mutex

// VK_NV_win32_keyed_mutex
////////////////////////////
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME: &str = "VK_NV_win32_keyed_mutex\0";

// feature: VK_KHR_get_physical_device_properties2

// VK_KHR_get_physical_device_properties2
///////////////////////////////////////////
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &str = "VK_KHR_get_physical_device_properties2\0";

// feature: VK_KHX_device_group

// VK_KHX_device_group
////////////////////////
#[cfg(feature = "VK_KHX_device_group")]
pub const VK_KHX_DEVICE_GROUP_SPEC_VERSION: u32 = 2;
#[cfg(feature = "VK_KHX_device_group")]
pub const VK_KHX_DEVICE_GROUP_EXTENSION_NAME: &str = "VK_KHX_device_group\0";

// feature: VK_EXT_validation_flags

// VK_EXT_validation_flags
////////////////////////////
#[cfg(feature = "VK_EXT_validation_flags")]
pub const VK_EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_validation_flags")]
pub const VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME: &str = "VK_EXT_validation_flags\0";

// feature: VK_NN_vi_surface

// VK_NN_vi_surface
/////////////////////
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub const VK_NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NN_vi_surface")]
#[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
pub const VK_NN_VI_SURFACE_EXTENSION_NAME: &str = "VK_NN_vi_surface\0";

// feature: VK_KHR_shader_draw_parameters

// VK_KHR_shader_draw_parameters
//////////////////////////////////
#[cfg(feature = "VK_KHR_shader_draw_parameters")]
pub const VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_shader_draw_parameters")]
pub const VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME: &str = "VK_KHR_shader_draw_parameters\0";

// feature: VK_EXT_shader_subgroup_ballot

// VK_EXT_shader_subgroup_ballot
//////////////////////////////////
#[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
pub const VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
pub const VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME: &str = "VK_EXT_shader_subgroup_ballot\0";

// feature: VK_EXT_shader_subgroup_vote

// VK_EXT_shader_subgroup_vote
////////////////////////////////
#[cfg(feature = "VK_EXT_shader_subgroup_vote")]
pub const VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_shader_subgroup_vote")]
pub const VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME: &str = "VK_EXT_shader_subgroup_vote\0";

// feature: VK_KHR_maintenance1

// VK_KHR_maintenance1
////////////////////////
#[cfg(feature = "VK_KHR_maintenance1")]
pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_maintenance1")]
pub const VK_KHR_MAINTENANCE1_EXTENSION_NAME: &str = "VK_KHR_maintenance1\0";

// feature: VK_KHX_device_group_creation

// VK_KHX_device_group_creation
/////////////////////////////////
#[cfg(feature = "VK_KHX_device_group_creation")]
pub const VK_KHX_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHX_device_group_creation")]
pub const VK_KHX_DEVICE_GROUP_CREATION_EXTENSION_NAME: &str = "VK_KHX_device_group_creation\0";

// feature: VK_KHR_external_memory_capabilities

// VK_KHR_external_memory_capabilities
////////////////////////////////////////
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &str = "VK_KHR_external_memory_capabilities\0";

// feature: VK_KHR_external_memory

// VK_KHR_external_memory
///////////////////////////
#[cfg(feature = "VK_KHR_external_memory")]
pub const VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_memory")]
pub const VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &str = "VK_KHR_external_memory\0";

// feature: VK_KHR_external_memory_win32

// VK_KHR_external_memory_win32
/////////////////////////////////
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_memory_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &str = "VK_KHR_external_memory_win32\0";

// feature: VK_KHR_external_memory_fd

// VK_KHR_external_memory_fd
//////////////////////////////
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub const VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub const VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &str = "VK_KHR_external_memory_fd\0";

// feature: VK_KHR_win32_keyed_mutex

// VK_KHR_win32_keyed_mutex
/////////////////////////////
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &str = "VK_KHR_win32_keyed_mutex\0";

// feature: VK_KHR_external_semaphore_capabilities

// VK_KHR_external_semaphore_capabilities
///////////////////////////////////////////
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &str = "VK_KHR_external_semaphore_capabilities\0";

// feature: VK_KHR_external_semaphore

// VK_KHR_external_semaphore
//////////////////////////////
#[cfg(feature = "VK_KHR_external_semaphore")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_semaphore")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &str = "VK_KHR_external_semaphore\0";

// feature: VK_KHR_external_semaphore_win32

// VK_KHR_external_semaphore_win32
////////////////////////////////////
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &str = "VK_KHR_external_semaphore_win32\0";

// feature: VK_KHR_external_semaphore_fd

// VK_KHR_external_semaphore_fd
/////////////////////////////////
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &str = "VK_KHR_external_semaphore_fd\0";

// feature: VK_KHR_push_descriptor

// VK_KHR_push_descriptor
///////////////////////////
#[cfg(feature = "VK_KHR_push_descriptor")]
pub const VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_push_descriptor")]
pub const VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &str = "VK_KHR_push_descriptor\0";

// feature: VK_KHR_16bit_storage

// VK_KHR_16bit_storage
/////////////////////////
#[cfg(feature = "VK_KHR_16bit_storage")]
pub const VK_KHR_16BIT_STORAGE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_16bit_storage")]
pub const VK_KHR_16BIT_STORAGE_EXTENSION_NAME: &str = "VK_KHR_16bit_storage\0";

// feature: VK_KHR_incremental_present

// VK_KHR_incremental_present
///////////////////////////////
#[cfg(feature = "VK_KHR_incremental_present")]
pub const VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_incremental_present")]
pub const VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &str = "VK_KHR_incremental_present\0";

// feature: VK_KHR_descriptor_update_template

// VK_KHR_descriptor_update_template
//////////////////////////////////////
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &str = "VK_KHR_descriptor_update_template\0";

// feature: VK_NVX_device_generated_commands

// VK_NVX_device_generated_commands
/////////////////////////////////////
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub const VK_NVX_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
#[cfg(feature = "VK_NVX_device_generated_commands")]
pub const VK_NVX_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &str = "VK_NVX_device_generated_commands\0";

// feature: VK_NV_clip_space_w_scaling

// VK_NV_clip_space_w_scaling
///////////////////////////////
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub const VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub const VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &str = "VK_NV_clip_space_w_scaling\0";

// feature: VK_EXT_direct_mode_display

// VK_EXT_direct_mode_display
///////////////////////////////
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub const VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub const VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &str = "VK_EXT_direct_mode_display\0";

// feature: VK_EXT_acquire_xlib_display

// VK_EXT_acquire_xlib_display
////////////////////////////////
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
#[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &str = "VK_EXT_acquire_xlib_display\0";

// feature: VK_EXT_display_surface_counter

// VK_EXT_display_surface_counter
///////////////////////////////////
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &str = "VK_EXT_display_surface_counter\0";
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT: VkStructureType = VkStructureType::E_SURFACE_CAPABILITIES_2_EXT;

// feature: VK_EXT_display_control

// VK_EXT_display_control
///////////////////////////
#[cfg(feature = "VK_EXT_display_control")]
pub const VK_EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_display_control")]
pub const VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME: &str = "VK_EXT_display_control\0";

// feature: VK_GOOGLE_display_timing

// VK_GOOGLE_display_timing
/////////////////////////////
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub const VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub const VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &str = "VK_GOOGLE_display_timing\0";

// feature: VK_NV_sample_mask_override_coverage

// VK_NV_sample_mask_override_coverage
////////////////////////////////////////
#[cfg(feature = "VK_NV_sample_mask_override_coverage")]
pub const VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_sample_mask_override_coverage")]
pub const VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME: &str = "VK_NV_sample_mask_override_coverage\0";

// feature: VK_NV_geometry_shader_passthrough

// VK_NV_geometry_shader_passthrough
//////////////////////////////////////
#[cfg(feature = "VK_NV_geometry_shader_passthrough")]
pub const VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_geometry_shader_passthrough")]
pub const VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME: &str = "VK_NV_geometry_shader_passthrough\0";

// feature: VK_NV_viewport_array2

// VK_NV_viewport_array2
//////////////////////////
#[cfg(feature = "VK_NV_viewport_array2")]
pub const VK_NV_VIEWPORT_ARRAY2_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_viewport_array2")]
pub const VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME: &str = "VK_NV_viewport_array2\0";

// feature: VK_NVX_multiview_per_view_attributes

// VK_NVX_multiview_per_view_attributes
/////////////////////////////////////////
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &str = "VK_NVX_multiview_per_view_attributes\0";

// feature: VK_NV_viewport_swizzle

// VK_NV_viewport_swizzle
///////////////////////////
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub const VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &str = "VK_NV_viewport_swizzle\0";

// feature: VK_EXT_discard_rectangles

// VK_EXT_discard_rectangles
//////////////////////////////
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub const VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub const VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &str = "VK_EXT_discard_rectangles\0";

// feature: VK_EXT_conservative_rasterization

// VK_EXT_conservative_rasterization
//////////////////////////////////////
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &str = "VK_EXT_conservative_rasterization\0";

// feature: VK_EXT_swapchain_colorspace

// VK_EXT_swapchain_colorspace
////////////////////////////////
#[cfg(feature = "VK_EXT_swapchain_colorspace")]
pub const VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION: u32 = 3;
#[cfg(feature = "VK_EXT_swapchain_colorspace")]
pub const VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME: &str = "VK_EXT_swapchain_colorspace\0";

// feature: VK_EXT_hdr_metadata

// VK_EXT_hdr_metadata
////////////////////////
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub const VK_EXT_HDR_METADATA_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub const VK_EXT_HDR_METADATA_EXTENSION_NAME: &str = "VK_EXT_hdr_metadata\0";

// feature: VK_KHR_shared_presentable_image

// VK_KHR_shared_presentable_image
////////////////////////////////////
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &str = "VK_KHR_shared_presentable_image\0";

// feature: VK_KHR_external_fence_capabilities

// VK_KHR_external_fence_capabilities
///////////////////////////////////////
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &str = "VK_KHR_external_fence_capabilities\0";

// feature: VK_KHR_external_fence

// VK_KHR_external_fence
//////////////////////////
#[cfg(feature = "VK_KHR_external_fence")]
pub const VK_KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_fence")]
pub const VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME: &str = "VK_KHR_external_fence\0";

// feature: VK_KHR_external_fence_win32

// VK_KHR_external_fence_win32
////////////////////////////////
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_fence_win32")]
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub const VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &str = "VK_KHR_external_fence_win32\0";

// feature: VK_KHR_external_fence_fd

// VK_KHR_external_fence_fd
/////////////////////////////
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub const VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub const VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &str = "VK_KHR_external_fence_fd\0";

// feature: VK_KHR_maintenance2

// VK_KHR_maintenance2
////////////////////////
#[cfg(feature = "VK_KHR_maintenance2")]
pub const VK_KHR_MAINTENANCE2_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_maintenance2")]
pub const VK_KHR_MAINTENANCE2_EXTENSION_NAME: &str = "VK_KHR_maintenance2\0";

// feature: VK_KHR_get_surface_capabilities2

// VK_KHR_get_surface_capabilities2
/////////////////////////////////////
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &str = "VK_KHR_get_surface_capabilities2\0";

// feature: VK_KHR_variable_pointers

// VK_KHR_variable_pointers
/////////////////////////////
#[cfg(feature = "VK_KHR_variable_pointers")]
pub const VK_KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_variable_pointers")]
pub const VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME: &str = "VK_KHR_variable_pointers\0";

// feature: VK_MVK_ios_surface

// VK_MVK_ios_surface
///////////////////////
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub const VK_MVK_IOS_SURFACE_SPEC_VERSION: u32 = 2;
#[cfg(feature = "VK_MVK_ios_surface")]
#[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME: &str = "VK_MVK_ios_surface\0";

// feature: VK_MVK_macos_surface

// VK_MVK_macos_surface
/////////////////////////
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 2;
#[cfg(feature = "VK_MVK_macos_surface")]
#[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &str = "VK_MVK_macos_surface\0";

// feature: VK_EXT_external_memory_dma_buf

// VK_EXT_external_memory_dma_buf
///////////////////////////////////
#[cfg(feature = "VK_EXT_external_memory_dma_buf")]
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_external_memory_dma_buf")]
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME: &str = "VK_EXT_external_memory_dma_buf\0";

// feature: VK_EXT_queue_family_foreign

// VK_EXT_queue_family_foreign
////////////////////////////////
#[cfg(feature = "VK_EXT_queue_family_foreign")]
pub const VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_queue_family_foreign")]
pub const VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME: &str = "VK_EXT_queue_family_foreign\0";

// feature: VK_KHR_dedicated_allocation

// VK_KHR_dedicated_allocation
////////////////////////////////
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub const VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub const VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &str = "VK_KHR_dedicated_allocation\0";

// feature: VK_EXT_sampler_filter_minmax

// VK_EXT_sampler_filter_minmax
/////////////////////////////////
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub const VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub const VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &str = "VK_EXT_sampler_filter_minmax\0";

// feature: VK_KHR_storage_buffer_storage_class

// VK_KHR_storage_buffer_storage_class
////////////////////////////////////////
#[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
pub const VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
pub const VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME: &str = "VK_KHR_storage_buffer_storage_class\0";

// feature: VK_AMD_gpu_shader_int16

// VK_AMD_gpu_shader_int16
////////////////////////////
#[cfg(feature = "VK_AMD_gpu_shader_int16")]
pub const VK_AMD_GPU_SHADER_INT16_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_gpu_shader_int16")]
pub const VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME: &str = "VK_AMD_gpu_shader_int16\0";

// feature: VK_AMD_mixed_attachment_samples

// VK_AMD_mixed_attachment_samples
////////////////////////////////////
#[cfg(feature = "VK_AMD_mixed_attachment_samples")]
pub const VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_mixed_attachment_samples")]
pub const VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME: &str = "VK_AMD_mixed_attachment_samples\0";

// feature: VK_AMD_shader_fragment_mask

// VK_AMD_shader_fragment_mask
////////////////////////////////
#[cfg(feature = "VK_AMD_shader_fragment_mask")]
pub const VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_shader_fragment_mask")]
pub const VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME: &str = "VK_AMD_shader_fragment_mask\0";

// feature: VK_EXT_shader_stencil_export

// VK_EXT_shader_stencil_export
/////////////////////////////////
#[cfg(feature = "VK_EXT_shader_stencil_export")]
pub const VK_EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_shader_stencil_export")]
pub const VK_EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME: &str = "VK_EXT_shader_stencil_export\0";

// feature: VK_EXT_sample_locations

// VK_EXT_sample_locations
////////////////////////////
#[cfg(feature = "VK_EXT_sample_locations")]
pub const VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_sample_locations")]
pub const VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &str = "VK_EXT_sample_locations\0";

// feature: VK_KHR_relaxed_block_layout

// VK_KHR_relaxed_block_layout
////////////////////////////////
#[cfg(feature = "VK_KHR_relaxed_block_layout")]
pub const VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_relaxed_block_layout")]
pub const VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME: &str = "VK_KHR_relaxed_block_layout\0";

// feature: VK_KHR_get_memory_requirements2

// VK_KHR_get_memory_requirements2
////////////////////////////////////
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: &str = "VK_KHR_get_memory_requirements2\0";

// feature: VK_KHR_image_format_list

// VK_KHR_image_format_list
/////////////////////////////
#[cfg(feature = "VK_KHR_image_format_list")]
pub const VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_image_format_list")]
pub const VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &str = "VK_KHR_image_format_list\0";

// feature: VK_EXT_blend_operation_advanced

// VK_EXT_blend_operation_advanced
////////////////////////////////////
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub const VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub const VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &str = "VK_EXT_blend_operation_advanced\0";

// feature: VK_NV_fragment_coverage_to_color

// VK_NV_fragment_coverage_to_color
/////////////////////////////////////
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &str = "VK_NV_fragment_coverage_to_color\0";

// feature: VK_NV_framebuffer_mixed_samples

// VK_NV_framebuffer_mixed_samples
////////////////////////////////////
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &str = "VK_NV_framebuffer_mixed_samples\0";

// feature: VK_NV_fill_rectangle

// VK_NV_fill_rectangle
/////////////////////////
#[cfg(feature = "VK_NV_fill_rectangle")]
pub const VK_NV_FILL_RECTANGLE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_NV_fill_rectangle")]
pub const VK_NV_FILL_RECTANGLE_EXTENSION_NAME: &str = "VK_NV_fill_rectangle\0";

// feature: VK_EXT_post_depth_coverage

// VK_EXT_post_depth_coverage
///////////////////////////////
#[cfg(feature = "VK_EXT_post_depth_coverage")]
pub const VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_post_depth_coverage")]
pub const VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME: &str = "VK_EXT_post_depth_coverage\0";

// feature: VK_KHR_sampler_ycbcr_conversion

// VK_KHR_sampler_ycbcr_conversion
////////////////////////////////////
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &str = "VK_KHR_sampler_ycbcr_conversion\0";

// feature: VK_KHR_bind_memory2

// VK_KHR_bind_memory2
////////////////////////
#[cfg(feature = "VK_KHR_bind_memory2")]
pub const VK_KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_KHR_bind_memory2")]
pub const VK_KHR_BIND_MEMORY_2_EXTENSION_NAME: &str = "VK_KHR_bind_memory2\0";

// feature: VK_EXT_validation_cache

// VK_EXT_validation_cache
////////////////////////////
#[cfg(feature = "VK_EXT_validation_cache")]
pub const VK_EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_validation_cache")]
pub const VK_EXT_VALIDATION_CACHE_EXTENSION_NAME: &str = "VK_EXT_validation_cache\0";
#[cfg(feature = "VK_EXT_validation_cache")]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT::E_VALIDATION_CACHE_EXT_EXT;

// feature: VK_EXT_shader_viewport_index_layer

// VK_EXT_shader_viewport_index_layer
///////////////////////////////////////
#[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
pub const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
pub const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME: &str = "VK_EXT_shader_viewport_index_layer\0";

// feature: VK_EXT_global_priority

// VK_EXT_global_priority
///////////////////////////
#[cfg(feature = "VK_EXT_global_priority")]
pub const VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;
#[cfg(feature = "VK_EXT_global_priority")]
pub const VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME: &str = "VK_EXT_global_priority\0";

// feature: VK_EXT_external_memory_host

// VK_EXT_external_memory_host
////////////////////////////////
#[cfg(feature = "VK_EXT_external_memory_host")]
pub const VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_EXT_external_memory_host")]
pub const VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &str = "VK_EXT_external_memory_host\0";

// feature: VK_AMD_buffer_marker

// VK_AMD_buffer_marker
/////////////////////////
#[cfg(feature = "VK_AMD_buffer_marker")]
pub const VK_AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
#[cfg(feature = "VK_AMD_buffer_marker")]
pub const VK_AMD_BUFFER_MARKER_EXTENSION_NAME: &str = "VK_AMD_buffer_marker\0";
