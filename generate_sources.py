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


def get_node_name(node):
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


class NamedNode(object):

    def __init__(self, element, name=None, parent=None):
        if name is None:
            name = get_node_name(element)
            if name is None:
                raise ValueError('node with tag %s has no name' % element.tag)
        self.name = name
        self.element = element
        self.tag = element.tag
        self.parent = parent
        self.registry = None
        self.stripped_name = strip_api(name)
        self.stripped_vendorid = None
        cur = parent
        while cur is not None and cur is not self:
            if isinstance(cur, Registry):
                self.registry = cur
                break
            cur = cur.parent
        if self.parent is not None and self.registry is None:
            raise ValueError('node with tag %s and name %s is not connected to a registry' % (
                element.tag, element.name))
        if self.registry is not None:
            basename = self.stripped_name
            for vendorid in self.registry.vendorids:
                if basename.endswith(vendorid):
                    basename = basename[0:-len(vendorid)]
                    self.stripped_vendorid = vendorid
                    if basename.endswith('_'):
                        basename = basename[0:-1]
                        self.stripped_vendorid = '_' + self.stripped_vendorid
                    break


class VendorIdNode(NamedNode):

    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)


class TypeNode(NamedNode):

    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)
        self.category = element.get('category')
        self.parent_type = element.get('parent')
        self.requires = element.get('requires')
        type_element = element.find('type')
        if type_element is not None:
            self.type = type_element.text
            if type_element.tail is not None and type_element.tail.startswith('*'):
                self.type = self.type + '*'
        self.members_list = []
        self.members_dict = {}


class TypeMemberNode(NamedNode):

    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)
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
        self.expand = element.get('expand')
        if self.expand is None and self.type is not None:
            basename = self.name
            if self.stripped_vendorid is not None:
                basename = basename[0:-len(self.stripped_vendorid)]
            if self.type == 'bitmask' and basename.endswith('FlagBits'):
                basename = basename[0:-8]
            self.expand = re.sub('([a-z0-9])([A-Z])',
                                 r'\1_\2', basename).upper()


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
            node = VendorIdNode(element, parent=self)
            fill_named_node(self.vendorid_list, self.vendorid_dict, node)
            self.vendorids.add(node.name)

        self.all_types_list = []
        self.all_types_dict = {}
        for element in self.root.findall('types/type'):
            node = TypeNode(element, parent=self)
            fill_named_node(self.all_types_list, self.all_types_dict, node)
            for element2 in element.findall('member'):
                node2 = TypeMemberNode(element2, parent=node)
                fill_named_node(node.members_list, node.members_dict, node2)

        self.all_enums_list = []
        self.all_enums_dict = {}
        self.all_enumvalues_list = []
        self.all_enumvalues_dict = {}
        for element in self.root.findall('enums'):
            node = EnumNode(element, parent=self)
            fill_named_node(self.all_enums_list, self.all_enums_dict, node)
            for element2 in element.findall('enum'):
                node2 = EnumValueNode(element2, parent=node)
                fill_named_node(self.all_enumvalues_list,
                                self.all_enumvalues_dict, node2)
                fill_named_node(node.values_list, node.values_dict, node2)


def write_go_comment(out, content, doc=False):
    if isinstance(content, str):
        content = [content]
    for part in content:
        if isinstance(part, ET.Element):
            part = part.text
        if len(part) <= 0:
            continue
        lines = part.splitlines()
        if len(lines) == 1:
            if doc:
                out.write('/// ')
            else:
                out.write('// ')
            out.write(lines[0])
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


def get_enum_value(enumvalue):
    value = enumvalue.element.get('value')
    if value is not None:
        return value
    bitpos = enumvalue.element.get('bitpos')
    if bitpos is not None:
        return '1<<%s' % bitpos
    return 'TODO'


def write_enum(out, options, enum):
    basetype = 'Enum'
    stripped_name = enum.stripped_name
    if enum.type == 'bitmask':
        basetype = 'Flags'
        tmp = stripped_name
        if enum.stripped_vendorid is not None:
            tmp = tmp[0:-len(enum.stripped_vendorid)]
        if tmp.endswith('FlagBits'):
            stripped_name = tmp[0:-4] + 's' + (enum.stripped_vendorid or '')
    if enum.type is None:
        return  # skip "API Constants"-section
        # for value in enum.values_list:
        #    comment = value.element.get('comment')
        #    if comment is not None:
        #        comment = ' //! ' + comment
        #    out.write('pub const %s : basic_types::%s = %s;%s\n' % (strip_api(value.name), basetype, get_enum_value(value), comment or ''))
    elif options.get('enum_modules', False):
        out.write('#[allow(non_snake_case)]\n')
        out.write('pub mod %s {\n' % stripped_name)
        out.write('  use basic_types;\n')
        out.write('  pub type %s = basic_types::%s;\n' % (basetype, basetype))
        for value in enum.values_list:
            comment = value.element.get('comment')
            if comment is not None:
                out.write('  /// %s\n' % comment)
            out.write('  pub const %s_%s : %s = %s;\n' % (
                basetype[0], value.short_name.upper(), basetype, get_enum_value(value)))
        out.write('}\n')
    else:
        out.write('pub type %s = basic_types::%s;\n' %
                  (stripped_name, basetype))
        for value in enum.values_list:
            comment = value.element.get('comment')
            if comment is not None:
                out.write('/// %s\n' % comment)
            out.write('pub const %s : %s = %s;\n' %
                      (value.name.upper(), stripped_name, get_enum_value(value)))

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


def map_type(type_name):
    if type_name in TYPE_MAP:
        return TYPE_MAP[type_name]
    if type_name.endswith('*'):
        return '*const ' + strip_api(type_name[:-1])
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


def map_type_with_name(type_name, node):
    name = node.name
    namenode = node.element.find('name')
    type_name = map_type(type_name)
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
    comment = type_node.element.get('comment')
    if comment is not None:
        out.write('/// %s\n' % comment)
    out.write('pub type %s = %s;\n' % (strip_api(type_node.name), map_type(type_node.type)))


def write_type_struct(out, options, type_node):
    comment = type_node.element.get('comment')
    if comment is not None:
        out.write('/// %s\n' % comment)
    out.write('#[repr(C)]\n')
    out.write('pub struct %s {\n' % strip_api(type_node.name))
    for member in type_node.members_list:
        comment = member.element.get('comment')
        if comment is not None:
            out.write('  /// %s\n' % comment)
        type_name, name = map_type_with_name(member.type, member)
        out.write('  %s: %s,\n' % (map_keyword(name), type_name))
    out.write('}\n')


def write_type_union(out, options, type_node):
    comment = type_node.element.get('comment')
    if comment is not None:
        out.write('/// %s\n' % comment)
    out.write('pub enum %s {\n' % strip_api(type_node.name))
    for member in type_node.members_list:
        comment = member.element.get('comment')
        if comment is not None:
            out.write('  /// %s\n' % comment)
        type_name, name = map_type_with_name(member.type, member)
        out.write('  %s(%s),\n' % (name, type_name))
    out.write('}\n')


def write_type_funcpointer(out, options, type_node):
    comment = type_node.element.get('comment')
    if comment is not None:
        out.write('/// %s\n' % comment)
    returntype = type_node.element.text
    if returntype.startswith('typedef '):
        returntype = returntype[8:].strip()
    pos = returntype.find('(')
    if pos > 0:
        returntype = returntype[0:pos].strip()
    else:
        logging.warning('expected \'(\' in functionpointer %s', type_node.name)
    argtypes = [map_type(n.tail is not None and n.tail.startswith('*') and n.text + '*' or n.text) for n in type_node.element.findall('type')]
    out.write('pub type %s = fn(%s)%s;\n' % (strip_api(type_node.name), ', '.join(argtypes), returntype != 'void' and (' -> ' + map_type(returntype)) or ''))


def write_type(out, options, type_node):
    if type_node.category == 'basetype' or type_node.category == 'handle':
        write_type_basetype(out, options, type_node)
    elif type_node.category == 'struct':
        write_type_struct(out, options, type_node)
    elif type_node.category == 'union':
        write_type_union(out, options, type_node)
    elif type_node.category == 'funcpointer':
        write_type_funcpointer(out, options, type_node)


def write(out, options, registry):
    write_go_comment(out, registry.root.find('comment').text)
    out.write("""
#[allow(unused_imports)]
use basic_types;
""")
    for enum_node in registry.all_enums_list:
        write_enum(out, options, enum_node)
    for type_node in registry.all_types_list:
        write_type(out, options, type_node)


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
    options = {
        #'enum_modules': True
    }
    with open(vk_out_rs, 'w') as out:
        write(out, options, registry)

if __name__ == "__main__":
    main()
