use std::cmp;
use std::fmt;
use std::hash::{Hash, Hasher};
use {AsRaw, RawStruct};

#[derive(Copy, Clone)]
pub struct VkDispatchableHandle<'l,T: 'l>(&'l T);

#[cfg(target_pointer_width = "64")]
#[derive(Copy, Clone)]
pub struct VkNonDispatchableHandle<'l,T: 'l>(&'l T);

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
pub struct VkNonDispatchableHandle<'l, T>(NonZeroU64, ::std::marker::PhantomData<&'l T>);

impl<'l, T> VkDispatchableHandle<'l, T> {
  #[inline]
  pub fn value(&self) -> usize {
    unsafe { *(self as *const Self as *const usize) }
  }
}

impl<'l, T> VkNonDispatchableHandle<'l , T> {
  #[inline]
  pub fn value(&self) -> u64 {
    unsafe { *(self as *const Self as *const u64) }
  }
}

unsafe impl<'l, T> RawStruct for VkDispatchableHandle<'l, T> {
  type Raw = usize;
}

unsafe impl<'l, T> RawStruct for VkNonDispatchableHandle<'l, T> {
  type Raw = u64;
}

// implement PartialEq, Eq, PartialOrd, Ord, Hash and Debug in the value field

impl<'l, T: Copy> fmt::Debug for VkDispatchableHandle<'l, T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:#x}", self.value())
  }
}

impl<'l, T: Copy> fmt::Debug for VkNonDispatchableHandle<'l, T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:#x}", self.value())
  }
}

impl<'l, T: Copy> PartialEq for VkDispatchableHandle<'l, T> {
  fn eq(&self, other: &Self) -> bool {
    self.value() == other.value()
  }
}
impl<'l, T: Copy> PartialEq for VkNonDispatchableHandle<'l, T> {
  fn eq(&self, other: &Self) -> bool {
    self.value() == other.value()
  }
}
impl<'l, T: Copy> PartialOrd for VkDispatchableHandle<'l, T> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.value().partial_cmp(&other.value())
  }
}
impl<'l, T: Copy> PartialOrd for VkNonDispatchableHandle<'l, T> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.value().partial_cmp(&other.value())
  }
}
impl<'l, T: Copy> Eq for VkDispatchableHandle<'l, T> {}
impl<'l, T: Copy> Eq for VkNonDispatchableHandle<'l, T> {}
impl<'l, T: Copy> Ord for VkDispatchableHandle<'l, T> {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.value().cmp(&other.value())
  }
}
impl<'l, T: Copy> Ord for VkNonDispatchableHandle<'l, T> {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.value().cmp(&other.value())
  }
}
impl<'l, T: Copy> Hash for VkDispatchableHandle<'l, T> {
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.value().hash(state)
  }
}
impl<'l, T: Copy> Hash for VkNonDispatchableHandle<'l, T> {
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
  // I have done sone magic with the NonZero value optimization.
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
  // I have done sone magic with the NonZero value optimization.
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

#[cfg(test)]
#[test]
fn test_result_size() {
  // This test just checks, that some my assumptions are valid.
  // I have done sone magic with the NonZero value optimization.
  use enums::{VkError, VkResult};
  assert_size!(4, VkError);
  assert_size!(4, VkResult);
  assert_size!(8, VkResult<u32>);
}

#[cfg(test)]
#[test]
fn test_result_assignment() {
  // This test just checks, that some my assumptions are valid.
  // I have done sone magic with the NonZero value optimization.
  use enums::{VkError, VkResult};
  use std::mem::transmute as t;
  unsafe {
    let r0: VkResult = t(0u32);
    assert_eq!(Ok(()), r0);
    let r1: VkResult = t(1u32);
    assert_eq!(Err(VkError::NOT_READY), r1);
    let r2: VkResult = t(2u32);
    assert_eq!(Err(VkError::TIMEOUT), r2);
    let r3: VkResult = t(3u32);
    assert_eq!(Err(VkError::EVENT_SET), r3);
    let r4: VkResult = t(4u32);
    assert_eq!(Err(VkError::EVENT_RESET), r4);
    let r5: VkResult = t(5u32);
    assert_eq!(Err(VkError::INCOMPLETE), r5);

    let r6: VkResult = t(!0u32);
    assert_eq!(Err(VkError::ERROR_OUT_OF_HOST_MEMORY), r6);
    let r7: VkResult = t(!2u32);
    assert_eq!(Err(VkError::ERROR_INITIALIZATION_FAILED), r7);
  }
}

pub fn vk_make_version(major: u16, minor: u16, patch: u16) -> u32 {
  (((major as u32) << 22) | ((minor as u32) << 12) | (patch as u32))
}

impl<'h, T> AsRaw for VkDispatchableHandle<'h, T> {
  type Output = usize;
  #[inline]
  unsafe fn as_raw(self) -> usize {
    self.value()
  }
}

impl<'h, T> AsRaw for VkNonDispatchableHandle<'h, T> {
  type Output = u64;
  #[inline]
  unsafe fn as_raw(self) -> u64 {
    self.value()
  }
}

#[inline]
pub fn cstr_from_bytes_until_nul<'a, T: AsRef<[u8]> + ?Sized>(s: &'a T) -> ::std::borrow::Cow<'a, ::std::ffi::CStr> {
  use std::borrow::Cow;
  use std::ffi::{CStr, CString};
  let s = s.as_ref();
  unsafe {
    for (i, &c) in s.into_iter().enumerate() {
      if c == 0 {
        return Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(&s[0..i]));
      }
    }
    let mut owned = Vec::<u8>::new();
    owned.extend_from_slice(s);
    Cow::Owned(CString::from_vec_unchecked(owned))
  }
}
