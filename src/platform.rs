#[doc(no_inline)]
pub use std::os::raw::{c_char, c_int, c_void};

#[doc(no_inline)]
pub use std::ffi::CStr;

#[doc(no_inline)]
pub use utils::vk_make_version;

pub mod wsi {

  #[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
  pub mod xlib {

    #[doc(hidden)]
    pub enum Display__ {}
    pub type Display = Display__;

    pub type Window = u32;
    pub type VisualID = u32;
    pub type RROutput = u32;
  }

  #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
  pub mod xcb {
    #![allow(non_camel_case_types)]

    #[doc(hidden)]
    pub enum xcb_connection_t__ {}
    pub type xcb_connection_t = xcb_connection_t__;

    pub type xcb_visualid_t = u32;
    pub type xcb_window_t = u32;
  }

  #[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
  pub mod android {

    #[doc(hidden)]
    pub enum ANativeWindow__ {}
    pub type ANativeWindow = ANativeWindow__;
  }

  #[cfg(feature = "VK_USE_PLATFORM_MIR_KHR")]
  pub mod mir {

    #[doc(hidden)]
    pub enum MirConnection__ {}
    pub type MirConnection = MirConnection__;

    #[doc(hidden)]
    pub enum MirSurface__ {}
    pub type MirSurface = MirSurface__;
  }

  #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
  pub mod wayland {
    #![allow(non_camel_case_types)]

    #[doc(hidden)]
    pub enum wl_display__ {}
    pub type wl_display = wl_display__;

    #[doc(hidden)]
    pub enum wl_surface__ {}
    pub type wl_surface = wl_surface__;
  }

  #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
  pub mod win32 {
    #![allow(non_camel_case_types)]

    use std::os::raw::c_void;

    pub type BOOL = i32;
    pub type DWORD = u32;
    pub type WCHAR = u16;
    pub type LPVOID = *mut c_void;
    pub type LPCWSTR = *const WCHAR;

    pub type HANDLE = *mut c_void;

    #[doc(hidden)]
    pub enum HINSTANCE__ {}
    pub type HINSTANCE = *mut HINSTANCE__;

    #[doc(hidden)]
    pub enum HWND__ {}
    pub type HWND = *mut HWND__;

    #[doc(hidden)]
    pub struct SECURITY_ATTRIBUTES__ {
      pub nLength: DWORD,
      pub lpSecurityDescriptor: LPVOID,
      pub bInheritHandle: BOOL,
    }
    pub type SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES__;
  }
}
