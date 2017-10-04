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

//! Vulkan bindings for the rust programming language.
//!
//! # Usage
//!
//! ```rust
//! extern crate vulkan_rs;
//! use vulkan_rs::prelude::vk_version_1_0::*;
//! use std::ffi::CString;
//!
//! fn main() {
//!     let app_aame = CString::new("Application name").unwrap();
//!     let app_info = VkApplicationInfo {
//!         sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
//!         pNext: vk_null(),
//!         pApplicationName: app_aame.as_ptr(),
//!         applicationVersion: VkVersion::new(1,0,0).into(),
//!         pEngineName: app_aame.as_ptr(),
//!         engineVersion: VkVersion::new(1,0,0).into(),
//!         apiVersion: VK_API_VERSION_1_0.into(),
//!     };
//!     let create_info = VkInstanceCreateInfo {
//!         sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
//!         pNext: vk_null(),
//!         flags: VkFlags::NONE,
//!         pApplicationInfo: &app_info,
//!         enabledLayerCount: 0,
//!         ppEnabledLayerNames: vk_null(),
//!         enabledExtensionCount: 0,
//!         ppEnabledExtensionNames: vk_null(),
//!     };
//!     let instance = vkCreateInstance(&create_info, None).unwrap();
//!     println!("created instance {:?}", instance);
//!     // ...
//!     vkDestroyInstance(instance, None);
//! }
//! ```



#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

/// Construct an API version number.
///
/// This macro can be used when constructing the `VkApplicationInfo.apiVersion` parameter passed to `vkCreateInstance`.
macro_rules! vk_make_version {
    // TODO: use `const fn` when feature stabilized
    ( $major:expr, $minor:expr, $patch:expr ) => {
        $crate::util::VkVersion(($major << 22) | ($minor << 12) | $patch)
    };
}

/// Define a bitmask-type for a coresponding bit-enumeration.
macro_rules! vk_define_bitmask {
    ( $bitmask_ty:ident, $enum_type:ty ) => {
        pub type $bitmask_ty = VkFlags<$enum_type>;
        impl $enum_type {
            #[inline]
            pub fn flags(self) -> $bitmask_ty {
                $bitmask_ty::one(self)
            }
        }
        impl $crate::util::VkFlagBits for $enum_type {
            #[inline]
            fn value(self) -> u32 {
                self as u32
            }
        }
        impl ::std::ops::BitAnd<$enum_type> for $enum_type {
            type Output = $bitmask_ty;
            #[inline]
            fn bitand(self, rhs: $enum_type) -> $bitmask_ty {
                $bitmask_ty::one(self) & rhs
            }
        }
        impl ::std::ops::BitOr<$enum_type> for $enum_type {
            type Output = $bitmask_ty;
            #[inline]
            fn bitor(self, rhs: $enum_type) -> $bitmask_ty {
                $bitmask_ty::one(self) | rhs
            }
        }
        impl ::std::ops::BitAnd<$bitmask_ty> for $enum_type {
            type Output = $bitmask_ty;
            #[inline]
            fn bitand(self, rhs: $bitmask_ty) -> $bitmask_ty {
                $bitmask_ty::one(self) & rhs
            }
        }
        impl ::std::ops::BitOr<$bitmask_ty> for $enum_type {
            type Output = $bitmask_ty;
            #[inline]
            fn bitor(self, rhs: $bitmask_ty) -> $bitmask_ty {
                $bitmask_ty::one(self) | rhs
            }
        }
    };
    ( $bitmask_ty:ident ) => {
        pub type $bitmask_ty = $crate::util::VkFlags;
    };
}

/// Define a dispatchable handle.
macro_rules! vk_define_handle {
    ( $name:ident ) => {
        #[repr(C)]
        #[derive(Copy,Clone,PartialEq,Eq,Default,Debug)]
        pub struct $name ($crate::util::VkDispatchableHandle);
        impl $crate::util::VkNullHandle for $name  {
            const NULL : $name = $name($crate::util::VkDispatchableHandle::NULL);
        }
        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl ::std::fmt::Pointer for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:p}", self.0)
            }
        }
        impl ::std::fmt::LowerHex for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:x}", self.0)
            }
        }
        impl ::std::fmt::UpperHex for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:X}", self.0)
            }
        }
    };
}

/// Define a non-dispatchable handle.
macro_rules! vk_define_non_dispatchable_handle {
    ( $name:ident ) => {
        #[repr(C)]
        #[derive(Copy,Clone,PartialEq,Eq,Default,Debug)]
        pub struct $name ($crate::util::VkNonDispatchableHandle);
        impl $crate::util::VkNullHandle for $name {
            const NULL : $name = $name($crate::util::VkNonDispatchableHandle::NULL);
        }
        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl ::std::fmt::Pointer for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:p}", self.0)
            }
        }
        impl ::std::fmt::LowerHex for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:x}", self.0)
            }
        }
        impl ::std::fmt::UpperHex for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:X}", self.0)
            }
        }
    };
}

pub mod platform;
pub mod util;
pub mod cmds;

mod types; // generated!

pub mod prelude;



#[test]
fn test_type_sizes() {
    let ptr_size = ::std::mem::size_of::<extern "system" fn()>();
    let fnptr_size = ::std::mem::size_of::<extern "system" fn()>();

    assert_eq!(4, ::std::mem::size_of::<util::VkFlags>(), "check flag size");
    assert_eq!(4, ::std::mem::size_of::<types::VkColorComponentFlags>(), "check flag size");
    assert_eq!(4, ::std::mem::size_of::<types::VkResult>(), "check enum size");
    assert_eq!(ptr_size, ::std::mem::size_of::<types::VkDevice>(), "check dispatchable handle size");
    assert_eq!(8, ::std::mem::size_of::<types::VkImage>(), "check non-dispatchable handle size");
    assert_eq!(fnptr_size, ::std::mem::size_of::<types::PFN_vkVoidFunction>(), "check function pointer size");
}
