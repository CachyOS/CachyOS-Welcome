FROM archlinux/archlinux:base-devel

WORKDIR /app
ENV CARGO_TERM_COLOR=always

RUN pacman -Syu --noconfirm --noprogressbar git rustup glib2 gtk3 pkg-config meson wget

RUN useradd -d /app builder
RUN echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
USER builder

RUN echo "Reading /etc/os-release" \
    cat /etc/os-release || true

RUN sudo chown builder:builder .

RUN rustup toolchain install nightly
RUN rustup component add cargo
RUN rustup component add clippy

RUN wget https://github.com/CachyOS/CachyOS-PKGBUILDS/raw/master/cachyos-hello-git/PKGBUILD

RUN makepkg -sf --noconfirm --needed
