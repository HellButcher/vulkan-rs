#!/usr/bin/python3
#
# Copyright (c) 2017 Christoph Hommelsheim
# Copyright (c) 2013-2017 The Khronos Group Inc.
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

import argparse, cProfile, pdb, string, sys, time, os
__file_dir__ = os.path.dirname(os.path.abspath(__file__))
__vkspec_dir__ = os.path.join(os.path.dirname(__file_dir__), 'vulkan_spec', 'Vulkan-Docs', 'src', 'spec')
__vkreg_file__ = os.path.join(__vkspec_dir__, 'vk.xml')

if not os.path.isdir(__vkspec_dir__):
    raise Exception("""directory %s doesn't exist, please run:
      $ cd %s
      $ git clone https://github.com/KhronosGroup/Vulkan-Docs
    """%(__vkspec_dir__, __file_dir__))

sys.path = [__vkspec_dir__] + sys.path

from reg import *
from generator import write
from rustgenerator import RustGeneratorOptions, RustTypesOutputGenerator, \
    RustFfiOutputGenerator, RustSafeOutputGenerator, RustAliasOutputGenerator, \
    RustUtilsGeneratorOptions, RustUtilsOutputGenerator

# Simple timer functions
startTime = None

def startTimer(timeit):
    global startTime
    startTime = time.clock()

def endTimer(timeit, msg):
    global startTime
    endTime = time.clock()
    if (timeit):
        write(msg, endTime - startTime, file=sys.stderr)
        startTime = None

# Turn a list of strings into a regexp string matching exactly those strings
def makeREstring(list):
    return '^(' + '|'.join(list) + ')$'

# Returns a directory of [ generator function, generator options ] indexed
# by specified short names. The generator options incorporate the following
# parameters:
#
# extensions - list of extension names to include.
# protect - True if re-inclusion protection should be added to headers
# directory - path to directory in which to generate the target(s)
def makeGenOpts(extensions = [], removeExtensions = [], protect = True, directory = '.'):
    global genOpts
    genOpts = {}

    # Descriptive names for various regexp patterns used to select
    # versions and extensions
    allVersions     = allExtensions = '.*'
    noVersions      = noExtensions = None

    addExtensions     = makeREstring(extensions)
    removeExtensions  = makeREstring(removeExtensions)

    # Copyright text prefixing all headers (list of strings).
    prefixStrings = [
        '/*',
        '** Copyright (c) 2015-2017 The Khronos Group Inc.',
        '**',
        '** Licensed under the Apache License, Version 2.0 (the "License");',
        '** you may not use this file except in compliance with the License.',
        '** You may obtain a copy of the License at',
        '**',
        '**     http://www.apache.org/licenses/LICENSE-2.0',
        '**',
        '** Unless required by applicable law or agreed to in writing, software',
        '** distributed under the License is distributed on an "AS IS" BASIS,',
        '** WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.',
        '** See the License for the specific language governing permissions and',
        '** limitations under the License.',
        '*/',
        ''
    ]

    # Text specific to Vulkan headers
    vkPrefixStrings = [
        '/*',
        '** This file is generated from the Khronos Vulkan XML API Registry.',
        '**',
        '*/',
        ''
    ]

    # Vulkan 1.0 - source for core API + extensions.
    # To generate just the core API,
    # change to 'defaultExtensions = None' below.
    commonOptions = dict(
        directory         = directory,
        apiname           = 'vulkan',
        profile           = None,
        versions          = allVersions,
        emitversions      = allVersions,
        defaultExtensions = 'vulkan',
        addExtensions     = None,
        removeExtensions  = None,
        prefixText        = prefixStrings + vkPrefixStrings,
    )
    #
    genOpts['vulkan_types.rs'] = [
          RustTypesOutputGenerator,
          RustGeneratorOptions(
            filename          = 'vulkan_types.rs',
            **commonOptions)
        ]
    genOpts['vulkan_ffi.rs'] = [
          RustFfiOutputGenerator,
          RustGeneratorOptions(
            filename          = 'vulkan_ffi.rs',
            **commonOptions)
        ]
    genOpts['vulkan_safe.rs'] = [
          RustSafeOutputGenerator,
          RustGeneratorOptions(
            filename          = 'vulkan_safe.rs',
            **commonOptions)
        ]
    genOpts['vulkan_alias.rs'] = [
          RustAliasOutputGenerator,
          RustGeneratorOptions(
            filename          = 'vulkan_alias.rs',
            **commonOptions)
        ]
    genOpts['vulkan_utils.rs'] = [
          RustUtilsOutputGenerator,
          RustUtilsGeneratorOptions(
            filename               = 'vulkan_utils.rs',
            generateGetName        = [ 'VkResult' ],
            generateGetDescription = [ 'VkResult' ],
            generateGetFromStr     = [ 'VkShaderStageFlagBits', 'VkLogicOp',
                'VkStencilOp', 'VkBlendOp', 'VkBlendFactor', 'VkFrontFace',
                'VkCullModeFlagBits', 'VkCompareOp',
                'VkFilter', 'VkIndexType', 'VkPrimitiveTopology', 'VkPolygonMode'],
            **commonOptions)
        ]
#
# Generate a target based on the options in the matching genOpts{} object.
# This is encapsulated in a function so it can be profiled and/or timed.
# The args parameter is an parsed argument object containing the following
# fields that are used:
#   target - target to generate
#   directory - directory to generate it in
#   protect - True if re-inclusion wrappers should be created
#   extensions - list of additional extensions to include in generated
#   interfaces
def genTarget(args):
    global genOpts

    # Create generator options with specified parameters
    makeGenOpts(extensions = args.extension,
                removeExtensions = args.removeExtension,
                protect = args.protect,
                directory = args.directory)
    for target in args.target:
        if target not in genOpts.keys():
            write('No generator options for unknown target:', target, file=sys.stderr)
            return
    for target in args.target:
        createGenerator = genOpts[target][0]
        options = genOpts[target][1]

        if not args.quiet:
            write('* Building', options.filename, file=sys.stderr)

        startTimer(args.time)
        gen = createGenerator(errFile=errWarn,
                              warnFile=errWarn,
                              diagFile=diag)
        reg.apiReset()
        reg.setGenerator(gen)
        reg.apiGen(options)

        if not args.quiet:
            write('* Generated', options.filename, file=sys.stderr)
        endTimer(args.time, '* Time to generate ' + options.filename + ' =')


# -extension name - may be a single extension name, a a space-separated list
# of names, or a regular expression.
if __name__ == '__main__':
    parser = argparse.ArgumentParser()

    parser.add_argument('-extension', action='append',
                        default=[],
                        help='Specify an extension or extensions to add to targets')
    parser.add_argument('-removeExtension', action='append',
                        default=[],
                        help='Specify an extension or extensions to remove from targets')
    parser.add_argument('-debug', action='store_true',
                        help='Enable debugging')
    parser.add_argument('-dump', action='store_true',
                        help='Enable dump to stderr')
    parser.add_argument('-diagfile', action='store',
                        default=None,
                        help='Write diagnostics to specified file')
    parser.add_argument('-errfile', action='store',
                        default=None,
                        help='Write errors and warnings to specified file instead of stderr')
    parser.add_argument('-noprotect', dest='protect', action='store_false',
                        help='Disable inclusion protection in output headers')
    parser.add_argument('-profile', action='store_true',
                        help='Enable profiling')
    parser.add_argument('-registry', action='store',
                        default=__vkreg_file__,
                        help='Use specified registry file instead of vk.xml')
    parser.add_argument('-time', action='store_true',
                        help='Enable timing')
    parser.add_argument('-validate', action='store_true',
                        help='Enable group validation')
    parser.add_argument('-o', action='store', dest='directory',
                        default='.',
                        help='Create target and related files in specified directory')
    parser.add_argument('target', metavar='target', nargs='+',
                        help='Specify target')
    parser.add_argument('-quiet', action='store_true', default=False,
                        help='Suppress script output during normal execution.')

    args = parser.parse_args()

    # This splits arguments which are space-separated lists
    args.extension = [name for arg in args.extension for name in arg.split()]

    # Load & parse registry
    reg = Registry()

    startTimer(args.time)
    tree = etree.parse(args.registry)
    endTimer(args.time, '* Time to make ElementTree =')

    startTimer(args.time)
    reg.loadElementTree(tree)
    endTimer(args.time, '* Time to parse ElementTree =')

    if (args.validate):
        reg.validateGroups()

    if (args.dump):
        write('* Dumping registry to regdump.txt', file=sys.stderr)
        reg.dumpReg(filehandle = open('regdump.txt', 'w', encoding='utf-8'))

    # create error/warning & diagnostic files
    if (args.errfile):
        errWarn = open(args.errfile, 'w', encoding='utf-8')
    else:
        errWarn = sys.stderr

    if (args.diagfile):
        diag = open(args.diagfile, 'w', encoding='utf-8')
    else:
        diag = None

    if (args.debug):
        pdb.run('genTarget(args)')
    elif (args.profile):
        import cProfile, pstats
        cProfile.run('genTarget(args)', 'profile.txt')
        p = pstats.Stats('profile.txt')
        p.strip_dirs().sort_stats('time').print_stats(50)
    else:
        genTarget(args)
