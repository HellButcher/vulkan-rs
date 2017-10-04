use std::fmt;
use std::io;
use std::io::{Read,Write,Result};
use super::registry::*;

const NEW_LINE : &str = "\n";
const INDENT : &str = "    ";

lazy_static! {
    static ref LICENSE: Option<&'static str> = {
        match ::std::fs::File::open("LICENSE") {
            Err(_) => {
                None
            },
            Ok(mut f) => {
                unsafe {
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).expect("reading LICENSE");
                    let res : &'static str = ::std::mem::transmute(contents.as_str());
                    ::std::mem::forget(contents);
                    Some(res)
                }
            },
        }
    };
}

pub struct CodeWriteWrapper<'a> {
    /// current line io output
    line: usize,
    /// current position in line
    column: usize,

    // indention
    prefixes: Vec<*const str>,

    buf: &'a mut (Write+'a),
}

impl<'a> CodeWriteWrapper<'a> {
    pub fn new(w: &'a mut (Write+'a)) -> CodeWriteWrapper<'a> {
        CodeWriteWrapper{
            line: 0,
            column: 0,
            prefixes: Vec::new(),
            buf: w,
        }
    }
}

pub trait CodeWrite {
    fn write_str(&mut self, s: &str) -> Result<()>;
    #[inline]
    fn write_char(&mut self, c: char) -> Result<()> {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }
    fn write_fmt(&mut self, args: fmt::Arguments) -> Result<()> {
        struct Adapter<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<()>,
        }

        impl<'a, T: CodeWrite + ?Sized> fmt::Write for Adapter<'a, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_str(s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut adapter = Adapter{ inner: self, error: Ok(()) };
        match fmt::write(&mut adapter, args) {
            Ok(()) => Ok(()),
            Err(_) => {
                if adapter.error.is_err() {
                    adapter.error
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "formatter error"))
                }
            }
        }
    }

    #[inline]
    fn new_line(&mut self) -> Result<()> {
        self.write_str(NEW_LINE)
    }

    #[inline]
    fn indent<F>(&mut self, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        self.push_indent();
        let res = f(self);
        self.pop_indent();
        return res;
    }

    #[inline]
    fn prefixed<F>(&mut self, prefix: &str, f: F) -> Result<()>
        where F: FnOnce(&mut Self) -> Result<()>
    {
        self.push_prefix(prefix);
        let res = f(self);
        self.pop_prefix();
        return res;
    }

    fn push_prefix(&mut self, prefix: &str);

    fn pop_prefix(&mut self);

    #[inline]
    fn push_indent(&mut self) {
        self.push_prefix(INDENT)
    }

    #[inline]
    fn pop_indent(&mut self) {
        self.pop_prefix();
    }
}

impl<'a> CodeWriteWrapper<'a> {

    fn write_until_eol(&mut self, buf: &[u8]) -> Result<(usize, usize)> {
        let mut i= 0;
        let len = buf.len();
        while i < len {
            let c = buf[i];
            if c == '\n' as u8 {
                self.buf.write_all(&buf[0..i])?;
                let written = i;
                self.column += written;
                i += 1;
                return Ok((written, i));
            } else if c == '\r' as u8 {
                self.buf.write_all(&buf[0..i])?;
                let written = i;
                self.column += written;
                i += 1;
                if i < len && buf[i] == '\n' as u8 {
                    i += 1;
                }
                return Ok((written, i));
            } else {
                i += 1;
            }
        }
        self.buf.write_all(buf)?;
        self.column += len;
        Ok((len, len))
    }

    fn new_line(&mut self) -> Result<usize> {
        let nl = NEW_LINE.as_bytes();
        self.buf.write_all(nl)?;
        self.buf.flush()?;
        self.line += 1;
        self.column = 0;
        Ok(nl.len())
    }

    fn write_without_prefix(&mut self, mut buf: &[u8]) -> Result<usize> {
        let mut written = 0;
        while buf.len() > 0 {
            let (w, o) = self.write_until_eol(buf)?;
            written += w;
            buf = &buf[o..];
            if w == o {
                return Ok(written);
            } else {
                written += self.new_line()?;
            }
        }
        Ok(written)
    }

    fn write_with_prefix(&mut self, mut buf: &[u8]) -> Result<usize> {
        let mut written = 0;
        while buf.len() > 0 {
            if self.column == 0 {
                for i in 0..self.prefixes.len() {
                    let prefix = unsafe{&*self.prefixes[i]};
                    written += self.write_without_prefix(prefix.as_bytes())?;
                }
            }
            let (w, o) = self.write_until_eol(buf)?;
            written += w;
            buf = &buf[o..];
            if w == o {
                return Ok(written);
            } else {
                written += self.new_line()?;
            }
        }
        Ok(written)
    }
}

impl<'a> CodeWrite for CodeWriteWrapper<'a> {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        self.write_with_prefix(bytes)?;
        Ok(())
    }

    fn push_prefix(&mut self, prefix: &str) {
        self.prefixes.push(prefix);
    }

    fn pop_prefix(&mut self) {
        self.prefixes.pop();
    }

    #[inline]
    fn new_line(&mut self) -> Result<()> {
        self.new_line()?;
        Ok(())
    }
}

fn strip_snake_prefix<'l>(prefix: &str, strip_from: &'l str) -> &'l str {
    let mut check = prefix;
    let mut result = strip_from;
    while let Some(p) = check.find('_') {
        let part = &check[..p+1];
        check = &check[p+1..];
        if part.len() > 0 && result.starts_with(part) {
            result = &result[part.len()..];
        } else {
            return result;
        }
    }
    if check.len() > 0 && result.starts_with(check) {
        result = &result[check.len()..];
    }
    return result;
}

fn strip_vendor_suffix<'l>(strip_from: &'l str, reg: &Registry) -> (&'l str, &'l str) {
    for sfx in &reg.tags {
        if strip_from.ends_with(sfx) {
            let pos = strip_from.len()-sfx.len();
            let stripped = &strip_from[..pos];
            let vendor = &strip_from[pos..];
            if stripped.ends_with('_') || stripped.ends_with(char::is_lowercase) {
                return (&stripped[..stripped.len()-1], vendor);
            } else {
                return (stripped, vendor)
            }
        }
    }
    return (strip_from, "");
}

pub trait CommonGeneratorWriter {

    fn write_begin_file<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection) -> Result<()> {
        write!(w, "// DO NOT EDIT!\n")?;
        write!(w, "// This file is generated from the Khronos Vulkan XML API Registry.\n\n")?;

        if let Some(license) = *LICENSE {
            write!(w, "/*\n")?;
            w.prefixed("**  ", |w|{
                write!(w, "{}\n", license)?;
                Ok(())
            })?;
            write!(w, "*/\n\n")?;
        }

        self.write_header(w, sel)
    }

    fn write_end_file<W: CodeWrite>(&mut self, w: &mut W, _: &Selection) -> Result<()> {
        write!(w, "\n")?;
        Ok(())
    }

    fn write_header<W: CodeWrite>(&mut self, _: &mut W, _: &Selection) -> Result<()> {
        Ok(())
    }
}

fn write_sub_type_definition<W: CodeWrite, G: GeneratorWriter>(s: &mut G, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &TypeDefinition) -> Result<()> {
    use self::TypeDefinition::*;
    match *ty {
        Defined(ref d) => {
            s.write_defined_definition(w, sel, f, d)
        },
        Include(ref d) => {
            s.write_include_definition(w, sel, f, d)
        },
        Constant(ref d) => {
            s.write_constant_definition(w, sel, f, d)
        },
        BaseType(ref d) => {
            s.write_basetype_definition(w, sel, f, d)
        },
        BitMask(ref d) => {
            s.write_bitmask_definition(w, sel, f, d)
        },
        Enum(ref d) => {
            s.write_enum_definition(w, sel, f, d)
        },
        Handle(ref d) => {
            s.write_handle_definition(w, sel, f, d)
        },
        FunctionPointer(ref d) => {
            s.write_funcpointer_definition(w, sel, f, d)
        },
        Struct(ref d) => {
            s.write_struct_definition(w, sel, f, d)
        },
        Union(ref d) => {
            s.write_union_definition(w, sel, f, d)
        },
    }
}

pub trait GeneratorWriter: CommonGeneratorWriter {

    fn write_begin_feature<W: CodeWrite>(&mut self, w: &mut W, _: &Selection, f: &FeatureSet) -> Result<()> {
        write!(w, "\n// {}\n\n", f.name)?;
        Ok(())
    }

    fn write_end_feature<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet) -> Result<()> {
        Ok(())
    }

    fn write_type_definition<W: CodeWrite>(&mut self, w: &mut W, sel: &Selection, f: &FeatureSet, ty: &TypeDefinition) -> Result<()> {
        use self::TypeDefinition::*;
        match *ty {
            Defined(ref d) => {
                self.write_defined_definition(w, sel, f, d)
            },
            Include(ref d) => {
                self.write_include_definition(w, sel, f, d)
            },
            Constant(ref d) => {
                self.write_constant_definition(w, sel, f, d)
            },
            BaseType(ref d) => {
                self.write_basetype_definition(w, sel, f, d)
            },
            BitMask(ref d) => {
                self.write_bitmask_definition(w, sel, f, d)
            },
            Enum(ref d) => {
                self.write_enum_definition(w, sel, f, d)
            },
            Handle(ref d) => {
                self.write_handle_definition(w, sel, f, d)
            },
            FunctionPointer(ref d) => {
                self.write_funcpointer_definition(w, sel, f, d)
            },
            Struct(ref d) => {
                self.write_struct_definition(w, sel, f, d)
            },
            Union(ref d) => {
                self.write_union_definition(w, sel, f, d)
            },
        }
    }

    fn write_defined_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &BasicDefinition) -> Result<()> {
        Ok(())
    }

    fn write_include_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &IncludeDefinition) -> Result<()> {
        Ok(())
    }

    fn write_constant_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &ConstantDefinition) -> Result<()> {
        Ok(())
    }

    fn write_basetype_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &BasicDefinition) -> Result<()> {
        Ok(())
    }

    fn write_bitmask_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &BasicDefinition) -> Result<()> {
        Ok(())
    }

    fn write_enum_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &BasicDefinition) -> Result<()> {
        Ok(())
    }

    fn write_handle_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &HandleDefinition) -> Result<()> {
        Ok(())
    }

    fn write_funcpointer_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &CommandDefinition) -> Result<()> {
        Ok(())
    }

    fn write_struct_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &StructDefinition) -> Result<()> {
        Ok(())
    }

    fn write_union_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &StructDefinition) -> Result<()> {
        Ok(())
    }

    fn write_enum_group_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &EnumGroupDefinition) -> Result<()> {
        Ok(())
    }

    fn write_enum_item_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &EnumItemDefinition) -> Result<()> {
        Ok(())
    }

    fn write_command_definition<W: CodeWrite>(&mut self, _: &mut W, _: &Selection, _: &FeatureSet, _: &CommandDefinition) -> Result<()> {
        Ok(())
    }

}

pub trait Visitor {
    fn visit_begin(&mut self, _: &Selection) -> Result<()> {
        Ok(())
    }
    fn visit_end(&mut self, _: &Selection) -> Result<()> {
        Ok(())
    }
    fn visit_begin_feature(&mut self, _: &Selection, _: &SelectionFeatureSet) -> Result<()> {
        Ok(())
    }
    fn visit_end_feature(&mut self, _: &Selection, _: &SelectionFeatureSet) -> Result<()> {
        Ok(())
    }
    fn visit_type_definition(&mut self, _: &Selection, _: &SelectionFeatureSet, _: &TypeDefinition) -> Result<()> {
        Ok(())
    }
    fn visit_enum_group_definition(&mut self, _: &Selection, _: &SelectionFeatureSet, _: &EnumGroupDefinition) -> Result<()> {
        Ok(())
    }
    fn visit_enum_item_definition(&mut self, _: &Selection, _: &SelectionFeatureSet, _: &EnumItemDefinition) -> Result<()> {
        Ok(())
    }
    fn visit_command_definition(&mut self, _: &Selection, _: &SelectionFeatureSet, _: &CommandDefinition) -> Result<()> {
        Ok(())
    }

    fn visit_selection(&mut self, sel: &Selection) -> io::Result<()> {
        self.visit_begin(sel)?;
        for f in &sel.features {
            self.visit_selection_feature_set(sel, f)?;
        }
        self.visit_end(sel)
    }

    fn visit_selection_feature_set(&mut self, sel: &Selection, f: &SelectionFeatureSet) -> io::Result<()> {
        self.visit_begin_feature(sel, f)?;
        for item in &f.items {
            self.visit_selection_item(sel, f, *item)?;
        }
        self.visit_end_feature(sel, f)
    }

    fn visit_selection_item(&mut self, sel: &Selection, f: &SelectionFeatureSet, item: SelectionItemRef) -> io::Result<()> {
        match item {
            SelectionItemRef::Type(ty) => self.visit_type_definition(sel, f, ty),
            SelectionItemRef::EnumGroup(eg) => self.visit_enum_group_definition(sel, f, eg),
            SelectionItemRef::EnumItem(ei) => self.visit_enum_item_definition(sel, f, ei),
            SelectionItemRef::Command(cmd) => self.visit_command_definition(sel, f, cmd),
        }
    }
}

pub struct OututGenerator<'l, 'w, G: GeneratorWriter+'l, W: CodeWrite+'w> {
    g: &'l mut G,
    w: &'w mut W,
}

impl<'l, 'w, G: GeneratorWriter+'l, W: CodeWrite+'w> OututGenerator<'l, 'w, G, W> {
    pub fn new(g: &'l mut G, w: &'w mut W) -> OututGenerator<'l, 'w, G, W> {
        OututGenerator{g, w: w}
    }
}

impl<'l, 'w, G: GeneratorWriter+'l, W: CodeWrite+'w> Visitor for OututGenerator<'l, 'w, G, W> {
    fn visit_begin(&mut self, sel: &Selection) -> Result<()> {
        self.g.write_begin_file(self.w, sel)
    }
    fn visit_end(&mut self, sel: &Selection) -> Result<()> {
        self.g.write_end_file(self.w, sel)
    }
    fn visit_begin_feature(&mut self, sel: &Selection, f: &SelectionFeatureSet) -> Result<()> {
        self.g.write_begin_feature(self.w, sel, f)
    }
    fn visit_end_feature(&mut self, sel: &Selection, f: &SelectionFeatureSet) -> Result<()> {
        self.g.write_end_feature(self.w, sel, f)
    }
    fn visit_type_definition(&mut self, sel: &Selection, f: &SelectionFeatureSet, ty: &TypeDefinition) -> Result<()> {
        self.g.write_type_definition(self.w, sel, f, ty)
    }
    fn visit_enum_group_definition(&mut self, sel: &Selection, f: &SelectionFeatureSet, eg: &EnumGroupDefinition) -> Result<()> {
        self.g.write_enum_group_definition(self.w, sel, f, eg)
    }
    fn visit_enum_item_definition(&mut self, sel: &Selection, f: &SelectionFeatureSet, ei: &EnumItemDefinition) -> Result<()> {
        self.g.write_enum_item_definition(self.w, sel, f, ei)
    }
    fn visit_command_definition(&mut self, sel: &Selection, f: &SelectionFeatureSet, cmd: &CommandDefinition) -> Result<()> {
        self.g.write_command_definition(self.w, sel, f, cmd)
    }
}

impl<'r> Selection<'r> {
    pub fn visit(&self, gen: &mut Visitor) -> io::Result<()> {
        gen.visit_selection(self)
    }
    pub fn generate<G: GeneratorWriter,W: Write>(&self, g: &mut G, w: &mut W) -> io::Result<()> {
        self.generate_code(g, &mut CodeWriteWrapper::new(w))
    }
    pub fn generate_code<G: GeneratorWriter,W: CodeWrite>(&self, g: &mut G, w: &mut W) -> io::Result<()> {
        self.visit(&mut OututGenerator::new(g, w))
    }
}


#[derive(Copy,Clone,Default,Eq,PartialEq,Debug)]
pub struct CodeStyle {
    pub snake_case_fields: bool,
    pub snake_case_commands: bool,
}

mod rust;
pub mod rust_types;
pub mod rust_cmds;
pub mod rust_alias;
