project('cachyos-hello', 'rust',
        version: '0.10.8',
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

cargo = find_program('cargo', required: true)
cargo_script = find_program('build-aux/cargo.py')

version = meson.project_version()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'

if get_option('profile') == 'development'
  profile = 'Devel'
  application_id = '@0@.@1@'.format(base_id, profile)
else
  profile = ''
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
