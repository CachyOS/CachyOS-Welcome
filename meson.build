project('cachyos-hello', 'cpp',
        version: '0.6.9',
        license: 'GPLv3',
        meson_version: '>=0.55.0',
        default_options: ['cpp_std=c++17',
                          'buildtype=debugoptimized',
                          'warning_level=3',
                          'werror=false',
                          'b_ndebug=if-release'])

is_debug_build = get_option('buildtype').startswith('debug')
cc = meson.get_compiler('cpp')
if cc.get_id() == 'clang'
    specific_cc_flags = [
        '-nostdlib++',
        #'-stdlib=libc++',
        '-nodefaultlibs',
    ]
    specific_link_flags = [
        '-fuse-ld=lld',
    ]
    add_global_arguments(cc.get_supported_arguments(specific_cc_flags), language : 'cpp')
    add_global_link_arguments(cc.get_supported_link_arguments(specific_link_flags), language : 'cpp')
endif

if is_debug_build
    add_global_arguments('-D_GLIBCXX_ASSERTIONS', language : 'cpp')
endif

# Common dependencies
fmt = dependency('fmt', version : ['>=8.0.0'], fallback : ['fmt', 'fmt_dep'])
gtkmm = dependency('gtkmm-4.0', version : ['>=1.8.0'])

src_files = files(
    'src/hello.cpp', 'src/hello.hpp',
    'src/main.cpp',
)

possible_cc_flags = [
    '-Wshadow',

    '-Wnon-virtual-dtor',

    '-Wold-style-cast',
    '-Wcast-align',
    '-Wunused',
    '-Woverloaded-virtual',

    '-Wpedantic', # non-standard C++
    '-Wconversion', # type conversion that may lose data
    '-Wsign-conversion',
    '-Wnull-dereference',
    '-Wdouble-promotion', # float to double

    '-Wformat=2',
    '-Wimplicit-fallthrough', # fallthrough without an explicit annotation
]

if cc.get_id() == 'gcc'
    possible_cc_flags += [
        '-Wmisleading-indentation',

        '-Wduplicated-cond',
        '-Wduplicated-branches',
        '-Wlogical-op',
        '-Wuseless-cast',

        '-Wsuggest-attribute=cold',
        '-Wsuggest-attribute=format',
        '-Wsuggest-attribute=malloc',
        '-Wsuggest-attribute=noreturn',
        '-Wsuggest-attribute=pure',
        '-Wsuggest-final-methods',
        '-Wsuggest-final-types',
        '-Wdiv-by-zero',
        '-Wanalyzer-double-fclose',
        '-Wanalyzer-double-free',
        '-Wanalyzer-malloc-leak',
        '-Wanalyzer-use-after-free',
    ]
endif

if not is_debug_build
   if cc.get_id() == 'gcc'
      possible_cc_flags += [
         '-flto',
         '-fwhole-program',
         '-fuse-linker-plugin',
      ]
   else
      possible_cc_flags += [
         '-flto=thin',
      ]
   endif

   possible_cc_flags += ['-fdata-sections', '-ffunction-sections']
   possible_link_flags = ['-Wl,--gc-sections', '-Wl,--export-dynamic']
   add_project_link_arguments(cc.get_supported_link_arguments(possible_link_flags), language : 'cpp')
endif

add_project_arguments(cc.get_supported_arguments(possible_cc_flags), language : 'cpp')

executable(
  'cachyos-hello',
  src_files,
  dependencies: [fmt, gtkmm],
  include_directories: [include_directories('src')],
  install: true)

install_data (
    meson.project_name () + '.desktop',
    install_dir: join_paths(get_option('datadir'), 'applications')
)

meson.add_install_script('postinstall.sh')

summary(
  {
    'Build type': get_option('buildtype'),
  },
  bool_yn: true
)

clangtidy = find_program('clang-tidy', required: false)

if clangtidy.found()
    run_target(
        'tidy',
        command: [
            clangtidy,
            '-checks=*,-fuchsia-default-arguments',
            '-p', meson.build_root()
        ] + src_files)
endif
