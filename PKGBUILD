# PKGBUILD for local testing
# This file allows building from the current local source instead of downloading a tarball.

pkgname=libkorean
pkgver=$(grep -m 1 '^version =' Cargo.toml | cut -d '"' -f 2)
pkgrel=1
pkgdesc="Korean input method library (local build)"
arch=('x86_64' 'aarch64')
url="https://github.com/reo2248/libkorean"
license=('MIT')
depends=('glibc' 'gcc-libs')
makedepends=('cargo')
provides=('libkorean.so')
conflicts=('libkorean' 'libkorean-git')

prepare() {
  export CARGO_HOME="$srcdir/cargo-home"
  cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
  export CARGO_HOME="$srcdir/cargo-home"
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  export CARGO_HOME="$srcdir/cargo-home"
  cargo test --frozen --release
}

package() {
  # Libraries and Binaries
  install -Dm755 "$srcdir/target/release/libkorean.so" -t "$pkgdir/usr/lib/"
  install -Dm755 "$srcdir/target/release/korean" "$srcdir/target/release/korean-hanja-search" -t "$pkgdir/usr/bin/"

  # Headers
  install -Dm644 "$startdir/src/korean.h" "$pkgdir/usr/include/libkorean/korean.h"

  # pkg-config
  install -Dm644 "$startdir/libkorean.pc" "$pkgdir/usr/lib/pkgconfig/libkorean.pc"

  # Data files
  install -d "$pkgdir/usr/share/libkorean/keyboards"
  install -m644 "$startdir"/data/keyboards/*.yaml "$pkgdir/usr/share/libkorean/keyboards/"
  install -Dm644 "$startdir/data/hanja/hanja.txt" "$pkgdir/usr/share/libkorean/hanja/hanja.txt"

  # License
  install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/libkorean/LICENSE"
}
