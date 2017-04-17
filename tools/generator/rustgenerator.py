#
# Copyright (c) 2013-2017 The Khronos Group Inc.
# Copyright (c) 2017 Christoph Hommelsheim
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import io,os,re,sys
from generator import *


TYPE_MAP = {
    # types defined in vk_platform.h
    'void':         'raw::c_void',
    'char':         'raw::c_char',
    'int':          'raw::c_int',
    'unsigned int': 'raw::c_uint',
    'float':        'f32',
    'uint8_t':      'u8',
    'uint32_t':     'u32',
    'uint64_t':     'u64',
    'int32_t':      'i32',
    'size_t':       'usize',

    'VK_DEFINE_HANDLE': 'VkHandle',
    'VK_DEFINE_NON_DISPATCHABLE_HANDLE': 'VkDispatchableHandle',
}

TYPE_SIZE_MAP = {
    'float':    4,
    'double':   8,
    'uint8_t':  1,
    'uint16_t': 2,
    'uint32_t': 4,
    'uint64_t': 8,
    'int8_t':  1,
    'int16_t': 2,
    'int32_t': 4,
    'int64_t': 8,
}

KEYWORDS = set(['as', 'break', 'const', 'continue', 'crate', 'else', 'enum',
    'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop', 'match',
    'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'static', 'struct',
    'type', 'trait', 'true', 'unsafe', 'use', 'where', 'while'])

IGNORED = set(['VK_NULL_HANDLE'])

# RustGeneratorOptions - subclass of GeneratorOptions.
#
# Adds options used by RustOutputGenerator objects during Rust language source
# generation.
#
# Additional members
#   prefixText - list of strings to prefix generated header with
#     (usually a copyright statement + calling convention macros).
class RustGeneratorOptions(GeneratorOptions):
    """Represents options during rust interface generation"""
    def __init__(self,
                 filename = None,
                 aliasFilename = None,
                 ffiFilename = None,
                 directory = '.',
                 apiname = None,
                 profile = None,
                 versions = '.*',
                 emitversions = '.*',
                 defaultExtensions = None,
                 addExtensions = None,
                 removeExtensions = None,
                 sortProcedure = regSortFeatures,
                 prefixText = "",
                 alignFuncParam = 0):
        GeneratorOptions.__init__(self, filename, directory, apiname, profile,
                                  versions, emitversions, defaultExtensions,
                                  addExtensions, removeExtensions, sortProcedure)
        self.prefixText      = prefixText
        self.alignFuncParam  = alignFuncParam
        self.aliasFilename   = aliasFilename
        self.ffiFilename     = ffiFilename

# RustOutputGenerator - subclass of OutputGenerator.
# Generates Rust-language API interfaces.
#
# ---- methods ----
# RustOutputGenerator(errFile, warnFile, diagFile) - args as for
#   OutputGenerator. Defines additional internal state.
# ---- methods overriding base class ----
# beginFile(genOpts)
# endFile()
# beginFeature(interface, emit)
# endFeature()
# genType(typeinfo,name)
# genStruct(typeinfo,name)
# genGroup(groupinfo,name)
# genEnum(enuminfo, name)
# genCmd(cmdinfo)
class RustOutputGenerator(OutputGenerator):
    """Generate specified API interfaces in a specific style, such as rust code"""
    # This is an ordered list of sections in the header file.
    SECTIONS = ['include', 'define', 'basetype', 'handle', 'enum', 'group',
                'bitmask', 'funcpointer', 'struct', 'command']
    ALIAS_SECTIONS = [ 'alias' ]
    #
    def __init__(self,
                 errFile = sys.stderr,
                 warnFile = sys.stderr,
                 diagFile = sys.stdout):
        OutputGenerator.__init__(self, errFile, warnFile, diagFile)
        # Internal state - accumulators for different inner block text
        self.sections = dict([(section, []) for section in self.SECTIONS+self.ALIAS_SECTIONS])
        self.aliasOutFile = None
        self.ffiOutFile = None
    #
    def beginFile(self, genOpts):
        OutputGenerator.beginFile(self, genOpts)
        #
        if self.genOpts.aliasFilename != None:
            filename = self.genOpts.directory + '/' + self.genOpts.aliasFilename
            self.aliasOutFile = io.open(filename, 'w', encoding='utf-8')
        else:
            self.aliasOutFile = None

        if self.genOpts.ffiFilename != None:
            filename = self.genOpts.directory + '/' + self.genOpts.ffiFilename
            self.ffiOutFile = io.open(filename, 'w', encoding='utf-8')
        else:
            self.ffiOutFile = None


        # User-supplied prefix text, if any (list of strings)
        if genOpts.prefixText:
            for s in genOpts.prefixText:
                write(s, file=self.outFile)
    def endFile(self):
        if self.aliasOutFile is not None:
            self.aliasOutFile.close()
        if self.ffiOutFile is not None:
            self.ffiOutFile.close()
        # Finish processing in superclass
        OutputGenerator.endFile(self)
    #
    def beginFeature(self, interface, emit):
        # Start processing in superclass
        OutputGenerator.beginFeature(self, interface, emit)
        # Rust-specific
        # Accumulate includes, defines, types, enums, function pointer typedefs,
        # end function prototypes separately for this feature. They're only
        # printed in endFeature().
        self.sections = dict([(section, []) for section in self.SECTIONS+self.ALIAS_SECTIONS])
    def endFeature(self):
        # Rust-specific
        # Actually write the interface to the output file.
        if len(self.sections['command'])>0:
            self.appendSection('command', '}\n')
        if self.emit:
            write('', file=self.outFile)
            write('// feature ', self.featureName, file=self.outFile)
            for section in self.SECTIONS:
                contents = self.sections[section]
                if contents:
                    if self.ffiOutFile is not None and section == 'command':
                        write('\n'.join(contents), file=self.ffiOutFile)
                        write('', file=self.ffiOutFile)
                    else:
                        write('\n'.join(contents), file=self.outFile)
                        write('', file=self.outFile)
            if self.aliasOutFile is not None:
                write('', file=self.aliasOutFile)
                write('// feature ', self.featureName, file=self.aliasOutFile)
                for section in self.ALIAS_SECTIONS:
                    contents = self.sections[section]
                    if contents:
                        write('\n'.join(contents), file=self.aliasOutFile)
                        write('', file=self.aliasOutFile)

        # Finish processing in superclass
        OutputGenerator.endFeature(self)
    #
    # Append a definition to the specified section
    def appendSection(self, section, text):
        # self.sections[section].append('SECTION: ' + section + '\n')
        self.sections[section].append(text)
    #
    def stripPrefixFromName(self, name):
        if name.lower().startswith('vk_'):
            return name[3:]
        if name.startswith('vk'):
            return name[2].lower()+name[3:]
        if name.startswith('Vk'):
            return name[2:]
        return name
    #
    def splitModifierTypeAndName(self, param):
        namedecl = ''
        typedecl = ''
        typemodf = param.text or ''
        for elem in param:
            text = elem.text or ''
            tail = elem.tail or ''
            if elem.tag == 'name':
                namedecl += text
                typemodf += tail
            elif elem.tag == 'type':
                typedecl += text
                typemodf += tail
            else:
                typemodf += text + tail
        namedecl = namedecl.strip()
        typedecl = typedecl.strip()
        typemodf = typemodf.strip()
        return typemodf, typedecl, namedecl
    #
    # splitRustTypeAndName  - return the rust-type and the name for an element
    # containing c-types and a name
    def splitRustTypeAndName(self, param, in_function_params=False):
        typemodf, typedecl, namedecl = self.splitModifierTypeAndName(param)
        #
        if namedecl in KEYWORDS:
            namedecl = 'p'+namedecl
        #
        # construct equivalent rust type
        if typemodf=='' and typedecl == 'void':
            rusttype = None
        else:
            if typedecl in TYPE_MAP:
                rusttype = TYPE_MAP[typedecl]
            else:
                rusttype = typedecl
            # convert c-modifiers to rust modifiers
            const=False
            while len(typemodf)>0:
                if typemodf.startswith('const'):
                    const=True
                    typemodf=typemodf[5:].lstrip()
                elif typemodf.startswith('*'):
                    if const:
                        const=False
                        rusttype = '*const %s'%rusttype
                    else:
                        rusttype = '*mut %s'%rusttype
                    typemodf=typemodf[1:].lstrip()
                elif typemodf.startswith('['):
                    lpos = typemodf.index(']')
                    if lpos<2:
                        if const:
                            const=False
                            rusttype = '&[%s]'%rusttype
                        else:
                            rusttype = '&mut [%s]'%rusttype
                    else:
                        arraysize = typemodf[1:lpos]
                        if self.registry.lookupElementInfo(arraysize, self.registry.enumdict) is not None:
                            arraysize = '%s as usize' % arraysize
                        rusttype = '[%s;%s]' % (rusttype, arraysize)
                    typemodf=typemodf[lpos+1:].lstrip()
                elif typemodf.startswith('struct'):
                    # ignore struct
                    typemodf=typemodf[6:].lstrip()
                else:
                    raise Exception("unable to convert c-modifier to rust: "+typemodf)
            if in_function_params and rusttype.startswith('['):
                if const:
                    rusttype = '*const%s'%rusttype
                else:
                    rusttype = '*mut%s'%rusttype
            elif const:
                raise Exception("unable to apply the last const-modifier for %s (%s: %s)" % (param, namedecl, rusttype))
        return namedecl, rusttype
    #
    # makeRustParamDecl - return a string which is an indented, formatted
    # declaration for a <param> or <member> block (e.g. function parameter
    # or structure/union member).
    # param - Element (<param> or <member>) to format
    # aligncol - if non-zero, attempt to align the nested <name> element
    #   at this column
    def makeRustParamDecl(self, param, aligncol, in_function_params=False):
        namedecl, paramdecl = self.splitRustTypeAndName(param, in_function_params=in_function_params)
        return (namedecl+':').ljust(aligncol-1) + ' ' + paramdecl, namedecl, paramdecl
    #
    # getRustParamNameLength - return the length of the type field is an indented, formatted
    # declaration for a <param> or <member> block (e.g. function parameter
    # or structure/union member).
    # param - Element (<param> or <member>) to identify
    def getRustParamNameLength(self, param):
        return len(param.find('name').text)+2
    #
    # makeRustDecls - return Rust prototype and function pointer typedef for a
    #   command, as a two-element list of strings.
    # cmd - Element containing a <command> tag
    def makeRustDecl(self, cmd):
        """Generate Rust function signature for <command> Element"""
        proto = cmd.find('proto')
        params = cmd.findall('param')
        #
        # Insert the function return type/name.
        # Done by walking the tree for <proto> element by element.
        # etree has elem.text followed by (elem[i], elem[i].tail)
        #   for each child element and any following text.
        # The contents of the <name> tag is ignored and prepended later.
        cmdname, rettype = self.splitRustTypeAndName(proto, in_function_params=False)
        # Now add the parameter declaration list. Concatenate all the text from
        # a <param> node without the tags. No tree walking required since all
        # tags are ignored.
        # The contents of the <name> tags are ignored and prepended to the types.
        if len(params) > 0:
            targetLen = 0
            for param in params:
                targetLen = max(targetLen, self.getRustParamNameLength(param))
            argsdecl = '(\n'
            first = True
            for param in params:
                if not first:
                    argsdecl += ',\n'
                argsdecl += '    ' + self.makeRustParamDecl(param, int((targetLen+3)/4)*4, in_function_params=True)[0]
                first = False
            argsdecl += ')'
        else:
            argsdecl = '()'
        cmddecl = 'pub fn ' + cmdname + argsdecl
        if rettype is not None:
            cmddecl += '\n    -> ' + rettype
        return cmddecl
    #
    def enumToRustValue(self, elem, needsNum):
        (numVal,strVal) = self.enumToValue(elem, needsNum)
        strType = None
        if strVal is not None:
            str1 = ''
            str2 = ''
            if strVal.startswith('('):
                strVal = strVal[1:-1]
            if strVal.startswith('~'):
                strVal = strVal[1:]
                str1 = '!'
            pos = strVal.rfind('-')
            if pos>1:
                str2 = strVal[pos:]
                strVal = strVal[:pos]
            lStrVal = strVal.lower()
            if lStrVal.endswith('f') and '0x' not in lStrVal:
                strVal = strVal[:-1]
                strType = 'f32'
            elif lStrVal.endswith('d') and '0x' not in lStrVal:
                strVal = strVal[:-1]
                strType = 'f64'
            elif lStrVal.endswith('ull'):
                strVal = strVal[:-3]
                strType = 'u64'
            elif lStrVal.endswith('ul'):
                strVal = strVal[:-2]
                strType = 'u32'
            elif lStrVal.endswith('u'):
                strVal = strVal[:-1]
                strType = 'u32'
            elif lStrVal.endswith('ll'):
                strVal = strVal[:-2]
                strType = 'i64'
            elif lStrVal.endswith('l'):
                strVal = strVal[:-1]
                strType = 'i32'
            elif lStrVal.startswith('-'):
                strType = 'i32'
            if strType is not None:
                strVal = strVal + strType
            elif strVal.startswith('"'):
                strType = '&\'static str'
            else:
                strType = 'VkEnum'
        return (numVal,str1+strVal+str2,strType)
    #
    def getTypeSize(self, typename, _visited=None):
        if typename in TYPE_SIZE_MAP:
            return TYPE_SIZE_MAP[typename]
        if _visited is None:
            _visited = set()
        elif typename in _visited:
            raise Exception("loop detected with type " + typename)
        _visited.add(typename)
        typeinfo = self.registry.lookupElementInfo(typename, self.registry.typedict)
        typeElem = typeinfo.elem
        category = typeElem.get('category')
        if category == 'struct':
            size = 0
            for member in typeElem.findall('member'):
                size += self.getParamTypeSize(member, _visited)
            return size
        elif category == 'union':
            size = 0
            for member in typeElem.findall('member'):
                size = max(size, self.getParamTypeSize(member, _visited))
            return size
        elif category == 'bitmask' or category == 'enum':
            return 32
        elif category == 'basetype':
            return self.getParamTypeSize(typeElem, _visited)
        else:
            raise Exception("unable to calculate size of type " + typename)
    #
    def getParamTypeSize(self, param, _visited=None):
        typemodf, typedecl, namedecl = self.splitModifierTypeAndName(param)
        size = self.getTypeSize(typedecl, _visited)
        while len(typemodf)>0:
            if typemodf.startswith('['):
                lpos = typemodf.index(']')
                if lpos<2:
                    raise Exception("unable to calculate size with unspecified array length " + typemodf)
                else:
                    size *= int(typemodf[1:lpos])
                typemodf=typemodf[lpos+1:].lstrip()
            else:
                raise Exception("unable to calculate size of type with modifier " + typemodf)
        return size
    #
    # Type generation
    def genType(self, typeinfo, name):
        OutputGenerator.genType(self, typeinfo, name)
        if name in IGNORED:
            return
        typeElem = typeinfo.elem
        # If the type is a struct type, traverse the imbedded <member> tags
        # generating a structure. Otherwise, emit the tag text.
        category = typeElem.get('category')
        if category == 'struct':
            self.genStruct(typeinfo, name)
        elif category == 'union':
            self.genUnion(typeinfo, name)
        elif category == 'funcpointer':
            self.genFuncPointer(typeinfo, name)
        elif category == 'handle':
            self.genHandle(typeinfo, name)
        elif category == 'basetype':
            self.genBasetype(typeinfo, name)
        elif category == 'bitmask':
            self.genBitmask(typeinfo, name)
        elif category == 'define':
            self.genDefine(typeinfo, name)
        elif category is None or category == 'include':
            pass
        else:
            raise Exception("unknown type " + name + " with category " + category)
    #
    # Struct (e.g. C "struct" type) generation.
    # This is a special case of the <type> tag where the contents are
    # interpreted as a set of <member> tags instead of freeform C
    # C type declarations. The <member> tags are just like <param>
    # tags - they are a declaration of a struct or union member.
    # Only simple member declarations are supported (no nested
    # structs etc.)
    def genStruct(self, typeinfo, typeName):
        OutputGenerator.genStruct(self, typeinfo, typeName)
        guard  = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        body = guard
        body += '#[repr(C)]\n'
        body += 'pub struct %s {\n' % typeName
        targetLen = 0
        for member in typeinfo.elem.findall('.//member'):
            targetLen = max(targetLen, self.getRustParamNameLength(member))
        initial_values=[]
        for member in typeinfo.elem.findall('.//member'):
            decl, namedecl, paramdecl = self.makeRustParamDecl(member, int((targetLen+3)/4)*4, in_function_params=False)
            init_value = member.get('values')
            if init_value is not None and not ',' in init_value:
                initial_values.append((namedecl, init_value))
            body += '    pub ' + decl
            body += ',\n'
        body += '}\n\n'
        body += guard
        body += 'impl Default for %s {\n' % typeName
        body += '    fn default () -> %s {\n' % typeName
        if len(initial_values)<=0:
            body += '        return unsafe { ::std::mem::zeroed() };\n'
        else:
            body += '        return unsafe { %s {\n' % typeName
            for name, value in initial_values:
                body += '            %s: %s,\n' % (name, value)
            body += '            ..::std::mem::zeroed()\n'
            body += '        }};\n'
        body += '    }\n'
        body += '}\n'
        self.appendSection('struct', body)
        #
        alias = guard + 'pub use types::%s as %s;'%(typeName,self.stripPrefixFromName(typeName))
        self.appendSection('alias', alias)
    #
    # Struct (e.g. C "struct" type) generation.
    # This is a special case of the <type> tag where the contents are
    # interpreted as a set of <member> tags instead of freeform C
    # C type declarations. The <member> tags are just like <param>
    # tags - they are a declaration of a struct or union member.
    # Only simple member declarations are supported (no nested
    # structs etc.)
    def genUnion(self, typeinfo, typeName):
        OutputGenerator.genStruct(self, typeinfo, typeName)
        size = self.getTypeSize(typeName)
        guard = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        body  = '// union %s\n' % typeName
        body += guard
        body += '#[repr(C)]\n'
        body += 'pub struct %s {\n' % typeName
        body += '    data: [u8;%d]\n' % size
        body += '}\n'
        body += guard
        body += 'impl %s {\n' % typeName
        for member in typeinfo.elem.findall('.//member'):
            namedecl, typedecl = self.splitRustTypeAndName(member, in_function_params=False)
            body += '  #[inline] pub fn as_%s(&self) -> *const %s {\n' % (namedecl, typedecl)
            body += '    unsafe { ::std::mem::transmute(&self.data) }\n'
            body += '  }\n'
            body += '  #[inline] pub fn as_%s_mut(&mut self) -> *mut %s {\n' % (namedecl, typedecl)
            body += '    unsafe { ::std::mem::transmute(&self.data) }\n'
            body += '  }\n'
        body += '}\n\n'
        body += guard
        body += 'impl Default for %s {\n' % typeName
        body += '    fn default () -> %s {\n' % typeName
        body += '        return unsafe { ::std::mem::zeroed() };\n'
        body += '    }\n'
        body += '}\n'
        self.appendSection('struct', body)
        #
        alias = guard + 'pub use types::%s as %s;'%(typeName,self.stripPrefixFromName(typeName))
        self.appendSection('alias', alias)
    #
    # Callback (e.g. Rust function pointer type) generation.
    def genFuncPointer(self, typeinfo, typeName):
        guard = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        body = guard
        body += 'pub type ' + typeName + ' = extern fn ();\n'
        self.appendSection('funcpointer', body)
        #
        alias = guard + 'pub use types::%s as %s;'%(typeName,self.stripPrefixFromName(typeName))
        self.appendSection('alias', alias)
    #
    # Handle (e.g. Rust type definition) generation.
    def genHandle(self, typeinfo, typeName):
        guard = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        typeref = typeinfo.elem.find('type').text
        body = guard
        handleType = TYPE_MAP[typeref]
        body += '#[repr(C)]\n'
        body += '#[derive(Clone,Copy)]\n'
        body += 'pub struct %s (%s);\n'%(typeName, handleType)
        body += 'impl ::util::VkNullHandle for %s {\n' % (typeName)
        body += '    fn null() -> %s { return %s(%s{value:0}); }\n' % (typeName,typeName,handleType)
        body += '}\n'
        body += 'impl Default for %s {\n' % (typeName)
        body += '    fn default() -> %s { return %s(%s{value:0}); }\n' % (typeName,typeName,handleType)
        body += '}\n'
        self.appendSection('handle', body)
        #
        alias = guard + 'pub use types::%s as %s;'%(typeName,self.stripPrefixFromName(typeName))
        self.appendSection('alias', alias)
    #
    # Basetype (e.g. Rust type definition) generation.
    def genBasetype(self, typeinfo, typeName):
        guard = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        typeref = typeinfo.elem.find('type').text
        body = guard
        body += 'pub type ' + typeName + ' = ' + TYPE_MAP[typeref] + ';'
        self.appendSection('basetype', body)
        #
        alias = guard + 'pub use types::%s as %s;'%(typeName,self.stripPrefixFromName(typeName))
        self.appendSection('alias', alias)
    #
    # Bitmask generation
    def genBitmask(self, typeinfo, typeName):
        guard = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        enumflags = typeinfo.elem.get('requires')
        if enumflags is None:
            enumflags = 'VkEnum'
        body = guard
        body += 'pub use self::' + enumflags + ' as ' + typeName + ';'
        self.appendSection('bitmask', body)
        #
        alias = guard + 'pub use types::%s as %s;'%(typeName,self.stripPrefixFromName(typeName))
        self.appendSection('alias', alias)
    #
    # Group (e.g. C "enum" type) generation.
    # These are concatenated together with other types.
    def genGroup(self, groupinfo, groupName):
        OutputGenerator.genGroup(self, groupinfo, groupName)
        if groupName in IGNORED:
            return
        groupElem = groupinfo.elem

        expandName = re.sub(r'([0-9a-z_])([A-Z0-9][^A-Z0-9]?)',r'\1_\2',groupName).upper()

        expandPrefix = expandName
        expandSuffix = ''
        expandSuffixMatch = re.search(r'[A-Z][A-Z]+$',groupName)
        if expandSuffixMatch:
            expandSuffix = '_' + expandSuffixMatch.group()
            # Strip off the suffix from the prefix
            expandPrefix = expandName.rsplit(expandSuffix, 1)[0]

        # Prefix
        body = '\npub type ' + groupName + ' = VkEnum;\n'
        alias = '\npub use types::%s as %s;\n'%(groupName,self.stripPrefixFromName(groupName))
        prefix = 'pub const '
        # @@ Should use the type="bitmask" attribute instead
        isEnum = ('FLAG_BITS' not in expandPrefix)

        # Loop over the nested 'enum' tags. Keep track of the minimum and
        # maximum numeric values, if they can be determined; but only for
        # core API enumerants, not extension enumerants. This is inferred
        # by looking for 'extends' attributes.
        minName = None
        for elem in groupElem.findall('enum'):
            # find the coresponding feature
            featureInfo = None
            featureName = None
            guard = ''
            if 'extname' in elem.attrib:
                featureName = elem.attrib['extname']
                featureInfo = self.registry.lookupElementInfo(featureName, self.registry.apidict)
                if featureInfo is None:
                    featureInfo = self.registry.lookupElementInfo(featureName, self.registry.extdict)
                if 'protect' in featureInfo.elem.attrib:
                    guard= '#[cfg(feature = "%s")]\n' % featureInfo.elem.attrib['protect']
            #
            # Convert the value to an integer and use that to track min/max.
            # Values of form -(number) are accepted but nothing more complex.
            # Should catch exceptions here for more complex constructs. Not yet.
            (numVal,strVal,strType) = self.enumToRustValue(elem, True)
            name = elem.get('name')
            if name in IGNORED:
                continue
            if strVal.startswith('-'):
                strVal = '(%s) as u32' % strVal
            # Extension enumerants are only included if they are required
            if self.isEnumRequired(elem):
                body += guard + prefix + name + ' : ' + groupName + ' = ' + strVal + ";\n"
                alias += guard + 'pub use types::%s as %s;\n'%(name, self.stripPrefixFromName(name))


            if (isEnum and elem.get('extends') is None):
                if (minName == None):
                    minName = maxName = name
                    minValue = maxValue = numVal
                elif (numVal < minValue):
                    minName = name
                    minValue = numVal
                elif (numVal > maxValue):
                    maxName = name
                    maxValue = numVal
        # Generate min/max value tokens and a range-padding enum. Need some
        # additional padding to generate correct names...
        if isEnum:
            body += prefix + expandPrefix + "_BEGIN_RANGE" + expandSuffix + ' : ' + groupName + ' = ' + minName + ';\n'
            body += prefix + expandPrefix + "_END_RANGE" + expandSuffix + ' : ' + groupName + ' = ' + maxName + ';\n'
            body += prefix + expandPrefix + "_RANGE_SIZE" + expandSuffix + ' : ' + groupName + ' = (' + maxName + ' as i32 - ' + minName + ' as i32 + 1i32) as u32;\n'
            alias += 'pub use types::%s_BEGIN_RANGE%s as %s_BEGIN_RANGE%s;\n'%(expandPrefix,expandSuffix,self.stripPrefixFromName(expandPrefix),expandSuffix)
            alias += 'pub use types::%s_END_RANGE%s as %s_END_RANGE%s;\n'%(expandPrefix,expandSuffix,self.stripPrefixFromName(expandPrefix),expandSuffix)
            alias += 'pub use types::%s_RANGE_SIZE%s as %s_RANGE_SIZE%s;\n'%(expandPrefix,expandSuffix,self.stripPrefixFromName(expandPrefix),expandSuffix)

        body += prefix + expandPrefix + "_MAX_ENUM" + expandSuffix + ' : ' + groupName + ' = 0x7FFFFFFF;\n'

        if groupElem.get('type') == 'bitmask':
            section = 'bitmask'
        else:
            section = 'group'
        self.appendSection(section, body)
        #
        self.appendSection('alias', alias)
    #
    # Enumerant generation
    # <enum> tags may specify their values in several ways, but are usually
    # just integers.
    def genEnum(self, enuminfo, name):
        OutputGenerator.genEnum(self, enuminfo, name)
        if name in IGNORED:
            return
        guard = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        (numVal,strVal,strType) = self.enumToRustValue(enuminfo.elem, False)
        body = guard
        body += 'pub const ' + name.ljust(33) + ': ' + strType.ljust(6) + ' = ' + strVal + ';'
        self.appendSection('enum', body)
    #
    def genDefine(self, typeinfo, typeName):
        guard = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        body = guard
        nameElem = typeinfo.elem.find('name')
        if nameElem is None:
            return
        typeElem = typeinfo.elem.find('type')
        if typeElem is not None:
            if typeElem.text == 'VK_MAKE_VERSION':
                body += 'pub const %s : VkVersionInfo = vk_make_version!%s;'%(typeName, typeElem.tail.strip())
            else:
                return
        else:
            try:
                value = int(nameElem.tail)
            except:
                return
            body += 'pub const %s : u32 = %d;'%(typeName, value)
        self.appendSection('define', body)
        #
        alias = guard + 'pub use types::%s as %s;'%(typeName,self.stripPrefixFromName(typeName))
        self.appendSection('alias', alias)
    #
    # Command generation
    def genCmd(self, cmdinfo, name):
        if name in IGNORED:
            return
        guard = ''
        if self.featureExtraProtect is not None:
            guard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        if len(self.sections['command'])==0:
            body = guard + '#[link(name = "vulkan")]\nextern "C" {\n'
            self.appendSection('command', body)
        OutputGenerator.genCmd(self, cmdinfo, name)
        #
        body = self.makeRustDecl(cmdinfo.elem)
        self.appendSection('command', body + ';\n')
        #
        alias = guard + 'pub use ffi::%s as %s;'%(name,self.stripPrefixFromName(name))
        self.appendSection('alias', alias)
