#![cfg_attr(nightly, feature(core))]
#![cfg_attr(nightly, feature(nonzero))]
#![cfg_attr(nightly, feature(try_trait))]

#[cfg(nightly)]
extern crate core;

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;

#[macro_use]
extern crate log;

#[macro_use]
extern crate bitflags;

use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_char;

#[cfg(test)]
macro_rules! assert_eq {
  ($expected:expr, $actual:expr) => {{
    let expected = $expected;
    let actual = $actual;
    assert!(
      expected == actual,
      "{}: expected = {:?}, actual = {:?}",
      stringify!($actual),
      expected,
      actual
    );
  }};
}

#[cfg(test)]
macro_rules! assert_size {
  ($expected:ty, $actual:ty) => {{
    let expected = ::std::mem::size_of::<$expected>();
    let actual = ::std::mem::size_of::<$actual>();
    assert!(
      expected == actual,
      "{}: expected size = {:?}, actual = {:?}",
      stringify!($actual),
      expected,
      actual
    );
  }};
  ($expected:expr, $actual:ty) => {{
    let expected: usize = $expected;
    let actual = ::std::mem::size_of::<$actual>();
    assert!(
      expected == actual,
      "{}: expected size = {:?}, actual = {:?}",
      stringify!($actual),
      expected,
      actual
    );
  }};
}

macro_rules! define_enum {
  (
    $(#[$outer:meta])*
    pub enum $name:ident {
      $(
        $(#[$inner:ident $($args:tt)*])*
        $item:ident = $value:expr
      ),+
    }
  ) => {
    $(#[$outer])*
    #[repr(u32)]
    #[allow(non_camel_case_types)]
    #[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
    pub enum $name {
      $(
        $(#[$inner $($args)*])*
        $item = $value
      ),+
    }
    unsafe impl $crate::Primitive for $name {}
  };
}

macro_rules! define_bitmask {
  (
    $(#[$outer:meta])*
    pub enum $name:ident {
      $(
        $(#[$inner:ident $($args:tt)*])*
        $item:ident = $value:expr
      ),+
    }
  ) => {
    bitflags! {
      $(#[$outer])*
      pub struct $name: u32 {
        $(
          $(#[$inner $($args)*])*
          const $item = $value;
        )+
      }
    }
    unsafe impl $crate::Primitive for $name {}
  };
}

macro_rules! vk_make_version {
  ($major:expr, $minor:expr) => {
    vk_make_version!($major, $minor, 0)
  };
  ($major:expr, $minor:expr, $patch:expr) => {
    (($major << 22) | ($minor << 12) | $patch)
  };
}

pub mod platform;
pub mod utils;

// generated modules
pub mod dispatch_commands;
mod dispatch_table;
pub mod enums;
pub mod prelude;
mod protos;
pub mod types;
pub mod types_base;
pub mod types_raw;
// end of generated modules

mod dl;
mod loader;

unsafe trait RawStruct: Sized {
  type Raw;

  #[inline]
  fn as_raw(&self) -> &Self::Raw {
    unsafe { ::std::mem::transmute(self) }
  }
  #[inline]
  unsafe fn as_raw_mut(&mut self) -> &mut Self::Raw {
    ::std::mem::transmute(self)
  }
}

trait AsRaw {
  type Output: Copy;
  unsafe fn as_raw(self) -> Self::Output;
}

pub unsafe trait StructExtends<T> {
  #[doc(hidden)]
  unsafe fn extend(&self, next: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void;
}

unsafe trait Primitive {}

trait Zero {
  fn zero() -> Self;
}

macro_rules! primitive_impls {
  ($($T:ty = $V:expr),+) => {
    $(
      impl Zero for $T {
        #[inline]
        fn zero() -> $T { $V }
      }
      unsafe impl Primitive for $T{}
    )+
  }
}

primitive_impls!{
  u8 = 0, i8 = 0, u16 = 0, i16 = 0, u32 = 0, i32 = 0, u64 = 0, i64 = 0,
  usize = 0, isize = 0, f32 = 0.0, f64 = 0.0
}

impl<T: Sized> Zero for *const T {
  #[inline]
  fn zero() -> *const T {
    ::std::ptr::null()
  }
}

impl<T: Sized> Zero for *mut T {
  #[inline]
  fn zero() -> *mut T {
    ::std::ptr::null_mut()
  }
}

unsafe impl<P> RawStruct for P
where
  P: Primitive,
{
  type Raw = Self;
  #[inline]
  fn as_raw(&self) -> &Self {
    self
  }
  #[inline]
  unsafe fn as_raw_mut(&mut self) -> &mut Self {
    self
  }
}

impl<T: AsRaw> AsRaw for Option<T>
where
  T::Output: Zero,
{
  type Output = T::Output;
  #[inline]
  unsafe fn as_raw(self) -> T::Output {
    if let Some(v) = self {
      v.as_raw()
    } else {
      T::Output::zero()
    }
  }
}

impl<'a, T> AsRaw for &'a T
where
  T: RawStruct,
{
  type Output = *const T::Raw;
  #[inline]
  unsafe fn as_raw(self) -> *const T::Raw {
    RawStruct::as_raw(self)
  }
}

impl<'a, T> AsRaw for &'a mut T
where
  T: RawStruct,
{
  type Output = *mut T::Raw;
  #[inline]
  unsafe fn as_raw(self) -> *mut T::Raw {
    RawStruct::as_raw_mut(self)
  }
}

impl<'a, T> AsRaw for &'a [T]
where
  T: RawStruct,
{
  type Output = *const T::Raw;
  #[inline]
  unsafe fn as_raw(self) -> *const T::Raw {
    if let Some(a) = self.first() {
      RawStruct::as_raw(a)
    } else {
      ::std::ptr::null()
    }
  }
}

impl<'a, T> AsRaw for &'a mut [T]
where
  T: RawStruct,
{
  type Output = *mut T::Raw;
  #[inline]
  unsafe fn as_raw(self) -> *mut T::Raw {
    if let Some(a) = self.first_mut() {
      RawStruct::as_raw_mut(a)
    } else {
      ::std::ptr::null_mut()
    }
  }
}

impl<T> AsRaw for *const T
where
  T: RawStruct,
{
  type Output = *const T::Raw;
  #[inline]
  unsafe fn as_raw(self) -> *const T::Raw {
    RawStruct::as_raw(&(*self))
  }
}

impl<T> AsRaw for *mut T
where
  T: RawStruct,
{
  type Output = *mut T::Raw;
  #[inline]
  unsafe fn as_raw(self) -> *mut T::Raw {
    RawStruct::as_raw_mut(&mut (*self))
  }
}

impl<'a> AsRaw for &'a AsRef<CStr> {
  type Output = *const c_char;
  #[inline]
  unsafe fn as_raw(self) -> *const c_char {
    self.as_ref().as_ptr()
  }
}

// manual functions

impl<'a> types::VkInstanceCreateInfo<'a> {
  pub fn set_enabled_extension_names(mut self, value: &[*const c_char]) -> Self {
    unsafe {
      self.as_raw_mut().enabledExtensionCount = value.len() as u32;
      self.as_raw_mut().ppEnabledExtensionNames = value.as_ptr();
      self
    }
  }
  pub fn set_enabled_layer_names(mut self, value: &[*const c_char]) -> Self {
    unsafe {
      self.as_raw_mut().enabledLayerCount = value.len() as u32;
      self.as_raw_mut().ppEnabledLayerNames = value.as_ptr();
      self
    }
  }
}

impl<'a> types::VkDeviceCreateInfo<'a> {
  pub fn set_enabled_extension_names(mut self, value: &[*const c_char]) -> Self {
    unsafe {
      self.as_raw_mut().enabledExtensionCount = value.len() as u32;
      self.as_raw_mut().ppEnabledExtensionNames = value.as_ptr();
      self
    }
  }
  pub fn set_enabled_layer_names(mut self, value: &[*const c_char]) -> Self {
    unsafe {
      self.as_raw_mut().enabledLayerCount = value.len() as u32;
      self.as_raw_mut().ppEnabledLayerNames = value.as_ptr();
      self
    }
  }
}

impl types::VkExtent2D {
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
  pub fn with_depth(self, depth: u32) -> types::VkExtent3D {
    types::VkExtent3D {
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

impl<I: Into<(u32, u32)>> From<I> for types::VkExtent2D {
  #[inline]
  fn from(value: I) -> Self {
    let (width, height) = value.into();
    Self { width, height }
  }
}

impl fmt::Debug for types::VkExtent2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkExtent2D({:?},{:?})", self.width, self.height)
  }
}

impl types::VkExtent3D {
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

impl<I: Into<(u32, u32, u32)>> From<I> for types::VkExtent3D {
  #[inline]
  fn from(value: I) -> Self {
    let (width, height, depth) = value.into();
    Self { width, height, depth }
  }
}

impl fmt::Debug for types::VkExtent3D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkExtent3D({:?},{:?},{:?})", self.width, self.height, self.depth)
  }
}

impl types::VkOffset2D {
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
  pub fn with_z(self, z: i32) -> types::VkOffset3D {
    types::VkOffset3D {
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

impl<I: Into<(i32, i32)>> From<I> for types::VkOffset2D {
  #[inline]
  fn from(value: I) -> Self {
    let (x, y) = value.into();
    Self { x, y }
  }
}

impl fmt::Debug for types::VkOffset2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkOffset2D({:?},{:?})", self.x, self.y)
  }
}

impl types::VkOffset3D {
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

impl<I: Into<(i32, i32, i32)>> From<I> for types::VkOffset3D {
  #[inline]
  fn from(value: I) -> Self {
    let (x, y, z) = value.into();
    Self { x, y, z }
  }
}

impl fmt::Debug for types::VkOffset3D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkOffset3D({:?},{:?},{:?})", self.x, self.y, self.z)
  }
}

impl types::VkRect2D {
  pub const ZERO: Self = Self {
    offset: types::VkOffset2D::ZERO,
    extent: types::VkExtent2D::ZERO,
  };
  pub const MAX: Self = Self {
    offset: types::VkOffset2D::MIN,
    extent: types::VkExtent2D::MAX,
  };
  #[inline]
  pub fn of<O, E>(offset: O, extent: E) -> Self
  where
    O: Into<types::VkOffset2D>,
    E: Into<types::VkExtent2D>,
  {
    Self {
      offset: offset.into(),
      extent: extent.into(),
    }
  }
}

impl fmt::Debug for types::VkRect2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "VkRect2D({:?},{:?})", self.offset, self.extent)
  }
}

impl types::VkComponentMapping {
  pub const IDENTITY: Self = Self {
    r: enums::VkComponentSwizzle::R,
    g: enums::VkComponentSwizzle::G,
    b: enums::VkComponentSwizzle::B,
    a: enums::VkComponentSwizzle::A,
  };
}

impl fmt::Debug for types::VkComponentMapping {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{:?},{:?},{:?},{:?}]", self.r, self.g, self.b, self.a)
  }
}

impl types::VkClearColorValue {
  pub const BLACK: Self = Self { uint32: [0, 0, 0, 0] };
}

impl From<(u32, u32, u32, u32)> for types::VkClearColorValue {
  #[inline]
  fn from(value: (u32, u32, u32, u32)) -> Self {
    Self {
      uint32: [value.0, value.1, value.2, value.3],
    }
  }
}

impl From<[u32; 4]> for types::VkClearColorValue {
  #[inline]
  fn from(value: [u32; 4]) -> Self {
    Self { uint32: value }
  }
}

impl From<(i32, i32, i32, i32)> for types::VkClearColorValue {
  #[inline]
  fn from(value: (i32, i32, i32, i32)) -> Self {
    Self {
      int32: [value.0, value.1, value.2, value.3],
    }
  }
}

impl From<[i32; 4]> for types::VkClearColorValue {
  #[inline]
  fn from(value: [i32; 4]) -> Self {
    Self { int32: value }
  }
}

impl From<(f32, f32, f32, f32)> for types::VkClearColorValue {
  #[inline]
  fn from(value: (f32, f32, f32, f32)) -> Self {
    Self {
      float32: [value.0, value.1, value.2, value.3],
    }
  }
}

impl From<[f32; 4]> for types::VkClearColorValue {
  #[inline]
  fn from(value: [f32; 4]) -> Self {
    Self { float32: value }
  }
}
