import xml.etree.ElementTree as etree
import collections
from enum import Enum
from .utils import camel_to_screaming_snake_case

_NAME_='name'
_TYPE_='type'
_ENUM_='enum'
_COMMENT_='comment'

class Named:
    def __init__(self, name):
        self.name = name
        if name is None:
            raise ValueError("name must not be None", self)

    def __repr__(self):
        repr = super(Named, self).__repr__()
        return '%s@%s' % (repr, self.name)

    def to_tuple(self):
        return (self.name, self)

class NamedObjectDict(collections.OrderedDict):
    __notexists_marker = object()
    def __contains__(self, obj):
        if hasattr(obj, _NAME_):
            key = getattr(obj, _NAME_)
            value = self.get(key, NamedObjectDict.__notexists_marker)
            return value is not NamedObjectDict.__notexists_marker and value == obj
        else:
            return super(NamedObjectDict, self).__contains__(obj)

    def __iter__(self):
        return self.iter()

    def iter(self):
        return iter(self.values())

    def add(self, obj):
        key = obj.name
        self[key] = obj

    def append(self, obj):
        key = obj.name
        if key in self:
            del self[key]
        self[key] = obj

    def addunique(self, obj):
        key = obj.name
        if key in self:
            raise KeyError("an item with the same name is already in thid directory", key)
        self[key] = obj
    appendunique = addunique
    
    def remove(self, obj):
        key = obj.name
        del self[key]

    def first(self):
        return next(iter(self.values()))

    def last(self):
        return next(reversed(self.values()))

class TagScanner:
    _end_of_stream = object()
    def __init__(self, elem):
        if isinstance(elem, str):
            self.text = elem
            self.elems_iter = iter([])
        else:
            self.text = elem.text
            self.elems_iter = iter(elem)
        self.current = None
        self.read = 0

        self.strip()

    def strip(self):
        if self.text is None:
            return
        i = 0
        while i < len(self.text) and self.text[i].isspace():
            i += 1
        self.text = self.text[i:]
        self.read += i

    def symbol(self, keyword):
        if self.text is None or len(self.text) == 0:
            return False
        l = len(keyword)
        if not self.text.startswith(keyword):
            return False
        if len(self.text) == l or not keyword[0].isalpha() or not self.text[l].isalnum():
            self.text = self.text[l:]
            self.read += l
            self.strip()
            return True
        return False

    def identifier(self):
        if self.text is None or len(self.text) == 0 or not (self.text[0] == '_' or self.text[0].isalpha()):
            return None
        i = 1
        while i < len(self.text) and (self.text[i] == '_' or self.text[i].isalnum()):
            i += 1
        value = self.text[:i]
        self.text = self.text[i:]
        self.read += i
        self.strip()
        return value

    def peek_tag(self):
        if self.text is not None and len(self.text) > 0:
            return None
        if self.current is None:
            self.current = next(self.elems_iter, TagScanner._end_of_stream)
        if self.current is TagScanner._end_of_stream:
            return None
        return self.current

    def tag(self, tag):
        current = self.peek_tag()
        if current is None or current.tag != tag:
            return None
        self.current = None
        self.text = current.tail
        self.strip()
        return current

    def end(self):
        return (self.text is None or len(self.text) == 0) and self.peek_tag() is None

def _parse_typedef(s, **kwargs):
    if not isinstance(s, TagScanner):
        s = TagScanner(s)
    if not s.symbol('typedef'):
        raise ValueError('expected `typedef`, got %s' % repr(s.text))
    res = _parse_specifier(s, **kwargs)
    if not s.symbol(';'):
        raise ValueError('expected `;`, got %s (%s)' % (repr(s.text), repr(res)))
    return res

def _expect_one(s):
    if len(s) != 1:
        raise ValueError('expected only one item')
    return s[0]

def _parse_specifiers(s, multiple=False):
    ty2, name = _parse_specifier(s)
    if not multiple:
        return ty2, name
    def all(ty2, name):
        yield ty2, name
        while s.symbol(','):
            ty2, name = _parse_specifier(s)
            yield ty2, name
    return all(ty2, name)

def _parse_specifier(s, multiple=False):
    if not isinstance(s, TagScanner):
        s = TagScanner(s)
    is_const = s.symbol('const')
    s.symbol('struct') # opt
    typename = s.tag('type')
    if typename is not None:
        typename = typename.text
    elif s.symbol('void'): # for funcptrs
        typename = 'void'
    elif s.symbol('VkBool32'): # for funcptrs
        typename = 'VkBool32'
    else:
        raise ValueError('expected type tag, got %s' % repr(s.text))
    
    ty = NamedTypeRef(typename)
    if is_const:
        ty = ConstTypeRef(ty)
    return _parse_declarators(s, ty, multiple=multiple)


def _parse_declarators(s, ty, multiple=False):
    ty2, name = _parse_declarator(s, ty)
    if not multiple:
        return ty2, name
    def all(ty2, name):
        yield ty2, name
        while s.symbol(','):
            ty2, name = _parse_declarator(s, ty)
            yield ty2, name
    return all(ty2, name)


def _parse_declarator(s, ty):
    s.symbol('VKAPI_PTR') #opt
    while s.symbol('*'):
        ty = PointerTypeRef(ty)
        if s.symbol('const'):
            ty = ConstTypeRef(ty)
    if s.symbol('('):
        res_ty, name = _parse_declarator(s, ty)
        if res_ty == ty:
            ins_ty = _WrapperTypeRef(ty)
        else:
            ins_ty = res_ty._find(ty)
        if not s.symbol(')'):
            raise ValueError('expected closing parentesis `)`, got %s (ty=%s, name=%s)' % (repr(s.text), repr(res_ty), name))
    else:
        ins_ty = _WrapperTypeRef(ty)
        res_ty = ins_ty

        name = s.tag('name')
        if name is not None:
            name = name.text
        else:
            name = s.identifier() #opt

    while True:
        if s.symbol('['): #array
            enum = s.tag('enum')
            if enum is not None:
                dim = enum.text
                if not s.symbol(']'):
                    raise ValueError('expected closing bracket `]`, got %s (ty=%s, name=%s)' % (repr(s.text), repr(res_ty), name))
            else:
                i = s.text.index(']')
                dim = s.text[:i] or None
                s.text = s.text[i+1:]
                s.read += i + 1
                s.strip()
            ins_ty.arg = ArrayTypeRef(ins_ty.arg, dim)
        elif s.symbol('(void)'): #function
            ins_ty.arg = FunctionTypeRef(ins_ty.arg, [])
        elif s.symbol('('): #function
            ins_ty.arg = FunctionTypeRef(ins_ty.arg, [BaseParameter(par_ty, par_name) for par_ty, par_name in _parse_specifiers(s, multiple=True)])
            if not s.symbol(')'):
                raise ValueError('expected closing parentesis `)`, got %s (ty=%s, name=%s)' % (repr(s.text), repr(res_ty), name))
        else:
            break
    if isinstance(res_ty, _WrapperTypeRef):
        res_ty = res_ty.arg
    return res_ty, name

class _TypeRefMeta(type):
    _CLASSES_ = []

    def __new__(cls, clsname, superclasses, attributedict):
        ty = type.__new__(cls, clsname, superclasses, attributedict)
        if not clsname.endswith('TypeRef'):
            return ty
        name = clsname[:-7]
        if len(name) == 0:
            return ty
        _TypeRefMeta._CLASSES_.append(ty)
        setattr(TypeRef, name.upper(), ty)
        return ty

class TypeRef(metaclass=_TypeRefMeta):
    def _resolve(self, registry):
        arg = getattr(self, 'arg', None)
        if arg is not None:
            arg._resolve(registry)
    def __eq__(self, other):
        if not isinstance(other, TypeRef):
            return False
        if self.__class__ != other.__class__:
            return False
        arg = getattr(self, 'arg', None)
        if arg is not None and arg != other.arg:
            return False
        return True
    def __ne__(self, other):
        return not self.__eq__(other)
    def __str__(self):
        return self.c_decl()
    def __repr__(self):
        return '%s(%s)' % (self.__class__.__name__, self.c_decl())
    def c_decl(self, decl=''):
       raise NotImplementedError()
    def _find(self, ty):
        arg = getattr(self, 'arg', None)
        if arg is ty:
            return self
        elif arg is not None:
            return arg._find(ty)
        return None
    def is_const(self):
        arg = getattr(self, 'arg', None)
        if arg is not None and arg.__class__ == TypeRef.CONST:
            return True
        return False
    def is_ptr(self):
        return False
class NamedTypeRef(TypeRef):
    _ORD_=0
    def __init__(self, name):
        self.name = name
    def _resolve(self, registry):
        self.resolved_type = registry.types[self.name]
    def __eq__(self, other):
        if isinstance(other, str):
            return self.name == other
        if not TypeRef.__eq__(self, other):
            return False
        return self.name == other.name
    def c_decl(self, decl=''):
        if decl is None or len(decl) == 0:
            return self.name
        else:
            return '%s %s' % (self.name, decl)
class ConstTypeRef(TypeRef):
    def __init__(self, arg):
        self.arg = arg
        self._ORD_ = arg._ORD_
    def c_decl(self, decl=''):
        if self._ORD_ == 0:
            return 'const %s' % self.arg.c_decl(decl)
        elif decl is None or len(decl) == 0:
            return self.arg.c_decl('const')
        else:
            return self.arg.c_decl('const %s' % decl)
    def is_const(self):
        return True
class PointerTypeRef(TypeRef):
    _ORD_=10
    def __init__(self, arg):
        self.arg = arg
    def c_decl(self, decl=''):
        decl = '*%s' % decl
        if self._ORD_ < self.arg._ORD_:
            decl = '(%s)'%decl
        return self.arg.c_decl(decl)
    def is_ptr(self):
        return True
class ArrayTypeRef(TypeRef):
    _ORD_=20
    def __init__(self, arg, dim=None):
        self.arg = arg
        self.dim = dim
    def c_decl(self, decl=''):
        dim = '[]'
        if self.dim is not None:
            dim = '[%s]' % self.dim
        if self._ORD_ < self.arg._ORD_:
            decl = '(%s)'%decl
        return self.arg.c_decl('%s%s' % (decl, dim))
    def _resolve(self, registry):
        super(ArrayTypeRef,self)._resolve(registry)
        self.enum = None
        if self.dim is not None and len(self.dim) > 0 and self.dim[0].isalpha():
            self.enum = registry.enum_items[self.dim]
    def __eq__(self, other):
        if not TypeRef.__eq__(self, other):
            return False
        return self.dim == other.dim
    def is_ptr(self):
        return self.dim is None
class FunctionTypeRef(TypeRef):
    _ORD_=20
    def __init__(self, arg, params=None):
        self.arg = arg
        self.params = self.members = params
    def c_decl(self, decl=''):
        if self._ORD_ < self.arg._ORD_:
            decl = '(%s)'%decl
        return self.returns.c_decl('%s(%s)' % (decl, ', '.join([p.type.c_decl(p.name) for p in self.params])))
    def __getattr__(self, name):
        if name == 'returns':
            return self.arg
        else:
            raise KeyError
class _WrapperTypeRef(TypeRef):
    def __init__(self, arg):
        self.arg = arg

TypeRef.VOID = NamedTypeRef("void")
TypeRef.CHAR = NamedTypeRef("char")
TypeRef.RESULT = NamedTypeRef("VkResult")
TypeRef.BOOL = NamedTypeRef("VkBool32")
TypeRef.VOID_PTR = PointerTypeRef(ConstTypeRef(TypeRef.VOID))
TypeRef.STRING = PointerTypeRef(ConstTypeRef(TypeRef.CHAR))

class Typed:
    def __init__(self, ty):
        self.type = ty


    def _resolve(self, registry):
        if self.type is not None:
            self.type._resolve(registry)

class Composed:
    def __init__(self, member_elems):

        self.members = NamedObjectDict()
        for member_elem in member_elems:
            self.members.addunique(Member(self, member_elem))

    def _resolve(self, registry):
        for member in self.members:
            member._resolve(registry)

class BaseElem:
    def __init__(self, elem):
        self.elem = elem

class Tag(BaseElem,Named):
    def __init__(self, registry, elem):
        self.registry = registry
        BaseElem.__init__(self, elem)
        Named.__init__(self, elem.get(_NAME_))

class _BaseTypeElemMeta(type):
    _CLASSES_ = []
    _CATEGORY_TO_CLASS_ = {}

    def __new__(cls, clsname, superclasses, attributedict):
        ty = type.__new__(cls, clsname, superclasses, attributedict)
        if '_CATEGORY_' not in attributedict:
            return ty
        name = clsname
        if name.endswith('Type'):
            name = name[:-4]
        cat = attributedict['_CATEGORY_']
        _BaseTypeElemMeta._CLASSES_.append(ty)
        existing = _BaseTypeElemMeta._CATEGORY_TO_CLASS_.get(cat, None)
        if existing is not None:
            raise KeyError('category %s is already assigned to class %s' % (cat.value, existing))
        if cat is not None:
            setattr(BaseTypeElem, cat.upper(), ty)
        setattr(BaseTypeElem, name.upper(), ty)
        _BaseTypeElemMeta._CATEGORY_TO_CLASS_[cat] = ty
        return ty

class BaseTypeElem(BaseElem, Named, metaclass=_BaseTypeElemMeta):
    def __init__(self, registry, name, elem):
        self.registry = registry
        BaseElem.__init__(self, elem)
        Named.__init__(self, name)
        self.returnedonly = (elem.get('returnedonly') == 'true')
        self.requires = _split_attrib(elem.get('requires'))
        self.requiering_feature = None
        self.docs = None

    @staticmethod
    def new_from_category(cat, *args, **kwargs):
        return _BaseTypeElemMeta._CATEGORY_TO_CLASS_[cat](*args, **kwargs)


class Include(BaseTypeElem):
    _CATEGORY_ = 'include'
    def __init__(self, registry, elem):
        name = elem.get(_NAME_)
        if name is None:
            name = elem.find(_NAME_).text
        BaseTypeElem.__init__(self, registry, name, elem)

class Define(BaseTypeElem):
    _CATEGORY_ = 'define'
    def __init__(self, registry, elem):
        name = elem.get(_NAME_)
        value = None
        if name is None:
            name_elem = elem.find(_NAME_)
            name = name_elem.text
            value = name_elem.tail
            if value and value.startswith('('):
                value = None # this is a macro
        BaseTypeElem.__init__(self, registry, name, elem)
        self.is_deprecated = elem.text and elem.text.find('DEPRECATED')>=0
        type_elem = elem.find(_TYPE_)
        self.macro_call = None
        if type_elem is not None:
            self.macro_call = type_elem.text
            value = type_elem.tail
        if value:
            comment_pos = value.find('//')
            if comment_pos >= 0:
                value = value[:comment_pos]
            value = value.strip()
        self.value = value or None
        

class ProvidedType(BaseTypeElem):
    _CATEGORY_ = None
    def __init__(self, registry, elem):
        BaseTypeElem.__init__(self, registry, elem.get(_NAME_), elem)

class TypedefType(BaseTypeElem,Typed):
    _CATEGORY_ = 'basetype'
    def __init__(self, registry, elem):
        ty, name = _parse_typedef(elem)
        BaseTypeElem.__init__(self, registry, name, elem)
        Typed.__init__(self, ty)

class FlagsType(BaseTypeElem):
    _CATEGORY_ = 'bitmask'
    def __init__(self, registry, elem):
        BaseTypeElem.__init__(self, registry, elem.find(_NAME_).text, elem)

class HandleType(BaseTypeElem):
    _CATEGORY_ = 'handle'
    def __init__(self, registry, elem):
        BaseTypeElem.__init__(self, registry, elem.find(_NAME_).text, elem)
        self.non_dispatchable = elem.find(_TYPE_).text == 'VK_DEFINE_NON_DISPATCHABLE_HANDLE'
        if not self.non_dispatchable:
            if self.name in ['VkInstance', 'VkPhysicalDevice']:
                self.dispatch_table = DispatchTable.Instance
            else:
                self.dispatch_table = DispatchTable.Device

class EnumType(BaseTypeElem):
    _CATEGORY_ = 'enum'
    def __init__(self, registry, elem):
        BaseTypeElem.__init__(self, registry, elem.get(_NAME_), elem)
        self.group = None

class FunctionPointerType(BaseTypeElem,Typed):
    _CATEGORY_ = 'funcpointer'
    def __init__(self, registry, elem):
        ty, name = _parse_typedef(elem)
        BaseTypeElem.__init__(self, registry, name, elem)
        Typed.__init__(self, ty)
        if not isinstance(ty, PointerTypeRef) or not isinstance(ty.arg, FunctionTypeRef):
            raise ValueError('expected function pointer')
        self.returns = ty.arg.returns
        self.params = ty.arg.params

class StructType(BaseTypeElem,Composed):
    _CATEGORY_ = 'struct'
    def __init__(self, registry, elem):
        BaseTypeElem.__init__(self, registry, elem.get(_NAME_), elem)
        Composed.__init__(self, elem.findall('member'))
        self.structextends = _split_attrib(elem.get('structextends'))

class UnionType(BaseTypeElem,Composed):
    _CATEGORY_ = 'union'
    def __init__(self, registry, elem):
        BaseTypeElem.__init__(self, registry, elem.get(_NAME_), elem)
        Composed.__init__(self, elem.findall('member'))

class BaseParameter(Named,Typed):
    def __init__(self, ty, name):
        Named.__init__(self, name)
        Typed.__init__(self, ty)

class Member(BaseElem,BaseParameter):
    def __init__(self, container, elem):
        if hasattr(container, 'registry'):
            self.registry = container.registry
        ty, name = _parse_specifier(elem)
        BaseElem.__init__(self, elem)
        Named.__init__(self, name)
        Typed.__init__(self, ty)
        self.is_out = False
        self.len_for = None
        self.container = container
        length = elem.get('altlen', elem.get('len'))
        if length:
            self.len = length.split(',')[0]
        else:
            self.len = None
        self.values = elem.get('values')
        optional = elem.get('optional')
        if optional == 'true':
            self.optional = True
        elif optional == 'false':
            self.optional = False
        else:
            self.optional = None
        self.docs = None

    def _resolve(self, registry):
        super(Member, self)._resolve(registry)
        self.is_out = self.type.__class__ == TypeRef.POINTER and not self.type.is_const()
        if self.len and self.len != 'null-terminated':
            if self.len in self.container.members:
                m = self.container.members[self.len]
                if not m.is_out:
                    self.is_out = False
                if m.len_for is None:
                    m.len_for = []
                m.len_for.append(self)
            #else:
            #    self.is_out = False

class EnumGroup(BaseElem,Named):
    def __init__(self, registry, elem):
        self.registry = registry
        BaseElem.__init__(self, elem)
        Named.__init__(self, elem.get(_NAME_))
        self.type = elem.get(_TYPE_)
        self.requiering_feature = None
        self.docs = None

        self.enum_items = NamedObjectDict()
        for enum_item_elem in elem.findall('enum'):
            self.enum_items.addunique(EnumItem(self, enum_item_elem))

        self.enum_type = None
        if self.type is not None:
            self.enum_type = registry.types[self.name]
            if self.enum_type.group is not None and self.enum_type.group is not self:
                raise ValueError('enum-group / enum-type mismatch', self, self.enum_type, self.enum_type.group)
            self.enum_type.group = self

class EnumItem(BaseElem,Named):

    _EXT_BASE_ = 1000000000
    _EXT_BLOCK_SIZE_ = 1000

    def __init__(self, enum_group, elem, extnumber=None):
        if hasattr(enum_group, 'registry'):
            self.registry = enum_group.registry
        BaseElem.__init__(self, elem)
        Named.__init__(self, elem.get(_NAME_))
        self.value = elem.get('value')
        self.bitpos = elem.get('bitpos')
        self.enum_group = enum_group
        self.requiering_feature = None
        self.docs = None
        if self.bitpos is not None:
            self.bitpos = int(self.bitpos)
        offset = elem.get('offset')
        if offset is not None:
            offset = int(offset)
        if self.value is None and offset is not None:
            v = EnumItem._EXT_BASE_ + (extnumber - 1) * EnumItem._EXT_BLOCK_SIZE_ + offset
            if elem.get('dir') == '-':
                v = -v;
            self.value = str(v)
        if self.value is None and self.bitpos is not None:
            self.value = '0x%08x' % (1<<self.bitpos)

class Command(BaseElem,Named,Composed):
    def __init__(self, registry, elem):
        self.registry = registry
        proto = Member(self, elem.find('proto'))
        BaseElem.__init__(self, elem)
        Named.__init__(self, proto.name)
        Composed.__init__(self, elem.findall('param'))
        self.proto = proto #alias
        self.returns = proto.type #alias
        self.params = self.members #alias
        self.requiering_feature = None
        self.docs = None
        self.out_param = None
        self.successcodes = set(_split_attrib(elem.get('successcodes')))
        self.errorcodes = set(_split_attrib(elem.get('errorcodes')))

    def _resolve(self, registry):
        super(Command, self)._resolve(registry)
        if self.proto is not None:
            self.proto._resolve(registry)
        first_param = self.params.first()
        self.dispatch_table = DispatchTable.Loader
        if first_param is not None and isinstance(first_param.type, NamedTypeRef):
            resolved_type = first_param.type.resolved_type
            if isinstance(resolved_type, HandleType):
                self.dispatch_table = resolved_type.dispatch_table
                if not self.name.endswith('ProcAddr'):
                    # vkDestroy*
                    first_param.optional = False
        last_param = self.params.last()
        if last_param is not None and last_param.is_out and (self.returns == TypeRef.VOID or self.returns == TypeRef.RESULT):
            self.out_param = last_param

class DispatchTable(Enum):
    Loader = 'loader'
    Instance = 'instance'
    Device = 'device'

class FeatureGroup(BaseElem,Named):
    def __init__(self, registry, elem):
        self.registry = registry
        BaseElem.__init__(self, elem)
        Named.__init__(self, elem.get(_NAME_))
        self.is_extension = elem.tag == 'extension'
        self.api = elem.get('api')
        self.supported = elem.get('supported') or self.api
        if self.is_extension:
            self.number = int(elem.get('number'))
        else:
            self.number = float(elem.get('number'))
        self.type = elem.get('type')
        self.protect = elem.get('protect')

        self.requires = _split_attrib(elem.get('requires'))
        self.requires_recursive = set()
        
        self.required_types = []
        self.required_commands = []
        self.required_enum_items = []

        self.removed_types = []
        self.removed_commands = []
        self.removed_enum_items = []

        self.types = []
        self.commands = []
        self.enum_items = []

        self.docs = None

    def _get_or_create_enum_group(self, extends):
        if extends is None:
            if self.name in self.registry.enum_groups:
                group = self.registry.enum_groups[self.name]
                if group.type is not None:
                    raise ValueError("already defined a group", self.name)
            else:
                elem = etree.SubElement(self.elem, 'enums', attrib={'name': self.name})
                group = EnumGroup(self.registry, elem)
                self.registry.enum_groups.addunique(group)
            return group
        else:
            return self.registry.enum_groups[extends]

    def _create_enum_item(self, elem):
        extends = elem.get('extends')
        group = self._get_or_create_enum_group(extends)
        item = EnumItem(group, elem, extnumber=self.number)
        group.enum_items.addunique(item)
        self.registry.enum_items.addunique(item)
        return item

    def require(self, elem):
        name = elem.get(_NAME_)
        if name is None:
            raise ValueError('require without name', elem)
        if elem.tag == 'type':
            self.required_types.append(self.registry.types[name])
        elif elem.tag == 'command':
            self.required_commands.append(self.registry.commands[name])
        elif elem.tag == 'enum':
            if elem.get('value') is not None or elem.get('offset') is not None or elem.get('bitpos') is not None:
                self.required_enum_items.append(self._create_enum_item(elem))
            else:
                self.required_enum_items.append(self.registry.enum_items[name])
        else:
            raise ValueError('unknown require', elem)

    def remove(self, elem):
        name = elem.get(_NAME_)
        if name is None:
            raise ValueError('remove without name', elem)
        if elem.tag == 'type':
            self.removed_types.append(name)
        elif elem.tag == 'command':
            self.removed_commands.append(name)
        elif elem.tag == 'enum':
            self.removed_enum_items.append(name)
        else:
            raise ValueError('unknown require', elem)


class Registry(BaseElem):

    @staticmethod
    def load_file(file, feature_filter=None):
        return Registry.load_tree(etree.parse(file), feature_filter=feature_filter)

    @staticmethod
    def load_tree(tree, feature_filter=None):
        return Registry(tree.getroot(), feature_filter=feature_filter)

    def __init__(self, elem, feature_filter=None):
        BaseElem.__init__(self, elem)

        self.vendor_ids = NamedObjectDict()
        for vendorid_elem in elem.findall('vendorids/vendorid'):
            self.vendor_ids.addunique(Tag(self, vendorid_elem))

        self.tags = NamedObjectDict()
        for tag_elem in elem.findall('tags/tag'):
            self.tags.addunique(Tag(self, tag_elem))
        
        self.types = NamedObjectDict()
        for type_elem in elem.findall('types/type'):
            self.types.addunique(BaseTypeElem.new_from_category(type_elem.get('category'), self, type_elem))
        
        self.enum_groups = NamedObjectDict()
        for enum_group_elem in elem.findall('enums'):
            self.enum_groups.addunique(EnumGroup(self, enum_group_elem))
        
        self.enum_items = NamedObjectDict()
        for group in self.enum_groups:
            for item in group.enum_items:
                self.enum_items.addunique(item)
        
        self.commands = NamedObjectDict()
        for command_elem in elem.findall('commands/command'):
            self.commands.addunique(Command(self, command_elem))
        self._resolve()
        self._select_features(feature_filter)
        self._enrich_enum_groups()

    def _resolve(self):
        for ty in self.types:
            if hasattr(ty, '_resolve'):
                ty._resolve(self)
        for command in self.commands:
            command._resolve(self)
    
    def _select_features(self, feature_filter=None):
        if feature_filter is None:
            feature_filter = (lambda elem: (elem.get('supported') or elem.get('api') or 'vulkan') == 'vulkan')
        features = NamedObjectDict()
        def collect_feature(f_elem):
            feature = FeatureGroup(self, f_elem)
            features.addunique(feature)
            for r_elem in f_elem.findall('require'):
                if not feature_filter(r_elem):
                    continue
                for e in r_elem.findall('*'):
                    if e.tag == 'comment':
                        continue
                    feature.require(e)
            for r_elem in f_elem.findall('remove'):
                if not feature_filter(r_elem):
                    continue
                for e in r_elem.findall('*'):
                    if e.tag == 'comment':
                        continue
                    feature.remove(e)
        for f_elem in self.elem.findall('feature'):
            if not feature_filter(f_elem):
                continue
            collect_feature(f_elem)
        for f_elem in self.elem.findall('extensions/extension'):
            if not feature_filter(f_elem):
                continue
            collect_feature(f_elem)
        
        
        self.features = NamedObjectDict() # create re-ordered list of features
        already_included_features = set()
        def require_type(ty, f):
            if ty.requiering_feature is not None:
                #if ty.requiering_feature is not f and ty.requiering_feature.is_extension and ty.requiering_feature.name not in f.requires_recursive:
                #    raise ValueError('type required in multiple extensions', ty, f, ty.requiering_feature)
                return False
            ty.requiering_feature = f
            if isinstance(ty, EnumType) and ty.group.requiering_feature is None:
                ty.group.requiering_feature = f
            require_refs(_get_elem_requirements(ty.elem), f)
            f.types.append(ty)
            return True
        def require_enum_item(item, f):
            if item.requiering_feature is not None:
                #if item.requiering_feature is not f and item.requiering_feature.is_extension and item.requiering_feature.name not in f.requires_recursive:
                #    raise ValueError('enum item required in multiple extensions', item, f, item.requiering_feature)
                return False
            item.requiering_feature = f
            if item.enum_group is not None and item.enum_group.requiering_feature is None:
                item.enum_group.requiering_feature = f
            if item.enum_group is not None and item.enum_group.enum_type is not None:
                require_type(item.enum_group.enum_type, f)
            f.enum_items.append(item)
            return True
        def require_command(command, f):
            if command.requiering_feature is not None:
                if command.requiering_feature is not f and command.requiering_feature.is_extension and command.requiering_feature.name not in f.requires_recursive:
                    raise ValueError('command required in multiple extensions', command, f, command.requiering_feature)
                return False
            command.requiering_feature = f
            require_refs(_get_elem_requirements(command.elem), f)
            f.commands.append(command)
            return True
        def require_refs(refs, f):
            for t, name in refs:
                if t == 'type':
                    require_type(self.types[name], f)
                elif t == 'enum':
                    require_enum_item(self.enum_items[name], f)
                elif t == 'command':
                    require_command(self.commands[name], f)
        def require_feature(f):
            if f.name in already_included_features:
                return
            already_included_features.add(f.name)
            f.requires_recursive.update(f.requires)
            for r in f.requires:
                if r in already_included_features:
                    continue
                required_feature = features[r]
                require_feature(required_feature)
                f.requires_recursive.update(required_feature.requires_recursive)

            for ty in f.required_types:
                require_type(ty, f)
            for enum_item in f.required_enum_items:
                require_enum_item(enum_item, f)
            for command in f.required_commands:
                require_command(command, f)

            self.features.addunique(f)
        for f in features:
            require_feature(f)
    
    def _enrich_enum_groups(self):
        for group in self.enum_groups:
            prefix = camel_to_screaming_snake_case(group.name)
            maxprefix = len(prefix)
            while maxprefix>0:
                checkprefix = prefix[:maxprefix]
                all_ok = True
                for item in group.enum_items:
                    if not item.name.startswith(checkprefix):
                        all_ok = False
                        maxprefix = checkprefix.rfind('_')
                        break
                if all_ok:
                    break
            # store the results
            group.prefix = prefix[:maxprefix]
            for item in group.enum_items:
                item.shortname = item.name[maxprefix:]
                if item.shortname.startswith('_'):
                    item.shortname = item.shortname[1:]



def _get_elem_requirements(elem):
    for r in _split_attrib(elem.get('requires')):
        yield 'type', r
    for e in elem.findall('.//type'):
        yield 'type', e.text
    for e in elem.findall('.//enum'):
        yield 'enum', e.text

def _split_attrib(attrib):
    if attrib is None or len(attrib) <= 0:
        return []
    else:
        return attrib.split(',')

def _parse_typed_elem(elems, start=None):
    if start is None:
        start = elems.text or ''
    start = start.strip()
    if start == 'typedef' or start.startswith('typedef '):
        start = start[7:].strip()

    modifiers = start
    typename = None
    name = None
    in_array = False
    for elem in elems:
        text = elem.text or ''
        tail = elem.tail or ''
        if elem.tag == _NAME_:
            if name is not None:
                raise ValueError("there are multiple <name> tags")
            name = text
            modifiers += '{name}'
        elif elem.tag == _TYPE_:
            if typename is not None:
                raise ValueError("there are multiple <type> tags")
            typename = text
            modifiers += '{type}'
        elif elem.tag == _ENUM_:
            modifiers += text
        elif elem.tag == _COMMENT_:
            pass
        else:
            raise ValueError("Unexpected tag <%s>" % elem.tag)
        tail = tail.strip()
        end = tail.find(';')
        if end >= 0:
            tail = tail[:end]
            modifiers += tail
            break
        modifiers += tail
    return modifiers, typename, name



