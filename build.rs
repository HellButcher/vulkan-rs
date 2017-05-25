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

#[cfg(all(feature = "VK_USE_PLATFORM_DEFAULT", target_os = "windows"))]
fn enable_default_platform() {
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_WIN32_KHR\"");
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_WIN32_KHX\"");
}

#[cfg(all(feature = "VK_USE_PLATFORM_DEFAULT", target_os = "linux"))]
fn enable_default_platform() {
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_XLIB_KHR\"");
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_XLIB_XRANDR_EXT\"");
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_XCB_KHR\"");
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_WAYLAND_KHR\"");
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_MIR_KHR\"");
}

#[cfg(all(feature = "VK_USE_PLATFORM_DEFAULT", target_os = "android"))]
fn enable_default_platform() {
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_ANDROID_KHR\"");
}

#[cfg(all(feature = "VK_USE_PLATFORM_DEFAULT", target_os = "ios"))]
fn enable_default_platform() {
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_IOS_MVK\"");
}

#[cfg(all(feature = "VK_USE_PLATFORM_DEFAULT", target_os = "macos"))]
fn enable_default_platform() {
    println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_MACOS_MVK\"");
}

#[cfg(not(feature = "VK_USE_PLATFORM_DEFAULT"))]
fn enable_default_platform() {}

fn main() {
    let generator_path = ::std::path::Path::new("tools").join("generator");
    let genvk_py_path = generator_path.join("genvk.py");
    let vkdoc_path = generator_path.join("Vulkan-Docs");

    let out_dir = ::std::env::var("OUT_DIR").unwrap();

    for path in generator_path
            .read_dir()
            .unwrap()
            .map(|p| p.unwrap().path())
            .filter(|p| p.is_file() && p.extension().unwrap() == "py") {
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    enable_default_platform();

    if !vkdoc_path.exists() {
        let status = ::std::process::Command::new("git")
            .arg("clone")
            .arg("https://github.com/KhronosGroup/Vulkan-Docs")
            .arg("Vulkan-Docs")
            .current_dir(generator_path)
            .status()
            .unwrap();
        if !status.success() {
            panic!("`git clone [...]/Vulkan-Docs` exited with status code {}",
                   status)
        }
    }

    let status = ::std::process::Command::new("python3")
        .arg(genvk_py_path)
        .arg("-o")
        .arg(out_dir)
        .arg("vulkan_types.rs")
        .arg("vulkan_ffi.rs")
        .arg("vulkan_safe.rs")
        .arg("vulkan_alias.rs")
        .arg("vulkan_utils.rs")
        .status()
        .unwrap();
    if !status.success() {
        panic!("`genvk.py vulkan.rs` exited with status code {}", status)
    }
}
