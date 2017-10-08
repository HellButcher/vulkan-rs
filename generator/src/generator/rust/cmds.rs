use registry::*;
use std::collections::BTreeMap;
use super::super::*;
use super::*;
use std::io::Result;

pub struct DispatchTablePreprocessor<'r>{
    selection: &'r Selection<'r>,
    commands_by_table: BTreeMap<&'static str, Vec<(String, String)>>,
}

pub trait DispatchGeneratorWriter: CommonGeneratorWriter {

    fn write_begin_table<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &str) -> Result<()> {
        Ok(())
    }
    fn write_end_table<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &str) -> Result<()> {
        Ok(())
    }

    fn write_begin_feature<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, _: &str, f: &FeatureSet) -> Result<()> {
        write!(w, "// {}\n", f.name)?;
        Ok(())
    }

    fn write_end_feature<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &str, _: &FeatureSet) -> Result<()> {
        Ok(())
    }

    fn write_command_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &str, _: &FeatureSet, _: &CommandDefinition) -> Result<()>;

}

impl<'r> DispatchTablePreprocessor<'r> {
    pub fn new(sel: &'r Selection) -> Result<DispatchTablePreprocessor<'r>> {
        let mut t = DispatchTablePreprocessor {
            selection: sel,
            commands_by_table: BTreeMap::new(),
        };
        sel.visit(&mut t)?;
        Ok(t)
    }

    pub fn generate<G: DispatchGeneratorWriter, W: Write>(&self, gen: &mut G, w: &mut W) -> Result<()> {
        self.generate_code(gen, &mut CodeWriteWrapper::new(w))
    }

    pub fn generate_code<G: DispatchGeneratorWriter, W: CodeWrite>(&self, gen: &mut G, w: &mut W) -> Result<()> {
        gen.write_begin_file(w, self.selection)?;
        for (table_name, cmds) in &self.commands_by_table {
            gen.write_begin_table(w, self.selection, table_name)?;
            let mut current_feat : Option<&FeatureSet> = None;
            for &(ref feat, ref cmd) in cmds {
                let feat = match current_feat {
                    Some(s) if &s.name==feat => {
                        s
                    },
                    old_feat => {
                        if let Some(old_feat) = old_feat {
                            gen.write_end_feature(w, self.selection, table_name, old_feat)?;
                        }
                        if let Some(new_feat) = self.selection.feature_by_name.get(feat.as_str()) {
                            gen.write_begin_feature(w, self.selection, table_name, new_feat)?;
                            current_feat = Some(new_feat);
                            new_feat
                        } else {
                            return Err(io::Error::new(io::ErrorKind::Other, format!("unable to resolve feature {}", feat)));
                        }
                    }
                };
                if let Some(cmd) = self.selection.command_by_name.get(cmd.as_str()) {
                    gen.write_command_definition(w, self.selection, table_name, feat, cmd)?;
                } else {
                    return Err(io::Error::new(io::ErrorKind::Other, format!("unable to resole command {}", cmd)));
                }
            }
            if let Some(old_feat) = current_feat {
                gen.write_end_feature(w, self.selection, table_name, old_feat)?;
            }
            gen.write_end_table(w, self.selection, table_name)?;
        }
        gen.write_end_file(w, self.selection)
    }
}

impl<'r> Visitor for DispatchTablePreprocessor<'r> {
    fn visit_command_definition(&mut self, sel: &Selection, f: &SelectionFeatureSet, cmd: &CommandDefinition) -> Result<()> {
        let dispatch_tab = get_dispatch_table_for_command(cmd, sel)?;
        self.commands_by_table.entry(dispatch_tab).or_insert(Vec::new()).push((f.name.clone(), cmd.name.clone()));
        Ok(())
    }
}

pub struct DispatchTableWriter();

impl DispatchTableWriter {
    pub fn new() -> DispatchTableWriter {
        DispatchTableWriter()
    }
}

impl CommonGeneratorWriter for DispatchTableWriter {
    fn write_header<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection) -> Result<()> {
        write!(w, "use std::os::raw;\n")?;
        write!(w, "use platform::_all::*;\n\n")?;
        write!(w, "use types::*;\n\n")?;

        write!(w, "// API: {}\n\n", sel.api)?;

        Ok(())
    }
}

impl DispatchGeneratorWriter for DispatchTableWriter {

    fn write_begin_table<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, table_name: &str) -> Result<()> {
        write!(w, "#[allow(non_snake_case)]\n")?;
        write!(w, "pub struct {}Table {{\n", table_name)?;
        w.push_indent();
        Ok(())
    }

    fn write_end_table<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, _: &str) -> Result<()> {
        w.pop_indent();
        write!(w, "}}\n")?;
        Ok(())
    }

    fn write_command_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, _: &str, f: &FeatureSet, cmd: &CommandDefinition) -> Result<()> {
        write_feature_protect(w, sel, f)?;
        write!(w, "pub {}: extern \"system\" fn (", cmd.name)?;
        let mut first = true;
        for param in &cmd.parameters {
            if first {
                first = false;
            } else {
                write!(w, ", ")?;
            }
            write!(w, "{}", def_type_ref(sel, param, 0))?;
        }
        write!(w, ")")?;
        if !cmd.return_type.is_empty() {
            write!(w, " -> {}", return_type_ref(sel, &cmd.return_type, 0))?;
        }
        write!(w, ",\n")?;
        Ok(())
    }
}

pub struct DispatchTableImplWriter{
    is_in_protect: bool,
}

impl DispatchTableImplWriter {
    pub fn new() -> DispatchTableImplWriter {
        DispatchTableImplWriter{
            is_in_protect: false,
        }
    }
}

impl CommonGeneratorWriter for DispatchTableImplWriter {
    fn write_begin_file<W: CodeWrite>(&mut self, w: &mut W, _: &Selection) -> Result<()> {
        write!(w, "use std::mem::transmute;\n\n")?;
        write!(w, "use util::VkResultObj;\n\n")?;
        Ok(())
    }
}

impl DispatchGeneratorWriter for DispatchTableImplWriter {

    fn write_begin_table<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, table_name: &str) -> Result<()> {
        write!(w, "impl {}Table {{\n", table_name)?;
        w.push_indent();
        if table_name == "VkLoader" {
            write!(w, "pub unsafe fn load<F>(&mut self, get_proc_addr: F) -> VkResultObj\n")?;
            write!(w, "    where F: Fn(&str) -> VkResultObj<PFN_vkVoidFunction>,\n")?;
        } else {
            write!(w, "pub unsafe fn load<F,E>(&mut self, get_proc_addr: F, is_extension_enabled: E) -> VkResultObj\n")?;
            write!(w, "    where F: Fn(&str) -> VkResultObj<PFN_vkVoidFunction>,\n")?;
            write!(w, "          E: Fn(&str) -> bool,\n")?;
        }
        write!(w, "{{\n")?;
        w.push_indent();
        Ok(())
    }

    fn write_end_table<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, _: &str) -> Result<()> {
        write!(w, "Ok(VK_SUCCESS)\n")?;
        w.pop_indent();
        write!(w, "}}\n")?;
        w.pop_indent();
        write!(w, "}}\n")?;
        Ok(())
    }


    fn write_begin_feature<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, _: &str, f: &FeatureSet) -> Result<()> {
        write!(w, "// {}\n", f.name)?;
        self.is_in_protect = write_feature_protect(w, sel, f)?;
        if self.is_in_protect {
            write!(w, "{{\n")?;
            w.push_indent();
        }
        if f.is_extension {
            write!(w, "if is_extension_enabled(\"{}\") {{\n", f.name)?;
            w.push_indent();
        }
        Ok(())
    }

    fn write_end_feature<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, _: &str, f: &FeatureSet) -> Result<()> {
        if f.is_extension {
            w.pop_indent();
            write!(w, "}}\n")?;
        }
        if self.is_in_protect {
            w.pop_indent();
            write!(w, "}}\n")?;
            self.is_in_protect = false;
        }
        Ok(())
    }

    fn write_command_definition<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, _: &str, _: &FeatureSet, cmd: &CommandDefinition) -> Result<()> {
        write!(w, "self.{} = transmute(get_proc_addr(\"{}\")?);\n", cmd.name, cmd.name)?;
        Ok(())
    }
}


pub struct DispatchCommandImplWriter {
    pub style: CodeStyle,
}

impl CommonGeneratorWriter for DispatchCommandImplWriter {
    fn write_header<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection) -> Result<()> {
        if !self.style.snake_case_commands {
            write!(w, "#![allow(non_snake_case)]\n\n")?;
        }
        write!(w, "use std::os::raw;\n")?;
        write!(w, "use super::table;\n")?;
        write!(w, "use types::*;\n")?;
        write!(w, "use platform::_all::*;\n\n")?;

        write!(w, "// API: {}\n\n", sel.api)?;

        Ok(())
    }
}

fn generate_dispatch_call<W: CodeWrite,F>(w: &mut W, sel: &Selection, _: &FeatureSet, cmd: &CommandDefinition, get_param_name: F) -> Result<()>
    where F: Fn(&ParameterDefinition) -> String
{
    let dispatch_tab = get_dispatch_table_for_command(cmd, sel)?;
    write!(w, "if let Some(_dt) = table::{}Table::get() {{\n", dispatch_tab)?;
    w.indent(|w|{
        write!(w, "(_dt.{})(", cmd.name)?;
        for (i, param) in (&cmd.parameters).into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            write!(w, "{}", get_param_name(param))?;
        }
        write!(w, ")\n")?;
        Ok(())
    })?;
    if let Some(errorvalue) = cmd.returns_status() {
        write!(w, "}} else {{\n")?;
        w.indent(|w|{
            write!(w, "{}\n", errorvalue)
        })?;
    }
    write!(w, "}}")
}

impl GeneratorWriter for DispatchCommandImplWriter {
    fn write_command_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, cmd: &CommandDefinition) -> Result<()> {
        let cmd_name = self.style.command_name(&cmd.name);

        write_documentation(w,sel, f, cmd)?;
        write_feature_protect(w, sel, f)?;
        write!(w, "#[inline]\n")?;
        write!(w, "pub unsafe fn {} (", cmd_name)?;
        for (i, param) in (&cmd.parameters).into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            write!(w, "{}: {}", self.style.param_name(&param.name), def_type_ref(sel, param, 0))?;
        }
        write!(w, ")")?;
        if !cmd.return_type.is_empty() {
            write!(w, " -> {}", return_type_ref(sel, &cmd.return_type, 0))?;
        }
        write!(w, " {{\n")?;
        w.indent(|w|{
            generate_dispatch_call(w, sel, f, cmd, |p| self.style.param_name(&p.name))?;
            write!(w, "\n")
        })?;
        write!(w, "}}\n")?;
        Ok(())
    }
}



pub struct SafeCommandImplWriter {
    pub style: CodeStyle,
}

impl CommonGeneratorWriter for SafeCommandImplWriter {
    fn write_header<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection) -> Result<()> {
        if !self.style.snake_case_commands {
            write!(w, "#![allow(non_snake_case)]\n\n")?;
        }
        write!(w, "use std::os::raw;\n")?;
        write!(w, "use std::ptr;\n")?;
        write!(w, "use std::mem;\n")?;
        write!(w, "use std::ffi::CString;\n")?;
        write!(w, "use super::ffi;\n")?;
        write!(w, "use types::*;\n")?;
        write!(w, "use platform::_all::*;\n\n")?;
        write!(w, "use util::VkResultObj;\n\n")?;

        write!(w, "// API: {}\n\n", sel.api)?;

        Ok(())
    }
}


impl GeneratorWriter for SafeCommandImplWriter {
    fn write_command_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, cmd: &CommandDefinition) -> Result<()> {
        let cmd_name = self.style.command_name(&cmd.name);
        let type_format_flags;
        let lifetime;
        match get_return_param(sel, cmd) {
            Some(p) if (sel.get_type_ref_flags(&p.base_type) & TF_CONTAINS_HANDLE) != 0 => {
                type_format_flags = TYPE_FORMAT_LIFETIME;
                lifetime = "<'l>";
            },
            _ => {
                type_format_flags = TYPE_FORMAT_SAFE;
                lifetime = "";
            },
        }
        let (params_with_info, result) = get_safe_params(sel, cmd, &self.style, type_format_flags);

        write_documentation(w,sel, f, cmd)?;
        write_feature_protect(w, sel, f)?;
        write!(w, "#[inline]\n")?;
        write!(w, "pub fn {}{} (", cmd_name, lifetime)?;
        let mut i = 0;
        for param in &params_with_info {
            if !param.length_for.is_empty() {
                continue;
            }
            if i > 0 {
                write!(w, ", ")?;
            }
            i += 1;
            write!(w, "{}: {}", param.name, param.safe_type)?;
        }
        write!(w, ")")?;
        let mut handle_enumeration = false;
        if let Some(ref r) = result {
            if let Some(p) = r.length_param {
                handle_enumeration = p.base_type.modifiers == &[TypeModifier::Pointer];
            }
            if !cmd.return_type.is_empty() {
                if !handle_enumeration && cmd.success_codes.len() > 1 {
                    write!(w, " -> VkResultObj<({}, VkResult)>", r.safe_type)?;
                } else {
                    write!(w, " -> VkResultObj<{}>", r.safe_type)?;
                }
            } else {
                write!(w, " -> {}", r.safe_type)?;
            }
        } else if cmd.returns_error() {
            if cmd.success_codes.len() > 1 {
                write!(w, " -> VkResultObj")?;
            } else {
                write!(w, " -> VkResultObj<()>")?;
            }
        } else if !cmd.return_type.is_empty() {
            write!(w, " -> {}", return_type_ref(sel, &cmd.return_type, type_format_flags))?;
        }
        let handle_vk_incomplete = handle_enumeration && cmd.returns_error() && cmd.success_codes.contains("VK_INCOMPLETE");
        if result.is_some() && cmd.returns_error() {
            // validate success_codes
            if !cmd.success_codes.is_empty() && !cmd.success_codes.contains("VK_SUCCESS") {
                return Err(io::Error::new(io::ErrorKind::Other, format!("command {} doesn't return VK_SUCCESS", cmd.name)));
            }
            if handle_enumeration {
                if handle_vk_incomplete && cmd.success_codes.len() != 2 || !handle_vk_incomplete && cmd.success_codes.len() != 1 {
                    return Err(io::Error::new(io::ErrorKind::Other, format!("command {} returns somethind unexpected", cmd.name)));
                }
            }
        }

        write!(w, " {{\n")?;
        w.indent(|w|{
            write!(w, "unsafe {{\n")?;
            w.indent(|w|{
                for param in &params_with_info {
                    if param.length_for.is_empty() {
                        if let Some(ref tmp_var) = param.tmp_var {
                            write!(w, "{}\n", tmp_var)?;
                        }
                    } else if param.param.base_type.modifiers == &[TypeModifier::Pointer] {
                        write!(w, "let mut {} : {} = 0;\n", param.name, return_type_ref(sel, &param.param.base_type, 0))?;
                    } else {
                        write!(w, "let {} = {}.len() as {};\n", param.name, self.style.param_name(&param.length_for[0].name), def_type_ref(sel, param.param, 0))?;
                        if let Some(errorvalue) = cmd.returns_status() {
                            for more_arg in &param.length_for[1..] {
                                if more_arg.base_type.modifiers == &[TypeModifier::Pointer] {
                                    continue; // skip return values
                                }
                                write!(w, "if {} != {}.len() as u32 {{\n", param.name, self.style.param_name(&more_arg.name))?;
                                w.indent(|w| {
                                    if let Some(_) = result {
                                        write!(w, "return Err({});\n", errorvalue)
                                    } else if cmd.returns_error() {
                                        write!(w, "return Err({});\n", errorvalue)
                                    } else {
                                        write!(w, "return {};\n", errorvalue)
                                    }
                                })?;
                                write!(w, "}}\n")?;
                            }
                        }
                    }
                }
                if let Some(SafeParamInfoEntry{tmp_var: Some(ref tmp_var), ..}) = result {
                    write!(w, "{}\n", tmp_var)?;
                }
                if handle_vk_incomplete {
                    write!(w, "loop {{\n")?;
                    w.push_indent();
                }
                if result.is_some() && !cmd.return_type.is_empty() || cmd.returns_error() {
                    write!(w, "let _res = ")?;
                }
                let mut dispatch_call = format!("ffi::{}(", cmd.name); // wispatch call, without return-argument
                let mut dispatch_nargs = 0;
                for param in &params_with_info {
                    if dispatch_nargs > 0 {
                        dispatch_call.push_str(", ");
                    }
                    dispatch_nargs += 1;
                    if !param.length_for.is_empty() && param.param.base_type.modifiers == &[TypeModifier::Pointer] {
                        dispatch_call.push_str("&mut ");
                        dispatch_call.push_str(param.name.as_str());
                    } else {
                        dispatch_call.push_str(param.value.as_str());
                    }
                }
                if result.is_some() && dispatch_nargs > 0 {
                    dispatch_call.push_str(", ");
                }
                write!(w, "{}", dispatch_call)?;
                if let Some(ref r) = result {
                    if handle_enumeration {
                        write!(w, "ptr::null_mut()")?;
                    } else {
                        write!(w, "{}", r.value)?;
                    }
                    write!(w, ");\n")?;
                    if !cmd.return_type.is_empty() {
                        if !handle_enumeration && cmd.success_codes.len() > 1 {
                            write!(w, "if _res.is_error() {{ return Err(_res); }}\n")?;
                        } else {
                            write!(w, "if _res != VK_SUCCESS {{ return Err(_res); }}\n")?;
                        }
                    }
                    if let Some(r_len) = r.length_param {
                        if handle_enumeration {
                            write!(w, "if {} > 0 {{\n", self.style.param_name(&r_len.name))?;
                            w.indent(|w|{
                                write!(w, "{} = Vec::with_capacity({} as usize);\n", r.name, self.style.param_name(&r_len.name))?;
                                if !cmd.return_type.is_empty() {
                                    write!(w, "let _res = ")?;
                                }
                                write!(w, "{}{}.as_mut_ptr());\n", dispatch_call, r.name)?;
                                if !cmd.return_type.is_empty() {
                                    if handle_vk_incomplete {
                                        write!(w, "if _res == VK_INCOMPLETE {{ continue; }} ")?;
                                    }
                                    write!(w, "if _res != VK_SUCCESS {{ return Err(_res); }}\n")?;
                                }
                                Ok(())
                            })?;
                            write!(w, "}}\n")?;
                        }
                    }
                    if let Some(ref set_len) = r.set_length {
                        write!(w, "{}\n", set_len)?;
                    }
                    if !cmd.return_type.is_empty() {
                        if !handle_enumeration && cmd.success_codes.len() > 1 {
                            write!(w, "return Ok(({}, _res));\n", r.name)?;
                        } else {
                            write!(w, "return Ok({});\n", r.name)?;
                        }
                    } else {
                        write!(w, "return {};\n", r.name)?;
                    }
                } else if cmd.returns_error() {
                    write!(w, ");\n")?;
                    if cmd.success_codes.len() > 1 {
                        write!(w, "if _res.is_error() {{ return Err(_res); }}\n")?;
                        write!(w, "return Ok(_res);\n")?;
                    } else {
                        write!(w, "if _res != VK_SUCCESS {{ return Err(_res); }}\n")?;
                        write!(w, "return Ok(());\n")?;
                    }
                } else {
                    write!(w, ")\n")?;
                }
                if handle_vk_incomplete {
                    w.pop_indent();
                    write!(w, "}}\n")?;
                }
                Ok(())
            })?;
            write!(w, "}}\n")
        })?;
        write!(w, "}}\n")
    }
}
