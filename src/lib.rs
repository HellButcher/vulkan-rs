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
#[allow(non_snake_case)]
pub mod util {
    use std::fmt;
    use std::error;

    #[derive(Copy,Clone,PartialEq,Eq)]
    pub struct VkError (::types::VkResult);

    impl VkError {
        #[inline]
        pub fn is_success(self) -> bool{
            return (self.0 as i32) >= 0;
        }
        #[inline]
        pub fn is_error(self) -> bool {
            return (self.0 as i32) < 0;
        }
        #[inline]
        pub fn name(self) -> &'static str {
            return get_VkResult_name(self.0);
        }
        #[inline]
        pub fn description(self) -> &'static str {
            return get_VkResult_description(self.0);
        }
    }

    impl From<::types::VkResult> for VkError {
        #[inline]
        fn from(value: ::types::VkResult) -> VkError {
            return VkError(value);
        }
    }

    impl fmt::Debug for VkError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let name = self.name();
            if name.len() > 0 {
                write!(f, "{}", name)
            } else {
                write!(f, "VkResult({})", self.0)
            }
        }
    }

    impl fmt::Display for VkError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let name = self.name();
            if name.len() > 0 {
                write!(f, "{}", name)
            } else {
                write!(f, "VkResult({})", self.0)
            }
        }
    }

    impl error::Error for VkError {
        #[inline]
        fn description(&self) -> &str {
            return get_VkResult_description(self.0);
        }
    }

    pub type VkResultObj<T=::types::VkResult> = Result<T, VkError>;

    pub use std::ptr::null_mut as vk_null;

    pub trait VkNullHandle: Sized {
        fn null() -> Self;
    }

    #[inline]
    pub fn vk_null_handle<T: VkNullHandle>() -> T {
        T::null()
    }

    include!(concat!(env!("OUT_DIR"), "/vulkan_utils.rs"));
}

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod types;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod ffi;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod safe;

pub use types::*;

pub mod vk {
    use ffi as cmds;
    use types;
    pub use types::VkEnum as Enum;
    pub use types::VkHandle as Handle;
    pub use types::VkNonDispatchableHandle as NonDispatchableHandle;
    pub use util::vk_null as null;
    pub use util::vk_null_handle as null_handle;
    pub use util::VkResultObj as ResultObj;
    pub use util::VkError as Error;
    pub use platform;

    include!(concat!(env!("OUT_DIR"), "/vulkan_alias.rs"));

}

pub mod prelude {
    pub use types::*;
    pub use ffi::*;
    pub use platform as vk_platform;
    pub use util::{vk_null, vk_null_handle};
    pub use util::VkResultObj;
    pub use util::VkError;
}
