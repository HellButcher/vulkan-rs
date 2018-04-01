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
extern crate rustc_version;
extern crate shaderc;

use std::env;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

use rustc_version::{version_meta, Channel};

#[cfg(all(feature = "VK_USE_PLATFORM_DEFAULT", target_os = "windows"))]
fn enable_default_platform() {
  println!("cargo:rustc-cfg=feature=\"VK_USE_PLATFORM_WIN32_KHR\"");
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

const NUM_SHADER_TYPES: usize = 6;
const SHADER_EXTS: [&'static str; NUM_SHADER_TYPES] = ["comp", "frag", "geom", "tesc", "tese", "vert"]; // sort!
const SHADER_TYPES: [shaderc::ShaderKind; NUM_SHADER_TYPES] = {
  use shaderc::ShaderKind::*;
  [
    DefaultCompute,
    DefaultFragment,
    DefaultGeometry,
    DefaultTessControl,
    DefaultTessEvaluation,
    DefaultVertex,
  ]
};

#[allow(unused)]
fn compile_sharers() {
  let mut compiler = shaderc::Compiler::new().unwrap();

  let out_dir = env::var("OUT_DIR").unwrap();
  let out_dir = Path::new(&out_dir);
  let shader_path = Path::new("examples");
  for src_path in shader_path
    .read_dir()
    .unwrap()
    .map(|p| p.unwrap().path())
    .filter(|p| p.is_file())
  {
    let file_ext = src_path.extension().unwrap();
    let shader_type = match SHADER_EXTS.binary_search(&file_ext.to_str().unwrap()) {
      Err(_) => {
        continue;
      }
      Ok(i) => SHADER_TYPES[i],
    };

    println!("cargo:rerun-if-changed={}", src_path.to_str().unwrap());
    let rel_path = src_path.strip_prefix(&shader_path).unwrap();
    let out_path = out_dir
      .join(rel_path)
      .with_extension(format!("{}.spv", file_ext.to_str().unwrap()));
    ::std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();

    // read the filte
    let mut source = String::new();
    {
      File::open(&src_path)
        .expect("open source")
        .read_to_string(&mut source)
        .expect("reading source");
    }
    let binary = compiler
      .compile_into_spirv(
        &source,
        shader_type,
        src_path.to_str().unwrap(),
        "main",
        None,
      )
      .expect("compile");

    {
      let bytes = unsafe { ::std::slice::from_raw_parts(binary.as_binary().as_ptr() as *const u8, binary.len()) };
      File::create(&out_path)
        .expect("create file")
        .write(bytes)
        .expect("write file");
    }
  }
}

fn main() {
  // enable "nightly" if building in nightly
  match version_meta().unwrap().channel {
    Channel::Nightly | Channel::Dev => {
      println!("cargo:rustc-cfg=nightly");
    }
    _ => {}
  }

  println!("cargo:rerun-if-changed=build.rs");
  enable_default_platform();
  //compile_sharers();
}
