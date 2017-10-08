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
extern crate shaderc;

use std::env;
use std::io::{Read,Write};
use std::fs::File;
use std::path::Path;
use std::process::Command;

extern crate vulkan_rs_generator;

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

fn generate_vulkan_bindings() {
    use vulkan_rs_generator::registry;
    use vulkan_rs_generator::generator::rust as rustgen;

    let vkdoc_path = Path::new("vulkan_spec").join("Vulkan-Docs");
    let registry_path = vkdoc_path.join("src/spec/vk.xml");

    //let out_dir = env::var("OUT_DIR").unwrap();
    //let out_dir = Path::new(&out_dir);
    let out_dir = Path::new("src");

    if !vkdoc_path.exists() {
        let status = Command::new("git")
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .arg(vkdoc_path)
            .status()
            .expect("git submodule update");
        if !status.success() {
            panic!("`git submodule update init [...]` exited with status code {}", status)
        }
    }

    println!("cargo:rerun-if-changed={}", registry_path.to_str().unwrap());

    let registry_file = File::open(registry_path).expect("opening reg");
    let registry = registry::RegistryData::read(registry_file).expect("reading reg");
    let registry = registry::Registry::new(&registry);

    println!("registry: num-types: {}", registry.types.len());
    println!("registry: num-enums: {}", registry.enum_groups.len());
    println!("registry: num-cmds: {}", registry.commands.len());
    println!("registry: num-enum_extensions: {}", registry.enum_extensions.len());

    let selection = {
        let mut s = registry::Selection::new(&registry);
        s.ignore_feature(registry::SelectionNameRef::Type("VK_NULL_HANDLE"));
        for f in &registry.features {
            s.select_feature_set(f).expect("selecting feature");
        }
        for e in &registry.extensions {
            s.select_feature_set(e).expect("selecting extension");
        }
        s
    };

    println!("selection: num-selections: {}", selection.selected_set.len());
    println!("selection: num-features: {}", selection.features.len());
    let style = rustgen::CodeStyle {
        snake_case_commands: false,
        snake_case_fields: false,
    };
    {
        let mut out_file = File::create(out_dir.join("types.rs")).expect("create types.rs");
        let mut gen = rustgen::types::TypesGenerator{ style };
        selection.generate(&mut gen, &mut out_file).expect("generate types");
    }
    {
        let mut out_file = File::create(out_dir.join("prelude.rs")).expect("create prelude.rs");
        let mut gen = rustgen::alias::AliasGenerator{
            strip_api_prefix: false,
            use_feature_modules: true,
            snake_case_commands: false,
        };
        selection.generate(&mut gen, &mut out_file).expect("generate prelude");
    }
    {
        let tables = rustgen::cmds::DispatchTablePreprocessor::new(&selection).expect("preprocess table");
        let mut out_file = File::create(out_dir.join("cmds/table.rs")).expect("create cmds/table.rs");
        tables.generate(&mut rustgen::cmds::DispatchTableWriter::new(), &mut out_file).expect("generate dispatch table");
        tables.generate(&mut rustgen::cmds::DispatchTableImplWriter::new(), &mut out_file).expect("generate dispatch table impl");
    }
    {
        let mut out_file = File::create(out_dir.join("cmds/dispatch.rs")).expect("create cmd/dispatch.rs");
        let mut gen = rustgen::cmds::DispatchCommandImplWriter{ style };
        selection.generate(&mut gen, &mut out_file).expect("generate dispatch cmd impl");
    }
    {
        let mut out_file = File::create(out_dir.join("cmds/safe.rs")).expect("create cmd/safe.rs");
        let mut gen = rustgen::cmds::SafeCommandImplWriter{ style };
        selection.generate(&mut gen, &mut out_file).expect("generate safe cmd impl");
    }
    // {
    //     let mut out_file = File::create(out_dir.join("vulkan_alias.rs")).expect("create vk.rs");
    //     let mut gen = generator::rust_alias::AliasGenerator{
    //         strip_api_prefix: true,
    //         use_feature_modules: false,
    //     };
    //     selection.generate(&mut generator::OututGenerator::new(&mut gen, &mut out_file)).expect("generate bindings");
    // }
}



const NUM_SHADER_TYPES : usize = 6;
const SHADER_EXTS : [&'static str;NUM_SHADER_TYPES] = ["comp", "frag", "geom", "tesc", "tese", "vert"]; // sort!
const SHADER_TYPES: [shaderc::ShaderKind;NUM_SHADER_TYPES] = {
    use shaderc::ShaderKind::*;
    [DefaultCompute, DefaultFragment, DefaultGeometry, DefaultTessControl, DefaultTessEvaluation, DefaultVertex]
};

fn compile_sharers() {
    let mut compiler = shaderc::Compiler::new().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let shader_path = Path::new("examples");
    for src_path in shader_path.read_dir()
            .unwrap()
            .map(|p| p.unwrap().path())
            .filter(|p| p.is_file()) {
        let file_ext = src_path.extension().unwrap();
        let shader_type = match SHADER_EXTS.binary_search(&file_ext.to_str().unwrap()) {
            Err(_) => { continue; },
            Ok(i) => SHADER_TYPES[i],
        };

        println!("cargo:rerun-if-changed={}", src_path.to_str().unwrap());
        let rel_path = src_path.strip_prefix(&shader_path).unwrap();
        let out_path = out_dir.join(rel_path).with_extension(format!("{}.spv", file_ext.to_str().unwrap()));
        ::std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();

        // read the filte
        let mut source = String::new();
        {
            File::open(&src_path).expect("open source")
                .read_to_string(&mut source).expect("reading source");
        }
        let binary = compiler.compile_into_spirv(
            &source,
            shader_type,
            src_path.to_str().unwrap(),
            "main",
            None
        ).expect("compile");

        {
            let bytes = unsafe { ::std::slice::from_raw_parts(binary.as_binary().as_ptr() as *const u8, binary.len()) };
            File::create(&out_path).expect("create file")
                .write(bytes).expect("write file");
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    enable_default_platform();
    generate_vulkan_bindings();
    compile_sharers();
}
