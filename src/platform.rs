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

//! Types used by the window-system-interface.

#[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
pub mod xlib {
    use std::os::raw::c_void;

    pub type Display = c_void;
    pub type Window = u32;
    pub type VisualID = u32;
    pub type RROutput = u32;
}
#[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
pub mod xcb {
    use std::os::raw::c_void;

    pub type xcb_connection_t = c_void;
    pub type xcb_visualid_t = u32;
    pub type xcb_window_t = u32;
}
#[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
pub mod android {
    use std::os::raw::c_void;

    pub type ANativeWindow = c_void;
}
#[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
pub mod mir {
    use std::os::raw::c_void;

    pub type MirConnection = c_void;
    pub type MirSurface = c_void;
}
#[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
pub mod wayland {
    use std::os::raw::c_void;

    pub type wl_display = c_void;
    pub type wl_surface = c_void;
}
#[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
pub mod win32 {
    use std::os::raw::c_void;

    pub type HANDLE = usize;
    pub type HINSTANCE = HANDLE;
    pub type HWND = HANDLE;
    pub type SECURITY_ATTRIBUTES = c_void;
    pub type DWORD = u32;
    pub type LPCWSTR = *const u16;
}
pub mod _all {
    #[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
    pub use platform::xlib::*;
    #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
    pub use platform::xcb::*;
    #[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
    pub use platform::android::*;
    #[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
    pub use platform::mir::*;
    #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
    pub use platform::wayland::*;
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    pub use platform::win32::*;
}
