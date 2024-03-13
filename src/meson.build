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
