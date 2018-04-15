use std::fmt;
use std::cmp;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;
use {AsRaw, RawStruct};

#[derive(Copy, Clone)]
pub struct VkDispatchableHandle<T>(NonNull<T>);

#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
pub struct VkNonDispatchableHandle<T>(NonNull<T>);

#[cfg(nightly)]
#[cfg(target_pointer_width = "32")]
use std::num::NonZeroU64;

#[cfg(not(nightly))]
#[cfg(target_pointer_width = "32")]
#[allow(unused)]
#[derive(Copy, Clone)]
#[repr(u64)]
enum NonZeroU64 {
  Min = 1u64,
  Max = !0u64,
}

#[cfg(target_pointer_width = "32")]
#[derive(Copy, Clone)]
pub struct VkNonDispatchableHandle<T>(NonZeroU64, ::std::marker::PhantomData<*const T>);

impl<T> VkDispatchableHandle<T> {
  #[inline]
  pub fn value(self) -> usize {
    unsafe { ::std::mem::transmute(self) }
  }
}

impl<T> VkNonDispatchableHandle<T> {
  #[inline]
  pub fn value(self) -> u64 {
    unsafe { ::std::mem::transmute(self) }
  }
}

unsafe impl<T> RawStruct for VkDispatchableHandle<T> {
  type Raw = usize;
}

unsafe impl<T> RawStruct for VkNonDispatchableHandle<T> {
  type Raw = u64;
}

// implement PartialEq, Eq, PartialOrd, Ord, Hash and Debug in the value field

impl<T: Copy> fmt::Debug for VkDispatchableHandle<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:#x}", self.value())
  }
}

impl<T: Copy> fmt::Debug for VkNonDispatchableHandle<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:#x}", self.value())
  }
}

impl<T: Copy> PartialEq for VkDispatchableHandle<T> {
  fn eq(&self, other: &Self) -> bool {
    self.value() == other.value()
  }
}
impl<T: Copy> PartialEq for VkNonDispatchableHandle<T> {
  fn eq(&self, other: &Self) -> bool {
    self.value() == other.value()
  }
}
impl<T: Copy> PartialOrd for VkDispatchableHandle<T> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.value().partial_cmp(&other.value())
  }
}
impl<T: Copy> PartialOrd for VkNonDispatchableHandle<T> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.value().partial_cmp(&other.value())
  }
}
impl<T: Copy> Eq for VkDispatchableHandle<T> {}
impl<T: Copy> Eq for VkNonDispatchableHandle<T> {}
impl<T: Copy> Ord for VkDispatchableHandle<T> {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.value().cmp(&other.value())
  }
}
impl<T: Copy> Ord for VkNonDispatchableHandle<T> {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.value().cmp(&other.value())
  }
}
impl<T: Copy> Hash for VkDispatchableHandle<T> {
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.value().hash(state)
  }
}
impl<T: Copy> Hash for VkNonDispatchableHandle<T> {
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.value().hash(state)
  }
}

#[cfg(test)]
#[test]
fn test_handle_size() {
  // This test just checks, that some my assumptions are valid.
  // I habe done sone magic with the NonZero value optimization.
  use types::{VkDevice, VkImage};
  assert_size!(*const u8, VkDispatchableHandle<u8>);
  assert_size!(8, VkNonDispatchableHandle<u8>);
  assert_size!(*const u8, VkDevice);
  assert_size!(8, VkImage);

  assert_size!(*const u8, Option<VkDispatchableHandle<u8>>);
  assert_size!(8, Option<VkNonDispatchableHandle<u8>>);
  assert_size!(*const u8, Option<VkDevice>);
  assert_size!(8, Option<VkImage>);
}

#[cfg(test)]
#[test]
fn test_handle_assignment() {
  // This test just checks, that some my assumptions are valid.
  // I habe done sone magic with the NonZero value optimization.
  use std::mem::transmute as t;
  unsafe {
    let h1: VkDispatchableHandle<u8> = t(!0usize);
    let mut h1o: Option<VkDispatchableHandle<u8>> = t(h1);
    assert_eq!(h1o, Some(h1));
    assert_eq!(true, h1o.is_some());
    assert_eq!(false, h1o.is_none());
    assert_eq!(!0usize, h1.value());
    assert_eq!(!0usize, h1o.unwrap().value());
    h1o = None;
    assert_eq!(false, h1o.is_some());
    assert_eq!(true, h1o.is_none());
    assert_eq!(0usize, t(h1o));
    let hi_bit1: usize = !((!0usize) >> 1);
    h1o = t(hi_bit1);
    assert_eq!(true, h1o.is_some());
    assert_eq!(false, h1o.is_none());
    assert_eq!(hi_bit1, h1o.unwrap().value());
    h1o = t(1usize);
    assert_eq!(true, h1o.is_some());
    assert_eq!(false, h1o.is_none());
    assert_eq!(1usize, h1o.unwrap().value());

    // in an earlier version, I used a reference as the NonZero value, and on 32-bit systems,
    // I extended the struct to 64bit with an additional u32 value. But this didn't work in the
    // following cases.
    let h2: VkNonDispatchableHandle<u8> = t(!0u64);
    let mut h2o: Option<VkNonDispatchableHandle<u8>> = t(h2);
    assert_eq!(h2o, Some(h2));
    assert_eq!(true, h2o.is_some());
    assert_eq!(false, h2o.is_none());
    assert_eq!(!0u64, t(h2));
    assert_eq!(!0u64, t(h2o));
    h2o = None;
    assert_eq!(false, h2o.is_some());
    assert_eq!(true, h2o.is_none());
    assert_eq!(0u64, t(h2o));
    let hi_bit2: u64 = !((!0u64) >> 1);
    h2o = t(hi_bit2);
    assert_eq!(true, h2o.is_some());
    assert_eq!(false, h2o.is_none());
    assert_eq!(hi_bit2, h2o.unwrap().value());
    h2o = t(1u64);
    assert_eq!(true, h2o.is_some());
    assert_eq!(false, h2o.is_none());
    assert_eq!(1u64, h2o.unwrap().value());
    h2o = t(2u64);
    assert_eq!(true, h2o.is_some());
    assert_eq!(false, h2o.is_none());
    assert_eq!(2u64, h2o.unwrap().value());
    h2o = t(1336u64);
    assert_eq!(true, h2o.is_some());
    assert_eq!(false, h2o.is_none());
    assert_eq!(1336u64, h2o.unwrap().value());
  }
}

pub fn vk_make_version(major: u16, minor: u16, patch: u16) -> u32 {
  (((major as u32) << 22) | ((minor as u32) << 12) | (patch as u32))
}

impl<T> AsRaw for VkDispatchableHandle<T> {
  type Output = usize;
  #[inline]
  unsafe fn as_raw(self) -> usize {
    self.value()
  }
}

impl<T> AsRaw for VkNonDispatchableHandle<T> {
  type Output = u64;
  #[inline]
  unsafe fn as_raw(self) -> u64 {
    self.value()
  }
}
