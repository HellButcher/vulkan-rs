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

# pylint: disable=I0011,C0103,C0301,C0326,R0201,W0613

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
    'VK_DEFINE_NON_DISPATCHABLE_HANDLE': 'VkNonDispatchableHandle',
}

TYPE_PROPERTY_MAP = {
    'float':    (4, True),
    'double':   (8, True),
    'uint8_t':  (1, True),
    'uint16_t': (2, True),
    'uint32_t': (4, True),
    'uint64_t': (8, True),
    'int8_t':   (1, True),
    'int16_t':  (2, True),
    'int32_t':  (4, True),
    'int64_t':  (8, True),

    'void':         (None, False),
    'char':         (None, True),
    'int':          (None, True),
    'unsigned int': (None, True),
    'size_t':       (None, True),
    'VK_DEFINE_HANDLE': (None, True),
    'VK_DEFINE_NON_DISPATCHABLE_HANDLE': (None, True),
}

KEYWORDS = set(['as', 'break', 'const', 'continue', 'crate', 'else', 'enum', \
    'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop', 'match', \
    'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'static', 'struct', \
    'type', 'trait', 'true', 'unsafe', 'use', 'where', 'while'])

IGNORED = set(['VK_NULL_HANDLE'])

RE_LINK = re.compile(r'\b[a-z](?:link|name):([^\s]+)')
RE_CODE = re.compile(r'\bcode:([^\s]+)')

# RustGeneratorOptions - subclass of GeneratorOptions.
#
# Adds options used by RustOutputGenerator objects during Rust language source
# generation.
#
# Additional members
#   prefixText - list of strings to prefix generated header with
#     (usually a copyright statement + calling convention macros).
class RustGeneratorOptions(GeneratorOptions):
    def __init__(self,
                 prefixText = "",
                 *args, **kwargs):
        GeneratorOptions.__init__(self, *args, **kwargs)
        self.prefixText      = prefixText
#
# RustBaseOutputGenerator - subclass of OutputGenerator.
# Basic class for Rust OutputGenerators
class RustBaseOutputGenerator(OutputGenerator):
    BASE_SECTIONS = ['include', 'define', 'basetype', 'handle', 'enum',  'group', 'bitmask', 'funcpointer', 'struct', 'command']
    # sub-classes of this generator should adjust this variable
    ALL_SECTIONS = BASE_SECTIONS
    #
    def __init__(self, *args, **kwargs):
        OutputGenerator.__init__(self, *args, **kwargs)
        # Internal state - accumulators for different inner block text
        self.sections = dict([(section, []) for section in self.ALL_SECTIONS])
        self.featureGuard = ''
    #
    def openFile(self, filename):
        if filename != None:
            filename = self.genOpts.directory + '/' + filename
            return io.open(filename, 'w', encoding='utf-8')
        else:
            return None
    #
    def beginFile(self, genOpts):
        OutputGenerator.beginFile(self, genOpts)
        #
        # User-supplied prefix text, if any (list of strings)
        if hasattr(genOpts, 'prefixText') and genOpts.prefixText:
            for s in genOpts.prefixText:
                write(s, file=self.outFile)
    #
    def endFile(self):
        OutputGenerator.endFile(self)
    #
    def writeSections(self, sections, outFile):
        if outFile is None:
            return
        for section in sections:
            contents = self.sections[section]
            if contents:
                write('', file=outFile)
                write('// feature %s' % self.featureName, file=outFile)
                write('// --------%s' % ('-'*len(self.featureName)), file=outFile)
                write('', file=outFile)
                break
        for section in sections:
            contents = self.sections[section]
            if contents:
                write('\n'.join(contents), file=outFile)
                write('', file=outFile)
    #
    def beginFeature(self, interface, emit):
        # Start processing in superclass
        OutputGenerator.beginFeature(self, interface, emit)
        self.sections = dict([(section, []) for section in self.ALL_SECTIONS])
        if self.featureExtraProtect is not None:
            self.featureGuard = '#[cfg(feature = "%s")]\n' % self.featureExtraProtect
        else:
            self.featureGuard = ''
    #
    def endFeature(self):
        OutputGenerator.endFeature(self)
    #
    def appendSection(self, section, text):
        # self.sections[section].append('SECTION: ' + section + '\n')
        self.sections[section].append(text)
    #
    def stripPrefixFromName(self, name):
        if name.lower().startswith('vk_'):
            return name[3:]
        if len(name) > 2 and name[2].isupper():
            if name.startswith('vk'):
                return name[2].lower()+name[3:]
            if name.startswith('Vk'):
                return name[2:]
        return name
    #
    def getCName(self, elem):
        if elem is None:
            return None
        n = elem.get('name')
        if n is not None:
            return n
        n = elem.find('name')
        if n is not None and n.text is not None:
            return n.text
        return None
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
    def splitRustTypeAndName(self, param, in_function_params=False):
        typemodf, typedecl, namedecl = self.splitModifierTypeAndName(param)
        #
        if namedecl in KEYWORDS:
            namedecl = 'p'+namedecl
        #
        # construct equivalent rust type
        if typemodf == '' and typedecl == 'void':
            rusttype = None
        else:
            if typedecl in TYPE_MAP:
                rusttype = TYPE_MAP[typedecl]
            else:
                rusttype = typedecl
            # convert c-modifiers to rust modifiers
            const = False
            while len(typemodf) > 0:
                if typemodf.startswith('const'):
                    const = True
                    typemodf = typemodf[5:].lstrip()
                elif typemodf.startswith('*'):
                    if const:
                        const = False
                        rusttype = '*const %s'%rusttype
                    else:
                        rusttype = '*mut %s'%rusttype
                    typemodf = typemodf[1:].lstrip()
                elif typemodf.startswith('['):
                    lpos = typemodf.index(']')
                    if lpos < 2:
                        if const:
                            const = False
                            rusttype = '&[%s]'%rusttype
                        else:
                            rusttype = '&mut [%s]'%rusttype
                    else:
                        arraysize = typemodf[1:lpos]
                        if self.registry.lookupElementInfo(arraysize, self.registry.enumdict) is not None:
                            arraysize = '%s as usize' % arraysize
                        rusttype = '[%s;%s]' % (rusttype, arraysize)
                    typemodf = typemodf[lpos+1:].lstrip()
                elif typemodf.startswith('struct'):
                    # ignore struct
                    typemodf = typemodf[6:].lstrip()
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
    # get the numerical value, teh string representation and the type of an enum
    def enumToRustValue(self, elem, needsNum):
        (numVal, strVal) = self.enumToValue(elem, needsNum)
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
            if pos > 1:
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
    # get properties of the given type
    def getTypeProperties(self, typename, _visited=None):
        if typename in TYPE_PROPERTY_MAP:
            return TYPE_PROPERTY_MAP[typename]
        if _visited is None:
            _visited = dict()
        elif typename in _visited:
            result = _visited[typename]
            if result is None:
                raise Exception("loop detected with type " + typename)
            else:
                return result
        _visited[typename] = None
        typeinfo = self.registry.lookupElementInfo(typename, self.registry.typedict)
        typeElem = typeinfo.elem
        category = typeElem.get('category')
        size = 0
        cloneable = True
        if category == 'struct':
            for member in typeElem.findall('member'):
                (member_size, member_cloneable) = self.getParamTypeProperties(member, _visited)
                if member_size is None:
                    size = None
                elif size is not None:
                    size += member_size
                cloneable = cloneable and member_cloneable
        elif category == 'union':
            cloneable = False
            for member in typeElem.findall('member'):
                (member_size, _) = self.getParamTypeProperties(member, _visited)
                if member_size is None:
                    size = None
                elif size is not None:
                    size = max(size, member_size)
        elif category == 'bitmask' or category == 'enum':
            size = 32
            cloneable = True
        elif category == 'funcpointer' or category == 'include' or category is None:
            size = None
            cloneable = False
        elif category == 'basetype' or category == 'handle':
            elem = typeElem.find('type')
            if elem is None or elem.text is None:
                raise Exception("unable to calculate size of type " + typename)
            (size, cloneable) = self.getTypeProperties(elem.text, _visited)
        else:
            raise Exception("unable to calculate size of type " + typename)
        result = (size, cloneable)
        _visited[typename] = result
        return result
    #
    # get properties of the type of the given parameter or field
    def getParamTypeProperties(self, param, _visited=None):
        typemodf, typedecl, _ = self.splitModifierTypeAndName(param)
        (size, cloneable) = self.getTypeProperties(typedecl, _visited)
        while len(typemodf)>0:
            if typemodf.startswith('*'):
                cloneable = False
                size = None
                typemodf=typemodf[1:].lstrip()
            elif typemodf.startswith('const*'):
                cloneable = False
                size = None
                typemodf=typemodf[6:].lstrip()
            elif typemodf.startswith('const '):
                typemodf=typemodf[6:].lstrip()
            elif typemodf.startswith('struct '):
                cloneable = False
                size = None
                typemodf=typemodf[7:].lstrip()
            elif typemodf.startswith('['):
                cloneable = False
                lpos = typemodf.index(']')
                if lpos<2:
                    size = None
                elif size is not None:
                    try:
                        size *= int(typemodf[1:lpos])
                    except:
                        size = None
                typemodf=typemodf[lpos+1:].lstrip()
            else:
                raise Exception("unable to calculate size of type with modifier " + typemodf)
        return size, cloneable
    #
    def replaceDocTags(self, doc):
        doc = RE_LINK.sub(r'`\1`', doc)
        doc = RE_CODE.sub(r'`\1`', doc)
        return doc
    #
    def getDocumentation(self, name, elem=None, indent=''):
        if name is not None and hasattr(self.registry, 'doc_descriptions') and name in self.registry.doc_descriptions:
            d = self.registry.doc_descriptions[name]
            if d:
                return '%s/// %s\n' % (indent, self.replaceDocTags(d))
        if elem is not None:
            d = elem.get('comment')
            if d is not None:
                return '%s/// %s\n' % (indent, self.replaceDocTags(d))
        return ''
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
    # generate types with category "struct".
    def genStruct(self, typeinfo, typeName):
        self.validateFeature('struct', typeName)
    #
    # generate types with category "union".
    def genUnion(self, typeinfo, typeName):
        self.validateFeature('union', typeName)
    #
    # generate types with category "funcpointer".
    def genFuncPointer(self, typeinfo, typeName):
        self.validateFeature('funcpointer', typeName)
    #
    # generate types with category "handle".
    def genHandle(self, typeinfo, typeName):
        self.validateFeature('handle', typeName)
    #
    # generate types with category "basetype".
    def genBasetype(self, typeinfo, typeName):
        self.validateFeature('basetype', typeName)
    #
    # generate types with category "bitmask".
    def genBitmask(self, typeinfo, typeName):
        self.validateFeature('bitmask', typeName)
    #
    # generate types with category "define".
    def genDefine(self, typeinfo, name):
        self.validateFeature('define', name)
        if name in IGNORED:
            return
        nameElem = typeinfo.elem.find('name')
        if nameElem is None:
            return
        typeElem = typeinfo.elem.find('type')
        if typeElem is not None:
            if typeElem.text == 'VK_MAKE_VERSION':
                self.genDefineConstant(typeinfo, name, 'VkVersionInfo', 'vk_make_version!%s'%typeElem.tail.strip())
        else:
            try:
                value = int(nameElem.tail)
            except:
                return  # do not generate this define
            self.genDefineConstant(typeinfo, name, 'u32', '%d'%value)
    #
    # generate constants for types with category "define".
    def genDefineConstant(self, typeinfo, constName, constType, constValue):
        pass
    #
    # generate enum groups
    def genGroup(self, groupinfo, groupName):
        OutputGenerator.genGroup(self, groupinfo, groupName)
        if groupName in IGNORED:
            return
        #
        expandName = re.sub(r'([0-9a-z_])([A-Z0-9][^A-Z0-9]?)', r'\1_\2', groupName).upper()
        expandPrefix = expandName
        expandSuffix = ''
        expandSuffixMatch = re.search(r'[A-Z][A-Z]+$', groupName)
        if expandSuffixMatch:
            # Strip off the suffix (like KHR) from the prefix
            expandSuffix = '_' + expandSuffixMatch.group()
            expandPrefix = expandName.rsplit(expandSuffix, 1)[0]
        #
        isEnum = ('FLAG_BITS' not in expandPrefix)
        #
        # Loop over the nested 'enum' tags. Keep track of the minimum and
        # maximum numeric values, if they can be determined; but only for
        # core API enumerants, not extension enumerants. This is inferred
        # by looking for 'extends' attributes.
        minName = None
        entries = []
        for elem in groupinfo.elem.findall('enum'):
            # find the coresponding feature
            featureInfo = None
            featureGuard = ''
            if 'extname' in elem.attrib:
                featureName = elem.attrib['extname']
                featureInfo = self.registry.lookupElementInfo(featureName, self.registry.apidict)
                if featureInfo is None:
                    featureInfo = self.registry.lookupElementInfo(featureName, self.registry.extdict)
                if 'protect' in featureInfo.elem.attrib:
                    featureExtraProtect = featureInfo.elem.attrib['protect']
                    featureGuard = '#[cfg(feature = "%s")]\n' % featureExtraProtect
            #
            # Convert the value to an integer and use that to track min/max.
            # Values of form -(number) are accepted but nothing more complex.
            # Should catch exceptions here for more complex constructs. Not yet.
            (numVal, strVal, strType) = self.enumToRustValue(elem, True)
            name = elem.get('name')
            if name in IGNORED:
                continue
            if strVal.startswith('-'):
                strVal = '(%s) as u32' % strVal
            # Extension enumerants are only included if they are required
            if self.isEnumRequired(elem):
                entries.append((name, numVal, strVal, featureGuard, elem))
            #
            if isEnum and elem.get('extends') is None:
                if minName == None:
                    minName = maxName = name
                    minValue = maxValue = numVal
                elif numVal < minValue:
                    minName = name
                    minValue = numVal
                elif numVal > maxValue:
                    maxName = name
                    maxValue = numVal
        # Generate min/max value tokens and a range-padding enum. Need some
        # additional padding to generate correct names...
        if isEnum:
            entries.append((expandPrefix+"_BEGIN_RANGE"+expandSuffix, minValue, minName, '', None))
            entries.append((expandPrefix+"_END_RANGE"+expandSuffix, maxValue, maxName, '', None))
            rangeValue = minValue is not None and maxValue is not None and maxValue - minValue  + 1 or None
            rangeStr = '(' + maxName + ' as i32 - ' + minName + ' as i32 + 1i32) as u32'
            entries.append((expandPrefix+"_RANGE_SIZE"+expandSuffix, rangeValue, rangeStr, '', None))
        entries.append((expandPrefix+"_MAX_ENUM"+expandSuffix, 0x7FFFFFFF, '0x7FFFFFFF', '', None))
        #
        self.genGroupEnums(groupinfo, groupName, isEnum, entries)
    #
    # generate the enums of a group
    def genGroupEnums(self, groupinfo, groupName, isEnum, entries):
        pass
#
# RustOutputGenerator - subclass of RustBaseOutputGenerator.
# Generates Rust-language API interfaces.
class RustTypesOutputGenerator(RustBaseOutputGenerator):
    #
    def endFeature(self):
        if self.emit:
            self.writeSections(self.BASE_SECTIONS, self.outFile)
        # Finish processing in superclass
        RustBaseOutputGenerator.endFeature(self)
    #
    def genStruct(self, typeinfo, typeName):
        RustBaseOutputGenerator.genStruct(self, typeinfo, typeName)
        size, cloneable = self.getTypeProperties(typeName)
        body = ''
        body += self.getDocumentation(typeName, typeinfo.elem)
        body += self.featureGuard
        if cloneable:
            body += '#[derive(Copy,Clone)]\n'
        body += '#[repr(C)]\n'
        body += 'pub struct %s {\n' % typeName
        initial_values=[]
        for member in typeinfo.elem.findall('.//member'):
            namedecl, paramdecl = self.splitRustTypeAndName(member, in_function_params=False)
            init_value = member.get('values')
            if init_value is not None and not ',' in init_value:
                initial_values.append((namedecl, init_value))
            body += self.getDocumentation('%s::%s' % (typeName, self.getCName(member)), member, indent='    ')
            body += '    pub %s: %s,\n' % (namedecl, paramdecl)
        body += '}\n\n'
        body += self.featureGuard
        body += 'impl Default for %s {\n' % typeName
        body += '    fn default () -> %s {\n' % typeName
        if len(initial_values)<=0:
            body += '        return unsafe { ::std::mem::zeroed() };\n'
        else:
            body += '        return unsafe {\n'
            body += '            %s {\n' % typeName
            for name, value in initial_values:
                body += '                %s: %s,\n' % (name, value)
            body += '                ..::std::mem::zeroed()\n'
            body += '            }\n'
            body += '        };\n'
        body += '    }\n'
        body += '}\n'
        self.appendSection('struct', body)
    #
    def genUnion(self, typeinfo, typeName):
        RustBaseOutputGenerator.genUnion(self, typeinfo, typeName)
        size, cloneable = self.getTypeProperties(typeName)
        if size is None:
            raise Exception("Unable to determine size of union " + typeName)
        body  = ''
        body += self.getDocumentation(typeName, typeinfo.elem)
        body += self.featureGuard
        if cloneable:
            body += '#[derive(Copy,Clone)]\n'
        body += '#[repr(C)]\n'
        body += 'pub struct %s {\n' % typeName
        body += '    data: [u8;%d]\n' % size
        body += '}\n'
        body += self.featureGuard
        body += 'impl %s {\n' % typeName
        for member in typeinfo.elem.findall('.//member'):
            namedecl, typedecl = self.splitRustTypeAndName(member, in_function_params=False)
            body += self.getDocumentation('%s::%s' % (typeName, self.getCName(member)), member, indent='    ')
            body += '  #[inline] pub fn as_%s(&self) -> & %s {\n' % (namedecl, typedecl)
            body += '    unsafe { ::std::mem::transmute(&self.data) }\n'
            body += '  }\n'
            body += self.getDocumentation('%s::%s' % (typeName, self.getCName(member)), member, indent='    ')
            body += '  #[inline] pub fn as_%s_mut(&mut self) -> &mut %s {\n' % (namedecl, typedecl)
            body += '    unsafe { ::std::mem::transmute(&mut self.data) }\n'
            body += '  }\n'
        body += '}\n\n'
        body += self.featureGuard
        body += 'impl Default for %s {\n' % typeName
        body += '    fn default () -> %s {\n' % typeName
        body += '        return unsafe { ::std::mem::zeroed() };\n'
        body += '    }\n'
        body += '}\n'
        self.appendSection('struct', body)
    #
    def genFuncPointer(self, typeinfo, typeName):
        RustBaseOutputGenerator.genFuncPointer(self, typeinfo, typeName)
        body  = ''
        body += self.getDocumentation(typeName, typeinfo.elem)
        body += self.featureGuard
        body += 'pub type ' + typeName + ' = extern fn ();\n'
        self.appendSection('funcpointer', body)
    #
    def genHandle(self, typeinfo, typeName):
        RustBaseOutputGenerator.genHandle(self, typeinfo, typeName)
        typeref = typeinfo.elem.find('type').text
        handleType = TYPE_MAP[typeref]
        body  = ''
        body += self.getDocumentation(typeName, typeinfo.elem)
        body += self.featureGuard
        body += '#[repr(C)]\n'
        body += '#[derive(Clone,Copy,Debug,PartialEq,Eq)]\n'
        body += 'pub struct %s (%s);\n'%(typeName, handleType)
        body += 'impl ::util::VkNullHandle for %s {\n' % (typeName)
        body += '    fn null() -> %s {\n' % typeName
        body += '        return %s(%s{value:0});' % (typeName,handleType)
        body += '    }\n'
        body += '}\n'
        body += 'impl Default for %s {\n' % (typeName)
        body += '    fn default() -> %s {\n' % typeName
        body += '        return %s(%s{value:0});\n' % (typeName,handleType)
        body += '    }\n'
        body += '}\n'
        self.appendSection('handle', body)
    #
    def genBasetype(self, typeinfo, typeName):
        RustBaseOutputGenerator.genBasetype(self, typeinfo, typeName)
        typeref = typeinfo.elem.find('type').text
        body  = ''
        body += self.getDocumentation(typeName, typeinfo.elem)
        body += self.featureGuard
        body += 'pub type ' + typeName + ' = ' + TYPE_MAP[typeref] + ';'
        self.appendSection('basetype', body)
    #
    def genBitmask(self, typeinfo, typeName):
        RustBaseOutputGenerator.genBitmask(self, typeinfo, typeName)
        enumflags = typeinfo.elem.get('requires')
        if enumflags is None:
            enumflags = 'VkEnum'
        body = ''
        body += self.getDocumentation(typeName, typeinfo.elem)
        body += self.featureGuard
        body += 'pub use self::' + enumflags + ' as ' + typeName + ';'
        self.appendSection('bitmask', body)
    #
    def genDefineConstant(self, typeinfo, constName, constType, constValue):
        RustBaseOutputGenerator.genDefineConstant(self, typeinfo, constName, constType, constValue)
        body = ''
        body += self.getDocumentation(constName, typeinfo.elem)
        body += self.featureGuard
        body += 'pub const %s : %s = %s;' % (constName, constType, constValue)
        self.appendSection('define', body)
    #
    def genGroupEnums(self, groupinfo, groupName, isEnum, entries):
        RustBaseOutputGenerator.genGroupEnums(self, groupinfo, groupName, isEnum, entries)
        #
        if groupinfo.elem.get('type') == 'bitmask':
            section = 'bitmask'
        else:
            section = 'group'
        #
        body = '\n'
        body += self.getDocumentation(groupName, groupinfo.elem)
        body += 'pub type ' + groupName + ' = VkEnum;'
        self.appendSection(section, body)
        #
        for name, _, strVal, featureGuard, elem in entries:
            body = ''
            body += self.getDocumentation('%s::%s' % (groupName, name), elem)
            body += featureGuard
            body += 'pub const %s : %s = %s;' % (name, groupName, strVal)
            self.appendSection(section, body)
    #
    def genEnum(self, enuminfo, name):
        RustBaseOutputGenerator.genEnum(self, enuminfo, name)
        if name in IGNORED:
            return
        (_, strVal, strType) = self.enumToRustValue(enuminfo.elem, False)
        #
        body = ''
        body += self.getDocumentation(name, enuminfo.elem)
        body += self.featureGuard
        body += 'pub const %s: %s = %s;' % (name, strType, strVal)
        self.appendSection('enum', body)
#
# RustFfiOutputGenerator - subclass of RustBaseOutputGenerator.
# Generates the Rust Foreign Function Interface.
class RustFfiOutputGenerator(RustBaseOutputGenerator):
    #
    BASE_SECTIONS = [ 'command' ]
    ALL_SECTIONS = BASE_SECTIONS
    #
    def endFeature(self):
        if len(self.sections['command'])>0:
            self.appendSection('command', '}\n')
        if self.emit:
            self.writeSections(self.BASE_SECTIONS, self.outFile)
        # Finish processing in superclass
        RustBaseOutputGenerator.endFeature(self)
    #
    def genCmd(self, cmdinfo, name):
        RustBaseOutputGenerator.genCmd(self, cmdinfo, name)
        if name in IGNORED:
            return
        if len(self.sections['command']) == 0:
            body  = self.featureGuard
            body += '#[link(name = "vulkan")]\n'
            body += 'extern "C" {\n'
            self.appendSection('command', body)
        RustBaseOutputGenerator.genCmd(self, cmdinfo, name)
        #
        proto = cmdinfo.elem.find('proto')
        params = cmdinfo.elem.findall('param')
        #
        # find the command name and the return type
        cmdname, rettype = self.splitRustTypeAndName(proto, in_function_params=False)
        #
        body = ''
        body += self.getDocumentation(name, cmdinfo.elem)
        body += 'pub fn %s(' % cmdname
        #
        # now build the argument list
        prefix = ' '*len(body)
        first = True
        for param in params:
            if not first:
                body += ',\n'
                body += prefix
            paramname, paramtype = self.splitRustTypeAndName(param, in_function_params=True)
            body += '%s: %s' % (paramname, paramtype)
            first = False
        body += ')'
        #
        # append the return type
        if rettype is not None:
            if len(params) > 0:
                body += '\n'
                body += prefix
            else:
                body += ' '
            body += '-> ' + rettype
        #
        self.appendSection('command', body + ';\n')

#
# RustSafeOutputGenerator - subclass of RustBaseOutputGenerator.
# Generates the safe Rust Function Interface.
class RustSafeOutputGenerator(RustBaseOutputGenerator):
    #
    BASE_SECTIONS = [ 'command' ]
    ALL_SECTIONS = BASE_SECTIONS
    #
    def endFeature(self):
        if self.emit:
            self.writeSections(self.BASE_SECTIONS, self.outFile)
        # Finish processing in superclass
        RustBaseOutputGenerator.endFeature(self)
    #
    def genCmd(self, cmdinfo, name):
        RustBaseOutputGenerator.genCmd(self, cmdinfo, name)
        if name in IGNORED:
            return
        #
        proto = cmdinfo.elem.find('proto')
        params = cmdinfo.elem.findall('param')
        #
        # find the command name and the return type
        cmdname, ffirettype = self.splitRustTypeAndName(proto, in_function_params=False)
        #
        # build a information structure for arguments
        numparams = len(params)
        paramlist = []
        param_by_name = dict()
        length_arg_for_input = dict()
        length_arg_for_result = dict()
        for i, param in enumerate(params):
            paramname, paramtype = self.splitRustTypeAndName(param, in_function_params=True)
            paramlen = param.get('len')
            resulttype = None
            #
            if (ffirettype is None or ffirettype == 'VkResult') and i == numparams-1 and paramtype.startswith('*mut '):
                resulttype = paramtype[5:]
                if paramlen is not None:
                    resulttype = 'Vec<%s>' % resulttype
                    if paramlen in length_arg_for_result:
                        length_arg_for_result[paramlen].append(i)
                    else:
                        length_arg_for_result[paramlen] = [i]
            elif paramlen is not None:
                if paramlen in length_arg_for_input:
                    length_arg_for_input[paramlen].append(i)
                else:
                    length_arg_for_input[paramlen] = [i]
            param_by_name[paramname] = i
            paramlist.append((param, paramname, paramtype, resulttype))
        #
        arglist = [] # the actual argument list
        prepare = [] # task to do before calling the function
        handover = [] # pass argument to the function
        vectoradjust = None # (lengthVariable, reserveStatement) when returning a vector, and a second invocation is required
        complete = [] # things to do after calling the function
        returns = None
        for param, paramname, paramtype, resulttype in paramlist:
            paramlen = param.get('len')
            if paramlen is not None:
                paramlen = paramlen.replace('::','.')
            paramoptional = param.get('optional') == 'true'
            if paramname in length_arg_for_input:
                if paramtype.startswith('*'):
                    raise Exception("inconsistency: don't know how to handle param %s in %s"(paramname, cmdname))
                sliceargs = length_arg_for_input[paramname]
                _, sliceargname, _, _ = paramlist[sliceargs[0]]
                prepare.append('let %s = %s.len();' % (paramname, sliceargname))
                for moreslicearg in sliceargs[1:]:
                    _, sliceargname, _, _ = paramlist[moreslicearg]
                    prepare.append('assert!(%s == %s.len());' % (paramname, sliceargname))
                handover.append('%s as %s' % (paramname, paramtype))
            elif paramname in length_arg_for_result:
                if paramtype.startswith('*mut '):
                    prepare.append('let mut %s : %s = 0;' % (paramname, paramtype[5:]))
                    handover.append('&mut %s' % paramname)
                elif not paramtype.startswith('*'):
                    arglist.append((paramname, paramtype))
                    handover.append(paramname)
                else:
                    raise Exception("inconsistency: don't know how to handle param %s in %s"(paramname, cmdname))
            elif resulttype is not None:
                returns = (paramname, resulttype)
                if paramlen is None:
                    if resulttype.startswith('*'):
                        prepare.append('let mut %s : %s = util::vk_null();' % (paramname, resulttype))
                    else:
                        prepare.append('let mut %s : %s = Default::default();' % (paramname, resulttype))
                    handover.append('&mut %s' % paramname)
                elif paramlen == 'null-terminated' or ',' in paramlen:
                    raise Exception("unable to return param " + paramname + " in command " + cmdname)
                elif '.' in paramlen:
                    prepare.append('let mut %s : %s = Vec::with_capacity(%s as usize);' % (paramname, resulttype, paramlen))
                    handover.append('%s.as_mut_ptr()' % paramname)
                    complete.append('unsafe { %s.set_len(%s as usize) };' % (paramname, paramlen))
                elif paramlen in param_by_name:
                    _, _, lenparamtype, _ = paramlist[param_by_name[paramlen]]
                    if lenparamtype.startswith('*mut '):
                        prepare.append('let mut %s : %s = Vec::new();' % (paramname, resulttype))
                        vectoradjust = (paramlen, '%s.reserve_exact(%s as usize);' % (paramname, paramlen))
                        handover.append('%s.as_mut_ptr()' % paramname)
                        complete.append('unsafe { %s.set_len(%s as usize) };' % (paramname, paramlen))
                    else:
                        prepare.append('let mut %s : %s = Vec::with_capacity(%s as usize);' % (paramname, resulttype, paramlen))
                        handover.append('%s.as_mut_ptr()' % paramname)
                        complete.append('unsafe { %s.set_len(%s as usize) };' % (paramname, paramlen))
                else:
                    raise Exception("unable to return param " + paramname + " in command " + cmdname)
            elif paramtype == '*const '+TYPE_MAP['char'] and paramlen == 'null-terminated':
                if paramoptional:
                    arglist.append((paramname, 'Option<&str>'))
                    prepare.append('let %s = %s.map(|s| CString::new(s).unwrap());' % (paramname, paramname))
                    prepare.append('let %s = %s.map_or(util::vk_null() as %s, |s| s.as_ptr());' % (paramname, paramname, paramtype))
                    handover.append(paramname)
                else:
                    arglist.append((paramname, '&str'))
                    prepare.append('let %s = CString::new(%s).unwrap();' % (paramname, paramname))
                    handover.append('%s.as_ptr()' % paramname)
            elif paramtype.startswith('*') and paramlen is None:
                safetype = paramtype.replace('*mut ', '&mut ').replace('*const ', '&')
                if paramoptional:
                    arglist.append((paramname, 'Option<%s>' % safetype))
                    prepare.append('let %s = %s.map_or(util::vk_null() as %s, |s| s as %s);' % (paramname, paramname, paramtype, paramtype))
                    handover.append(paramname)
                else:
                    arglist.append((paramname, safetype))
                    handover.append(paramname)
            elif paramtype.startswith('*'):
                if paramtype.startswith('*mut '):
                    safetype = paramtype[5:]
                    argcastptr = '*mut '
                elif paramtype.startswith('*const '):
                    safetype = paramtype[7:]
                    argcastptr = '*const '
                else:
                    raise Exception("unexpected state")
                argcast = ''
                if safetype.endswith('c_void'):
                    argcast = ' as %s' % (argcastptr+safetype)
                    safetype = 'u8'
                safetype = '&[%s]' % safetype
                if '.' in paramlen:
                    if paramoptional:
                        arglist.append((paramname, 'Option<%s>' % safetype))
                        handover.append('%s.map_or(util::vk_null(), |s| s.as_ptr()%s)' % (paramname,argcast))
                        prepare.append('assert!(%s.map_or(0 as usize, |s| s.len()) == %s as usize);' % (paramname, paramlen))
                    else:
                        arglist.append((paramname, safetype))
                        handover.append('%s.as_ptr()%s' % (paramname,argcast))
                        prepare.append('assert!(%s.len() == %s as usize);' % (paramname, paramlen))
                elif paramlen in param_by_name:
                    _, _, lenparamtype, _ = paramlist[param_by_name[paramlen]]
                    if paramoptional:
                        arglist.append((paramname, 'Option<%s>' % safetype))
                        handover.append('%s.map_or(util::vk_null(), |s| s.as_ptr()%s)' % (paramname,argcast))
                    else:
                        arglist.append((paramname, safetype))
                        handover.append('%s.as_ptr()%s' % (paramname,argcast))
                else:
                    raise Exception("unable to return param " + paramname + " in command " + cmdname)
            else:
                arglist.append((paramname, paramtype))
                handover.append(paramname)
        #
        successcodes = cmdinfo.elem.get('successcodes')
        if successcodes is not None:
            successcodes = successcodes.split(",")
        else:
            successcodes = []
        #
        # maybe a loop for handling 'VK_INCOMPLETE' is required
        repeat_VK_INCOMPLETE = vectoradjust is not None and ffirettype == 'VkResult' and 'VK_INCOMPLETE' in successcodes
        if repeat_VK_INCOMPLETE:
            successcodes.remove('VK_INCOMPLETE')
        #
        # handle return type and return values
        if returns is not None:
            (returnvalue, returntype) = returns
            if ffirettype is not None:
                if ffirettype == 'VkResult' and len(successcodes) > 1:
                    returns = ('Ok((%s, _result))' % returnvalue, 'util::VkResultObj<(%s, VkResult)>' % returntype)
                elif ffirettype == 'VkResult':
                    returns = ('Ok(%s)' % returnvalue, 'util::VkResultObj<%s>' % returntype)
                else:
                    returns = ('(%s, _result)' % returnvalue, '(%s, %s)' % (returntype, ffirettype))
        elif ffirettype is not None:
            if ffirettype == 'VkResult':
                returns = ('Ok(_result)', 'util::VkResultObj')
            else:
                returns = ('_result', ffirettype)
        #
        #
        #
        #now build the functions
        functionHeader = 'pub fn %s(' % cmdname
        body  = ''
        body += self.getDocumentation(name, cmdinfo.elem)
        body += self.featureGuard
        if not repeat_VK_INCOMPLETE:
            body += '#[inline]'
        body += functionHeader
        prefix = ' '*len(functionHeader)
        #
        # now build the argument list
        first = True
        for argname, argtype in arglist:
            if not first:
                body += ',\n'
                body += prefix
            body += '%s: %s' % (argname, argtype)
            first = False
        body += ')'
        #
        # append the return type
        if returns is not None:
            (_, returntype) = returns
            if len(arglist) > 0:
                body += '\n'
                body += prefix + '-> ' + returntype
            else:
                body += ' -> ' + returntype
        body += '\n'
        #
        body += '{\n'
        for statement in prepare:
            body += '    %s\n' % statement
        #
        if repeat_VK_INCOMPLETE:
            body += '    loop {\n'
            indent = '    '
        else:
            indent = ''
        #
        handover1 = vectoradjust is None and handover or handover[:-1]+['util::vk_null()']
        fncall = 'unsafe { ffi::%s(%s) }' % (cmdname, ', '.join(handover1))
        if ffirettype is not None:
            if vectoradjust is not None:
                body += indent+'    let mut _result = %s;\n' % fncall
            else:
                body += indent+'    let _result = %s;\n' % fncall
            if ffirettype == 'VkResult':
                body += indent+'    if (_result as i32) < 0 { return Err(_result.into()); }\n'
        else:
            body += indent
            body += '    %s;\n' % fncall
        #
        if vectoradjust is not None:
            (vectorcount, reserveStatement) = vectoradjust
            body += indent+'    if %s > 0 {\n' % vectorcount
            body += indent+'        %s\n' % reserveStatement
            fncall = 'unsafe { ffi::%s(%s) };' % (cmdname, ', '.join(handover))
            if ffirettype is not None:
                body += indent+'        _result = %s;\n' % fncall
                if ffirettype == 'VkResult':
                    body += indent+'        if (_result as i32) < 0 { return Err(_result.into()); }\n'
            else:
                body += indent+'        %s;\n' % fncall
            body += indent+'    }\n'
        #
        if repeat_VK_INCOMPLETE:
            body += indent+'    if _result == VK_INCOMPLETE { continue; }\n'
        #
        for statement in complete:
            body += indent+'    %s\n' % statement
        #
        if returns is not None:
            (returnvalue, _) = returns
            body += indent+'    return %s;\n' % returnvalue
        #
        if repeat_VK_INCOMPLETE:
            body += '    }\n'
        #
        body += '}\n'
        self.appendSection('command', body)
#
# RustAliasOutputGenerator - subclass of RustBaseOutputGenerator.
# generates aliases for types, constants and functions
# the aliase-names are the original names without the 'Vk' prefix
class RustAliasOutputGenerator(RustBaseOutputGenerator):
    #
    def endFeature(self):
        if self.emit:
            self.writeSections(self.BASE_SECTIONS, self.outFile)
        # Finish processing in superclass
        RustBaseOutputGenerator.endFeature(self)
    #
    def genStruct(self, typeinfo, typeName):
        RustBaseOutputGenerator.genStruct(self, typeinfo, typeName)
        alias  = self.featureGuard
        alias += 'pub use types::%s as %s;'%(typeName, self.stripPrefixFromName(typeName))
        self.appendSection('struct', alias)
    #
    def genUnion(self, typeinfo, typeName):
        RustBaseOutputGenerator.genUnion(self, typeinfo, typeName)
        alias  = self.featureGuard
        alias += 'pub use types::%s as %s;'%(typeName, self.stripPrefixFromName(typeName))
        self.appendSection('struct', alias)
    #
    def genFuncPointer(self, typeinfo, typeName):
        RustBaseOutputGenerator.genFuncPointer(self, typeinfo, typeName)
        alias  = self.featureGuard
        alias += 'pub use types::%s as %s;'%(typeName, self.stripPrefixFromName(typeName))
        self.appendSection('funcpointer', alias)
    #
    def genHandle(self, typeinfo, typeName):
        RustBaseOutputGenerator.genHandle(self, typeinfo, typeName)
        alias  = self.featureGuard
        alias += 'pub use types::%s as %s;'%(typeName, self.stripPrefixFromName(typeName))
        self.appendSection('handle', alias)
    #
    def genBasetype(self, typeinfo, typeName):
        RustBaseOutputGenerator.genBasetype(self, typeinfo, typeName)
        alias  = self.featureGuard
        alias += 'pub use types::%s as %s;'%(typeName, self.stripPrefixFromName(typeName))
        self.appendSection('basetype', alias)
    #
    def genBitmask(self, typeinfo, typeName):
        RustBaseOutputGenerator.genBitmask(self, typeinfo, typeName)
        alias  = self.featureGuard
        alias += 'pub use types::%s as %s;'%(typeName, self.stripPrefixFromName(typeName))
        self.appendSection('bitmask', alias)
    #
    def genDefineConstant(self, typeinfo, constName, constType, constValue):
        RustBaseOutputGenerator.genDefineConstant(self, typeinfo, constName, constType, constValue)
        alias  = self.featureGuard
        alias += 'pub use types::%s as %s;'%(constName, self.stripPrefixFromName(constName))
        self.appendSection('define', alias)
    #
    def genGroupEnums(self, groupinfo, groupName, isEnum, entries):
        RustBaseOutputGenerator.genGroupEnums(self, groupinfo, groupName, isEnum, entries)
        #
        if groupinfo.elem.get('type') == 'bitmask':
            section = 'bitmask'
        else:
            section = 'group'
        #
        alias = '\npub use types::%s as %s;'%(groupName, self.stripPrefixFromName(groupName))
        self.appendSection(section, alias)
        #
        for name, numVal, strVal, featureGuard, elem in entries:
            alias  = featureGuard
            alias += 'pub use types::%s as %s;\n'%(name, self.stripPrefixFromName(name))
            self.appendSection(section, alias)
    #
    def genEnum(self, enuminfo, name):
        RustBaseOutputGenerator.genEnum(self, enuminfo, name)
        if name in IGNORED:
            return
        alias  = self.featureGuard
        alias += 'pub use self::types::%s as %s;'%(name, self.stripPrefixFromName(name))
        self.appendSection('enum', alias)
    #
    def genCmd(self, cmdinfo, name):
        RustBaseOutputGenerator.genCmd(self, cmdinfo, name)
        if name in IGNORED:
            return
        alias  = self.featureGuard
        alias += 'pub use self::cmds::%s as %s;' % (name, self.stripPrefixFromName(name))
        self.appendSection('command', alias)
#
# RustUtilsGeneratorOptions - subclass of RustGeneratorOptions.
#
# Adds options used by RustUtilsOutputGenerator
#
# Additional members
#   generateGetName - list of group names to generate
#      `get_${GROUPNAME}_name(value: ${GROUPNAME}) -> &'static str`
#   generateGetDescription - list of group names to generate
#      `get_${GROUPNAME}_description(value: ${GROUPNAME}) -> &'static str`
class RustUtilsGeneratorOptions(RustGeneratorOptions):
    def __init__(self,
            generateGetName = [],
            generateGetDescription = [],
            generateGetFromStr = [],
            *args, **kwargs):
        RustGeneratorOptions.__init__(self, *args, **kwargs)
        self.generateGetName = set(generateGetName)
        self.generateGetDescription = set(generateGetDescription)
        self.generateGetFromStr = set(generateGetFromStr)
#
# RustUtilsOutputGenerator - subclass of RustBaseOutputGenerator.
# generates utility functions for converting Enum-Group Values to strings.
class RustUtilsOutputGenerator(RustBaseOutputGenerator):
    #
    ALL_SECTIONS = [ 'group' ]
    #
    def endFeature(self):
        if self.emit:
            self.writeSections(self.ALL_SECTIONS, self.outFile)
        # Finish processing in superclass
        RustBaseOutputGenerator.endFeature(self)
    #
    def genGroupEnums(self, groupinfo, groupName, isEnum, entries):
        RustBaseOutputGenerator.genGroupEnums(self, groupinfo, groupName, isEnum, entries)
        #
        if groupName in self.genOpts.generateGetName:
            sorted_entries = sorted(filter(lambda item: item[4] is not None, entries), key=(lambda item: item[1]))
            body  = '/// get the name of a given %s.\n' % groupName
            body += 'pub fn get_%s_name(v: ::types::%s) -> &\'static str {\n' % (groupName, groupName)
            body += '    let v = v as i32;\n'
            body += '    static LOOKUP : [(i32, &\'static str); %d] = [\n' % len(sorted_entries)
            for name, numVal, strVal, featureGuard, elem in sorted_entries:
                body += '        (%d, "%s"),\n' % (numVal, name)
            body += '    ];\n'
            body += '    return match LOOKUP.binary_search_by_key(&v, |&(a,_)| a) {\n'
            body += '        Ok(i) => LOOKUP[i].1,\n'
            body += '        Err(_) => "",\n'
            body += '    };\n'
            body += '}\n'
            self.appendSection('group', body)
        #
        if groupName in self.genOpts.generateGetDescription:
            sorted_entries = sorted(filter(lambda item: item[4] is not None, entries), key=(lambda item: item[1]))
            body  = '/// get the description of a given %s. If no descriptionis known, the name is returned.\n' % groupName
            body += 'pub fn get_%s_description(v: ::types::%s) -> &\'static str {\n' % (groupName, groupName)
            body += '    let v = v as i32;\n'
            body += '    static LOOKUP : [(i32, &\'static str); %d] = [\n' % len(sorted_entries)
            for name, numVal, strVal, featureGuard, elem in sorted_entries:
                body += '        (%d, "%s"),\n' % (numVal, elem.get('comment') or name)
            body += '    ];\n'
            body += '    return match LOOKUP.binary_search_by_key(&v, |&(a,_)| a) {\n'
            body += '        Ok(i) => LOOKUP[i].1,\n'
            body += '        Err(_) => "",\n'
            body += '    };\n'
            body += '}\n'
            self.appendSection('group', body)
        #
        if groupName in self.genOpts.generateGetFromStr:
            sorted_entries = sorted(filter(lambda item: item[4] is not None, entries), key=(lambda item: item[0]))
            body  = '/// get the value of a %s from the name.\n' % groupName
            body += 'pub fn get_%s_from_str(s: &str) -> Option<::types::%s> {\n' % (groupName, groupName)
            body += '    static LOOKUP : [(&\'static str, ::types::%s); %d] = [\n' % (groupName, len(sorted_entries))
            for name, numVal, strVal, featureGuard, elem in sorted_entries:
                body += '        ("%s", %s),\n' % (name, strVal)
            body += '    ];\n'
            body += '    return match LOOKUP.binary_search_by(|&(a,_)| a.cmp(s)) {\n'
            body += '        Ok(i) => Some(LOOKUP[i].1),\n'
            body += '        Err(_) => None,\n'
            body += '    };\n'
            body += '}\n'
            self.appendSection('group', body)


#
