use registry::*;
use case::CaseStyle::{SnakeCase,PascalCase};
use std::ascii::AsciiExt;
use std::collections::BTreeMap;
use super::*;
use std::io::Result;


const BUILTIN_TYPES : [(&str,&str);15] = [
    ("void",         "raw::c_void"),
    ("char",         "raw::c_char"),
    ("int",          "raw::c_int"),
    ("unsigned int", "raw::c_uint"),
    ("size_t",       "usize"),
    ("float",        "f32"),
    ("double",       "f64"),
    ("uint8_t",      "u8"),
    ("uint16_t",     "u16"),
    ("uint32_t",     "u32"),
    ("uint64_t",     "u64"),
    ("int8_t",       "i8"),
    ("int16_t",      "i16"),
    ("int32_t",      "i32"),
    ("int64_t",      "i64"),
];


const KEYWORDS : [&str;33]  = ["as", "break", "const", "continue", "crate", "else", "enum",
    "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
    "mod", "move", "mut", "pub", "ref", "return", "self", "static", "struct",
    "type", "trait", "true", "unsafe", "use", "where", "while"];

pub fn get_dispatch_table_for_handle(handle: &str, reg: &Registry) -> Result<&'static str> {
    match handle {
        "VkDevice" => Ok("VkDevice"),
        "VkInstance" => Ok("VkInstance"),
        _ => {
            if let Some(&&TypeDefinition::Handle(ref h)) = reg.type_by_name.get(handle) {
                for parent in &h.parents {
                    if let Ok(h) = get_dispatch_table_for_handle(parent.as_str(), reg) {
                        return Ok(h);
                    }
                }
                Err(io::Error::new(io::ErrorKind::Other, format!("unable to detect dispatch-table for {}", handle)))
            } else {
                Ok("VkLoader")
            }
        }
    }
}

pub fn get_dispatch_table_for_command(cmd: &CommandDefinition, reg: &Registry) -> Result<&'static str> {
    if cmd.parameters.is_empty() {
        Ok("VkLoader")
    } else {
        get_dispatch_table_for_handle(cmd.parameters[0].base_type.type_name.as_str(), reg)
    }
}

#[inline]
pub fn strip_enum_name(prefix: &str, strip_from: &str, reg: &Registry) -> String {
    let (strip_from, vendor) = strip_vendor_suffix(strip_from, reg);
    let stripped = strip_snake_prefix(&prefix, strip_from);
    let enumn_name = PascalCase.apply_to_snake(stripped) + vendor;
    norm_snake_kw(&enumn_name, "E")
}

#[inline]
pub fn write_feature_protect<W: CodeWrite>(w: &mut W, _: &Selection, f: &FeatureSet) -> Result<bool> {
    if !f.protect.is_empty() {
        write!(w, "#[cfg(feature = \"{}\")]\n", f.protect)?;
        Ok(true)
    } else {
        Ok(false)
    }
}


pub fn write_documentation<W: CodeWrite>(w: &mut W, _: &Selection, _: &FeatureSet, def: &BasicDefinition) -> Result<()> {
    //let def = def.deref();
    let mut comment = def.comment.as_str();
    while comment.starts_with('/') {
        comment = &comment[1..];
    }
    comment = comment.trim();
    if comment.len() > 0 {
        w.prefixed("/// ", |w|{
            write!(w, "{}\n", comment)?;
            Ok(())
        })?;
    }
    Ok(())
}

pub fn get_variant_type<'l>(sel: &'l Selection<'l>, v: &VariantValue, default_use_usize: bool) -> Option<&'l str> {
    match *v {
        VariantValue::None | VariantValue::Raw(_)  => None,
        VariantValue::String(_) => Some("&str"),
        VariantValue::Enum(ref s) => {
            if let Some(e) = sel.enum_item_by_name.get(s.as_str()) {
                if !e.group.is_empty() {
                    return Some(e.group.as_str());
                }
            }
            None
        },
        VariantValue::Bit(_) => Some("u32"),
        VariantValue::Integer(i, mut bits) => {
            if bits == 0 {
                if i >= 0 {
                    if default_use_usize {
                        return Some("usize");
                    } else if i as u64 <= ::std::u32::MAX as u64 {
                        return Some("u32");
                    } else {
                        return Some("u64");
                    }
                }
                bits = if ::std::i32::MIN as i64 <= i && i <= ::std::i32::MAX as i64 { 32 } else { 64 };
            }
            match bits {
                8 => Some("i8"), 16 => Some("i16"), 32 => Some("i32"), 64 => Some("i64"),
                _ => None,
            }
        },
        VariantValue::UnsignedInteger(i, mut bits) => {
            if bits == 0 {
                if default_use_usize {
                    return Some("usize");
                }
                if (i as i64) < 0 {
                    bits = if (i as i64) < -(::std::u32::MAX as i64) { 64 } else { 32 };
                } else {
                    bits = if i <= ::std::u32::MAX as u64 { 32 } else { 64 };
                }
            }
            match bits {
                8 => Some("u8"), 16 => Some("u16"), 32 => Some("u32"), 64 => Some("u64"),
                _ => None,
            }
        },
        VariantValue::Float(_, mut bits) => {
            if bits == 0 {
                bits = 32;
            }
            match bits {
                8 => Some("f8"), 16 => Some("f16"), 32 => Some("f32"), 64 => Some("f64"),
                _ => None,
            }
        },
    }
}

pub fn get_variant_value(_: &Selection, v: &VariantValue) -> Option<String> {
    match *v {
        VariantValue::None => None,
        VariantValue::String(ref s) | VariantValue::Raw(ref s) => Some(s.to_owned()),
        VariantValue::Enum(ref s) => Some(s.to_ascii_uppercase()),
        VariantValue::Bit(i) => Some(format!("1<<{}", i)),
        VariantValue::Integer(i, _) => {
            if i > 1024 {
                Some(format!("0x{:x}", i))
            } else {
                Some(format!("{}", i))
            }
        },
        VariantValue::UnsignedInteger(mut i, _) => {
            let neg : bool = (i as i64) < 0i64;
            if neg {
                i = !i;
            }
            let neg_op = if neg { "!" } else { "" };
            if i > 1024 {
                Some(format!("{}0x{:x}", neg_op, i))
            } else {
                Some(format!("{}{}", neg_op, i))
            }
        },
        VariantValue::Float(f, _) => {
            Some(format!("{:e}", f))
        },
    }
}

pub fn get_variant_value_for_enum(sel: &Selection, v: &VariantValue) -> Option<String> {
    match *v {
        VariantValue::None => None,
        VariantValue::String(ref s) | VariantValue::Raw(ref s) => Some(s.to_owned()),
        VariantValue::Enum(ref s) => {
            if let Some(_) = sel.enum_item_by_name.get(s.as_str()) {
                Some(format!("{} as u32", s.to_ascii_uppercase()))
            } else {
                Some(s.to_owned())
            }
        },
        VariantValue::Bit(i) => Some(format!("1<<{}", i)),
        VariantValue::Integer(i, _) => {
            if i < 0 {
                Some(format!("!{}", !(i as u64)))
            } else {
                Some(format!("{}", i))
            }
        },
        VariantValue::UnsignedInteger(i, _) => {
            if (i as i64) < 0 {
                Some(format!("!{}", !i))
            } else {
                Some(format!("{}", i))
            }
        },
        VariantValue::Float(f, _) => {
            Some(format!("{:e}", f))
        },
    }
}

#[inline]
pub fn get_variant_value_and_type<'l>(sel: &'l Selection<'l>, v: &VariantValue, default_use_usize: bool) -> Option<(String,&'l str)> {
    if let Some(ty) = get_variant_type(sel, v, default_use_usize) {
        if let Some(val) = get_variant_value(sel, v) {
            return Some((val, ty));
        }
    }
    None
}

pub const TYPE_FORMAT_SAFE : u32 = 0x01;
pub const TYPE_FORMAT_LIFETIME : u32 = 0; // disabled. originnal value: 0x02;

pub fn format_named_type_ref<W: fmt::Write>(w: &mut W, sel: &Selection, ty: &str, format: u32) -> fmt::Result {
    for &(c_type, r_type) in &BUILTIN_TYPES {
        if ty == c_type {
            write!(w, "{}", r_type)?;
            return Ok(());
        }
    }
    write!(w, "{}", ty)?;
    if (format & TYPE_FORMAT_LIFETIME) != 0 && (sel.get_type_flags(ty) & TF_CONTAINS_HANDLE) != 0 {
        write!(w, "<'l>")?;
    }
    Ok(())
}

pub fn format_type_ref<W: fmt::Write>(w: &mut W, sel: &Selection, ty: &TypeReference, format: u32) -> fmt::Result {
    let mut i = ty.modifiers.len();
    while i > 0 {
        i -= 1;
        match ty.modifiers[i] {
            TypeModifier::Const => {}, // ignore
            TypeModifier::Pointer => {
                if i > 0 && ty.modifiers[i-1] == TypeModifier::Const {
                    if (format & TYPE_FORMAT_SAFE) != 0 {
                        // if (format & TYPE_FORMAT_LIFETIME) != 0 && (sel.get_type_flags(ty) & TF_CONTAINS_HANDLE) != 0 {
                        //     format &= !TYPE_FORMAT_LIFETIME;
                        //     write!(w, "&'l ")?;
                        // } else {
                            write!(w, "&")?;
                        // }
                    } else {
                        write!(w, "*const ")?;
                    }
                    i -= 1;
                } else {
                    if (format & TYPE_FORMAT_SAFE) != 0 {
                        // if (format & TYPE_FORMAT_LIFETIME) != 0 && (sel.get_type_flags(ty) & TF_CONTAINS_HANDLE) != 0 {
                        //     format &= !TYPE_FORMAT_LIFETIME;
                        //     write!(w, "&'l mut ")?;
                        // } else {
                            write!(w, "&mut ")?;
                        // }
                    } else {
                        write!(w, "*mut ")?;
                    }
                }
            },
            TypeModifier::Array(ref dim) => {
                let rest = TypeReference{
                    type_name: ty.type_name.clone(),
                    modifiers: ty.modifiers[..i].iter().cloned().collect(),
                };
                write!(w, "[")?;
                format_type_ref(w, sel, &rest, format)?;
                write!(w, ";{}]", dim)?;
                return Ok(());
            }
        }
    }
    format_named_type_ref(w, sel, ty.type_name.as_str(), format)
}

pub fn named_type_ref(sel: &Selection, ty: &str, format: u32) -> String {
    let mut w = String::new();
    format_named_type_ref(&mut w, sel, ty, format).expect("unexpected");
    w
}

pub fn type_ref(sel: &Selection, ty: &TypeReference, format: u32) -> String {
    let mut w = String::new();
    format_type_ref(&mut w, sel, ty, format).expect("unexpected");
    w
}

pub fn return_type_ref(sel: &Selection, ty: &TypeReference, format: u32) -> String {
    if ty.modifiers.is_empty() {
        if ty.type_name.starts_with("PFN_") {
            return format!("Option<{}>", ty.type_name);
        }
    } else if ty.modifiers.ends_with(&[TypeModifier::Pointer]) && !ty.modifiers.ends_with(&[TypeModifier::Const, TypeModifier::Pointer]) {
        let rest = TypeReference{
            type_name: ty.type_name.clone(),
            modifiers: ty.modifiers[..ty.modifiers.len()-1].iter().cloned().collect(),
        };
        return type_ref(sel, &rest, format & !TYPE_FORMAT_SAFE)
    }
    type_ref(sel, ty, format)
}

pub fn def_type_ref(sel: &Selection, def: &BasicDefinition, format: u32) -> String {
    // if def.base_type.modifiers.is_empty() && def.base_type.type_name == "uint32_t" && def.name.ends_with("Version") {
    //     return "util::VkVersion".to_owned();
    // }
    type_ref(sel, &def.base_type, format)
}

pub fn get_param_name_map<'l>(params: &'l[ParameterDefinition]) -> BTreeMap<&'l str, &'l ParameterDefinition> {
    let mut names : BTreeMap<&'l str, &'l ParameterDefinition> = BTreeMap::new();
    for param in params {
        names.insert(param.name.as_str(), param);
    }
    names
}

pub fn get_param_length_map<'l>(params: &'l[ParameterDefinition]) -> BTreeMap<&'l str, Vec<&'l ParameterDefinition>> {
    let mut lengths : BTreeMap<&'l str, Vec<&'l ParameterDefinition>> = BTreeMap::new();
    for param in params {
        if let Some(ref l) = param.len {
            lengths.entry(l.as_str()).or_insert(Vec::new()).push(param);
        }
    }
    lengths
}

pub struct SafeParamInfoEntry<'l> {
    pub param: &'l ParameterDefinition,
    pub length_for: Vec<&'l ParameterDefinition>,
    pub length_param: Option<&'l ParameterDefinition>,
    pub name: String,
    pub safe_type: String,
    pub tmp_var: Option<String>,
    pub set_length: Option<String>,
    pub value: String,
}

pub fn get_return_param<'l>(_: &Selection, cmd: &'l CommandDefinition) -> Option<&'l ParameterDefinition> {
    if let Some(p) = cmd.parameters.last() {
        if (cmd.return_type.is_empty() || cmd.returns_error()) && p.base_type.modifiers.starts_with(&[TypeModifier::Pointer]) {
            return Some(p);
        }
    }
    None
}

pub fn get_safe_params<'l>(sel: &Selection, cmd: &'l CommandDefinition, style: &CodeStyle, format: u32) -> (Vec<SafeParamInfoEntry<'l>>, Option<SafeParamInfoEntry<'l>>)
{
    let param_names = get_param_name_map(&cmd.parameters);
    let param_lengths = get_param_length_map(&cmd.parameters);
    let mut params_and_lengths: Vec<SafeParamInfoEntry<'l>> = Vec::new();
    let mut result: Option<SafeParamInfoEntry<'l>> = None;
    let last = cmd.parameters.len() - 1;
    for (i, param) in (&cmd.parameters).into_iter().enumerate() {
        let length_param;
        if let Some(ref l) = param.len {
            if let Some(p) = param_names.get(l.as_str()) {
                length_param = Some(*p);
            } else {
                length_param = None;
            }
        } else {
            length_param = None;
        }
        let name = style.param_name(&param.name);
        let length_for;
        if i==last && (cmd.return_type.is_empty() || cmd.returns_error()) && param.base_type.modifiers.starts_with(&[TypeModifier::Pointer]) {
            let safe_type;
            let tmp_var;
            let value;
            let set_length;
            if let Some(ref l) = param.len {
                safe_type = format!("Vec<{}>", return_type_ref(sel, &param.base_type, format));
                match length_param {
                    Some(p) if p.base_type.modifiers == &[TypeModifier::Pointer] => {
                        tmp_var = format!("let mut {} : {} = Vec::new();", name, safe_type);
                        set_length = Some(format!("{}.set_len({} as usize);", name, style.param_name(&p.name)));
                    },
                    _ => {
                        let len_parts = l.split("::").collect::<Vec<&str>>();
                        let mut len_str = style.param_name(len_parts[0]);
                        for len_part in &len_parts[1..] {
                            len_str += ".";
                            len_str += &style.field_name(len_part);
                        }
                        tmp_var = format!("let mut {} : {} = Vec::with_capacity({} as usize);", name, safe_type, len_str);
                        set_length = Some(format!("{}.set_len({} as usize);", name, len_str));
                    }
                }
                value = format!("{}.as_mut_ptr()", name);
            } else {
                set_length = None;
                safe_type = return_type_ref(sel, &param.base_type, format);
                if safe_type.starts_with("*mut") {
                    tmp_var = format!("let mut {} : {} = ptr::null_mut();", name, safe_type);
                } else if safe_type.starts_with("*const") {
                    tmp_var = format!("let mut {} : {} = ptr::null();", name, safe_type);
                } else {
                    tmp_var = format!("let mut {} : {} = Default::default();", name, safe_type);
                }
                value = format!("&mut {}", name);
            }
            result = Some(SafeParamInfoEntry{param, length_for: Vec::new(), length_param, name, safe_type, tmp_var: Some(tmp_var), set_length, value});
            continue;
        } else if let Some(v) = param_lengths.get(param.name.as_str()) {
            length_for = v.clone()
        } else {
            length_for = Vec::new();
        }

        let (safe_type, tmp_var, value) = safe_param_type_ref(name.as_str(), sel, cmd, param, format);

        params_and_lengths.push(SafeParamInfoEntry{param, length_for, length_param, name, safe_type, tmp_var, set_length: None, value});
    }
    (params_and_lengths, result)
}


pub fn safe_param_type_ref(param_name: &str, sel: &Selection, _: &CommandDefinition, param: &ParameterDefinition, format: u32) -> (String,Option<String>,String) {
    let ty = &param.base_type;
    if ty.modifiers.ends_with(&[TypeModifier::Const, TypeModifier::Pointer]) {
        match param.len {
            Some(ref len) if ty.modifiers.len()==2 && len == "null-terminated" && param.base_type.type_name == "char"=> {
                if param.optional {
                    let arg_type = "Option<&str>".to_owned();
                    let tmp_var = format!("let {0} = {0}.map(|s| CString::from_vec_unchecked(s.into()));\nlet {0} = {0}.map(|s|s.as_ptr()).unwrap_or(ptr::null());", param_name);
                    let get_arg = format!("{}", param_name);
                    return (arg_type, Some(tmp_var), get_arg);
                } else {
                    let arg_type = "&str".to_owned();
                    let tmp_var = format!("let {0} = CString::from_vec_unchecked({0}.into());", param_name);
                    let get_arg = format!("{}.as_ptr()", param_name);
                    return (arg_type, Some(tmp_var), get_arg);
                }
            },
            Some(ref len) => {
                let base_type = if ty.modifiers.len()==2 && ty.type_name == "void" {
                    named_type_ref(sel, "uint8_t", format)
                } else {
                    let rest = TypeReference{
                        type_name: ty.type_name.clone(),
                        modifiers: ty.modifiers[..ty.modifiers.len()-2].iter().cloned().collect(),
                    };
                    type_ref(sel, &rest, format)
                };

                if let Ok(len) = len.parse::<i64>() {
                    let arg_type = if param.optional {
                        format!("Option<&[{};{}]>", base_type, len)
                    } else {
                        format!("&[{};{}]", base_type, len)
                    };
                    let get_arg = format!("mem::transmute({})", param_name);
                    return (arg_type, None, get_arg);
                } else {
                    let arg_type = if param.optional {
                        format!("Option<&[{}]>", base_type)
                    } else {
                        format!("&[{}]", base_type)
                    };
                    let get_arg = if ty.type_name == "void" || ty.modifiers.len()>2 {
                        format!("mem::transmute({}.as_ptr())", param_name)
                    } else {
                        format!("{}.as_ptr()", param_name)
                    };
                    return (arg_type, None, get_arg);
                }
            }
            _ => {
                let base_type = named_type_ref(sel, ty.type_name.as_str(), format);
                if param.optional {
                    let arg_type = format!("Option<&{}>", base_type);
                    let get_arg = format!("mem::transmute({})", param_name);
                    return (arg_type, None, get_arg);
                } else {
                    let arg_type = format!("&{}", base_type);
                    return (arg_type, None, param_name.to_owned());
                }
            }
        }
    }
    if ty.modifiers == &[TypeModifier::Pointer] {
        match param.len {
            Some(ref len) => {
                let base_type = if ty.type_name == "void" {
                    named_type_ref(sel, "uint8_t", format)
                } else {
                    named_type_ref(sel, ty.type_name.as_str(), format)
                };

                if let Ok(len) = len.parse::<i64>() {
                    let arg_type = if param.optional {
                        format!("Option<&mut[{};{}]>", base_type, len)
                    } else {
                        format!("&mut[{};{}]", base_type, len)
                    };
                    let get_arg = format!("mem::transmute({})", param_name);
                    return (arg_type, None, get_arg);
                } else {
                    let arg_type = if param.optional {
                        format!("Option<&mut[{}]>", base_type)
                    } else {
                        format!("&mut[{}]", base_type)
                    };
                    let get_arg = if ty.type_name == "void" {
                        format!("mem::transmute({}.as_mut_ptr())", param_name)
                    } else {
                        format!("{}.as_mut_ptr()", param_name)
                    };
                    return (arg_type, None, get_arg);
                }
            }
            _ => {
                let base_type = named_type_ref(sel, ty.type_name.as_str(), format);
                if param.optional {
                    let arg_type = format!("Option<&mut {}>", base_type);
                    let get_arg = format!("mem::transmute({})", param_name);
                    return (arg_type, None, get_arg);
                } else {
                    let arg_type = format!("&mut {}", base_type);
                    return (arg_type, None, param_name.to_owned());
                }
            }
        }
    }

    return (def_type_ref(sel, param, format), None, param_name.to_owned());
}

pub fn norm_snake_kw(name: &str, kw_prefix: &str) -> String {
    if name.starts_with(char::is_numeric) {
        return kw_prefix.to_owned() + name;
    }
    if KEYWORDS.contains(&name) {
        return kw_prefix.to_owned() + name;
    }
    return name.to_owned();
}

pub fn norm_camel_kw(name: &str, kw_prefix: &str) -> String {
    if name.starts_with(char::is_numeric) {
        return kw_prefix.to_owned() + &name[..1].to_ascii_uppercase() + &name[1..];
    }
    if KEYWORDS.contains(&name) {
        return kw_prefix.to_owned() + &name[..1].to_ascii_uppercase() + &name[1..];
    }
    return name.to_owned();
}


impl CodeStyle {

    pub fn field_name(&self, name: &str) -> String {
        if self.snake_case_fields {
            norm_snake_kw(&SnakeCase.apply_to_camel(name), "p_")
        } else {
            norm_camel_kw(name, "p")
        }
    }

    pub fn command_name(&self, name: &str) -> String {
        if self.snake_case_commands {
            SnakeCase.apply_to_camel(name)
        } else {
            name.to_owned()
        }
    }

    pub fn param_name(&self, name: &str) -> String {
        norm_snake_kw(&SnakeCase.apply_to_camel(name), "p_")
    }
}
