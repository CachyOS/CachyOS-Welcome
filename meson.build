project('cachyos-hello', 'cpp',
        version: '0.6.7',
        license: 'GPLv3',
        meson_version: '>=0.53.0',
        default_options: ['cpp_std=c++17',
                          'warning_level=1',
                          'werror=true',
                          'b_ndebug=if-release'])

# Common dependencies
fmt = dependency('fmt', version : ['>=8.0.0'], fallback : ['fmt', 'fmt_dep'])
gtkmm = dependency('gtkmm-3.0', version : ['>=3.22.0'])

src_files = files(
    'src/hello.cpp', 'src/hello.hpp',
    'src/main.cpp',
)

executable(
  'cachyos-hello',
  src_files,
  dependencies: [fmt, gtkmm],
  include_directories: [include_directories('src')],
  install: true)

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
