/*
Copyright (c) 2016, Christoph Hommelsheim
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

//! Types and constants used by the vulkan api.

use std::os::raw;
use platform::_all::*;

/// Base type for enumerants.
pub type VkEnum = u32;

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
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct VkHandle {
    value: usize,
}

/// Base-type for a non-dispatchable object handle.
///
/// Most Vulkan handle types, are non-dispatchable.
#[repr(C)]
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct VkNonDispatchableHandle {
    value: u64,
}


impl ::std::fmt::Debug for VkHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "`H:{:x}`", self.value)
    }
}

impl ::std::fmt::Debug for VkNonDispatchableHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "`N:{:x}`", self.value)
    }
}

impl ::util::VkNullHandle for VkHandle {
    #[inline]
    fn null() -> VkHandle {
        return VkHandle { value: 0 };
    }
}

impl ::util::VkNullHandle for VkNonDispatchableHandle {
    #[inline]
    fn null() -> VkNonDispatchableHandle {
        return VkNonDispatchableHandle { value: 0 };
    }
}

impl Default for VkHandle {
    #[inline]
    fn default() -> VkHandle {
        return VkHandle { value: 0 };
    }
}

impl Default for VkNonDispatchableHandle {
    #[inline]
    fn default() -> VkNonDispatchableHandle {
        return VkNonDispatchableHandle { value: 0 };
    }
}


include!(concat!(env!("OUT_DIR"), "/vulkan_types.rs"));
