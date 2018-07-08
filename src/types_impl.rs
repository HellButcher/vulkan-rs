use enums;
use types::*;

use std::fmt;
use std::os::raw::c_char;

unsafe fn ext_equals(ext: *const c_char, check_ext: &[u8]) -> bool {
  if ext.is_null() {
    return false;
  }
  let ext = ::std::slice::from_raw_parts(ext, check_ext.len() + 1);
  for i in 0..check_ext.len() {
    if ext[i] as u8 != check_ext[i] {
      return false;
    }
    if check_ext[i] == 0 {
      return true;
    }
  }
  return ext[check_ext.len()] == 0;
}

unsafe fn has_ext(extension_count: u32, pp_extension_names: *const *const c_char, check_ext: &[u8]) -> bool {
  if extension_count == 0 || pp_extension_names.is_null() {
    return false;
  }
  let extension_names = ::std::slice::from_raw_parts(pp_extension_names, extension_count as usize);
  for ext in extension_names {
    if ext.is_null() {
      return false;
    }
    if ext_equals(*ext, check_ext) {
      return true;
    }
  }
  false
}

impl<'a> VkInstanceCreateInfo<'a> {
  pub fn set_enabled_extension_names(mut self, value: &[*const c_char]) -> Self {
    self.enabledExtensionCount = value.len() as u32;
    self.ppEnabledExtensionNames = value.as_ptr();
    self
  }
  pub fn set_enabled_layer_names(mut self, value: &[*const c_char]) -> Self {
    self.enabledLayerCount = value.len() as u32;
    self.ppEnabledLayerNames = value.as_ptr();
    self
  }

  pub fn is_extension_enabled(&self, ext: &str) -> bool {
    unsafe { has_ext(self.enabledExtensionCount, self.ppEnabledExtensionNames, ext.as_bytes()) }
  }
}

impl<'a> VkDeviceCreateInfo<'a> {
  pub fn set_enabled_extension_names(mut self, value: &[*const c_char]) -> Self {
    self.enabledExtensionCount = value.len() as u32;
    self.ppEnabledExtensionNames = value.as_ptr();
    self
  }
  pub fn set_enabled_layer_names(mut self, value: &[*const c_char]) -> Self {
    self.enabledLayerCount = value.len() as u32;
    self.ppEnabledLayerNames = value.as_ptr();
    self
  }

  pub fn is_extension_enabled(&self, ext: &str) -> bool {
    unsafe { has_ext(self.enabledExtensionCount, self.ppEnabledExtensionNames, ext.as_bytes()) }
  }
}

impl VkExtent2D {
  pub const ZERO: Self = Self { width: 0, height: 0 };
  pub const MAX: Self = Self {
    width: ::std::u32::MAX,
    height: ::std::u32::MAX,
  };
  #[inline]
  pub fn of(width: u32, height: u32) -> Self {
    Self { width, height }
  }

  #[inline]
  pub fn with_depth(self, depth: u32) -> VkExtent3D {
    VkExtent3D {
      width: self.width,
      height: self.height,
      depth,
    }
  }

  #[inline]
  pub fn as_tuple(self) -> (u32, u32) {
    (self.width, self.height)
  }
}

impl<I: Into<(u32, u32)>> From<I> for VkExtent2D {
  #[inline]
  fn from(value: I) -> Self {
    let (width, height) = value.into();
    Self { width, height }
  }
}

impl fmt::Debug for VkExtent2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkExtent2D({:?},{:?})", self.width, self.height)
  }
}

impl VkExtent3D {
  pub const ZERO: Self = Self {
    width: 0,
    height: 0,
    depth: 0,
  };
  pub const MAX: Self = Self {
    width: ::std::u32::MAX,
    height: ::std::u32::MAX,
    depth: ::std::u32::MAX,
  };
  #[inline]
  pub fn of(width: u32, height: u32, depth: u32) -> Self {
    Self { width, height, depth }
  }

  #[inline]
  pub fn as_triple(self) -> (u32, u32, u32) {
    (self.width, self.height, self.depth)
  }
}

impl<I: Into<(u32, u32, u32)>> From<I> for VkExtent3D {
  #[inline]
  fn from(value: I) -> Self {
    let (width, height, depth) = value.into();
    Self { width, height, depth }
  }
}

impl fmt::Debug for VkExtent3D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkExtent3D({:?},{:?},{:?})", self.width, self.height, self.depth)
  }
}

impl VkOffset2D {
  pub const ZERO: Self = Self { x: 0, y: 0 };
  pub const MIN: Self = Self {
    x: ::std::i32::MIN,
    y: ::std::i32::MIN,
  };
  pub const MAX: Self = Self {
    x: ::std::i32::MAX,
    y: ::std::i32::MAX,
  };
  #[inline]
  pub fn of(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  #[inline]
  pub fn with_z(self, z: i32) -> VkOffset3D {
    VkOffset3D {
      x: self.x,
      y: self.y,
      z,
    }
  }

  #[inline]
  pub fn as_tuple(self) -> (i32, i32) {
    (self.x, self.y)
  }
}

impl<I: Into<(i32, i32)>> From<I> for VkOffset2D {
  #[inline]
  fn from(value: I) -> Self {
    let (x, y) = value.into();
    Self { x, y }
  }
}

impl fmt::Debug for VkOffset2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkOffset2D({:?},{:?})", self.x, self.y)
  }
}

impl VkOffset3D {
  pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
  pub const MIN: Self = Self {
    x: ::std::i32::MIN,
    y: ::std::i32::MIN,
    z: ::std::i32::MIN,
  };
  pub const MAX: Self = Self {
    x: ::std::i32::MAX,
    y: ::std::i32::MAX,
    z: ::std::i32::MAX,
  };
  #[inline]
  pub fn of(x: i32, y: i32, z: i32) -> Self {
    Self { x, y, z }
  }

  #[inline]
  pub fn as_triple(self) -> (i32, i32) {
    (self.x, self.y)
  }
}

impl<I: Into<(i32, i32, i32)>> From<I> for VkOffset3D {
  #[inline]
  fn from(value: I) -> Self {
    let (x, y, z) = value.into();
    Self { x, y, z }
  }
}

impl fmt::Debug for VkOffset3D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkOffset3D({:?},{:?},{:?})", self.x, self.y, self.z)
  }
}

impl VkRect2D {
  pub const ZERO: Self = Self {
    offset: VkOffset2D::ZERO,
    extent: VkExtent2D::ZERO,
  };
  pub const MAX: Self = Self {
    offset: VkOffset2D::MIN,
    extent: VkExtent2D::MAX,
  };
  #[inline]
  pub fn of<O, E>(offset: O, extent: E) -> Self
  where
    O: Into<VkOffset2D>,
    E: Into<VkExtent2D>,
  {
    Self {
      offset: offset.into(),
      extent: extent.into(),
    }
  }
}

impl fmt::Debug for VkRect2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkRect2D({:?},{:?})", self.offset, self.extent)
  }
}

impl VkComponentMapping {
  pub const IDENTITY: Self = Self {
    r: enums::VkComponentSwizzle::R,
    g: enums::VkComponentSwizzle::G,
    b: enums::VkComponentSwizzle::B,
    a: enums::VkComponentSwizzle::A,
  };
}

impl fmt::Debug for VkComponentMapping {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{:?},{:?},{:?},{:?}]", self.r, self.g, self.b, self.a)
  }
}

impl VkClearColorValue {
  pub const BLACK: Self = Self { uint32: [0, 0, 0, 0] };
}

impl From<(u32, u32, u32, u32)> for VkClearColorValue {
  #[inline]
  fn from(value: (u32, u32, u32, u32)) -> Self {
    Self {
      uint32: [value.0, value.1, value.2, value.3],
    }
  }
}

impl From<[u32; 4]> for VkClearColorValue {
  #[inline]
  fn from(value: [u32; 4]) -> Self {
    Self { uint32: value }
  }
}

impl From<(i32, i32, i32, i32)> for VkClearColorValue {
  #[inline]
  fn from(value: (i32, i32, i32, i32)) -> Self {
    Self {
      int32: [value.0, value.1, value.2, value.3],
    }
  }
}

impl From<[i32; 4]> for VkClearColorValue {
  #[inline]
  fn from(value: [i32; 4]) -> Self {
    Self { int32: value }
  }
}

impl From<(f32, f32, f32, f32)> for VkClearColorValue {
  #[inline]
  fn from(value: (f32, f32, f32, f32)) -> Self {
    Self {
      float32: [value.0, value.1, value.2, value.3],
    }
  }
}

impl From<[f32; 4]> for VkClearColorValue {
  #[inline]
  fn from(value: [f32; 4]) -> Self {
    Self { float32: value }
  }
}
