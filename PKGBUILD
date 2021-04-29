pkgname=imount
pkgver=0.1.0
pkgrel=1
pkgdesc="Tool for automatic mounting Apple devices created by KOOMPI"
url="https://github.com/koompi/imount"
license=("MIT")
arch=("x86_64")
depends=("ifuse" "fuse2" "libimobiledevice" "libplist" "libnotify" "systemd")
makedepends=("rustup")
source=("git+https://github.com/koompi/imount.git")
md5sums=('SKIP')

build() {
  cd $srcdir/imount
  rustup default stable
  cargo build --release --bin imount-daemon
  cargo build --release --bin imount
}

package() {
  mkdir -p $pkgdir/usr/{bin,lib/systemd/system/}
  install -Dm755 $srcdir/imount/target/release/imount $pkgdir/usr/bin
  install -Dm755 $srcdir/imount/target/release/imount-daemon $pkgdir/usr/bin
  install -Dm644 $srcdir/imount/imount.service $pkgdir/usr/lib/systemd/system/
}