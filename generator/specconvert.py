import subprocess
from .utils import which

_asciidoctor_exe = None
_pandoc_exe = None

def call_asciidoctor(input_filename, output_filename=None, backend='docbook', doctype=None):
    global _asciidoctor_exe
    if _asciidoctor_exe is None:
        _asciidoctor_exe = which('asciidoctor')
        if _asciidoctor_exe is None:
            _asciidoctor_exe = False
            print('`asciidoctor` not found in PATH.')
            return
    if _asciidoctor_exe is False:
        return
    cmdline = [_asciidoctor_exe, '--no-header-footer', '--safe', '-b', backend]
    if doctype:
        cmdline.append('-d')
        cmdline.append(doctype)
    if output_filename:
        cmdline.append('-o')
        cmdline.append(output_filename)
    cmdline.append(input_filename)
    res = subprocess.run(cmdline)
    if res.returncode != 0:
        raise subprocess.CalledProcessError('Command %s returned %d', res.args, res.returncode)

def call_pandoc(input_filename, output_filename=None, frontend='docbook', backend='commonmark'):
    global _pandoc_exe
    if _pandoc_exe is None:
        _pandoc_exe = which('pandoc')
        if _pandoc_exe is None:
            _pandoc_exe = False
            print('`asciidoctor` not found in PATH.')
            return
    if _pandoc_exe is False:
        return
    cmdline = [_pandoc_exe, '--normalize', '--parse-raw', '--reference-links', '--wrap=auto', '--columns=80', '-f', frontend, '-t', backend]
    if output_filename:
        cmdline.append('-o')
        cmdline.append(output_filename)
    cmdline.append(input_filename)
    res = subprocess.run(cmdline)
    if res.returncode != 0:
        raise subprocess.CalledProcessError('Command %s returned %d', res.args, res.returncode)
