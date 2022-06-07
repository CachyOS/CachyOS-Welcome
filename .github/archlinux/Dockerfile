FROM archlinux/archlinux:base-devel

WORKDIR /app
COPY . .

RUN pacman -Syu --noconfirm --noprogressbar git rustup glib2 gtk3 pkg-config meson

RUN useradd -d /app builder
RUN echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
USER builder

RUN echo "Reading /etc/os-release" \
    cat /etc/os-release || true

RUN sudo chown builder:builder .

RUN rustup toolchain install nightly
RUN rustup component add cargo
RUN rustup component add clippy

RUN meson --prefix=/usr build
RUN echo "============ Running build =============" && cargo build --verbose
RUN echo "============ Running test  =============" && cargo test --verbose
