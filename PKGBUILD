# Maintainer: Utkarsh Bhardwaj (Passeriform) <passeriform.ub@gmail.com>
pkgname=godwit-daemon
pkgver=0.1.10
pkgrel=1
pkgdesc="A daemon runner for GodWit."
arch=('x86_64' 'i686' 'armv6h' 'armv7h')
url="https://www.passeriform.com/prod/GodWit/daemon"
license=('MIT OR Apache-2.0')
depends=('wmctrl' 'gcc' 'lz4')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Passeriform/GodWit-Daemon/archive/v$pkgver.tar.gz")
optdepends=('godwit')
provides=('godwit')
backup=()
options=()
install=
changelog=
md5sums=('92ef3ee44a4e9a920c9aaf9fb7bcf60a')


prepare() {
  # Fix naming inconsistency
  mv "GodWit-Daemon-$pkgver" "$pkgname-$pkgver"
}

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --all-features
}

check() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo test --release
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
  chmod +x ./scripts/install-misc && ./scripts/install-misc ${pkgdir} ${pkgver}
}
