use registry::*;
use std::ascii::AsciiExt;
use case::CaseStyle::SnakeCase;
use super::*;
use super::rust::*;
use std::io::Result;

pub struct AliasGenerator{
    pub strip_api_prefix: bool,
    pub use_feature_modules: bool,
    pub snake_case_commands: bool,
}

impl AliasGenerator {

    fn strip_vk_prefix<'l>(&self, s: &'l str) -> &'l str {
        if s.starts_with("VK_") || s.starts_with("vk_") {
            return &s[3..];
        } else if s.starts_with("Vk") || s.starts_with("vk") {
            let s = &s[2..];
            if s.starts_with(|c| 'a'<=c && c<= 'z') {
                return s;
            }
        }
        return s;
    }

    fn cmd_case(&self, name: &str) -> String {
        if self.snake_case_commands {
            SnakeCase.apply_to_camel(name)
        } else {
            name.to_owned()
        }
    }

    fn write_type_alias<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, name: &str) -> Result<()> {
        self.write_feature_protect(w, sel, f)?;
        if self.strip_api_prefix {
            let name = self.strip_vk_prefix(name);
            write!(w, "pub use types::{} as {};\n", name, name)?;
        } else {
            write!(w, "pub use types::{};\n", name)?;
        }
        Ok(())
    }

    fn write_command_alias<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, name: &str) -> Result<()> {
        self.write_feature_protect(w, sel, f)?;
        if self.strip_api_prefix {
            write!(w, "pub use cmds::{} as {};\n", name, self.cmd_case(self.strip_vk_prefix(name)))?;
        } else {
            write!(w, "pub use cmds::{};\n", self.cmd_case(name))?;
        }
        Ok(())
    }

    fn write_feature_protect<W: CodeWrite>(&self, w: &mut W, sel: &Selection, f: &FeatureSet) -> Result<()> {
        if !self.use_feature_modules {
            write_feature_protect(w, sel, f)?;
        }
        Ok(())
    }
}


impl CommonGeneratorWriter for AliasGenerator {

    fn write_header<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection) -> Result<()> {
        write!(w, "// API: {}\n\n", sel.api)?;

        if self.use_feature_modules {
            write!(w, "mod common {{\n")?;
            w.push_indent();
        }
        if self.strip_api_prefix {
            write!(w, "pub use util::vk_null as null;\n")?;
            write!(w, "pub use util::vk_null_handle as null_handle;\n")?;
            write!(w, "pub use util::VkResultObj as ResultObj;\n")?;
            write!(w, "pub use util::VkNullHandle as NullHandle;\n")?;
            write!(w, "pub use util::VkError as Error;\n")?;
            write!(w, "pub use util::VkVersion as Version;\n")?;
            write!(w, "pub use platform;\n")?;
        } else {
            write!(w, "pub use util::{{vk_null, vk_null_handle, VkNullHandle, VkResultObj, VkError, VkVersion}};\n")?;
            write!(w, "pub use platform as vk_platform;\n")?;
        }
        if self.use_feature_modules {
            w.pop_indent();
            write!(w, "}}\n")?;
        }

        Ok(())
    }

    fn write_end_file<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection) -> Result<()> {
        if self.use_feature_modules {
            write!(w, "pub mod all {{\n")?;
            w.indent(|w|{
                write!(w, "pub use super::common::*;\n")?;
                for f in &sel.features {
                    write_feature_protect(w, sel, f)?;
                    write!(w, "pub use super::{}::*;\n", f.name.to_ascii_lowercase())?;
                }
                Ok(())
            })?;
            write!(w, "}}\n\n")?;
        }
        Ok(())
    }
}

impl GeneratorWriter for AliasGenerator {


    fn write_begin_feature<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet) -> Result<()> {
        if self.use_feature_modules {
            write_feature_protect(w, sel, f)?;
            write!(w, "pub mod {} {{\n", f.name.to_ascii_lowercase())?;
            w.push_indent();
            write!(w, "pub use super::common::*;\n")?;
        } else {
            write!(w, "\n// {}\n\n", f.name)?;
        }
        Ok(())
    }

    fn write_end_feature<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, _: &FeatureSet) -> Result<()> {
        if self.use_feature_modules {
            w.pop_indent();
            write!(w, "}}\n\n")?;
        }
        Ok(())
    }

    fn write_constant_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &ConstantDefinition) -> Result<()> {
        if ty.base_type.type_name == "VK_MAKE_VERSION" {
            self.write_type_alias(w, sel, f, &ty.name)?;
        } else if let Some(_) = get_variant_value_and_type(sel, &ty.value, false) {
            self.write_type_alias(w, sel, f, &ty.name)?;
        }
        // don't write the other defines
        Ok(())
    }

    fn write_basetype_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &BasicDefinition) -> Result<()> {
        self.write_type_alias(w, sel, f, &ty.name)
    }

    fn write_bitmask_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &BasicDefinition) -> Result<()> {
        self.write_type_alias(w, sel, f, &ty.name)
    }

    fn write_enum_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &BasicDefinition) -> Result<()> {
        self.write_type_alias(w, sel, f, &ty.name)
    }

    fn write_enum_item_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, item: &EnumItemDefinition) -> Result<()> {
        let upper_name = item.name.to_ascii_uppercase();
        self.write_type_alias(w, sel, f, &upper_name)
    }

    fn write_handle_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &HandleDefinition) -> Result<()> {
        self.write_type_alias(w, sel, f, &ty.name)
    }

    fn write_funcpointer_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &CommandDefinition) -> Result<()> {
        self.write_type_alias(w, sel, f, &ty.name)
    }

    fn write_struct_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &StructDefinition) -> Result<()> {
        self.write_type_alias(w, sel, f, &ty.name)
    }

    fn write_union_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &StructDefinition) -> Result<()> {
        self.write_type_alias(w, sel, f, &ty.name)
    }

    fn write_command_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, cmd: &CommandDefinition) -> Result<()> {
        self.write_command_alias(w, sel, f, &cmd.name)
    }
}
