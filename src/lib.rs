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

#[allow(const_err)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod platform;

macro_rules! vk_make_version {
    ( $major:expr, $minor:expr, $patch:expr ) => {
        (($major << 22) | ($minor << 12) | $patch)
     };
}

// utilities
pub mod util {
    pub type VkResultObj<T> = Result<T, ::types::VkResult>;

    pub use std::ptr::null_mut as vk_null;

    pub trait VkNullHandle: Sized {
        fn null() -> Self;
    }

    #[inline]
    pub fn vk_null_handle<T: VkNullHandle>() -> T {
        T::null()
    }
}

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod types;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod ffi;

pub use types::*;

pub mod vk {
    pub use types::VkEnum as Enum;
    pub use types::VkHandle as Handle;
    pub use types::VkDispatchableHandle as DispatchableHandle;
    pub use util::vk_null as null;
    pub use util::vk_null_handle as null_handle;
    pub use platform;

    include!(concat!(env!("OUT_DIR"), "/vulkan_alias.rs"));

}

pub mod safe;

pub mod prelude {
    pub use types::*;
    pub use ffi::*;
    pub use platform as vk_platform;
    pub use util::{vk_null, vk_null_handle};
    pub use util::VkResultObj;
}
