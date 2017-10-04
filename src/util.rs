/*
**  Copyright (c) 2016, Christoph Hommelsheim
**  All rights reserved.
**
**  Redistribution and use in source and binary forms, with or without
**  modification, are permitted provided that the following conditions are met:
**
**  * Redistributions of source code must retain the above copyright notice, this
**    list of conditions and the following disclaimer.
**
**  * Redistributions in binary form must reproduce the above copyright notice,
**    this list of conditions and the following disclaimer in the documentation
**    and/or other materials provided with the distribution.
**
**  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
**  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
**  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
**  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
**  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
**  DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
**  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
**  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
**  OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
**  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
**
*/

//! utilities

use std::fmt;
use std::ops;
use std::collections::BTreeSet;
use std::borrow::Cow;
use std::os::raw;
use std::ffi::CStr;
use types;

/// Holds a compressed version triple.
///
/// - Bits 0 (LSB) to 11: patch version
/// - Bits 12 to 21: minor version
/// - Bits 22 to 31: major version
#[repr(C)]
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct VkVersion(pub u32);

impl VkVersion {
    #[inline]
    pub fn new(major: u32, minor: u32, patch: u32) -> VkVersion {
        // TODO: make `const fn` when feature stabilized
        VkVersion((major << 22) | (minor << 12) | patch)
    }
    #[inline]
    pub fn major(self) -> u32 {
        self.0 >> 22
    }
    #[inline]
    pub fn minor(self) -> u32 {
        (self.0 >> 12) & 0x3ff
    }
    #[inline]
    pub fn patch(self) -> u32 {
        self.0 & 0xfff
    }
}

impl fmt::Display for VkVersion {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}
impl fmt::Debug for VkVersion {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VkVersion({}.{}.{})", self.major(), self.minor(), self.patch())
    }
}
impl fmt::LowerHex for VkVersion {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
impl fmt::UpperHex for VkVersion {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl Into<u32> for VkVersion {
    #[inline]
    fn into(self) -> u32 {
        self.0
    }
}

impl From<u32> for VkVersion {
    #[inline]
    fn from(value: u32) -> VkVersion {
        VkVersion(value)
    }
}

/// Base-type for a dispatchable object handle.
///
/// The only dispatchable handle types are those related to device and instance management.
#[repr(C)]
#[derive(Copy,Clone,PartialEq,Eq)]
pub struct VkDispatchableHandle{
    value: usize,
}

/// Base-type for a non-dispatchable object handle.
///
/// Most Vulkan handle types, are non-dispatchable.
#[repr(C)]
#[derive(Copy,Clone,PartialEq,Eq)]
pub struct VkNonDispatchableHandle {
    value: u64,
}

impl VkDispatchableHandle {
    pub const NULL : VkDispatchableHandle = VkDispatchableHandle { value: 0 };
}
impl VkNonDispatchableHandle {
    pub const NULL : VkNonDispatchableHandle = VkNonDispatchableHandle { value: 0 };
}

impl fmt::Debug for VkDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "`H:{:#x}`", self.value)
    }
}
impl fmt::Display for VkDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "`H:{:#x}`", self.value)
    }
}
impl fmt::Pointer for VkDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:#x}", self.value)
    }
}
impl fmt::LowerHex for VkDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:x}", self.value)
    }
}
impl fmt::UpperHex for VkDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:X}", self.value)
    }
}

impl fmt::Debug for VkNonDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "`N:{:#x}`", self.value)
    }
}
impl fmt::Display for VkNonDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "`N:{:#x}`", self.value)
    }
}
impl fmt::Pointer for VkNonDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:#x}", self.value)
    }
}
impl fmt::LowerHex for VkNonDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:x}", self.value)
    }
}
impl fmt::UpperHex for VkNonDispatchableHandle {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:X}", self.value)
    }
}

impl VkNullHandle for VkDispatchableHandle {
    const NULL : VkDispatchableHandle = VkDispatchableHandle::NULL;

    #[inline]
    fn null() -> VkDispatchableHandle {
        VkDispatchableHandle::NULL
    }
}

impl VkNullHandle for VkNonDispatchableHandle {
    const NULL : VkNonDispatchableHandle = VkNonDispatchableHandle::NULL;

    #[inline]
    fn null() -> VkNonDispatchableHandle {
        VkNonDispatchableHandle::NULL
    }
}

impl Default for VkDispatchableHandle {
    #[inline]
    fn default() -> VkDispatchableHandle {
        VkDispatchableHandle::NULL
    }
}

impl Default for VkNonDispatchableHandle {
    #[inline]
    fn default() -> VkNonDispatchableHandle {
        VkNonDispatchableHandle::NULL
    }
}

pub trait VkDestroyableHandle: VkNullHandle {
    type Owner: Default+Copy+Clone;
    fn destroy(self, owner: Self::Owner, p_allocator: Option<&types::VkAllocationCallbacks>);
}

pub type VkError = ::types::VkResult;

impl VkError {
    #[inline]
    pub fn is_success(self) -> bool{
        return (self as i32) >= 0;
    }
    #[inline]
    pub fn is_error(self) -> bool {
        return (self as i32) < 0;
    }

    #[inline]
    pub fn into_result(self) -> VkResultObj {
        if self.is_error() {
            Err(self)
        } else {
            Ok(self)
        }
    }

    #[inline]
    pub fn from_result(result: VkResultObj) -> Self {
        match result {
            Ok(r) | Err(r) => r,
        }
    }
}

impl fmt::Display for VkError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}
impl fmt::LowerHex for VkError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", *self as u32)
    }
}
impl fmt::UpperHex for VkError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", *self as u32)
    }
}

// impl error::Error for VkError {
//     #[inline]
//     fn description(&self) -> &str {
//         return get_VkResult_description(self.0);
//     }
// }

impl Into<VkResultObj> for VkError {
    #[inline]
    fn into(self) -> VkResultObj {
        if self.is_error() {
            Err(self)
        } else {
            Ok(self)
        }
    }
}

impl Into<VkResultObj<()>> for VkError {
    #[inline]
    fn into(self) -> VkResultObj<()> {
        if self != ::types::VK_SUCCESS {
            Err(self)
        } else {
            Ok(())
        }
    }
}

impl From<VkResultObj> for VkError {
    #[inline]
    fn from(result: VkResultObj) -> VkError {
        match result {
            Ok(r) | Err(r) => r,
        }
    }
}

impl From<VkResultObj<()>> for VkError {
    #[inline]
    fn from(result: VkResultObj<()>) -> VkError {
        match result {
            Ok(()) => ::types::VK_SUCCESS,
            Err(r) => r,
        }
    }
}

#[cfg_attr(feature="nightly", feature(try_trait))]
#[cfg(feature="nightly")]
impl ops::Try for VkError {
    type Ok = VkError;
    type Error = VkError;

    #[inline]
    fn into_result(self) -> Result<VkError,VkError> {
        if self.is_error() {
            Err(self)
        } else {
            Ok(self)
        }
    }

    #[inline]
    fn from_ok(v: VkError) -> VkError {
        v
    }

    #[inline]
    fn from_error(v: VkError) -> VkError {
        v
    }
}


/// A `std::result::Result` wrapper for `VkResult`.
pub type VkResultObj<T=::types::VkResult> = Result<T, VkError>;

pub use std::ptr::null_mut as vk_null;

/// Support trait for the `vk_null_handle()` function
pub trait VkNullHandle: Sized+PartialEq+Eq+Copy+Clone+Sized {

    const NULL : Self;

    /// Returns a reserved non-valid object handle.
    #[inline]
    fn null() -> Self {
        Self::NULL
    }

    /// tests if the handle is the NULL_HANDLE
    #[inline]
    fn is_null(self) -> bool {
        self == Self::NULL
    }
}

/// Returns a reserved non-valid object handle.
#[inline]
pub fn vk_null_handle<T>() -> T where T: VkNullHandle {
    T::NULL
}

pub unsafe fn extensions_list_to_set<'l>(p: *const *const raw::c_char, len: u32) -> BTreeSet<Cow<'l, str>> {
    let mut extensions : BTreeSet<Cow<str>> = BTreeSet::new();
    if !p.is_null() {
        for ext in ::std::slice::from_raw_parts(p, len as usize) {
            if ext.is_null() {
                break;
            }
            extensions.insert(CStr::from_ptr(*ext).to_string_lossy());
        }
    }
    extensions
}

pub trait VkFlagBits: Copy + Clone + 'static {
    fn value(self) -> u32;
    #[inline]
    fn flags(self) -> VkFlags<Self> {
        VkFlags::one(self)
    }
}

#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub enum VkVoid{}

impl VkFlagBits for VkVoid {
    fn value(self) -> u32 {
        unreachable!()
    }
}

#[repr(C)]
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct VkFlags<E:VkFlagBits=VkVoid>(u32, ::std::marker::PhantomData<E>);

impl<E:VkFlagBits> VkFlags<E> {

    pub const NONE : VkFlags<E> = VkFlags(0, ::std::marker::PhantomData);

    #[inline]
    pub fn none() -> VkFlags<E> {
        VkFlags::NONE
    }

    #[inline]
    pub fn one(e: E) -> VkFlags<E> {
        VkFlags(e.value(), ::std::marker::PhantomData)
    }

    #[inline]
    pub fn all() -> VkFlags<E> {
        VkFlags(0xffffffff, ::std::marker::PhantomData)
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_not_empty(self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn contains(self, other: E) -> bool {
        self.0 & other.value() != 0
    }

    #[inline]
    pub fn contains_one_of(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub fn contains_all_of(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl<E:VkFlagBits> Default for VkFlags<E> {
    #[inline]
    fn default() -> VkFlags<E> {
        VkFlags::NONE
    }
}

impl<E:VkFlagBits> From<E> for VkFlags<E> {
    #[inline]
    fn from(e: E) -> VkFlags<E> {
        VkFlags::one(e)
    }
}

impl<E:VkFlagBits> ops::BitAnd<E> for VkFlags<E> {
    type Output = VkFlags<E>;
    #[inline]
    fn bitand(self, rhs: E) -> VkFlags<E> {
        VkFlags(self.0 & rhs.value(), ::std::marker::PhantomData)
    }
}
impl<E:VkFlagBits> ops::BitOr<E> for VkFlags<E> {
    type Output = VkFlags<E>;
    #[inline]
    fn bitor(self, rhs: E) -> VkFlags<E> {
        VkFlags(self.0 | rhs.value(), ::std::marker::PhantomData)
    }
}
impl<E:VkFlagBits> ops::BitAnd<VkFlags<E>> for VkFlags<E> {
    type Output = VkFlags<E>;
    #[inline]
    fn bitand(self, rhs: VkFlags<E>) -> VkFlags<E> {
        VkFlags(self.0 & rhs.0, ::std::marker::PhantomData)
    }
}
impl<E:VkFlagBits> ops::BitOr<VkFlags<E>> for VkFlags<E> {
    type Output = VkFlags<E>;
    #[inline]
    fn bitor(self, rhs: VkFlags<E>) -> VkFlags<E> {
        VkFlags(self.0 | rhs.0, ::std::marker::PhantomData)
    }
}

impl<E:VkFlagBits> fmt::Debug for VkFlags<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VkFlags({:#x})", self.0)
    }
}
impl<E:VkFlagBits> fmt::Display for VkFlags<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
impl<E:VkFlagBits> fmt::LowerHex for VkFlags<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
impl<E:VkFlagBits> fmt::UpperHex for VkFlags<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}
impl<E:VkFlagBits> fmt::Octal for VkFlags<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:o}", self.0)
    }
}
impl<E:VkFlagBits> fmt::Binary for VkFlags<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}
