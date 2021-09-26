project('cachyos-hello', 'cpp',
        version: '0.6.8',
        license: 'GPLv3',
        meson_version: '>=0.55.0',
        default_options: ['cpp_std=c++17',
                          'buildtype=debugoptimized',
                          'warning_level=3',
                          'werror=true',
                          'b_ndebug=if-release'])

cc = meson.get_compiler('cpp')

# Common dependencies
fmt = dependency('fmt', version : ['>=8.0.0'], fallback : ['fmt', 'fmt_dep'])
gtkmm = dependency('gtkmm-3.0', version : ['>=3.22.0'])

src_files = files(
    'src/hello.cpp', 'src/hello.hpp',
    'src/main.cpp',
)

possible_cc_flags = [
        '-Wshadow',

        '-Wnon-virtual-dtor',

        '-Wcast-align',
        '-Wunused',
        '-Woverloaded-virtual',

        '-Wpedantic', # non-standard C++
        '-Wconversion', # type conversion that may lose data
        '-Wnull-dereference',
        '-Wdouble-promotion', # float to double

        '-Wformat=2',
]

if cc.get_id() == 'gcc'
        possible_cc_flags += [
                '-Wmisleading-indentation',

                '-Wduplicated-cond',
                '-Wduplicated-branches',
                '-Wlogical-op',
        ]
endif

if get_option('buildtype') != 'debug'
   if cc.get_id() == 'gcc'
      possible_cc_flags += [
         '-flto',
         '-fwhole-program',
      ]
   else
      possible_cc_flags += [
         '-flto=thin',
      ]
   endif
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
