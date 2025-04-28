project(
  meson.project_name(),
  'rust',
  version: '0.16.4',
  license: 'GPL-3.0-or-later',
  meson_version: '>= 0.59.0',
  default_options: [
    'buildtype=debugoptimized',
    'warning_level=1',
    'profile=default',
  ],
)

cargo = find_program('cargo')

# Qt6 Dependencies
qt6_dep = dependency('qt6',
  modules: ['Core', 'Qml', 'Quick', 'QuickControls2'],
  required: true
)
kirigami_dep = dependency('KF6Kirigami', # Or 'KF5Kirigami' if targeting Qt5/KF5
  modules: ['Kirigami'],
  required: true
)

executable(
  meson.project_name(),
  [], # No C/C++ sources, handled by cargo
  dependencies: [qt6_dep, kirigami_dep], # Link against Qt/Kirigami libs
  install: false, # Binary installed via cargo_build target
)

manifest_path = meson.project_source_root() / 'Cargo.toml'
cargo_home = meson.project_build_root() / 'cargo-home'
cargo_target_dir = meson.project_build_root() / 'src'

cargo_options = [ '--manifest-path', manifest_path ]
cargo_options += [ '--target-dir', cargo_target_dir ]

if get_option('profile') == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

cargo_env = [ 'CARGO_HOME=' + cargo_home ]

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)

test(
  'cargo-test',
  cargo,
  args: [
    'test',
    '--manifest-path=@0@'.format(manifest_path),
    '--target-dir=@0@'.format(cargo_target_dir),
    '--',
    '--nocapture',
  ],
  env: [
    'RUST_BACKTRACE=1',
    'CARGO_HOME=@0@'.format(cargo_home),
    'PATH=/app/bin:/usr/bin:/usr/lib/sdk/rust-stable/bin',
  ],
  timeout: 300, # give cargo more time
)

test(
  'cargo-clippy',
  cargo,
  args: [
    'clippy',
    '--manifest-path=@0@'.format(manifest_path),
    '--target-dir=@0@'.format(cargo_target_dir),
  ],
  env: [
    'CARGO_HOME=@0@'.format(cargo_home),
    'PATH=/app/bin:/usr/bin:/usr/lib/sdk/rust-stable/bin',
  ],
  timeout: 300, # give cargo more time
)

# Install QML files (adjust path as needed)
install_data(
  files('ui/main.qml', 'ui/HomePage.qml', 'ui/InfoPage.qml'), # Add other QML files here
  install_dir: get_option('datadir') / meson.project_name() / 'ui'
)

# Install icons/images (check if RESPREFIX needs adjustment based on Qt resource system)
install_data(
  # Assuming images are still needed and embedded/loaded differently
  # Update paths and potentially use Qt resource system instead
  # glob('data/img/*.png'), 
  # glob('data/img/*.svg'),
  # install_dir: get_option('datadir') / meson.project_name() / 'data/img'
)

# Install locale files (assuming i18n-embed still handles this)
# install_subdir(
#   'locale',
#   install_dir: get_option('localedir')
# )

# Install desktop file and icon (adjust icon name/path if needed)
app_id = 'io.github.cachyos.cachyos_welcome'
app_icon = app_id
appstream_id = app_id + '.appdata.xml'
desktop_file = app_id + '.desktop'

appstream_util = find_program('appstream-util', required: false)
if appstream_util.found()
  test('Validate appstream file', appstream_util, args: ['validate', meson.project_source_root() / 'data' / appstream_id])
endif

install_data(appstream_id, install_dir: get_option('datadir') / 'metainfo')
install_data(app_id + '.png', install_dir: get_option('datadir') / 'pixmaps') # Assuming PNG icon
install_data(desktop_file, install_dir: get_option('datadir') / 'applications')

# Meson runs cargo build
cargo_target_dir = meson.project_build_root() / 'src'
cargo_build_target_path = cargo_target_dir / get_option('profile') / meson.project_name()

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  # build_always_stale: true, # May not be needed if meson tracks inputs/outputs correctly
  output: meson.project_name() + '.bin', # Temporary output name
  command: ['env', 'meson-cargo', meson.project_build_root()],
  depends: [], # Meson should auto-detect Rust source changes
  capture: true
)

install_data(cargo_build_target_path, rename: meson.project_name(), install_dir: get_option('bindir'))


# Configure files (config.rs, potentially others)
conf = configuration_data()
conf.set('APP_ID', app_id)
conf.set('PROFILE', get_option('profile'))
conf.set('VERSION', meson.project_version())
conf.set('LOCALEDIR', get_option('prefix') / get_option('localedir'))
conf.set('PKGDATA', get_option('prefix') / get_option('datadir') / meson.project_name())
conf.set('APPICON', app_icon)

config_rs = configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: conf,
  install: false
)

# Ensure config.rs is generated before cargo build
# This might require a custom build script or more advanced meson setup


# Tests (need update for Qt)
# test(
#   'cargo-test',
#   cargo,
#   args: [
#     'test',
#     '--manifest-path=@0@'.format(meson.project_source_root() / 'Cargo.toml'),
#     '--target-dir=@0@'.format(cargo_target_dir),
#     '--',
#     '--nocapture',
#   ],
#   env: [
#     'RUST_BACKTRACE=1',
#     'CARGO_HOME=@0@'.format(meson.project_build_root() / 'cargo-home'),
#     # Potentially add QT_PLUGIN_PATH etc. if needed for tests
#   ],
#   timeout: 300,
# )

# test(
#   'cargo-clippy',
#   cargo,
#   args: [
#     'clippy',
#     '--manifest-path=@0@'.format(meson.project_source_root() / 'Cargo.toml'),
#     '--target-dir=@0@'.format(cargo_target_dir),
#   ],
#   env: [
#     'CARGO_HOME=@0@'.format(meson.project_build_root() / 'cargo-home'),
#   ],
#   timeout: 300,
# )
