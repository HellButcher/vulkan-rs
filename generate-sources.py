#!/usr/bin/env python3
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
    logging.basicConfig(format='%(asctime)s %(levelname)s: %(message)s', level=logging.DEBUG, stream=sys.stdout)

def get_node_name(node):
    name = node.get('name')
    if name is not None:
        return name
    namenode = node.find('name')
    if namenode is not None:
        return namenode.text

def fill_named_node(list, dict, node):
    if node.name in dict:
        raise KeyError('%s with name %s is already defined' % (node.tag, node.name))
    logging.debug('adding %s with name %s', node.tag, node.name)
    dict[node.name]=node
    if not hasattr(node, 'index'):
        node.index=len(list)
    list.append(node)
    return node

class NamedNode:
    def __init__(self, element, name=None, parent=None):
        if name is None:
            name = get_node_name(element)
            if name is None:
                raise ValueError('node with tag %s has no name' % element.tag)
        self.name = name
        self.element = element
        self.tag = element.tag
        self.parent = parent

class VendorIdNode(NamedNode):
    pass

class TypeNode(NamedNode):
    pass

class EnumNode(NamedNode):
    def __init__(self, element, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)
        self.values_list = []
        self.values_dict = {}
        self.type = element.get('type')
        self.expand = element.get('expand')
        self._stripped_vendorid = None
        if self.expand is None and self.type is not None:
            basename = self.name
            for vendorid in self.parent.vendorids:
                if basename.endswith(vendorid):
                    basename = basename[0:-len(vendorid)]
                    self._stripped_vendorid = vendorid
                    break
            if self.type=='bitmask' and basename.endswith('FlagBits'):
                basename = basename[0:-8]
            self.expand = re.sub('([a-z0-9])([A-Z])', r'\1_\2', basename).upper()
class EnumValueNode(NamedNode):
    def __init__(self, element, extension=None, *args, **kwargs):
        NamedNode.__init__(self, element, *args, **kwargs)
        self.extension = extension
        if self.parent is not None and self.parent.expand is not None and self.name.startswith(self.parent.expand+'_'):
            self.short_name = self.name[len(self.parent.expand)+1:]
            if self.parent._stripped_vendorid is not None and self.short_name.endswith('_'+self.parent._stripped_vendorid):
                self.short_name = self.short_name[:-len(self.parent._stripped_vendorid)-1]
        else:
            self.short_name = strip_api(self)
        if self.parent is not None and self.parent.type=='bitmask' and self.short_name.endswith('_BIT'):
            self.short_name = self.short_name[0:-4]


class Registry:
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

        self.vendorids = set(['EXT','KHR','ARB'])
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

        self.all_enums_list = []
        self.all_enums_dict = {}
        self.all_enumvalues_list = []
        self.all_enumvalues_dict = {}
        for element in self.root.findall('enums'):
            node = EnumNode(element, parent=self)
            fill_named_node(self.all_enums_list, self.all_enums_dict, node)
            for element2 in element.findall('enum'):
                node2 = EnumValueNode(element2, parent=node)
                fill_named_node(self.all_enumvalues_list, self.all_enumvalues_dict, node2)
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
        return '1<<%s'%bitpos
    return 'TODO'

def write_enum(out, options, enum):
    basetype = 'Enum'
    stripped_name = strip_api(enum.name)
    if enum.type=='bitmask':
        basetype = 'Flags'
        if stripped_name.endswith('FlagBits'):
            stripped_name = stripped_name[0:-8]
    if enum.type is None:
        return # skip "API Constants"-section
        #for value in enum.values_list:
        #    comment = value.element.get('comment')
        #    if comment is not None:
        #        comment = ' //! ' + comment
        #    out.write('pub const %s : basic_types::%s = %s;%s\n' % (strip_api(value.name), basetype, get_enum_value(value), comment or ''))
    elif options.get('enum_modules', False):
        out.write('#[allow(non_snake_case)]\n')
        out.write('pub mod %s {\n' % stripped_name)
        out.write('  use basic_types;\n')
        out.write('  pub type %s = basic_types::%s;\n' % (basetype,basetype))
        for value in enum.values_list:
            comment = value.element.get('comment')
            if comment is not None:
                out.write('  /// %s\n' % comment)
            out.write('  pub const %s_%s : %s = %s;\n' % (basetype[0], value.short_name.upper(), basetype, get_enum_value(value)))
        out.write('}\n')
    else:
        out.write('pub type %s = basic_types::%s;\n' % (stripped_name,basetype))
        for value in enum.values_list:
            comment = value.element.get('comment')
            if comment is not None:
                out.write('  /// %s\n' % comment)
            out.write('pub const %s : %s = %s;\n' % (value.name.upper(), stripped_name, get_enum_value(value)))

def write(out, options, registry):
    write_go_comment(out, registry.root.find('comment').text)
    out.write("""
#[allow(unused_imports)]
use basic_types;
""")
    for enum in registry.all_enums_list:
        write_enum(out,options,  enum)

def download_file(url, dest):
    import urllib.request
    file_name = url.split('/')[-1]
    request = urllib.request.Request(url)
    with urllib.request.urlopen(request) as response:
        logging.info("Downloading: %s", file_name)
        file_size_dl = 0
        block_sz = 8192
        with open(dest, 'wb') as file:
            while True:
                buffer = response.read(block_sz)
                if not buffer:
                    break
                file_size_dl += len(buffer)
                file.write(buffer)
                status = r"%10d" % file_size_dl
                status = status + chr(8)*(len(status)+1)
                logging.info(status)

if __name__ == "__main__":
    current_dir = os.path.dirname(os.path.abspath(__file__))
    vk_xml_path = os.path.join(current_dir,'vk.xml')
    vk_xml_url = 'https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/1.0/src/spec/vk.xml'
    vk_out_rs = os.path.join(current_dir,'src','vk.rs')
    if not os.path.isfile(vk_xml_path):
        download_file(vk_xml_url, vk_xml_path)
    registry = Registry(vk_xml_path)
    options = {
        'enum_modules': True
    }
    with open(vk_out_rs, 'w') as out:
        write(out, options, registry)
