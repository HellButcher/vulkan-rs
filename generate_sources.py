#!/usr/bin/env python3
# pylint: disable=C0301,C0111,R0903,R0902,R0912,W0613

import os
import sys
import re
stdlog = sys.stderr
try:
    import lxml.etree as ET
    stdlog.write("using lxml\n")
except ImportError:
    import xml.etree.ElementTree as ET

def get_element_name(element, _name_node=None):
    if _name_node is not None:
        namenode = element.find(_name_node)
        if namenode is not None:
            return namenode.text, namenode
    name = element.get('name')
    if name is not None:
        return name, None
    namenode = element.find('name')
    if namenode is not None:
        return namenode.text, namenode

def strip_api_prefix(registry, name):
    if name.startswith('PFN_'):
        api, name = strip_api_prefix(registry, name[4:])
        return (api, 'Pfn' + name)
    if name.lower().startswith('vk_'):
        return (name[:2], name[3:])
    if name.lower().startswith('vk'):
        return (name[:2], name[2:])
    return (None, name)

def strip_vendor_suffix(registry, name):
    for vendorid in registry.vendorids:
        if name.endswith(vendorid):
            name = name[0:-len(vendorid)]
            if name.endswith('_'):
                name = name[0:-1]
            return (name, vendorid)
    return (name, None)

def strip_name(registry, name):
    api, name = strip_api_prefix(registry, name)
    name, vendorid = strip_vendor_suffix(registry, name)
    return (api, name, vendorid)

first_cap_re = re.compile('(.)([A-Z][a-z]+)')
all_cap_re = re.compile('([a-z0-9])([A-Z])')
merge_underscore_re = re.compile('__+')
def decamelize(name):
    name = first_cap_re.sub(r'\1_\2', name)
    name = all_cap_re.sub(r'\1_\2', name).upper()
    name = merge_underscore_re.sub(r'_', name)
    return name

def camelize(name):
    if isinstance(name, str):
        name=name.split('_')
    return ''.join([x.capitalize() for x in name])

class Node(object):

    def __init__(self, element, parent=None):
        self.children = []
        self.named_children = dict()
        self.element = element
        self.tag = None
        if hasattr(element, 'tag'):
            self.tag = element.tag
        self.parent = parent
        self.registry = None
        if parent is not None:
            parent.children.append(self)

    def resolve(self, registry):
        self.registry = registry
        for child in self.children:
            child.resolve(registry)

    def __getattr__(self, name):
        return self.element.get(name)
    def __getitem__(self, key):
        return self.named_children[key]
    def __contains__(self, item):
        return item in self.named_children
    def __len__(self):
        return len(self.children)
    def __iter__(self):
        return self.children.__iter__()

    def __repr__(self):
        if hasattr(self.element, 'sourceline'):
            return '%s[%s](%s)' % (self.element.tag, type(self).__name__, self.element.sourceline)
        else:
            return '%s[%s]' % (self.element.tag, type(self).__name__)
    def __str__(self):
        return self.__repr__()

class NamedNode(Node):

    def __init__(self, element, name=None, _name_node=None, **kwargs):
        Node.__init__(self, element, **kwargs)
        self.name_node = None
        self.name = name
        if name is None:
            name, name_node = get_element_name(element, _name_node=_name_node)
            self.name = name
            self.name_node = name_node
            if name is None:
                raise ValueError('node %s has no name' % self.__repr__())
        self.stripped_api = self.stripped_name = self.stripped_vendorid = self.stripped_name_parts = None
        if self.parent is not None:
            key = (self.tag, name)
            if key in self.parent.named_children:
                raise KeyError('node %s is already defined here %s' % (self.__repr__(), self.parent.named_children[name].__repr__()))
            self.parent.named_children[key] = self

    def __repr__(self):
        return '%s@%s' % (self.name, Node.__repr__(self))

    def resolve(self, registry):
        Node.resolve(self, registry)
        self.stripped_name_parts = strip_name(registry, self.name)
        self.stripped_api, self.stripped_name, self.stripped_vendorid = self.stripped_name_parts

class VendorIdNode(NamedNode):

    def __init__(self, element, **kwargs):
        NamedNode.__init__(self, element, **kwargs)

class TypeRefNode(Node):
    def __init__(self, element, **kwargs):
        Node.__init__(self, element, **kwargs)
        self.name = element.text
        self.is_const = False
        self.is_pointer = False
        self.array_size = None
        self.referenced_type=None
        if element.tail is not None and element.tail.strip().startswith('*'):
            self.is_pointer = True
        parent_element = element.find('..')
        if parent_element is not None:
            prev_element=None
            element_index=None
            next_element=None
            i=0
            for e in parent_element.findall('*'):
                if e==element:
                    element_index = i
                elif element_index is None:
                    prev_element = e
                else:
                    next_element = e
                    break
                i=i+1
            if element_index is not None:
                if prev_element is not None:
                    prefix = prev_element.tail
                else:
                    prefix = parent_element.text
                if prefix is not None and prefix.strip()=='const':
                    self.is_const = True
                if next_element is not None and next_element.tag=='name' and next_element.tail is not None:
                    suffix = next_element.tail.strip()
                    if suffix.startswith('[') and suffix.endswith(']'):
                        self.array_size=int(suffix[1:-1])
        if self.is_const and not self.is_pointer and self.array_size is None:
            stdlog.write('type-ref %s defined const but is not a pointer\n' % self)

    def resolve(self, registry):
        Node.resolve(self, registry)
        self.referenced_type = registry[('type', self.name)]

    @staticmethod
    def of(parent, **kwargs):
        element = parent.element.find('type')
        if element is not None:
            return TypeRefNode(element, parent=parent, **kwargs)
        else:
            return None

    def bit_size(self):
        if self.is_pointer:
            raise ValueError('can\'t calculate size of pointer %s' % self)
        if self.name in TYPE_MAP:
            t = TYPE_MAP[self.name]
            try:
                s = int(t[1:])
            except:
                raise ValueError('can\'t calculate size of %s with basic type %s' % (self, t))
        else:
            s = self.referenced_type.bit_size()
        if self.array_size is not None:
            s *= self.array_size
        return s

    def is_void(self):
        return self.name=='void' and not self.is_pointer and not self.is_const and self.array_size is None

class TypeNode(NamedNode):

    def __init__(self, element, **kwargs):
        NamedNode.__init__(self, element, **kwargs)
        self.required_types=set()
        self.required=[]
        self.members = []
        self.types = []
        self.type=None
        self.returns = None
        self.params = []
        self._bit_size=None
        if element.get('category')=='funcpointer':
            self.restructure_funcpointer()
        self.add_elements()

    def add_elements(self):
        for element in self.element.findall('type'):
            self.types.append(TypeRefNode(element, parent=self))
        if len(self.types)>0:
            self.type = self.types[0]
        for element in self.element.findall('member'):
            self.members.append(TypeMemberNode(element, parent=self))
        if self.element.get('category')=='funcpointer':
            return_type_element = self.element.find('proto')
            self.returns = ParameterNode(return_type_element, parent=self)
            for element in self.element.findall('param'):
                self.params.append(ParameterNode(element, parent=self))

    def restructure_funcpointer(self):
        t = self.element.text
        if t is None:
            return
        t = t.strip()
        if not t.startswith('typedef '):
            return
        t = t[8:]
        # handle return type
        pos = t.find('(')
        if pos < 0:
            return
        t_prefix = None
        t_type = t[:pos].strip()
        t_suffix = ' '
        if t_type.startswith('const '):
            t_prefix = 'const '
            t_type = t_type[6:].strip()
        if t_type.endswith('*'):
            t_suffix = '* '
            t_type = t_type[:-1].strip()
        e_proto = ET.SubElement(self.element, 'proto')
        e_proto.text = t_prefix
        e_type = ET.SubElement(e_proto, 'type')
        e_type.text = t_type
        e_type.tail = t_suffix
        e_name = ET.SubElement(e_proto, 'name')
        e_name.text = self.name

        #handle params
        t = self.element.find('name').tail.strip()
        pos = t.find('(')
        if pos < 0:
            raise ValueError('expected parameter signature in %s' % self)
        t_prefix = t[pos+1:].strip()
        if len(t_prefix)>0:
            t_prefix = t_prefix+'_'
        else:
            t_prefix = None
        for e in self.element.findall('type'):
            e_param = ET.SubElement(self.element, 'param')
            e_param.text = t_prefix

            t_prefix = ''
            t_suffix = ' '
            t_name = None
            t_name_suffix = None
            t = e.tail
            if t is not None:
                t = t.strip()
                pos = t.find(')')
                if pos>0:
                    t = t[:pos].strip()
                pos = t.find(',')
                if pos>0:
                    t_prefix = t[pos+1:].lstrip()
                    t = t[:pos].strip()
                if t.startswith('*'):
                    t_suffix = '* '
                    t = t[1:].strip()
                pos = t.find('[')
                if pos>0:
                    t_name_suffix = t[pos:]
                    t = t[:pos].strip()
                t_name = t
            e_type = ET.SubElement(e_param, 'type')
            e_type.text = e.text
            e_type.tail = t_suffix
            if t_name is not None:
                e_name = ET.SubElement(e_param, 'name')
                e_name.text = t_name
                e_name.tail = t_name_suffix

    def resolve(self, registry):
        NamedNode.resolve(self, registry)
        requires = self.element.get('requires')
        if requires is not None:
            for req in requires.split(','):
                t = registry[('type', req)]
                self.required_types.add(t)
                self.required.append(t)
        elif self.category=='bitmask':
            name,vendor = strip_vendor_suffix(registry, self.name)
            if not name.endswith('Flags'):
                raise ValueError('bitmask %s expacted name ending with Flags' % self)
            requires = name[:-5] + 'FlagBits'
            if vendor is not None:
                self.requires = requires = requires + vendor
            try:
                n = registry[('type', requires)]
                self.required_types.add(n)
                self.required.append(n)
            except KeyError:
                stdlog.write('there is no FlagBits-definition for %s: automatically derived one.\n' % self)
                e = ET.SubElement(registry.root.find('types'), 'type')
                e.set('name', requires)
                e.set('category', 'enum')
                n = registry.add_type_element(e)
                self.required_types.add(n)
                self.required.append(n)
        elif self.category=='funcpointer':
            for t in self.returns.required_types:
                self.required_types.add(t)
            for p in self.params:
                for t in p.required_types:
                    self.required_types.add(t)
        for t in self.types:
            self.required_types.add(t.referenced_type)
        for m in self.members:
            for t in m.required_types:
                self.required_types.add(t)

    def bit_size(self):
        s = self._bit_size
        if s is not None:
            return s
        self._bit_size = 0
        c = self.category
        if c == 'basetype':
            s = self.types[0].bit_size()
        elif c == 'enum' or c == 'bitmask':
            s = 32
        elif c == 'struct':
            s = 0
            for m in self.members:
                s += m.type.bit_size()
        elif c == 'union':
            s = 0
            for m in self.members:
                tmp = m.type.bit_size()
                if tmp>s:
                    s = tmp
        else:
            raise ValueError('unable to calculate size for %s' % self)
        self._bit_size = s
        if s == 0:
            raise ValueError('calculated empty size for %s' % self)
        if s % 32:
            raise ValueError('calculated size for %s is not a multiple of 32-bit' % self)
        return s

class TypeMemberNode(NamedNode):

    def __init__(self, element, **kwargs):
        NamedNode.__init__(self, element, **kwargs)
        self.required_types=set()
        self.type = TypeRefNode.of(self)

    def resolve(self, registry):
        NamedNode.resolve(self, registry)
        if self.type is not None:
            self.required_types.add(self.type.referenced_type)

class EnumNode(NamedNode):

    def __init__(self, element, **kwargs):
        NamedNode.__init__(self, element, **kwargs)
        self.name_parts=decamelize(self.name).split('_')
        self.add_elements()

    def add_elements(self):
        for element in self.element.findall('enum'):
            EnumItemNode(element, parent=self)

class EnumItemNode(NamedNode):

    def __init__(self, element, extension=None, *args, **kwargs):
        NamedNode.__init__(self, element, **kwargs)
        self.name_parts=self.name.split('_')
        parent_parts = self.parent.name_parts
        if self.parent.stripped_vendorid is not None and self.name_parts[len(self.name_parts)-1].upper()==self.parent.stripped_vendorid.upper():
            self.name_parts = self.name_parts[:-1]
        prefixlen=min(len(parent_parts), len(self.name_parts)-1)
        i=0
        while i<prefixlen:
            if self.name_parts[i].upper() != parent_parts[i].upper():
                break
            i=i+1
        while i>0 and self.name_parts[i][0].isnumeric():
            i=i-1
        self.short_name_parts = self.name_parts[i:]
        self.short_name='_'.join(self.short_name_parts)

class CommandNode(NamedNode):

    def __init__(self, element, **kwargs):
        NamedNode.__init__(self, element, _name_node='proto/name', **kwargs)
        self.returns = None
        self.params = []
        self.required_types=set()
        self.errorcodes=element.get('errorcodes')
        self.successcodes=element.get('successcodes')
        if self.errorcodes is not None:
            self.errorcodes = self.errorcodes.split(',')
        if self.successcodes is not None:
            self.successcodes = self.successcodes.split(',')
        self.add_elements()

    def add_elements(self):
        return_type_element = self.element.find('proto')
        self.returns = ParameterNode(return_type_element, parent=self)
        for element in self.element.findall('param'):
            self.params.append(ParameterNode(element, parent=self))

    def resolve(self, registry):
        NamedNode.resolve(self, registry)
        for t in self.returns.required_types:
            self.required_types.add(t)
        for p in self.params:
            for t in p.required_types:
                self.required_types.add(t)

class ParameterNode(TypeMemberNode):

    def __init__(self, element, **kwargs):
        TypeMemberNode.__init__(self, element, **kwargs)

def load_tree(tree_or_filename):
    if hasattr(tree_or_filename, 'getroot'):
        stdlog.write('loaded registry from in-memory tree\n')
        return tree_or_filename
    else:
        stdlog.write('loading registry from file %s...\n' % tree_or_filename)
        with open(tree_or_filename, "r") as regfile:
            tree_or_filename = ET.parse(regfile)
        stdlog.write('loaded registry\n')
        return tree_or_filename

class Registry(Node):

    def __init__(self, tree_or_filename):
        Node.__init__(self, load_tree(tree_or_filename))
        self.root = self.element
        self.vendorids = set(['EXT', 'KHR', 'ARB', 'NV', 'AMD'])
        self.types=[]
        self.enums=[]
        self.commands=[]
        self.add_elements()

    def add_elements(self):
        for element in self.root.findall('vendorids/vendorid'):
            self.add_vendorid_element(element)
        for element in self.root.findall('types/type'):
            self.add_type_element(element)
        for element in self.root.findall('enums'):
            self.add_enum_element(element)
        for element in self.root.findall('commands/command'):
            self.add_command_element(element)

    def add_vendorid_element(self, element):
        node = VendorIdNode(element, parent=self)
        self.vendorids.add(node.name)
        return node

    def add_type_element(self, element):
        node = TypeNode(element, parent=self)
        self.types.append(node)
        return node

    def add_enum_element(self, element):
        node = EnumNode(element, parent=self)
        self.enums.append(node)
        return node

    def add_command_element(self, element):
        node = CommandNode(element, parent=self)
        self.commands.append(node)
        return node

    def resolve(self, registry=None):
        if registry is None:
            registry = self
        Node.resolve(self, registry)

TYPE_MAP = {
    'void':      'c_void', #std::os::raw::c_void
    'char':      'c_char', #std::os::raw::c_char
    'uint8_t':   'u8',
    'uint16_t':  'u16',
    'uint32_t':  'u32',
    'uint64_t':  'u64',
    'int8_t':    'i8',
    'int16_t':   'i16',
    'int32_t':   'i32',
    'int64_t':   'i64',
    'ssize_t':   'isize',
    'size_t':    'usize',
    'intptr_t':  'isize',
    'uintptr_t': 'usize',
    'float':     'f32',
    'double':    'f64',
    'VkEnum':    'basic_types::EnumRepr',
    'VkFlags':   'basic_types::FlagRepr',
    'VK_DEFINE_HANDLE': 'basic_types::Handle',
    'VK_DEFINE_NON_DISPATCHABLE_HANDLE': 'basic_types::DispatchableHandle',
}


def map_type(type_ref):
    if type_ref.name in TYPE_MAP:
        type_name = TYPE_MAP[type_ref.name]
    else:
        ref = type_ref.referenced_type
        type_name = ref.stripped_name
        if ref.category is None and ref.requires is not None and ref.requires.endswith('.h'):
            if type_ref.is_pointer:
                type_name = TYPE_MAP['void']
            else:
                type_name = TYPE_MAP['uintptr_t']

    if type_ref.is_pointer:
        if type_ref.is_const:
            type_name = '*const '+type_name
        else:
            type_name = '*mut '+type_name
    if type_ref.array_size is not None:
        type_name = '[%s; %d]' % (type_name, type_ref.array_size)
    return type_name

KEYWORD_MAP = {
    'as': 'as_',
    'break': 'break_',
    'const': 'const_',
    'continue': 'continue_',
    'crate': 'crate_',
    'else': 'else_',
    'enum': 'enum_',
    'extern': 'extern_',
    'false': 'false_',
    'fn': 'fn_',
    'for': 'for_',
    'if': 'if_',
    'impl': 'impl_',
    'in': 'in_',
    'let': 'let_',
    'loop': 'loop_',
    'match': 'match_',
    'mod': 'mod_',
    'move': 'move_',
    'mut': 'mut_',
    'pub': 'pub_',
    'ref': 'ref_',
    'return': 'return_',
    'self': 'self_',
    'static': 'static_',
    'struct': 'struct_',
    'type': 'type_',
    'trait': 'trait_',
    'true': 'true_',
    'unsafe': 'unsafe_',
    'use': 'use_',
    'where': 'where_',
    'while': 'while_',
}


def map_keyword(name, type_name=None):
    if name in KEYWORD_MAP:
        if type_name is not None:
            return type_name[0].lower() + name[0].upper() + name[1:]
        return KEYWORD_MAP[name]
    return name

def write_go_comment(out, content, doc=False, indent=''):
    if content is None:
        return
    lines = content.splitlines()
    if len(lines) == 1:
        line = lines[0]
        if line.startswith('//'):
            line = line[2:].strip()
        out.write(indent)
        if doc:
            out.write('/// ')
        else:
            out.write('// ')
        out.write(line)
        out.write('\n')
    else:
        if not doc:
            out.write(indent)
            out.write('/*\n')
        for line in lines:
            out.write(indent)
            if doc:
                out.write('/// ')
            else:
                out.write(' * ')
            out.write(line)
            out.write('\n')
        if not doc:
            out.write(indent)
            out.write(' */\n')

def write_type_basetype(out, options, type_node):
    write_go_comment(out, type_node.comment, doc=True)
    out.write('pub type %s = %s;\n' % (type_node.stripped_name, map_type(type_node.type)))

def write_type_enum_simple(out, options, type_node):
    write_go_comment(out, type_node.comment, doc=True)
    out.write('pub type %s = basic_types::EnumRepr;\n' % (type_node.stripped_name))

def write_type_enum_complex(out, options, type_node):
    write_go_comment(out, type_node.comment, doc=True)
    out.write('#[derive(Copy,Clone)]\n')
    out.write('#[repr(u32)]\n')
    out.write('pub enum %s {\n' % (type_node.stripped_name))
    enum_node = None
    try:
        enum_node = type_node.registry[('enums', type_node.name)]
    except KeyError:
        pass
    if enum_node is not None:
        write_enum_complex(out, options, enum_node)
    else:
        out.write('  MaxEnum = 0x7FFFFFFF,\n')
    out.write('}\n')
    out.write('vk_enum_impl!(%s);\n' % (type_node.stripped_name))

def write_type_enum(out, options, type_node):
    if options.get('simple_enums') is True:
        write_type_enum_simple(out, options, type_node)
    else:
        write_type_enum_complex(out, options, type_node)

def write_type_bitmask(out, options, type_node):
    bits_type = type_node.required[0]
    write_go_comment(out, type_node.comment, doc=True)
    out.write('vk_bitmask!(pub %s for %s);\n' % (type_node.stripped_name, bits_type.stripped_name))

def write_type_struct(out, options, type_node):
    write_go_comment(out, type_node.comment, doc=True)
    out.write('#[repr(C)]\n')
    out.write('pub struct %s {\n' % type_node.stripped_name)
    for member in type_node.members:
        write_go_comment(out, member.comment, doc=True, indent='  ')
        out.write('  pub %s: %s,\n' % (map_keyword(decamelize(member.name).lower()), map_type(member.type)))
    out.write('}\n')


def write_type_union(out, options, type_node):
    size = type_node.bit_size()
    write_go_comment(out, type_node.comment, doc=True)
    out.write('#[repr(C)]\n')
    out.write('pub struct %s {\n' % type_node.stripped_name)
    out.write('  data: [u32; %d],\n' % (size/32))
    out.write('}\n')
    out.write('impl %s {\n' % type_node.stripped_name)
    for member in type_node.members:
        mem_size = member.type.bit_size()
        out.write('  #[inline] pub fn as_%s(&self) -> &%s {\n' % (decamelize(member.name).lower(), map_type(member.type)))
        out.write('    unsafe { mem::transmute(&self.data) }\n')
        out.write('  }\n')
    for member in type_node.members:
        mem_size = member.type.bit_size()
        out.write('  #[inline] pub fn from_%s(value: %s) -> %s {\n' % (decamelize(member.name).lower(), map_type(member.type), type_node.stripped_name))
        if mem_size<size:
            out.write('    let value = (value, [%s0u32]);\n' % (int((size-mem_size)/32-1) * '0u32, '))
        out.write('    %s {\n' % (type_node.stripped_name))
        out.write('      data: unsafe { mem::transmute(value) }\n')
        out.write('    }\n')
        out.write('  }\n')
    out.write('}\n')

def write_type_funcpointer(out, options, type_node):
    write_go_comment(out, type_node.comment, doc=True)
    out.write('pub type %s = extern fn(' % type_node.stripped_name)
    first = True
    for param in type_node.params:
        if first:
            first = False
        else:
            out.write(', ')
        out.write('%s: %s' % (map_keyword(decamelize(param.name).lower()), map_type(param.type)))
    out.write(')')
    if not type_node.returns.type.is_void():
        out.write(' -> %s' % (map_type(type_node.returns.type)))
    out.write(';\n')



def write_type(out, options, type_node):
    if type_node.name in TYPE_MAP:
        return
    if type_node.category == 'basetype' or type_node.category == 'handle':
        write_type_basetype(out, options, type_node)
    elif type_node.category == 'enum':
        write_type_enum(out, options, type_node)
    elif type_node.category == 'bitmask':
        write_type_bitmask(out, options, type_node)
    elif type_node.category == 'struct':
        write_type_struct(out, options, type_node)
    elif type_node.category == 'union':
        write_type_union(out, options, type_node)
    elif type_node.category == 'funcpointer':
        write_type_funcpointer(out, options, type_node)


def get_enum_value(enumvalue):
    value = enumvalue.element.get('value')
    if value is not None:
        value = value.strip()
        if value.startswith('-'):
            value = value + "i32 as u32"
        return value
    bitpos = enumvalue.element.get('bitpos')
    if bitpos is not None:
        return '1<<%s' % bitpos
    return 'TODO'

def write_enum_simple(out, options, enum):
    if len(enum.children) <= 0 or enum.type is None:
        return
    out.write('\n// enum: %s\n' % enum.name)
    for value in enum.children:
        write_go_comment(out, value.comment, doc=True)
        out.write('pub const %s : %s = %s;\n' % (value.stripped_name.upper(), enum.stripped_name, get_enum_value(value)))
    out.write('pub const %s_MAX_ENUM : %s = 0x7FFFFFFF;\n' % (decamelize(enum.stripped_name).upper(), enum.stripped_name))

def write_enum_complex(out, options, enum):
    if len(enum.children) <= 0 or enum.type is None:
        return
    for value in enum.children:
        write_go_comment(out, value.comment, doc=True, indent='  ')
        out.write('  %s = %s,\n' % (camelize(value.short_name_parts), get_enum_value(value)))

#MANUAL_COMMAND_IMPL=set([
#    'vkCreateGraphicsPipelines',
#    'vkCreateComputePipelines',
#    'vkAllocateDescriptorSets',
#    'vkAllocateCommandBuffers',
#    'vkCmdBindVertexBuffers',
#    'vkCmdUpdateBuffer',
#    'vkCreateSharedSwapchainsKHR',
#])

def write_command(out, options, command):
    #if command.name in MANUAL_COMMAND_IMPL:
    #    return
    write_go_comment(out, command.comment, doc=True)
    out.write('#[link_name="%s"]\n' % command.name)
    out.write('pub fn %s (' % decamelize(command.stripped_name).lower())
    first = True
    for param in command.params:
        if first:
            first = False
        else:
            out.write(', ')
        out.write('%s: %s' % (map_keyword(decamelize(param.name).lower()), map_type(param.type)))
    out.write(')')
    if not command.returns.type.is_void():
        out.write(' -> %s' % (map_type(command.returns.type)))
    out.write(';\n')

def write(out, options, registry):
    write_go_comment(out, registry.root.find('comment').text)
    out.write("""
#[allow(unused_imports)]
use basic_types;
use std::os::raw::{c_void,c_char};
use std::mem;

""")
    for type_node in registry.types:
        if type_node.category!='enum' and type_node.category!='bitmask':
            write_type(out, options, type_node)
    for type_node in registry.types:
        if type_node.category=='enum':
            write_type(out, options, type_node)
    for type_node in registry.types:
        if type_node.category=='bitmask':
            write_type(out, options, type_node)
    if options.get('simple_enums') is True:
        for enum_node in registry.enums:
            write_enum_simple(out, options, enum_node)
    out.write('\n')
    out.write('#[link(name="vulkan")]\n')
    out.write('extern "C" {\n')
    for command_node in registry.commands:
        write_command(out, options, command_node)
    out.write('}\n')

def urlopen(url):
    # pylint: disable=E0611,W0702,F0401,E1101
    try:
        import urllib.request
        request = urllib.request.Request(url)
        return urllib.request.urlopen(request)
    except:
        from urllib import urllib_urlopen
        return urllib_urlopen(url)

def download_file(url, dest):
    file_name = url.split('/')[-1]
    with urlopen(url) as response:
        stdlog.write("Downloading: %s\n" % file_name)
        file_size_dl = 0
        block_sz = 8192
        with open(dest, 'wb') as out:
            while True:
                buf = response.read(block_sz)
                if not buf:
                    break
                file_size_dl += len(buf)
                out.write(buf)
                status = r"%10d" % file_size_dl
                status = status + chr(8) * (len(status) + 1)
                stdlog.write(status)

def main():
    current_dir = os.path.dirname(os.path.abspath(__file__))
    vk_xml_path = os.path.join(current_dir, 'vk.xml')
    vk_xml_url = 'https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/1.0/src/spec/vk.xml'
    vk_out_rs = os.path.join(current_dir, 'src', 'vk.rs')
    if not os.path.isfile(vk_xml_path):
        download_file(vk_xml_url, vk_xml_path)
    registry = Registry(vk_xml_path)
    registry.resolve()
    options = {
        #'simple_enums': True
    }
    with open(vk_out_rs, 'w') as out:
        write(out, options, registry)

if __name__ == "__main__":
    main()
