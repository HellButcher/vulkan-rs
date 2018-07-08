#[cfg(not(target_os = "windows"))]
mod dl_impl {
  use libc;
  use std::ffi::{CStr, CString, OsStr};
  use std::io::{Error, ErrorKind, Result};
  use std::os::unix::ffi::OsStrExt;
  use std::ptr;
  use types::PFN_vkVoidFunction;

  pub struct Library(*mut libc::c_void);

  pub unsafe fn open<S: AsRef<OsStr>>(name: &S) -> Result<Library> {
    let name = CString::new(name.as_ref().as_bytes()).unwrap();
    libc::dlerror(); // clear errors
    let handle = libc::dlopen(name.as_ptr(), libc::RTLD_LOCAL | libc::RTLD_LAZY);
    if handle == ptr::null_mut() {
      let msg_cstr = CStr::from_ptr(libc::dlerror());
      return Err(Error::new(ErrorKind::Other, msg_cstr.to_string_lossy().to_string()));
    }
    Ok(Library(handle))
  }

  impl Library {
    pub unsafe fn get(&self, name: &str) -> Result<Option<PFN_vkVoidFunction>> {
      let name = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
      libc::dlerror(); // clear errors
      let sym = libc::dlsym(self.0, name.as_ptr());
      if sym == ptr::null_mut() {
        let msg = libc::dlerror();
        if msg != ptr::null_mut() {
          let msg_cstr = CStr::from_ptr(msg);
          return Err(Error::new(ErrorKind::Other, msg_cstr.to_string_lossy().to_string()));
        }
      }
      Ok(::std::mem::transmute(sym))
    }
  }

}

#[cfg(target_os = "windows")]
mod dl_impl {
  use kernel32;
  use std::ffi::{CStr, OsStr};
  use std::io::{Error, Result};
  use std::os::raw;
  use std::os::windows::ffi::OsStrExt;
  use std::ptr;
  use types_raw::PFN_vkVoidFunction;
  use winapi;

  pub struct Library(winapi::HMODULE);

  pub unsafe fn open<S: AsRef<OsStr>>(name: &S) -> Result<Library> {
    let name_wide: Vec<u16> = name.as_ref().encode_wide().chain(Some(0)).collect();
    SetLastError(0); // clear errors
    let handle = kernel32::LoadLibraryW(name_wide.as_ptr());
    if handle == ptr::null_mut() {
      let err = kernel32::GetLastError();
      if err == 0 {
        return Err(Error::new(
          ErrorKind::Other,
          "LoadLibraryW didn't return any library handle",
        ));
      } else {
        return Err(IoError::from_raw_os_error(err as i32));
      }
    }
    Ok(Library(handle))
  }

  impl Library {
    pub unsafe fn get(&self, name: &str) -> Result<Option<PFN_vkVoidFunction>> {
      let mut v: Vec<u8> = Vec::new();
      let name = CStr::from_bytes_with_nul(name.bytes()).unwrap();
      SetLastError(0); // clear errors
      let sym = GetProcAddress(self.0, name.as_ptr());
      if sym == ptr::null_mut() {
        let err = kernel32::GetLastError();
        if err != 0 {
          return Err(IoError::from_raw_os_error(err as i32));
        }
      }
      Ok(::std::mem::transmute(sym))
    }
  }
}

pub use self::dl_impl::*;
