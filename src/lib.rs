#![cfg_attr(nightly, feature(core))]
#![cfg_attr(nightly, feature(nonzero))]

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

use std::os::raw::c_char;
use std::ffi::CStr;

#[cfg(test)]
macro_rules! assert_eq {
    ($expected:expr, $actual:expr) => {
        {
            let expected = $expected;
            let actual = $actual;
            assert!(expected == actual, "{}: expected = {:?}, actual = {:?}", stringify!($actual), expected, actual);
        }
    };
}

#[cfg(test)]
macro_rules! assert_size {
    ($expected:ty, $actual:ty) => {
        {
            let expected = ::std::mem::size_of::<$expected>();
            let actual = ::std::mem::size_of::<$actual>();
            assert!(expected == actual, "{}: expected size = {:?}, actual = {:?}", stringify!($actual), expected, actual);
        }
    };
    ($expected:expr, $actual:ty) => {
        {
            let expected : usize = $expected;
            let actual = ::std::mem::size_of::<$actual>();
            assert!(expected == actual, "{}: expected size = {:?}, actual = {:?}", stringify!($actual), expected, actual);
        }
    };
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

pub mod utils;
pub mod platform;

// generated modules
pub mod enums;
pub mod types_base;
pub mod types_raw;
pub mod types;
mod protos;
mod dispatch_table;
pub mod dispatch_commands;
pub mod prelude;
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
