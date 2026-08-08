# Contributing to CachyOS Welcome

## Prerequisites

- Rust (stable, via `rustup`)
- GTK 3.24.33+
- `glib-2.0` and `gio-2.0` 2.66+

Install all dependencies on CachyOS/Arch:

```sh
sudo pacman -S --needed rustup gtk3 glib2
rustup toolchain install stable
```

## Building

```sh
cargo build
```

For a release build:

```sh
cargo build --release
```

## Code Style

The project uses `rustfmt`. Before submitting, run:

```sh
cargo fmt
```

Configuration is in [rustfmt.toml](rustfmt.toml).

## Translations

Translations use [Fluent](https://projectfluent.org/) and live in [i18n/](i18n/). Each locale has its own directory (e.g., `i18n/de/`).

To add a new language:

1. Copy `i18n/en/` to `i18n/<locale>/`
2. Translate the `.ftl` files
3. Add the locale to [i18n.toml](i18n.toml)

## Submitting Changes

1. Fork the repository and create a feature branch
2. Make your changes and ensure the build passes
3. Run `cargo fmt` and `cargo clippy`
4. Open a pull request with a clear description of the change

## Debugging

Run with verbose output using the `RUST_LOG` environment variable:

```sh
RUST_LOG=debug cargo run
```

Logs are written to both stdout and `~/.config/cachyos/cachyos-hello/cachyos-hello.log`.

To filter log output to a specific module:

```sh
RUST_LOG=cachyos_hello=debug cargo run
```

To suppress noisy crates:

```sh
RUST_LOG=debug,i2n_embed=warn,which=warn cargo run
```

## Reporting Issues

Use the [GitHub issue tracker](https://github.com/cachyos/cachyos-welcome/issues). Include your CachyOS version and steps to reproduce.
