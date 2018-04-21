#!/usr/bin/env python3
import os, re
from generator import Registry, extract_spec_pages, call_asciidoctor, call_pandoc

_VULKAN_DOCS_ROOT_ = 'Vulkan-Docs/'
_VULKAN_SPEC_DIR_ = _VULKAN_DOCS_ROOT_ + 'src/spec/'
_VULKAN_SPEC_DOC_DIR_ = _VULKAN_DOCS_ROOT_ + 'doc/specs/vulkan/'
_SPECFILES_ = [
    _VULKAN_SPEC_DOC_DIR_ + 'chapters/[A-Za-z]*.txt',
    _VULKAN_SPEC_DOC_DIR_ + 'appendices/[A-Za-z]*.txt',
    _VULKAN_SPEC_DOC_DIR_ + 'chapters/*/[A-Za-z]*.txt',
    _VULKAN_SPEC_DOC_DIR_ + 'appendices/*/[A-Za-z]*.txt',
]
_SPECDEST_DIR_ = 'target/extracted-docs/'

_BASE_REF_URL_='https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html'
_RE_ANCHOR_=re.compile(r'\]\(\#([a-zA-Z0-9_\-]+)\)')
_SUBST_ANCHOR_='][%s]'

def specfiles():
    from glob import glob
    for pattern in _SPECFILES_:
        for match in glob(pattern):
            yield match

def main(args):
    registry = Registry.load_file(_VULKAN_SPEC_DIR_ + 'vk.xml')
    os.makedirs(_SPECDEST_DIR_, exist_ok=True)
    for page in extract_spec_pages(specfiles(), registry):
        adoc_filename = _SPECDEST_DIR_ + page.name + '.adoc'
        docbook_filename = _SPECDEST_DIR_ + page.name + '.xml'
        md_filename = _SPECDEST_DIR_ + page.name + '.md'
        print('writing', adoc_filename, '; name =', page.name, ', type = ', page.type)
        with open(adoc_filename , 'w', encoding='utf-8') as f:
            f.writelines([l + '\n' for l in page.lines])
        call_asciidoctor (adoc_filename, docbook_filename)
        call_pandoc (docbook_filename, md_filename+'.tmp')
        with open(md_filename+'.tmp' , 'r', encoding='utf-8') as md_in:
            with open(md_filename , 'w', encoding='utf-8') as md_out:
                anchors=set()
                def subst(m):
                    a = m.group(1)
                    anchors.add(a)
                    return _SUBST_ANCHOR_ % a
                for line in md_in.readlines():
                    line = _RE_ANCHOR_.sub(subst, line)
                    md_out.write(line)
                if len(anchors) > 0:
                    md_out.write('\n')
                    for a in sorted(anchors):
                        md_out.write('[%s]: %s#%s\n' % (a, _BASE_REF_URL_, a))

        os.remove(md_filename+'.tmp')

if __name__ == '__main__':
    import sys
    sys.exit(main(sys.argv))
