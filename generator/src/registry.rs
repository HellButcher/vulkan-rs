/*
**  Copyright (c) 2016, Christoph Hommelsheim
**  All rights reserved.
**
**  Redistribution and use in source and binary forms, with or without
**  modification, are permitted provided that the following conditions are met:
**
**  * Redistributions of source code must retain the above copyright notice, this
**    list of conditions and the following disclaimer.
**
**  * Redistributions in binary form must reproduce the above copyright notice,
**    this list of conditions and the following disclaimer in the documentation
**    and/or other materials provided with the distribution.
**
**  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
**  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
**  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
**  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
**  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
**  DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
**  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
**  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
**  OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
**  OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
**
*/
use std::fmt;
use std::ops;
use std::collections::{HashSet,HashMap};
use xml::reader::{EventReader, XmlEvent};
use xml::common::{Position,TextPosition};
use xml::name::OwnedName;
use xml::attribute::OwnedAttribute;

#[derive(Debug)]
pub enum Error {
    IoError(::std::io::Error),
    XmlError(::xml::reader::Error),
    UnexpectedElement(XmlEvent, TextPosition),
    ParseError(String, TextPosition),
    GeneralError(String),
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Error {
        Error::IoError(e)
    }
}

impl From<::xml::reader::Error> for Error {
    fn from(e: ::xml::reader::Error) -> Error {
        Error::XmlError(e)
    }
}

pub type Result<V=()> = ::std::result::Result<V,Error>;

pub trait NamedElement {
    fn name(&self) -> &str;
}

#[derive(Clone, Debug, Default)]
pub struct RegistryData {
    pub api: Api,
    pub comment: String,
    pub tags: HashSet<String>,
    pub types: Vec<TypeDefinition>,
    pub enum_groups: Vec<EnumGroupDefinition>,
    pub commands: Vec<CommandDefinition>,
    pub features: Vec<FeatureSet>,
    pub extensions: Vec<FeatureSet>,
    pub enum_extensions: Vec<EnumItemDefinition>,
}

#[derive(Clone, Debug)]
pub struct Registry<'r> {
    data: &'r RegistryData,
    pub type_by_name: HashMap<&'r str,&'r TypeDefinition>,
    pub enum_group_by_name: HashMap<&'r str,&'r EnumGroupDefinition>,
    pub bitmask_by_groupname: HashMap<&'r str,&'r BasicDefinition>,
    pub enum_item_by_name: HashMap<&'r str,&'r EnumItemDefinition>,
    pub enum_items_by_group: HashMap<&'r str,Vec<&'r EnumItemDefinition>>,
    pub command_by_name: HashMap<&'r str,&'r CommandDefinition>,
    pub feature_by_name: HashMap<&'r str,&'r FeatureSet>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Api {
    Vulkan,
}

impl Default for Api {
    fn default() -> Api {
        Api::Vulkan
    }
}


impl Api {
    #[inline]
    pub fn from_str(s: &str) -> Option<Api> {
        use self::Api::*;
        match s {
            "vulkan" => Some(Vulkan),
            _ => None,
        }
    }
}

impl fmt::Display for Api {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Api::Vulkan => write!(fmt, "vulkan"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeDefinition {
    Defined(BasicDefinition),
    Include(IncludeDefinition),
    Constant(ConstantDefinition),
    BaseType(BasicDefinition),
    BitMask(BasicDefinition),
    Enum(BasicDefinition),
    Handle(HandleDefinition),
    FunctionPointer(CommandDefinition),
    Struct(StructDefinition),
    Union(StructDefinition),
}

#[derive(Clone, Debug, Default)]
pub struct BasicDefinition {
    pub name: String,
    pub requires: String,
    pub base_type: TypeReference,
    pub comment: String,
}

impl NamedElement for BasicDefinition {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl NamedElement for TypeDefinition {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Clone, Debug, Default)]
pub struct IncludeDefinition {
    base: BasicDefinition,
    pub include: String,
    pub relative: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ConstantDefinition {
    base: BasicDefinition,
    pub value: VariantValue,
}

#[derive(Clone, Debug, Default)]
pub struct EnumItemDefinition {
    base: BasicDefinition,
    pub value: VariantValue,
    pub group: String,
    pub extension: String,
    pub offset: Option<i32>,
}

#[derive(Clone, Debug, Default)]
pub struct EnumGroupDefinition {
    base: BasicDefinition,
    pub enum_type: EnumGroupType,
    pub items: Vec<EnumItemDefinition>,
}

#[derive(Clone, Debug, Default)]
pub struct HandleDefinition {
    base: BasicDefinition,
    pub dispatchable: bool,
    pub parents: HashSet<String>,
}

#[derive(Clone, Debug, Default)]
pub struct StructDefinition {
    base: BasicDefinition,
    pub members: Vec<ParameterDefinition>,
    pub returnedonly: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CommandDefinition {
    base: BasicDefinition,
    pub return_type: TypeReference,
    pub parameters: Vec<ParameterDefinition>,
    pub success_codes: HashSet<String>,
    pub error_codes: HashSet<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VariantValue {
    None,
    Bit(u64),
    Integer(i64, u8),
    UnsignedInteger(u64,u8),
    Float(f64,u8),
    String(String),
    Enum(String),
    Raw(String),
}

impl VariantValue {
    pub fn parse(s: &str) -> VariantValue {
        let s = s.trim();
        if s.starts_with(|c| 'A' <= c && c <= 'Z') {
            VariantValue::Enum(s.to_owned())
        } else {
            let (res, end) = VariantValue::parse_expression(s);
            if res == VariantValue::None && end < s.len() {
                VariantValue::Raw(s.to_owned())
            } else {
                res
            }
        }
    }
    fn parse_expression(s: &str) -> (VariantValue, usize) {
        let (lhs, lhs_end) = VariantValue::parse_unary_expression(s);
        if lhs_end!=0 {
            let s = &s[lhs_end..];
            if s.starts_with('+') {
                let (rhs, rhs_end) = VariantValue::parse_expression(&s[1..]);
                if rhs_end != 0 {
                    let v = &lhs + &rhs;
                    if v != VariantValue::None {
                        return (v, lhs_end+1+rhs_end);
                    }
                }
            } else if s.starts_with('-') {
                let (rhs, rhs_end) = VariantValue::parse_expression(&s[1..]);
                if rhs_end != 0 {
                    let v = &lhs - &rhs;
                    if v != VariantValue::None {
                        return (v, lhs_end+1+rhs_end);
                    }
                }
            }
        }
        (lhs, lhs_end)
    }

    fn parse_unary_expression(s: &str) -> (VariantValue, usize) {
        if s.starts_with('(') {
            match VariantValue::parse_expression(&s[1..]) {
                (VariantValue::None, _) => {
                    (VariantValue::None, 0)
                },
                (v, end) => {
                    if s[end+1..].starts_with(')') {
                        (v,end+2)
                    } else {
                        (VariantValue::None, 0)
                    }
                },
            }
        } else if s.starts_with('+') {
            match VariantValue::parse_literal(&s[1..]) {
                (VariantValue::None, _) | (VariantValue::String(_), _) | (VariantValue::Raw(_), _) | (VariantValue::Enum(_), _) => (VariantValue::None, 0),
                (v, end) => (v, end+1),
            }
        } else if s.starts_with('-') {
            match VariantValue::parse_literal(&s[1..]) {
                (VariantValue::None, _) | (VariantValue::String(_), _) | (VariantValue::Raw(_), _) | (VariantValue::Enum(_), _) => (VariantValue::None, 0),
                (VariantValue::Bit(i), end)                   => (VariantValue::Integer(-((1<<i) as i64), 0), end+1),
                (VariantValue::Integer(i, bits), end)         => (VariantValue::Integer(-i, bits), end+1),
                (VariantValue::UnsignedInteger(i, bits), end) => (VariantValue::Integer(-(i as i64), bits), end+1),
                (VariantValue::Float(f, bits), end)           => (VariantValue::Float(-f, bits), end+1),
            }
        } else if s.starts_with('~') {
            match VariantValue::parse_literal(&s[1..]) {
                (VariantValue::None, _) | (VariantValue::String(_), _) | (VariantValue::Raw(_), _) | (VariantValue::Float(_,_), _) | (VariantValue::Enum(_), _) => (VariantValue::None, 0),
                (VariantValue::Bit(i), end)                   => (VariantValue::UnsignedInteger(!(1<<i), 0), end+1),
                (VariantValue::Integer(i, bits), end)         => (VariantValue::UnsignedInteger(!(i as u64), bits), end+1),
                (VariantValue::UnsignedInteger(i, bits), end) => (VariantValue::UnsignedInteger(!i, bits), end+1),
            }
        } else {
            VariantValue::parse_literal(s)
        }
    }

    fn parse_literal(s: &str) -> (VariantValue, usize) {
        if s.is_empty() {
            (VariantValue::None, 0)
        } else if s.starts_with('"') { // string literal
            // find next unescaped quote character
            let mut chars = s[1..].char_indices();
            while let Some((i,c)) = chars.next() {
                if c == '"' {
                    return (VariantValue::String(s[..i+2].to_owned()), i+2);
                } else if c == '\\' {
                    chars.next(); // skip the next (escaped) character
                }
            }
            (VariantValue::None, 0) // missing quote character
        } else if s.starts_with("0x") || s.starts_with("0X") {
            let (value, end) = VariantValue::parse_digits(&s[2..], 16);
            if end == 0 {
                (VariantValue::None, 0)
            } else {
                (VariantValue::Integer(value as i64, 0), end+2)
            }
        } else if s.starts_with("0") && s[1..].starts_with(|c| '0'<= c && c<='7') {
            let (value, end) = VariantValue::parse_digits(s, 8);
            if end == 0 {
                (VariantValue::None, 0)
            } else {
                (VariantValue::Integer(value as i64, 0), end)
            }
        } else if s.starts_with(|c| '0' <= c && c<= '9') {
            let (value, end) = VariantValue::parse_digits(s, 10);
            if end == 0 {
                (VariantValue::None, 0)
            } else {
                let s = &s[end..];
                let (float_value, float_end) = VariantValue::parse_continue_float(s, value);
                if float_end == 0 {
                    let (bits, unsigned, sfx_end) = VariantValue::parse_integer_suffix(s);
                    if unsigned {
                        (VariantValue::UnsignedInteger(value, bits), end+sfx_end)
                    } else {
                        (VariantValue::Integer(value as i64, bits), end+sfx_end)
                    }
                } else {
                    let (bits, sfx_end) = VariantValue::parse_float_suffix(&s[float_end..]);
                    (VariantValue::Float(float_value, bits), end+float_end+sfx_end)
                }
            }
        } else if s.starts_with('.') {
            let (value, end) = VariantValue::parse_continue_float(s, 0);
            if end == 0 {
                (VariantValue::None, 0)
            } else {
                let (bits, sfx_end) = VariantValue::parse_float_suffix(&s[end..]);
                (VariantValue::Float(value, bits), end+sfx_end)
            }
        } else {
            (VariantValue::None, 0)
        }
    }

    fn parse_digits(s: &str, radix: u32) -> (u64, usize) {
        let mut value : u64 = 0;
        let mut chars = s.char_indices();
        while let Some((i,c)) = chars.next() {
            if let Some(d) = c.to_digit(radix) {
                value = (value * radix as u64) + d as u64;
            } else {
                return (value, i);
            }
        }
        (value, s.len())
    }

    fn parse_continue_float(s: &str, mut int_base: u64) -> (f64, usize) {
        let mut read_size = 0usize;
        let mut div = 1u64;
        if s.starts_with('.') {
            read_size = s.len();
            let mut chars = s[1..].char_indices();
            while let Some((i,c)) = chars.next() {
                if let Some(d) = c.to_digit(10) {
                    div *= 10;
                    int_base = (int_base * 10) + d as u64;
                } else {
                    read_size = i+1;
                    break;
                }
            }
        }
        let s = &s[read_size..];
        if s.starts_with('e') || s.starts_with('E') {
            read_size += 1;
            let mut s = &s[1..];
            let mut neg = false;
            if s.starts_with('-') {
                neg = true;
                read_size += 1;
                s = &s[1..];
            } else if s.starts_with('+') {
                read_size += 1;
                s = &s[1..];
            }
            let (exp, end) = VariantValue::parse_digits(&s[1..], 10);
            let exp = if neg { -(exp as i32) } else { exp as i32 };
            ((int_base as f64 / div as f64) * 10.0f64.powi(exp), read_size+end)
        } else {
            (int_base as f64 / div as f64, read_size)
        }
    }

    fn parse_float_suffix(s: &str) -> (u8, usize) {
        let mut bits = 0u8;
        let mut chars = s.char_indices();
        while let Some((i,c)) = chars.next() {
            match c {
                'l' | 'L' => {
                    bits = 64;
                },
                'f' | 'F' => {
                    if bits == 0 {
                        bits = 32;
                    }
                },
                _ => {
                    return (bits, i);
                }
            }
        }
        (bits, s.len())
    }

    fn parse_integer_suffix(s: &str) -> (u8, bool, usize) {
        let mut bits = 0u8;
        let mut unsigned = false;
        let mut chars = s.char_indices();
        while let Some((i,c)) = chars.next() {
            match c {
                'l' | 'L' => {
                    if bits==0 {
                        bits = 32
                    } else {
                        bits = 64; // second "L" suffix
                    }
                },
                'u' | 'U' => {
                    unsigned = true;
                },
                _ => {
                    return (bits, unsigned, i);
                }
            }
        }
        (bits, unsigned, s.len())
    }

    pub fn get_integer(&self, reg: &Registry) -> Option<i64> {
        match *self {
            VariantValue::Integer(v, _) => Some(v),
            VariantValue::UnsignedInteger(v, _) => Some(v as i64),
            VariantValue::Float(v, _) => Some(v as i64),
            VariantValue::Bit(v) => Some(1 << v),
            VariantValue::Raw(ref v) | VariantValue::Enum(ref v)=> {
                if let Some(e) = reg.enum_item_by_name.get(v.as_str()) {
                    e.value.get_integer(reg)
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}

macro_rules! add_sub_impl {
    ( $prim:ident ) => {
        impl<'l> ops::Add<$prim> for &'l VariantValue {
            type Output = VariantValue;
            fn add(self, rhs: $prim) -> VariantValue {
                match *self {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(i) => VariantValue::UnsignedInteger((1<<i as i64 + rhs as i64) as u64, 0),
                    VariantValue::Integer(i, bits) => VariantValue::Integer(i + rhs as i64, bits),
                    VariantValue::UnsignedInteger(i, bits) => VariantValue::UnsignedInteger((i as i64 + rhs as i64) as u64, bits),
                    VariantValue::Float(f, bits) => VariantValue::Float(f + rhs as f64, bits),
                }
            }
        }
        impl<'l> ops::Sub<$prim> for &'l VariantValue {
            type Output = VariantValue;
            fn sub(self, rhs: $prim) -> VariantValue {
                match *self {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(i) => VariantValue::UnsignedInteger((1<<i as i64 - rhs as i64) as u64, 0),
                    VariantValue::Integer(i, bits) => VariantValue::Integer(i - rhs as i64, bits),
                    VariantValue::UnsignedInteger(i, bits) => VariantValue::UnsignedInteger((i as i64 - rhs as i64) as u64, bits),
                    VariantValue::Float(f, bits) => VariantValue::Float(f - rhs as f64, bits),
                }
            }
        }
    };
}
add_sub_impl!(i8);
add_sub_impl!(i16);
add_sub_impl!(i32);
add_sub_impl!(i64);
add_sub_impl!(u8);
add_sub_impl!(u16);
add_sub_impl!(u32);
add_sub_impl!(u64);
add_sub_impl!(f32);
add_sub_impl!(f64);

impl<'l> ops::Add for &'l VariantValue {
    type Output = VariantValue;
    fn add(self, rhs: &VariantValue) -> VariantValue {
        use std::cmp::max;
        match *self {
            VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
            VariantValue::Bit(lhs) => {
                match *rhs {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(rhs) => VariantValue::UnsignedInteger((1<<lhs as i64 + 1<<rhs as i64) as u64, 0),
                    VariantValue::Integer(rhs, rhs_bits) => VariantValue::Integer(1<<lhs as i64 + rhs, rhs_bits),
                    VariantValue::UnsignedInteger(rhs, rhs_bits) => VariantValue::UnsignedInteger((1<<lhs as i64 + rhs as i64) as u64, rhs_bits),
                    VariantValue::Float(rhs, rhs_bits) => VariantValue::Float((1<<lhs) as f64 + rhs, rhs_bits),
                }
            },
            VariantValue::Integer(lhs, lhs_bits) => {
                match *rhs {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(rhs) => VariantValue::Integer((lhs + 1<<rhs as i64), lhs_bits),
                    VariantValue::Integer(rhs, rhs_bits) => VariantValue::Integer(lhs + rhs, max(lhs_bits, rhs_bits)),
                    VariantValue::UnsignedInteger(rhs, rhs_bits) => VariantValue::Integer((lhs + rhs as i64), max(lhs_bits, rhs_bits)),
                    VariantValue::Float(rhs, rhs_bits) => VariantValue::Float(lhs as f64 + rhs, max(lhs_bits, rhs_bits)),
                }
            },
            VariantValue::UnsignedInteger(lhs, lhs_bits) => {
                match *rhs {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(rhs) => VariantValue::UnsignedInteger((lhs as i64 + 1<<rhs as i64) as u64, lhs_bits),
                    VariantValue::Integer(rhs, rhs_bits) => VariantValue::Integer(lhs as i64 + rhs, max(lhs_bits, rhs_bits)),
                    VariantValue::UnsignedInteger(rhs, rhs_bits) => VariantValue::UnsignedInteger((lhs as i64 + rhs as i64) as u64, max(lhs_bits, rhs_bits)),
                    VariantValue::Float(rhs, rhs_bits) => VariantValue::Float(lhs as f64 + rhs, max(lhs_bits, rhs_bits)),
                }
            },
            VariantValue::Float(lhs, lhs_bits) => {
                match *rhs {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(rhs) => VariantValue::Integer((lhs as i64 + 1<<rhs as i64), lhs_bits),
                    VariantValue::Integer(rhs, rhs_bits) => VariantValue::Integer(lhs as i64 + rhs, max(lhs_bits, rhs_bits)),
                    VariantValue::UnsignedInteger(rhs, rhs_bits) => VariantValue::Integer((lhs as i64 + rhs as i64), max(lhs_bits, rhs_bits)),
                    VariantValue::Float(rhs, rhs_bits) => VariantValue::Float(lhs + rhs, max(lhs_bits, rhs_bits)),
                }
            },
        }
    }
}
impl ops::Add for VariantValue {
    type Output = VariantValue;
    #[inline]
    fn add(self, rhs: VariantValue) -> VariantValue {
        &self + &rhs
    }
}

impl<'l> ops::Sub for &'l VariantValue {
    type Output = VariantValue;
    fn sub(self, rhs: &VariantValue) -> VariantValue {
        use std::cmp::max;
        match *self {
            VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
            VariantValue::Bit(lhs) => {
                match *rhs {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(rhs) => VariantValue::UnsignedInteger((1<<lhs as i64 - 1<<rhs as i64) as u64, 0),
                    VariantValue::Integer(rhs, rhs_bits) => VariantValue::Integer(1<<lhs as i64 - rhs, rhs_bits),
                    VariantValue::UnsignedInteger(rhs, rhs_bits) => VariantValue::UnsignedInteger((1<<lhs as i64 - rhs as i64) as u64, rhs_bits),
                    VariantValue::Float(rhs, rhs_bits) => VariantValue::Float((1<<lhs) as f64 - rhs, rhs_bits),
                }
            },
            VariantValue::Integer(lhs, lhs_bits) => {
                match *rhs {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(rhs) => VariantValue::Integer((lhs - 1<<rhs as i64), lhs_bits),
                    VariantValue::Integer(rhs, rhs_bits) => VariantValue::Integer(lhs - rhs, max(lhs_bits, rhs_bits)),
                    VariantValue::UnsignedInteger(rhs, rhs_bits) => VariantValue::Integer((lhs - rhs as i64), max(lhs_bits, rhs_bits)),
                    VariantValue::Float(rhs, rhs_bits) => VariantValue::Float(lhs as f64 - rhs, max(lhs_bits, rhs_bits)),
                }
            },
            VariantValue::UnsignedInteger(lhs, lhs_bits) => {
                match *rhs {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(rhs) => VariantValue::UnsignedInteger((lhs as i64 - 1<<rhs as i64) as u64, lhs_bits),
                    VariantValue::Integer(rhs, rhs_bits) => VariantValue::Integer(lhs as i64 - rhs, max(lhs_bits, rhs_bits)),
                    VariantValue::UnsignedInteger(rhs, rhs_bits) => VariantValue::UnsignedInteger((lhs as i64 - rhs as i64) as u64, max(lhs_bits, rhs_bits)),
                    VariantValue::Float(rhs, rhs_bits) => VariantValue::Float(lhs as f64 - rhs, max(lhs_bits, rhs_bits)),
                }
            },
            VariantValue::Float(lhs, lhs_bits) => {
                match *rhs {
                    VariantValue::None | VariantValue::Raw(_) | VariantValue::Enum(_) | VariantValue::String(_) => VariantValue::None,
                    VariantValue::Bit(rhs) => VariantValue::Integer((lhs as i64 - 1<<rhs as i64), lhs_bits),
                    VariantValue::Integer(rhs, rhs_bits) => VariantValue::Integer(lhs as i64 - rhs, max(lhs_bits, rhs_bits)),
                    VariantValue::UnsignedInteger(rhs, rhs_bits) => VariantValue::Integer((lhs as i64 - rhs as i64), max(lhs_bits, rhs_bits)),
                    VariantValue::Float(rhs, rhs_bits) => VariantValue::Float(lhs - rhs, max(lhs_bits, rhs_bits)),
                }
            },
        }
    }
}
impl ops::Sub for VariantValue {
    type Output = VariantValue;
    #[inline]
    fn sub(self, rhs: VariantValue) -> VariantValue {
        &self + &rhs
    }
}

impl Default for VariantValue {
    fn default() -> VariantValue {
        VariantValue::None
    }
}

impl fmt::Display for VariantValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::VariantValue::*;
        match *self {
            None => { write!(f, "None") },
            Bit(i) => { write!(f, "1<<{}", i) },
            Integer(i,_) => { write!(f, "{}", i) },
            UnsignedInteger(i,_) => { write!(f, "{}", i) },
            Float(v,_) => { write!(f, "{}", v) },
            String(ref s) | Enum(ref s) | Raw(ref s) => { write!(f, "{}", s) },
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParameterDefinition {
    base: BasicDefinition,
    pub len: Option<String>,
    pub optional: bool,
    pub values: HashSet<String>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TypeReference {
    pub type_name: String,
    pub modifiers: Vec<TypeModifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeModifier {
    Const,
    Pointer,
    Array(VariantValue),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureSelection {
    Type(String),
    EnumItem(String),
    Command(String),
}

#[derive(Clone, Debug, Default)]
pub struct FeatureSet {
    pub name: String,
    pub api: HashSet<Api>,
    pub is_extension: bool,
    pub number: f32,
    pub protect: String,
    pub extension_number: Option<u32>,
    pub requirements: Vec<FeatureSelection>,
    pub removements: Vec<FeatureSelection>,
}

impl NamedElement for FeatureSet {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

macro_rules! definition_impl {
    ($n:ident $($l:tt)*) => {
        impl$($l)* ::std::ops::Deref for $n $($l)* {
            type Target = BasicDefinition;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }
        impl$($l)* ::std::ops::DerefMut for $n $($l)* {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
        impl$($l)* NamedElement for $n $($l)* {
            fn name(&self) -> &str {
                self.base.name.as_str()
            }
        }
    }
}

definition_impl!(IncludeDefinition);
definition_impl!(ConstantDefinition);
definition_impl!(HandleDefinition);
definition_impl!(StructDefinition);
definition_impl!(CommandDefinition);
definition_impl!(EnumGroupDefinition);
definition_impl!(EnumItemDefinition);
definition_impl!(ParameterDefinition);

impl ops::Deref for TypeDefinition {
    type Target = BasicDefinition;
    #[inline]
    fn deref(&self) -> &Self::Target {
        use self::TypeDefinition::*;
        match *self {
            Defined(ref d) => d,
            Include(ref d) => d,
            Constant(ref d) => d,
            BaseType(ref d) => d,
            BitMask(ref d) => d,
            Handle(ref d) => d,
            Enum(ref d) => d,
            FunctionPointer(ref d) => d,
            Struct(ref d) => d,
            Union(ref d) => d,
        }
    }
}
impl ops::DerefMut for TypeDefinition {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        use self::TypeDefinition::*;
        match *self {
            Defined(ref mut d) => d,
            Include(ref mut d) => d,
            Constant(ref mut d) => d,
            BaseType(ref mut d) => d,
            BitMask(ref mut d) => d,
            Handle(ref mut d) => d,
            Enum(ref mut d) => d,
            FunctionPointer(ref mut d) => d,
            Struct(ref mut d) => d,
            Union(ref mut d) => d,
        }
    }
}

#[inline]
fn find_attrib<'l>(elem_attributes: &'l Vec<OwnedAttribute>, name: &str) -> &'l str {
    match find_attrib_opt(elem_attributes, name) {
        Some(s) => s,
        None => "",
    }
}

fn find_attrib_opt<'l>(elem_attributes: &'l Vec<OwnedAttribute>, name: &str) -> Option<&'l str> {
    for attr in elem_attributes {
        if attr.name.local_name == name {
            return Some(attr.value.as_str());
        }
    }
    return None;
}

fn find_set_attrib(elem_attributes: &Vec<OwnedAttribute>, name: &str) -> HashSet<String> {
    let s = find_attrib(elem_attributes, name);
    if s.is_empty() {
        HashSet::new()
    } else {
        s.split(',').into_iter().map(|s| String::from(s)).collect::<HashSet<String>>()
    }
}

fn find_variant_attrib(elem_attributes: &Vec<OwnedAttribute>, name: &str) -> VariantValue {
    VariantValue::parse(find_attrib(elem_attributes, name))
}

fn parse_empty<R: ::std::io::Read>(parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
    use self::XmlEvent::*;
    loop {
        match parser.next() {
            Err(e) => { return Err(e.into())},
            Ok(EndElement { ref name, .. }) if name == elem_name => {
                return Ok(());
            },
            Ok(e @ StartElement { .. }) | Ok(e @ EndElement { .. }) => {
                return Err(Error::UnexpectedElement(e, parser.position()));
            }
            _ => {},
        }
    }
}

fn parse_text<R: ::std::io::Read>(parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result<String> {
    use self::XmlEvent::*;
    let mut text = String::new();
    loop {
        match parser.next() {
            Err(e) => { return Err(e.into())},
            Ok(CData(ref s)) | Ok(Characters(ref s)) | Ok(Whitespace(ref s)) =>  {
                text.push_str(s);
            },
            Ok(EndElement { ref name, .. }) if name == elem_name => {
                return Ok(text);
            },
            Ok(e @ StartElement { .. }) | Ok(e @ EndElement { .. }) => {
                return Err(Error::UnexpectedElement(e, parser.position()));
            }
            _ => {},
        }
    }
}


fn parse_ignore<R: ::std::io::Read>(parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
    use self::XmlEvent::*;
    loop {
        match parser.next() {
            Err(e) => { return Err(e.into())},
            Ok(StartElement { ref name, .. }) => {
                parse_ignore(parser, name)?;
            },
            Ok(EndElement { ref name, .. }) if name == elem_name =>  {
                return Ok(());
            },
            Ok(e @ EndElement { .. }) => {
                return Err(Error::UnexpectedElement(e, parser.position()));
            },
            _ => {},
        }
    }
}

fn parse_children<R: ::std::io::Read, F>(parser: &mut EventReader<R>, elem_name: &OwnedName, mut f: F) -> Result<()>
    where F: FnMut(&mut EventReader<R>, &OwnedName, &Vec<OwnedAttribute>) -> Result
{
    use self::XmlEvent::*;
    loop {
        match parser.next() {
            Err(e) => { return Err(e.into())},
            Ok(StartElement { ref name, ref attributes, .. }) => {
                f(parser, name, attributes)?;
            },
            Ok(EndElement { ref name, .. }) if name == elem_name => {
                return Ok(());
            },
            Ok(e @ EndElement { .. }) => {
                return Err(Error::UnexpectedElement(e, parser.position()));
            }
            _ => {},
        }
    }
}

impl RegistryData {
    pub fn new() -> RegistryData {
        let mut reg : RegistryData = Default::default();
        reg.api = Api::Vulkan;
        reg.tags.insert("ARB".into());
        reg.tags.insert("EXT".into());
        reg.tags.insert("KHR".into());
        reg.tags.insert("KHX".into());
        reg
    }

    pub fn read<R: ::std::io::Read>(read: R) -> Result<RegistryData> {
        RegistryData::parse(&mut EventReader::new(read))
    }

    pub fn parse<R: ::std::io::Read>(parser: &mut EventReader<R>) -> Result<RegistryData> {
        let mut reg = RegistryData::new();
        use self::XmlEvent::*;
        loop {
            match parser.next() {
                Err(e) => { return Err(e.into())},
                Ok(StartElement { ref name, .. }) if name.local_name == "registry" => {
                    reg.parse_registry(parser, name)?;
                    return Ok(reg);
                },
                Ok(e @ StartElement { .. }) | Ok(e @ EndElement { .. }) => {
                    return Err(Error::UnexpectedElement(e, parser.position()));
                }
                _ => {},
            }
        }
    }

    fn parse_registry<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, name: &OwnedName) -> Result<()> {
        parse_children(parser, name, |parser, name, attributes| {
            if name.local_name == "comment" {
                let mut comment = parse_text(parser, name)?;
                if let Some(p) = comment.find("--------------------") {
                    comment = comment[0..p].into();
                }
                self.comment.push_str(&comment);
            } else if name.local_name == "vendorids" {
                parse_children(parser, name, |parser, name, attributes| {
                    if name.local_name == "comment" {
                        parse_ignore(parser, name)
                    } else if name.local_name == "vendorid" {
                        self.tags.insert(find_attrib(attributes, "name").into());
                        parse_empty(parser, name)
                    } else {
                        Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
                    }
                })?;
            } else if name.local_name == "tags" {
                parse_children(parser, name, |parser, name, attributes| {
                    if name.local_name == "comment" {
                        parse_ignore(parser, name)
                    } else if name.local_name == "tag" {
                        self.tags.insert(find_attrib(attributes, "name").into());
                        parse_empty(parser, name)
                    } else {
                        Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
                    }
                })?;
            } else if name.local_name == "types" {
                parse_children(parser, name,  |parser, name, attributes| {
                    if name.local_name == "comment" {
                        parse_ignore(parser, name)
                    } else if name.local_name == "type" {
                        let ty = TypeDefinition::parse_type(parser, name, attributes)?;
                        self.types.push(ty);
                        Ok(())
                    } else {
                        Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
                    }
                })?;
            } else if name.local_name == "enums" {
                let mut enumgrp_def = EnumGroupDefinition::new(attributes);
                enumgrp_def.parse_enum_group(parser, name)?;
                self.enum_groups.push(enumgrp_def);
            } else if name.local_name == "commands" {
                parse_children(parser, name, |parser, name, attributes| {
                    if name.local_name == "comment" {
                        parse_ignore(parser, name)
                    } else if name.local_name == "command" {
                        let mut cmd_def = CommandDefinition::new(attributes);
                        cmd_def.parse_command(parser, name)?;
                        self.commands.push(cmd_def);
                        Ok(())
                    } else {
                        Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
                    }
                })?;
            } else if name.local_name == "feature" {
                let mut feature_def = FeatureSet::new(attributes);
                feature_def.is_extension = false;
                let additional_exts = feature_def.parse_feature(parser, name)?;
                if feature_def.api.contains(&self.api) {
                    self.enum_extensions.extend(additional_exts);
                    self.features.push(feature_def);
                }
            } else if name.local_name == "extensions" {
                parse_children(parser, name, |parser, name, attributes| {
                    if name.local_name == "comment" {
                        parse_ignore(parser, name)
                    } else if name.local_name == "extension" {
                        let mut extension_def = FeatureSet::new(attributes);
                        extension_def.is_extension = true;
                        let additional_exts = extension_def.parse_extension(parser, name)?;
                        if extension_def.api.contains(&self.api) {
                            self.enum_extensions.extend(additional_exts);
                            self.extensions.push(extension_def);
                        }
                        Ok(())
                    } else {
                        Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
                    }
                })?;
            } else {
                return Err(Error::ParseError(format!("unexpected {}", name), parser.position()));
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl TypeDefinition {
    fn parse_type<R: ::std::io::Read>(parser: &mut EventReader<R>, elem_name: &OwnedName, elem_attributes: &Vec<OwnedAttribute>) -> Result<TypeDefinition> {
        let category = find_attrib(elem_attributes, "category");

        if category == "" {
            let mut basic_def = BasicDefinition::new(elem_attributes);
            basic_def.parse_basic(parser, elem_name)?;
            return Ok(TypeDefinition::Defined(basic_def));
        }
        if category == "include" {
            let mut inc_def = IncludeDefinition::new(elem_attributes);
            inc_def.parse_include(parser, elem_name)?;
            return Ok(TypeDefinition::Include(inc_def));
        }
        if category == "define" {
            let mut const_def = ConstantDefinition::new(elem_attributes);
            const_def.parse_define(parser, elem_name)?;
            return Ok(TypeDefinition::Constant(const_def));
        }
        if category == "basetype" {
            let mut basic_def = BasicDefinition::new(elem_attributes);
            basic_def.parse_basic(parser, elem_name)?;
            return Ok(TypeDefinition::BaseType(basic_def));
        }
        if category == "bitmask" {
            let mut basic_def = BasicDefinition::new(elem_attributes);
            basic_def.parse_basic(parser, elem_name)?;
            return Ok(TypeDefinition::BitMask(basic_def));
        }
        if category == "handle" {
            let mut handle_def = HandleDefinition::new(elem_attributes);
            handle_def.parse_handle(parser, elem_name)?;
            return Ok(TypeDefinition::Handle(handle_def));
        }
        if category == "enum" {
            let mut basic_def = BasicDefinition::new(elem_attributes);
            basic_def.parse_basic(parser, elem_name)?;
            return Ok(TypeDefinition::Enum(basic_def));
        }
        if category == "funcpointer" {
            let mut funcptr_def = CommandDefinition::new(elem_attributes);
            funcptr_def.parse_funcpointer(parser, elem_name)?;
            return Ok(TypeDefinition::FunctionPointer(funcptr_def));
        }
        if category == "struct" {
            let mut struct_def = StructDefinition::new(elem_attributes);
            struct_def.parse_struct(parser, elem_name)?;
            return Ok(TypeDefinition::Struct(struct_def));
        }
        if category == "union" {
            let mut struct_def = StructDefinition::new(elem_attributes);
            struct_def.parse_union(parser, elem_name)?;
            return Ok(TypeDefinition::Union(struct_def));
        }

        return Err(Error::ParseError(format!("Unknown category `{}`", category), parser.position()));
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        use self::TypeDefinition::*;
        match *self {
            Defined(ref d) => d.get_requirements(),
            Include(ref d) => d.get_requirements(),
            Constant(ref d) => d.get_requirements(),
            BaseType(ref d) => d.get_requirements(),
            BitMask(ref d) => d.get_requirements(),
            Handle(ref d) => d.get_requirements(),
            Enum(ref d) => d.get_requirements(),
            FunctionPointer(ref d) => d.get_requirements(),
            Struct(ref d) => d.get_requirements(),
            Union(ref d) => d.get_requirements(),
        }
    }
}

impl BasicDefinition {

    fn new(elem_attributes: &Vec<OwnedAttribute>) -> BasicDefinition {
        BasicDefinition{
            name: find_attrib(elem_attributes, "name").into(),
            requires: find_attrib(elem_attributes, "requires").into(),
            comment: find_attrib(elem_attributes, "comment").into(),
            base_type: TypeReference::new(),
        }
    }

    fn parse_basic<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result<(String,usize,usize)> {
        use self::XmlEvent::*;
        let mut content = String::new();
        let mut end_of_type = 0;
        let mut end_of_name = 0;
        loop {
            match parser.next() {
                Err(e) => { return Err(e.into())},
                Ok(CData(ref s)) | Ok(Characters(ref s)) | Ok(Whitespace(ref s)) =>  {
                    content.push_str(s);
                },
                Ok(StartElement { ref name, .. }) if name.local_name == "type" => {
                    let text = parse_text(parser, name)?;
                    self.base_type.type_name.push_str(&text);
                    content.push_str(&text);
                    end_of_type = content.len();
                },
                Ok(StartElement { ref name, .. }) if name.local_name == "name" => {
                    let text = parse_text(parser, name)?;
                    self.name.push_str(&text);
                    content.push_str(&text);
                    end_of_name = content.len();
                },
                Ok(EndElement { ref name, .. }) if name == elem_name => {
                    return Ok((content, end_of_type, end_of_name));
                },
                Ok(e @ StartElement { .. }) | Ok(e @ EndElement { .. }) => {
                    return Err(Error::UnexpectedElement(e, parser.position()));
                }
                _ => {},
            }
        }
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res = Vec::new();
        if self.requires.len() > 0 {
            res.push(SelectionNameRef::Type(self.requires.as_str()));
        }
        if self.base_type.type_name.len() > 0 {
            res.extend(self.base_type.get_requirements());
        }
        res
    }
}

impl HandleDefinition {
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> HandleDefinition {
        let base = BasicDefinition::new(elem_attributes);
        HandleDefinition{
            base,
            parents: find_set_attrib(elem_attributes, "parent"),
            dispatchable: false,
        }
    }

    fn parse_handle<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
        self.base.parse_basic(parser, elem_name)?;
        self.dispatchable = self.base_type.type_name == "VK_DEFINE_HANDLE";
        Ok(())
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res = self.base.get_requirements();
        for p in &self.parents {
            res.push(SelectionNameRef::Type(p.as_str()));
        }
        res
    }
}


impl IncludeDefinition {
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> IncludeDefinition {
        let base = BasicDefinition::new(elem_attributes);
        IncludeDefinition{
            base,
            include: String::new(),
            relative: false,
        }
    }

    fn parse_include<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
        let (content, _, _) = self.base.parse_basic(parser, elem_name)?;
        match content.find('<') {
            Some(s) => match content.rfind('>') {
                None => { return Err(Error::ParseError(format!("Closing `>` not found in `{}`", content), parser.position())); },
                Some(e) => {
                    self.include = content[s..e].into();
                    self.relative = false;
                },
            },
            None => match content.find('"') {
                None => { return Err(Error::ParseError(format!("Opening `\"` not found in `{}`", content), parser.position())); },
                Some(s) => match content.rfind('"') {
                    None => { return Err(Error::ParseError(format!("Closing `\"` not found in `{}`", content), parser.position())); },
                    Some(e) => {
                        self.include = content[s..e].into();
                        self.relative = true;
                    },
                },
            }
        }
        Ok(())
    }
}

impl ConstantDefinition {
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> ConstantDefinition {
        let base = BasicDefinition::new(elem_attributes);
        ConstantDefinition{
            base,
            value: find_variant_attrib(elem_attributes, "value"),
        }
    }

    fn parse_define<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
        let (content, end_of_type, end_of_name) = self.base.parse_basic(parser, elem_name)?;
        if content.starts_with("//") {
            if let Some(p) = content.find(|c| c=='\n' || c=='\r') {
                self.comment.push_str(content[2..p].trim());
            }
        }
        let mut val = &content[::std::cmp::max(end_of_type, end_of_name)..];
        if let Some(p) = val.find("//") {
            val = &val[..p];
        }
        self.value = VariantValue::parse(val.into());
        Ok(())
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res = self.base.get_requirements();
        if let VariantValue::Enum(ref e) = self.value {
            res.push(SelectionNameRef::EnumItem(e.as_str()));
        }
        res
    }
}

impl CommandDefinition {
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> CommandDefinition {
        let base = BasicDefinition::new(elem_attributes);
        CommandDefinition{
            base,
            return_type: TypeReference::new(),
            parameters: Vec::new(),
            success_codes: find_set_attrib(elem_attributes, "successcodes"),
            error_codes: find_set_attrib(elem_attributes, "errorcodes"),
        }
    }

    fn parse_funcpointer<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
        use self::XmlEvent::*;
        // first parse return-type part
        match parser.next() {
            Ok(CData(ref c)) | Ok(Characters(ref c)) => {
                let mut tokens = c.split_whitespace();
                let mut return_type = match tokens.next() {
                    Some(tok) if tok=="typedef" => {
                        match tokens.next() {
                            Some(tok) => tok,
                            None => {
                                return Err(Error::ParseError("unexpected end, expected return type".to_owned(), parser.position()));
                            }
                        }
                    },
                    Some(tok) => tok,
                    None => {
                        return Err(Error::ParseError("unexpected end, expected return type".to_owned(), parser.position()));
                    }
                };
                while return_type.ends_with('*') {
                    return_type = &return_type[..return_type.len()-1];
                    self.return_type.modifiers.push(TypeModifier::Pointer);
                }
                self.return_type.type_name = return_type.to_owned();
                match tokens.next() {
                    Some(tok) if tok.starts_with("(") => {},
                    _ => {
                        return Err(Error::ParseError("unexpected end, expected \"(VKAPI_PTR\"".to_owned(), parser.position()));
                    }
                }
            },
            Ok(e)  => { return Err(Error::UnexpectedElement(e, parser.position())); },
            Err(e) => { return Err(e.into())},
        }
        // find the name of the functionpointer
        match parser.next() {
            Ok(StartElement { ref name, .. }) if name.local_name == "name" => {
                self.name = parse_text(parser, name)?;
            },
            Ok(e)  => { return Err(Error::UnexpectedElement(e, parser.position())); },
            Err(e) => { return Err(e.into())},
        }
        // parse parameters
        let mut mods : Vec<TypeModifier> = Vec::new();
        loop {
            match parser.next() {
                Ok(Whitespace(_)) => {},
                Ok(CData(c)) | Ok(Characters(c)) =>  {
                    let mut c = c.trim();
                    if c.starts_with(")(") {
                        c = &c[2..];
                    }
                    if c.starts_with("void)") {
                        // nothing to do
                    } else if let Err(l) = parse_c_modifiers(&mut mods, c) {
                        return Err(Error::ParseError(format!("unexpected modifiers '{}'", &c[l..]), parser.position()));
                    }
                },
                Ok(StartElement { ref name, .. }) if name.local_name == "type" => {
                    let type_name = parse_text(parser, name)?;
                    match parser.next() {
                        Ok(CData(c)) | Ok(Characters(c)) =>  {
                            let mut c = match parse_c_modifiers(&mut mods, c.as_str()) {
                                Ok(l) | Err(l) => &c[l..]
                            };
                            let param_name;
                            if let Some(name_end) = c.find(|c| !(c=='=' || 'a'<=c && c<='z' || 'A'<=c && c<='Z' || '0'<=c && c<='9')) {
                                param_name = &c[..name_end];
                                c = &c[name_end..];
                            } else {
                                param_name = c;
                                c="";
                            }
                            c = match parse_c_modifiers(&mut mods, c) {
                                Ok(l) | Err(l) => &c[l..]
                            };
                            let mut param = ParameterDefinition::new(&Vec::new());
                            param.base.name = param_name.to_owned();
                            param.base.base_type.type_name = type_name.to_owned();
                            param.base.base_type.modifiers = mods.clone();
                            mods.clear();
                            self.parameters.push(param);
                            if c.starts_with(')') {
                                // nothing to do
                            } else if c.starts_with(',') {
                                if let Err(l) = parse_c_modifiers(&mut mods, &c[1..]) {
                                    return Err(Error::ParseError(format!("unexpected modifiers '{}'", &c[l+1..]), parser.position()));
                                }
                            }
                        },
                        Ok(e)  => { return Err(Error::UnexpectedElement(e, parser.position())); },
                        Err(e) => { return Err(e.into())},
                    }
                },
                Ok(EndElement { ref name, .. }) if name == elem_name => {
                    return Ok(());
                },
                Ok(e)  => { return Err(Error::UnexpectedElement(e, parser.position())); },
                Err(e) => { return Err(e.into())},
            }
        }
    }

    fn parse_command<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, name: &OwnedName) -> Result {
        parse_children(parser, name, |parser, name, attributes| {
            if name.local_name == "comment" {
                parse_ignore(parser, name)
            } else if name.local_name == "proto" {
                let mut proto_def = ParameterDefinition::new(attributes);
                proto_def.parse_member(parser, name)?;
                if self.name.is_empty() {
                    self.name = proto_def.name.clone();
                }
                if self.return_type.is_empty() {
                    if proto_def.base_type.type_name != "void" || !proto_def.base_type.modifiers.is_empty() {
                        self.return_type = proto_def.base_type.clone();
                    }
                }
                Ok(())
            } else if name.local_name == "param" {
                let mut param_def = ParameterDefinition::new(attributes);
                param_def.parse_member(parser, name)?;
                self.parameters.push(param_def);
                Ok(())
            } else if name.local_name.ends_with("syncparams") {
                parse_ignore(parser, name)
            } else {
                Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
            }
        })?;
        Ok(())
    }


    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res = self.base.get_requirements();
        if !self.return_type.is_empty() {
            res.extend(self.return_type.get_requirements());
        }
        for m in &self.parameters {
            res.extend(m.get_requirements());
        }
        for c in &self.success_codes {
            res.push(SelectionNameRef::EnumItem(c.as_str()));
        }
        for c in &self.error_codes {
            res.push(SelectionNameRef::EnumItem(c.as_str()));
        }
        res
    }

    pub fn returns_status(&self) -> Option<&'static str> {
        if self.return_type.modifiers.contains(&TypeModifier::Pointer) {
            Some("util::vk_null()")
        } else if self.return_type.type_name.starts_with("PFN_") {
            Some("None")
        } else if self.return_type.type_name == "VkResult" {
            Some("VK_ERROR_INITIALIZATION_FAILED")
        } else if !self.return_type.type_name.is_empty() {
            Some("0")
        } else {
            None
        }
    }

    pub fn returns_error(&self) -> bool {
        self.return_type.modifiers.is_empty() && self.return_type.type_name == "VkResult"
    }
}

impl StructDefinition {
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> StructDefinition {
        let base = BasicDefinition::new(elem_attributes);
        StructDefinition{
            base,
            members: Vec::new(),
            returnedonly: find_attrib(elem_attributes, "returnedonly") == "true",
        }
    }

    fn parse_struct<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, name: &OwnedName) -> Result {
        parse_children(parser, name, |parser, name, attributes| {
            if name.local_name == "comment" {
                parse_ignore(parser, name)
            } else if name.local_name == "member" {
                let mut member_def = ParameterDefinition::new(attributes);
                member_def.parse_member(parser, name)?;
                self.members.push(member_def);
                Ok(())
            } else {
                Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
            }
        })?;
        Ok(())
    }

    #[inline]
    fn parse_union<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
        // this is the same
        self.parse_struct(parser, elem_name)
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res = self.base.get_requirements();
        for m in &self.members {
            res.extend(m.get_requirements());
        }
        res
    }
}

fn parse_c_modifiers(mods: &mut Vec<TypeModifier>, mut slice: &str) -> ::std::result::Result<usize,usize> {
    let mut read_size = 0usize;
    while !slice.is_empty() {
        if slice.starts_with(|c| c==' ' || c=='\n' || c=='\r' || c=='\t') {
            slice = &slice[1..];
            read_size += 1;
        } else if slice.starts_with('[') {
            match slice.find(']') {
                None => { return Err(read_size); },
                Some(p) => {
                    let dim = &slice[1..p].trim();
                    if dim.is_empty() {
                        mods.push(TypeModifier::Pointer);
                    } else {
                        mods.push(TypeModifier::Array(VariantValue::parse(dim)));
                    }
                    slice = &slice[p+1..];
                    read_size += p+1;
                },
            }
        } else if slice.starts_with('*') {
            mods.push(TypeModifier::Pointer);
            slice = &slice[1..];
            read_size += 1;
        } else if slice.starts_with("const") && !slice[5..].starts_with(char::is_alphanumeric) {
            mods.push(TypeModifier::Const);
            slice = &slice[5..];
            read_size += 5;
        } else if slice.starts_with("struct") && !slice[6..].starts_with(char::is_alphanumeric) {
            slice = &slice[6..].trim();
            read_size += 6;
        } else {
            return Err(read_size);
        }
    }
    Ok(read_size)
}

fn convert_c_modifiers(c_modifiers: &str) -> Result<Vec<TypeModifier>> {
    let mut mods : Vec<TypeModifier> = Vec::new();
    match parse_c_modifiers(&mut mods, c_modifiers) {
        Ok(_) => Ok(mods),
        Err(l) => Err(Error::GeneralError(format!("unparsable modifiers `{}`", &c_modifiers[l..]))),
    }
}

impl ParameterDefinition {
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> ParameterDefinition {
        let mut base = BasicDefinition::new(elem_attributes);
        base.base_type.type_name = find_attrib(elem_attributes, "type").into();
        ParameterDefinition{
            base,
            len: find_attrib_opt(elem_attributes, "len").map(|s| s.into()),
            optional: find_attrib(elem_attributes, "optional") == "true",
            values: find_set_attrib(elem_attributes, "values"),
        }
    }

    fn parse_member<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
        use self::XmlEvent::*;
        let mut modifiers = String::new();
        loop {
            match parser.next() {
                Err(e) => { return Err(e.into())},
                Ok(CData(ref s)) | Ok(Characters(ref s)) | Ok(Whitespace(ref s)) =>  {
                    modifiers.push_str(s.trim());
                },
                Ok(StartElement { ref name, .. }) if name.local_name == "type" => {
                    self.base_type.type_name.push_str(&parse_text(parser, name)?);
                },
                Ok(StartElement { ref name, .. }) if name.local_name == "name" => {
                    self.name.push_str(&parse_text(parser, name)?);
                },
                Ok(StartElement { ref name, .. }) if name.local_name == "enum" => {
                    modifiers.push_str(&parse_text(parser, name)?);
                },
                Ok(StartElement { ref name, .. }) if name.local_name == "comment" => {
                    self.base.comment = parse_text(parser, name)?;
                },
                Ok(EndElement { ref name, .. }) if name == elem_name => {
                    self.base_type.modifiers = convert_c_modifiers(&modifiers)?;
                    return Ok(());
                },
                Ok(e @ StartElement { .. }) | Ok(e @ EndElement { .. }) => {
                    return Err(Error::UnexpectedElement(e, parser.position()));
                }
                _ => {},
            }
        }
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res = self.base.get_requirements();
        for v in &self.values {
            res.push(SelectionNameRef::EnumItem(v.as_str()));
        }
        res
    }
}
#[derive(Copy,Clone,Eq,PartialEq,Debug,Ord,PartialOrd,Hash)]
pub enum EnumGroupType {
    Enum,
    BitMask,
    Other,
}

impl Default for EnumGroupType {
    fn default() -> EnumGroupType {
        EnumGroupType::Other
    }
}


impl EnumGroupType {
    #[inline]
    pub fn from_str(s: &str) -> EnumGroupType {
        use self::EnumGroupType::*;
        match s {
            "bitmask" => BitMask,
            "enum" => Enum,
            _ => Other,
        }
    }
}

impl fmt::Display for EnumGroupType {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::EnumGroupType::*;
        match *self {
            Enum => write!(fmt, "enum"),
            BitMask => write!(fmt, "bitmask"),
            Other => write!(fmt, "other"),
        }
    }
}

impl EnumGroupDefinition {
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> EnumGroupDefinition {
        let base = BasicDefinition::new(elem_attributes);
        EnumGroupDefinition{
            base,
            enum_type: EnumGroupType::from_str(find_attrib(elem_attributes, "type")),
            items: Vec::new(),
        }
    }

    fn parse_enum_group<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, name: &OwnedName) -> Result {
        parse_children(parser, name, |parser, name, attributes| {
            if name.local_name == "comment" {
                parse_ignore(parser, name)
            } else if name.local_name == "enum" {
                let mut item_def = EnumItemDefinition::new(attributes);
                item_def.parse_enum_item(parser, name)?;
                item_def.group = self.name.clone();
                self.items.push(item_def);
                Ok(())
            } else if name.local_name == "unused" {
                parse_empty(parser, name)
            } else {
                Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
            }
        })?;
        Ok(())
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res = self.base.get_requirements();
        for item in &self.items {
            res.push(SelectionNameRef::EnumItem(item.name()));
        }
        res
    }
}

pub const EXTENSION_BASE : u32 = 1000000000;
pub const EXTENSION_BLOCK_SIZE : u32 = 1000;

impl EnumItemDefinition {
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> EnumItemDefinition {
        let base = BasicDefinition::new(elem_attributes);
        let mut value = find_variant_attrib(elem_attributes, "value");
        if value == VariantValue::None {
            let bitpos = find_attrib(elem_attributes, "bitpos");
            if bitpos.len() > 0 {
                value = VariantValue::Bit(bitpos.parse::<u64> ().expect("parsing bitpos"));
            }
        }
        EnumItemDefinition{
            base,
            value,
            group: String::new(),
            extension: String::new(),
            offset: None,
        }
    }

    fn new_from_extension(elem_attributes: &Vec<OwnedAttribute>, ext: &FeatureSet) -> EnumItemDefinition {
        let mut def = EnumItemDefinition::new(elem_attributes);
        if def.value == VariantValue::None {
            let offset = find_attrib(elem_attributes, "offset");
            if offset.len() > 0 {
                let offset = offset.parse::<u32> ().expect("parsing offset");
                let value = EXTENSION_BASE + (ext.extension_number.unwrap_or(0u32) - 1u32) * EXTENSION_BLOCK_SIZE + offset;
                let mut value = value as i32;
                if find_attrib(elem_attributes, "dir") == "-" {
                    value = -value;
                }
                def.value = VariantValue::Integer(value as i64, 0);
            }
        }
        def
    }

    fn parse_enum_item<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, elem_name: &OwnedName) -> Result {
        self.base.parse_basic(parser, elem_name)?;
        Ok(())
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res = self.base.get_requirements();
        if let VariantValue::Enum(ref e) = self.value {
            res.push(SelectionNameRef::EnumItem(e.as_str()));
        }
        res
    }
}

impl FeatureSet{
    fn new(elem_attributes: &Vec<OwnedAttribute>) -> FeatureSet {
        let mut api : HashSet<Api> = HashSet::new();
        for apiname in find_attrib(elem_attributes, "api").split(',') {
            match Api::from_str(apiname) {
                Some(a) => { api.insert(a); }
                None => {},
            }
        }
        for apiname in find_attrib(elem_attributes, "supported").split(',') {
            match Api::from_str(apiname) {
                Some(a) => { api.insert(a); }
                None => {},
            }
        }
        let number = find_attrib(elem_attributes, "number");
        FeatureSet {
            name: find_attrib(elem_attributes, "name").into(),
            api: api,
            is_extension: false,
            number: number.parse::<f32>().unwrap_or(0.0),
            protect: find_attrib(elem_attributes, "protect").into(),
            extension_number: number.parse::<u32>().ok(),
            requirements: Vec::new(),
            removements:  Vec::new(),
        }
    }

    fn parse_feature<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, name: &OwnedName) -> Result<Vec<EnumItemDefinition>> {
        let mut enum_extensions : Vec<EnumItemDefinition> = Vec::new();
        parse_children(parser, name, |parser, name, _| {
            if name.local_name == "comment" {
                parse_ignore(parser, name)
            } else if name.local_name == "require" {
                parse_children(parser, name, |parser, name, attributes| {
                    if name.local_name == "comment" {
                        return parse_ignore(parser, name);
                    }
                    let require = if name.local_name == "type" {
                        FeatureSelection::Type(find_attrib(attributes, "name").into())
                    } else if name.local_name == "enum" {
                        let value = find_attrib(attributes, "value");
                        let offset = find_attrib(attributes, "offset");
                        let extends = find_attrib(attributes, "extends");
                        if !value.is_empty() || !offset.is_empty() || !extends.is_empty() {
                            let mut e = EnumItemDefinition::new_from_extension(attributes, self);
                            e.extension = self.name.clone();
                            e.group = extends.into();
                            enum_extensions.push(e);
                        }
                        FeatureSelection::EnumItem(find_attrib(attributes, "name").into())
                    } else if name.local_name == "command" {
                        FeatureSelection::Command(find_attrib(attributes, "name").into())
                    } else {
                        return Err(Error::ParseError(format!("unexpected {}", name), parser.position()));
                    };
                    self.requirements.push(require);
                    parse_empty(parser, name)
                })
            } else if name.local_name == "remove" {
                parse_children(parser, name, |parser, name, attributes| {
                    let remove = if name.local_name == "type" {
                        FeatureSelection::Type(find_attrib(attributes, "name").into())
                    } else if name.local_name == "enum" {
                        FeatureSelection::EnumItem(find_attrib(attributes, "name").into())
                    } else if name.local_name == "command" {
                        FeatureSelection::Command(find_attrib(attributes, "name").into())
                    } else {
                        return Err(Error::ParseError(format!("unexpected {}", name), parser.position()));
                    };
                    self.removements.push(remove);
                    parse_empty(parser, name)
                })
            } else {
                Err(Error::ParseError(format!("unexpected {}", name), parser.position()))
            }
        })?;
        Ok(enum_extensions)
    }

    #[inline]
    fn parse_extension<R: ::std::io::Read>(&mut self, parser: &mut EventReader<R>, name: &OwnedName) -> Result<Vec<EnumItemDefinition>> {
        self.parse_feature(parser, name)
    }
}

impl TypeReference {
    #[inline]
    pub fn new() -> TypeReference {
        TypeReference{
            type_name: String::new(),
            modifiers: Vec::new(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.type_name.is_empty()
    }

    pub fn get_requirements<'r>(&'r self) -> Vec<SelectionNameRef<'r>> {
        let mut res : Vec<SelectionNameRef<'r>> = Vec::new();
        if !self.type_name.is_empty() {
            res.push(SelectionNameRef::Type(self.type_name.as_str()));
        }
        for v in &self.modifiers {
            if let &TypeModifier::Array(VariantValue::Enum(ref e)) = v {
                res.push(SelectionNameRef::EnumItem(e.as_str()));
            }
        }
        res
    }
}

fn named_vec_to_name_map<'r, N: NamedElement>(data: &'r Vec<N>) -> HashMap<&'r str, &'r N> {
    let mut result : HashMap<&'r str, &'r N> = HashMap::new();
    for elem in data {
        result.insert(elem.name(), elem);
    }
    result
}

// type-flags
pub const TF_PRIMITIVE_TYPE : u32 = 0x01;
pub const TF_CONTAINS_HANDLE : u32 = 0x010000;
pub const TF_CONTAINS_REFERENCE : u32 = 0x020000;
const TF_PROPAGATION_MASK : u32 = 0xFFFF0000;

const BUILTIN_TYPE_INFOS : [(&str,usize, u32);10] = [
    // (name, size)
    ("float",        4, TF_PRIMITIVE_TYPE),
    ("double",       8, TF_PRIMITIVE_TYPE),
    ("uint8_t",      1, TF_PRIMITIVE_TYPE),
    ("uint16_t",     2, TF_PRIMITIVE_TYPE),
    ("uint32_t",     4, TF_PRIMITIVE_TYPE),
    ("uint64_t",     8, TF_PRIMITIVE_TYPE),
    ("int8_t",       1, TF_PRIMITIVE_TYPE),
    ("int16_t",      2, TF_PRIMITIVE_TYPE),
    ("int32_t",      4, TF_PRIMITIVE_TYPE),
    ("int64_t",      8, TF_PRIMITIVE_TYPE),
];

impl<'r> Into<Registry<'r>> for &'r RegistryData {
    fn into(self) -> Registry<'r> {
        Registry::new(self)
    }
}

impl<'r> Registry<'r> {
    pub fn new(data: &'r RegistryData) -> Registry<'r> {
        let mut enum_items_by_group : HashMap<&'r str, Vec<&'r EnumItemDefinition>> = HashMap::new();
        let mut enum_item_by_name : HashMap<&'r str, &'r EnumItemDefinition> = HashMap::new();
        let mut bitmask_by_groupname : HashMap<&'r str, &'r BasicDefinition> = HashMap::new();
        for group in &data.enum_groups {
            enum_items_by_group.insert(group.name(), (&group.items).into_iter().collect());
            for item in &group.items {
                enum_item_by_name.insert(item.name(), item);
            }
        }
        for typedef in &data.types {
            if let &TypeDefinition::BitMask(ref bm) = typedef {
                if !bm.requires.is_empty() {
                    bitmask_by_groupname.insert(bm.requires.as_str(), typedef);
                }
            }
        }
        for item in &data.enum_extensions {
            enum_items_by_group.entry(&item.group).or_insert(Vec::new()).push(item);
            enum_item_by_name.insert(item.name(), item);
        }
        let mut feature_map = named_vec_to_name_map(&data.features);
        feature_map.extend(named_vec_to_name_map(&data.extensions));
        Registry{
            data,
            type_by_name: named_vec_to_name_map(&data.types),
            enum_group_by_name: named_vec_to_name_map(&data.enum_groups),
            bitmask_by_groupname,
            enum_items_by_group,
            enum_item_by_name,
            command_by_name: named_vec_to_name_map(&data.commands),
            feature_by_name: feature_map,
        }
    }

    pub fn get_struct_def_size(&self, ty: &StructDefinition) -> Option<usize> {
        let mut member_size_sum = 0;
        for member in &ty.members {
            if let Some(member_size) = self.get_type_ref_size(&member.base_type) {
                member_size_sum += member_size;
            } else {
                return None;
            }
        }
        Some(member_size_sum)
    }

    pub fn get_union_def_size(&self, ty: &StructDefinition) -> Option<usize> {
        let mut member_size_max = 0;
        for member in &ty.members {
            if let Some(member_size) = self.get_type_ref_size(&member.base_type) {
                if member_size_max < member_size {
                    member_size_max = member_size;
                }
            } else {
                return None;
            }
        }
        Some(member_size_max)
    }

    pub fn get_struct_def_flags(&self, ty: &StructDefinition) -> u32 {
        let mut flags = 0;
        for member in &ty.members {
            flags |= self.get_type_ref_flags(&member.base_type) & TF_PROPAGATION_MASK;
        }
        flags
    }

    pub fn get_type_def_size(&self, ty: &TypeDefinition) -> Option<usize> {
        use self::TypeDefinition::*;
        match *ty {
            BaseType(ref d) => self.get_type_ref_size(&d.base_type),
            BitMask(_) | Enum(_) => Some(4),
            Struct(ref d) => self.get_struct_def_size(d),
            Union(ref d) => self.get_union_def_size(d),
            _ => None,
        }
    }

    pub fn get_type_def_flags(&self, ty: &TypeDefinition) -> u32 {
        use self::TypeDefinition::*;
        match *ty {
            BaseType(ref d) => self.get_type_ref_flags(&d.base_type),
            BitMask(_) | Enum(_) => TF_PRIMITIVE_TYPE,
            Handle(_) => TF_CONTAINS_HANDLE,
            Struct(ref d) | Union(ref d) => self.get_struct_def_flags(d),
            _ => 0,
        }
    }

    pub fn get_type_ref_size(&self, ty: &TypeReference) -> Option<usize> {
        let mut size = if let Some(s) = self.get_type_size(&ty.type_name) {
            s
        } else {
            return None;
        };
        for m in &ty.modifiers {
            match *m {
                TypeModifier::Const => {},
                TypeModifier::Pointer => {
                    return None;
                },
                TypeModifier::Array(ref v) => {
                    if let Some(dim) = v.get_integer(self) {
                        size *= dim as usize;
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(size)
    }

    pub fn get_type_ref_flags(&self, ty: &TypeReference) -> u32 {
        if ty.type_name == "void" && (ty.modifiers == &[TypeModifier::Pointer] || ty.modifiers == &[TypeModifier::Const, TypeModifier::Pointer]) {
            return 0;
        }
        let mut flags = self.get_type_flags(&ty.type_name);
        for m in &ty.modifiers {
            match *m {
                TypeModifier::Const => {},
                TypeModifier::Pointer => {
                    flags &= TF_PROPAGATION_MASK;
                    flags |= TF_CONTAINS_REFERENCE;
                    break;
                },
                TypeModifier::Array(ref v) => {
                    if v.get_integer(self).is_none() {
                        flags &= TF_PROPAGATION_MASK;
                        flags |= TF_CONTAINS_REFERENCE;
                        break;
                    }
                }
            }
        }
        flags
    }

    pub fn get_type_size(&self, name: &str) -> Option<usize> {
        // check builtin type
        for &(t, s, _) in &BUILTIN_TYPE_INFOS {
            if t == name {
                return Some(s);
            }
        }
        if let Some(t) = self.type_by_name.get(name) {
            self.get_type_def_size(t)
        } else {
            None
        }
    }

    pub fn get_type_flags(&self, name: &str) -> u32 {
        // check builtin type
        for &(t, _, f) in &BUILTIN_TYPE_INFOS {
            if t == name {
                return f;
            }
        }
        if let Some(t) = self.type_by_name.get(name) {
            self.get_type_def_flags(t)
        } else {
            0
        }
    }
}

impl<'r> ops::Deref for Registry<'r> {
    type Target = RegistryData;
    #[inline]
    fn deref(&self) -> &RegistryData {
        self.data
    }
}



#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SelectionNameRef<'r> {
    Type(&'r str),
    EnumItem(&'r str),
    Command(&'r str),
}

#[derive(Copy, Clone, Debug)]
pub enum SelectionItemRef<'r> {
    Type(&'r TypeDefinition),
    EnumGroup(&'r EnumGroupDefinition),
    EnumItem(&'r EnumItemDefinition),
    Command(&'r CommandDefinition),
}

impl<'r> SelectionNameRef<'r> {
    #[inline]
    pub fn from(s: &'r FeatureSelection) -> SelectionNameRef<'r> {
        match *s {
            FeatureSelection::Type(ref name) => SelectionNameRef::Type(name.as_str()),
            FeatureSelection::EnumItem(ref name) => SelectionNameRef::EnumItem(name.as_str()),
            FeatureSelection::Command(ref name) => SelectionNameRef::Command(name.as_str()),
        }
    }
}

pub struct SelectionFeatureSet<'r> {
    feature: &'r FeatureSet,
    pub items: Vec<SelectionItemRef<'r>>,
}
impl<'r> SelectionFeatureSet<'r> {
    pub fn new(feature: &'r FeatureSet) -> SelectionFeatureSet<'r> {
        SelectionFeatureSet{
            feature,
            items: Vec::new(),
        }
    }
}
impl<'r> ops::Deref for SelectionFeatureSet<'r> {
    type Target = FeatureSet;
    #[inline]
    fn deref(&self) -> &FeatureSet {
        self.feature
    }
}

pub struct Selection<'r> {
    reg: &'r Registry<'r>,
    pub selected_set: HashSet<SelectionNameRef<'r>>,
    pub enum_items_by_group: HashMap<&'r str,Vec<&'r EnumItemDefinition>>,
    pub features: Vec<SelectionFeatureSet<'r>>,
}

impl<'r> Selection<'r> {
    pub fn new(reg: &'r Registry<'r>) -> Selection<'r> {
        Selection{
            reg,
            selected_set: HashSet::new(),
            enum_items_by_group: HashMap::new(),
            features: Vec::new(),
        }
    }
    fn select_type(&mut self, sfs: &mut SelectionFeatureSet<'r>, ty: &'r TypeDefinition) -> Result {
        for req in ty.get_requirements() {
            self.select_feature(sfs, req)?;
        }

        sfs.items.push(SelectionItemRef::Type(ty));

        if let &TypeDefinition::Enum(ref e) = ty {
            if let Some(g) = self.reg.enum_group_by_name.get(e.name()) {
                sfs.items.push(SelectionItemRef::EnumGroup(g));
                for i in &g.items {
                    self.select_feature(sfs, SelectionNameRef::EnumItem(i.name()))?;
                }
            }
        }
        Ok(())
    }
    fn select_enum_item(&mut self, sfs: &mut SelectionFeatureSet<'r>, ei: &'r EnumItemDefinition) -> Result {
        if let VariantValue::Raw(ref name) = ei.value { // enum references an other enum
            if let Some(_) = self.reg.enum_item_by_name.get(name.as_str()) {
                self.select_feature(sfs, SelectionNameRef::EnumItem(name.as_str()))?
            }
        }

        sfs.items.push(SelectionItemRef::EnumItem(ei));
        self.enum_items_by_group.entry(ei.group.as_str()).or_insert(Vec::new()).push(ei);
        Ok(())
    }
    fn select_command(&mut self, sfs: &mut SelectionFeatureSet<'r>, cmd: &'r CommandDefinition) -> Result {
        for req in cmd.get_requirements() {
            self.select_feature(sfs, req)?;
        }

        sfs.items.push(SelectionItemRef::Command(cmd));
        Ok(())
    }

    pub fn ignore_feature(&mut self, f: SelectionNameRef<'r>) {
        self.selected_set.insert(f);
    }

    pub fn select_feature(&mut self, sfs: &mut SelectionFeatureSet<'r>, f: SelectionNameRef<'r>) -> Result {
        if self.selected_set.contains(&f) {
            return Ok(());
        }
        self.selected_set.insert(f);
        match f {
            SelectionNameRef::Type(name) => {
                if let Some(ty) = self.reg.type_by_name.get(name) {
                    self.select_type(sfs, ty)
                } else {
                    Err(Error::GeneralError(format!("unable to select type {}: name not known", name)))
                }
            },
            SelectionNameRef::EnumItem(name) => {
                if let Some(ei) = self.reg.enum_item_by_name.get(name) {
                    self.select_enum_item(sfs, ei)
                } else {
                    Err(Error::GeneralError(format!("unable to generate enum {}: name not known", name)))
                }
            },
            SelectionNameRef::Command(name) => {
                if let Some(cmd) = self.reg.command_by_name.get(name) {
                    self.select_command(sfs, cmd)
                } else {
                    Err(Error::GeneralError(format!("unable to generate command {}: name not known", name)))
                }
            },
        }
    }

    pub fn select_feature_set(&mut self, fs: &'r FeatureSet) -> Result {
        let mut sfs = SelectionFeatureSet::new(fs);
        for req in &fs.requirements {
            self.select_feature(&mut sfs, SelectionNameRef::from(req))?;
        }
        self.sort_feature_set(&mut sfs);
        self.features.push(sfs);
        Ok(())
    }

    fn sort_feature_set(&mut self, sfs: &mut SelectionFeatureSet) {
        sfs.items.sort_by_key(|item|{
            use self::TypeDefinition::*;
            match *item {
                SelectionItemRef::Type(&Include(_)) => 0,
                SelectionItemRef::Type(&Defined(_)) => 1,
                SelectionItemRef::Type(&Constant(_)) => 1,
                SelectionItemRef::Type(&BaseType(_)) => 1,
                SelectionItemRef::Type(&Handle(_)) => 2,
                SelectionItemRef::Type(&Enum(_) )=> 3,
                SelectionItemRef::Type(&BitMask(_)) => 3,
                SelectionItemRef::EnumItem(_) => 3,
                SelectionItemRef::EnumGroup(_) => 3,
                SelectionItemRef::Type(&FunctionPointer(_)) => 4,
                SelectionItemRef::Type(&Struct(_)) => 5,
                SelectionItemRef::Type(&Union(_)) => 5,
                SelectionItemRef::Command(_) => 6,
            }
        });
    }
}

impl<'r> ops::Deref for Selection<'r> {
    type Target = Registry<'r>;
    #[inline]
    fn deref(&self) -> &Registry<'r> {
        self.reg
    }
}
