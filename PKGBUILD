# Maintainer: Joost Molenaar <jjm@j0057.nl>

pkgname=dsmr-prometheus
pkgdesc='DSMR P1 port exporter for Prometheus'
pkgver=0.1.0
pkgrel=1
arch=(x86_64)
url='https://github.com/j0057/dsmr-prometheus'
license=('unknown')
makedepends=(git cargo systemd)
source=("dsmr-prometheus::git+file:///$(pwd)")
md5sums=('SKIP')

prepare() {
    cd "$srcdir/dsmr-prometheus"
    cargo fetch --offline --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/dsmr-prometheus"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$srcdir/dsmr-prometheus"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    cd "$srcdir"
    install -v -o root -g root -m 755 -D dsmr-prometheus/target/release/dsmr-prometheus -t $pkgdir/usr/bin
    install -v -o root -g root -m 644 -D dsmr-prometheus/dsmr-prometheus.service -t $pkgdir/usr/lib/systemd/system
}
