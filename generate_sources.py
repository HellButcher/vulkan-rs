#!/usr/bin/env python3
# pylint: disable=C0301,C0111,R0903,R0902,R0912,W0613

import os
import sys
import re
import logging
import xml.etree.ElementTree as ET

if __name__ == '__main__':
    if sys.stdout.isatty():
        logging.addLevelName(logging.DEBUG, "\033[1;36m%s\033[1;0m" % logging.getLevelName(logging.DEBUG))
        logging.addLevelName(logging.INFO, "\033[1;34m%s\033[1;0m" % logging.getLevelName(logging.INFO))
        logging.addLevelName(logging.WARNING, "\033[1;33m%s\033[1;0m" % logging.getLevelName(logging.WARNING))
        logging.addLevelName(logging.ERROR, "\033[1;31m%s\033[1;0m" % logging.getLevelName(logging.ERROR))
        logging.addLevelName(logging.CRITICAL, "\033[1;41m%s\033[1;0m" % logging.getLevelName(logging.CRITICAL))
    logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s', level=logging.INFO, stream=sys.stdout)


def get_node_name(node, _name_node=None):
    if _name_node is not None:
        name = node.find(_name_node)
        if name is not None:
            return name.text
    name = node.get('name')
    if name is not None:
        return name
    namenode = node.find('name')
    if namenode is not None:
        return namenode.text


def fill_named_node(lst, dct, node):
    if node.name in dct:
        raise KeyError('%s with name %s is already defined' %
                       (node.tag, node.name))
    logging.debug('adding %s with name %s', node.tag, node.name)
    dct[node.name] = node
    if not hasattr(node, 'index'):
        node.index = len(lst)
    lst.append(node)
    return node


class Node(object):

    def __init__(self, element, parent=None):
        self.element = element
        self.tag = element.tag
        self.parent = parent
        self.registry = None
        cur = parent
        while cur is not None and cur is not self:
            if isinstance(cur, Registry):
                self.registry = cur
                break
            cur = cur.parent
        if self.parent is not None and self.registry is None:
            raise ValueError('node with tag %s and name %s is not connected to a registry' % (
                element.tag, element.name))

    def link(self):
        pass


class NamedNode(Node):

    def __init__(self, element, name=None, _name_node=None, *args, **kwargs):
        Node.__init__(self, element, *args, **kwargs)
        if name is None:
            name = get_node_name(element, _name_node=_name_node)
            if name is None:
                raise ValueError('node with tag %s has no name' % element.tag)
        self.name = name
        self.stripped_name = strip_api(name)
        self.stripped_vendorid = None
        if self.registry is not None:
            basename = self.stripped_name
            for vendorid in self.registry.vendorids:
                if basename.endswith(vendorid):
                    basename = basename[0:-len(vendorid)]
                    self.stripped_vendorid = vendorid
                    if basename.endswith('_'):
                        self.stripped_name = basename[0:-1]
                        self.stripped_vendorid = '_' + self.stripped_vendorid
                    break


class VendorIdNode(NamedNode):

    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)


class TypeNode(NamedNode):

    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)
        self.category = element.get('category')
        self.parent_types = element.get('parent')
        self.parent_types_nodes = None
        self.requires = element.get('requires')
        self.requires_nodes = None
        self.flags_node = None
        self.bits_node = None
        type_element = element.find('type')
        if type_element is not None:
            self.type = type_element.text
            if type_element.tail is not None and type_element.tail.startswith('*'):
                self.type = self.type + '*'
        self.members_list = []
        self.members_dict = {}

    def link(self):
        NamedNode.link(self)
        if self.registry is None:
            return
        if self.requires is not None:
            nodes = self.requires_nodes = []
            for require in self.requires.split(','):
                require = require.strip()
                if require in self.registry.all_types_dict:
                    nodes.append(self.registry.all_types_dict[require])
                else:
                    logging.warning('there is no type-definition for the required type \'%s\'', require)
        if self.parent_types is not None:
            nodes = self.parent_types_nodes = []
            for parent_type in self.parent_types.split(','):
                if parent_type in self.registry.all_types_dict:
                    nodes.append(self.registry.all_types_dict[parent_type])
                else:
                    logging.warning('there is no type-definition for the parent type \'%s\'', parent_type)
        if self.category == 'bitmask':
            if self.requires_nodes is not None:
                if len(self.requires_nodes) != 1:
                    logging.warning('flags-type \'%s\' defines multiple required types', self.name)
                elif self.requires_nodes[0].category != 'enum':
                    logging.warning('flags-type \'%s\' requires a non-enum type', self.name)
                else:
                    self.bits_node = self.requires_nodes[0]
            if self.bits_node is None:
                bits_name = self.name.replace("Flags", "FlagBits")
                if bits_name == self.name:
                    logging.warning('can not detect bits-type for flags \'%s\'', self.name)
                else:
                    if bits_name not in self.registry.all_types_dict:
                        logging.info('there is no type-definition for bits-type \'%s\': automatically derived one.', bits_name)
                        type_element = ET.SubElement(self.registry.root.find('types'), 'type')
                        type_element.set('name', bits_name)
                        type_element.set('category', 'enum')
                        self.registry.add_type_element(type_element)
                    self.bits_node = self.registry.all_types_dict[bits_name]
            if self.bits_node is not None:
                if self.bits_node.flags_node is not None:
                    logging.warning('bits-type \'%s\' has already an associated flags-type \'%s\': can not assign \'%s\'', self.bits_node.name, self.bits_node.flags_node.name, self.name)
                else:
                    self.bits_node.flags_node = self
        for node in self.members_list:
            node.link()


class TypeMemberNode(NamedNode):

    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)
        self.type = None
        type_element = element.find('type')
        if type_element is not None:
            self.type = type_element.text
            if type_element.tail is not None and type_element.tail.startswith('*'):
                self.type = self.type + '*'


class EnumNode(NamedNode):

    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)
        self.values_list = []
        self.values_dict = {}
        self.type = element.get('type')
        self.type_node = None
        self.expand = element.get('expand')
        if self.expand is None and self.type is not None:
            basename = self.name
            if self.stripped_vendorid is not None:
                basename = basename[0:-len(self.stripped_vendorid)]
            if self.type == 'bitmask' and basename.endswith('FlagBits'):
                basename = basename[0:-8]
            self.expand = re.sub('([a-z0-9])([A-Z])',
                                 r'\1_\2', basename).upper()

    def link(self):
        NamedNode.link(self)
        if self.registry is None or self.type is None:
            return
        if self.name not in self.registry.all_types_dict:
            logging.info('there is no type-definition for the enum \'%s\': automatically derived one.', self.name)
            type_element = ET.SubElement(self.registry.root.find('types'), 'type')
            type_element.set('name', self.name)
            type_element.set('category', 'enum')
            self.registry.add_type_element(type_element)
        self.type_node = self.registry.all_types_dict[self.name]
        self.type_node.enum_node = self
        for node in self.values_list:
            node.link()


class EnumValueNode(NamedNode):

    def __init__(self, element, extension=None, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)
        self.extension = extension
        if self.parent is not None and self.parent.expand is not None and self.name.startswith(self.parent.expand + '_'):
            self.short_name = self.name[len(self.parent.expand) + 1:]
            if self.parent.stripped_vendorid is not None and self.short_name.endswith('_' + self.parent.stripped_vendorid):
                self.short_name = self.short_name[
                    :-len(self.parent.stripped_vendorid) - 1]
        else:
            self.short_name = strip_api(self)
        if self.parent is not None and self.parent.type == 'bitmask' and self.short_name.endswith('_BIT'):
            self.short_name = self.short_name[0:-4]


class CommandNode(NamedNode):

    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, _name_node='proto/name', *args, **kwargs)
        self.param_list = []
        self.param_dict = {}
        tmp = element.get('successcodes')
        self.successcodes = tmp is not None and tmp.split(',') or None
        tmp = element.get('errorcodes')
        self.errorcodes = tmp is not None and tmp.split(',') or None
        self.return_type = None
        type_element = element.find('proto/type')
        if type_element is not None:
            self.return_type = type_element.text
            if type_element.tail is not None and type_element.tail.startswith('*'):
                self.return_type = self.return_type + '*'

    def link(self):
        NamedNode.link(self)
        for node in self.param_list:
            node.link()


class ParameterNode(TypeMemberNode):

    def __init__(self, element, *args, **kwargs):
        TypeMemberNode.__init__(self, element, *args, **kwargs)
        self.length = element.get('len')
        self.length_param_for = None
        self.length_param = None
    def link(self):
        TypeMemberNode.link(self)
        if not isinstance(self.parent, CommandNode):
            logging.warning('Parameter \'%s\' is not a child of a command', self.name)
            return
        if self.length is not None:
            if not self.type.endswith('*'):
                logging.warning('param \'%s\' of \'%s\' is not a pointer', self.length, self.parent.name)
            if self.length == 'null-terminated':
                return
            if self.length not in self.parent.param_dict:
                logging.warning('there is no param \'%s\' in \'%s\'', self.length, self.parent.name)
            else:
                self.length_param = self.parent.param_dict[self.length]
                if self.length_param.length_param_for is not None:
                    logging.warning('param \'%s\' is already used as langth param in \'%s\'', self.length, self.parent.name)
                else:
                    self.length_param.length_param_for = self


class Registry(object):

    def __init__(self, tree_or_filename):
        if isinstance(tree_or_filename, ET.ElementTree):
            self.tree = tree_or_filename
            logging.info('loaded registry from in-memory tree')
        else:
            logging.info('loading registry from file %s...', tree_or_filename)
            with open(tree_or_filename, "r") as regfile:
                self.tree = ET.parse(regfile)
            logging.debug('loaded registry')
        self.root = self.tree.getroot()
        self.element = self.root

        self.vendorids = set(['EXT', 'KHR', 'ARB'])
        self.vendorid_list = []
        self.vendorid_dict = {}
        for element in self.root.findall('vendorids/vendorid'):
            self.add_vendorid_element(element)

        self.all_types_list = []
        self.all_types_dict = {}
        for element in self.root.findall('types/type'):
            self.add_type_element(element)

        self.all_enums_list = []
        self.all_enums_dict = {}
        self.all_enumvalues_list = []
        self.all_enumvalues_dict = {}
        for element in self.root.findall('enums'):
            self.add_enum_element(element)

        self.all_commands_list = []
        self.all_commands_dict = {}
        for element in self.root.findall('commands/command'):
            self.add_command_element(element)

    def add_vendorid_element(self, element):
        node = VendorIdNode(element, parent=self)
        fill_named_node(self.vendorid_list, self.vendorid_dict, node)
        self.vendorids.add(node.name)

    def add_type_element(self, element):
        node = TypeNode(element, parent=self)
        fill_named_node(self.all_types_list, self.all_types_dict, node)
        for element2 in element.findall('member'):
            node2 = TypeMemberNode(element2, parent=node)
            fill_named_node(node.members_list, node.members_dict, node2)

    def add_enum_element(self, element):
        node = EnumNode(element, parent=self)
        fill_named_node(self.all_enums_list, self.all_enums_dict, node)
        for element2 in element.findall('enum'):
            node2 = EnumValueNode(element2, parent=node)
            fill_named_node(self.all_enumvalues_list,
                            self.all_enumvalues_dict, node2)
            fill_named_node(node.values_list, node.values_dict, node2)

    def add_command_element(self, element):
        node = CommandNode(element, parent=self)
        fill_named_node(self.all_commands_list, self.all_commands_dict, node)
        for element2 in element.findall('param'):
            node2 = ParameterNode(element2, parent=node)
            fill_named_node(node.param_list, node.param_dict, node2)

    def link(self):
        for node in self.vendorid_list:
            node.link()
        for node in self.all_types_list:
            node.link()
        for node in self.all_enums_list:
            node.link()
        for node in self.all_commands_list:
            node.link()

def write_go_comment(out, content, doc=False):
    if isinstance(content, str):
        content = [content]
    elif isinstance(content, Node):
        content = [content.element]
    elif isinstance(content, ET.Element):
        content = [content]
    for part in content:
        if isinstance(part, ET.Element):
            if part.tag == 'comment':
                part = part.text
            elif part.get('comment') is not None:
                part = part.get('comment')
            else:
                continue
        if len(part) <= 0:
            continue
        lines = part.splitlines()
        if len(lines) == 1:
            line = lines[0]
            if line.startswith('//'):
                line = line[2:].strip()
            if doc:
                out.write('/// ')
            else:
                out.write('// ')
            out.write(line)
            out.write('\n')
        else:
            if not doc:
                out.write('/*\n')
            for line in lines:
                if doc:
                    out.write('/// ')
                else:
                    out.write(' * ')
                out.write(line)
                out.write('\n')
            if not doc:
                out.write(' */\n')


def strip_api(name_or_node):
    if isinstance(name_or_node, NamedNode):
        name = name_or_node.name
    else:
        name = name_or_node
    if name.startswith('PFN_'):
        return 'PFN_' + strip_api(name[4:])
    if name.lower().startswith('vk'):
        name = name[2:]
    if name.startswith('_'):
        name = name[1:]
    return name

TYPE_MAP = {
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
    'VK_DEFINE_HANDLE': 'basic_types::Handle',
    'VK_DEFINE_NON_DISPATCHABLE_HANDLE': 'basic_types::DispatchableHandle',
    'void*': '*const u32',
    'char*': '*const u8',
    'uint8_t*':   '*const u8',
    'uint16_t*':  '*const u16',
    'uint32_t*':  '*const u32',
    'uint64_t*':  '*const u64',
    'int8_t*':    '*const i8',
    'int16_t*':   '*const i16',
    'int32_t*':   '*const i32',
    'int64_t*':   '*const i64',
    'ssize_t*':   '*const isize',
    'size_t*':    '*const usize',
    'intptr_t*':  '*const isize',
    'uintptr_t*': '*const usize',
    'float*':     '*const f32',
    'double*':    '*const f64',
}


def map_type(type_name, registry=None, as_reference=False):
    if type_name in TYPE_MAP:
        return TYPE_MAP[type_name]
    if as_reference and registry is not None and type_name.endswith('*'):
        type_name2 = type_name[:-1]
        if type_name2 in registry.all_types_dict:
            ref = registry.all_types_dict[type_name2]
            if ref.category=='struct' or ref.category=='union':
                return '&'+ref.stripped_name
    if type_name.endswith('*'):
        return '*const ' + strip_api(type_name[:-1])
    elif registry is not None and type_name in registry.all_types_dict:
        return registry.all_types_dict[type_name].stripped_name
    else:
        return strip_api(type_name)

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


def map_type_with_name(type_name, node, as_reference=False):
    name = node.name
    namenode = node.element.find('name')
    type_name = map_type(type_name, registry=node.registry, as_reference=as_reference)
    basetype = type_name
    array_idx = name.find('[')
    array_idx2 = name.find(']')
    if array_idx > 0 and array_idx < array_idx2:
        array_len = int(name[array_idx + 1:array_idx2])
        name = name[0:array_idx]
        type_name = '[%s; %d]' % (type_name, array_len)
    elif namenode is not None and namenode.tail is not None:
        tail = namenode.tail
        array_idx = tail.find('[')
        array_idx2 = tail.find(']')
        if array_idx >= 0 and array_idx < array_idx2:
            array_len = int(tail[array_idx + 1:array_idx2])
            type_name = '[%s; %d]' % (type_name, array_len)
    name = map_keyword(name, basetype)
    return type_name, name

def write_type_basetype(out, options, type_node):
    write_go_comment(out, type_node, doc=True)
    out.write('pub type %s = %s;\n' % (type_node.stripped_name, map_type(type_node.type, type_node.registry)))

def write_type_enum(out, options, type_node):
    write_go_comment(out, type_node, doc=True)
    out.write('pub type %s = basic_types::Enum;\n' % (type_node.stripped_name))

def write_type_bitmask(out, options, type_node):
    write_go_comment(out, type_node, doc=True)
    out.write('pub type %s = basic_types::Flags;\n' % (type_node.stripped_name))

def write_type_struct(out, options, type_node):
    write_go_comment(out, type_node, doc=True)
    out.write('#[repr(C)]\n')
    out.write('pub struct %s {\n' % type_node.stripped_name)
    for member in type_node.members_list:
        comment = member.element.get('comment')
        if comment is not None:
            out.write('  /// %s\n' % comment)
        type_name, name = map_type_with_name(member.type, member)
        out.write('  %s: %s,\n' % (map_keyword(name), type_name))
    out.write('}\n')


def write_type_union(out, options, type_node):
    write_go_comment(out, type_node, doc=True)
    out.write('pub enum %s {\n' % type_node.stripped_name)
    for member in type_node.members_list:
        comment = member.element.get('comment')
        if comment is not None:
            out.write('  /// %s\n' % comment)
        type_name, name = map_type_with_name(member.type, member)
        out.write('  %s(%s),\n' % (name, type_name))
    out.write('}\n')


def write_type_funcpointer(out, options, type_node):
    write_go_comment(out, type_node, doc=True)
    returntype = type_node.element.text
    if returntype.startswith('typedef '):
        returntype = returntype[8:].strip()
    pos = returntype.find('(')
    if pos > 0:
        returntype = returntype[0:pos].strip()
    else:
        logging.warning('expected \'(\' in functionpointer %s', type_node.name)
    out.write('pub type %s = fn(' % type_node.stripped_name)
    out.write(', '.join([map_type(n.tail is not None and n.tail.startswith('*') and n.text + '*' or n.text, type_node.registry) for n in type_node.element.findall('type')]))
    out.write(')')
    if returntype != 'void':
        out.write(' -> ')
        out.write(map_type(returntype, type_node.registry))
    out.write(';\n')


def write_type(out, options, type_node):
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
        return value
    bitpos = enumvalue.element.get('bitpos')
    if bitpos is not None:
        return '1<<%s' % bitpos
    return 'TODO'

def write_enum(out, options, enum):
    if len(enum.values_list) <= 0 or enum.type is None:
        return
    out.write('\n// enum: %s\n' % enum.name)
    for value in enum.values_list:
        write_go_comment(out, value, doc=True)
        out.write('pub const %s : %s = %s;\n' % (value.stripped_name.upper(), enum.stripped_name, get_enum_value(value)))

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
    write_go_comment(out, command, doc=True)
    out.write('pub fn %s (' % command.stripped_name)
    first=True
    for param in command.param_list:
        if param.length_param_for is not None:
            continue
        type_name = param.type
        is_array_type = type_name.endswith('*') and param.length is not None and param.length != 'null-terminated'
        if is_array_type:
            type_name = type_name[:-1]
        type_name, name = map_type_with_name(type_name, param, as_reference=True)

        if first:
            first = False
        else:
            out.write(', ')
        out.write(name)
        out.write(': ')
        if is_array_type:
            out.write('&[%s]'%type_name)
        else:
            out.write(type_name)
    out.write(')')
    if command.return_type != 'void':
        out.write(' -> ')
        out.write(map_type(command.return_type, command.registry))
    out.write(' {\n')
    out.write('}\n')


def write(out, options, registry):
    write_go_comment(out, registry.root.find('comment').text)
    out.write("""
#[allow(unused_imports)]
use basic_types;

""")
    for type_node in registry.all_types_list:
        write_type(out, options, type_node)
    for enum_node in registry.all_enums_list:
        write_enum(out, options, enum_node)
    for command_node in registry.all_commands_list:
        write_command(out, options, command_node)


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
        logging.info("Downloading: %s", file_name)
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
                logging.info(status)

def main():
    current_dir = os.path.dirname(os.path.abspath(__file__))
    vk_xml_path = os.path.join(current_dir, 'vk.xml')
    vk_xml_url = 'https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/1.0/src/spec/vk.xml'
    vk_out_rs = os.path.join(current_dir, 'src', 'vk.rs')
    if not os.path.isfile(vk_xml_path):
        download_file(vk_xml_url, vk_xml_path)
    registry = Registry(vk_xml_path)
    registry.link()
    options = {
        #'enum_modules': True
    }
    with open(vk_out_rs, 'w') as out:
        write(out, options, registry)

if __name__ == "__main__":
    main()
