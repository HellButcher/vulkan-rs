/* GENERATED FILE */

// feature: VK_VERSION_1_0

/// Return API version number for Vulkan 1.0
pub const VK_API_VERSION_1_0: u32 = vk_make_version!(1, 0, 0);

/// Vulkan header file version number
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
  pub enum VkImageLayout {
    UNDEFINED = 0,
    GENERAL = 1,
    COLOR_ATTACHMENT_OPTIMAL = 2,
    DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    SHADER_READ_ONLY_OPTIMAL = 5,
    TRANSFER_SRC_OPTIMAL = 6,
    TRANSFER_DST_OPTIMAL = 7,
    PREINITIALIZED = 8,

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    PRESENT_SRC_KHR = 1000001002,

    // feature: VK_KHR_shared_presentable_image
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    SHARED_PRESENT_KHR = 1000111000,

    // feature: VK_KHR_maintenance2
    #[cfg(feature = "VK_KHR_maintenance2")]
    DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR = 1000117000,
    #[cfg(feature = "VK_KHR_maintenance2")]
    DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR = 1000117001
  }
}

define_enum! {

  /// Specify how contents of an attachment are treated at the beginning of a subpass
  pub enum VkAttachmentLoadOp {
    LOAD = 0,
    CLEAR = 1,
    DONT_CARE = 2
  }
}

define_enum! {

  /// Specify how contents of an attachment are treated at the end of a subpass
  pub enum VkAttachmentStoreOp {
    STORE = 0,
    DONT_CARE = 1
  }
}

define_enum! {

  /// Specifies the type of an image object
  pub enum VkImageType {
    E_1D = 0,
    E_2D = 1,
    E_3D = 2
  }
}

define_enum! {

  /// Specifies the tiling arrangement of data in an image
  pub enum VkImageTiling {
    OPTIMAL = 0,
    LINEAR = 1
  }
}

define_enum! {

  /// Image view types
  pub enum VkImageViewType {
    E_1D = 0,
    E_2D = 1,
    E_3D = 2,
    CUBE = 3,
    E_1D_ARRAY = 4,
    E_2D_ARRAY = 5,
    CUBE_ARRAY = 6
  }
}

define_enum! {

  /// Enumerant specifying a command buffer level
  pub enum VkCommandBufferLevel {
    PRIMARY = 0,
    SECONDARY = 1
  }
}

define_enum! {

  /// Specify how a component is swizzled
  pub enum VkComponentSwizzle {
    IDENTITY = 0,
    ZERO = 1,
    ONE = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6
  }
}

define_enum! {

  /// Specifies the type of a descriptor in a descriptor set
  pub enum VkDescriptorType {
    SAMPLER = 0,
    COMBINED_IMAGE_SAMPLER = 1,
    SAMPLED_IMAGE = 2,
    STORAGE_IMAGE = 3,
    UNIFORM_TEXEL_BUFFER = 4,
    STORAGE_TEXEL_BUFFER = 5,
    UNIFORM_BUFFER = 6,
    STORAGE_BUFFER = 7,
    UNIFORM_BUFFER_DYNAMIC = 8,
    STORAGE_BUFFER_DYNAMIC = 9,
    INPUT_ATTACHMENT = 10
  }
}

define_enum! {

  /// Specify the type of queries managed by a query pool
  pub enum VkQueryType {
    OCCLUSION = 0,
    PIPELINE_STATISTICS = 1,
    TIMESTAMP = 2
  }
}

define_enum! {

  /// Specify border color used for texture lookups
  pub enum VkBorderColor {
    FLOAT_TRANSPARENT_BLACK = 0,
    INT_TRANSPARENT_BLACK = 1,
    FLOAT_OPAQUE_BLACK = 2,
    INT_OPAQUE_BLACK = 3,
    FLOAT_OPAQUE_WHITE = 4,
    INT_OPAQUE_WHITE = 5
  }
}

define_enum! {

  /// Specify the bind point of a pipeline object to a command buffer
  pub enum VkPipelineBindPoint {
    GRAPHICS = 0,
    COMPUTE = 1
  }
}

define_enum! {

  /// Encode pipeline cache version
  pub enum VkPipelineCacheHeaderVersion {
    ONE = 1
  }
}

define_enum! {

  /// Supported primitive topologies
  pub enum VkPrimitiveTopology {
    POINT_LIST = 0,
    LINE_LIST = 1,
    LINE_STRIP = 2,
    TRIANGLE_LIST = 3,
    TRIANGLE_STRIP = 4,
    TRIANGLE_FAN = 5,
    LINE_LIST_WITH_ADJACENCY = 6,
    LINE_STRIP_WITH_ADJACENCY = 7,
    TRIANGLE_LIST_WITH_ADJACENCY = 8,
    TRIANGLE_STRIP_WITH_ADJACENCY = 9,
    PATCH_LIST = 10
  }
}

define_enum! {

  /// Buffer and image sharing modes
  pub enum VkSharingMode {
    EXCLUSIVE = 0,
    CONCURRENT = 1
  }
}

define_enum! {

  /// Type of index buffer indices
  pub enum VkIndexType {
    UINT16 = 0,
    UINT32 = 1
  }
}

define_enum! {

  /// Specify filters used for texture lookups
  pub enum VkFilter {
    NEAREST = 0,
    LINEAR = 1,

    // feature: VK_IMG_filter_cubic
    #[cfg(feature = "VK_IMG_filter_cubic")]
    CUBIC_IMG = 1000015000
  }
}

define_enum! {

  /// Specify mipmap mode used for texture lookups
  pub enum VkSamplerMipmapMode {
    NEAREST = 0,
    LINEAR = 1
  }
}

define_enum! {

  /// Specify behavior of sampling with texture coordinates outside an image
  pub enum VkSamplerAddressMode {
    REPEAT = 0,
    MIRRORED_REPEAT = 1,
    CLAMP_TO_EDGE = 2,
    CLAMP_TO_BORDER = 3,

    // feature: VK_KHR_sampler_mirror_clamp_to_edge
    #[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
    MIRROR_CLAMP_TO_EDGE = 4
  }
}

define_enum! {

  /// Stencil comparison function
  pub enum VkCompareOp {
    NEVER = 0,
    LESS = 1,
    EQUAL = 2,
    LESS_OR_EQUAL = 3,
    GREATER = 4,
    NOT_EQUAL = 5,
    GREATER_OR_EQUAL = 6,
    ALWAYS = 7
  }
}

define_enum! {

  /// Control polygon rasterization mode
  pub enum VkPolygonMode {
    FILL = 0,
    LINE = 1,
    POINT = 2,

    // feature: VK_NV_fill_rectangle
    #[cfg(feature = "VK_NV_fill_rectangle")]
    FILL_RECTANGLE_NV = 1000153000
  }
}

define_bitmask! {

  /// Bitmask controlling triangle culling
  pub enum VkCullModeFlagBits {
    NONE = 0,
    FRONT_BIT = 1<<0,
    BACK_BIT = 1<<1,
    FRONT_AND_BACK = 0x00000003
  }
}

define_enum! {

  /// Interpret polygon front-facing orientation
  pub enum VkFrontFace {
    COUNTER_CLOCKWISE = 0,
    CLOCKWISE = 1
  }
}

define_enum! {

  /// Framebuffer blending factors
  pub enum VkBlendFactor {
    ZERO = 0,
    ONE = 1,
    SRC_COLOR = 2,
    ONE_MINUS_SRC_COLOR = 3,
    DST_COLOR = 4,
    ONE_MINUS_DST_COLOR = 5,
    SRC_ALPHA = 6,
    ONE_MINUS_SRC_ALPHA = 7,
    DST_ALPHA = 8,
    ONE_MINUS_DST_ALPHA = 9,
    CONSTANT_COLOR = 10,
    ONE_MINUS_CONSTANT_COLOR = 11,
    CONSTANT_ALPHA = 12,
    ONE_MINUS_CONSTANT_ALPHA = 13,
    SRC_ALPHA_SATURATE = 14,
    SRC1_COLOR = 15,
    ONE_MINUS_SRC1_COLOR = 16,
    SRC1_ALPHA = 17,
    ONE_MINUS_SRC1_ALPHA = 18
  }
}

define_enum! {

  /// Framebuffer blending operations
  pub enum VkBlendOp {
    ADD = 0,
    SUBTRACT = 1,
    REVERSE_SUBTRACT = 2,
    MIN = 3,
    MAX = 4,

    // feature: VK_EXT_blend_operation_advanced
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    ZERO_EXT = 1000148000,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    SRC_EXT = 1000148001,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    DST_EXT = 1000148002,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    SRC_OVER_EXT = 1000148003,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    DST_OVER_EXT = 1000148004,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    SRC_IN_EXT = 1000148005,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    DST_IN_EXT = 1000148006,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    SRC_OUT_EXT = 1000148007,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    DST_OUT_EXT = 1000148008,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    SRC_ATOP_EXT = 1000148009,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    DST_ATOP_EXT = 1000148010,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    XOR_EXT = 1000148011,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    MULTIPLY_EXT = 1000148012,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    SCREEN_EXT = 1000148013,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    OVERLAY_EXT = 1000148014,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    DARKEN_EXT = 1000148015,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    LIGHTEN_EXT = 1000148016,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    COLORDODGE_EXT = 1000148017,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    COLORBURN_EXT = 1000148018,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    HARDLIGHT_EXT = 1000148019,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    SOFTLIGHT_EXT = 1000148020,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    DIFFERENCE_EXT = 1000148021,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    EXCLUSION_EXT = 1000148022,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    INVERT_EXT = 1000148023,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    INVERT_RGB_EXT = 1000148024,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    LINEARDODGE_EXT = 1000148025,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    LINEARBURN_EXT = 1000148026,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    VIVIDLIGHT_EXT = 1000148027,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    LINEARLIGHT_EXT = 1000148028,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    PINLIGHT_EXT = 1000148029,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    HARDMIX_EXT = 1000148030,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    HSL_HUE_EXT = 1000148031,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    HSL_SATURATION_EXT = 1000148032,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    HSL_COLOR_EXT = 1000148033,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    HSL_LUMINOSITY_EXT = 1000148034,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    PLUS_EXT = 1000148035,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    PLUS_CLAMPED_EXT = 1000148036,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    PLUS_CLAMPED_ALPHA_EXT = 1000148037,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    PLUS_DARKER_EXT = 1000148038,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    MINUS_EXT = 1000148039,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    MINUS_CLAMPED_EXT = 1000148040,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    CONTRAST_EXT = 1000148041,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    INVERT_OVG_EXT = 1000148042,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    RED_EXT = 1000148043,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    GREEN_EXT = 1000148044,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    BLUE_EXT = 1000148045
  }
}

define_enum! {

  /// Stencil comparison function
  pub enum VkStencilOp {
    KEEP = 0,
    ZERO = 1,
    REPLACE = 2,
    INCREMENT_AND_CLAMP = 3,
    DECREMENT_AND_CLAMP = 4,
    INVERT = 5,
    INCREMENT_AND_WRAP = 6,
    DECREMENT_AND_WRAP = 7
  }
}

define_enum! {

  /// Framebuffer logical operations
  pub enum VkLogicOp {
    CLEAR = 0,
    AND = 1,
    AND_REVERSE = 2,
    COPY = 3,
    AND_INVERTED = 4,
    NO_OP = 5,
    XOR = 6,
    OR = 7,
    NOR = 8,
    EQUIVALENT = 9,
    INVERT = 10,
    OR_REVERSE = 11,
    COPY_INVERTED = 12,
    OR_INVERTED = 13,
    NAND = 14,
    SET = 15
  }
}

define_enum! {

  /// Allocation type
  pub enum VkInternalAllocationType {
    EXECUTABLE = 0
  }
}

define_enum! {

  /// Allocation scope
  pub enum VkSystemAllocationScope {
    COMMAND = 0,
    OBJECT = 1,
    CACHE = 2,
    DEVICE = 3,
    INSTANCE = 4
  }
}

define_enum! {

  /// Supported physical device types
  pub enum VkPhysicalDeviceType {
    OTHER = 0,
    INTEGRATED_GPU = 1,
    DISCRETE_GPU = 2,
    VIRTUAL_GPU = 3,
    CPU = 4
  }
}

define_enum! {

  /// Specify rate at which vertex attributes are pulled from buffers
  pub enum VkVertexInputRate {
    VERTEX = 0,
    INSTANCE = 1
  }
}

define_enum! {

  /// Available image formats
  pub enum VkFormat {
    UNDEFINED = 0,
    R4G4_UNORM_PACK8 = 1,
    R4G4B4A4_UNORM_PACK16 = 2,
    B4G4R4A4_UNORM_PACK16 = 3,
    R5G6B5_UNORM_PACK16 = 4,
    B5G6R5_UNORM_PACK16 = 5,
    R5G5B5A1_UNORM_PACK16 = 6,
    B5G5R5A1_UNORM_PACK16 = 7,
    A1R5G5B5_UNORM_PACK16 = 8,
    R8_UNORM = 9,
    R8_SNORM = 10,
    R8_USCALED = 11,
    R8_SSCALED = 12,
    R8_UINT = 13,
    R8_SINT = 14,
    R8_SRGB = 15,
    R8G8_UNORM = 16,
    R8G8_SNORM = 17,
    R8G8_USCALED = 18,
    R8G8_SSCALED = 19,
    R8G8_UINT = 20,
    R8G8_SINT = 21,
    R8G8_SRGB = 22,
    R8G8B8_UNORM = 23,
    R8G8B8_SNORM = 24,
    R8G8B8_USCALED = 25,
    R8G8B8_SSCALED = 26,
    R8G8B8_UINT = 27,
    R8G8B8_SINT = 28,
    R8G8B8_SRGB = 29,
    B8G8R8_UNORM = 30,
    B8G8R8_SNORM = 31,
    B8G8R8_USCALED = 32,
    B8G8R8_SSCALED = 33,
    B8G8R8_UINT = 34,
    B8G8R8_SINT = 35,
    B8G8R8_SRGB = 36,
    R8G8B8A8_UNORM = 37,
    R8G8B8A8_SNORM = 38,
    R8G8B8A8_USCALED = 39,
    R8G8B8A8_SSCALED = 40,
    R8G8B8A8_UINT = 41,
    R8G8B8A8_SINT = 42,
    R8G8B8A8_SRGB = 43,
    B8G8R8A8_UNORM = 44,
    B8G8R8A8_SNORM = 45,
    B8G8R8A8_USCALED = 46,
    B8G8R8A8_SSCALED = 47,
    B8G8R8A8_UINT = 48,
    B8G8R8A8_SINT = 49,
    B8G8R8A8_SRGB = 50,
    A8B8G8R8_UNORM_PACK32 = 51,
    A8B8G8R8_SNORM_PACK32 = 52,
    A8B8G8R8_USCALED_PACK32 = 53,
    A8B8G8R8_SSCALED_PACK32 = 54,
    A8B8G8R8_UINT_PACK32 = 55,
    A8B8G8R8_SINT_PACK32 = 56,
    A8B8G8R8_SRGB_PACK32 = 57,
    A2R10G10B10_UNORM_PACK32 = 58,
    A2R10G10B10_SNORM_PACK32 = 59,
    A2R10G10B10_USCALED_PACK32 = 60,
    A2R10G10B10_SSCALED_PACK32 = 61,
    A2R10G10B10_UINT_PACK32 = 62,
    A2R10G10B10_SINT_PACK32 = 63,
    A2B10G10R10_UNORM_PACK32 = 64,
    A2B10G10R10_SNORM_PACK32 = 65,
    A2B10G10R10_USCALED_PACK32 = 66,
    A2B10G10R10_SSCALED_PACK32 = 67,
    A2B10G10R10_UINT_PACK32 = 68,
    A2B10G10R10_SINT_PACK32 = 69,
    R16_UNORM = 70,
    R16_SNORM = 71,
    R16_USCALED = 72,
    R16_SSCALED = 73,
    R16_UINT = 74,
    R16_SINT = 75,
    R16_SFLOAT = 76,
    R16G16_UNORM = 77,
    R16G16_SNORM = 78,
    R16G16_USCALED = 79,
    R16G16_SSCALED = 80,
    R16G16_UINT = 81,
    R16G16_SINT = 82,
    R16G16_SFLOAT = 83,
    R16G16B16_UNORM = 84,
    R16G16B16_SNORM = 85,
    R16G16B16_USCALED = 86,
    R16G16B16_SSCALED = 87,
    R16G16B16_UINT = 88,
    R16G16B16_SINT = 89,
    R16G16B16_SFLOAT = 90,
    R16G16B16A16_UNORM = 91,
    R16G16B16A16_SNORM = 92,
    R16G16B16A16_USCALED = 93,
    R16G16B16A16_SSCALED = 94,
    R16G16B16A16_UINT = 95,
    R16G16B16A16_SINT = 96,
    R16G16B16A16_SFLOAT = 97,
    R32_UINT = 98,
    R32_SINT = 99,
    R32_SFLOAT = 100,
    R32G32_UINT = 101,
    R32G32_SINT = 102,
    R32G32_SFLOAT = 103,
    R32G32B32_UINT = 104,
    R32G32B32_SINT = 105,
    R32G32B32_SFLOAT = 106,
    R32G32B32A32_UINT = 107,
    R32G32B32A32_SINT = 108,
    R32G32B32A32_SFLOAT = 109,
    R64_UINT = 110,
    R64_SINT = 111,
    R64_SFLOAT = 112,
    R64G64_UINT = 113,
    R64G64_SINT = 114,
    R64G64_SFLOAT = 115,
    R64G64B64_UINT = 116,
    R64G64B64_SINT = 117,
    R64G64B64_SFLOAT = 118,
    R64G64B64A64_UINT = 119,
    R64G64B64A64_SINT = 120,
    R64G64B64A64_SFLOAT = 121,
    B10G11R11_UFLOAT_PACK32 = 122,
    E5B9G9R9_UFLOAT_PACK32 = 123,
    D16_UNORM = 124,
    X8_D24_UNORM_PACK32 = 125,
    D32_SFLOAT = 126,
    S8_UINT = 127,
    D16_UNORM_S8_UINT = 128,
    D24_UNORM_S8_UINT = 129,
    D32_SFLOAT_S8_UINT = 130,
    BC1_RGB_UNORM_BLOCK = 131,
    BC1_RGB_SRGB_BLOCK = 132,
    BC1_RGBA_UNORM_BLOCK = 133,
    BC1_RGBA_SRGB_BLOCK = 134,
    BC2_UNORM_BLOCK = 135,
    BC2_SRGB_BLOCK = 136,
    BC3_UNORM_BLOCK = 137,
    BC3_SRGB_BLOCK = 138,
    BC4_UNORM_BLOCK = 139,
    BC4_SNORM_BLOCK = 140,
    BC5_UNORM_BLOCK = 141,
    BC5_SNORM_BLOCK = 142,
    BC6H_UFLOAT_BLOCK = 143,
    BC6H_SFLOAT_BLOCK = 144,
    BC7_UNORM_BLOCK = 145,
    BC7_SRGB_BLOCK = 146,
    ETC2_R8G8B8_UNORM_BLOCK = 147,
    ETC2_R8G8B8_SRGB_BLOCK = 148,
    ETC2_R8G8B8A1_UNORM_BLOCK = 149,
    ETC2_R8G8B8A1_SRGB_BLOCK = 150,
    ETC2_R8G8B8A8_UNORM_BLOCK = 151,
    ETC2_R8G8B8A8_SRGB_BLOCK = 152,
    EAC_R11_UNORM_BLOCK = 153,
    EAC_R11_SNORM_BLOCK = 154,
    EAC_R11G11_UNORM_BLOCK = 155,
    EAC_R11G11_SNORM_BLOCK = 156,
    ASTC_4x4_UNORM_BLOCK = 157,
    ASTC_4x4_SRGB_BLOCK = 158,
    ASTC_5x4_UNORM_BLOCK = 159,
    ASTC_5x4_SRGB_BLOCK = 160,
    ASTC_5x5_UNORM_BLOCK = 161,
    ASTC_5x5_SRGB_BLOCK = 162,
    ASTC_6x5_UNORM_BLOCK = 163,
    ASTC_6x5_SRGB_BLOCK = 164,
    ASTC_6x6_UNORM_BLOCK = 165,
    ASTC_6x6_SRGB_BLOCK = 166,
    ASTC_8x5_UNORM_BLOCK = 167,
    ASTC_8x5_SRGB_BLOCK = 168,
    ASTC_8x6_UNORM_BLOCK = 169,
    ASTC_8x6_SRGB_BLOCK = 170,
    ASTC_8x8_UNORM_BLOCK = 171,
    ASTC_8x8_SRGB_BLOCK = 172,
    ASTC_10x5_UNORM_BLOCK = 173,
    ASTC_10x5_SRGB_BLOCK = 174,
    ASTC_10x6_UNORM_BLOCK = 175,
    ASTC_10x6_SRGB_BLOCK = 176,
    ASTC_10x8_UNORM_BLOCK = 177,
    ASTC_10x8_SRGB_BLOCK = 178,
    ASTC_10x10_UNORM_BLOCK = 179,
    ASTC_10x10_SRGB_BLOCK = 180,
    ASTC_12x10_UNORM_BLOCK = 181,
    ASTC_12x10_SRGB_BLOCK = 182,
    ASTC_12x12_UNORM_BLOCK = 183,
    ASTC_12x12_SRGB_BLOCK = 184,

    // feature: VK_IMG_format_pvrtc
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    PVRTC1_2BPP_UNORM_BLOCK_IMG = 1000054000,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    PVRTC1_4BPP_UNORM_BLOCK_IMG = 1000054001,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    PVRTC2_2BPP_UNORM_BLOCK_IMG = 1000054002,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    PVRTC2_4BPP_UNORM_BLOCK_IMG = 1000054003,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    PVRTC1_2BPP_SRGB_BLOCK_IMG = 1000054004,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    PVRTC1_4BPP_SRGB_BLOCK_IMG = 1000054005,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    PVRTC2_2BPP_SRGB_BLOCK_IMG = 1000054006,
    #[cfg(feature = "VK_IMG_format_pvrtc")]
    PVRTC2_4BPP_SRGB_BLOCK_IMG = 1000054007,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G8B8G8R8_422_UNORM_KHR = 1000156000,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    B8G8R8G8_422_UNORM_KHR = 1000156001,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G8_B8_R8_3PLANE_420_UNORM_KHR = 1000156002,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G8_B8R8_2PLANE_420_UNORM_KHR = 1000156003,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G8_B8_R8_3PLANE_422_UNORM_KHR = 1000156004,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G8_B8R8_2PLANE_422_UNORM_KHR = 1000156005,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G8_B8_R8_3PLANE_444_UNORM_KHR = 1000156006,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    R10X6_UNORM_PACK16_KHR = 1000156007,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    R10X6G10X6_UNORM_2PACK16_KHR = 1000156008,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR = 1000156009,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR = 1000156010,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR = 1000156011,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR = 1000156012,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR = 1000156013,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR = 1000156014,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR = 1000156015,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR = 1000156016,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    R12X4_UNORM_PACK16_KHR = 1000156017,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    R12X4G12X4_UNORM_2PACK16_KHR = 1000156018,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR = 1000156019,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR = 1000156020,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR = 1000156021,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR = 1000156022,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR = 1000156023,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR = 1000156024,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR = 1000156025,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR = 1000156026,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G16B16G16R16_422_UNORM_KHR = 1000156027,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    B16G16R16G16_422_UNORM_KHR = 1000156028,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G16_B16_R16_3PLANE_420_UNORM_KHR = 1000156029,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G16_B16R16_2PLANE_420_UNORM_KHR = 1000156030,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G16_B16_R16_3PLANE_422_UNORM_KHR = 1000156031,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G16_B16R16_2PLANE_422_UNORM_KHR = 1000156032,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    G16_B16_R16_3PLANE_444_UNORM_KHR = 1000156033
  }
}

define_enum! {

  /// Vulkan structure types (`stype`)
  pub enum VkStructureType {
    APPLICATION_INFO = 0,
    INSTANCE_CREATE_INFO = 1,
    DEVICE_QUEUE_CREATE_INFO = 2,
    DEVICE_CREATE_INFO = 3,
    SUBMIT_INFO = 4,
    MEMORY_ALLOCATE_INFO = 5,
    MAPPED_MEMORY_RANGE = 6,
    BIND_SPARSE_INFO = 7,
    FENCE_CREATE_INFO = 8,
    SEMAPHORE_CREATE_INFO = 9,
    EVENT_CREATE_INFO = 10,
    QUERY_POOL_CREATE_INFO = 11,
    BUFFER_CREATE_INFO = 12,
    BUFFER_VIEW_CREATE_INFO = 13,
    IMAGE_CREATE_INFO = 14,
    IMAGE_VIEW_CREATE_INFO = 15,
    SHADER_MODULE_CREATE_INFO = 16,
    PIPELINE_CACHE_CREATE_INFO = 17,
    PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    GRAPHICS_PIPELINE_CREATE_INFO = 28,
    COMPUTE_PIPELINE_CREATE_INFO = 29,
    PIPELINE_LAYOUT_CREATE_INFO = 30,
    SAMPLER_CREATE_INFO = 31,
    DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    DESCRIPTOR_POOL_CREATE_INFO = 33,
    DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    WRITE_DESCRIPTOR_SET = 35,
    COPY_DESCRIPTOR_SET = 36,
    FRAMEBUFFER_CREATE_INFO = 37,
    RENDER_PASS_CREATE_INFO = 38,
    COMMAND_POOL_CREATE_INFO = 39,
    COMMAND_BUFFER_ALLOCATE_INFO = 40,
    COMMAND_BUFFER_INHERITANCE_INFO = 41,
    COMMAND_BUFFER_BEGIN_INFO = 42,
    RENDER_PASS_BEGIN_INFO = 43,
    BUFFER_MEMORY_BARRIER = 44,
    IMAGE_MEMORY_BARRIER = 45,
    MEMORY_BARRIER = 46,
    LOADER_INSTANCE_CREATE_INFO = 47,
    LOADER_DEVICE_CREATE_INFO = 48,

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
    #[cfg(feature = "VK_KHR_swapchain")]
    PRESENT_INFO_KHR = 1000001001,

    // feature: VK_KHR_display
    #[cfg(feature = "VK_KHR_display")]
    DISPLAY_MODE_CREATE_INFO_KHR = 1000002000,
    #[cfg(feature = "VK_KHR_display")]
    DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001,

    // feature: VK_KHR_display_swapchain
    #[cfg(feature = "VK_KHR_display_swapchain")]
    DISPLAY_PRESENT_INFO_KHR = 1000003000,

    // feature: VK_KHR_xlib_surface
    #[cfg(feature = "VK_KHR_xlib_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
    XLIB_SURFACE_CREATE_INFO_KHR = 1000004000,

    // feature: VK_KHR_xcb_surface
    #[cfg(feature = "VK_KHR_xcb_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
    XCB_SURFACE_CREATE_INFO_KHR = 1000005000,

    // feature: VK_KHR_wayland_surface
    #[cfg(feature = "VK_KHR_wayland_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
    WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000,

    // feature: VK_KHR_mir_surface
    #[cfg(feature = "VK_KHR_mir_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
    MIR_SURFACE_CREATE_INFO_KHR = 1000007000,

    // feature: VK_KHR_android_surface
    #[cfg(feature = "VK_KHR_android_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
    ANDROID_SURFACE_CREATE_INFO_KHR = 1000008000,

    // feature: VK_KHR_win32_surface
    #[cfg(feature = "VK_KHR_win32_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,

    // feature: VK_EXT_debug_report
    #[cfg(feature = "VK_EXT_debug_report")]
    DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = 1000011000,

    // feature: VK_AMD_rasterization_order
    #[cfg(feature = "VK_AMD_rasterization_order")]
    PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD = 1000018000,

    // feature: VK_EXT_debug_marker
    #[cfg(feature = "VK_EXT_debug_marker")]
    DEBUG_MARKER_OBJECT_NAME_INFO_EXT = 1000022000,
    #[cfg(feature = "VK_EXT_debug_marker")]
    DEBUG_MARKER_OBJECT_TAG_INFO_EXT = 1000022001,
    #[cfg(feature = "VK_EXT_debug_marker")]
    DEBUG_MARKER_MARKER_INFO_EXT = 1000022002,

    // feature: VK_NV_dedicated_allocation
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV = 1000026000,
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV = 1000026001,
    #[cfg(feature = "VK_NV_dedicated_allocation")]
    DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV = 1000026002,

    // feature: VK_AMD_texture_gather_bias_lod
    #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
    TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD = 1000041000,

    // feature: VK_KHX_multiview
    #[cfg(feature = "VK_KHX_multiview")]
    RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX = 1000053000,
    #[cfg(feature = "VK_KHX_multiview")]
    PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX = 1000053001,
    #[cfg(feature = "VK_KHX_multiview")]
    PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX = 1000053002,

    // feature: VK_NV_external_memory
    #[cfg(feature = "VK_NV_external_memory")]
    EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV = 1000056000,
    #[cfg(feature = "VK_NV_external_memory")]
    EXPORT_MEMORY_ALLOCATE_INFO_NV = 1000056001,

    // feature: VK_NV_external_memory_win32
    #[cfg(feature = "VK_NV_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    IMPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057000,
    #[cfg(feature = "VK_NV_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    EXPORT_MEMORY_WIN32_HANDLE_INFO_NV = 1000057001,

    // feature: VK_NV_win32_keyed_mutex
    #[cfg(feature = "VK_NV_win32_keyed_mutex")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV = 1000058000,

    // feature: VK_KHR_get_physical_device_properties2
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    PHYSICAL_DEVICE_FEATURES_2_KHR = 1000059000,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    PHYSICAL_DEVICE_PROPERTIES_2_KHR = 1000059001,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    FORMAT_PROPERTIES_2_KHR = 1000059002,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059003,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR = 1000059004,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    QUEUE_FAMILY_PROPERTIES_2_KHR = 1000059005,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR = 1000059006,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR = 1000059007,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR = 1000059008,

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    MEMORY_ALLOCATE_FLAGS_INFO_KHX = 1000060000,
    #[cfg(feature = "VK_KHX_device_group")]
    DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX = 1000060003,
    #[cfg(feature = "VK_KHX_device_group")]
    DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX = 1000060004,
    #[cfg(feature = "VK_KHX_device_group")]
    DEVICE_GROUP_SUBMIT_INFO_KHX = 1000060005,
    #[cfg(feature = "VK_KHX_device_group")]
    DEVICE_GROUP_BIND_SPARSE_INFO_KHX = 1000060006,
    #[cfg(feature = "VK_KHX_device_group")]
    ACQUIRE_NEXT_IMAGE_INFO_KHX = 1000060010,
    #[cfg(feature = "VK_KHX_device_group")]
    BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHX = 1000060013,
    #[cfg(feature = "VK_KHX_device_group")]
    BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHX = 1000060014,
    #[cfg(feature = "VK_KHX_device_group")]
    DEVICE_GROUP_PRESENT_CAPABILITIES_KHX = 1000060007,
    #[cfg(feature = "VK_KHX_device_group")]
    IMAGE_SWAPCHAIN_CREATE_INFO_KHX = 1000060008,
    #[cfg(feature = "VK_KHX_device_group")]
    BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX = 1000060009,
    #[cfg(feature = "VK_KHX_device_group")]
    DEVICE_GROUP_PRESENT_INFO_KHX = 1000060011,
    #[cfg(feature = "VK_KHX_device_group")]
    DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX = 1000060012,

    // feature: VK_EXT_validation_flags
    #[cfg(feature = "VK_EXT_validation_flags")]
    VALIDATION_FLAGS_EXT = 1000061000,

    // feature: VK_NN_vi_surface
    #[cfg(feature = "VK_NN_vi_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
    VI_SURFACE_CREATE_INFO_NN = 1000062000,

    // feature: VK_KHX_device_group_creation
    #[cfg(feature = "VK_KHX_device_group_creation")]
    PHYSICAL_DEVICE_GROUP_PROPERTIES_KHX = 1000070000,
    #[cfg(feature = "VK_KHX_device_group_creation")]
    DEVICE_GROUP_DEVICE_CREATE_INFO_KHX = 1000070001,

    // feature: VK_KHR_external_memory_capabilities
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR = 1000071000,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR = 1000071001,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR = 1000071002,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    EXTERNAL_BUFFER_PROPERTIES_KHR = 1000071003,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    PHYSICAL_DEVICE_ID_PROPERTIES_KHR = 1000071004,

    // feature: VK_KHR_external_memory
    #[cfg(feature = "VK_KHR_external_memory")]
    EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR = 1000072000,
    #[cfg(feature = "VK_KHR_external_memory")]
    EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR = 1000072001,
    #[cfg(feature = "VK_KHR_external_memory")]
    EXPORT_MEMORY_ALLOCATE_INFO_KHR = 1000072002,

    // feature: VK_KHR_external_memory_win32
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR = 1000073000,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR = 1000073001,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    MEMORY_WIN32_HANDLE_PROPERTIES_KHR = 1000073002,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    MEMORY_GET_WIN32_HANDLE_INFO_KHR = 1000073003,

    // feature: VK_KHR_external_memory_fd
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    IMPORT_MEMORY_FD_INFO_KHR = 1000074000,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    MEMORY_FD_PROPERTIES_KHR = 1000074001,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    MEMORY_GET_FD_INFO_KHR = 1000074002,

    // feature: VK_KHR_win32_keyed_mutex
    #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR = 1000075000,

    // feature: VK_KHR_external_semaphore_capabilities
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR = 1000076000,
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    EXTERNAL_SEMAPHORE_PROPERTIES_KHR = 1000076001,

    // feature: VK_KHR_external_semaphore
    #[cfg(feature = "VK_KHR_external_semaphore")]
    EXPORT_SEMAPHORE_CREATE_INFO_KHR = 1000077000,

    // feature: VK_KHR_external_semaphore_win32
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR = 1000078000,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR = 1000078001,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    D3D12_FENCE_SUBMIT_INFO_KHR = 1000078002,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR = 1000078003,

    // feature: VK_KHR_external_semaphore_fd
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    IMPORT_SEMAPHORE_FD_INFO_KHR = 1000079000,
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    SEMAPHORE_GET_FD_INFO_KHR = 1000079001,

    // feature: VK_KHR_push_descriptor
    #[cfg(feature = "VK_KHR_push_descriptor")]
    PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR = 1000080000,

    // feature: VK_KHR_16bit_storage
    #[cfg(feature = "VK_KHR_16bit_storage")]
    PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR = 1000083000,

    // feature: VK_KHR_incremental_present
    #[cfg(feature = "VK_KHR_incremental_present")]
    PRESENT_REGIONS_KHR = 1000084000,

    // feature: VK_KHR_descriptor_update_template
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR = 1000085000,

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    OBJECT_TABLE_CREATE_INFO_NVX = 1000086000,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX = 1000086001,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    CMD_PROCESS_COMMANDS_INFO_NVX = 1000086002,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX = 1000086003,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    DEVICE_GENERATED_COMMANDS_LIMITS_NVX = 1000086004,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    DEVICE_GENERATED_COMMANDS_FEATURES_NVX = 1000086005,

    // feature: VK_NV_clip_space_w_scaling
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV = 1000087000,

    // feature: VK_EXT_display_surface_counter
    #[cfg(feature = "VK_EXT_display_surface_counter")]
    SURFACE_CAPABILITIES_2_EXT = 1000090000,

    // feature: VK_EXT_display_control
    #[cfg(feature = "VK_EXT_display_control")]
    DISPLAY_POWER_INFO_EXT = 1000091000,
    #[cfg(feature = "VK_EXT_display_control")]
    DEVICE_EVENT_INFO_EXT = 1000091001,
    #[cfg(feature = "VK_EXT_display_control")]
    DISPLAY_EVENT_INFO_EXT = 1000091002,
    #[cfg(feature = "VK_EXT_display_control")]
    SWAPCHAIN_COUNTER_CREATE_INFO_EXT = 1000091003,

    // feature: VK_GOOGLE_display_timing
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    PRESENT_TIMES_INFO_GOOGLE = 1000092000,

    // feature: VK_NVX_multiview_per_view_attributes
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX = 1000097000,

    // feature: VK_NV_viewport_swizzle
    #[cfg(feature = "VK_NV_viewport_swizzle")]
    PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV = 1000098000,

    // feature: VK_EXT_discard_rectangles
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT = 1000099000,
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT = 1000099001,

    // feature: VK_EXT_conservative_rasterization
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT = 1000101000,
    #[cfg(feature = "VK_EXT_conservative_rasterization")]
    PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT = 1000101001,

    // feature: VK_EXT_hdr_metadata
    #[cfg(feature = "VK_EXT_hdr_metadata")]
    HDR_METADATA_EXT = 1000105000,

    // feature: VK_KHR_shared_presentable_image
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    SHARED_PRESENT_SURFACE_CAPABILITIES_KHR = 1000111000,

    // feature: VK_KHR_external_fence_capabilities
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR = 1000112000,
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    EXTERNAL_FENCE_PROPERTIES_KHR = 1000112001,

    // feature: VK_KHR_external_fence
    #[cfg(feature = "VK_KHR_external_fence")]
    EXPORT_FENCE_CREATE_INFO_KHR = 1000113000,

    // feature: VK_KHR_external_fence_win32
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    IMPORT_FENCE_WIN32_HANDLE_INFO_KHR = 1000114000,
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    EXPORT_FENCE_WIN32_HANDLE_INFO_KHR = 1000114001,
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    FENCE_GET_WIN32_HANDLE_INFO_KHR = 1000114002,

    // feature: VK_KHR_external_fence_fd
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    IMPORT_FENCE_FD_INFO_KHR = 1000115000,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    FENCE_GET_FD_INFO_KHR = 1000115001,

    // feature: VK_KHR_maintenance2
    #[cfg(feature = "VK_KHR_maintenance2")]
    PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR = 1000117000,
    #[cfg(feature = "VK_KHR_maintenance2")]
    RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR = 1000117001,
    #[cfg(feature = "VK_KHR_maintenance2")]
    IMAGE_VIEW_USAGE_CREATE_INFO_KHR = 1000117002,
    #[cfg(feature = "VK_KHR_maintenance2")]
    PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR = 1000117003,

    // feature: VK_KHR_get_surface_capabilities2
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    PHYSICAL_DEVICE_SURFACE_INFO_2_KHR = 1000119000,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    SURFACE_CAPABILITIES_2_KHR = 1000119001,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    SURFACE_FORMAT_2_KHR = 1000119002,

    // feature: VK_KHR_variable_pointers
    #[cfg(feature = "VK_KHR_variable_pointers")]
    PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR = 1000120000,

    // feature: VK_MVK_ios_surface
    #[cfg(feature = "VK_MVK_ios_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
    IOS_SURFACE_CREATE_INFO_MVK = 1000122000,

    // feature: VK_MVK_macos_surface
    #[cfg(feature = "VK_MVK_macos_surface")]
    #[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
    MACOS_SURFACE_CREATE_INFO_MVK = 1000123000,

    // feature: VK_KHR_dedicated_allocation
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    MEMORY_DEDICATED_REQUIREMENTS_KHR = 1000127000,
    #[cfg(feature = "VK_KHR_dedicated_allocation")]
    MEMORY_DEDICATED_ALLOCATE_INFO_KHR = 1000127001,

    // feature: VK_EXT_sampler_filter_minmax
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT = 1000130000,
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT = 1000130001,

    // feature: VK_EXT_sample_locations
    #[cfg(feature = "VK_EXT_sample_locations")]
    SAMPLE_LOCATIONS_INFO_EXT = 1000143000,
    #[cfg(feature = "VK_EXT_sample_locations")]
    RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT = 1000143001,
    #[cfg(feature = "VK_EXT_sample_locations")]
    PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT = 1000143002,
    #[cfg(feature = "VK_EXT_sample_locations")]
    PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT = 1000143003,
    #[cfg(feature = "VK_EXT_sample_locations")]
    MULTISAMPLE_PROPERTIES_EXT = 1000143004,

    // feature: VK_KHR_get_memory_requirements2
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146000,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146001,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR = 1000146002,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    MEMORY_REQUIREMENTS_2_KHR = 1000146003,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR = 1000146004,

    // feature: VK_KHR_image_format_list
    #[cfg(feature = "VK_KHR_image_format_list")]
    IMAGE_FORMAT_LIST_CREATE_INFO_KHR = 1000147000,

    // feature: VK_EXT_blend_operation_advanced
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT = 1000148000,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT = 1000148001,
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT = 1000148002,

    // feature: VK_NV_fragment_coverage_to_color
    #[cfg(feature = "VK_NV_fragment_coverage_to_color")]
    PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV = 1000149000,

    // feature: VK_NV_framebuffer_mixed_samples
    #[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
    PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV = 1000152000,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR = 1000156000,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLER_YCBCR_CONVERSION_INFO_KHR = 1000156001,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    BIND_IMAGE_PLANE_MEMORY_INFO_KHR = 1000156002,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR = 1000156003,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR = 1000156004,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR = 1000156005,

    // feature: VK_KHR_bind_memory2
    #[cfg(feature = "VK_KHR_bind_memory2")]
    BIND_BUFFER_MEMORY_INFO_KHR = 1000157000,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    BIND_IMAGE_MEMORY_INFO_KHR = 1000157001,

    // feature: VK_EXT_validation_cache
    #[cfg(feature = "VK_EXT_validation_cache")]
    VALIDATION_CACHE_CREATE_INFO_EXT = 1000160000,
    #[cfg(feature = "VK_EXT_validation_cache")]
    SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT = 1000160001,

    // feature: VK_EXT_global_priority
    #[cfg(feature = "VK_EXT_global_priority")]
    DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT = 1000174000,

    // feature: VK_EXT_external_memory_host
    #[cfg(feature = "VK_EXT_external_memory_host")]
    IMPORT_MEMORY_HOST_POINTER_INFO_EXT = 1000178000,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    MEMORY_HOST_POINTER_PROPERTIES_EXT = 1000178001,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT = 1000178002
  }
}

define_enum! {

  /// Specify how commands in the first subpass of a render pass are provided
  pub enum VkSubpassContents {
    INLINE = 0,
    SECONDARY_COMMAND_BUFFERS = 1
  }
}

define_enum! {

  /// Vulkan command return codes
  pub enum VkError {
    NOT_READY = 1,
    TIMEOUT = 2,
    EVENT_SET = 3,
    EVENT_RESET = 4,
    INCOMPLETE = 5,
    ERROR_OUT_OF_HOST_MEMORY = !0,
    ERROR_OUT_OF_DEVICE_MEMORY = !1,
    ERROR_INITIALIZATION_FAILED = !2,
    ERROR_DEVICE_LOST = !3,
    ERROR_MEMORY_MAP_FAILED = !4,
    ERROR_LAYER_NOT_PRESENT = !5,
    ERROR_EXTENSION_NOT_PRESENT = !6,
    ERROR_FEATURE_NOT_PRESENT = !7,
    ERROR_INCOMPATIBLE_DRIVER = !8,
    ERROR_TOO_MANY_OBJECTS = !9,
    ERROR_FORMAT_NOT_SUPPORTED = !10,
    ERROR_FRAGMENTED_POOL = !11,

    // feature: VK_KHR_surface
    #[cfg(feature = "VK_KHR_surface")]
    ERROR_SURFACE_LOST_KHR = !999999999,
    #[cfg(feature = "VK_KHR_surface")]
    ERROR_NATIVE_WINDOW_IN_USE_KHR = !1000000000,

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    SUBOPTIMAL_KHR = 1000001003,
    #[cfg(feature = "VK_KHR_swapchain")]
    ERROR_OUT_OF_DATE_KHR = !1000001003,

    // feature: VK_KHR_display_swapchain
    #[cfg(feature = "VK_KHR_display_swapchain")]
    ERROR_INCOMPATIBLE_DISPLAY_KHR = !1000003000,

    // feature: VK_EXT_debug_report
    #[cfg(feature = "VK_EXT_debug_report")]
    ERROR_VALIDATION_FAILED_EXT = !1000011000,

    // feature: VK_NV_glsl_shader
    #[cfg(feature = "VK_NV_glsl_shader")]
    ERROR_INVALID_SHADER_NV = !1000011999,

    // feature: VK_KHR_maintenance1
    #[cfg(feature = "VK_KHR_maintenance1")]
    ERROR_OUT_OF_POOL_MEMORY_KHR = !1000068999,

    // feature: VK_KHR_external_memory
    #[cfg(feature = "VK_KHR_external_memory")]
    ERROR_INVALID_EXTERNAL_HANDLE_KHR = !1000072002,

    // feature: VK_EXT_global_priority
    #[cfg(feature = "VK_EXT_global_priority")]
    ERROR_NOT_PERMITTED_EXT = !1000174000
  }
}

impl ::std::error::Error for VkError {
  fn description(&self) -> &str {
    if *self == VkError::NOT_READY {
      return "A fence or query has not yet completed";
    }
    if *self == VkError::TIMEOUT {
      return "A wait operation has not completed in the specified time";
    }
    if *self == VkError::EVENT_SET {
      return "An event is signaled";
    }
    if *self == VkError::EVENT_RESET {
      return "An event is unsignaled";
    }
    if *self == VkError::INCOMPLETE {
      return "A return array was too small for the result";
    }
    if *self == VkError::ERROR_OUT_OF_HOST_MEMORY {
      return "A host memory allocation has failed";
    }
    if *self == VkError::ERROR_OUT_OF_DEVICE_MEMORY {
      return "A device memory allocation has failed";
    }
    if *self == VkError::ERROR_INITIALIZATION_FAILED {
      return "Initialization of a object has failed";
    }
    if *self == VkError::ERROR_DEVICE_LOST {
      return "The logical device has been lost";
    }
    if *self == VkError::ERROR_MEMORY_MAP_FAILED {
      return "Mapping of a memory object has failed";
    }
    if *self == VkError::ERROR_LAYER_NOT_PRESENT {
      return "Layer specified does not exist";
    }
    if *self == VkError::ERROR_EXTENSION_NOT_PRESENT {
      return "Extension specified does not exist";
    }
    if *self == VkError::ERROR_FEATURE_NOT_PRESENT {
      return "Requested feature is not available on this device";
    }
    if *self == VkError::ERROR_INCOMPATIBLE_DRIVER {
      return "Unable to find a Vulkan driver";
    }
    if *self == VkError::ERROR_TOO_MANY_OBJECTS {
      return "Too many objects of the type have already been created";
    }
    if *self == VkError::ERROR_FORMAT_NOT_SUPPORTED {
      return "Requested format is not supported on this device";
    }
    if *self == VkError::ERROR_FRAGMENTED_POOL {
      return "A requested pool allocation has failed due to fragmentation of the pool's memory";
    }
    #[cfg(feature = "VK_KHR_surface")]
    {
      if *self == VkError::ERROR_SURFACE_LOST_KHR {
        return "surface_lost_khr";
      }
    }
    #[cfg(feature = "VK_KHR_surface")]
    {
      if *self == VkError::ERROR_NATIVE_WINDOW_IN_USE_KHR {
        return "native_window_in_use_khr";
      }
    }
    #[cfg(feature = "VK_KHR_swapchain")]
    {
      if *self == VkError::SUBOPTIMAL_KHR {
        return "suboptimal_khr";
      }
    }
    #[cfg(feature = "VK_KHR_swapchain")]
    {
      if *self == VkError::ERROR_OUT_OF_DATE_KHR {
        return "out_of_date_khr";
      }
    }
    #[cfg(feature = "VK_KHR_display_swapchain")]
    {
      if *self == VkError::ERROR_INCOMPATIBLE_DISPLAY_KHR {
        return "incompatible_display_khr";
      }
    }
    #[cfg(feature = "VK_EXT_debug_report")]
    {
      if *self == VkError::ERROR_VALIDATION_FAILED_EXT {
        return "validation_failed_ext";
      }
    }
    #[cfg(feature = "VK_NV_glsl_shader")]
    {
      if *self == VkError::ERROR_INVALID_SHADER_NV {
        return "invalid_shader_nv";
      }
    }
    #[cfg(feature = "VK_KHR_maintenance1")]
    {
      if *self == VkError::ERROR_OUT_OF_POOL_MEMORY_KHR {
        return "out_of_pool_memory_khr";
      }
    }
    #[cfg(feature = "VK_KHR_external_memory")]
    {
      if *self == VkError::ERROR_INVALID_EXTERNAL_HANDLE_KHR {
        return "invalid_external_handle_khr";
      }
    }
    #[cfg(feature = "VK_EXT_global_priority")]
    {
      if *self == VkError::ERROR_NOT_PERMITTED_EXT {
        return "not_permitted_ext";
      }
    }
    "unknown"
  }
}
impl ::std::fmt::Display for VkError {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "{} ({})", ::std::error::Error::description(self), *self as i32)
  }
}

/// Vulkan command return codes
pub type VkResult<V = ()> = Result<V, VkError>;
// feature: VK_VERSION_1_0

define_enum! {

  /// Indicate which dynamic state is taken from dynamic state commands
  pub enum VkDynamicState {
    VIEWPORT = 0,
    SCISSOR = 1,
    LINE_WIDTH = 2,
    DEPTH_BIAS = 3,
    BLEND_CONSTANTS = 4,
    DEPTH_BOUNDS = 5,
    STENCIL_COMPARE_MASK = 6,
    STENCIL_WRITE_MASK = 7,
    STENCIL_REFERENCE = 8,

    // feature: VK_NV_clip_space_w_scaling
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    VIEWPORT_W_SCALING_NV = 1000087000,

    // feature: VK_EXT_discard_rectangles
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    DISCARD_RECTANGLE_EXT = 1000099000,

    // feature: VK_EXT_sample_locations
    #[cfg(feature = "VK_EXT_sample_locations")]
    SAMPLE_LOCATIONS_EXT = 1000143000
  }
}

// feature: VK_KHR_descriptor_update_template

#[cfg(feature = "VK_KHR_descriptor_update_template")]
define_enum! {

  /// Indicates the valid usage of the descriptor update template
  pub enum VkDescriptorUpdateTemplateTypeKHR {
    DESCRIPTOR_SET_KHR = 0,
    PUSH_DESCRIPTORS_KHR = 1
  }
}

// feature: VK_VERSION_1_0

define_enum! {

  /// Specify an enumeration to track object handle types
  pub enum VkObjectType {
    UNKNOWN = 0,
    INSTANCE = 1,
    PHYSICAL_DEVICE = 2,
    DEVICE = 3,
    QUEUE = 4,
    SEMAPHORE = 5,
    COMMAND_BUFFER = 6,
    FENCE = 7,
    DEVICE_MEMORY = 8,
    BUFFER = 9,
    IMAGE = 10,
    EVENT = 11,
    QUERY_POOL = 12,
    BUFFER_VIEW = 13,
    IMAGE_VIEW = 14,
    SHADER_MODULE = 15,
    PIPELINE_CACHE = 16,
    PIPELINE_LAYOUT = 17,
    RENDER_PASS = 18,
    PIPELINE = 19,
    DESCRIPTOR_SET_LAYOUT = 20,
    SAMPLER = 21,
    DESCRIPTOR_POOL = 22,
    DESCRIPTOR_SET = 23,
    FRAMEBUFFER = 24,
    COMMAND_POOL = 25,

    // feature: VK_KHR_surface
    #[cfg(feature = "VK_KHR_surface")]
    SURFACE_KHR = 1000000000,

    // feature: VK_KHR_swapchain
    #[cfg(feature = "VK_KHR_swapchain")]
    SWAPCHAIN_KHR = 1000001000,

    // feature: VK_KHR_display
    #[cfg(feature = "VK_KHR_display")]
    DISPLAY_KHR = 1000002000,
    #[cfg(feature = "VK_KHR_display")]
    DISPLAY_MODE_KHR = 1000002001,

    // feature: VK_EXT_debug_report
    #[cfg(feature = "VK_EXT_debug_report")]
    DEBUG_REPORT_CALLBACK_EXT = 1000011000,

    // feature: VK_KHR_descriptor_update_template
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    DESCRIPTOR_UPDATE_TEMPLATE_KHR = 1000085000,

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    OBJECT_TABLE_NVX = 1000086000,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    INDIRECT_COMMANDS_LAYOUT_NVX = 1000086001,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLER_YCBCR_CONVERSION_KHR = 1000156000,

    // feature: VK_EXT_validation_cache
    #[cfg(feature = "VK_EXT_validation_cache")]
    VALIDATION_CACHE_EXT = 1000160000
  }
}

define_bitmask! {

  /// Bitmask specifying capabilities of queues in a queue family
  pub enum VkQueueFlagBits {
    GRAPHICS_BIT = 1<<0,
    COMPUTE_BIT = 1<<1,
    TRANSFER_BIT = 1<<2,
    SPARSE_BINDING_BIT = 1<<3
  }
}

define_bitmask! {

  /// Bitmask specifying properties for a memory type
  pub enum VkMemoryPropertyFlagBits {
    DEVICE_LOCAL_BIT = 1<<0,
    HOST_VISIBLE_BIT = 1<<1,
    HOST_COHERENT_BIT = 1<<2,
    HOST_CACHED_BIT = 1<<3,
    LAZILY_ALLOCATED_BIT = 1<<4
  }
}

define_bitmask! {

  /// Bitmask specifying attribute flags for a heap
  pub enum VkMemoryHeapFlagBits {
    DEVICE_LOCAL_BIT = 1<<0,

    // feature: VK_KHX_device_group_creation
    #[cfg(feature = "VK_KHX_device_group_creation")]
    MULTI_INSTANCE_BIT_KHX = 1<<1
  }
}

define_bitmask! {

  /// Bitmask specifying memory access types that will participate in a memory
  /// dependency
  pub enum VkAccessFlagBits {
    INDIRECT_COMMAND_READ_BIT = 1<<0,
    INDEX_READ_BIT = 1<<1,
    VERTEX_ATTRIBUTE_READ_BIT = 1<<2,
    UNIFORM_READ_BIT = 1<<3,
    INPUT_ATTACHMENT_READ_BIT = 1<<4,
    SHADER_READ_BIT = 1<<5,
    SHADER_WRITE_BIT = 1<<6,
    COLOR_ATTACHMENT_READ_BIT = 1<<7,
    COLOR_ATTACHMENT_WRITE_BIT = 1<<8,
    DEPTH_STENCIL_ATTACHMENT_READ_BIT = 1<<9,
    DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1<<10,
    TRANSFER_READ_BIT = 1<<11,
    TRANSFER_WRITE_BIT = 1<<12,
    HOST_READ_BIT = 1<<13,
    HOST_WRITE_BIT = 1<<14,
    MEMORY_READ_BIT = 1<<15,
    MEMORY_WRITE_BIT = 1<<16,

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    COMMAND_PROCESS_READ_BIT_NVX = 1<<17,
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    COMMAND_PROCESS_WRITE_BIT_NVX = 1<<18,

    // feature: VK_EXT_blend_operation_advanced
    #[cfg(feature = "VK_EXT_blend_operation_advanced")]
    COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 1<<19
  }
}

define_bitmask! {

  /// Bitmask specifying allowed usage of a buffer
  pub enum VkBufferUsageFlagBits {
    TRANSFER_SRC_BIT = 1<<0,
    TRANSFER_DST_BIT = 1<<1,
    UNIFORM_TEXEL_BUFFER_BIT = 1<<2,
    STORAGE_TEXEL_BUFFER_BIT = 1<<3,
    UNIFORM_BUFFER_BIT = 1<<4,
    STORAGE_BUFFER_BIT = 1<<5,
    INDEX_BUFFER_BIT = 1<<6,
    VERTEX_BUFFER_BIT = 1<<7,
    INDIRECT_BUFFER_BIT = 1<<8
  }
}

define_bitmask! {

  /// Bitmask specifying additional parameters of a buffer
  pub enum VkBufferCreateFlagBits {
    SPARSE_BINDING_BIT = 1<<0,
    SPARSE_RESIDENCY_BIT = 1<<1,
    SPARSE_ALIASED_BIT = 1<<2
  }
}

define_bitmask! {

  /// Bitmask specifying a pipeline stage
  pub enum VkShaderStageFlagBits {
    VERTEX_BIT = 1<<0,
    TESSELLATION_CONTROL_BIT = 1<<1,
    TESSELLATION_EVALUATION_BIT = 1<<2,
    GEOMETRY_BIT = 1<<3,
    FRAGMENT_BIT = 1<<4,
    COMPUTE_BIT = 1<<5,
    ALL_GRAPHICS = 0x0000001F,
    ALL = 0x7FFFFFFF
  }
}

define_bitmask! {

  /// Bitmask specifying intended usage of an image
  pub enum VkImageUsageFlagBits {
    TRANSFER_SRC_BIT = 1<<0,
    TRANSFER_DST_BIT = 1<<1,
    SAMPLED_BIT = 1<<2,
    STORAGE_BIT = 1<<3,
    COLOR_ATTACHMENT_BIT = 1<<4,
    DEPTH_STENCIL_ATTACHMENT_BIT = 1<<5,
    TRANSIENT_ATTACHMENT_BIT = 1<<6,
    INPUT_ATTACHMENT_BIT = 1<<7
  }
}

define_bitmask! {

  /// Bitmask specifying additional parameters of an image
  pub enum VkImageCreateFlagBits {
    SPARSE_BINDING_BIT = 1<<0,
    SPARSE_RESIDENCY_BIT = 1<<1,
    SPARSE_ALIASED_BIT = 1<<2,
    MUTABLE_FORMAT_BIT = 1<<3,
    CUBE_COMPATIBLE_BIT = 1<<4,

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    BIND_SFR_BIT_KHX = 1<<6,

    // feature: VK_KHR_maintenance1
    #[cfg(feature = "VK_KHR_maintenance1")]
    BIT_2D_ARRAY_COMPATIBLE_BIT_KHR = 1<<5,

    // feature: VK_KHR_maintenance2
    #[cfg(feature = "VK_KHR_maintenance2")]
    BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR = 1<<7,
    #[cfg(feature = "VK_KHR_maintenance2")]
    EXTENDED_USAGE_BIT_KHR = 1<<8,

    // feature: VK_EXT_sample_locations
    #[cfg(feature = "VK_EXT_sample_locations")]
    SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 1<<12,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    DISJOINT_BIT_KHR = 1<<9,

    // feature: VK_KHR_bind_memory2
    #[cfg(feature = "VK_KHR_bind_memory2")]
    ALIAS_BIT_KHR = 1<<10
  }
}

define_bitmask! {

  /// Bitmask controlling how a pipeline is created
  pub enum VkPipelineCreateFlagBits {
    DISABLE_OPTIMIZATION_BIT = 1<<0,
    ALLOW_DERIVATIVES_BIT = 1<<1,
    DERIVATIVE_BIT = 1<<2,

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHX = 1<<3,
    #[cfg(feature = "VK_KHX_device_group")]
    DISPATCH_BASE_KHX = 1<<4
  }
}

define_bitmask! {

  /// Bitmask controlling which components are written to the framebuffer
  pub enum VkColorComponentFlagBits {
    R_BIT = 1<<0,
    G_BIT = 1<<1,
    B_BIT = 1<<2,
    A_BIT = 1<<3
  }
}

define_bitmask! {

  /// Bitmask specifying initial state and behavior of a fence
  pub enum VkFenceCreateFlagBits {
    SIGNALED_BIT = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying features supported by a buffer
  pub enum VkFormatFeatureFlagBits {
    SAMPLED_IMAGE_BIT = 1<<0,
    STORAGE_IMAGE_BIT = 1<<1,
    STORAGE_IMAGE_ATOMIC_BIT = 1<<2,
    UNIFORM_TEXEL_BUFFER_BIT = 1<<3,
    STORAGE_TEXEL_BUFFER_BIT = 1<<4,
    STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 1<<5,
    VERTEX_BUFFER_BIT = 1<<6,
    COLOR_ATTACHMENT_BIT = 1<<7,
    COLOR_ATTACHMENT_BLEND_BIT = 1<<8,
    DEPTH_STENCIL_ATTACHMENT_BIT = 1<<9,
    BLIT_SRC_BIT = 1<<10,
    BLIT_DST_BIT = 1<<11,
    SAMPLED_IMAGE_FILTER_LINEAR_BIT = 1<<12,

    // feature: VK_IMG_filter_cubic
    #[cfg(feature = "VK_IMG_filter_cubic")]
    SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 1<<13,

    // feature: VK_KHR_maintenance1
    #[cfg(feature = "VK_KHR_maintenance1")]
    TRANSFER_SRC_BIT_KHR = 1<<14,
    #[cfg(feature = "VK_KHR_maintenance1")]
    TRANSFER_DST_BIT_KHR = 1<<15,

    // feature: VK_EXT_sampler_filter_minmax
    #[cfg(feature = "VK_EXT_sampler_filter_minmax")]
    SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT = 1<<16,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    MIDPOINT_CHROMA_SAMPLES_BIT_KHR = 1<<17,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR = 1<<18,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR = 1<<19,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR = 1<<20,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR = 1<<21,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    DISJOINT_BIT_KHR = 1<<22,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    COSITED_CHROMA_SAMPLES_BIT_KHR = 1<<23
  }
}

define_bitmask! {

  /// Bitmask specifying constraints on a query
  pub enum VkQueryControlFlagBits {
    PRECISE_BIT = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying how and when query results are returned
  pub enum VkQueryResultFlagBits {
    BIT_64 = 1<<0,
    WAIT_BIT = 1<<1,
    WITH_AVAILABILITY_BIT = 1<<2,
    PARTIAL_BIT = 1<<3
  }
}

define_bitmask! {

  /// Bitmask specifying usage behavior for command buffer
  pub enum VkCommandBufferUsageFlagBits {
    ONE_TIME_SUBMIT_BIT = 1<<0,
    RENDER_PASS_CONTINUE_BIT = 1<<1,
    SIMULTANEOUS_USE_BIT = 1<<2
  }
}

define_bitmask! {

  /// Bitmask specifying queried pipeline statistics
  pub enum VkQueryPipelineStatisticFlagBits {
    INPUT_ASSEMBLY_VERTICES_BIT = 1<<0,
    INPUT_ASSEMBLY_PRIMITIVES_BIT = 1<<1,
    VERTEX_SHADER_INVOCATIONS_BIT = 1<<2,
    GEOMETRY_SHADER_INVOCATIONS_BIT = 1<<3,
    GEOMETRY_SHADER_PRIMITIVES_BIT = 1<<4,
    CLIPPING_INVOCATIONS_BIT = 1<<5,
    CLIPPING_PRIMITIVES_BIT = 1<<6,
    FRAGMENT_SHADER_INVOCATIONS_BIT = 1<<7,
    TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 1<<8,
    TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 1<<9,
    COMPUTE_SHADER_INVOCATIONS_BIT = 1<<10
  }
}

define_bitmask! {

  /// Bitmask specifying which aspects of an image are included in a view
  pub enum VkImageAspectFlagBits {
    COLOR_BIT = 1<<0,
    DEPTH_BIT = 1<<1,
    STENCIL_BIT = 1<<2,
    METADATA_BIT = 1<<3,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    PLANE_0_BIT_KHR = 1<<4,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    PLANE_1_BIT_KHR = 1<<5,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    PLANE_2_BIT_KHR = 1<<6
  }
}

define_bitmask! {

  /// Bitmask specifying additional information about a sparse image resource
  pub enum VkSparseImageFormatFlagBits {
    SINGLE_MIPTAIL_BIT = 1<<0,
    ALIGNED_MIP_SIZE_BIT = 1<<1,
    NONSTANDARD_BLOCK_SIZE_BIT = 1<<2
  }
}

define_bitmask! {

  /// Bitmask specifying usage of a sparse memory binding operation
  pub enum VkSparseMemoryBindFlagBits {
    METADATA_BIT = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying pipeline stages
  pub enum VkPipelineStageFlagBits {
    TOP_OF_PIPE_BIT = 1<<0,
    DRAW_INDIRECT_BIT = 1<<1,
    VERTEX_INPUT_BIT = 1<<2,
    VERTEX_SHADER_BIT = 1<<3,
    TESSELLATION_CONTROL_SHADER_BIT = 1<<4,
    TESSELLATION_EVALUATION_SHADER_BIT = 1<<5,
    GEOMETRY_SHADER_BIT = 1<<6,
    FRAGMENT_SHADER_BIT = 1<<7,
    EARLY_FRAGMENT_TESTS_BIT = 1<<8,
    LATE_FRAGMENT_TESTS_BIT = 1<<9,
    COLOR_ATTACHMENT_OUTPUT_BIT = 1<<10,
    COMPUTE_SHADER_BIT = 1<<11,
    TRANSFER_BIT = 1<<12,
    BOTTOM_OF_PIPE_BIT = 1<<13,
    HOST_BIT = 1<<14,
    ALL_GRAPHICS_BIT = 1<<15,
    ALL_COMMANDS_BIT = 1<<16,

    // feature: VK_NVX_device_generated_commands
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    COMMAND_PROCESS_BIT_NVX = 1<<17
  }
}

define_bitmask! {

  /// Bitmask specifying usage behavior for a command pool
  pub enum VkCommandPoolCreateFlagBits {
    TRANSIENT_BIT = 1<<0,
    RESET_COMMAND_BUFFER_BIT = 1<<1
  }
}

define_bitmask! {

  /// Bitmask controlling behavior of a command pool reset
  pub enum VkCommandPoolResetFlagBits {
    RELEASE_RESOURCES_BIT = 1<<0
  }
}

define_bitmask! {

  /// Bitmask controlling behavior of a command buffer reset
  pub enum VkCommandBufferResetFlagBits {
    RELEASE_RESOURCES_BIT = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying sample counts supported for an image used for storage
  /// operations
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
  pub enum VkAttachmentDescriptionFlagBits {
    MAY_ALIAS_BIT = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying sets of stencil state for which to update the compare mask
  pub enum VkStencilFaceFlagBits {
    FACE_FRONT_BIT = 1<<0,
    FACE_BACK_BIT = 1<<1,
    FRONT_AND_BACK = 0x00000003
  }
}

define_bitmask! {

  /// Bitmask specifying certain supported operations on a descriptor pool
  pub enum VkDescriptorPoolCreateFlagBits {
    FREE_DESCRIPTOR_SET_BIT = 1<<0
  }
}

define_bitmask! {

  /// Bitmask specifying how execution and memory dependencies are formed
  pub enum VkDependencyFlagBits {
    BY_REGION_BIT = 1<<0,

    // feature: VK_KHX_multiview
    #[cfg(feature = "VK_KHX_multiview")]
    VIEW_LOCAL_BIT_KHX = 1<<1,

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    DEVICE_GROUP_BIT_KHX = 1<<2
  }
}

// feature: VK_KHR_surface

#[cfg(feature = "VK_KHR_surface")]
define_enum! {

  /// presentation mode supported for a surface
  pub enum VkPresentModeKHR {
    IMMEDIATE_KHR = 0,
    MAILBOX_KHR = 1,
    FIFO_KHR = 2,
    FIFO_RELAXED_KHR = 3,

    // feature: VK_KHR_shared_presentable_image
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    SHARED_DEMAND_REFRESH_KHR = 1000111000,
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    SHARED_CONTINUOUS_REFRESH_KHR = 1000111001
  }
}

#[cfg(feature = "VK_KHR_surface")]
define_enum! {

  /// supported color space of the presentation engine
  pub enum VkColorSpaceKHR {
    SRGB_NONLINEAR_KHR = 0,

    // feature: VK_EXT_swapchain_colorspace
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    DISPLAY_P3_NONLINEAR_EXT = 1000104001,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    EXTENDED_SRGB_LINEAR_EXT = 1000104002,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    DCI_P3_LINEAR_EXT = 1000104003,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    DCI_P3_NONLINEAR_EXT = 1000104004,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    BT709_LINEAR_EXT = 1000104005,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    BT709_NONLINEAR_EXT = 1000104006,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    BT2020_LINEAR_EXT = 1000104007,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    HDR10_ST2084_EXT = 1000104008,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    DOLBYVISION_EXT = 1000104009,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    HDR10_HLG_EXT = 1000104010,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ADOBERGB_LINEAR_EXT = 1000104011,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ADOBERGB_NONLINEAR_EXT = 1000104012,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    PASS_THROUGH_EXT = 1000104013,
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    EXTENDED_SRGB_NONLINEAR_EXT = 1000104014
  }
}

// feature: VK_KHR_display

#[cfg(feature = "VK_KHR_display")]
define_bitmask! {

  /// Alpha blending type
  pub enum VkDisplayPlaneAlphaFlagBitsKHR {
    OPAQUE_BIT_KHR = 1<<0,
    GLOBAL_BIT_KHR = 1<<1,
    PER_PIXEL_BIT_KHR = 1<<2,
    PER_PIXEL_PREMULTIPLIED_BIT_KHR = 1<<3
  }
}

// feature: VK_KHR_surface

#[cfg(feature = "VK_KHR_surface")]
define_bitmask! {

  /// alpha compositing modes supported on a device
  pub enum VkCompositeAlphaFlagBitsKHR {
    OPAQUE_BIT_KHR = 1<<0,
    PRE_MULTIPLIED_BIT_KHR = 1<<1,
    POST_MULTIPLIED_BIT_KHR = 1<<2,
    INHERIT_BIT_KHR = 1<<3
  }
}

#[cfg(feature = "VK_KHR_surface")]
define_bitmask! {

  /// presentation transforms supported on a device
  pub enum VkSurfaceTransformFlagBitsKHR {
    IDENTITY_BIT_KHR = 1<<0,
    ROTATE_90_BIT_KHR = 1<<1,
    ROTATE_180_BIT_KHR = 1<<2,
    ROTATE_270_BIT_KHR = 1<<3,
    HORIZONTAL_MIRROR_BIT_KHR = 1<<4,
    HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 1<<5,
    HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 1<<6,
    HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 1<<7,
    INHERIT_BIT_KHR = 1<<8
  }
}

// feature: VK_EXT_debug_report

#[cfg(feature = "VK_EXT_debug_report")]
define_bitmask! {

  /// Bitmask specifying events which cause a debug report callback
  pub enum VkDebugReportFlagBitsEXT {
    INFORMATION_BIT_EXT = 1<<0,
    WARNING_BIT_EXT = 1<<1,
    PERFORMANCE_WARNING_BIT_EXT = 1<<2,
    ERROR_BIT_EXT = 1<<3,
    DEBUG_BIT_EXT = 1<<4
  }
}

#[cfg(feature = "VK_EXT_debug_report")]
define_enum! {

  /// Specify the type of an object handle
  pub enum VkDebugReportObjectTypeEXT {
    UNKNOWN_EXT = 0,
    INSTANCE_EXT = 1,
    PHYSICAL_DEVICE_EXT = 2,
    DEVICE_EXT = 3,
    QUEUE_EXT = 4,
    SEMAPHORE_EXT = 5,
    COMMAND_BUFFER_EXT = 6,
    FENCE_EXT = 7,
    DEVICE_MEMORY_EXT = 8,
    BUFFER_EXT = 9,
    IMAGE_EXT = 10,
    EVENT_EXT = 11,
    QUERY_POOL_EXT = 12,
    BUFFER_VIEW_EXT = 13,
    IMAGE_VIEW_EXT = 14,
    SHADER_MODULE_EXT = 15,
    PIPELINE_CACHE_EXT = 16,
    PIPELINE_LAYOUT_EXT = 17,
    RENDER_PASS_EXT = 18,
    PIPELINE_EXT = 19,
    DESCRIPTOR_SET_LAYOUT_EXT = 20,
    SAMPLER_EXT = 21,
    DESCRIPTOR_POOL_EXT = 22,
    DESCRIPTOR_SET_EXT = 23,
    FRAMEBUFFER_EXT = 24,
    COMMAND_POOL_EXT = 25,
    SURFACE_KHR_EXT = 26,
    SWAPCHAIN_KHR_EXT = 27,
    DEBUG_REPORT_CALLBACK_EXT_EXT = 28,
    DISPLAY_KHR_EXT = 29,
    DISPLAY_MODE_KHR_EXT = 30,
    OBJECT_TABLE_NVX_EXT = 31,
    INDIRECT_COMMANDS_LAYOUT_NVX_EXT = 32,
    VALIDATION_CACHE_EXT_EXT = 33,

    // feature: VK_KHR_descriptor_update_template
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT = 1000085000,

    // feature: VK_KHR_sampler_ycbcr_conversion
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    SAMPLER_YCBCR_CONVERSION_KHR_EXT = 1000156000
  }
}

// feature: VK_AMD_rasterization_order

#[cfg(feature = "VK_AMD_rasterization_order")]
define_enum! {

  /// Specify rasterization order for a graphics pipeline
  pub enum VkRasterizationOrderAMD {
    STRICT_AMD = 0,
    RELAXED_AMD = 1
  }
}

// feature: VK_NV_external_memory_capabilities

#[cfg(feature = "VK_NV_external_memory_capabilities")]
define_bitmask! {

  /// Bitmask specifying external memory handle types
  pub enum VkExternalMemoryHandleTypeFlagBitsNV {
    OPAQUE_WIN32_BIT_NV = 1<<0,
    OPAQUE_WIN32_KMT_BIT_NV = 1<<1,
    D3D11_IMAGE_BIT_NV = 1<<2,
    D3D11_IMAGE_KMT_BIT_NV = 1<<3
  }
}

#[cfg(feature = "VK_NV_external_memory_capabilities")]
define_bitmask! {

  /// Bitmask specifying external memory features
  pub enum VkExternalMemoryFeatureFlagBitsNV {
    DEDICATED_ONLY_BIT_NV = 1<<0,
    EXPORTABLE_BIT_NV = 1<<1,
    IMPORTABLE_BIT_NV = 1<<2
  }
}

// feature: VK_EXT_validation_flags

#[cfg(feature = "VK_EXT_validation_flags")]
define_enum! {

  /// Specify validation checks to disable
  pub enum VkValidationCheckEXT {
    ALL_EXT = 0,
    SHADERS_EXT = 1
  }
}

// feature: VK_NVX_device_generated_commands

#[cfg(feature = "VK_NVX_device_generated_commands")]
define_bitmask! {

  /// Bitmask specifying allowed usage of a indirect commands layout
  pub enum VkIndirectCommandsLayoutUsageFlagBitsNVX {
    UNORDERED_SEQUENCES_BIT_NVX = 1<<0,
    SPARSE_SEQUENCES_BIT_NVX = 1<<1,
    EMPTY_EXECUTIONS_BIT_NVX = 1<<2,
    INDEXED_SEQUENCES_BIT_NVX = 1<<3
  }
}

#[cfg(feature = "VK_NVX_device_generated_commands")]
define_bitmask! {

  /// Bitmask specifying allowed usage of an object entry
  pub enum VkObjectEntryUsageFlagBitsNVX {
    GRAPHICS_BIT_NVX = 1<<0,
    COMPUTE_BIT_NVX = 1<<1
  }
}

#[cfg(feature = "VK_NVX_device_generated_commands")]
define_enum! {

  /// Enum specifying
  pub enum VkIndirectCommandsTokenTypeNVX {
    PIPELINE_NVX = 0,
    DESCRIPTOR_SET_NVX = 1,
    INDEX_BUFFER_NVX = 2,
    VERTEX_BUFFER_NVX = 3,
    PUSH_CONSTANT_NVX = 4,
    DRAW_INDEXED_NVX = 5,
    DRAW_NVX = 6,
    DISPATCH_NVX = 7
  }
}

#[cfg(feature = "VK_NVX_device_generated_commands")]
define_enum! {

  /// Enum specifying object table entry type
  pub enum VkObjectEntryTypeNVX {
    DESCRIPTOR_SET_NVX = 0,
    PIPELINE_NVX = 1,
    INDEX_BUFFER_NVX = 2,
    VERTEX_BUFFER_NVX = 3,
    PUSH_CONSTANT_NVX = 4
  }
}

// feature: VK_VERSION_1_0

define_bitmask! {

  /// Bitmask specifying descriptor set layout properties
  pub enum VkDescriptorSetLayoutCreateFlagBits {

    // feature: VK_KHR_push_descriptor
    #[cfg(feature = "VK_KHR_push_descriptor")]
    PUSH_DESCRIPTOR_BIT_KHR = 1<<0
  }
}

// feature: VK_KHR_external_memory_capabilities

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
define_bitmask! {

  /// Bit specifying external memory handle types
  pub enum VkExternalMemoryHandleTypeFlagBitsKHR {
    OPAQUE_FD_BIT_KHR = 1<<0,
    OPAQUE_WIN32_BIT_KHR = 1<<1,
    OPAQUE_WIN32_KMT_BIT_KHR = 1<<2,
    D3D11_TEXTURE_BIT_KHR = 1<<3,
    D3D11_TEXTURE_KMT_BIT_KHR = 1<<4,
    D3D12_HEAP_BIT_KHR = 1<<5,
    D3D12_RESOURCE_BIT_KHR = 1<<6,

    // feature: VK_EXT_external_memory_dma_buf
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    DMA_BUF_BIT_EXT = 1<<9,

    // feature: VK_EXT_external_memory_host
    #[cfg(feature = "VK_EXT_external_memory_host")]
    HOST_ALLOCATION_BIT_EXT = 1<<7,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT = 1<<8
  }
}

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
define_bitmask! {

  /// Bitmask specifying features of an external memory handle type
  pub enum VkExternalMemoryFeatureFlagBitsKHR {
    DEDICATED_ONLY_BIT_KHR = 1<<0,
    EXPORTABLE_BIT_KHR = 1<<1,
    IMPORTABLE_BIT_KHR = 1<<2
  }
}

// feature: VK_KHR_external_semaphore_capabilities

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
define_bitmask! {

  /// Bitmask of valid external semaphore handle types
  pub enum VkExternalSemaphoreHandleTypeFlagBitsKHR {
    OPAQUE_FD_BIT_KHR = 1<<0,
    OPAQUE_WIN32_BIT_KHR = 1<<1,
    OPAQUE_WIN32_KMT_BIT_KHR = 1<<2,
    D3D12_FENCE_BIT_KHR = 1<<3,
    SYNC_FD_BIT_KHR = 1<<4
  }
}

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
define_bitmask! {

  /// Bitfield describing features of an external semaphore handle type
  pub enum VkExternalSemaphoreFeatureFlagBitsKHR {
    EXPORTABLE_BIT_KHR = 1<<0,
    IMPORTABLE_BIT_KHR = 1<<1
  }
}

// feature: VK_KHR_external_semaphore

#[cfg(feature = "VK_KHR_external_semaphore")]
define_bitmask! {

  /// Bitmask specifying additional parameters of semaphore payload import
  pub enum VkSemaphoreImportFlagBitsKHR {
    TEMPORARY_BIT_KHR = 1<<0
  }
}

// feature: VK_KHR_external_fence_capabilities

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
define_bitmask! {

  /// Bitmask of valid external fence handle types
  pub enum VkExternalFenceHandleTypeFlagBitsKHR {
    OPAQUE_FD_BIT_KHR = 1<<0,
    OPAQUE_WIN32_BIT_KHR = 1<<1,
    OPAQUE_WIN32_KMT_BIT_KHR = 1<<2,
    SYNC_FD_BIT_KHR = 1<<3
  }
}

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
define_bitmask! {

  /// Bitfield describing features of an external fence handle type
  pub enum VkExternalFenceFeatureFlagBitsKHR {
    EXPORTABLE_BIT_KHR = 1<<0,
    IMPORTABLE_BIT_KHR = 1<<1
  }
}

// feature: VK_KHR_external_fence

#[cfg(feature = "VK_KHR_external_fence")]
define_bitmask! {

  /// Bitmask specifying additional parameters of fence payload import
  pub enum VkFenceImportFlagBitsKHR {
    TEMPORARY_BIT_KHR = 1<<0
  }
}

// feature: VK_EXT_display_surface_counter

#[cfg(feature = "VK_EXT_display_surface_counter")]
define_bitmask! {

  /// Surface-relative counter types
  pub enum VkSurfaceCounterFlagBitsEXT {
    VBLANK_EXT = 1<<0
  }
}

// feature: VK_EXT_display_control

#[cfg(feature = "VK_EXT_display_control")]
define_enum! {

  /// Possible power states for a display
  pub enum VkDisplayPowerStateEXT {
    OFF_EXT = 0,
    SUSPEND_EXT = 1,
    ON_EXT = 2
  }
}

#[cfg(feature = "VK_EXT_display_control")]
define_enum! {

  /// Events that can occur on a device object
  pub enum VkDeviceEventTypeEXT {
    DISPLAY_HOTPLUG_EXT = 0
  }
}

#[cfg(feature = "VK_EXT_display_control")]
define_enum! {

  /// Events that can occur on a display object
  pub enum VkDisplayEventTypeEXT {
    FIRST_PIXEL_OUT_EXT = 0
  }
}

// feature: VK_KHX_device_group

#[cfg(feature = "VK_KHX_device_group")]
define_bitmask! {

  /// Bitmask specifying supported peer memory features
  pub enum VkPeerMemoryFeatureFlagBitsKHX {
    COPY_SRC_BIT_KHX = 1<<0,
    COPY_DST_BIT_KHX = 1<<1,
    GENERIC_SRC_BIT_KHX = 1<<2,
    GENERIC_DST_BIT_KHX = 1<<3
  }
}

#[cfg(feature = "VK_KHX_device_group")]
define_bitmask! {

  /// Bitmask specifying flags for a device memory allocation
  pub enum VkMemoryAllocateFlagBitsKHX {
    DEVICE_MASK_BIT_KHX = 1<<0
  }
}

#[cfg(feature = "VK_KHX_device_group")]
define_bitmask! {

  /// Bitmask specifying supported device group present modes
  pub enum VkDeviceGroupPresentModeFlagBitsKHX {
    LOCAL_BIT_KHX = 1<<0,
    REMOTE_BIT_KHX = 1<<1,
    SUM_BIT_KHX = 1<<2,
    LOCAL_MULTI_DEVICE_BIT_KHX = 1<<3
  }
}

// feature: VK_KHR_swapchain

#[cfg(feature = "VK_KHR_swapchain")]
define_bitmask! {

  /// Bitmask controlling swapchain creation
  pub enum VkSwapchainCreateFlagBitsKHR {

    // feature: VK_KHX_device_group
    #[cfg(feature = "VK_KHX_device_group")]
    BIND_SFR_BIT_KHX = 1<<0
  }
}

// feature: VK_NV_viewport_swizzle

#[cfg(feature = "VK_NV_viewport_swizzle")]
define_enum! {

  /// Specify how a viewport coordinate is swizzled
  pub enum VkViewportCoordinateSwizzleNV {
    POSITIVE_X_NV = 0,
    NEGATIVE_X_NV = 1,
    POSITIVE_Y_NV = 2,
    NEGATIVE_Y_NV = 3,
    POSITIVE_Z_NV = 4,
    NEGATIVE_Z_NV = 5,
    POSITIVE_W_NV = 6,
    NEGATIVE_W_NV = 7
  }
}

// feature: VK_EXT_discard_rectangles

#[cfg(feature = "VK_EXT_discard_rectangles")]
define_enum! {

  /// Specify the discard rectangle mode
  pub enum VkDiscardRectangleModeEXT {
    INCLUSIVE_EXT = 0,
    EXCLUSIVE_EXT = 1
  }
}

// feature: VK_VERSION_1_0

define_bitmask! {

  /// Bitmask specifying usage of a subpass
  pub enum VkSubpassDescriptionFlagBits {

    // feature: VK_NVX_multiview_per_view_attributes
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    PER_VIEW_ATTRIBUTES_BIT_NVX = 1<<0,
    #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
    PER_VIEW_POSITION_X_ONLY_BIT_NVX = 1<<1
  }
}

// feature: VK_KHR_maintenance2

#[cfg(feature = "VK_KHR_maintenance2")]
define_enum! {

  /// Enum specifying the point clipping behaviour
  pub enum VkPointClippingBehaviorKHR {
    ALL_CLIP_PLANES_KHR = 0,
    USER_CLIP_PLANES_ONLY_KHR = 1
  }
}

// feature: VK_EXT_sampler_filter_minmax

#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
define_enum! {

  /// Specify reduction mode for texture filtering
  pub enum VkSamplerReductionModeEXT {
    WEIGHTED_AVERAGE_EXT = 0,
    MIN_EXT = 1,
    MAX_EXT = 2
  }
}

// feature: VK_KHR_maintenance2

#[cfg(feature = "VK_KHR_maintenance2")]
define_enum! {

  /// Enum describing tessellation domain origin
  pub enum VkTessellationDomainOriginKHR {
    UPPER_LEFT_KHR = 0,
    LOWER_LEFT_KHR = 1
  }
}

// feature: VK_KHR_sampler_ycbcr_conversion

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
define_enum! {

  /// Color model component of a color space
  pub enum VkSamplerYcbcrModelConversionKHR {
    RGB_IDENTITY_KHR = 0,
    YCBCR_IDENTITY_KHR = 1,
    YCBCR_709_KHR = 2,
    YCBCR_601_KHR = 3,
    YCBCR_2020_KHR = 4
  }
}

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
define_enum! {

  /// Range of encoded values in a color space
  pub enum VkSamplerYcbcrRangeKHR {
    ITU_FULL_KHR = 0,
    ITU_NARROW_KHR = 1
  }
}

#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
define_enum! {

  /// Position of downsampled chroma samples
  pub enum VkChromaLocationKHR {
    COSITED_EVEN_KHR = 0,
    MIDPOINT_KHR = 1
  }
}

// feature: VK_EXT_blend_operation_advanced

#[cfg(feature = "VK_EXT_blend_operation_advanced")]
define_enum! {

  /// Enumerant specifying the blend overlap parameter
  pub enum VkBlendOverlapEXT {
    UNCORRELATED_EXT = 0,
    DISJOINT_EXT = 1,
    CONJOINT_EXT = 2
  }
}

// feature: VK_NV_framebuffer_mixed_samples

#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
define_enum! {

  /// Specify the discard rectangle mode
  pub enum VkCoverageModulationModeNV {
    NONE_NV = 0,
    RGB_NV = 1,
    ALPHA_NV = 2,
    RGBA_NV = 3
  }
}

// feature: VK_EXT_validation_cache

#[cfg(feature = "VK_EXT_validation_cache")]
define_enum! {

  /// Encode validation cache version
  pub enum VkValidationCacheHeaderVersionEXT {
    ONE_EXT = 1
  }
}

// feature: VK_AMD_shader_info

#[cfg(feature = "VK_AMD_shader_info")]
define_enum! {
  pub enum VkShaderInfoTypeAMD {
    STATISTICS_AMD = 0,
    BINARY_AMD = 1,
    DISASSEMBLY_AMD = 2
  }
}

// feature: VK_EXT_global_priority

#[cfg(feature = "VK_EXT_global_priority")]
define_enum! {

  /// Values specifying a system-wide queue priority
  pub enum VkQueueGlobalPriorityEXT {
    LOW_EXT = 128,
    MEDIUM_EXT = 256,
    HIGH_EXT = 512,
    REALTIME_EXT = 1024
  }
}

// feature: VK_EXT_conservative_rasterization

#[cfg(feature = "VK_EXT_conservative_rasterization")]
define_enum! {

  /// Specify the conservative rasterization mode
  pub enum VkConservativeRasterizationModeEXT {
    DISABLED_EXT = 0,
    OVERESTIMATE_EXT = 1,
    UNDERESTIMATE_EXT = 2
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
pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VkColorSpaceKHR::SRGB_NONLINEAR_KHR;

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
  VkStructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
#[cfg(feature = "VK_EXT_debug_report")]
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT: VkDebugReportObjectTypeEXT =
  VkDebugReportObjectTypeEXT::DEBUG_REPORT_CALLBACK_EXT_EXT;

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
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT: VkStructureType = VkStructureType::SURFACE_CAPABILITIES_2_EXT;

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
  VkDebugReportObjectTypeEXT::VALIDATION_CACHE_EXT_EXT;

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
