import os, io, sys, re
from .utils import CodeGenerator, camel_to_snake_case, camel_to_screaming_snake_case, delegate_type, which
from .registry import BaseTypeElem, Member, Typed, TypeRef, DispatchTable

PREDEFINED_TYPES = {
    'int8_t': 'i8',
    'uint8_t': 'u8',
    'int16_t': 'i16',
    'uint16_t': 'u16',
    'int32_t': 'i32',
    'uint32_t': 'u32',
    'int64_t': 'i64',
    'uint64_t': 'u64',
    'float': 'f32',
    'double': 'f64',
    'void': 'c_void',
    'char': 'c_char',
    'int': 'c_int',
    'size_t': 'usize',
}

PREDEFINED_UTILS = {
    'VK_MAKE_VERSION': ('vk_make_version', 'u32'),
}

IGNORED = set([
    'VK_TRUE', 'VK_FALSE', 'VK_NULL_HANDLE'
])

RESERVED_KEYWORDS = set([
    '_', 'abstract', 'alignof', 'as', 'become',
    'box', 'break', 'const', 'continue', 'crate',
    'do', 'else', 'enum', 'extern', 'false',
    'final', 'fn', 'for', 'if', 'impl',
    'in', 'let', 'loop', 'macro', 'match',
    'mod', 'move', 'mut', 'offsetof', 'override',
    'priv', 'proc', 'pub', 'pure', 'ref',
    'return', 'Self', 'self', 'sizeof', 'static',
    'struct', 'super', 'trait', 'true', 'type',
    'typeof', 'unsafe', 'unsized', 'use', 'virtual',
    'where', 'while', 'yield', 
])

INCLUDE_TO_MODULE = {
    'X11/Xlib.h':                 'wsi::xlib',
    'X11/extensions/Xrandr.h':    'wsi::xlib',
    'android/native_window.h':    'wsi::android',
    'mir_toolkit/client_types.h': 'wsi::mir',
    'wayland-client.h':           'wsi::wayland',
    'windows.h':                  'wsi::win32',
    'xcb/xcb.h':                  'wsi::xcb',
}


STRUCT_TYPES = set([BaseTypeElem.STRUCT, BaseTypeElem.UNION])

_rustfmt_config = None
_rustfmt_exe = None

def rustfmt(filename):
    global _rustfmt_config, _rustfmt_exe
    if _rustfmt_exe is None:
        _rustfmt_exe = which('rustfmt')
        if _rustfmt_exe is None:
            _rustfmt_exe = False
            print('`rustfmt` not found in PATH. The file `%s` will not be formatted!' % filename)
            return
    if _rustfmt_exe is False:
        return
    import subprocess
    rustfmt_cmdline = [_rustfmt_exe]
    if _rustfmt_config is None:
        if os.path.isfile('./rustfmt.toml'):
            _rustfmt_config = './rustfmt.toml'
        else:
            _rustfmt_config = False
    if _rustfmt_config is not False:
        rustfmt_cmdline.append('--config-path')
        rustfmt_cmdline.append(str(_rustfmt_config))
    rustfmt_cmdline.append(filename)
    res = subprocess.run(rustfmt_cmdline)
    if res.returncode == 3:
        print('`rustfmt` was unable to format `%s` properly, but the code was valid: the error was ignored' % filename)
    elif res.returncode != 0:
        raise subprocess.CalledProcessError('Command %s returned %d', res.args, res.returncode)

def _lifetime_diamond(lifetimes, with_subtyping=False):
    if not lifetimes:
        return ''
    else:
        lifetime_args = list()
        prev = None
        for l in reversed(sorted(lifetimes)):
            if prev is not None and with_subtyping:
                arg = '\'%s: \'%s' % (l, prev)
            else:
                arg = '\'%s' % l
            lifetime_args.append(arg)
            prev = l
        return '<%s>' % ','.join(lifetime_args)

class RustCodeGenerator(CodeGenerator):
    def __init__(self, out, *args, **kwargs):
        if isinstance(out, str):
            self._filename = out
            self._old_out = None
        else:
            from tempfile import NamedTemporaryFile
            self._old_out = out
            out = NamedTemporaryFile('w', delete=False)
            self._filename = out.name
        super(RustCodeGenerator, self).__init__(out, *args, **kwargs)
    def close(self):
        super(RustCodeGenerator, self).close()
        self.out.close()
        rustfmt(self._filename)
        if self._old_out is not None:
            self.out = self._old_out
            with open(self._filename, "r") as input:
                for line in iter(input.readlines()):
                    self.out.write(line)
            os.unlink(self._filename)
            self._old_out = None


class ImportGenerator(CodeGenerator):
    def __init__(self, *args, **kwargs):
        super(ImportGenerator, self).__init__(*args, **kwargs)
        self.imports = set()
        self._target_out = self.out
        self.out = io.StringIO()
    
    def close(self):
        if self._target_out is not None:
            contents = self.out.getvalue()
            self.out.close()
            self.out = self._target_out
            self._target_out = None
            if len(self.imports) > 0:
                for imp in sorted(self.imports):
                    self('use ', imp, ';').nl()
                self.nl()
            self.imports = None
            self.out.write(contents)
        super(ImportGenerator, self).close()

_RE_SUB_ = re.compile(r'\-[0-9]+$')
class RustGenerator:
    def __init__(self, registry):
        self.registry = registry
        self.target = 'src'
        self._import_generator = None

    def manage_imports(self, gen):
        if self._import_generator is not None and self._import_generator.imports is not None:
            raise ValueError('canonly open one import-generator at a time')
        self._import_generator = ImportGenerator(gen)
        return self._import_generator

    def add_import(self, imp):
        if self._import_generator is not None and self._import_generator.imports is not None:
            self._import_generator.imports.add(imp)
        return imp

    def rust_value(self, value, **kwargs):
        if value is None:
            return 'None', 'Option<c_void>'
        if value.startswith('(') and value.endswith(')'):
            value = value[1:-1]
        if value.startswith('"'):
            return '"%s\\0"' % value[1:-1], '&str'
        if value[0].isalpha():
            item = self.registry.enum_items[value]
            name = self.rust_enum_item_name(item)
            if item.enum_group.type is None:
                _, ty = self.rust_enum_item_value(item, **kwargs)
                if not kwargs.get('no_imports', False):
                    self.add_import('enums')
                return 'enums::' + name, ty
            else:
                return item.enum_group.name + '::' + name, item.enum_group.name
        ty = 'u32'
        sub = _RE_SUB_.search(value)
        if sub is not None:
            sub = sub.group(0)
            value = value[:-len(sub)]
        else:
            sub = ''
        if value.endswith('f'):
            ty = 'f32'
            value = value + '32'
        elif value.endswith('ULL'):
            ty = 'u64'
            value = value[:-3] + 'u64'
        elif value.endswith('LL'):
            ty = 'i64'
            value = value[:-2] + 'i64'
        elif value.endswith('U'):
            ty = 'u32'
            value = value[:-1] + 'u32'
        if value.startswith('~'):
            value = '!' + value[1:]
        return value+sub, ty

    def rust_dimension_value(self, value, **kwargs):
        value, ty = self.rust_value(value, **kwargs)
        if value is not None and value[0].isalpha() and ty != 'usize':
            # requires a cast
            return '%s as usize' % value
        return value

    def rust_enum_item_value(self, item, **kwargs):
        if item.bitpos is not None:
            return '1<<%s' % item.bitpos, 'u32'
        v, t = self.rust_value(item.value, **kwargs)
        if item.enum_group.type is not None:
            if v.startswith('-'):
                i = int(v[1:])
                v = '!%s' % (i-1)
            return v, 'u32'
        return v, t

    def rust_enum_item_name(self, item):
        if item.enum_group.type is None:
            return item.name
        name = item.shortname
        if name[0].isnumeric() or name in RESERVED_KEYWORDS:
            if item.enum_group.type == 'bitmask':
                if name.endswith('_BIT'):
                    name = name[:-4]
                name = 'BIT_' + name
            else:
                name = 'E_' + name
        return name

    def rust_member_name(self, member):
        if isinstance(member, str):
            name = member
        else:
            name = member.name
        if name in RESERVED_KEYWORDS:
            name = 'e' + name[0].upper() + name[1:]
        return name
    rust_param_name = rust_member_name

    def rust_member_function(self, member, keyword_prefix='_'):
        if isinstance(member, str):
            name = member
        else:
            name = member.name
        name = camel_to_snake_case(name)
        if name.startswith('pp_'):
            name = name[3:]
        elif name.startswith('p_'):
            name = name[2:]
        if name in RESERVED_KEYWORDS:
            name = keyword_prefix + name
        return name
    rust_param_name = rust_member_name

    def rust_composed_lifetimes(self, ty, **kwargs):
        lifetimes = set()
        for m in ty.members:
            if m.name == 'pNext' and m.type == TypeRef.VOID_PTR:
                lifetimes.add('l')
            else:
                (_, _, _, member_lifetimes) = self.rust_type_details(m, no_imports=True, **kwargs)
                lifetimes.update(member_lifetimes)
        return lifetimes

    def rust_type_details(self, ty, **kwargs):
        if isinstance(ty, str):
            ty = self.registry.types[ty]
        elif isinstance(ty, Member):
            m = ty
            ty = m.type
            if 'optional' not in kwargs:
                kwargs['optional'] = m.optional
            if 'len' not in kwargs:
                kwargs['len'] = m.len
        as_raw_conv = False
        lifetimes = set()
        if isinstance(ty, BaseTypeElem):
            type_name = ty.name
            raw_type_name = None
            if type_name in PREDEFINED_TYPES:
                type_name = PREDEFINED_TYPES[type_name]
                return (type_name, type_name, as_raw_conv, lifetimes)
            elif type_name in RESERVED_KEYWORDS:
                type_name = 'T' + type_name
            if ty.name == 'VkBool32':
                as_raw_conv = True
                raw_type_name = 'VkBool32'
                type_name = 'bool'
            elif ty.__class__ is BaseTypeElem.PROVIDED:
                self.add_import('platform::*')
                if len(ty.requires) == 1:
                    mod = INCLUDE_TO_MODULE.get(ty.requires[0], None)
                    if mod is not None:
                        type_name = mod + '::' + type_name
            elif ty.__class__ is BaseTypeElem.FUNCTIONPOINTER and kwargs.pop('optional', None):
                type_name = 'Option<%s>' % type_name
            elif ty.__class__ in STRUCT_TYPES:
                if not kwargs.get('as_param', None):
                    lifetimes.update(self.rust_composed_lifetimes(ty))
                    type_name += _lifetime_diamond(lifetimes)
            elif ty.__class__ is BaseTypeElem.HANDLE:
                raw_type_name = ty.non_dispatchable and 'u64' or 'usize'
                as_raw_conv = True
                if not kwargs.get('as_param', None):
                    lifetimes.add('h')
                    type_name += '<\'h>'
                if kwargs.pop('optional', None):
                    type_name = 'Option<%s>' % type_name
            if raw_type_name is None:
                raw_type_name = type_name
            return (type_name, raw_type_name, as_raw_conv, lifetimes)
        elif not isinstance(ty, TypeRef):
            raise ValueError('unable to hadle arg', ty)
        elif ty.is_named():
            return self.rust_type_details(ty.name, **kwargs)
        elif ty.is_function():
            params = ', '.join([self.rust_raw_type(p.type, as_param=True) for p in ty.arg.params])
            res = ''
            if ty.arg.returns != TypeRef.VOID:
                res = ' -> %s' % self.rust_raw_type(ty.arg.returns, as_param=True, optional=True)
            type_name = 'extern "system" fn (%s)%s' % (params, res)
            if kwargs.pop('optional', None):
                type_name = 'Option<%s>' % type_name
            return (type_name, type_name, as_raw_conv, lifetimes)
        elif ty.is_ptr():
            length = kwargs.pop('len', None)
            opt = kwargs.pop('optional', None)
            if ty.is_const():
                arg = ty.arg.arg
                mut = ''
            else:
                arg = ty.arg
                mut = 'mut '
            (type_name, raw_typename, as_raw_conv, arg_lifetimes) = self.rust_type_details(arg, **kwargs)
            lifetimes.update(arg_lifetimes)
            raw_typename = '*%s%s' % (mut or 'const ', raw_typename)
            if not mut and arg == TypeRef.CHAR and length == 'null-terminated':
                if not kwargs.get('as_param', None):
                    lt = '\'l '
                    lifetimes.add('l')
                else:
                    lt = ''
                type_name = '&%sAsRef<CStr>' % lt
                lifetimes.add('l')
                as_raw_conv = True
                if opt:
                    type_name = 'Option<%s>' % type_name
            elif length:
                if arg == TypeRef.VOID:
                    type_name = 'u8'
                if not kwargs.get('as_param', None):
                    lt = '\'l '
                    lifetimes.add('l')
                else:
                    lt = ''
                type_name = '&%s%s[%s]' % (lt, mut, type_name)
                lifetimes.add('l')
                as_raw_conv = True
            elif arg == TypeRef.VOID or (arg.is_named() and arg.resolved_type.__class__ is BaseTypeElem.PROVIDED):
                type_name = raw_typename
            else:
                if not kwargs.get('as_param', None):
                    lt = '\'l '
                    lifetimes.add('l')
                else:
                    lt = ''
                type_name = '&%s%s%s' % (lt, mut, type_name)
                if opt:
                    type_name = 'Option<%s>' % type_name
                    as_raw_conv = True
            return (type_name, raw_typename, as_raw_conv, lifetimes)
        elif ty.is_array():
            if ty.is_const():
                arg = ty.arg.arg
            else:
                arg = ty.arg
            (type_name, raw_typename, as_raw_conv, arg_lifetimes) = self.rust_type_details(arg, **kwargs)
            lifetimes.update(arg_lifetimes)
            dim = self.rust_dimension_value(ty.dim, no_imports=kwargs.get('no_imports', False))
            type_name = '[%s; %s]' % (type_name, dim)
            raw_typename = '[%s; %s]' % (raw_typename, dim)
            return (type_name, raw_typename, as_raw_conv, lifetimes)
        raise ValueError('unable to hadle arg', ty)

    def rust_raw_type(self, ty, **kwargs):
        (_typename, raw_typename, _as_raw_conv, _lifetimes) = self.rust_type_details(ty, **kwargs)
        return raw_typename

    def rust_safe_type(self, ty, **kwargs):
        (typename, _raw_typename, _as_raw_conv, _lifetimes) = self.rust_type_details(ty, **kwargs)
        return typename

    def rust_param_as_raw(self, ty, declname=None, **kwargs):
        if isinstance(ty, Member):
            if declname is None:
                declname = self.rust_param_name(ty)
            if ty.is_out and (ty.len_for or ty is getattr(ty.container, 'out_param', None)):
                if ty.len:
                    declname = '%s.as_mut_ptr()' % declname
                else:
                    declname = '&mut %s' % declname
        (typename, raw_typename, as_raw_conv, _lifetimes) = self.rust_type_details(ty, no_imports=True, **kwargs)
        if as_raw_conv:
            if '&' in declname:
                declname = '(%s)' %declname
            declname = '%s.as_raw()' % declname
            if 'u8' in typename and 'c_void' in raw_typename:
                declname = '%s as %s' % (declname,raw_typename)
        return declname

    def is_public(self, ty):
        if isinstance(ty, str):
            ty = self.registry.types[ty]
        if isinstance(ty, Member):
            m = ty
            ty = m.type
            if m.values:
                return False
            if m.len_for:
                return False
            if m.len:
                return False
        if isinstance(ty, BaseTypeElem):
            return True
        if not isinstance(ty, TypeRef):
            raise ValueError('unable to hadle arg', ty)
        if ty.__class__ == TypeRef.POINTER:
            if ty.arg.__class__ == TypeRef.CONST:
                return self.is_public(ty.arg.arg)
            return self.is_public(ty.arg)
        if ty == TypeRef.VOID:
            return False
        return True
    
    def _generate_docs(self, obj, gen, short=False):
        docs = obj.docs
        if docs is None:
            return
        gen.nl()
        for line in docs:
            if short and line == '':
                return
            gen('/// ', line).nl()

    def _generate_feature_comment(self, feature, gen):
        gen.nl()
        gen('// feature: ', feature.name).nl()

    def _generate_feature_comment_nonconsecutive(self, feature, gen):
        last = getattr(gen, '_last_feature', None)
        if last is feature:
            return
        gen._last_feature = feature
        self._generate_feature_comment(feature, gen)

    def _generate_feature_protect(self, feature, gen):
        if feature.is_extension:
            gen('#[cfg(feature = "', feature.name, '")]').nl()
        if feature.protect:
            gen('#[cfg(feature = "', feature.protect, '")]').nl()
    
    def _is_feature_protect(self, feature):
        return feature.is_extension or feature.protect is not None

    def generate_all(self):
        self.generate_enums()
        self.generate_types()
        self.generate_protos()
        self.generate_dispatch_table()
        self.generate_dispatch_commands()
        self.generate_prelude()

    def generate_types(self, file=None):
        if file is None:
            file = os.path.join(self.target,'types.rs')
        reg = self.registry
        with RustCodeGenerator(file) as gen:
            gen("/* GENERATED FILE */").nl()
            gen.nl()
            gen('#![allow(non_snake_case)]').nl()
            gen.nl()
            gen('#[path = "types_impl.rs"]').nl()
            gen('pub mod types_impl;').nl()
            gen.nl()
            with self.manage_imports(gen) as gen:
                self.add_import('AsRaw')
                self.add_import('Struct')
                for feature in reg.features:
                    with gen.open_nonempty() as nonempty_gen:
                        self._generate_feature_comment(feature, nonempty_gen)
                        for ty in feature.types:
                            self._generate_type(ty, gen)

    def _generate_type(self, ty, gen):
        if ty.name in IGNORED:
            return
        delegate_type(self, '_generate_type_', ty, gen)

    def _generate_type_basetype(self, ty, gen):
        self._generate_docs(ty, gen)
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('pub type ', ty.name, ' = ', self.rust_raw_type(ty.type), ';').nl()

    def _generate_type_bitmask(self, ty, gen):
        if len(ty.requires) == 1:
            enumtype = ty.requires[0]
        else:
            enumtype = 'VkFlags'
        self._generate_docs(ty, gen)
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('pub type ', ty.name, ' = ', enumtype, ';').nl()

    def _generate_type_enum(self, ty, gen):
        self._generate_docs(ty, gen, short=True)
        self._generate_feature_protect(ty.requiering_feature, gen)
        if ty.name != ty.group.name:
            gen('pub use enums::', ty.group.name, ' as ', ty.name, ';').nl()
        else:
            gen('pub use enums::', ty.group.name, ';').nl()

    def _generate_type_handle(self, ty, gen):
        base = ty.non_dispatchable and 'VkNonDispatchableHandle' or 'VkDispatchableHandle'
        self.add_import('utils::%s' % base)
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('#[doc(hidden)]').nl()
        gen('#[derive(Copy,Clone)]').nl()
        gen('pub enum ', ty.name, '__ {}').nl()
        self._generate_docs(ty, gen)
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('pub type ', ty.name, '<\'l> = ', base, '<\'l,', ty.name, '__>;').nl()
    
    def _generate_type_funcpointer(self, ty, gen):
        self._generate_docs(ty, gen)
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('#[allow(non_camel_case_types)]').nl()
        gen('pub type ', ty.name, ' = ', self.rust_raw_type(ty.type), ';').nl()

    def _generate_type_struct(self, ty, gen):
        members = []
        lifetimes = set()
        has_p_next = False
        for i, member in enumerate(ty.members):
            name = self.rust_member_name(member)
            (typename, raw_typename, as_raw_conv, member_lifetimes) = self.rust_type_details(member)
            lifetimes.update(member_lifetimes)
            hidden = False
            if i == 0 and member.name == 'sType' and typename == 'VkStructureType' and member.values:
                hidden = True
            elif i == 1 and member.name == 'pNext' and member.type == TypeRef.VOID_PTR:
                self.add_import('std::cell::Cell')
                typename = raw_typename = 'Cell<%s>' % raw_typename
                hidden = True
                has_p_next = True
                lifetimes.add('l')
            elif member.len_for:
                hidden = 'readonly'
            typename = typename.replace(' mut ', ' ')
            members.append({
                'obj': member,
                'name': name,
                'typename': typename,
                'raw_typename': raw_typename,
                'as_raw_conv': as_raw_conv, 
                'lifetimes': member_lifetimes,
                'hidden': hidden,
            })
        self._generate_docs(ty, gen)
        gen('#[repr(C)]').nl()
        if not lifetimes:
            gen('#[derive(Copy,Clone)]').nl()
        self._generate_feature_protect(ty.requiering_feature, gen)
        lifetime_params = _lifetime_diamond(lifetimes, with_subtyping=True)
        lifetime_args = _lifetime_diamond(lifetimes)
        gen('pub struct ', ty.name, lifetime_params, ' {').nl()
        used_lifetimes = set()
        with gen.open_indention():
            for member in members:
                if not member['hidden'] and not member['as_raw_conv'] and self.is_public(member['obj']):
                    used_lifetimes.update(member['lifetimes'])
                    gen('pub ', member['name'], ' : ', member['typename'], ',').nl()
                else:
                    gen(member['name'], ' : ', member['raw_typename'], ',').nl()
            unused_lifetimes = lifetimes - used_lifetimes
            if unused_lifetimes:
                gen('_p: ::std::marker::PhantomData<(', ','.join(['&\'%s u8' % l for l in unused_lifetimes]), ')>,').nl()
        gen('}').nl()
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('impl', lifetime_params, ' ', ty.name, lifetime_args, ' {').nl()
        with gen.open_indention():
            if not ty.returnedonly:
                gen('#[inline]').nl()
                gen('pub fn new() -> ', ty.name, lifetime_args ,' {').nl()
                with gen.open_indention():
                    gen('unsafe {').nl()
                    with gen.open_indention():
                        default_values = []
                        for m in members:
                            if m['obj'].values:
                                default_values.append((m['name'], m['obj'].values))
                        if default_values:
                            gen(ty.name, ' {').nl()
                            with gen.open_indention():
                                for name, value in default_values:
                                    value, _ = self.rust_value(value)
                                    gen(name, ': ', value, ', ').nl()
                                gen('..::std::mem::zeroed()').nl()
                            gen('}').nl()
                        else:
                            gen('::std::mem::zeroed()').nl()
                    gen('}').nl()
                gen('}').nl()
                for mem in members:
                    m = mem['obj']
                    if mem['hidden']:
                        continue
                    if not self.is_public(m):
                        if m.type.__class__ == TypeRef.POINTER and m.type.arg.__class__ == TypeRef.CONST:
                            arg = m.type.arg.arg
                            if arg.__class__ == TypeRef.POINTER and arg.arg.__class__ == TypeRef.CONST and arg.arg.arg == TypeRef.CHAR: #TODO
                                print('unable to handle setter for %s::%s (2)' % (ty.name, m.name))
                                continue
                        if m.len and m.len != 'null-terminated':
                            if m.len not in ty.members: #TODO
                                print('unable to handle setter for %s::%s (3)' % (ty.name, m.name))
                                continue
                            if len(ty.members[m.len].len_for) != 1: #TODO
                                print('unable to handle setter for %s::%s (4)' % (ty.name, m.name))
                                continue
                    gen('#[inline]').nl()
                    valuetype = mem['typename']
                    gen('pub fn set_', self.rust_member_function(m, ''), '(mut self, value: ', valuetype, ') -> Self {').nl()
                    with gen.open_indention():
                        if m.len and m.len != 'null-terminated':
                            len_param = ty.members[m.len]
                            gen('self. ', self.rust_param_name(len_param), ' = value.len() as ', self.rust_raw_type(len_param), ';').nl()
                        if not mem['as_raw_conv']:
                            gen('self.', mem['name'], ' = value;').nl()
                        else:
                            gen('unsafe {').nl()
                            as_raw_call = self.rust_param_as_raw(m, declname='value')
                            with gen.open_indention():
                                gen('self.', mem['name'], ' = ', as_raw_call, ';').nl()
                            gen('}').nl()
                        gen('self').nl()
                    gen('}').nl()
            for mem in members:
                m = mem['obj']
                if mem['hidden'] and mem['hidden'] != 'readonly':
                    continue
                if m.len and m.len != 'null-terminated':
                    continue # TODO
                #elif m.len and m.len not in ty.members:
                #    continue # TODO
                if m.type == TypeRef.BOOL:
                    gen('#[inline]').nl()
                    gen('pub fn is_', self.rust_member_function(m, ''), '(&self) -> bool {').nl()
                    with gen.open_indention():
                        gen('self.', mem['name'], ' != 0').nl()
                    gen('}').nl()
                elif m.len == 'null-terminated':
                    gen('#[inline]').nl()
                    gen('pub fn ', self.rust_member_function(m, 'get_'), '(&self) -> &CStr {').nl()
                    with gen.open_indention():
                        gen('unsafe { ::std::ffi::CStr::from_ptr(self.', mem['name'],') }').nl()
                    gen('}').nl()
                elif m.type.__class__ == TypeRef.NAMED and m.type.resolved_type.__class__ in STRUCT_TYPES:
                    gen('#[inline]').nl()
                    gen('pub fn ', self.rust_member_function(m, 'get_'), '(&self) -> &', mem['typename'],' {').nl()
                    with gen.open_indention():
                        gen('&self.', mem['name']).nl()
                    gen('}').nl()
                elif not mem['as_raw_conv']:
                    gen('#[inline]').nl()
                    gen('pub fn ', self.rust_member_function(m, 'get_'), '(&self) -> ', mem['typename'],' {').nl()
                    with gen.open_indention():
                        gen('self.', mem['name']).nl()
                    gen('}').nl()
            if has_p_next and not ty.structextends:
                self.add_import('StructExtends')
                gen('#[inline]').nl()
                gen('pub fn extend<E>(self, e: &E) -> Self where E: StructExtends<Self> + Sized {').nl()
                with gen.open_indention():
                    gen('unsafe { self.pNext.set(e.extend(self.pNext.get())) };').nl()
                    gen('self').nl()
                gen('}').nl()

        gen('}').nl()
        if not ty.returnedonly:
            self._generate_feature_protect(ty.requiering_feature, gen)
            gen('impl', lifetime_params, ' Default for ', ty.name, lifetime_args, ' {').nl()
            with gen.open_indention():
                gen('fn default() -> ', ty.name, lifetime_args,' { ', ty.name, '::new() }').nl()
            gen('}').nl()
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('unsafe impl', lifetime_params, ' Struct for ', ty.name, lifetime_args, ' {}').nl()
        if ty.structextends:
            for ext in ty.structextends:
                ext_typename, _, _, ext_lifetimes = self.rust_type_details(ext)
                if 'l' in ext_lifetimes:
                    # rename 'l to 'm
                    ext_lifetimes.remove('l')
                    ext_lifetimes.add('m')
                    ext_typename = ext_typename.replace('\'l', '\'m')
                ext_lifetimes.update(lifetimes)
                ext_lifetime_params = _lifetime_diamond(ext_lifetimes, with_subtyping=True)
                self.add_import('StructExtends')
                self._generate_feature_protect(ty.requiering_feature, gen)
                gen('unsafe impl', ext_lifetime_params, ' StructExtends<', ext_typename,'> for ', ty.name, lifetime_args, ' {').nl()
                with gen.open_indention():
                    gen('#[inline]').nl()
                    gen('unsafe fn extend(&self, next: *const c_void) -> *const c_void {').nl()
                    with gen.open_indention():
                        gen('assert!(self.pNext.get().is_null());').nl()
                        gen('self.pNext.set(next);').nl()
                        gen('self as *const ', ty.name,' as *const c_void').nl()
                    gen('}').nl()
                gen('}').nl()

        size = ty.size()
        if size is None:
            raise ValueError('struct has unknown size', ty)
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('#[cfg(test)]').nl()
        gen('#[test]').nl()
        gen('fn test_struct_size_', camel_to_snake_case(ty.name), '() {').nl()
        with gen.open_indention():
            bytes, ints, pointers, _ = size
            add = ''
            if ints > 0:
                gen('let int_size = ::std::mem::size_of::<::std::os::raw::c_int>();').nl()
                add += ' + int_size * %d' % ints
            if pointers > 0:
                gen('let ptr_size = ::std::mem::size_of::<usize>();').nl()
                add += ' + ptr_size * %d' % pointers
            gen('assert_size!(', bytes, add ,', ', ty.name,');').nl()
        gen('}').nl()

    def _generate_type_union(self, ty, gen):
        self._generate_docs(ty, gen)
        gen('#[repr(C)]').nl()
        gen('#[derive(Copy,Clone)]').nl()
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('pub union ', ty.name, ' {').nl()
        with gen.open_indention():
            for member in ty.members:
                if self.is_public(member):
                    gen('pub ')
                gen(self.rust_member_name(member), ' : ', self.rust_safe_type(member), ',').nl()
        gen('}').nl()
        self._generate_feature_protect(ty.requiering_feature, gen)
        gen('unsafe impl Struct for ', ty.name, ' {}').nl()
        gen('#[cfg(test)]').nl()
        gen('#[test]').nl()
        gen('fn test_union_size_', camel_to_snake_case(ty.name), '() {').nl()
        with gen.open_indention():
            gen('assert_size!(', ty.size()[0] ,', ', ty.name,');').nl()
        gen('}').nl()
        

    def generate_enums(self, file=None):
        if file is None:
            file = os.path.join(self.target,'enums.rs')
        reg = self.registry
        with RustCodeGenerator(file) as gen:
            gen("/* GENERATED FILE */").nl()
            gen.nl()

            for ty in reg.types:
                if ty.__class__ == BaseTypeElem.DEFINE:
                    self._generate_define(ty, gen)

            for group in reg.enum_groups:
                self._generate_enum_group(group, gen)

    def _generate_define(self, define, gen):
        if define.requiering_feature is None \
            or define.name in IGNORED \
            or define.value is None \
            or define.is_deprecated:
            return
        call = ''
        if define.macro_call:
            call, ty = PREDEFINED_UTILS[define.macro_call]
            call = '%s!' % call
            value = define.value
        else:
            value, ty = self.rust_value(define.value)
        self._generate_feature_comment_nonconsecutive(define.requiering_feature, gen)
        self._generate_docs(define, gen)
        self._generate_feature_protect(define.requiering_feature, gen)
        gen('pub const ', define.name, ' : ', ty, ' = ', call, value, ';').nl()


    def _generate_enum_group(self, group, gen):
        if group.requiering_feature is None \
            or group.name in IGNORED:
            return
        self._generate_feature_comment_nonconsecutive(group.requiering_feature, gen)
        gen.nl()
        if group.type is None:
            self._generate_enum_group_defines(group, gen)
        elif group.name == 'VkResult':
            self._generate_enum_group_error_enum(group, gen)
        else:
            self._generate_enum_group_enum(group, gen)

    def _generate_enum_group_defines(self, group, gen):
        gen('// ', group.name).nl()
        gen('/////', '/'*len(group.name)).nl()
        for item in group.enum_items:
            if item.requiering_feature is None \
                or item.name in IGNORED:
                continue
            elif item.requiering_feature is not group.requiering_feature:
                self._generate_feature_comment_nonconsecutive(item.requiering_feature, gen)
            self._generate_docs(item, gen)
            self._generate_feature_protect(item.requiering_feature, gen)
            name = self.rust_enum_item_name(item)
            value, ty = self.rust_enum_item_value(item)
            gen('pub const ', name, ' : ', ty, ' = ', value, ';').nl()

    def _generate_enum_group_enum_item(self, group, item, gen):
        with_guard = item.requiering_feature is not None and item.requiering_feature is not group.requiering_feature
        if with_guard:
            self._generate_feature_comment_nonconsecutive(item.requiering_feature, gen)
        self._generate_docs(item, gen)
        if with_guard:
            self._generate_feature_protect(item.requiering_feature, gen)
        name = self.rust_enum_item_name(item)
        value, _ = self.rust_enum_item_value(item)
        gen(name, ' = ', value)

    def _generate_enum_group_enum(self, group, gen):
        self._generate_feature_protect(group.requiering_feature, gen)
        gen('define_', group.type, '! {').nl()
        with gen.open_indention():
            self._generate_docs(group.enum_type, gen)
            gen('pub enum ', group.name, ' {').nl()
            with gen.open_indention():
                for i, item in enumerate(group.enum_items):
                    if i>0:
                        gen(',').nl()
                    self._generate_enum_group_enum_item(group, item, gen)
            gen.nl()
            gen('}').nl()
        gen('}').nl()
        gen._last_feature = group.requiering_feature
        
    def _generate_enum_group_error_enum(self, group, gen):
        gen('define_enum! {').nl()
        with gen.open_indention():
            self._generate_docs(group.enum_type, gen)
            gen('pub enum VkError {').nl()
            with gen.open_indention():
                for i, item in enumerate(group.enum_items):
                    if i == 0: # skip SUCCESS
                        continue
                    if i > 1:
                        gen(',').nl()
                    self._generate_enum_group_enum_item(group, item, gen)
            gen.nl()
            gen('}').nl()
        gen('}').nl()
        gen.nl()
        gen('impl ::std::error::Error for VkError {').nl()
        with gen.open_indention():
            gen('fn description(&self) -> &str {').nl()
            with gen.open_indention():
                for i, item in enumerate(group.enum_items):
                    if i == 0: # skip SUCCESS
                        continue
                    with_guard = item.requiering_feature is not None and item.requiering_feature is not group.requiering_feature
                    if with_guard:
                        self._generate_feature_protect(item.requiering_feature, gen)
                        gen('{').nl().i()
                    name = self.rust_enum_item_name(item)
                    comment = item.comment
                    if not comment and item.docs and len(item.docs)>0:
                        comment = item.docs[0].strip()
                    if not comment:
                        comment = item.shortname.lower()
                        if comment.startswith('error_'):
                            comment = comment[6:]
                    else:
                        dot = comment.find('.')
                        if dot > 0:
                            comment = comment[:dot]
                    gen('if *self == VkError::', name, '{ return "', comment,'"; }').nl()
                    if with_guard:
                        gen.o()('}').nl()
                gen('"unknown"').nl()
            gen('}').nl()
        gen('}').nl()
        gen('impl ::std::fmt::Display for VkError {').nl()
        with gen.open_indention():
            gen('fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {').nl()
            with gen.open_indention():
                gen('write!(f, "{} ({})", ::std::error::Error::description(self), *self as i32)')
            gen('}').nl()
        gen('}').nl()
        self._generate_docs(group.enum_type, gen)
        gen('pub type VkResult<V=()> = Result<V,VkError>;')

    def _generate_command_signature(self, base_cmd, gen, paramnames=True, method=None, safe=False, with_return=True, **kwargs):
        if method is None:
            method = safe is True and self.rust_safe_type or self.rust_raw_type
        gen('(')
        i = 0
        for param in base_cmd.params:
            if safe and (param is base_cmd.out_param or param.len_for):
                continue
            if i>0:
                gen(', ')
            i += 1
            if paramnames and param.name:
                gen(self.rust_param_name(param.name), ': ')
            gen(method(param, as_param=True, **kwargs))
        gen(')')
        if with_return and base_cmd.returns != TypeRef.VOID:
            gen(' -> ', method(base_cmd.returns, optional=True, **kwargs))

    def generate_protos(self, file=None):
        if file is None:
            file = os.path.join(self.target, 'protos.rs')
        reg = self.registry
        with RustCodeGenerator(file) as gen:
            gen("/* GENERATED FILE */").nl()
            gen.nl()
            gen('#![allow(non_camel_case_types)]').nl()
            gen.nl()
            with self.manage_imports(gen) as gen:
                self.add_import('platform::*')
                self.add_import('types::*')
                for feature in reg.features:
                    with gen.open_nonempty() as nonempty_gen:
                        self._generate_feature_comment(feature, nonempty_gen)
                        for command in feature.commands:
                            self._generate_proto(command, gen)

    def _generate_proto(self, command, gen):
        self._generate_feature_protect(command.requiering_feature, gen)
        gen('pub type PFN_', command.name, ' = extern "system" fn ' )
        self._generate_command_signature(command, gen, paramnames=False, safe=False)
        gen(';').nl()


    def generate_dispatch_table(self, file=None):
        if file is None:
            file = os.path.join(self.target, 'dispatch_table.rs')
        with RustCodeGenerator(file) as gen:
            gen("/* GENERATED FILE */").nl()
            gen.nl()
            gen('use protos::*;').nl()
            gen('use types::PFN_vkVoidFunction;').nl()
            gen.nl()
            for table in DispatchTable:
                self._generate_dispatch_table(table, gen)
            gen.nl()
            gen.nl()
            for table in DispatchTable:
                self._generate_dispatch_table_init(table, gen)

    def _generate_dispatch_table(self, table, gen):
        gen('#[allow(non_snake_case)]').nl()
        gen('pub struct Vk', table.value.capitalize(), 'DispatchTable {').nl()
        with gen.open_indention():
            for feature in self.registry.features:
                with gen.open_nonempty() as nonempty_gen:
                    self._generate_feature_comment(feature, nonempty_gen)
                    for command in feature.commands:
                        if command.dispatch_table is not table \
                            or command.name.endswith('ProcAddr') \
                            or command.name in IGNORED:
                            continue
                        self._generate_feature_protect(feature, gen)
                        gen('pub ', command.name, ': Option<PFN_', command.name, '>,').nl()
        gen('}').nl()
        gen.nl()
    
    def _generate_dispatch_table_init(self, table, gen):
        table_name = table.value.capitalize()
        gen('impl Vk', table_name, 'DispatchTable {').nl()
        with gen.open_indention():
            if table is DispatchTable.Loader:
                gen('pub unsafe fn load<R,F1>(gpa: F1)')
            else:
                gen('pub unsafe fn load<R,F1,F2>(gpa: F1, has_ext: F2)')
            gen(' -> Result<Vk', table_name, 'DispatchTable, R>').nl()
            with gen.open_indention():
                gen('where').nl()
                with gen.open_indention():
                    gen('F1: Fn(&str) -> Result<PFN_vkVoidFunction, R>,').nl()
                    if table is not DispatchTable.Loader:
                        gen('F2: Fn(&str) -> bool,').nl()
            gen('{').nl()
            with gen.open_indention():
                gen('use std::mem::transmute as tm;').nl()
                gen('let mut tab : Vk', table_name, 'DispatchTable = ::std::mem::zeroed();')
                for feature in self.registry.features:
                    with gen.open_nonempty() as nonempty_gen:
                        self._generate_feature_comment(feature, nonempty_gen)
                        if self._is_feature_protect(feature):
                            self._generate_feature_protect(feature, nonempty_gen)
                            nonempty_gen('{').nl()
                            nonempty_gen.i()
                        if feature.is_extension:
                            nonempty_gen('if has_ext("', feature.name, '\\0") {').nl()
                            nonempty_gen.i()
                        try:
                            for command in feature.commands:
                                if command.dispatch_table is not table \
                                    or command.name.endswith('ProcAddr') \
                                    or command.name in IGNORED:
                                    continue
                                gen('tab.', command.name, ' = tm(gpa("', command.name, '\\0")?);').nl()
                        finally:
                            if feature.is_extension:
                                nonempty_gen.o()
                                nonempty_gen('}').nl()
                            if self._is_feature_protect(feature):
                                nonempty_gen.o()
                                nonempty_gen('}').nl()
                gen('Ok(tab)').nl()
            gen('}').nl()
        gen('}').nl()
        gen.nl()


    def generate_dispatch_commands(self, file=None):
        if file is None:
            file = os.path.join(self.target, 'dispatch_commands.rs')
        reg = self.registry
        with RustCodeGenerator(file) as gen:
            gen("/* GENERATED FILE */").nl()
            gen.nl()
            gen('#![allow(non_snake_case)]').nl()
            gen.nl()
            with self.manage_imports(gen) as gen:
                self.add_import('AsRaw')
                self.add_import('platform::*')
                self.add_import('enums::{VkError,VkResult}')
                self.add_import('types::*')
                self.add_import('dispatch_table::*')
                gen.nl()
                for feature in reg.features:
                    with gen.open_nonempty() as nonempty_gen:
                        self._generate_feature_comment(feature, nonempty_gen)
                        for command in feature.commands:
                            if command.name.endswith('ProcAddr') \
                                or command.name in IGNORED:
                                continue
                            self._generate_dispatch_command(command, gen)

    def _generate_dispatch_command(self, command, gen):
        table_name = command.dispatch_table.value.capitalize()

        if command.dispatch_table is DispatchTable.Loader:
            handle_arg = ''
        else:
            handle_arg = self.rust_param_name(next(iter(command.params))) + ', '
        
        is_create = False
        for table2 in DispatchTable:
            table2_name = table2.name.capitalize()
            if command.name == 'vkCreate%s'%table2_name:
                is_create = table2_name
                break

        # is this the destroy command for the dispatch_table
        is_destroy = command.name == 'vkDestroy%s'%table_name

        # remove lifetime 'l, we only care about lifetime 'h (for handles)
        lifetimes = self.rust_composed_lifetimes(command) - set(['l'])
        def safe_dispatch_type(*args, **kwargs):
            tyname = self.rust_safe_type(*args, **kwargs)
            return tyname.replace('&\'l ', '&').replace('<\'l>', '').replace('\'l', '\'_')

        out_param = command.out_param
        out_typename = None
        out_typename_return = None
        out_convert = ''
        if out_param:
            if command.out_param.type.arg == TypeRef.BOOL:
                out_typename = 'VkBool32'
            else:
                out_typename = safe_dispatch_type(command.out_param.type.arg)
            if out_param.len:
                if out_param.type.arg == TypeRef.VOID:
                    out_typename = 'u8'
                out_typename = 'Vec<%s>' % out_typename
            out_typename_return = out_typename
            if command.out_param.type.arg == TypeRef.BOOL:
                out_typename_return = 'bool'
                out_convert = ' != 0'
        self._generate_docs(command, gen)
        self._generate_feature_protect(command.requiering_feature, gen)
        gen('pub fn ', command.name, _lifetime_diamond(lifetimes))
        self._generate_command_signature(command, gen, method=safe_dispatch_type, safe=True, with_return=False)
        result_convert = ''
        if out_param and command.returns == TypeRef.RESULT:
            gen(' -> VkResult<', out_typename_return,'>')
        elif command.returns == TypeRef.BOOL:
            gen(' -> bool')
            result_convert = ' != 0'
        elif command.returns != TypeRef.VOID:
            gen(' -> ', safe_dispatch_type(command.returns, optional=True))
        elif out_param:
            gen(' -> ', out_typename_return)

        gen(' {').nl()
        with gen.open_indention():
            gen('unsafe {').nl()
            with gen.open_indention():

                # add length params
                enumerate_len_param = None
                enumerate_with_incomplete = False
                for param in command.params:
                    if param.len_for:
                        if param.is_out:
                            enumerate_len_param = param
                            gen('let mut ', self.rust_param_name(param), ': ', safe_dispatch_type(param.type.arg), ' = 0;').nl()
                        else:
                            gen('let ', self.rust_param_name(param), ' = ', self.rust_param_name(param.len_for[0]), '.len() as ', safe_dispatch_type(param), ';').nl()
                            for len_for_param in param.len_for[1:]:
                                gen('assert!(', self.rust_param_name(param), ' as usize == ', self.rust_param_name(len_for_param),'.len());').nl()
                if enumerate_len_param:
                    enumerate_with_incomplete = command.returns == TypeRef.RESULT and 'VK_INCOMPLETE' in command.successcodes

                # add return param
                out_param = command.out_param
                out_len_expr = None
                if out_param:
                    out_paramname = self.rust_param_name(out_param)
                    gen('let mut ', out_paramname, ': ', out_typename, ' = ')
                    if out_param.len:
                        if enumerate_len_param:
                            out_len_expr = self.rust_param_name(enumerate_len_param)
                            gen('Vec::new();').nl()
                        else:
                            out_len_elems = out_param.len.split('::')
                            out_len_expr = self.rust_param_name(out_len_elems[0]) + ''.join(['.%s()' % self.rust_member_function(p) for p in out_len_elems[1:]])
                            gen('Vec::with_capacity(', out_len_expr,' as usize);').nl()
                    else:
                        gen('::std::mem::zeroed();').nl()
                
                is_check_result = command.returns == TypeRef.RESULT
                if is_check_result and is_destroy:
                    gen('let _r = ')
                gen('Vk', table_name, 'DispatchTable::with(', handle_arg, '|_t|{').nl()
                with gen.open_indention():
                    if enumerate_with_incomplete:
                        gen('loop {').nl().i()
                    all_params_as_raw = [self.rust_param_as_raw(p) for p in command.params]
                    if enumerate_len_param:
                        if is_check_result:
                            gen('let _r = ')
                        all_args = ', '.join(all_params_as_raw[:-1] + ['::std::ptr::null_mut()'])
                        gen('_t.', command.name, '.unwrap()(', all_args, ');').nl()
                        if enumerate_with_incomplete:
                            gen('if _r == Err(VkError::INCOMPLETE) { continue; }').nl()
                        if is_check_result:
                            gen('if let Err(_e) = _r { return Err(_e); }').nl()
                        gen('if ', self.rust_param_name(enumerate_len_param) ,' == 0 {').nl()
                        with gen.open_indention():
                            if is_check_result:
                                gen('return Ok(', out_paramname, out_convert, ');').nl()
                            else:
                                gen('return ', out_paramname, out_convert, ';').nl()
                        gen('}').nl()
                        gen(self.rust_param_name(out_param) ,' = Vec::with_capacity(', out_len_expr,' as usize);').nl()

                    if is_check_result and (is_create or out_param):
                        gen('let _r = ')

                    all_args = ', '.join(all_params_as_raw)
                    gen('_t.', command.name, '.unwrap()(', all_args, ')')

                    if out_param:
                        gen(';').nl()
                        if enumerate_with_incomplete:
                            gen('if _r == Err(VkError::INCOMPLETE) { continue; }').nl()
                        if is_check_result:
                            gen('if let Err(_e) = _r { return Err(_e); }').nl()
                    elif is_create:
                        gen(';').nl()
                        if is_check_result:
                            gen('if let Err(_e) = _r { return Err(_e); }').nl()
                    else:
                        gen.nl()
                    
                    if out_param and out_param.len:
                        gen(self.rust_param_name(out_param) ,'.set_len(', out_len_expr,' as usize);').nl()

                    if is_create:
                        all_args = ', '.join([self.rust_param_name(p.name) for p in command.params])
                        gen('Vk', is_create, 'DispatchTable::create(', all_args, ');').nl()
                    
                    if out_param:
                        if is_check_result:
                            gen('return Ok(', out_paramname, out_convert, ');').nl()
                        else:
                            gen('return ', out_paramname, out_convert, ';').nl()
                        if enumerate_with_incomplete:
                            gen.o()
                            gen('}').nl()
                    elif is_create and is_check_result:
                        gen('_r').nl()

                gen('})', result_convert)
                if is_destroy:
                    gen(';').nl()
                    gen('Vk', table_name, 'DispatchTable::destroy(', handle_arg, ');')
                    if is_check_result:
                        gen('return _r;')
                gen.nl()
            gen('}').nl()
        gen('}').nl()

    def generate_prelude(self, file=None):
        if file is None:
            file = os.path.join(self.target, 'prelude.rs')
        reg = self.registry
        with RustCodeGenerator(file) as gen:
            gen("/* GENERATED FILE */").nl()
            gen.nl()
            for f in reg.features:
                protect = True
                if f.is_extension:
                    protect = False
                    self._generate_docs(f, gen)
                    self._generate_feature_protect(f, gen)
                    gen('pub mod ', f.name.lower(), '{').nl()
                    gen.i()

                for enum_item in f.enum_items:
                    if enum_item.enum_group.type is not None \
                        or enum_item.name in IGNORED:
                        continue
                    if protect:
                        self._generate_feature_protect(f, gen)
                    gen('pub use enums::', enum_item.name, ';').nl()

                for ty in f.types:
                    if ty.name == 'VkResult':
                        gen('pub use enums::VkError;').nl()
                    if ty.name in PREDEFINED_UTILS:
                        name, _ = PREDEFINED_UTILS[ty.name]
                        gen('pub use utils::', name, ';').nl()
                        continue
                    if ty.__class__ == BaseTypeElem.PROVIDED \
                        or ty.__class__ == BaseTypeElem.INCLUDE \
                        or ty.name in IGNORED:
                        continue
                    if ty.__class__ == BaseTypeElem.DEFINE:
                        if ty.is_deprecated or ty.value is None:
                            continue
                        if protect:
                            self._generate_feature_protect(f, gen)
                        gen('pub use enums::', ty.name, ';').nl()
                    else:
                        if protect:
                            self._generate_feature_protect(f, gen)
                        gen('pub use types::', ty.name, ';').nl()
                for cmd in f.commands:
                    if cmd.name in PREDEFINED_UTILS:
                        name, _ = PREDEFINED_UTILS[cmd.name]
                        gen('pub use utils::', name, ';').nl()
                        continue
                    if cmd.name.endswith('ProcAddr') \
                        or cmd.name in IGNORED:
                        continue
                    if protect:
                        self._generate_feature_protect(f, gen)
                    gen('pub use dispatch_commands::', cmd.name, ';').nl()

                if f.is_extension:
                    gen.o()
                    gen('}').nl()

