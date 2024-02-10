project('cachyos-hello', 'rust',
        version: '0.10.4',
        license: 'GPLv3',
        meson_version: '>=0.56.0',
        default_options: ['buildtype=debugoptimized',
                          'warning_level=3',
                          'werror=true',
                          'b_ndebug=if-release'])

i18n = import('i18n')
gnome = import('gnome')

base_id = 'org.cachyos.hello'

dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')
dependency('gtk+-3.0', version: '>= 3.24.33')

desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)
cargo = find_program('cargo', required: true)
cargo_script = find_program('build-aux/cargo.py')

version = meson.project_version()
version_array = version.split('.')
major_version = version_array[0].to_int()
minor_version = version_array[1].to_int()
version_micro = version_array[2].to_int()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'

if get_option('profile') == 'development'
  profile = 'Devel'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD').stdout().strip()
  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format(vcs_tag)
  endif
  application_id = '@0@.@1@'.format(base_id, profile)
else
  profile = ''
  version_suffix = ''
  application_id = base_id
endif

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + version,
  meson.project_source_root()
)

if get_option('profile') == 'development'
  # Setup pre-commit hook for ensuring coding style is always consistent
  message('Setting up git pre-commit hook..')
  run_command('cp', '-f', 'hooks/pre-commit.hook', '.git/hooks/pre-commit')
endif

cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

#subdir('po')
subdir('src')
subdir('icons')

install_data (
    meson.project_name () + '.desktop',
    install_dir: join_paths(get_option('datadir'), 'applications')
)

meson.add_install_script('postinstall.sh')
