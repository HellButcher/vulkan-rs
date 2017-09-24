use registry::*;
use case::CaseStyle::ScreamingSnakeCase;
use std::ascii::AsciiExt;
use super::*;
use super::rust::*;
use std::io::Result;

pub struct TypesGenerator{
    pub style: CodeStyle,
}

impl CommonGeneratorWriter for TypesGenerator {

    fn write_header<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection) -> Result<()> {
        if !self.style.snake_case_fields {
            write!(w, "#![allow(non_snake_case)]\n\n")?;
        }
        write!(w, "use std::os::raw;\n")?;
        write!(w, "use util;\n")?;
        write!(w, "use platform::_all::*;\n\n")?;

        write!(w, "// API: {}\n\n", sel.api)?;

        Ok(())
    }

}

impl GeneratorWriter for TypesGenerator {

    fn write_constant_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &ConstantDefinition) -> Result<()> {
        if ty.base_type.type_name == "VK_MAKE_VERSION" {
            write_documentation(w, sel, f, ty)?;
            write_feature_protect(w, sel, f)?;
            write!(w, "pub const {} : u32 = vk_make_version!{};\n", ty.name, ty.value)?;
        } else if let Some((rust_value,rust_type)) = get_variant_value_and_type(sel, &ty.value, false) {
            write_documentation(w, sel, f, ty)?;
            write_feature_protect(w, sel, f)?;
            write!(w, "pub const {} : {} = {};\n", ty.name, rust_type, rust_value)?;
        }
        // don't write the other defines
        Ok(())
    }

    fn write_basetype_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &BasicDefinition) -> Result<()> {
        write_documentation(w, sel, f, ty)?;
        write_feature_protect(w, sel, f)?;
        write!(w, "pub type {} = {};\n", ty.name, type_ref(sel, &ty.base_type, false))?;
        Ok(())
    }

    fn write_bitmask_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &BasicDefinition) -> Result<()> {
        write_documentation(w, sel, f, ty)?;
        write_feature_protect(w, sel, f)?;
        if !ty.requires.is_empty() {
            write!(w, "vk_define_bitmask!({}, {});\n", ty.name, ty.requires)?;
        } else {
            write!(w, "vk_define_bitmask!({});\n", ty.name)?;
        }
        Ok(())
    }

    fn write_enum_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &BasicDefinition) -> Result<()> {
        let (name,_) = strip_vendor_suffix(ty.name(), sel);
        let prefix = ScreamingSnakeCase.apply_to_camel(name) + "_";

        write_documentation(w, sel, f, ty)?;
        write_feature_protect(w, sel, f)?;
        if ty.name == "VkResult" {
            write!(w, "#[must_use]\n")?;
        }
        write!(w, "#[repr(u32)]\n")?;
        write!(w, "#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]\n")?;
        write!(w, "pub enum {} {{\n", ty.name)?;
        let mut default : Option<String> = None;
        if let Some(items) = sel.enum_items_by_group.get(ty.name()) {
            w.indent(|w|{
                for item in items {
                    let stripped_item_name = strip_enum_name(&prefix, &item.name, sel);
                    if let Some(item_ext) = sel.feature_by_name.get(item.extension.as_str()) {
                        write_feature_protect(w, sel, item_ext)?;
                    }
                    if let Some(rust_value) = get_variant_value_for_enum(sel, &item.value) {
                        write!(w, "{} = {},\n", stripped_item_name, rust_value)?;
                        if default.is_none() && rust_value == "0" {
                            default = Some(stripped_item_name.clone())
                        }
                    } else {
                        write!(w, "{},\n", stripped_item_name)?;
                    }
                }
                Ok(())
            })?;
        }
        write!(w, "}}\n")?;
        if let Some(default) = default {
            write!(w, "impl Default for {} {{\n", ty.name)?;
            w.indent(|w|{
                write!(w, "fn default() -> {0} {{ {0}::{1} }}\n", ty.name, default)
            })?;
            write!(w, "}}\n")?;
        }
        Ok(())
    }

    fn write_enum_item_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, item: &EnumItemDefinition) -> Result<()> {
        if let Some(_) = sel.type_by_name.get(item.group.as_str()) {
            let (groupname,_) = strip_vendor_suffix(item.group.as_str(), sel);
            let prefix = ScreamingSnakeCase.apply_to_camel(groupname) + "_";
            let stripped_item_name = strip_enum_name(&prefix, &item.name, sel);
            write_feature_protect(w, sel, f)?;
            write!(w, "pub const {} : {} = {}::{};\n", item.name.to_ascii_uppercase(), item.group, item.group, stripped_item_name)?;
        } else {
            let default_use_usize = item.name.contains("_SIZE") || item.name.contains("_MAX");
            write_feature_protect(w, sel, f)?;
            if let Some((rust_value,rust_type)) = get_variant_value_and_type(sel, &item.value, default_use_usize) {
                write!(w, "pub const {} : {} = {};\n", item.name, rust_type, rust_value)?;
            } else {
                write!(w, "pub const {} : u32 = {};\n", item.name, item.value)?;
            }
        }
        Ok(())
    }

    fn write_handle_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &HandleDefinition) -> Result<()> {
        write_documentation(w, sel, f, ty)?;
        write_feature_protect(w, sel, f)?;
        if ty.dispatchable {
            write!(w, "vk_define_handle!({});\n", ty.name)?;
        } else {
            write!(w, "vk_define_non_dispatchable_handle!({});\n", ty.name)?;
        }
        Ok(())
    }

    fn write_funcpointer_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &CommandDefinition) -> Result<()> {
        let name = if ty.name.starts_with("PFN_") {
            ty.name.clone()
        } else {
            format!("PFN_{}", ty.name)
        };
        write_documentation(w, sel, f, ty)?;
        write_feature_protect(w, sel, f)?;
        write!(w, "#[allow(non_camel_case_types)]\n")?;
        write!(w, "pub type {} = extern \"system\" fn (", name)?;
        let mut first = true;
        for param in &ty.parameters {
            if first {
                first = false;
            } else {
                write!(w, ", ")?;
            }
            write!(w, "{}", type_ref(sel, &param.base_type, false))?;
        }
        write!(w, ")")?;
        if !ty.return_type.is_empty() {
            write!(w, " -> {}", return_type_ref(sel, &ty.return_type))?;
        }
        write!(w, ";\n")?;
        Ok(())
    }

    fn write_struct_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &StructDefinition) -> Result<()> {
        write_documentation(w, sel, f, ty)?;
        write_feature_protect(w, sel, f)?;
        write!(w, "#[repr(C)]\n")?;
        if let Some(_) = sel.get_struct_def_size(ty) {
            write!(w, "#[derive(Copy,Clone)]\n")?;
        }
        write!(w, "pub struct {} {{\n", ty.name)?;
        w.indent(|w|{
            for m in &ty.members {
                write_documentation(w, sel, f, m)?;
                write!(w, "pub {}: {},\n", self.style.field_name(&m.name), type_ref(sel, &m.base_type, false))?;
            }
            Ok(())
        })?;
        write!(w, "}}\n")?;
        write_feature_protect(w, sel, f)?;
        write!(w, "impl {} {{\n", ty.name)?;
        w.indent(|w|{
            write!(w, "#[inline]\n")?;
            write!(w, "pub fn new() -> {} {{\n", ty.name)?;
            w.indent(|w|{
                write!(w, "unsafe {{ ::std::mem::zeroed() }}\n")
            })?;
            write!(w, "}}\n")
        })?;
        write!(w, "}}\n")?;
        write_feature_protect(w, sel, f)?;
        write!(w, "impl Default for {} {{\n", ty.name)?;
        w.indent(|w|{
            write!(w, "#[inline]\n")?;
            write!(w, "fn default() -> {} {{\n", ty.name)?;
            w.indent(|w|{
                write!(w, "unsafe {{ ::std::mem::zeroed() }}\n")
            })?;
            write!(w, "}}\n")
        })?;
        write!(w, "}}\n")?;
        Ok(())
    }

    fn write_union_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &StructDefinition) -> Result<()> {
        write_documentation(w, sel, f, ty)?;
        write_feature_protect(w, sel, f)?;
        write!(w, "#[repr(C)]\n")?;
        if let Some(_) = sel.get_union_def_size(ty) {
            write!(w, "#[derive(Copy,Clone)]\n")?;
        }
        write!(w, "pub union {} {{\n", ty.name)?;
        w.indent(|w|{
            for m in &ty.members {
                write_documentation(w, sel, f, m)?;
                write!(w, "pub {}: {},\n", self.style.field_name(&m.name), type_ref(sel, &m.base_type, false))?;
            }
            Ok(())
        })?;
        write!(w, "}}\n")?;
        write!(w, "impl {} {{\n", ty.name)?;
        w.indent(|w|{
            write!(w, "#[inline]\n")?;
            write!(w, "pub fn new() -> {} {{ unsafe {{ ::std::mem::zeroed() }} }}\n", ty.name)?;
            for m in &ty.members {
                let member_name = self.style.field_name(&m.name);
                let member_type = type_ref(sel, &m.base_type, false);
                write_documentation(w, sel, f, m)?;
                write!(w, "#[inline]\n")?;
                write!(w, "pub fn from_{}(v: {}) -> {} {{\n", member_name, member_type, ty.name)?;
                w.indent(|w|{
                    write!(w, "{}{{ {}: v }}\n", ty.name, member_name)
                })?;
                write!(w, "}}\n")?;
            }
            Ok(())
        })?;
        write!(w, "}}\n")?;
        write_feature_protect(w, sel, f)?;
        write!(w, "impl Default for {} {{\n", ty.name)?;
        w.indent(|w|{
            write!(w, "#[inline]\n")?;
            write!(w, "fn default() -> {} {{\n", ty.name)?;
            w.indent(|w|{
                write!(w, "unsafe {{ ::std::mem::zeroed() }}\n")
            })?;
            write!(w, "}}\n")
        })?;
        write!(w, "}}\n")?;
        Ok(())
    }
}