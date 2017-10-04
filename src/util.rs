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
pub type VkVersionInfo = u32;

/// Base-type for a dispatchable object handle.
///
/// The only dispatchable handle types are those related to device and instance management.
#[repr(C)]
#[derive(Copy,Clone,PartialEq,Eq)]
pub struct VkDispatchableHandle{
    value: usize,
    //phantom: ::std::marker::PhantomData<& 'l raw::c_void>
}

/// Base-type for a non-dispatchable object handle.
///
/// Most Vulkan handle types, are non-dispatchable.
#[repr(C)]
#[derive(Copy,Clone,PartialEq,Eq)]
pub struct VkNonDispatchableHandle {
    value: u64,
    //phantom: ::std::marker::PhantomData<& 'l raw::c_void>
}

impl fmt::Debug for VkDispatchableHandle {
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

impl fmt::Debug for VkNonDispatchableHandle {
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

impl VkNullHandle for VkDispatchableHandle {
    #[inline]
    fn null() -> VkDispatchableHandle {
        VkDispatchableHandle { value: 0 } //, phantom: ::std::marker::PhantomData }
    }
}

impl VkNullHandle for VkNonDispatchableHandle {
    #[inline]
    fn null() -> VkNonDispatchableHandle {
        VkNonDispatchableHandle { value: 0 } //, phantom: ::std::marker::PhantomData }
    }
}

impl Default for VkDispatchableHandle {
    #[inline]
    fn default() -> VkDispatchableHandle {
        VkDispatchableHandle { value: 0 } //, phantom: ::std::marker::PhantomData }
    }
}

impl Default for VkNonDispatchableHandle {
    #[inline]
    fn default() -> VkNonDispatchableHandle {
        VkNonDispatchableHandle { value: 0 } //, phantom: ::std::marker::PhantomData }
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
impl ::std::ops::Try for VkError {
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
    /// Returns a reserved non-valid object handle.
    #[inline]
    fn null() -> Self;

    /// tests if the handle is the NULL_HANDLE
    #[inline]
    fn is_null(self) -> bool {
        self == Self::null()
    }
}

/// Returns a reserved non-valid object handle.
#[inline]
pub fn vk_null_handle<T>() -> T where T: VkNullHandle {
    T::null()
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
