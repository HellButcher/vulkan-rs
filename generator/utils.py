import os, io, re, sys
from enum import Enum

class CodeGenerator:
    def __init__(self, out, linesep=os.linesep):
        self._owned_out = False
        if isinstance(out, str):
            out = io.open(out, 'w')
            self._owned_out = True
        self.indent = ''
        self.indent_chars ='\t'
        self.out = out
        self.linesep = linesep
        self._at_begin_of_line = True
        self._listeners = []

    def __enter__(self):
        return self

    def __exit__(self, type, value, traceback):
        self.close()

    def __getattr__(self, name):
        return getattr(self.out, name)

    def __call__(self, *args, **kwargs):
        return self.write(*args)

    def close(self):
        if self._owned_out:
            self.out.close()

    def i(self, indent_chars=None):
        if indent_chars is None:
            indent_chars = self.indent_chars
        self.indent += indent_chars
        return self

    def o(self, indent_chars=None):
        if indent_chars is None:
            indent_chars = self.indent_chars
        if self.indent.endswith(indent_chars):
            self.indent = self.indent[:-len(indent_chars)]
        else:
            raise ValueError("inconsistend indention", self.indent, indent_chars)
        return self

    def open_indention(self, f=None):
        if f is None:
            w = getattr(self, '_open_indention', None)
            if w is None:
                w = _WithIndention(self)
                self._open_indention = w
            return w
        else:
            self.i()
            try:
                f()
            finally:
                self.o()

    def open_nonempty(self, f=None):
        if f is not None:
            with self.open_empty_filter(self) as sub:
                f(sub)
            return
        return _WithNonEmpty(self)

    def _notify_listeners(self, method_name, reverse=False):
        listeners = self._listeners
        if reverse is True:
            listeners = reversed(listeners)
        for l in listeners:
            m = getattr(l, method_name, None)
            if m is not None:
                m(self)

    def write(self, *args, **kwargs):
        out = kwargs.pop('out', self.out)
        def write_line(s):
            if len(s) > 0:
                self._notify_listeners('_on_before_write')
                if self._at_begin_of_line:
                    out.write(self.indent)
                out.write(s)
                self._at_begin_of_line = False
                self._notify_listeners('_on_after_write', reverse=True)
        for arg in args:
            s = str(arg)
            eol = s.find('\n')
            while eol >= 0:
                l = s[:eol]
                s = s[eol+1:]
                eol = s.find('\n')
                if l.endswith('\r'):
                    l = l[:-1]
                write_line(l)
                self.nl()
            write_line(s)
        return self

    def nl(self, **kwargs):
        out = kwargs.pop('out', self.out)
        self._notify_listeners('_on_before_write')
        out.write(self.linesep)
        self._at_begin_of_line = True
        self._notify_listeners('_on_after_write', reverse=True)
        return self

class _WithIndention:
    def __init__(self, gen):
        self.gen = gen

    def __enter__(self):
        return self.gen.i()

    def __exit__(self, type, value, traceback):
        self.gen.o()

class _WithNonEmpty:
    def __init__(self, gen, *args, **kwargs):
        self.empty_gen = CodeGenerator(io.StringIO(), *args, **kwargs)
        self.base_gen = gen
        self.base_gen._listeners.append(self)

    def __enter__(self):
        if self.empty_gen is None:
            raise ValueError('invalid state')
        return self

    def __exit__(self, type, value, traceback):
        self.close()

    def __getattr__(self, name):
        if self.empty_gen is None:
            return getattr(self.base_gen, name)
        else:
            return getattr(self.empty_gen, name)

    def __call__(self, *args, **kwargs):
        return self.write(*args, **kwargs)

    def close(self):
        if self.empty_gen is not None:
            self.base_gen._listeners.remove(self)
            self.empty_gen = None

    def _on_before_write(self, gen):
        if self.empty_gen is None:
            raise ValueError('invalid state')
        v = self.empty_gen.out.getvalue()
        i = self.empty_gen.indent
        self.close()
        self.base_gen.write(v)
        self.base_gen.i(i)



def delegate_type(target, method_prefix, ty, *args, **kwargs):
    cat = ty._CATEGORY_
    if cat is not None:
        f = getattr(target, method_prefix+cat, None)
        if f is not None:
            f(ty, *args, **kwargs)

def _identity(s):
    return s

def _lower(s):
    return s.lower()

def _upper(s):
    return s.lower()

def _lower_first(s):
    if len(s) <= 0:
        return s
    else:
        return s[0].lower() + s[1:]

def _upper_first(s):
    if len(s) <= 0:
        return s
    else:
        return s[0].upper() + s[1:]

_first_cap_re = re.compile('(.)([A-Z][a-z]+)')
_all_cap_re = re.compile('([a-z0-9])([A-Z])')
_merge_underscore_re = re.compile('__+')

def camel_to_snake_case(s):
    s = _first_cap_re.sub(r'\1_\2', s)
    s = _all_cap_re.sub(r'\1_\2', s).lower()
    s = _merge_underscore_re.sub(r'_', s)
    return s

def camel_to_screaming_snake_case(s):
    return camel_to_snake_case(s).upper()

def snake_to_upper_camel_case(s):
    return ''.join([part.capitalize() for part in s.split('_')])

def snake_to_lower_camel_case(s):
    return _lower_first(snake_to_upper_camel_case(s))

class Case(Enum):
    Unspecified = 'unspecified'
    Lower = 'lower'
    Upper = 'upper'
    LowerCamel = 'lower_camel'
    UpperCamel = 'upper_camel'
    Snake = 'snake'
    ScreamingSnake = 'screaming_snake'

    def from_case(self, s):
        if '_' in s:
            return self.from_snake(s)
        else:
            return self.from_camel(s)

Case.Unspecified.from_snake = _identity
Case.Unspecified.from_camel = _identity
Case.Lower.from_snake = _lower
Case.Lower.from_camel = _lower
Case.Upper.from_snake = _upper
Case.Upper.from_camel = _upper
Case.LowerCamel.from_camel = _lower_first
Case.LowerCamel.from_snake = snake_to_lower_camel_case
Case.UpperCamel.from_camel = _upper_first
Case.UpperCamel.from_snake = snake_to_upper_camel_case
Case.Snake.from_camel = camel_to_snake_case
Case.Snake.from_snake = _lower
Case.ScreamingSnake.from_camel = camel_to_screaming_snake_case
Case.ScreamingSnake.from_snake = _upper

def _is_exe(fpath):
    return os.path.isfile(fpath) and os.access(fpath, os.X_OK)

def which(program):
    if sys.platform == "win32" and not program.endswith(".exe"):
        program += ".exe"

    fpath, fname = os.path.split(program)
    if fpath:
        if _is_exe(program):
            return program
    else:
        for path in os.environ["PATH"].split(os.pathsep):
            exe_file = os.path.join(path, program)
            if _is_exe(exe_file):
                return exe_file

    return None
