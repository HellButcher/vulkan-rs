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
// end of generated modules

mod dl;
mod loader;

trait AsRaw {
  type Output: Copy;
  unsafe fn as_raw(self) -> Self::Output;
}

unsafe trait Struct: Sized {}
unsafe trait Primitive {}

pub unsafe trait StructExtends<T> {
  #[doc(hidden)]
  unsafe fn extend(&self, next: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void;
}

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

unsafe impl<P> Struct for P
where
  P: Primitive,
{
}

impl<T> AsRaw for Option<T>
where
  T: AsRaw,
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

impl<'l, T: 'l + Struct> AsRaw for &'l T {
  type Output = *const T;
  #[inline]
  unsafe fn as_raw(self) -> *const T {
    self
  }
}

impl<'l, T: 'l + Struct> AsRaw for &'l mut T {
  type Output = *mut T;
  #[inline]
  unsafe fn as_raw(self) -> *mut T {
    self
  }
}

impl<'l, T: 'l + Struct> AsRaw for &'l [T] {
  type Output = *const T;
  #[inline]
  unsafe fn as_raw(self) -> *const T {
    if self.len() > 0 {
      self.as_ptr()
    } else {
      ::std::ptr::null()
    }
  }
}
impl<'l, T: 'l + Struct> AsRaw for &'l mut [T] {
  type Output = *mut T;
  #[inline]
  unsafe fn as_raw(self) -> *mut T {
    if self.len() > 0 {
      self.as_mut_ptr()
    } else {
      ::std::ptr::null_mut()
    }
  }
}

impl<T: Struct> AsRaw for *const T {
  type Output = *const T;
  #[inline]
  unsafe fn as_raw(self) -> *const T {
    self
  }
}

impl<T: Struct> AsRaw for *mut T {
  type Output = *mut T;
  #[inline]
  unsafe fn as_raw(self) -> *mut T {
    self
  }
}

impl<'a> AsRaw for &'a AsRef<CStr> {
  type Output = *const c_char;
  #[inline]
  unsafe fn as_raw(self) -> *const c_char {
    self.as_ref().as_ptr()
  }
}

impl AsRaw for bool {
  type Output = types::VkBool32;
  #[inline]
  unsafe fn as_raw(self) -> types::VkBool32 {
    if self {
      1
    } else {
      0
    }
  }
}
