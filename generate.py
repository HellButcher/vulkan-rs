#!/usr/bin/env python3
import os, re
from generator import Registry
from generator.rust import RustGenerator

_VULKAN_DOCS_ROOT_ = 'Vulkan-Docs/'
_VULKAN_SPEC_DIR_ = _VULKAN_DOCS_ROOT_ + 'src/spec/'
_SPECDEST_DIR_ = 'target/extracted-docs/'


_CARGO_TOML_ = 'Cargo.toml'

def read_full_docs(name):
    filename = _SPECDEST_DIR_ + name + '.md'
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            return f.readlines()
    except OSError:
        return None

def initialize_docs(registry, short=False):
    if short:
        from extract_docs import specfiles, extract_spec_pages
        import textwrap
        docs = dict()
        for page in extract_spec_pages(specfiles(), registry, short=True):
            docs[page.name] = page.desc
        def read_docs(name):
            desc = docs.get(name, None)
            if desc:
                desc = textwrap.wrap(desc, 80)
            return desc
    else:
        read_docs = read_full_docs
    for cmd in registry.commands:
        cmd.docs = read_docs(cmd.name)
    for ty in registry.types:
        ty.docs = read_docs(ty.name)


def update_cargo_toml(reg):
    with open(_CARGO_TOML_ + '.tmp', 'w', encoding='utf-8') as f_out:
        with open(_CARGO_TOML_, 'r', encoding='utf-8') as f_in:
            do_write=True
            for l in f_in.readlines():
                if l.rstrip() == '#END OF VULKAN FEATURES':
                    do_write = True
                if do_write:
                    f_out.write(l)
                if l.rstrip() == '#START OF VULKAN FEATURES':
                    do_write = False
                    for f in sorted(reg.features, key=lambda f: f.name):
                        if f.is_extension:
                            f_out.write('%s = [%s]\n' % (f.name, ', '.join(
                                ['"%s"' % r for r in f.requires]
                            )))
    os.rename(_CARGO_TOML_ + '.tmp', _CARGO_TOML_)

def main(args):
    registry = Registry.load_file(_VULKAN_SPEC_DIR_ + 'vk.xml')
    initialize_docs(registry, short=True)
    rust_gen = RustGenerator(registry)

    rust_gen.generate_all()
    update_cargo_toml(registry)

if __name__ == '__main__':
    import sys
    sys.exit(main(sys.argv))
