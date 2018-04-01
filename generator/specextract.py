import re, sys

_RE_BEGIN_ = re.compile(r"^\[open,(?P<attribs>refpage=.*)\]")
_RE_ATTRIB_ = re.compile(r"([a-z]+)='([^'\\]*(?:\\.[^'\\]*)*)'")
_RE_CODE_NAME_ = re.compile(r"\b(?:[spefd]name|code):([a-zA-Z][a-zA-Z0-9_]*)\b")
_RE_CODE_LINK_ = re.compile(r"\b[spefd]link:([a-zA-Z][a-zA-Z0-9_]*)\b")
_RE_CODE_SCOPE_ = re.compile(r"\`([a-zA-Z][a-zA-Z0-9_]*)\`\:\:\`([a-zA-Z][a-zA-Z0-9_]*)\`")
_RE_ANCHOR_ = re.compile(r"\[\[([a-zA-Z0-9_\-]+)\]\]")

class Page:
    _TYPES_=set(['structs', 'protos', 'funcpointers', 'flags', 'enums', 'handles', 'basetypes', 'defines'])
    def __init__(self, refpage, type, desc=None, xrefs=None):
        self.name = refpage
        self.desc = desc
        self.type = type
        self.xrefs = xrefs
        self.lines = [desc, '']
        if desc is None:
            raise ValueError('unknown missing desc for %s' % self.name)
        if self.type not in Page._TYPES_:
            raise ValueError('unknown type %s for %s' % (self.type, self.name))
    def append(self, line):
        if not line and len(self.lines)>0 and not self.lines[-1]:
            return
        line = _RE_CODE_NAME_.sub(r'`\1`', line)
        line = _RE_CODE_LINK_.sub(r'`\1`', line)
        line = _RE_CODE_SCOPE_.sub(r'`\1::\2`', line)
        self.lines.append(line)

def extract_spec_pages(specs, registry):
    for s in specs:
        for page in extract_spec_page(s, registry):
            yield page

def extract_spec_page(file, registry):
    if isinstance(file, str):
        with open(file, 'r', encoding='utf-8') as f:
            try:
                for page in extract_spec_page(f, registry):
                    yield page
            except ValueError as e:
                print('ValueError', e, 'in file', file, file=sys.stderr)
            except KeyError as e:
                print('KeyError', e, 'in file', file, file=sys.stderr)
        return
    ifdef_sects=[]
    ifdef_dis=0
    state=None
    page = None
    def check_section(sec):
        for s1 in sec.split(','):
            res = True
            for s2 in sec.split('+'):
                if s2 not in registry.features:
                    res = False
                    break
            if res is True:
                return True
        return False
    def start_ifndef(line, check):
        nonlocal ifdef_dis
        if line.endswith('[]'):
            section = line[:-2]
            ifdef_sects.append(section)
            if ifdef_dis == 0 and not check(section):
                ifdef_dis = len(ifdef_sects)
            return None
        elif line.endswith(']'):
            ofs = line.index('[')
            section = line[:ofs]
            line = line[ofs+1:-1]
            return line
        else:
            raise ValueError('unable to handle line ifdef::%s', line)
    for line in iter(file.readlines()):
        line=line.rstrip()
        if line.startswith('ifdef::'):
            line = start_ifndef(line[7:], check_section)
            if line is None:
                continue
        if line.startswith('ifndef::'):
            line = start_ifndef(line[8:], lambda s: not check_section(s))
            if line is None:
                continue
        if line.startswith('endif::'):
            line = line[7:]
            if line.endswith('[]'):
                section = line[:-2]
            else:
                raise ValueError('unable to handle line endif::%s', line)
            if len(ifdef_sects) == 0 or ifdef_sects[-1] != section:
                raise KeyError('ifdef-endif pairs for %s are dissync' % line)
            ifdef_sects.pop()
            if len(ifdef_sects) < ifdef_dis:
                ifdef_dis = 0
            continue
        if ifdef_dis>0:
            continue
        m = _RE_BEGIN_.search(line)
        if m is not None:
            state = 'begin'
            attribs = dict(_RE_ATTRIB_.findall(m.group('attribs')))
            page = Page(**attribs)
        elif page is None:
            continue #ignore content that is outside of pages
        elif line == '--':
            if state == 'begin':
                state = None
            elif state is None:
                yield page
                page = None
            else:
                raise ValueError('unexpected state `%s`' % state) 
        elif line.startswith('include::../api'):
            # rewrite some formulations (because the `include::../api` will not be included)
            l = -1
            check = page.lines[-1]
            if check == '':
                l = -2
                check = page.lines[-2]
            if check.startswith('The ') and (check.endswith(' is defined as:') or check.endswith(' is:') or check.endswith(' are:')):
                del page.lines[l]
            elif check.startswith('To ') and check.endswith(', call:'):
                del page.lines[l]
            elif check.endswith(', call:'):
                page.lines[l] = check[:-7] + '.'
            elif check.endswith(', are:'):
                page.lines[l] = check[:-6] + '.'
            elif check.endswith(':'):
                page.lines[l] = check[:-1] + '.'
        elif line.startswith('include::'):
            continue #ignore line
        elif line.startswith('.Valid-Usage') or line.startswith('.Valid Usage'):
            if state is not None:
                raise ValueError('unexpected state `%s`' % state) 
            state = 'valid-usage:begin'
        elif state == 'valid-usage:begin':
            if line == '****':
                state = 'valid-usage:body'
            else:
                raise ValueError('unexpected line `%s`' % line) 
        elif state == 'valid-usage:body':
            if line == '****':
                state = None
            else:
                continue # ignore line
        elif state is not None:
            raise ValueError('unexpected state `%s`' % state) 
        else:
            page.append(line)

